//! Generate SyntaxKind definitions as well as typed AST definitions for nodes and tokens.
//! This is derived from rust-analyzer/xtask/codegen

use std::vec;

use super::{
	kinds_src::{AstSrc, Field},
	to_lower_snake_case, Mode,
};
use crate::codegen::kinds_src::TokenKind;
use crate::{
	codegen::{
		self,
		generate_nodes::generate_nodes,
		generate_syntax_kinds::generate_syntax_kinds,
		generate_tokens::generate_tokens,
		kinds_src::{AstEnumSrc, AstNodeSrc, KINDS_SRC},
		update,
	},
	project_root, Result, SYNTAX_ELEMENT_TYPE,
};
use ungrammar::{Grammar, Rule, Token};

pub fn generate_ast(mode: Mode) -> Result<()> {
	let grammar_src = include_str!("../../js.ungram");
	let grammar: Grammar = grammar_src.parse().unwrap();
	let ast = make_ast(&grammar);

	let tokens_file = project_root().join(codegen::AST_TOKENS);
	let contents = generate_tokens(&ast)?;
	update(tokens_file.as_path(), &contents, mode)?;

	let ast_nodes_file = project_root().join(codegen::AST_NODES);
	let contents = generate_nodes(&ast)?;
	update(ast_nodes_file.as_path(), &contents, mode)?;

	let syntax_kinds_file = project_root().join(codegen::SYNTAX_KINDS);
	let contents = generate_syntax_kinds(KINDS_SRC)?;
	update(syntax_kinds_file.as_path(), &contents, mode)?;

	Ok(())
}

fn make_ast(grammar: &Grammar) -> AstSrc {
	let tokens = "Whitespace Comment"
		.split_ascii_whitespace()
		.map(|it| it.to_string())
		.collect::<Vec<_>>();

	let mut ast = AstSrc {
		tokens,
		..Default::default()
	};

	for node in grammar.iter() {
		let name = grammar[node].name.clone();
		if name == SYNTAX_ELEMENT_TYPE {
			continue;
		}

		let rule = &grammar[node].rule;

		match handle_alternatives(grammar, rule) {
			Some(variants) => ast.enums.push(AstEnumSrc {
				documentation: vec![],
				name,
				variants,
			}),
			None => {
				let mut fields = vec![];
				handle_rule(&mut fields, grammar, rule, None, false, false, false);
				ast.nodes.push(AstNodeSrc {
					documentation: vec![],
					name,
					fields,
				})
			}
		}
	}

	ast.enums.push(AstEnumSrc {
		name: "AnyNode".to_string(),
		variants: ast.nodes.iter().map(|node| &node.name).cloned().collect(),
		documentation: vec![],
	});

	ast
}

fn handle_alternatives(grammar: &Grammar, rule: &Rule) -> Option<Vec<String>> {
	match rule {
		// this is for enums
		Rule::Alt(alternatives) => {
			let mut all_alternatives = vec![];
			for alternative in alternatives {
				match alternative {
					Rule::Node(it) => all_alternatives.push(grammar[*it].name.clone()),
					Rule::Token(it) if grammar[*it].name == ";" => (),
					_ => return None,
				}
			}
			Some(all_alternatives)
		}
		_ => None,
	}
}

fn clean_token_name(grammar: &Grammar, token: &Token) -> String {
	let mut name = grammar[*token].name.clone();

	// These tokens, when parsed to proc_macro2::TokenStream, generates a stream of bytes
	// that can't be recognized by [quote].
	// Hence, they need to be decorated with single quotes.
	if "[]{}()`".contains(&name) {
		name = format!("'{}'", name);
	}
	name
}

