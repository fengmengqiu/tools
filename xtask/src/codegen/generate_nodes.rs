use std::collections::HashMap;

use super::kinds_src::AstSrc;
use crate::codegen::kinds_src::TokenKind;
use crate::{
	codegen::{kinds_src::Field, to_lower_snake_case, to_upper_snake_case},
	Result, SYNTAX_ELEMENT_TYPE,
};
use quote::{format_ident, quote};

pub fn generate_nodes(ast: &AstSrc) -> Result<String> {
	let (node_defs, node_boilerplate_impls): (Vec<_>, Vec<_>) = ast
		.nodes
		.iter()
		.map(|node| {
			let name = format_ident!("{}", node.name);
			let node_kind = format_ident!("{}", to_upper_snake_case(node.name.as_str()));
			let mut slot = 0usize;

			let methods = node
				.fields
				.iter()
				.filter(|field| {
					!matches!(
						field,
						Field::Token { manual: true, .. } | Field::Node { manual: true, .. }
					)
				})
				.map(|field| match field {
					Field::Token { kind, name, .. } => {
						// TODO: make the mandatory/optional bit
						let method_name = field.method_name();
						let is_optional = field.is_optional();

						match kind {
							TokenKind::Many(kinds) => {
								let tokens = token_kinds_to_code(kinds.as_slice());
								let method_name = format_ident!("{}", name);

								if is_optional {
									quote! {
										pub fn #method_name(&self) -> Option<SyntaxToken> {
											support::find_token(&self.syntax, #tokens)
										}
									}
								} else {
									quote! {
										pub fn #method_name(&self) -> SyntaxResult<SyntaxToken> {
											support::find_required_token(&self.syntax, #tokens)
										}
									}
								}
							}
							TokenKind::Single(kind) => {
								let token_kind_code = token_kind_to_code(kind.as_str());
								if is_optional {
									quote! {
										pub fn #method_name(&self) -> Option<SyntaxToken> {
											support::token(&self.syntax, #token_kind_code)
										}
									}
								} else {
									quote! {
										pub fn #method_name(&self) -> SyntaxResult<SyntaxToken> {
											support::required_token(&self.syntax, #token_kind_code)
										}
									}
								}
							}
						}
					}
					Field::Node {
						ty,
						optional,
						has_many,
						separated,
						..
					} => {
						let is_syntax_type = &ty.eq(SYNTAX_ELEMENT_TYPE);
						let ty = format_ident!("{}", &ty);

						let method_name = field.method_name();
						// this is when we encounter a node that has "Unknown" in its name
						// it will return tokens a and nodes regardless because there's an error
						// inside the code
						if *is_syntax_type {
							quote! {
								pub fn items(&self) -> SyntaxElementChildren {
									support::elements(&self.syntax)
								}
							}
						} else if *optional {
							quote! {
								pub fn #method_name(&self) -> Option<#ty> {
									support::node(&self.syntax)
								}
							}
						} else if *has_many {
							let field = if *separated {
								quote! {
									pub fn #method_name(&self) -> AstSeparatedList<#ty> {
										support::separated_list(&self.syntax, #slot)
									}
								}
							} else {
								quote! {
									pub fn #method_name(&self) -> AstNodeList<#ty> {
										support::node_list(&self.syntax, #slot)
									}
								}
							};

							slot += 1;
							field
						} else {
							quote! {
								pub fn #method_name(&self) -> SyntaxResult<#ty> {
									support::required_node(&self.syntax)
								}
							}
						}
					}
				});

			let fields = node.fields.iter().map(|field| {
				let is_syntax_element_children = matches!(
					field,
					Field::Node {
						ref ty,
						..
					} if ty == SYNTAX_ELEMENT_TYPE
				);

				let name = match field {
					Field::Token {
						name,
						kind: TokenKind::Many(_),
						..
					} => format_ident!("{}", name),
					_ => field.method_name(),
				};

				let string_name = name.to_string();

				if is_syntax_element_children {
					quote! {
						.field("items", &support::DebugSyntaxElementChildren(self.items()))
					}
				} else if field.is_many() {
					quote! {
						.field(#string_name, &self.#name())
					}
				} else if field.is_optional() {
					quote! {
						.field(#string_name, &support::DebugOptionalElement(self.#name()))
					}
				} else {
					quote! {
						.field(#string_name, &support::DebugSyntaxResult(self.#name()))
					}
				}
			});

			let string_name = name.to_string();

			(
				quote! {
					// TODO: review documentation
					// #[doc = #documentation]
					#[derive(Clone, PartialEq, Eq, Hash)]
					pub struct #name {
						pub(crate) syntax: SyntaxNode,
					}

					impl #name {
						#(#methods)*
					}

				},
				quote! {
					impl AstNode for #name {
						fn can_cast(kind: SyntaxKind) -> bool {
							kind == #node_kind
						}
						fn cast(syntax: SyntaxNode) -> Option<Self> {
							if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
						}
						fn syntax(&self) -> &SyntaxNode { &self.syntax }
					}

					impl std::fmt::Debug for #name {
						fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
							f.debug_struct(#string_name)
								#(#fields)*
								.finish()
						}
					}
				},
			)
		})
		.unzip();

	// it maps enum name A and its corresponding variants
	let name_to_variants: HashMap<_, _> = ast
		.enums
		.iter()
		.map(|current_enum| (current_enum.name.clone(), current_enum.variants.clone()))
		.collect();

	let (enum_defs, enum_boilerplate_impls): (Vec<_>, Vec<_>) = ast
		.enums
		.iter()
		.map(|en| {
			// here we collect all the variants because this will generate the enums
			// so we don't care about filtered variants
			let variants_for_enum: Vec<_> = en
				.variants
				.iter()
				.map(|en| {
					let variant_name = format_ident!("{}", en);

					quote! {
						#variant_name(#variant_name)
					}
				})
				.collect();

			// Here we make the partition
			//
			// Inside an enum, we can have variants that point to a "flat" type or to another enum;
			// we want to divide these variants as we will generate a different code based on these requirements
			let (variant_of_variants, simple_variants): (Vec<_>, Vec<_>) =
				en.variants.iter().partition(|current_enum| {
					if let Some(variants) = name_to_variants.get(*current_enum) {
						!variants.is_empty()
					} else {
						false
					}
				});

			let variants: Vec<_> = simple_variants
				.iter()
				.map(|var| format_ident!("{}", var))
				.collect();

			let name = format_ident!("{}", en.name);
			let kinds: Vec<_> = variants
				.iter()
				.map(|name| format_ident!("{}", to_upper_snake_case(&name.to_string())))
				.collect();

			let variant_cast: Vec<_> = simple_variants
				.iter()
				.map(|current_enum| {
					let variant_is_enum = ast.enums.iter().find(|e| &e.name == *current_enum);
					let variant_name = format_ident!("{}", current_enum);

					if variant_is_enum.is_some() {
						quote! {
							#variant_name::cast(syntax)?
						}
					} else {
						quote! {
							#variant_name { syntax }
						}
					}
				})
				.collect();

			// variant of variants
			let vv: Vec<_> = variant_of_variants
				.iter()
				.enumerate()
				.map(|(i, en)| {
					let variant_name = format_ident!("{}", en);
					let variable_name = format_ident!("{}", to_lower_snake_case(en.as_str()));
					(
						// cast() code
						if i != variant_of_variants.len() - 1 {
							quote! {
							if let Some(#variable_name) = #variant_name::cast(syntax.clone()) {
									return Some(#name::#variant_name(#variable_name));
							}}
						} else {
							// if this is the last variant, do not clone syntax
							quote! {
								if let Some(#variable_name) = #variant_name::cast(syntax) {
									return Some(#name::#variant_name(#variable_name));
							}}
						},
						// can_cast() code
						quote! {
							k if #variant_name::can_cast(k) => true,
						},
						// syntax() code
						quote! {
							#name::#variant_name(it) => it.syntax()
						},
					)
				})
				.collect();

			let vv_cast = vv.iter().map(|v| v.0.clone());

			let vv_can_cast = vv.iter().map(|v| v.1.clone());
			let vv_syntax = vv.iter().map(|v| v.2.clone());

			let all_kinds = if !kinds.is_empty() {
				quote! {
					#(#kinds)|* => true,
				}
			} else {
				quote! {}
			};

			let cast_fn = if !kinds.is_empty() {
				quote! {
					let res = match syntax.kind() {
						#(
							#kinds => #name::#variants(#variant_cast),
						)*
						_ =>  {
							#(
								#vv_cast
							)*
							return None
						}
					};
					Some(res)
				}
			} else {
				quote! {
						#(
						#vv_cast
					)*
					None
				}
			};

			let can_cast_fn = if en.variants.iter().any(|v| !simple_variants.contains(&v)) {
				quote! {
					match kind {
						#all_kinds
						#(#vv_can_cast)*
						_ => false
					}
				}
			} else {
				quote! {
					matches!(kind, #(#kinds)|*)
				}
			};

			let variant_can_cast: Vec<_> = simple_variants
				.iter()
				.map(|_| {
					quote! {
						&it.syntax
					}
				})
				.collect();

			let all_variant_names = en
				.variants
				.iter()
				.map(|variant| format_ident!("{}", variant));

			(
				quote! {
					// #[doc = #doc]
					#[derive(Clone, PartialEq, Eq, Hash)]
					pub enum #name {
						#(#variants_for_enum),*
					}
				},
				quote! {
					#(
					impl From<#variants> for #name {
						fn from(node: #variants) -> #name {
							#name::#variants(node)
						}
					}
					)*

					impl AstNode for #name {
						fn can_cast(kind: SyntaxKind) -> bool {
							#can_cast_fn
						}
						fn cast(syntax: SyntaxNode) -> Option<Self> {
								#cast_fn
						}
						fn syntax(&self) -> &SyntaxNode {
							match self {
								#(
								#name::#variants(it) => #variant_can_cast,
								)*
								#(
									#vv_syntax
								),*

							}
						}
					}

					impl std::fmt::Debug for #name {
						fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
							match self {
							#(
								#name::#all_variant_names(it) => std::fmt::Debug::fmt(it, f),
							)*
						}
							}
					}
				},
			)
		})
		.unzip();

	let enum_names = ast.enums.iter().map(|it| &it.name);
	let node_names = ast.nodes.iter().map(|it| &it.name);

	let display_impls = enum_names
		.chain(node_names.clone())
		.map(|it| format_ident!("{}", it))
		.map(|name| {
			quote! {
				impl std::fmt::Display for #name {
					fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
						std::fmt::Display::fmt(self.syntax(), f)
					}
				}
			}
		});

	let ast = quote! {
	#![allow(clippy::enum_variant_names)]
	// sometimes we generate comparison of simple tokens
	#![allow(clippy::match_like_matches_macro)]

	use crate::{
		ast::*,
		SyntaxKind::{self, *},
		SyntaxNode, SyntaxToken, T,
		SyntaxResult
	};


		#(#node_defs)*
		#(#enum_defs)*
		#(#node_boilerplate_impls)*
		#(#enum_boilerplate_impls)*
		#(#display_impls)*
	};

	let ast = ast
		.to_string()
		.replace("T ! [ ", "T![")
		.replace(" ] )", "])");

	let pretty = crate::reformat(ast)?;
	Ok(pretty)
}

fn token_kind_to_code(name: &str) -> proc_macro2::TokenStream {
	let token: proc_macro2::TokenStream = name.parse().unwrap();
	quote! { T![#token] }
}

fn token_kinds_to_code(kinds: &[String]) -> proc_macro2::TokenStream {
	let streamed_tokens: Vec<proc_macro2::TokenStream> = kinds
		.iter()
		.map(|kind| token_kind_to_code(kind.as_str()))
		.collect();

	quote! {
		&[#(#streamed_tokens),*]
	}
}
