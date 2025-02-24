//! General utility functions for parsing and error checking.

use crate::{ast, AstNode, CompletedMarker, Parser, TextRange, Token};
use rslint_syntax::{SyntaxKind, T};

/// Check if the use of a statement label is valid and the label is defined.
///
/// # Panics
/// Panics if the marker is not a name with an ident
// FIXME: Labels should not cross function boundaries
pub fn check_label_use(p: &mut Parser, label: &Token) {
	let name = p.token_src(label);

	if p.state.labels.get(name).is_none() {
		let err = p
			.err_builder(&format!("Use of undefined statement label `{}`", name))
			.primary(&label.range, "This label is used, but it is never defined");

		p.error(err);
	}
}

/// Get the precedence of a token
pub fn get_precedence(tok: SyntaxKind) -> Option<u8> {
	Some(match tok {
		T![||] | T![??] => 1,
		T![&&] => 2,
		T![|] => 3,
		T![^] => 4,
		T![&] => 5,
		T![==] | T![!=] | T![===] | T![!==] => 6,
		T![>] | T![>=] | T![<] | T![<=] => 7,
		T![<<] | T![>>] | T![>>>] => 8,
		T![+] | T![-] => 9,
		T![*] | T![/] => 10,
		T![%] | T![**] => 11,
		_ => return None,
	})
}

/// Check if the var declaration in a for statement has multiple declarators, which is invalid
pub fn check_for_stmt_declaration(p: &mut Parser, marker: &CompletedMarker) {
	#[allow(deprecated)]
	let parsed = p.parse_marker::<ast::JsVariableDeclaration>(marker);
	let excess = parsed.declarators().iter().skip(1).collect::<Vec<_>>();

	if !excess.is_empty() {
		let start = marker
			.offset_range(
				p,
				excess
					.first()
					.unwrap()
					.as_ref()
					.unwrap()
					.syntax()
					.text_trimmed_range(),
			)
			.start();
		let end = marker
			.offset_range(
				p,
				excess
					.last()
					.unwrap()
					.as_ref()
					.unwrap()
					.syntax()
					.text_trimmed_range(),
			)
			.end();

		let err = p
			.err_builder("For statement variable declarations may only have one declaration")
			.primary(TextRange::new(start, end), "");

		p.error(err);
	}
}

pub(crate) fn expect_keyword(p: &mut Parser, keyword_name: &str, kind: SyntaxKind) {
	if p.at(T![ident]) && p.cur_src() == keyword_name {
		p.bump_remap(kind);
	} else {
		let err = if p.cur() == SyntaxKind::EOF {
			p.err_builder(&format!(
				"expected `{}` but instead the file ends",
				keyword_name
			))
			.primary(p.cur_tok().range, "the file ends here")
		} else {
			p.err_builder(&format!(
				"expected `{}` but instead found `{}`",
				keyword_name,
				p.cur_src()
			))
			.primary(p.cur_tok().range, "unexpected")
		};

		p.error(err);
	}
}