fn handle_rule(
	fields: &mut Vec<Field>,
	grammar: &Grammar,
	rule: &Rule,
	label: Option<&str>,
	optional: bool,
	has_many: bool,
	manual: bool,
) {
	match rule {
		Rule::Labeled { label, rule } => {
			// Some methods need to be manually implemented because they need some custom logic;
			// we use the prefix "manual__" to exclude labelled nodes.

			if handle_tokens_in_unions(fields, grammar, rule, label, optional) {
				return;
			}

			let manually_implemented = label.as_str().starts_with("manual__");
			handle_rule(
				fields,
				grammar,
				rule,
				Some(if manually_implemented {
					label.trim_start_matches("manual__")
				} else {
					label
				}),
				optional,
				has_many,
				manually_implemented,
			)
		}
		Rule::Node(node) => {
			let ty = grammar[*node].name.clone();
			let name = label
				.map(String::from)
				.unwrap_or_else(|| to_lower_snake_case(&ty));
			let field = Field::Node {
				name,
				ty,
				optional,
				has_many,
				separated: false,
				manual,
			};
			fields.push(field);
		}
		Rule::Token(token) => {
			let name = clean_token_name(grammar, token);

			let field = Field::Token {
				name: label.map(String::from).unwrap_or_else(|| name.clone()),
				kind: TokenKind::Single(name),
				optional,
				manual,
			};
			fields.push(field);
		}

		Rule::Rep(rule) => {
			handle_rule(fields, grammar, rule, label, false, true, manual);
		}
		Rule::Opt(rule) => {
			handle_rule(fields, grammar, rule, label, true, false, manual);
		}
		Rule::Alt(rules) => {
			for rule in rules {
				handle_rule(fields, grammar, rule, label, false, false, manual);
			}
		}

		Rule::Seq(rules) => {
			if handle_comma_list(fields, grammar, label, rules.as_slice()) {
				return;
			}

			for rule in rules {
				handle_rule(fields, grammar, rule, label, false, false, manual);
			}
		}
	};

	// (T (',' T)* ','?)
	// (T (',' T)*)
	fn handle_comma_list(
		acc: &mut Vec<Field>,
		grammar: &Grammar,
		label: Option<&str>,
		rules: &[Rule],
	) -> bool {
		// Does it match (T * ',')?
		let (node, repeat, trailing_separator) = match rules {
			[Rule::Node(node), Rule::Rep(repeat), Rule::Opt(trailing_separator)] => {
				(node, repeat, Some(trailing_separator))
			}
			[Rule::Node(node), Rule::Rep(repeat)] => (node, repeat, None),
			_ => return false,
		};

		// Is the repeat a ()*?
		let repeat = match &**repeat {
			Rule::Seq(it) => it,
			_ => return false,
		};

		// Does the repeat match (token node))
		match repeat.as_slice() {
			[comma, Rule::Node(n)] => {
				let separator_matches_trailing = if let Some(trailing) = trailing_separator {
					&**trailing == comma
				} else {
					true
				};

				if n != node || !separator_matches_trailing {
					return false;
				}
			}
			_ => return false,
		}

		let ty = grammar[*node].name.clone();

		let name = label
			.map(String::from)
			.unwrap_or_else(|| pluralize(&to_lower_snake_case(&ty)));

		acc.push(Field::Node {
			name,
			ty,
			optional: false,
			has_many: true,
			separated: true,
			manual: false,
		});

		true
	}

	fn pluralize(s: &str) -> String {
		format!("{}s", s)
	}
}

// handle cases like:  `op: ('-' | '+' | '*')`
fn handle_tokens_in_unions(
	fields: &mut Vec<Field>,
	grammar: &Grammar,
	rule: &Rule,
	label: &str,
	optional: bool,
) -> bool {
	let (rule, optional) = match rule {
		Rule::Opt(rule) => (&**rule, true),
		_ => (rule, optional),
	};

	let rule = match rule {
		Rule::Alt(rule) => rule,
		_ => return false,
	};

	let mut token_kinds = vec![];
	for rule in rule.iter() {
		match rule {
			Rule::Token(token) => token_kinds.push(clean_token_name(grammar, token)),
			_ => return false,
		}
	}

	let field = Field::Token {
		name: label.to_string(),
		kind: TokenKind::Many(token_kinds),
		optional,
		manual: false,
	};
	fields.push(field);
	true
}
