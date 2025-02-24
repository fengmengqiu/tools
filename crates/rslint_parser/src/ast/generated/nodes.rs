//! Generated file, do not edit by hand, see `xtask/src/codegen`

#![allow(clippy::enum_variant_names)]
#![allow(clippy::match_like_matches_macro)]
use crate::{
	ast::*,
	SyntaxKind::{self, *},
	SyntaxNode, SyntaxResult, SyntaxToken, T,
};
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsUnknownStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsUnknownStatement {
	pub fn items(&self) -> SyntaxElementChildren { support::elements(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsUnknownExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsUnknownExpression {
	pub fn items(&self) -> SyntaxElementChildren { support::elements(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsUnknownMember {
	pub(crate) syntax: SyntaxNode,
}
impl JsUnknownMember {
	pub fn items(&self) -> SyntaxElementChildren { support::elements(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsUnknownBinding {
	pub(crate) syntax: SyntaxNode,
}
impl JsUnknownBinding {
	pub fn items(&self) -> SyntaxElementChildren { support::elements(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsUnknownAssignment {
	pub(crate) syntax: SyntaxNode,
}
impl JsUnknownAssignment {
	pub fn items(&self) -> SyntaxElementChildren { support::elements(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsUnknownModifier {
	pub(crate) syntax: SyntaxNode,
}
impl JsUnknownModifier {
	pub fn items(&self) -> SyntaxElementChildren { support::elements(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsUnknownImportAssertionEntry {
	pub(crate) syntax: SyntaxNode,
}
impl JsUnknownImportAssertionEntry {
	pub fn items(&self) -> SyntaxElementChildren { support::elements(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsUnknownNamedImportSpecifier {
	pub(crate) syntax: SyntaxNode,
}
impl JsUnknownNamedImportSpecifier {
	pub fn items(&self) -> SyntaxElementChildren { support::elements(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct List {
	pub(crate) syntax: SyntaxNode,
}
impl List {
	pub fn items(&self) -> SyntaxElementChildren { support::elements(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Ident {
	pub(crate) syntax: SyntaxNode,
}
impl Ident {
	pub fn ident_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![ident])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsScript {
	pub(crate) syntax: SyntaxNode,
}
impl JsScript {
	pub fn interpreter_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![js_shebang])
	}
	pub fn directives(&self) -> AstNodeList<JsDirective> {
		support::node_list(&self.syntax, 0usize)
	}
	pub fn statements(&self) -> AstNodeList<JsAnyStatement> {
		support::node_list(&self.syntax, 1usize)
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsModule {
	pub(crate) syntax: SyntaxNode,
}
impl JsModule {
	pub fn interpreter_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![js_shebang])
	}
	pub fn directives(&self) -> AstNodeList<JsDirective> {
		support::node_list(&self.syntax, 0usize)
	}
	pub fn items(&self) -> AstNodeList<JsAnyModuleItem> { support::node_list(&self.syntax, 1usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsDirective {
	pub(crate) syntax: SyntaxNode,
}
impl JsDirective {
	pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![js_string_literal])
	}
	pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [;]) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsBlockStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsBlockStatement {
	pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['{'])
	}
	pub fn statements(&self) -> AstNodeList<JsAnyStatement> {
		support::node_list(&self.syntax, 0usize)
	}
	pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['}'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsEmptyStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsEmptyStatement {
	pub fn semicolon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [;])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsExpressionStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsExpressionStatement {
	pub fn expression(&self) -> SyntaxResult<JsAnyExpression> {
		support::required_node(&self.syntax)
	}
	pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [;]) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsIfStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsIfStatement {
	pub fn if_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![if])
	}
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['('])
	}
	pub fn test(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![')'])
	}
	pub fn consequent(&self) -> SyntaxResult<JsAnyStatement> {
		support::required_node(&self.syntax)
	}
	pub fn else_clause(&self) -> Option<JsElseClause> { support::node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsDoWhileStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsDoWhileStatement {
	pub fn do_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![do])
	}
	pub fn body(&self) -> SyntaxResult<JsAnyStatement> { support::required_node(&self.syntax) }
	pub fn while_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![while])
	}
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['('])
	}
	pub fn test(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![')'])
	}
	pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [;]) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsWhileStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsWhileStatement {
	pub fn while_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![while])
	}
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['('])
	}
	pub fn test(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![')'])
	}
	pub fn body(&self) -> SyntaxResult<JsAnyStatement> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ForStmt {
	pub(crate) syntax: SyntaxNode,
}
impl ForStmt {
	pub fn for_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![for])
	}
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['('])
	}
	pub fn init(&self) -> Option<ForStmtInit> { support::node(&self.syntax) }
	pub fn first_semi_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [;])
	}
	pub fn test(&self) -> Option<ForStmtTest> { support::node(&self.syntax) }
	pub fn update(&self) -> Option<ForStmtUpdate> { support::node(&self.syntax) }
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![')'])
	}
	pub fn cons(&self) -> SyntaxResult<JsAnyStatement> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ForInStmt {
	pub(crate) syntax: SyntaxNode,
}
impl ForInStmt {
	pub fn for_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![for])
	}
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['('])
	}
	pub fn left(&self) -> SyntaxResult<ForLeft> { support::required_node(&self.syntax) }
	pub fn in_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![in])
	}
	pub fn right(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![')'])
	}
	pub fn cons(&self) -> SyntaxResult<JsAnyStatement> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ForOfStmt {
	pub(crate) syntax: SyntaxNode,
}
impl ForOfStmt {
	pub fn for_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![for])
	}
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['('])
	}
	pub fn left(&self) -> SyntaxResult<ForLeft> { support::required_node(&self.syntax) }
	pub fn of_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![of])
	}
	pub fn right(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![')'])
	}
	pub fn cons(&self) -> SyntaxResult<JsAnyStatement> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsContinueStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsContinueStatement {
	pub fn continue_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![continue])
	}
	pub fn label_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![ident]) }
	pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [;]) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsBreakStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsBreakStatement {
	pub fn break_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![break])
	}
	pub fn label_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![ident]) }
	pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [;]) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsReturnStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsReturnStatement {
	pub fn return_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![return])
	}
	pub fn argument(&self) -> Option<JsAnyExpression> { support::node(&self.syntax) }
	pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [;]) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsWithStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsWithStatement {
	pub fn with_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![with])
	}
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['('])
	}
	pub fn object(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![')'])
	}
	pub fn body(&self) -> SyntaxResult<JsAnyStatement> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsLabeledStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsLabeledStatement {
	pub fn label_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![ident])
	}
	pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [:])
	}
	pub fn body(&self) -> SyntaxResult<JsAnyStatement> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsSwitchStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsSwitchStatement {
	pub fn switch_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![switch])
	}
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['('])
	}
	pub fn discriminant(&self) -> SyntaxResult<JsAnyExpression> {
		support::required_node(&self.syntax)
	}
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![')'])
	}
	pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['{'])
	}
	pub fn cases(&self) -> AstNodeList<JsAnySwitchClause> {
		support::node_list(&self.syntax, 0usize)
	}
	pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['}'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsThrowStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsThrowStatement {
	pub fn throw_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![throw])
	}
	pub fn argument(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
	pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [;]) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsTryStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsTryStatement {
	pub fn try_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![try])
	}
	pub fn body(&self) -> SyntaxResult<JsBlockStatement> { support::required_node(&self.syntax) }
	pub fn catch_clause(&self) -> SyntaxResult<JsCatchClause> {
		support::required_node(&self.syntax)
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsTryFinallyStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsTryFinallyStatement {
	pub fn try_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![try])
	}
	pub fn body(&self) -> SyntaxResult<JsBlockStatement> { support::required_node(&self.syntax) }
	pub fn catch_clause(&self) -> Option<JsCatchClause> { support::node(&self.syntax) }
	pub fn finally_clause(&self) -> SyntaxResult<JsFinallyClause> {
		support::required_node(&self.syntax)
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsDebuggerStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsDebuggerStatement {
	pub fn debugger_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![debugger])
	}
	pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [;]) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsFunctionDeclaration {
	pub(crate) syntax: SyntaxNode,
}
impl JsFunctionDeclaration {
	pub fn async_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![async]) }
	pub fn function_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![function])
	}
	pub fn star_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [*]) }
	pub fn id(&self) -> SyntaxResult<JsAnyBinding> { support::required_node(&self.syntax) }
	pub fn type_parameters(&self) -> Option<TsTypeParams> { support::node(&self.syntax) }
	pub fn parameter_list(&self) -> SyntaxResult<JsParameterList> {
		support::required_node(&self.syntax)
	}
	pub fn return_type(&self) -> Option<TsTypeAnnotation> { support::node(&self.syntax) }
	pub fn body(&self) -> SyntaxResult<JsFunctionBody> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsClassDeclaration {
	pub(crate) syntax: SyntaxNode,
}
impl JsClassDeclaration {
	pub fn class_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![class])
	}
	pub fn id(&self) -> SyntaxResult<JsAnyBinding> { support::required_node(&self.syntax) }
	pub fn implements_clause(&self) -> Option<TsImplementsClause> { support::node(&self.syntax) }
	pub fn extends_clause(&self) -> Option<JsExtendsClause> { support::node(&self.syntax) }
	pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['{'])
	}
	pub fn members(&self) -> AstNodeList<JsAnyClassMember> {
		support::node_list(&self.syntax, 0usize)
	}
	pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['}'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsVariableDeclarationStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsVariableDeclarationStatement {
	pub fn declaration(&self) -> SyntaxResult<JsVariableDeclaration> {
		support::required_node(&self.syntax)
	}
	pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [;]) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsEnum {
	pub(crate) syntax: SyntaxNode,
}
impl TsEnum {
	pub fn const_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![const]) }
	pub fn enum_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![enum])
	}
	pub fn ident(&self) -> SyntaxResult<Ident> { support::required_node(&self.syntax) }
	pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['{'])
	}
	pub fn members(&self) -> AstNodeList<TsEnumMember> { support::node_list(&self.syntax, 0usize) }
	pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['}'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsTypeAliasDecl {
	pub(crate) syntax: SyntaxNode,
}
impl TsTypeAliasDecl {
	pub fn type_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![type])
	}
	pub fn type_params(&self) -> SyntaxResult<TsTypeParams> { support::required_node(&self.syntax) }
	pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [=])
	}
	pub fn ty(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsNamespaceDecl {
	pub(crate) syntax: SyntaxNode,
}
impl TsNamespaceDecl {
	pub fn declare_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![declare])
	}
	pub fn ident(&self) -> SyntaxResult<Ident> { support::required_node(&self.syntax) }
	pub fn dot_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [.]) }
	pub fn body(&self) -> SyntaxResult<TsNamespaceBody> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsModuleDecl {
	pub(crate) syntax: SyntaxNode,
}
impl TsModuleDecl {
	pub fn declare_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![declare])
	}
	pub fn global_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![global]) }
	pub fn module_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![module])
	}
	pub fn dot_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [.]) }
	pub fn ident(&self) -> SyntaxResult<Ident> { support::required_node(&self.syntax) }
	pub fn body(&self) -> SyntaxResult<TsNamespaceBody> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsInterfaceDecl {
	pub(crate) syntax: SyntaxNode,
}
impl TsInterfaceDecl {
	pub fn declare_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![declare]) }
	pub fn interface_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![interface])
	}
	pub fn type_params(&self) -> SyntaxResult<TsTypeParams> { support::required_node(&self.syntax) }
	pub fn extends_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![extends]) }
	pub fn extends(&self) -> Option<TsExprWithTypeArgs> { support::node(&self.syntax) }
	pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['{'])
	}
	pub fn members(&self) -> SyntaxResult<TsTypeElement> { support::required_node(&self.syntax) }
	pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['}'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsElseClause {
	pub(crate) syntax: SyntaxNode,
}
impl JsElseClause {
	pub fn else_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![else])
	}
	pub fn alternate(&self) -> SyntaxResult<JsAnyStatement> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ForStmtInit {
	pub(crate) syntax: SyntaxNode,
}
impl ForStmtInit {
	pub fn inner(&self) -> SyntaxResult<ForHead> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ForStmtTest {
	pub(crate) syntax: SyntaxNode,
}
impl ForStmtTest {
	pub fn expr(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ForStmtUpdate {
	pub(crate) syntax: SyntaxNode,
}
impl ForStmtUpdate {
	pub fn expr(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsVariableDeclaration {
	pub(crate) syntax: SyntaxNode,
}
impl JsVariableDeclaration {
	pub fn kind_token(&self) -> SyntaxResult<SyntaxToken> {
		support::find_required_token(&self.syntax, &[T![var], T![const], T![let]])
	}
	pub fn declarators(&self) -> AstSeparatedList<JsVariableDeclarator> {
		support::separated_list(&self.syntax, 0usize)
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsCaseClause {
	pub(crate) syntax: SyntaxNode,
}
impl JsCaseClause {
	pub fn case_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![case])
	}
	pub fn test(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
	pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [:])
	}
	pub fn consequent(&self) -> AstNodeList<JsAnyStatement> {
		support::node_list(&self.syntax, 0usize)
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsDefaultClause {
	pub(crate) syntax: SyntaxNode,
}
impl JsDefaultClause {
	pub fn default_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![default])
	}
	pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [:])
	}
	pub fn consequent(&self) -> AstNodeList<JsAnyStatement> {
		support::node_list(&self.syntax, 0usize)
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsCatchClause {
	pub(crate) syntax: SyntaxNode,
}
impl JsCatchClause {
	pub fn catch_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![catch])
	}
	pub fn declaration(&self) -> Option<JsCatchDeclaration> { support::node(&self.syntax) }
	pub fn body(&self) -> SyntaxResult<JsBlockStatement> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsFinallyClause {
	pub(crate) syntax: SyntaxNode,
}
impl JsFinallyClause {
	pub fn finally_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![finally])
	}
	pub fn body(&self) -> SyntaxResult<JsBlockStatement> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsCatchDeclaration {
	pub(crate) syntax: SyntaxNode,
}
impl JsCatchDeclaration {
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['('])
	}
	pub fn binding(&self) -> SyntaxResult<JsAnyBindingPattern> {
		support::required_node(&self.syntax)
	}
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![')'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsArrayExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsArrayExpression {
	pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['['])
	}
	pub fn elements(&self) -> AstSeparatedList<JsAnyArrayElement> {
		support::separated_list(&self.syntax, 0usize)
	}
	pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![']'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsArrowFunctionExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsArrowFunctionExpression {
	pub fn async_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![async]) }
	pub fn type_parameters(&self) -> Option<TsTypeParams> { support::node(&self.syntax) }
	pub fn parameter_list(&self) -> Option<JsAnyArrowFunctionParameters> {
		support::node(&self.syntax)
	}
	pub fn fat_arrow_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [=>])
	}
	pub fn return_type(&self) -> Option<TsTypeAnnotation> { support::node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsAssignmentExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsAssignmentExpression {
	pub fn left(&self) -> SyntaxResult<JsAnyAssignmentPattern> {
		support::required_node(&self.syntax)
	}
	pub fn operator_token(&self) -> SyntaxResult<SyntaxToken> {
		support::find_required_token(
			&self.syntax,
			&[
				T ! [=],
				T ! [+=],
				T ! [-=],
				T ! [*=],
				T ! [/=],
				T ! [%=],
				T ! [**=],
				T ! [>>=],
				T ! [<<=],
				T ! [>>>=],
				T ! [&=],
				T ! [|=],
				T ! [^=],
				T ! [&&=],
				T ! [||=],
				T ! [??=],
			],
		)
	}
	pub fn right(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsAwaitExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsAwaitExpression {
	pub fn await_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![await])
	}
	pub fn argument(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsBinaryExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsBinaryExpression {
	pub fn left(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
	pub fn operator(&self) -> SyntaxResult<SyntaxToken> {
		support::find_required_token(
			&self.syntax,
			&[
				T ! [<],
				T ! [>],
				T ! [<=],
				T ! [>=],
				T ! [==],
				T ! [===],
				T ! [!=],
				T ! [!==],
				T ! [+],
				T ! [-],
				T ! [*],
				T ! [/],
				T ! [%],
				T ! [**],
				T ! [<<],
				T ! [>>],
				T ! [>>>],
				T ! [&],
				T ! [|],
				T ! [^],
				T![in],
				T![instanceof],
			],
		)
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsClassExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsClassExpression {
	pub fn class_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![class])
	}
	pub fn id(&self) -> Option<JsAnyBinding> { support::node(&self.syntax) }
	pub fn extends_clause(&self) -> Option<JsExtendsClause> { support::node(&self.syntax) }
	pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['{'])
	}
	pub fn members(&self) -> AstNodeList<JsAnyClassMember> {
		support::node_list(&self.syntax, 0usize)
	}
	pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['}'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsConditionalExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsConditionalExpression {
	pub fn test(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
	pub fn question_mark_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [?])
	}
	pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [:])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsComputedMemberExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsComputedMemberExpression {
	pub fn object(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
	pub fn optional_chain_token_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T ! [?.])
	}
	pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['['])
	}
	pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![']'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsFunctionExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsFunctionExpression {
	pub fn async_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![async]) }
	pub fn function_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![function])
	}
	pub fn star_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [*]) }
	pub fn id(&self) -> Option<JsAnyBinding> { support::node(&self.syntax) }
	pub fn type_parameters(&self) -> Option<TsTypeParams> { support::node(&self.syntax) }
	pub fn parameters(&self) -> SyntaxResult<JsParameterList> {
		support::required_node(&self.syntax)
	}
	pub fn return_type(&self) -> Option<TsTypeAnnotation> { support::node(&self.syntax) }
	pub fn body(&self) -> SyntaxResult<JsFunctionBody> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsImportCallExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsImportCallExpression {
	pub fn import_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![import])
	}
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['('])
	}
	pub fn argument(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![')'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsLogicalExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsLogicalExpression {
	pub fn left(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
	pub fn operator(&self) -> SyntaxResult<SyntaxToken> {
		support::find_required_token(&self.syntax, &[T ! [??], T ! [||], T ! [&&]])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsObjectExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsObjectExpression {
	pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['{'])
	}
	pub fn members(&self) -> AstSeparatedList<JsAnyObjectMember> {
		support::separated_list(&self.syntax, 0usize)
	}
	pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['}'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsParenthesizedExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsParenthesizedExpression {
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['('])
	}
	pub fn expression(&self) -> SyntaxResult<JsAnyExpression> {
		support::required_node(&self.syntax)
	}
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![')'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsIdentifierExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsIdentifierExpression {
	pub fn name(&self) -> SyntaxResult<JsReferenceIdentifier> {
		support::required_node(&self.syntax)
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsSequenceExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsSequenceExpression {
	pub fn left(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
	pub fn comma_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [,])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsStaticMemberExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsStaticMemberExpression {
	pub fn object(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
	pub fn operator(&self) -> SyntaxResult<SyntaxToken> {
		support::find_required_token(&self.syntax, &[T ! [.], T ! [?.]])
	}
	pub fn member(&self) -> SyntaxResult<JsAnyName> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsSuperExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsSuperExpression {
	pub fn super_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![super])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsThisExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsThisExpression {
	pub fn this_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![this])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsUnaryExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsUnaryExpression {
	pub fn operator(&self) -> SyntaxResult<SyntaxToken> {
		support::find_required_token(
			&self.syntax,
			&[
				T![delete],
				T![void],
				T![typeof],
				T ! [+],
				T ! [-],
				T ! [~],
				T![!],
			],
		)
	}
	pub fn argument(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsPreUpdateExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsPreUpdateExpression {
	pub fn operator(&self) -> SyntaxResult<SyntaxToken> {
		support::find_required_token(&self.syntax, &[T ! [++], T ! [--]])
	}
	pub fn operand(&self) -> SyntaxResult<JsAnyAssignment> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsPostUpdateExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsPostUpdateExpression {
	pub fn operand(&self) -> SyntaxResult<JsAnyAssignment> { support::required_node(&self.syntax) }
	pub fn operator(&self) -> SyntaxResult<SyntaxToken> {
		support::find_required_token(&self.syntax, &[T ! [++], T ! [--]])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsYieldExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsYieldExpression {
	pub fn yield_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![yield])
	}
	pub fn star_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [*]) }
	pub fn argument(&self) -> Option<JsAnyExpression> { support::node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Template {
	pub(crate) syntax: SyntaxNode,
}
impl Template {
	pub fn backtick_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['`'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct NewExpr {
	pub(crate) syntax: SyntaxNode,
}
impl NewExpr {
	pub fn new_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![new])
	}
	pub fn type_args(&self) -> Option<TsTypeArgs> { support::node(&self.syntax) }
	pub fn object(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
	pub fn arguments(&self) -> SyntaxResult<ArgList> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CallExpr {
	pub(crate) syntax: SyntaxNode,
}
impl CallExpr {
	pub fn type_args(&self) -> Option<TsTypeArgs> { support::node(&self.syntax) }
	pub fn callee(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
	pub fn arguments(&self) -> SyntaxResult<ArgList> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct NewTarget {
	pub(crate) syntax: SyntaxNode,
}
impl NewTarget {
	pub fn new_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![new])
	}
	pub fn dot_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [.])
	}
	pub fn target_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![target])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ImportMeta {
	pub(crate) syntax: SyntaxNode,
}
impl ImportMeta {
	pub fn import_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![import])
	}
	pub fn dot_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [.])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsNonNull {
	pub(crate) syntax: SyntaxNode,
}
impl TsNonNull {
	pub fn expr(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
	pub fn excl_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![!])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsAssertion {
	pub(crate) syntax: SyntaxNode,
}
impl TsAssertion {
	pub fn expr(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
	pub fn ident(&self) -> SyntaxResult<Ident> { support::required_node(&self.syntax) }
	pub fn l_angle_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [<])
	}
	pub fn ty(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax) }
	pub fn r_angle_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [>])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsConstAssertion {
	pub(crate) syntax: SyntaxNode,
}
impl TsConstAssertion {
	pub fn expr(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
	pub fn ident(&self) -> SyntaxResult<Ident> { support::required_node(&self.syntax) }
	pub fn l_angle_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [<])
	}
	pub fn const_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![const])
	}
	pub fn r_angle_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [>])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsTypeArgs {
	pub(crate) syntax: SyntaxNode,
}
impl TsTypeArgs {
	pub fn l_angle_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [<])
	}
	pub fn args(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax) }
	pub fn r_angle_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [>])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ArgList {
	pub(crate) syntax: SyntaxNode,
}
impl ArgList {
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['('])
	}
	pub fn args(&self) -> AstSeparatedList<JsAnyExpression> {
		support::separated_list(&self.syntax, 0usize)
	}
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![')'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsTypeParams {
	pub(crate) syntax: SyntaxNode,
}
impl TsTypeParams {
	pub fn l_angle_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [<]) }
	pub fn params(&self) -> SyntaxResult<TsTypeParam> { support::required_node(&self.syntax) }
	pub fn r_angle_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [>]) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsParameterList {
	pub(crate) syntax: SyntaxNode,
}
impl JsParameterList {
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['('])
	}
	pub fn parameters(&self) -> AstSeparatedList<JsAnyParameter> {
		support::separated_list(&self.syntax, 0usize)
	}
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![')'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsTypeAnnotation {
	pub(crate) syntax: SyntaxNode,
}
impl TsTypeAnnotation {
	pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [:])
	}
	pub fn ty(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsFunctionBody {
	pub(crate) syntax: SyntaxNode,
}
impl JsFunctionBody {
	pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['{'])
	}
	pub fn directives(&self) -> AstNodeList<JsDirective> {
		support::node_list(&self.syntax, 0usize)
	}
	pub fn statements(&self) -> AstNodeList<JsAnyStatement> {
		support::node_list(&self.syntax, 1usize)
	}
	pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['}'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsSpread {
	pub(crate) syntax: SyntaxNode,
}
impl JsSpread {
	pub fn dotdotdot_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [...])
	}
	pub fn argument(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsArrayHole {
	pub(crate) syntax: SyntaxNode,
}
impl JsArrayHole {}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsReferenceIdentifier {
	pub(crate) syntax: SyntaxNode,
}
impl JsReferenceIdentifier {
	pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![ident])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsLiteralMemberName {
	pub(crate) syntax: SyntaxNode,
}
impl JsLiteralMemberName {
	pub fn value(&self) -> SyntaxResult<SyntaxToken> {
		support::find_required_token(
			&self.syntax,
			&[T![ident], T![js_string_literal], T![js_number_literal]],
		)
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsComputedMemberName {
	pub(crate) syntax: SyntaxNode,
}
impl JsComputedMemberName {
	pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['['])
	}
	pub fn expression(&self) -> SyntaxResult<JsAnyExpression> {
		support::required_node(&self.syntax)
	}
	pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![']'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsPropertyObjectMember {
	pub(crate) syntax: SyntaxNode,
}
impl JsPropertyObjectMember {
	pub fn name(&self) -> SyntaxResult<JsAnyObjectMemberName> {
		support::required_node(&self.syntax)
	}
	pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [:])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsMethodObjectMember {
	pub(crate) syntax: SyntaxNode,
}
impl JsMethodObjectMember {
	pub fn async_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![async]) }
	pub fn star_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [*]) }
	pub fn name(&self) -> SyntaxResult<JsAnyObjectMemberName> {
		support::required_node(&self.syntax)
	}
	pub fn type_params(&self) -> Option<TsTypeParams> { support::node(&self.syntax) }
	pub fn parameter_list(&self) -> SyntaxResult<JsParameterList> {
		support::required_node(&self.syntax)
	}
	pub fn return_type(&self) -> Option<TsTypeAnnotation> { support::node(&self.syntax) }
	pub fn body(&self) -> SyntaxResult<JsFunctionBody> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsGetterObjectMember {
	pub(crate) syntax: SyntaxNode,
}
impl JsGetterObjectMember {
	pub fn get_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![get])
	}
	pub fn name(&self) -> SyntaxResult<JsAnyObjectMemberName> {
		support::required_node(&self.syntax)
	}
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['('])
	}
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![')'])
	}
	pub fn return_type(&self) -> Option<TsTypeAnnotation> { support::node(&self.syntax) }
	pub fn body(&self) -> SyntaxResult<JsFunctionBody> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsSetterObjectMember {
	pub(crate) syntax: SyntaxNode,
}
impl JsSetterObjectMember {
	pub fn set_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![set])
	}
	pub fn name(&self) -> SyntaxResult<JsAnyObjectMemberName> {
		support::required_node(&self.syntax)
	}
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['('])
	}
	pub fn parameter(&self) -> SyntaxResult<JsAnyBindingPattern> {
		support::required_node(&self.syntax)
	}
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![')'])
	}
	pub fn body(&self) -> SyntaxResult<JsFunctionBody> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsShorthandPropertyObjectMember {
	pub(crate) syntax: SyntaxNode,
}
impl JsShorthandPropertyObjectMember {
	pub fn name(&self) -> SyntaxResult<JsReferenceIdentifier> {
		support::required_node(&self.syntax)
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsImplementsClause {
	pub(crate) syntax: SyntaxNode,
}
impl TsImplementsClause {
	pub fn implements_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![implements])
	}
	pub fn interfaces(&self) -> AstSeparatedList<TsExprWithTypeArgs> {
		support::separated_list(&self.syntax, 0usize)
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsExtendsClause {
	pub(crate) syntax: SyntaxNode,
}
impl JsExtendsClause {
	pub fn extends_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![extends])
	}
	pub fn super_class(&self) -> SyntaxResult<JsAnyExpression> {
		support::required_node(&self.syntax)
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsExprWithTypeArgs {
	pub(crate) syntax: SyntaxNode,
}
impl TsExprWithTypeArgs {
	pub fn item(&self) -> SyntaxResult<TsEntityName> { support::required_node(&self.syntax) }
	pub fn type_params(&self) -> SyntaxResult<TsTypeArgs> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsPrivateClassMemberName {
	pub(crate) syntax: SyntaxNode,
}
impl JsPrivateClassMemberName {
	pub fn hash_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [#])
	}
	pub fn id_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![ident])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsConstructorClassMember {
	pub(crate) syntax: SyntaxNode,
}
impl JsConstructorClassMember {
	pub fn access_modifier(&self) -> Option<TsAccessibility> { support::node(&self.syntax) }
	pub fn name(&self) -> SyntaxResult<JsLiteralMemberName> { support::required_node(&self.syntax) }
	pub fn parameter_list(&self) -> SyntaxResult<JsConstructorParameterList> {
		support::required_node(&self.syntax)
	}
	pub fn body(&self) -> SyntaxResult<JsFunctionBody> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsPropertyClassMember {
	pub(crate) syntax: SyntaxNode,
}
impl JsPropertyClassMember {
	pub fn modifiers(&self) -> Option<JsAnyModifier> { support::node(&self.syntax) }
	pub fn access_modifier(&self) -> Option<TsAccessibility> { support::node(&self.syntax) }
	pub fn abstract_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![abstract])
	}
	pub fn static_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![static]) }
	pub fn name(&self) -> SyntaxResult<JsAnyClassMemberName> {
		support::required_node(&self.syntax)
	}
	pub fn question_mark_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T ! [?])
	}
	pub fn excl_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![!]) }
	pub fn ty(&self) -> Option<TsTypeAnnotation> { support::node(&self.syntax) }
	pub fn value(&self) -> Option<JsEqualValueClause> { support::node(&self.syntax) }
	pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [;]) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsMethodClassMember {
	pub(crate) syntax: SyntaxNode,
}
impl JsMethodClassMember {
	pub fn access_modifier(&self) -> Option<TsAccessibility> { support::node(&self.syntax) }
	pub fn static_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![static]) }
	pub fn abstract_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![abstract])
	}
	pub fn async_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![async]) }
	pub fn star_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [*]) }
	pub fn name(&self) -> SyntaxResult<JsAnyClassMemberName> {
		support::required_node(&self.syntax)
	}
	pub fn type_parameters(&self) -> Option<TsTypeParams> { support::node(&self.syntax) }
	pub fn parameter_list(&self) -> SyntaxResult<JsParameterList> {
		support::required_node(&self.syntax)
	}
	pub fn return_type(&self) -> Option<TsTypeAnnotation> { support::node(&self.syntax) }
	pub fn body(&self) -> SyntaxResult<JsFunctionBody> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsGetterClassMember {
	pub(crate) syntax: SyntaxNode,
}
impl JsGetterClassMember {
	pub fn access_modifier(&self) -> Option<TsAccessibility> { support::node(&self.syntax) }
	pub fn abstract_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![abstract])
	}
	pub fn static_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![static]) }
	pub fn get_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![get])
	}
	pub fn name(&self) -> SyntaxResult<JsAnyClassMemberName> {
		support::required_node(&self.syntax)
	}
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['('])
	}
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![')'])
	}
	pub fn return_type(&self) -> Option<TsTypeAnnotation> { support::node(&self.syntax) }
	pub fn body(&self) -> SyntaxResult<JsFunctionBody> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsSetterClassMember {
	pub(crate) syntax: SyntaxNode,
}
impl JsSetterClassMember {
	pub fn access_modifier(&self) -> Option<TsAccessibility> { support::node(&self.syntax) }
	pub fn abstract_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![abstract])
	}
	pub fn static_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![static]) }
	pub fn set_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![set])
	}
	pub fn name(&self) -> SyntaxResult<JsAnyClassMemberName> {
		support::required_node(&self.syntax)
	}
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['('])
	}
	pub fn parameter(&self) -> SyntaxResult<JsAnyBindingPattern> {
		support::required_node(&self.syntax)
	}
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![')'])
	}
	pub fn body(&self) -> SyntaxResult<JsFunctionBody> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsEmptyClassMember {
	pub(crate) syntax: SyntaxNode,
}
impl JsEmptyClassMember {
	pub fn semicolon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [;])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsIndexSignature {
	pub(crate) syntax: SyntaxNode,
}
impl TsIndexSignature {
	pub fn readonly_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![readonly])
	}
	pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['['])
	}
	pub fn pat(&self) -> SyntaxResult<JsAnyBinding> { support::required_node(&self.syntax) }
	pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [:])
	}
	pub fn ty(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax) }
	pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![']'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsAccessibility {
	pub(crate) syntax: SyntaxNode,
}
impl TsAccessibility {
	pub fn private_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![private])
	}
	pub fn readonly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![readonly])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsConstructorParameterList {
	pub(crate) syntax: SyntaxNode,
}
impl JsConstructorParameterList {
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['('])
	}
	pub fn parameters(&self) -> AstSeparatedList<JsAnyConstructorParameter> {
		support::separated_list(&self.syntax, 0usize)
	}
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![')'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsConstructorParam {
	pub(crate) syntax: SyntaxNode,
}
impl TsConstructorParam {
	pub fn readonly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![readonly])
	}
	pub fn pat(&self) -> SyntaxResult<JsAnyBindingPattern> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsBindingPatternWithDefault {
	pub(crate) syntax: SyntaxNode,
}
impl JsBindingPatternWithDefault {
	pub fn pattern(&self) -> SyntaxResult<JsAnyBindingPattern> {
		support::required_node(&self.syntax)
	}
	pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [=])
	}
	pub fn default(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsEqualValueClause {
	pub(crate) syntax: SyntaxNode,
}
impl JsEqualValueClause {
	pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [=])
	}
	pub fn expression(&self) -> SyntaxResult<JsAnyExpression> {
		support::required_node(&self.syntax)
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsModifier {
	pub(crate) syntax: SyntaxNode,
}
impl JsModifier {
	pub fn declare_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![declare]) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsIdentifierAssignment {
	pub(crate) syntax: SyntaxNode,
}
impl JsIdentifierAssignment {
	pub fn name_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![ident])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsStaticMemberAssignment {
	pub(crate) syntax: SyntaxNode,
}
impl JsStaticMemberAssignment {
	pub fn object(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
	pub fn dot_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [.])
	}
	pub fn member(&self) -> SyntaxResult<JsAnyName> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsComputedMemberAssignment {
	pub(crate) syntax: SyntaxNode,
}
impl JsComputedMemberAssignment {
	pub fn object(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
	pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['['])
	}
	pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![']'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsParenthesizedAssignment {
	pub(crate) syntax: SyntaxNode,
}
impl JsParenthesizedAssignment {
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['('])
	}
	pub fn assignment(&self) -> SyntaxResult<JsAnyAssignment> {
		support::required_node(&self.syntax)
	}
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![')'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsAssignmentWithDefault {
	pub(crate) syntax: SyntaxNode,
}
impl JsAssignmentWithDefault {
	pub fn pattern(&self) -> SyntaxResult<JsAnyAssignmentPattern> {
		support::required_node(&self.syntax)
	}
	pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [=])
	}
	pub fn default(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsArrayAssignmentPattern {
	pub(crate) syntax: SyntaxNode,
}
impl JsArrayAssignmentPattern {
	pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['['])
	}
	pub fn elements(&self) -> AstSeparatedList<JsAnyArrayAssignmentPatternElement> {
		support::separated_list(&self.syntax, 0usize)
	}
	pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![']'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsObjectAssignmentPattern {
	pub(crate) syntax: SyntaxNode,
}
impl JsObjectAssignmentPattern {
	pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['{'])
	}
	pub fn properties(&self) -> AstSeparatedList<JsAnyObjectAssignmentPatternMember> {
		support::separated_list(&self.syntax, 0usize)
	}
	pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['}'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsArrayAssignmentPatternRestElement {
	pub(crate) syntax: SyntaxNode,
}
impl JsArrayAssignmentPatternRestElement {
	pub fn dotdotdot_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [...])
	}
	pub fn pattern(&self) -> SyntaxResult<JsAnyAssignmentPattern> {
		support::required_node(&self.syntax)
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsObjectAssignmentPatternShorthandProperty {
	pub(crate) syntax: SyntaxNode,
}
impl JsObjectAssignmentPatternShorthandProperty {
	pub fn identifier(&self) -> SyntaxResult<JsAnyAssignment> {
		support::required_node(&self.syntax)
	}
	pub fn init(&self) -> Option<JsEqualValueClause> { support::node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsObjectAssignmentPatternProperty {
	pub(crate) syntax: SyntaxNode,
}
impl JsObjectAssignmentPatternProperty {
	pub fn member(&self) -> SyntaxResult<JsName> { support::required_node(&self.syntax) }
	pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [:])
	}
	pub fn pattern(&self) -> SyntaxResult<JsAnyAssignmentPattern> {
		support::required_node(&self.syntax)
	}
	pub fn init(&self) -> Option<JsEqualValueClause> { support::node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsObjectAssignmentPatternRest {
	pub(crate) syntax: SyntaxNode,
}
impl JsObjectAssignmentPatternRest {
	pub fn dotdotdot_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [...])
	}
	pub fn target(&self) -> SyntaxResult<JsAnyAssignment> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsName {
	pub(crate) syntax: SyntaxNode,
}
impl JsName {
	pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![ident])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsIdentifierBinding {
	pub(crate) syntax: SyntaxNode,
}
impl JsIdentifierBinding {
	pub fn name_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![ident])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsArrayBindingPattern {
	pub(crate) syntax: SyntaxNode,
}
impl JsArrayBindingPattern {
	pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['['])
	}
	pub fn elements(&self) -> AstSeparatedList<JsAnyArrayBindingPatternElement> {
		support::separated_list(&self.syntax, 0usize)
	}
	pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![']'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsObjectBindingPattern {
	pub(crate) syntax: SyntaxNode,
}
impl JsObjectBindingPattern {
	pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['{'])
	}
	pub fn properties(&self) -> AstSeparatedList<JsAnyObjectBindingPatternMember> {
		support::separated_list(&self.syntax, 0usize)
	}
	pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['}'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsArrayBindingPatternRestElement {
	pub(crate) syntax: SyntaxNode,
}
impl JsArrayBindingPatternRestElement {
	pub fn dotdotdot_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [...])
	}
	pub fn pattern(&self) -> SyntaxResult<JsAnyBindingPattern> {
		support::required_node(&self.syntax)
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsObjectBindingPatternProperty {
	pub(crate) syntax: SyntaxNode,
}
impl JsObjectBindingPatternProperty {
	pub fn member(&self) -> SyntaxResult<JsAnyObjectMemberName> {
		support::required_node(&self.syntax)
	}
	pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [:])
	}
	pub fn pattern(&self) -> SyntaxResult<JsAnyBindingPattern> {
		support::required_node(&self.syntax)
	}
	pub fn init(&self) -> Option<JsEqualValueClause> { support::node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsObjectBindingPatternRest {
	pub(crate) syntax: SyntaxNode,
}
impl JsObjectBindingPatternRest {
	pub fn dotdotdot_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [...])
	}
	pub fn binding(&self) -> SyntaxResult<JsAnyBinding> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsObjectBindingPatternShorthandProperty {
	pub(crate) syntax: SyntaxNode,
}
impl JsObjectBindingPatternShorthandProperty {
	pub fn identifier(&self) -> SyntaxResult<JsAnyBinding> { support::required_node(&self.syntax) }
	pub fn init(&self) -> Option<JsEqualValueClause> { support::node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsStringLiteralExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsStringLiteralExpression {
	pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![js_string_literal])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsNumberLiteralExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsNumberLiteralExpression {
	pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![js_number_literal])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsBigIntLiteralExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsBigIntLiteralExpression {
	pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![js_big_int_literal])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsBooleanLiteralExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsBooleanLiteralExpression {
	pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
		support::find_required_token(&self.syntax, &[T![true], T![false]])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsNullLiteralExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsNullLiteralExpression {
	pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![null])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsRegexLiteralExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsRegexLiteralExpression {
	pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![js_regex_literal])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsVariableDeclarator {
	pub(crate) syntax: SyntaxNode,
}
impl JsVariableDeclarator {
	pub fn id(&self) -> SyntaxResult<JsAnyBindingPattern> { support::required_node(&self.syntax) }
	pub fn init(&self) -> Option<JsEqualValueClause> { support::node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsImport {
	pub(crate) syntax: SyntaxNode,
}
impl JsImport {
	pub fn import_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![import])
	}
	pub fn import_clause(&self) -> SyntaxResult<AnyJsImportClause> {
		support::required_node(&self.syntax)
	}
	pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [;]) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ExportNamed {
	pub(crate) syntax: SyntaxNode,
}
impl ExportNamed {
	pub fn export_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![export])
	}
	pub fn type_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![type]) }
	pub fn from_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![from]) }
	pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['{'])
	}
	pub fn specifiers(&self) -> AstSeparatedList<Specifier> {
		support::separated_list(&self.syntax, 0usize)
	}
	pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['}'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ExportDefaultDecl {
	pub(crate) syntax: SyntaxNode,
}
impl ExportDefaultDecl {
	pub fn export_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![export])
	}
	pub fn default_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![default]) }
	pub fn type_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![type]) }
	pub fn decl(&self) -> SyntaxResult<DefaultDecl> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ExportDefaultExpr {
	pub(crate) syntax: SyntaxNode,
}
impl ExportDefaultExpr {
	pub fn export_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![export])
	}
	pub fn type_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![type]) }
	pub fn default_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![default]) }
	pub fn expr(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ExportWildcard {
	pub(crate) syntax: SyntaxNode,
}
impl ExportWildcard {
	pub fn export_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![export])
	}
	pub fn type_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![type]) }
	pub fn star_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [*])
	}
	pub fn as_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![as]) }
	pub fn ident(&self) -> Option<Ident> { support::node(&self.syntax) }
	pub fn from_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![from])
	}
	pub fn source_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![js_string_literal])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ExportDecl {
	pub(crate) syntax: SyntaxNode,
}
impl ExportDecl {
	pub fn export_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![export])
	}
	pub fn type_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![type]) }
	pub fn decl(&self) -> SyntaxResult<JsAnyExportDeclaration> {
		support::required_node(&self.syntax)
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsImportEqualsDecl {
	pub(crate) syntax: SyntaxNode,
}
impl TsImportEqualsDecl {
	pub fn import_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![import])
	}
	pub fn export_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![export])
	}
	pub fn ident(&self) -> SyntaxResult<Ident> { support::required_node(&self.syntax) }
	pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [=])
	}
	pub fn module(&self) -> SyntaxResult<TsModuleRef> { support::required_node(&self.syntax) }
	pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [;]) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsExportAssignment {
	pub(crate) syntax: SyntaxNode,
}
impl TsExportAssignment {
	pub fn export_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![export])
	}
	pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [=])
	}
	pub fn expr(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
	pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [;]) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsNamespaceExportDecl {
	pub(crate) syntax: SyntaxNode,
}
impl TsNamespaceExportDecl {
	pub fn export_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![export])
	}
	pub fn as_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![as])
	}
	pub fn namespace_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![namespace])
	}
	pub fn ident(&self) -> Option<Ident> { support::node(&self.syntax) }
	pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [;]) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsImportBareClause {
	pub(crate) syntax: SyntaxNode,
}
impl JsImportBareClause {
	pub fn source(&self) -> SyntaxResult<JsModuleSource> { support::required_node(&self.syntax) }
	pub fn assertion(&self) -> Option<JsImportAssertion> { support::node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsImportNamedClause {
	pub(crate) syntax: SyntaxNode,
}
impl JsImportNamedClause {
	pub fn default_specifier(&self) -> Option<JsDefaultImportSpecifier> {
		support::node(&self.syntax)
	}
	pub fn named_import(&self) -> SyntaxResult<JsAnyNamedImport> {
		support::required_node(&self.syntax)
	}
	pub fn from_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![from])
	}
	pub fn source(&self) -> SyntaxResult<JsModuleSource> { support::required_node(&self.syntax) }
	pub fn assertion(&self) -> Option<JsImportAssertion> { support::node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsImportDefaultClause {
	pub(crate) syntax: SyntaxNode,
}
impl JsImportDefaultClause {
	pub fn local_name(&self) -> SyntaxResult<JsAnyBinding> { support::required_node(&self.syntax) }
	pub fn from_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![from])
	}
	pub fn source(&self) -> SyntaxResult<JsModuleSource> { support::required_node(&self.syntax) }
	pub fn assertion(&self) -> Option<JsImportAssertion> { support::node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsImportNamespaceClause {
	pub(crate) syntax: SyntaxNode,
}
impl JsImportNamespaceClause {
	pub fn star_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [*])
	}
	pub fn as_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![as])
	}
	pub fn local_name(&self) -> SyntaxResult<JsAnyBinding> { support::required_node(&self.syntax) }
	pub fn from_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![from])
	}
	pub fn source(&self) -> SyntaxResult<JsModuleSource> { support::required_node(&self.syntax) }
	pub fn assertion(&self) -> Option<JsImportAssertion> { support::node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsModuleSource {
	pub(crate) syntax: SyntaxNode,
}
impl JsModuleSource {
	pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![js_string_literal])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsImportAssertion {
	pub(crate) syntax: SyntaxNode,
}
impl JsImportAssertion {
	pub fn assert_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![assert])
	}
	pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['{'])
	}
	pub fn assertions(&self) -> AstSeparatedList<JsAnyImportAssertionEntry> {
		support::separated_list(&self.syntax, 0usize)
	}
	pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['}'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsDefaultImportSpecifier {
	pub(crate) syntax: SyntaxNode,
}
impl JsDefaultImportSpecifier {
	pub fn local_name(&self) -> SyntaxResult<JsAnyBinding> { support::required_node(&self.syntax) }
	pub fn trailing_comma_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [,])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsNamedImportSpecifierList {
	pub(crate) syntax: SyntaxNode,
}
impl JsNamedImportSpecifierList {
	pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['{'])
	}
	pub fn specifiers(&self) -> AstSeparatedList<JsAnyNamedImportSpecifier> {
		support::separated_list(&self.syntax, 0usize)
	}
	pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['}'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsNamespaceImportSpecifier {
	pub(crate) syntax: SyntaxNode,
}
impl JsNamespaceImportSpecifier {
	pub fn star_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [*])
	}
	pub fn as_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![as])
	}
	pub fn local_name(&self) -> SyntaxResult<JsAnyBinding> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsShorthandNamedImportSpecifier {
	pub(crate) syntax: SyntaxNode,
}
impl JsShorthandNamedImportSpecifier {
	pub fn local_name(&self) -> SyntaxResult<JsAnyBinding> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsNamedImportSpecifier {
	pub(crate) syntax: SyntaxNode,
}
impl JsNamedImportSpecifier {
	pub fn name(&self) -> SyntaxResult<JsLiteralExportName> { support::required_node(&self.syntax) }
	pub fn as_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![as])
	}
	pub fn local_name(&self) -> SyntaxResult<JsAnyBinding> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsLiteralExportName {
	pub(crate) syntax: SyntaxNode,
}
impl JsLiteralExportName {
	pub fn value(&self) -> SyntaxResult<SyntaxToken> {
		support::find_required_token(&self.syntax, &[T![ident], T![js_string_literal]])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsImportAssertionEntry {
	pub(crate) syntax: SyntaxNode,
}
impl JsImportAssertionEntry {
	pub fn key(&self) -> SyntaxResult<SyntaxToken> {
		support::find_required_token(&self.syntax, &[T![ident], T![js_string_literal]])
	}
	pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [:])
	}
	pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![js_string_literal])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Specifier {
	pub(crate) syntax: SyntaxNode,
}
impl Specifier {
	pub fn name(&self) -> SyntaxResult<Ident> { support::required_node(&self.syntax) }
	pub fn as_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![as]) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsPrivateName {
	pub(crate) syntax: SyntaxNode,
}
impl JsPrivateName {
	pub fn hash_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [#])
	}
	pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![ident])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsRestParameter {
	pub(crate) syntax: SyntaxNode,
}
impl JsRestParameter {
	pub fn dotdotdot_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [...])
	}
	pub fn binding(&self) -> SyntaxResult<JsAnyBindingPattern> {
		support::required_node(&self.syntax)
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsExternalModuleRef {
	pub(crate) syntax: SyntaxNode,
}
impl TsExternalModuleRef {
	pub fn require_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![require])
	}
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['('])
	}
	pub fn module_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![js_string_literal])
	}
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![')'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsAny {
	pub(crate) syntax: SyntaxNode,
}
impl TsAny {
	pub fn any_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![any])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsUnknown {
	pub(crate) syntax: SyntaxNode,
}
impl TsUnknown {
	pub fn unknown_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![unknown])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsNumber {
	pub(crate) syntax: SyntaxNode,
}
impl TsNumber {
	pub fn ident(&self) -> SyntaxResult<Ident> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsObject {
	pub(crate) syntax: SyntaxNode,
}
impl TsObject {
	pub fn ident(&self) -> SyntaxResult<Ident> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsBoolean {
	pub(crate) syntax: SyntaxNode,
}
impl TsBoolean {
	pub fn ident(&self) -> SyntaxResult<Ident> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsBigint {
	pub(crate) syntax: SyntaxNode,
}
impl TsBigint {
	pub fn ident(&self) -> SyntaxResult<Ident> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsString {
	pub(crate) syntax: SyntaxNode,
}
impl TsString {
	pub fn ident(&self) -> SyntaxResult<Ident> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsSymbol {
	pub(crate) syntax: SyntaxNode,
}
impl TsSymbol {
	pub fn ident(&self) -> SyntaxResult<Ident> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsVoid {
	pub(crate) syntax: SyntaxNode,
}
impl TsVoid {
	pub fn void_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![void])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsUndefined {
	pub(crate) syntax: SyntaxNode,
}
impl TsUndefined {
	pub fn undefined_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![undefined])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsNull {
	pub(crate) syntax: SyntaxNode,
}
impl TsNull {
	pub fn null_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![null])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsNever {
	pub(crate) syntax: SyntaxNode,
}
impl TsNever {
	pub fn never_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![never])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsThis {
	pub(crate) syntax: SyntaxNode,
}
impl TsThis {
	pub fn this_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![this])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsLiteral {
	pub(crate) syntax: SyntaxNode,
}
impl TsLiteral {
	pub fn ident(&self) -> SyntaxResult<Ident> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsPredicate {
	pub(crate) syntax: SyntaxNode,
}
impl TsPredicate {
	pub fn lhs(&self) -> SyntaxResult<TsThisOrMore> { support::required_node(&self.syntax) }
	pub fn rhs(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsTuple {
	pub(crate) syntax: SyntaxNode,
}
impl TsTuple {
	pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['['])
	}
	pub fn elements(&self) -> SyntaxResult<TsTupleElement> { support::required_node(&self.syntax) }
	pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![']'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsParen {
	pub(crate) syntax: SyntaxNode,
}
impl TsParen {
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['('])
	}
	pub fn ty(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax) }
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![')'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsTypeRef {
	pub(crate) syntax: SyntaxNode,
}
impl TsTypeRef {
	pub fn name(&self) -> SyntaxResult<TsEntityName> { support::required_node(&self.syntax) }
	pub fn type_args(&self) -> SyntaxResult<TsTypeArgs> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsTemplate {
	pub(crate) syntax: SyntaxNode,
}
impl TsTemplate {
	pub fn elements(&self) -> SyntaxResult<TsTemplateElement> {
		support::required_node(&self.syntax)
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsMappedType {
	pub(crate) syntax: SyntaxNode,
}
impl TsMappedType {
	pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['{'])
	}
	pub fn readonly_modifier(&self) -> Option<TsMappedTypeReadonly> { support::node(&self.syntax) }
	pub fn minus_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [-]) }
	pub fn plus_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [+]) }
	pub fn question_mark_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T ! [?])
	}
	pub fn param(&self) -> SyntaxResult<TsMappedTypeParam> { support::required_node(&self.syntax) }
	pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [:])
	}
	pub fn ty(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax) }
	pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['}'])
	}
	pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [;]) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsImport {
	pub(crate) syntax: SyntaxNode,
}
impl TsImport {
	pub fn import_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![import])
	}
	pub fn type_args(&self) -> SyntaxResult<TsTypeArgs> { support::required_node(&self.syntax) }
	pub fn dot_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [.]) }
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['('])
	}
	pub fn qualifier(&self) -> SyntaxResult<TsEntityName> { support::required_node(&self.syntax) }
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![')'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsArray {
	pub(crate) syntax: SyntaxNode,
}
impl TsArray {
	pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['['])
	}
	pub fn ty(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax) }
	pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![']'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsIndexedArray {
	pub(crate) syntax: SyntaxNode,
}
impl TsIndexedArray {
	pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['['])
	}
	pub fn ty(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax) }
	pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![']'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsTypeOperator {
	pub(crate) syntax: SyntaxNode,
}
impl TsTypeOperator {
	pub fn ty(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsIntersection {
	pub(crate) syntax: SyntaxNode,
}
impl TsIntersection {
	pub fn types(&self) -> AstNodeList<TsType> { support::node_list(&self.syntax, 0usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsUnion {
	pub(crate) syntax: SyntaxNode,
}
impl TsUnion {
	pub fn types(&self) -> AstNodeList<TsType> { support::node_list(&self.syntax, 0usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsFnType {
	pub(crate) syntax: SyntaxNode,
}
impl TsFnType {
	pub fn params(&self) -> SyntaxResult<JsParameterList> { support::required_node(&self.syntax) }
	pub fn fat_arrow_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [=>])
	}
	pub fn return_type(&self) -> Option<TsType> { support::node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsConstructorType {
	pub(crate) syntax: SyntaxNode,
}
impl TsConstructorType {
	pub fn new_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![new])
	}
	pub fn params(&self) -> SyntaxResult<JsParameterList> { support::required_node(&self.syntax) }
	pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [:])
	}
	pub fn return_type(&self) -> Option<TsType> { support::node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsConditionalType {
	pub(crate) syntax: SyntaxNode,
}
impl TsConditionalType {
	pub fn ty(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax) }
	pub fn question_mark_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [?])
	}
	pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [:])
	}
	pub fn extends(&self) -> SyntaxResult<TsExtends> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsObjectType {
	pub(crate) syntax: SyntaxNode,
}
impl TsObjectType {
	pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['{'])
	}
	pub fn members(&self) -> AstNodeList<TsTypeElement> { support::node_list(&self.syntax, 0usize) }
	pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['}'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsInfer {
	pub(crate) syntax: SyntaxNode,
}
impl TsInfer {
	pub fn infer_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![infer])
	}
	pub fn ident(&self) -> SyntaxResult<Ident> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsTupleElement {
	pub(crate) syntax: SyntaxNode,
}
impl TsTupleElement {
	pub fn ident(&self) -> SyntaxResult<Ident> { support::required_node(&self.syntax) }
	pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [:])
	}
	pub fn question_mark_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [?])
	}
	pub fn dotdotdot_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [...]) }
	pub fn ty(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsEnumMember {
	pub(crate) syntax: SyntaxNode,
}
impl TsEnumMember {
	pub fn ident(&self) -> SyntaxResult<Ident> { support::required_node(&self.syntax) }
	pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [=])
	}
	pub fn value(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsTemplateElement {
	pub(crate) syntax: SyntaxNode,
}
impl TsTemplateElement {
	pub fn ty(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax) }
	pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['}'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsMappedTypeReadonly {
	pub(crate) syntax: SyntaxNode,
}
impl TsMappedTypeReadonly {
	pub fn minus_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [-]) }
	pub fn plus_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [+]) }
	pub fn readonly_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![readonly])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsMappedTypeParam {
	pub(crate) syntax: SyntaxNode,
}
impl TsMappedTypeParam {
	pub fn l_brack_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['[']) }
	pub fn name(&self) -> Option<TsTypeName> { support::node(&self.syntax) }
	pub fn r_brack_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![']']) }
	pub fn ident(&self) -> Option<Ident> { support::node(&self.syntax) }
	pub fn ty(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsTypeName {
	pub(crate) syntax: SyntaxNode,
}
impl TsTypeName {
	pub fn ident(&self) -> SyntaxResult<Ident> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsExtends {
	pub(crate) syntax: SyntaxNode,
}
impl TsExtends {
	pub fn extends_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![extends])
	}
	pub fn ty(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsModuleBlock {
	pub(crate) syntax: SyntaxNode,
}
impl TsModuleBlock {
	pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['{'])
	}
	pub fn items(&self) -> SyntaxResult<JsAnyStatement> { support::required_node(&self.syntax) }
	pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['}'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsTypeParam {
	pub(crate) syntax: SyntaxNode,
}
impl TsTypeParam {
	pub fn ident(&self) -> SyntaxResult<Ident> { support::required_node(&self.syntax) }
	pub fn constraint(&self) -> SyntaxResult<TsConstraint> { support::required_node(&self.syntax) }
	pub fn default(&self) -> SyntaxResult<TsDefault> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsConstraint {
	pub(crate) syntax: SyntaxNode,
}
impl TsConstraint {
	pub fn extends_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![extends])
	}
	pub fn ty(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsDefault {
	pub(crate) syntax: SyntaxNode,
}
impl TsDefault {
	pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [=])
	}
	pub fn ty(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsCallSignatureDecl {
	pub(crate) syntax: SyntaxNode,
}
impl TsCallSignatureDecl {
	pub fn type_params(&self) -> SyntaxResult<TsTypeParams> { support::required_node(&self.syntax) }
	pub fn parameters(&self) -> SyntaxResult<JsParameterList> {
		support::required_node(&self.syntax)
	}
	pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [:])
	}
	pub fn return_type(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsConstructSignatureDecl {
	pub(crate) syntax: SyntaxNode,
}
impl TsConstructSignatureDecl {
	pub fn new_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![new])
	}
	pub fn type_params(&self) -> SyntaxResult<TsTypeParams> { support::required_node(&self.syntax) }
	pub fn parameters(&self) -> SyntaxResult<JsParameterList> {
		support::required_node(&self.syntax)
	}
	pub fn colon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [:]) }
	pub fn return_type(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsPropertySignature {
	pub(crate) syntax: SyntaxNode,
}
impl TsPropertySignature {
	pub fn readonly_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![readonly])
	}
	pub fn prop(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
	pub fn question_mark_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [?])
	}
	pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [:])
	}
	pub fn ty(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsMethodSignature {
	pub(crate) syntax: SyntaxNode,
}
impl TsMethodSignature {
	pub fn readonly_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![readonly])
	}
	pub fn key(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
	pub fn type_params(&self) -> SyntaxResult<TsTypeParams> { support::required_node(&self.syntax) }
	pub fn parameters(&self) -> SyntaxResult<JsParameterList> {
		support::required_node(&self.syntax)
	}
	pub fn question_mark_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T ! [?])
	}
	pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [:])
	}
	pub fn return_type(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsQualifiedPath {
	pub(crate) syntax: SyntaxNode,
}
impl TsQualifiedPath {
	pub fn lhs(&self) -> SyntaxResult<TsEntityName> { support::required_node(&self.syntax) }
	pub fn dot_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [.])
	}
	pub fn rhs(&self) -> SyntaxResult<TsTypeName> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyRoot {
	JsScript(JsScript),
	JsModule(JsModule),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyStatement {
	JsBlockStatement(JsBlockStatement),
	JsEmptyStatement(JsEmptyStatement),
	JsExpressionStatement(JsExpressionStatement),
	JsIfStatement(JsIfStatement),
	JsDoWhileStatement(JsDoWhileStatement),
	JsWhileStatement(JsWhileStatement),
	ForStmt(ForStmt),
	ForInStmt(ForInStmt),
	ForOfStmt(ForOfStmt),
	JsContinueStatement(JsContinueStatement),
	JsBreakStatement(JsBreakStatement),
	JsReturnStatement(JsReturnStatement),
	JsWithStatement(JsWithStatement),
	JsLabeledStatement(JsLabeledStatement),
	JsSwitchStatement(JsSwitchStatement),
	JsThrowStatement(JsThrowStatement),
	JsTryStatement(JsTryStatement),
	JsTryFinallyStatement(JsTryFinallyStatement),
	JsDebuggerStatement(JsDebuggerStatement),
	JsFunctionDeclaration(JsFunctionDeclaration),
	JsClassDeclaration(JsClassDeclaration),
	JsVariableDeclarationStatement(JsVariableDeclarationStatement),
	TsEnum(TsEnum),
	TsTypeAliasDecl(TsTypeAliasDecl),
	TsNamespaceDecl(TsNamespaceDecl),
	TsModuleDecl(TsModuleDecl),
	TsInterfaceDecl(TsInterfaceDecl),
	JsUnknownStatement(JsUnknownStatement),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyModuleItem {
	JsAnyStatement(JsAnyStatement),
	JsImport(JsImport),
	ExportNamed(ExportNamed),
	ExportDefaultDecl(ExportDefaultDecl),
	ExportDefaultExpr(ExportDefaultExpr),
	ExportWildcard(ExportWildcard),
	ExportDecl(ExportDecl),
	TsImportEqualsDecl(TsImportEqualsDecl),
	TsExportAssignment(TsExportAssignment),
	TsNamespaceExportDecl(TsNamespaceExportDecl),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyExpression {
	JsAnyLiteralExpression(JsAnyLiteralExpression),
	JsArrayExpression(JsArrayExpression),
	JsArrowFunctionExpression(JsArrowFunctionExpression),
	JsAssignmentExpression(JsAssignmentExpression),
	JsAwaitExpression(JsAwaitExpression),
	JsBinaryExpression(JsBinaryExpression),
	JsClassExpression(JsClassExpression),
	JsConditionalExpression(JsConditionalExpression),
	JsComputedMemberExpression(JsComputedMemberExpression),
	JsFunctionExpression(JsFunctionExpression),
	JsImportCallExpression(JsImportCallExpression),
	JsLogicalExpression(JsLogicalExpression),
	JsObjectExpression(JsObjectExpression),
	JsParenthesizedExpression(JsParenthesizedExpression),
	JsIdentifierExpression(JsIdentifierExpression),
	JsSequenceExpression(JsSequenceExpression),
	JsStaticMemberExpression(JsStaticMemberExpression),
	JsSuperExpression(JsSuperExpression),
	JsThisExpression(JsThisExpression),
	JsUnaryExpression(JsUnaryExpression),
	JsPreUpdateExpression(JsPreUpdateExpression),
	JsPostUpdateExpression(JsPostUpdateExpression),
	JsYieldExpression(JsYieldExpression),
	Template(Template),
	NewExpr(NewExpr),
	CallExpr(CallExpr),
	NewTarget(NewTarget),
	ImportMeta(ImportMeta),
	TsNonNull(TsNonNull),
	TsAssertion(TsAssertion),
	TsConstAssertion(TsConstAssertion),
	JsUnknownExpression(JsUnknownExpression),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum ForHead {
	JsVariableDeclaration(JsVariableDeclaration),
	JsAnyExpression(JsAnyExpression),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum ForLeft {
	JsAnyAssignment(JsAnyAssignment),
	JsVariableDeclaration(JsVariableDeclaration),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyAssignment {
	JsIdentifierAssignment(JsIdentifierAssignment),
	JsStaticMemberAssignment(JsStaticMemberAssignment),
	JsComputedMemberAssignment(JsComputedMemberAssignment),
	JsParenthesizedAssignment(JsParenthesizedAssignment),
	JsUnknownAssignment(JsUnknownAssignment),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnySwitchClause {
	JsCaseClause(JsCaseClause),
	JsDefaultClause(JsDefaultClause),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyBindingPattern {
	JsAnyBinding(JsAnyBinding),
	JsArrayBindingPattern(JsArrayBindingPattern),
	JsObjectBindingPattern(JsObjectBindingPattern),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyLiteralExpression {
	JsStringLiteralExpression(JsStringLiteralExpression),
	JsNumberLiteralExpression(JsNumberLiteralExpression),
	JsBigIntLiteralExpression(JsBigIntLiteralExpression),
	JsBooleanLiteralExpression(JsBooleanLiteralExpression),
	JsNullLiteralExpression(JsNullLiteralExpression),
	JsRegexLiteralExpression(JsRegexLiteralExpression),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyBinding {
	JsIdentifierBinding(JsIdentifierBinding),
	JsUnknownBinding(JsUnknownBinding),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyArrowFunctionParameters {
	JsParameterList(JsParameterList),
	JsAnyBinding(JsAnyBinding),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyArrowFunctionBody {
	JsAnyExpression(JsAnyExpression),
	JsFunctionBody(JsFunctionBody),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyArrayElement {
	JsAnyExpression(JsAnyExpression),
	JsSpread(JsSpread),
	JsArrayHole(JsArrayHole),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyAssignmentPattern {
	JsAnyAssignment(JsAnyAssignment),
	JsArrayAssignmentPattern(JsArrayAssignmentPattern),
	JsObjectAssignmentPattern(JsObjectAssignmentPattern),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyName {
	JsName(JsName),
	JsPrivateName(JsPrivateName),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyObjectMemberName {
	JsLiteralMemberName(JsLiteralMemberName),
	JsComputedMemberName(JsComputedMemberName),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyObjectMember {
	JsPropertyObjectMember(JsPropertyObjectMember),
	JsMethodObjectMember(JsMethodObjectMember),
	JsGetterObjectMember(JsGetterObjectMember),
	JsSetterObjectMember(JsSetterObjectMember),
	JsShorthandPropertyObjectMember(JsShorthandPropertyObjectMember),
	JsSpread(JsSpread),
	JsUnknownMember(JsUnknownMember),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyClassMember {
	JsConstructorClassMember(JsConstructorClassMember),
	JsPropertyClassMember(JsPropertyClassMember),
	JsMethodClassMember(JsMethodClassMember),
	JsGetterClassMember(JsGetterClassMember),
	JsSetterClassMember(JsSetterClassMember),
	JsEmptyClassMember(JsEmptyClassMember),
	TsIndexSignature(TsIndexSignature),
	JsUnknownMember(JsUnknownMember),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyClassMemberName {
	JsLiteralMemberName(JsLiteralMemberName),
	JsComputedMemberName(JsComputedMemberName),
	JsPrivateClassMemberName(JsPrivateClassMemberName),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyConstructorParameter {
	TsConstructorParam(TsConstructorParam),
	JsAnyBindingPattern(JsAnyBindingPattern),
	JsBindingPatternWithDefault(JsBindingPatternWithDefault),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyModifier {
	JsModifier(JsModifier),
	JsUnknownModifier(JsUnknownModifier),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyArrayAssignmentPatternElement {
	JsAssignmentWithDefault(JsAssignmentWithDefault),
	JsAnyAssignmentPattern(JsAnyAssignmentPattern),
	JsArrayAssignmentPatternRestElement(JsArrayAssignmentPatternRestElement),
	JsArrayHole(JsArrayHole),
	JsUnknownAssignment(JsUnknownAssignment),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyObjectAssignmentPatternMember {
	JsObjectAssignmentPatternShorthandProperty(JsObjectAssignmentPatternShorthandProperty),
	JsObjectAssignmentPatternProperty(JsObjectAssignmentPatternProperty),
	JsObjectAssignmentPatternRest(JsObjectAssignmentPatternRest),
	JsUnknownAssignment(JsUnknownAssignment),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyArrayBindingPatternElement {
	JsArrayHole(JsArrayHole),
	JsAnyBindingPattern(JsAnyBindingPattern),
	JsBindingPatternWithDefault(JsBindingPatternWithDefault),
	JsArrayBindingPatternRestElement(JsArrayBindingPatternRestElement),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyObjectBindingPatternMember {
	JsObjectBindingPatternProperty(JsObjectBindingPatternProperty),
	JsObjectBindingPatternRest(JsObjectBindingPatternRest),
	JsObjectBindingPatternShorthandProperty(JsObjectBindingPatternShorthandProperty),
	JsIdentifierBinding(JsIdentifierBinding),
	JsUnknownBinding(JsUnknownBinding),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum TsType {
	TsAny(TsAny),
	TsUnknown(TsUnknown),
	TsNumber(TsNumber),
	TsObject(TsObject),
	TsBoolean(TsBoolean),
	TsBigint(TsBigint),
	TsString(TsString),
	TsSymbol(TsSymbol),
	TsVoid(TsVoid),
	TsUndefined(TsUndefined),
	TsNull(TsNull),
	TsNever(TsNever),
	TsThis(TsThis),
	TsLiteral(TsLiteral),
	TsPredicate(TsPredicate),
	TsTuple(TsTuple),
	TsParen(TsParen),
	TsTypeRef(TsTypeRef),
	TsTemplate(TsTemplate),
	TsMappedType(TsMappedType),
	TsImport(TsImport),
	TsArray(TsArray),
	TsIndexedArray(TsIndexedArray),
	TsTypeOperator(TsTypeOperator),
	TsIntersection(TsIntersection),
	TsUnion(TsUnion),
	TsFnType(TsFnType),
	TsConstructorType(TsConstructorType),
	TsConditionalType(TsConditionalType),
	TsObjectType(TsObjectType),
	TsInfer(TsInfer),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum AnyJsImportClause {
	JsImportBareClause(JsImportBareClause),
	JsName(JsName),
	JsImportNamedClause(JsImportNamedClause),
	JsImportDefaultClause(JsImportDefaultClause),
	JsImportNamespaceClause(JsImportNamespaceClause),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyNamedImport {
	JsNamedImportSpecifierList(JsNamedImportSpecifierList),
	JsNamespaceImportSpecifier(JsNamespaceImportSpecifier),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyNamedImportSpecifier {
	JsShorthandNamedImportSpecifier(JsShorthandNamedImportSpecifier),
	JsNamedImportSpecifier(JsNamedImportSpecifier),
	JsUnknownNamedImportSpecifier(JsUnknownNamedImportSpecifier),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyImportAssertionEntry {
	JsImportAssertionEntry(JsImportAssertionEntry),
	JsUnknownImportAssertionEntry(JsUnknownImportAssertionEntry),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum DefaultDecl {
	JsFunctionDeclaration(JsFunctionDeclaration),
	JsClassDeclaration(JsClassDeclaration),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyExportDeclaration {
	JsFunctionDeclaration(JsFunctionDeclaration),
	JsClassDeclaration(JsClassDeclaration),
	JsVariableDeclarationStatement(JsVariableDeclarationStatement),
	TsEnum(TsEnum),
	TsTypeAliasDecl(TsTypeAliasDecl),
	TsNamespaceDecl(TsNamespaceDecl),
	TsModuleDecl(TsModuleDecl),
	TsInterfaceDecl(TsInterfaceDecl),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyParameter {
	JsAnyBindingPattern(JsAnyBindingPattern),
	JsBindingPatternWithDefault(JsBindingPatternWithDefault),
	JsRestParameter(JsRestParameter),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum TsModuleRef {
	TsExternalModuleRef(TsExternalModuleRef),
	TsEntityName(TsEntityName),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum TsEntityName {
	TsTypeName(TsTypeName),
	TsQualifiedPath(TsQualifiedPath),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum TsThisOrMore {
	TsThis(TsThis),
	TsTypeName(TsTypeName),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum TsTypeElement {
	TsCallSignatureDecl(TsCallSignatureDecl),
	TsConstructSignatureDecl(TsConstructSignatureDecl),
	TsPropertySignature(TsPropertySignature),
	TsMethodSignature(TsMethodSignature),
	TsIndexSignature(TsIndexSignature),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum TsNamespaceBody {
	TsModuleBlock(TsModuleBlock),
	TsNamespaceDecl(TsNamespaceDecl),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum AnyNode {
	JsUnknownStatement(JsUnknownStatement),
	JsUnknownExpression(JsUnknownExpression),
	JsUnknownMember(JsUnknownMember),
	JsUnknownBinding(JsUnknownBinding),
	JsUnknownAssignment(JsUnknownAssignment),
	JsUnknownModifier(JsUnknownModifier),
	JsUnknownImportAssertionEntry(JsUnknownImportAssertionEntry),
	JsUnknownNamedImportSpecifier(JsUnknownNamedImportSpecifier),
	List(List),
	Ident(Ident),
	JsScript(JsScript),
	JsModule(JsModule),
	JsDirective(JsDirective),
	JsBlockStatement(JsBlockStatement),
	JsEmptyStatement(JsEmptyStatement),
	JsExpressionStatement(JsExpressionStatement),
	JsIfStatement(JsIfStatement),
	JsDoWhileStatement(JsDoWhileStatement),
	JsWhileStatement(JsWhileStatement),
	ForStmt(ForStmt),
	ForInStmt(ForInStmt),
	ForOfStmt(ForOfStmt),
	JsContinueStatement(JsContinueStatement),
	JsBreakStatement(JsBreakStatement),
	JsReturnStatement(JsReturnStatement),
	JsWithStatement(JsWithStatement),
	JsLabeledStatement(JsLabeledStatement),
	JsSwitchStatement(JsSwitchStatement),
	JsThrowStatement(JsThrowStatement),
	JsTryStatement(JsTryStatement),
	JsTryFinallyStatement(JsTryFinallyStatement),
	JsDebuggerStatement(JsDebuggerStatement),
	JsFunctionDeclaration(JsFunctionDeclaration),
	JsClassDeclaration(JsClassDeclaration),
	JsVariableDeclarationStatement(JsVariableDeclarationStatement),
	TsEnum(TsEnum),
	TsTypeAliasDecl(TsTypeAliasDecl),
	TsNamespaceDecl(TsNamespaceDecl),
	TsModuleDecl(TsModuleDecl),
	TsInterfaceDecl(TsInterfaceDecl),
	JsElseClause(JsElseClause),
	ForStmtInit(ForStmtInit),
	ForStmtTest(ForStmtTest),
	ForStmtUpdate(ForStmtUpdate),
	JsVariableDeclaration(JsVariableDeclaration),
	JsCaseClause(JsCaseClause),
	JsDefaultClause(JsDefaultClause),
	JsCatchClause(JsCatchClause),
	JsFinallyClause(JsFinallyClause),
	JsCatchDeclaration(JsCatchDeclaration),
	JsArrayExpression(JsArrayExpression),
	JsArrowFunctionExpression(JsArrowFunctionExpression),
	JsAssignmentExpression(JsAssignmentExpression),
	JsAwaitExpression(JsAwaitExpression),
	JsBinaryExpression(JsBinaryExpression),
	JsClassExpression(JsClassExpression),
	JsConditionalExpression(JsConditionalExpression),
	JsComputedMemberExpression(JsComputedMemberExpression),
	JsFunctionExpression(JsFunctionExpression),
	JsImportCallExpression(JsImportCallExpression),
	JsLogicalExpression(JsLogicalExpression),
	JsObjectExpression(JsObjectExpression),
	JsParenthesizedExpression(JsParenthesizedExpression),
	JsIdentifierExpression(JsIdentifierExpression),
	JsSequenceExpression(JsSequenceExpression),
	JsStaticMemberExpression(JsStaticMemberExpression),
	JsSuperExpression(JsSuperExpression),
	JsThisExpression(JsThisExpression),
	JsUnaryExpression(JsUnaryExpression),
	JsPreUpdateExpression(JsPreUpdateExpression),
	JsPostUpdateExpression(JsPostUpdateExpression),
	JsYieldExpression(JsYieldExpression),
	Template(Template),
	NewExpr(NewExpr),
	CallExpr(CallExpr),
	NewTarget(NewTarget),
	ImportMeta(ImportMeta),
	TsNonNull(TsNonNull),
	TsAssertion(TsAssertion),
	TsConstAssertion(TsConstAssertion),
	TsTypeArgs(TsTypeArgs),
	ArgList(ArgList),
	TsTypeParams(TsTypeParams),
	JsParameterList(JsParameterList),
	TsTypeAnnotation(TsTypeAnnotation),
	JsFunctionBody(JsFunctionBody),
	JsSpread(JsSpread),
	JsArrayHole(JsArrayHole),
	JsReferenceIdentifier(JsReferenceIdentifier),
	JsLiteralMemberName(JsLiteralMemberName),
	JsComputedMemberName(JsComputedMemberName),
	JsPropertyObjectMember(JsPropertyObjectMember),
	JsMethodObjectMember(JsMethodObjectMember),
	JsGetterObjectMember(JsGetterObjectMember),
	JsSetterObjectMember(JsSetterObjectMember),
	JsShorthandPropertyObjectMember(JsShorthandPropertyObjectMember),
	TsImplementsClause(TsImplementsClause),
	JsExtendsClause(JsExtendsClause),
	TsExprWithTypeArgs(TsExprWithTypeArgs),
	JsPrivateClassMemberName(JsPrivateClassMemberName),
	JsConstructorClassMember(JsConstructorClassMember),
	JsPropertyClassMember(JsPropertyClassMember),
	JsMethodClassMember(JsMethodClassMember),
	JsGetterClassMember(JsGetterClassMember),
	JsSetterClassMember(JsSetterClassMember),
	JsEmptyClassMember(JsEmptyClassMember),
	TsIndexSignature(TsIndexSignature),
	TsAccessibility(TsAccessibility),
	JsConstructorParameterList(JsConstructorParameterList),
	TsConstructorParam(TsConstructorParam),
	JsBindingPatternWithDefault(JsBindingPatternWithDefault),
	JsEqualValueClause(JsEqualValueClause),
	JsModifier(JsModifier),
	JsIdentifierAssignment(JsIdentifierAssignment),
	JsStaticMemberAssignment(JsStaticMemberAssignment),
	JsComputedMemberAssignment(JsComputedMemberAssignment),
	JsParenthesizedAssignment(JsParenthesizedAssignment),
	JsAssignmentWithDefault(JsAssignmentWithDefault),
	JsArrayAssignmentPattern(JsArrayAssignmentPattern),
	JsObjectAssignmentPattern(JsObjectAssignmentPattern),
	JsArrayAssignmentPatternRestElement(JsArrayAssignmentPatternRestElement),
	JsObjectAssignmentPatternShorthandProperty(JsObjectAssignmentPatternShorthandProperty),
	JsObjectAssignmentPatternProperty(JsObjectAssignmentPatternProperty),
	JsObjectAssignmentPatternRest(JsObjectAssignmentPatternRest),
	JsName(JsName),
	JsIdentifierBinding(JsIdentifierBinding),
	JsArrayBindingPattern(JsArrayBindingPattern),
	JsObjectBindingPattern(JsObjectBindingPattern),
	JsArrayBindingPatternRestElement(JsArrayBindingPatternRestElement),
	JsObjectBindingPatternProperty(JsObjectBindingPatternProperty),
	JsObjectBindingPatternRest(JsObjectBindingPatternRest),
	JsObjectBindingPatternShorthandProperty(JsObjectBindingPatternShorthandProperty),
	JsStringLiteralExpression(JsStringLiteralExpression),
	JsNumberLiteralExpression(JsNumberLiteralExpression),
	JsBigIntLiteralExpression(JsBigIntLiteralExpression),
	JsBooleanLiteralExpression(JsBooleanLiteralExpression),
	JsNullLiteralExpression(JsNullLiteralExpression),
	JsRegexLiteralExpression(JsRegexLiteralExpression),
	JsVariableDeclarator(JsVariableDeclarator),
	JsImport(JsImport),
	ExportNamed(ExportNamed),
	ExportDefaultDecl(ExportDefaultDecl),
	ExportDefaultExpr(ExportDefaultExpr),
	ExportWildcard(ExportWildcard),
	ExportDecl(ExportDecl),
	TsImportEqualsDecl(TsImportEqualsDecl),
	TsExportAssignment(TsExportAssignment),
	TsNamespaceExportDecl(TsNamespaceExportDecl),
	JsImportBareClause(JsImportBareClause),
	JsImportNamedClause(JsImportNamedClause),
	JsImportDefaultClause(JsImportDefaultClause),
	JsImportNamespaceClause(JsImportNamespaceClause),
	JsModuleSource(JsModuleSource),
	JsImportAssertion(JsImportAssertion),
	JsDefaultImportSpecifier(JsDefaultImportSpecifier),
	JsNamedImportSpecifierList(JsNamedImportSpecifierList),
	JsNamespaceImportSpecifier(JsNamespaceImportSpecifier),
	JsShorthandNamedImportSpecifier(JsShorthandNamedImportSpecifier),
	JsNamedImportSpecifier(JsNamedImportSpecifier),
	JsLiteralExportName(JsLiteralExportName),
	JsImportAssertionEntry(JsImportAssertionEntry),
	Specifier(Specifier),
	JsPrivateName(JsPrivateName),
	JsRestParameter(JsRestParameter),
	TsExternalModuleRef(TsExternalModuleRef),
	TsAny(TsAny),
	TsUnknown(TsUnknown),
	TsNumber(TsNumber),
	TsObject(TsObject),
	TsBoolean(TsBoolean),
	TsBigint(TsBigint),
	TsString(TsString),
	TsSymbol(TsSymbol),
	TsVoid(TsVoid),
	TsUndefined(TsUndefined),
	TsNull(TsNull),
	TsNever(TsNever),
	TsThis(TsThis),
	TsLiteral(TsLiteral),
	TsPredicate(TsPredicate),
	TsTuple(TsTuple),
	TsParen(TsParen),
	TsTypeRef(TsTypeRef),
	TsTemplate(TsTemplate),
	TsMappedType(TsMappedType),
	TsImport(TsImport),
	TsArray(TsArray),
	TsIndexedArray(TsIndexedArray),
	TsTypeOperator(TsTypeOperator),
	TsIntersection(TsIntersection),
	TsUnion(TsUnion),
	TsFnType(TsFnType),
	TsConstructorType(TsConstructorType),
	TsConditionalType(TsConditionalType),
	TsObjectType(TsObjectType),
	TsInfer(TsInfer),
	TsTupleElement(TsTupleElement),
	TsEnumMember(TsEnumMember),
	TsTemplateElement(TsTemplateElement),
	TsMappedTypeReadonly(TsMappedTypeReadonly),
	TsMappedTypeParam(TsMappedTypeParam),
	TsTypeName(TsTypeName),
	TsExtends(TsExtends),
	TsModuleBlock(TsModuleBlock),
	TsTypeParam(TsTypeParam),
	TsConstraint(TsConstraint),
	TsDefault(TsDefault),
	TsCallSignatureDecl(TsCallSignatureDecl),
	TsConstructSignatureDecl(TsConstructSignatureDecl),
	TsPropertySignature(TsPropertySignature),
	TsMethodSignature(TsMethodSignature),
	TsQualifiedPath(TsQualifiedPath),
}
impl AstNode for JsUnknownStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_UNKNOWN_STATEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsUnknownStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsUnknownStatement")
			.field("items", &support::DebugSyntaxElementChildren(self.items()))
			.finish()
	}
}
impl AstNode for JsUnknownExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_UNKNOWN_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsUnknownExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsUnknownExpression")
			.field("items", &support::DebugSyntaxElementChildren(self.items()))
			.finish()
	}
}
impl AstNode for JsUnknownMember {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_UNKNOWN_MEMBER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsUnknownMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsUnknownMember")
			.field("items", &support::DebugSyntaxElementChildren(self.items()))
			.finish()
	}
}
impl AstNode for JsUnknownBinding {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_UNKNOWN_BINDING }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsUnknownBinding {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsUnknownBinding")
			.field("items", &support::DebugSyntaxElementChildren(self.items()))
			.finish()
	}
}
impl AstNode for JsUnknownAssignment {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_UNKNOWN_ASSIGNMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsUnknownAssignment {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsUnknownAssignment")
			.field("items", &support::DebugSyntaxElementChildren(self.items()))
			.finish()
	}
}
impl AstNode for JsUnknownModifier {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_UNKNOWN_MODIFIER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsUnknownModifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsUnknownModifier")
			.field("items", &support::DebugSyntaxElementChildren(self.items()))
			.finish()
	}
}
impl AstNode for JsUnknownImportAssertionEntry {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_UNKNOWN_IMPORT_ASSERTION_ENTRY }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsUnknownImportAssertionEntry {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsUnknownImportAssertionEntry")
			.field("items", &support::DebugSyntaxElementChildren(self.items()))
			.finish()
	}
}
impl AstNode for JsUnknownNamedImportSpecifier {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_UNKNOWN_NAMED_IMPORT_SPECIFIER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsUnknownNamedImportSpecifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsUnknownNamedImportSpecifier")
			.field("items", &support::DebugSyntaxElementChildren(self.items()))
			.finish()
	}
}
impl AstNode for List {
	fn can_cast(kind: SyntaxKind) -> bool { kind == LIST }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for List {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("List")
			.field("items", &support::DebugSyntaxElementChildren(self.items()))
			.finish()
	}
}
impl AstNode for Ident {
	fn can_cast(kind: SyntaxKind) -> bool { kind == IDENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for Ident {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Ident")
			.field(
				"ident_token",
				&support::DebugSyntaxResult(self.ident_token()),
			)
			.finish()
	}
}
impl AstNode for JsScript {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_SCRIPT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsScript {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsScript")
			.field(
				"interpreter_token",
				&support::DebugOptionalElement(self.interpreter_token()),
			)
			.field("directives", &self.directives())
			.field("statements", &self.statements())
			.finish()
	}
}
impl AstNode for JsModule {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_MODULE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsModule {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsModule")
			.field(
				"interpreter_token",
				&support::DebugOptionalElement(self.interpreter_token()),
			)
			.field("directives", &self.directives())
			.field("items", &self.items())
			.finish()
	}
}
impl AstNode for JsDirective {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_DIRECTIVE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsDirective {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsDirective")
			.field(
				"value_token",
				&support::DebugSyntaxResult(self.value_token()),
			)
			.field(
				"semicolon_token",
				&support::DebugOptionalElement(self.semicolon_token()),
			)
			.finish()
	}
}
impl AstNode for JsBlockStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_BLOCK_STATEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsBlockStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsBlockStatement")
			.field(
				"l_curly_token",
				&support::DebugSyntaxResult(self.l_curly_token()),
			)
			.field("statements", &self.statements())
			.field(
				"r_curly_token",
				&support::DebugSyntaxResult(self.r_curly_token()),
			)
			.finish()
	}
}
impl AstNode for JsEmptyStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_EMPTY_STATEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsEmptyStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsEmptyStatement")
			.field(
				"semicolon_token",
				&support::DebugSyntaxResult(self.semicolon_token()),
			)
			.finish()
	}
}
impl AstNode for JsExpressionStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_EXPRESSION_STATEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsExpressionStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsExpressionStatement")
			.field("expression", &support::DebugSyntaxResult(self.expression()))
			.field(
				"semicolon_token",
				&support::DebugOptionalElement(self.semicolon_token()),
			)
			.finish()
	}
}
impl AstNode for JsIfStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_IF_STATEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsIfStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsIfStatement")
			.field("if_token", &support::DebugSyntaxResult(self.if_token()))
			.field(
				"l_paren_token",
				&support::DebugSyntaxResult(self.l_paren_token()),
			)
			.field("test", &support::DebugSyntaxResult(self.test()))
			.field(
				"r_paren_token",
				&support::DebugSyntaxResult(self.r_paren_token()),
			)
			.field("consequent", &support::DebugSyntaxResult(self.consequent()))
			.field(
				"else_clause",
				&support::DebugOptionalElement(self.else_clause()),
			)
			.finish()
	}
}
impl AstNode for JsDoWhileStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_DO_WHILE_STATEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsDoWhileStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsDoWhileStatement")
			.field("do_token", &support::DebugSyntaxResult(self.do_token()))
			.field("body", &support::DebugSyntaxResult(self.body()))
			.field(
				"while_token",
				&support::DebugSyntaxResult(self.while_token()),
			)
			.field(
				"l_paren_token",
				&support::DebugSyntaxResult(self.l_paren_token()),
			)
			.field("test", &support::DebugSyntaxResult(self.test()))
			.field(
				"r_paren_token",
				&support::DebugSyntaxResult(self.r_paren_token()),
			)
			.field(
				"semicolon_token",
				&support::DebugOptionalElement(self.semicolon_token()),
			)
			.finish()
	}
}
impl AstNode for JsWhileStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_WHILE_STATEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsWhileStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsWhileStatement")
			.field(
				"while_token",
				&support::DebugSyntaxResult(self.while_token()),
			)
			.field(
				"l_paren_token",
				&support::DebugSyntaxResult(self.l_paren_token()),
			)
			.field("test", &support::DebugSyntaxResult(self.test()))
			.field(
				"r_paren_token",
				&support::DebugSyntaxResult(self.r_paren_token()),
			)
			.field("body", &support::DebugSyntaxResult(self.body()))
			.finish()
	}
}
impl AstNode for ForStmt {
	fn can_cast(kind: SyntaxKind) -> bool { kind == FOR_STMT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for ForStmt {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ForStmt")
			.field("for_token", &support::DebugSyntaxResult(self.for_token()))
			.field(
				"l_paren_token",
				&support::DebugSyntaxResult(self.l_paren_token()),
			)
			.field("init", &support::DebugOptionalElement(self.init()))
			.field(
				"first_semi_token",
				&support::DebugSyntaxResult(self.first_semi_token()),
			)
			.field("test", &support::DebugOptionalElement(self.test()))
			.field(
				"second_semi_token",
				&support::DebugSyntaxResult(self.second_semi_token()),
			)
			.field("update", &support::DebugOptionalElement(self.update()))
			.field(
				"r_paren_token",
				&support::DebugSyntaxResult(self.r_paren_token()),
			)
			.field("cons", &support::DebugSyntaxResult(self.cons()))
			.finish()
	}
}
impl AstNode for ForInStmt {
	fn can_cast(kind: SyntaxKind) -> bool { kind == FOR_IN_STMT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for ForInStmt {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ForInStmt")
			.field("for_token", &support::DebugSyntaxResult(self.for_token()))
			.field(
				"l_paren_token",
				&support::DebugSyntaxResult(self.l_paren_token()),
			)
			.field("left", &support::DebugSyntaxResult(self.left()))
			.field("in_token", &support::DebugSyntaxResult(self.in_token()))
			.field("right", &support::DebugSyntaxResult(self.right()))
			.field(
				"r_paren_token",
				&support::DebugSyntaxResult(self.r_paren_token()),
			)
			.field("cons", &support::DebugSyntaxResult(self.cons()))
			.finish()
	}
}
impl AstNode for ForOfStmt {
	fn can_cast(kind: SyntaxKind) -> bool { kind == FOR_OF_STMT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for ForOfStmt {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ForOfStmt")
			.field("for_token", &support::DebugSyntaxResult(self.for_token()))
			.field(
				"l_paren_token",
				&support::DebugSyntaxResult(self.l_paren_token()),
			)
			.field("left", &support::DebugSyntaxResult(self.left()))
			.field("of_token", &support::DebugSyntaxResult(self.of_token()))
			.field("right", &support::DebugSyntaxResult(self.right()))
			.field(
				"r_paren_token",
				&support::DebugSyntaxResult(self.r_paren_token()),
			)
			.field("cons", &support::DebugSyntaxResult(self.cons()))
			.finish()
	}
}
impl AstNode for JsContinueStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_CONTINUE_STATEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsContinueStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsContinueStatement")
			.field(
				"continue_token",
				&support::DebugSyntaxResult(self.continue_token()),
			)
			.field(
				"label_token",
				&support::DebugOptionalElement(self.label_token()),
			)
			.field(
				"semicolon_token",
				&support::DebugOptionalElement(self.semicolon_token()),
			)
			.finish()
	}
}
impl AstNode for JsBreakStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_BREAK_STATEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsBreakStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsBreakStatement")
			.field(
				"break_token",
				&support::DebugSyntaxResult(self.break_token()),
			)
			.field(
				"label_token",
				&support::DebugOptionalElement(self.label_token()),
			)
			.field(
				"semicolon_token",
				&support::DebugOptionalElement(self.semicolon_token()),
			)
			.finish()
	}
}
impl AstNode for JsReturnStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_RETURN_STATEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsReturnStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsReturnStatement")
			.field(
				"return_token",
				&support::DebugSyntaxResult(self.return_token()),
			)
			.field("argument", &support::DebugOptionalElement(self.argument()))
			.field(
				"semicolon_token",
				&support::DebugOptionalElement(self.semicolon_token()),
			)
			.finish()
	}
}
impl AstNode for JsWithStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_WITH_STATEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsWithStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsWithStatement")
			.field("with_token", &support::DebugSyntaxResult(self.with_token()))
			.field(
				"l_paren_token",
				&support::DebugSyntaxResult(self.l_paren_token()),
			)
			.field("object", &support::DebugSyntaxResult(self.object()))
			.field(
				"r_paren_token",
				&support::DebugSyntaxResult(self.r_paren_token()),
			)
			.field("body", &support::DebugSyntaxResult(self.body()))
			.finish()
	}
}
impl AstNode for JsLabeledStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_LABELED_STATEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsLabeledStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsLabeledStatement")
			.field(
				"label_token",
				&support::DebugSyntaxResult(self.label_token()),
			)
			.field(
				"colon_token",
				&support::DebugSyntaxResult(self.colon_token()),
			)
			.field("body", &support::DebugSyntaxResult(self.body()))
			.finish()
	}
}
impl AstNode for JsSwitchStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_SWITCH_STATEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsSwitchStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsSwitchStatement")
			.field(
				"switch_token",
				&support::DebugSyntaxResult(self.switch_token()),
			)
			.field(
				"l_paren_token",
				&support::DebugSyntaxResult(self.l_paren_token()),
			)
			.field(
				"discriminant",
				&support::DebugSyntaxResult(self.discriminant()),
			)
			.field(
				"r_paren_token",
				&support::DebugSyntaxResult(self.r_paren_token()),
			)
			.field(
				"l_curly_token",
				&support::DebugSyntaxResult(self.l_curly_token()),
			)
			.field("cases", &self.cases())
			.field(
				"r_curly_token",
				&support::DebugSyntaxResult(self.r_curly_token()),
			)
			.finish()
	}
}
impl AstNode for JsThrowStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_THROW_STATEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsThrowStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsThrowStatement")
			.field(
				"throw_token",
				&support::DebugSyntaxResult(self.throw_token()),
			)
			.field("argument", &support::DebugSyntaxResult(self.argument()))
			.field(
				"semicolon_token",
				&support::DebugOptionalElement(self.semicolon_token()),
			)
			.finish()
	}
}
impl AstNode for JsTryStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_TRY_STATEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsTryStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsTryStatement")
			.field("try_token", &support::DebugSyntaxResult(self.try_token()))
			.field("body", &support::DebugSyntaxResult(self.body()))
			.field(
				"catch_clause",
				&support::DebugSyntaxResult(self.catch_clause()),
			)
			.finish()
	}
}
impl AstNode for JsTryFinallyStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_TRY_FINALLY_STATEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsTryFinallyStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsTryFinallyStatement")
			.field("try_token", &support::DebugSyntaxResult(self.try_token()))
			.field("body", &support::DebugSyntaxResult(self.body()))
			.field(
				"catch_clause",
				&support::DebugOptionalElement(self.catch_clause()),
			)
			.field(
				"finally_clause",
				&support::DebugSyntaxResult(self.finally_clause()),
			)
			.finish()
	}
}
impl AstNode for JsDebuggerStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_DEBUGGER_STATEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsDebuggerStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsDebuggerStatement")
			.field(
				"debugger_token",
				&support::DebugSyntaxResult(self.debugger_token()),
			)
			.field(
				"semicolon_token",
				&support::DebugOptionalElement(self.semicolon_token()),
			)
			.finish()
	}
}
impl AstNode for JsFunctionDeclaration {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_FUNCTION_DECLARATION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsFunctionDeclaration {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsFunctionDeclaration")
			.field(
				"async_token",
				&support::DebugOptionalElement(self.async_token()),
			)
			.field(
				"function_token",
				&support::DebugSyntaxResult(self.function_token()),
			)
			.field(
				"star_token",
				&support::DebugOptionalElement(self.star_token()),
			)
			.field("id", &support::DebugSyntaxResult(self.id()))
			.field(
				"type_parameters",
				&support::DebugOptionalElement(self.type_parameters()),
			)
			.field(
				"parameter_list",
				&support::DebugSyntaxResult(self.parameter_list()),
			)
			.field(
				"return_type",
				&support::DebugOptionalElement(self.return_type()),
			)
			.field("body", &support::DebugSyntaxResult(self.body()))
			.finish()
	}
}
impl AstNode for JsClassDeclaration {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_CLASS_DECLARATION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsClassDeclaration {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsClassDeclaration")
			.field(
				"class_token",
				&support::DebugSyntaxResult(self.class_token()),
			)
			.field("id", &support::DebugSyntaxResult(self.id()))
			.field(
				"implements_clause",
				&support::DebugOptionalElement(self.implements_clause()),
			)
			.field(
				"extends_clause",
				&support::DebugOptionalElement(self.extends_clause()),
			)
			.field(
				"l_curly_token",
				&support::DebugSyntaxResult(self.l_curly_token()),
			)
			.field("members", &self.members())
			.field(
				"r_curly_token",
				&support::DebugSyntaxResult(self.r_curly_token()),
			)
			.finish()
	}
}
impl AstNode for JsVariableDeclarationStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_VARIABLE_DECLARATION_STATEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsVariableDeclarationStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsVariableDeclarationStatement")
			.field(
				"declaration",
				&support::DebugSyntaxResult(self.declaration()),
			)
			.field(
				"semicolon_token",
				&support::DebugOptionalElement(self.semicolon_token()),
			)
			.finish()
	}
}
impl AstNode for TsEnum {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_ENUM }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsEnum {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsEnum")
			.field(
				"const_token",
				&support::DebugOptionalElement(self.const_token()),
			)
			.field("enum_token", &support::DebugSyntaxResult(self.enum_token()))
			.field("ident", &support::DebugSyntaxResult(self.ident()))
			.field(
				"l_curly_token",
				&support::DebugSyntaxResult(self.l_curly_token()),
			)
			.field("members", &self.members())
			.field(
				"r_curly_token",
				&support::DebugSyntaxResult(self.r_curly_token()),
			)
			.finish()
	}
}
impl AstNode for TsTypeAliasDecl {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_TYPE_ALIAS_DECL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsTypeAliasDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsTypeAliasDecl")
			.field("type_token", &support::DebugSyntaxResult(self.type_token()))
			.field(
				"type_params",
				&support::DebugSyntaxResult(self.type_params()),
			)
			.field("eq_token", &support::DebugSyntaxResult(self.eq_token()))
			.field("ty", &support::DebugSyntaxResult(self.ty()))
			.finish()
	}
}
impl AstNode for TsNamespaceDecl {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_NAMESPACE_DECL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsNamespaceDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsNamespaceDecl")
			.field(
				"declare_token",
				&support::DebugSyntaxResult(self.declare_token()),
			)
			.field("ident", &support::DebugSyntaxResult(self.ident()))
			.field(
				"dot_token",
				&support::DebugOptionalElement(self.dot_token()),
			)
			.field("body", &support::DebugSyntaxResult(self.body()))
			.finish()
	}
}
impl AstNode for TsModuleDecl {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_MODULE_DECL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsModuleDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsModuleDecl")
			.field(
				"declare_token",
				&support::DebugSyntaxResult(self.declare_token()),
			)
			.field(
				"global_token",
				&support::DebugOptionalElement(self.global_token()),
			)
			.field(
				"module_token",
				&support::DebugSyntaxResult(self.module_token()),
			)
			.field(
				"dot_token",
				&support::DebugOptionalElement(self.dot_token()),
			)
			.field("ident", &support::DebugSyntaxResult(self.ident()))
			.field("body", &support::DebugSyntaxResult(self.body()))
			.finish()
	}
}
impl AstNode for TsInterfaceDecl {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_INTERFACE_DECL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsInterfaceDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsInterfaceDecl")
			.field(
				"declare_token",
				&support::DebugOptionalElement(self.declare_token()),
			)
			.field(
				"interface_token",
				&support::DebugSyntaxResult(self.interface_token()),
			)
			.field(
				"type_params",
				&support::DebugSyntaxResult(self.type_params()),
			)
			.field(
				"extends_token",
				&support::DebugOptionalElement(self.extends_token()),
			)
			.field("extends", &support::DebugOptionalElement(self.extends()))
			.field(
				"l_curly_token",
				&support::DebugSyntaxResult(self.l_curly_token()),
			)
			.field("members", &support::DebugSyntaxResult(self.members()))
			.field(
				"r_curly_token",
				&support::DebugSyntaxResult(self.r_curly_token()),
			)
			.finish()
	}
}
impl AstNode for JsElseClause {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_ELSE_CLAUSE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsElseClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsElseClause")
			.field("else_token", &support::DebugSyntaxResult(self.else_token()))
			.field("alternate", &support::DebugSyntaxResult(self.alternate()))
			.finish()
	}
}
impl AstNode for ForStmtInit {
	fn can_cast(kind: SyntaxKind) -> bool { kind == FOR_STMT_INIT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for ForStmtInit {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ForStmtInit")
			.field("inner", &support::DebugSyntaxResult(self.inner()))
			.finish()
	}
}
impl AstNode for ForStmtTest {
	fn can_cast(kind: SyntaxKind) -> bool { kind == FOR_STMT_TEST }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for ForStmtTest {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ForStmtTest")
			.field("expr", &support::DebugSyntaxResult(self.expr()))
			.finish()
	}
}
impl AstNode for ForStmtUpdate {
	fn can_cast(kind: SyntaxKind) -> bool { kind == FOR_STMT_UPDATE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for ForStmtUpdate {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ForStmtUpdate")
			.field("expr", &support::DebugSyntaxResult(self.expr()))
			.finish()
	}
}
impl AstNode for JsVariableDeclaration {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_VARIABLE_DECLARATION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsVariableDeclaration {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsVariableDeclaration")
			.field("kind_token", &support::DebugSyntaxResult(self.kind_token()))
			.field("declarators", &self.declarators())
			.finish()
	}
}
impl AstNode for JsCaseClause {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_CASE_CLAUSE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsCaseClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsCaseClause")
			.field("case_token", &support::DebugSyntaxResult(self.case_token()))
			.field("test", &support::DebugSyntaxResult(self.test()))
			.field(
				"colon_token",
				&support::DebugSyntaxResult(self.colon_token()),
			)
			.field("consequent", &self.consequent())
			.finish()
	}
}
impl AstNode for JsDefaultClause {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_DEFAULT_CLAUSE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsDefaultClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsDefaultClause")
			.field(
				"default_token",
				&support::DebugSyntaxResult(self.default_token()),
			)
			.field(
				"colon_token",
				&support::DebugSyntaxResult(self.colon_token()),
			)
			.field("consequent", &self.consequent())
			.finish()
	}
}
impl AstNode for JsCatchClause {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_CATCH_CLAUSE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsCatchClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsCatchClause")
			.field(
				"catch_token",
				&support::DebugSyntaxResult(self.catch_token()),
			)
			.field(
				"declaration",
				&support::DebugOptionalElement(self.declaration()),
			)
			.field("body", &support::DebugSyntaxResult(self.body()))
			.finish()
	}
}
impl AstNode for JsFinallyClause {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_FINALLY_CLAUSE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsFinallyClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsFinallyClause")
			.field(
				"finally_token",
				&support::DebugSyntaxResult(self.finally_token()),
			)
			.field("body", &support::DebugSyntaxResult(self.body()))
			.finish()
	}
}
impl AstNode for JsCatchDeclaration {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_CATCH_DECLARATION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsCatchDeclaration {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsCatchDeclaration")
			.field(
				"l_paren_token",
				&support::DebugSyntaxResult(self.l_paren_token()),
			)
			.field("binding", &support::DebugSyntaxResult(self.binding()))
			.field(
				"r_paren_token",
				&support::DebugSyntaxResult(self.r_paren_token()),
			)
			.finish()
	}
}
impl AstNode for JsArrayExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_ARRAY_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsArrayExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsArrayExpression")
			.field(
				"l_brack_token",
				&support::DebugSyntaxResult(self.l_brack_token()),
			)
			.field("elements", &self.elements())
			.field(
				"r_brack_token",
				&support::DebugSyntaxResult(self.r_brack_token()),
			)
			.finish()
	}
}
impl AstNode for JsArrowFunctionExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_ARROW_FUNCTION_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsArrowFunctionExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsArrowFunctionExpression")
			.field(
				"async_token",
				&support::DebugOptionalElement(self.async_token()),
			)
			.field(
				"type_parameters",
				&support::DebugOptionalElement(self.type_parameters()),
			)
			.field(
				"parameter_list",
				&support::DebugOptionalElement(self.parameter_list()),
			)
			.field(
				"fat_arrow_token",
				&support::DebugSyntaxResult(self.fat_arrow_token()),
			)
			.field(
				"return_type",
				&support::DebugOptionalElement(self.return_type()),
			)
			.field("body", &support::DebugSyntaxResult(self.body()))
			.finish()
	}
}
impl AstNode for JsAssignmentExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_ASSIGNMENT_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsAssignmentExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsAssignmentExpression")
			.field("left", &support::DebugSyntaxResult(self.left()))
			.field(
				"operator_token",
				&support::DebugSyntaxResult(self.operator_token()),
			)
			.field("right", &support::DebugSyntaxResult(self.right()))
			.finish()
	}
}
impl AstNode for JsAwaitExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_AWAIT_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsAwaitExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsAwaitExpression")
			.field(
				"await_token",
				&support::DebugSyntaxResult(self.await_token()),
			)
			.field("argument", &support::DebugSyntaxResult(self.argument()))
			.finish()
	}
}
impl AstNode for JsBinaryExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_BINARY_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsBinaryExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsBinaryExpression")
			.field("left", &support::DebugSyntaxResult(self.left()))
			.field("operator", &support::DebugSyntaxResult(self.operator()))
			.field("right", &support::DebugSyntaxResult(self.right()))
			.finish()
	}
}
impl AstNode for JsClassExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_CLASS_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsClassExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsClassExpression")
			.field(
				"class_token",
				&support::DebugSyntaxResult(self.class_token()),
			)
			.field("id", &support::DebugOptionalElement(self.id()))
			.field(
				"extends_clause",
				&support::DebugOptionalElement(self.extends_clause()),
			)
			.field(
				"l_curly_token",
				&support::DebugSyntaxResult(self.l_curly_token()),
			)
			.field("members", &self.members())
			.field(
				"r_curly_token",
				&support::DebugSyntaxResult(self.r_curly_token()),
			)
			.finish()
	}
}
impl AstNode for JsConditionalExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_CONDITIONAL_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsConditionalExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsConditionalExpression")
			.field("test", &support::DebugSyntaxResult(self.test()))
			.field(
				"question_mark_token",
				&support::DebugSyntaxResult(self.question_mark_token()),
			)
			.field("consequent", &support::DebugSyntaxResult(self.consequent()))
			.field(
				"colon_token",
				&support::DebugSyntaxResult(self.colon_token()),
			)
			.field("alternate", &support::DebugSyntaxResult(self.alternate()))
			.finish()
	}
}
impl AstNode for JsComputedMemberExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_COMPUTED_MEMBER_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsComputedMemberExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsComputedMemberExpression")
			.field("object", &support::DebugSyntaxResult(self.object()))
			.field(
				"optional_chain_token_token",
				&support::DebugOptionalElement(self.optional_chain_token_token()),
			)
			.field(
				"l_brack_token",
				&support::DebugSyntaxResult(self.l_brack_token()),
			)
			.field("member", &support::DebugSyntaxResult(self.member()))
			.field(
				"r_brack_token",
				&support::DebugSyntaxResult(self.r_brack_token()),
			)
			.finish()
	}
}
impl AstNode for JsFunctionExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_FUNCTION_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsFunctionExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsFunctionExpression")
			.field(
				"async_token",
				&support::DebugOptionalElement(self.async_token()),
			)
			.field(
				"function_token",
				&support::DebugSyntaxResult(self.function_token()),
			)
			.field(
				"star_token",
				&support::DebugOptionalElement(self.star_token()),
			)
			.field("id", &support::DebugOptionalElement(self.id()))
			.field(
				"type_parameters",
				&support::DebugOptionalElement(self.type_parameters()),
			)
			.field("parameters", &support::DebugSyntaxResult(self.parameters()))
			.field(
				"return_type",
				&support::DebugOptionalElement(self.return_type()),
			)
			.field("body", &support::DebugSyntaxResult(self.body()))
			.finish()
	}
}
impl AstNode for JsImportCallExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_IMPORT_CALL_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsImportCallExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsImportCallExpression")
			.field(
				"import_token",
				&support::DebugSyntaxResult(self.import_token()),
			)
			.field(
				"l_paren_token",
				&support::DebugSyntaxResult(self.l_paren_token()),
			)
			.field("argument", &support::DebugSyntaxResult(self.argument()))
			.field(
				"r_paren_token",
				&support::DebugSyntaxResult(self.r_paren_token()),
			)
			.finish()
	}
}
impl AstNode for JsLogicalExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_LOGICAL_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsLogicalExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsLogicalExpression")
			.field("left", &support::DebugSyntaxResult(self.left()))
			.field("operator", &support::DebugSyntaxResult(self.operator()))
			.field("right", &support::DebugSyntaxResult(self.right()))
			.finish()
	}
}
impl AstNode for JsObjectExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_OBJECT_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsObjectExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsObjectExpression")
			.field(
				"l_curly_token",
				&support::DebugSyntaxResult(self.l_curly_token()),
			)
			.field("members", &self.members())
			.field(
				"r_curly_token",
				&support::DebugSyntaxResult(self.r_curly_token()),
			)
			.finish()
	}
}
impl AstNode for JsParenthesizedExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_PARENTHESIZED_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsParenthesizedExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsParenthesizedExpression")
			.field(
				"l_paren_token",
				&support::DebugSyntaxResult(self.l_paren_token()),
			)
			.field("expression", &support::DebugSyntaxResult(self.expression()))
			.field(
				"r_paren_token",
				&support::DebugSyntaxResult(self.r_paren_token()),
			)
			.finish()
	}
}
impl AstNode for JsIdentifierExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_IDENTIFIER_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsIdentifierExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsIdentifierExpression")
			.field("name", &support::DebugSyntaxResult(self.name()))
			.finish()
	}
}
impl AstNode for JsSequenceExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_SEQUENCE_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsSequenceExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsSequenceExpression")
			.field("left", &support::DebugSyntaxResult(self.left()))
			.field(
				"comma_token",
				&support::DebugSyntaxResult(self.comma_token()),
			)
			.field("right", &support::DebugSyntaxResult(self.right()))
			.finish()
	}
}
impl AstNode for JsStaticMemberExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_STATIC_MEMBER_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsStaticMemberExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsStaticMemberExpression")
			.field("object", &support::DebugSyntaxResult(self.object()))
			.field("operator", &support::DebugSyntaxResult(self.operator()))
			.field("member", &support::DebugSyntaxResult(self.member()))
			.finish()
	}
}
impl AstNode for JsSuperExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_SUPER_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsSuperExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsSuperExpression")
			.field(
				"super_token",
				&support::DebugSyntaxResult(self.super_token()),
			)
			.finish()
	}
}
impl AstNode for JsThisExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_THIS_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsThisExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsThisExpression")
			.field("this_token", &support::DebugSyntaxResult(self.this_token()))
			.finish()
	}
}
impl AstNode for JsUnaryExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_UNARY_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsUnaryExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsUnaryExpression")
			.field("operator", &support::DebugSyntaxResult(self.operator()))
			.field("argument", &support::DebugSyntaxResult(self.argument()))
			.finish()
	}
}
impl AstNode for JsPreUpdateExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_PRE_UPDATE_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsPreUpdateExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsPreUpdateExpression")
			.field("operator", &support::DebugSyntaxResult(self.operator()))
			.field("operand", &support::DebugSyntaxResult(self.operand()))
			.finish()
	}
}
impl AstNode for JsPostUpdateExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_POST_UPDATE_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsPostUpdateExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsPostUpdateExpression")
			.field("operand", &support::DebugSyntaxResult(self.operand()))
			.field("operator", &support::DebugSyntaxResult(self.operator()))
			.finish()
	}
}
impl AstNode for JsYieldExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_YIELD_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsYieldExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsYieldExpression")
			.field(
				"yield_token",
				&support::DebugSyntaxResult(self.yield_token()),
			)
			.field(
				"star_token",
				&support::DebugOptionalElement(self.star_token()),
			)
			.field("argument", &support::DebugOptionalElement(self.argument()))
			.finish()
	}
}
impl AstNode for Template {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TEMPLATE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for Template {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Template")
			.field(
				"backtick_token",
				&support::DebugSyntaxResult(self.backtick_token()),
			)
			.finish()
	}
}
impl AstNode for NewExpr {
	fn can_cast(kind: SyntaxKind) -> bool { kind == NEW_EXPR }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for NewExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("NewExpr")
			.field("new_token", &support::DebugSyntaxResult(self.new_token()))
			.field(
				"type_args",
				&support::DebugOptionalElement(self.type_args()),
			)
			.field("object", &support::DebugSyntaxResult(self.object()))
			.field("arguments", &support::DebugSyntaxResult(self.arguments()))
			.finish()
	}
}
impl AstNode for CallExpr {
	fn can_cast(kind: SyntaxKind) -> bool { kind == CALL_EXPR }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for CallExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("CallExpr")
			.field(
				"type_args",
				&support::DebugOptionalElement(self.type_args()),
			)
			.field("callee", &support::DebugSyntaxResult(self.callee()))
			.field("arguments", &support::DebugSyntaxResult(self.arguments()))
			.finish()
	}
}
impl AstNode for NewTarget {
	fn can_cast(kind: SyntaxKind) -> bool { kind == NEW_TARGET }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for NewTarget {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("NewTarget")
			.field("new_token", &support::DebugSyntaxResult(self.new_token()))
			.field("dot_token", &support::DebugSyntaxResult(self.dot_token()))
			.field(
				"target_token",
				&support::DebugSyntaxResult(self.target_token()),
			)
			.finish()
	}
}
impl AstNode for ImportMeta {
	fn can_cast(kind: SyntaxKind) -> bool { kind == IMPORT_META }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for ImportMeta {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ImportMeta")
			.field(
				"import_token",
				&support::DebugSyntaxResult(self.import_token()),
			)
			.field("dot_token", &support::DebugSyntaxResult(self.dot_token()))
			.finish()
	}
}
impl AstNode for TsNonNull {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_NON_NULL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsNonNull {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsNonNull")
			.field("expr", &support::DebugSyntaxResult(self.expr()))
			.field("excl_token", &support::DebugSyntaxResult(self.excl_token()))
			.finish()
	}
}
impl AstNode for TsAssertion {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_ASSERTION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsAssertion {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsAssertion")
			.field("expr", &support::DebugSyntaxResult(self.expr()))
			.field("ident", &support::DebugSyntaxResult(self.ident()))
			.field(
				"l_angle_token",
				&support::DebugSyntaxResult(self.l_angle_token()),
			)
			.field("ty", &support::DebugSyntaxResult(self.ty()))
			.field(
				"r_angle_token",
				&support::DebugSyntaxResult(self.r_angle_token()),
			)
			.finish()
	}
}
impl AstNode for TsConstAssertion {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_CONST_ASSERTION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsConstAssertion {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsConstAssertion")
			.field("expr", &support::DebugSyntaxResult(self.expr()))
			.field("ident", &support::DebugSyntaxResult(self.ident()))
			.field(
				"l_angle_token",
				&support::DebugSyntaxResult(self.l_angle_token()),
			)
			.field(
				"const_token",
				&support::DebugSyntaxResult(self.const_token()),
			)
			.field(
				"r_angle_token",
				&support::DebugSyntaxResult(self.r_angle_token()),
			)
			.finish()
	}
}
impl AstNode for TsTypeArgs {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_TYPE_ARGS }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsTypeArgs {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsTypeArgs")
			.field(
				"l_angle_token",
				&support::DebugSyntaxResult(self.l_angle_token()),
			)
			.field("args", &support::DebugSyntaxResult(self.args()))
			.field(
				"r_angle_token",
				&support::DebugSyntaxResult(self.r_angle_token()),
			)
			.finish()
	}
}
impl AstNode for ArgList {
	fn can_cast(kind: SyntaxKind) -> bool { kind == ARG_LIST }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for ArgList {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ArgList")
			.field(
				"l_paren_token",
				&support::DebugSyntaxResult(self.l_paren_token()),
			)
			.field("args", &self.args())
			.field(
				"r_paren_token",
				&support::DebugSyntaxResult(self.r_paren_token()),
			)
			.finish()
	}
}
impl AstNode for TsTypeParams {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_TYPE_PARAMS }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsTypeParams {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsTypeParams")
			.field(
				"l_angle_token",
				&support::DebugOptionalElement(self.l_angle_token()),
			)
			.field("params", &support::DebugSyntaxResult(self.params()))
			.field(
				"r_angle_token",
				&support::DebugOptionalElement(self.r_angle_token()),
			)
			.finish()
	}
}
impl AstNode for JsParameterList {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_PARAMETER_LIST }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsParameterList {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsParameterList")
			.field(
				"l_paren_token",
				&support::DebugSyntaxResult(self.l_paren_token()),
			)
			.field("parameters", &self.parameters())
			.field(
				"r_paren_token",
				&support::DebugSyntaxResult(self.r_paren_token()),
			)
			.finish()
	}
}
impl AstNode for TsTypeAnnotation {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_TYPE_ANNOTATION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsTypeAnnotation {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsTypeAnnotation")
			.field(
				"colon_token",
				&support::DebugSyntaxResult(self.colon_token()),
			)
			.field("ty", &support::DebugSyntaxResult(self.ty()))
			.finish()
	}
}
impl AstNode for JsFunctionBody {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_FUNCTION_BODY }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsFunctionBody {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsFunctionBody")
			.field(
				"l_curly_token",
				&support::DebugSyntaxResult(self.l_curly_token()),
			)
			.field("directives", &self.directives())
			.field("statements", &self.statements())
			.field(
				"r_curly_token",
				&support::DebugSyntaxResult(self.r_curly_token()),
			)
			.finish()
	}
}
impl AstNode for JsSpread {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_SPREAD }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsSpread {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsSpread")
			.field(
				"dotdotdot_token",
				&support::DebugSyntaxResult(self.dotdotdot_token()),
			)
			.field("argument", &support::DebugSyntaxResult(self.argument()))
			.finish()
	}
}
impl AstNode for JsArrayHole {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_ARRAY_HOLE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsArrayHole {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsArrayHole")
			.field(
				"hole_token",
				&support::DebugOptionalElement(self.hole_token()),
			)
			.finish()
	}
}
impl AstNode for JsReferenceIdentifier {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_REFERENCE_IDENTIFIER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsReferenceIdentifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsReferenceIdentifier")
			.field(
				"value_token",
				&support::DebugSyntaxResult(self.value_token()),
			)
			.finish()
	}
}
impl AstNode for JsLiteralMemberName {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_LITERAL_MEMBER_NAME }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsLiteralMemberName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsLiteralMemberName")
			.field("value", &support::DebugSyntaxResult(self.value()))
			.finish()
	}
}
impl AstNode for JsComputedMemberName {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_COMPUTED_MEMBER_NAME }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsComputedMemberName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsComputedMemberName")
			.field(
				"l_brack_token",
				&support::DebugSyntaxResult(self.l_brack_token()),
			)
			.field("expression", &support::DebugSyntaxResult(self.expression()))
			.field(
				"r_brack_token",
				&support::DebugSyntaxResult(self.r_brack_token()),
			)
			.finish()
	}
}
impl AstNode for JsPropertyObjectMember {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_PROPERTY_OBJECT_MEMBER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsPropertyObjectMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsPropertyObjectMember")
			.field("name", &support::DebugSyntaxResult(self.name()))
			.field(
				"colon_token",
				&support::DebugSyntaxResult(self.colon_token()),
			)
			.field("value", &support::DebugSyntaxResult(self.value()))
			.finish()
	}
}
impl AstNode for JsMethodObjectMember {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_METHOD_OBJECT_MEMBER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsMethodObjectMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsMethodObjectMember")
			.field(
				"async_token",
				&support::DebugOptionalElement(self.async_token()),
			)
			.field(
				"star_token",
				&support::DebugOptionalElement(self.star_token()),
			)
			.field("name", &support::DebugSyntaxResult(self.name()))
			.field(
				"type_params",
				&support::DebugOptionalElement(self.type_params()),
			)
			.field(
				"parameter_list",
				&support::DebugSyntaxResult(self.parameter_list()),
			)
			.field(
				"return_type",
				&support::DebugOptionalElement(self.return_type()),
			)
			.field("body", &support::DebugSyntaxResult(self.body()))
			.finish()
	}
}
impl AstNode for JsGetterObjectMember {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_GETTER_OBJECT_MEMBER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsGetterObjectMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsGetterObjectMember")
			.field("get_token", &support::DebugSyntaxResult(self.get_token()))
			.field("name", &support::DebugSyntaxResult(self.name()))
			.field(
				"l_paren_token",
				&support::DebugSyntaxResult(self.l_paren_token()),
			)
			.field(
				"r_paren_token",
				&support::DebugSyntaxResult(self.r_paren_token()),
			)
			.field(
				"return_type",
				&support::DebugOptionalElement(self.return_type()),
			)
			.field("body", &support::DebugSyntaxResult(self.body()))
			.finish()
	}
}
impl AstNode for JsSetterObjectMember {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_SETTER_OBJECT_MEMBER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsSetterObjectMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsSetterObjectMember")
			.field("set_token", &support::DebugSyntaxResult(self.set_token()))
			.field("name", &support::DebugSyntaxResult(self.name()))
			.field(
				"l_paren_token",
				&support::DebugSyntaxResult(self.l_paren_token()),
			)
			.field("parameter", &support::DebugSyntaxResult(self.parameter()))
			.field(
				"r_paren_token",
				&support::DebugSyntaxResult(self.r_paren_token()),
			)
			.field("body", &support::DebugSyntaxResult(self.body()))
			.finish()
	}
}
impl AstNode for JsShorthandPropertyObjectMember {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_SHORTHAND_PROPERTY_OBJECT_MEMBER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsShorthandPropertyObjectMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsShorthandPropertyObjectMember")
			.field("name", &support::DebugSyntaxResult(self.name()))
			.finish()
	}
}
impl AstNode for TsImplementsClause {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_IMPLEMENTS_CLAUSE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsImplementsClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsImplementsClause")
			.field(
				"implements_token",
				&support::DebugSyntaxResult(self.implements_token()),
			)
			.field("interfaces", &self.interfaces())
			.finish()
	}
}
impl AstNode for JsExtendsClause {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_EXTENDS_CLAUSE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsExtendsClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsExtendsClause")
			.field(
				"extends_token",
				&support::DebugSyntaxResult(self.extends_token()),
			)
			.field(
				"super_class",
				&support::DebugSyntaxResult(self.super_class()),
			)
			.finish()
	}
}
impl AstNode for TsExprWithTypeArgs {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_EXPR_WITH_TYPE_ARGS }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsExprWithTypeArgs {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsExprWithTypeArgs")
			.field("item", &support::DebugSyntaxResult(self.item()))
			.field(
				"type_params",
				&support::DebugSyntaxResult(self.type_params()),
			)
			.finish()
	}
}
impl AstNode for JsPrivateClassMemberName {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_PRIVATE_CLASS_MEMBER_NAME }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsPrivateClassMemberName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsPrivateClassMemberName")
			.field("hash_token", &support::DebugSyntaxResult(self.hash_token()))
			.field("id_token", &support::DebugSyntaxResult(self.id_token()))
			.finish()
	}
}
impl AstNode for JsConstructorClassMember {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_CONSTRUCTOR_CLASS_MEMBER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsConstructorClassMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsConstructorClassMember")
			.field(
				"access_modifier",
				&support::DebugOptionalElement(self.access_modifier()),
			)
			.field("name", &support::DebugSyntaxResult(self.name()))
			.field(
				"parameter_list",
				&support::DebugSyntaxResult(self.parameter_list()),
			)
			.field("body", &support::DebugSyntaxResult(self.body()))
			.finish()
	}
}
impl AstNode for JsPropertyClassMember {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_PROPERTY_CLASS_MEMBER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsPropertyClassMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsPropertyClassMember")
			.field(
				"modifiers",
				&support::DebugOptionalElement(self.modifiers()),
			)
			.field(
				"access_modifier",
				&support::DebugOptionalElement(self.access_modifier()),
			)
			.field(
				"abstract_token",
				&support::DebugOptionalElement(self.abstract_token()),
			)
			.field(
				"static_token",
				&support::DebugOptionalElement(self.static_token()),
			)
			.field("name", &support::DebugSyntaxResult(self.name()))
			.field(
				"question_mark_token",
				&support::DebugOptionalElement(self.question_mark_token()),
			)
			.field(
				"excl_token",
				&support::DebugOptionalElement(self.excl_token()),
			)
			.field("ty", &support::DebugOptionalElement(self.ty()))
			.field("value", &support::DebugOptionalElement(self.value()))
			.field(
				"semicolon_token",
				&support::DebugOptionalElement(self.semicolon_token()),
			)
			.finish()
	}
}
impl AstNode for JsMethodClassMember {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_METHOD_CLASS_MEMBER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsMethodClassMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsMethodClassMember")
			.field(
				"access_modifier",
				&support::DebugOptionalElement(self.access_modifier()),
			)
			.field(
				"static_token",
				&support::DebugOptionalElement(self.static_token()),
			)
			.field(
				"abstract_token",
				&support::DebugOptionalElement(self.abstract_token()),
			)
			.field(
				"async_token",
				&support::DebugOptionalElement(self.async_token()),
			)
			.field(
				"star_token",
				&support::DebugOptionalElement(self.star_token()),
			)
			.field("name", &support::DebugSyntaxResult(self.name()))
			.field(
				"type_parameters",
				&support::DebugOptionalElement(self.type_parameters()),
			)
			.field(
				"parameter_list",
				&support::DebugSyntaxResult(self.parameter_list()),
			)
			.field(
				"return_type",
				&support::DebugOptionalElement(self.return_type()),
			)
			.field("body", &support::DebugSyntaxResult(self.body()))
			.finish()
	}
}
impl AstNode for JsGetterClassMember {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_GETTER_CLASS_MEMBER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsGetterClassMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsGetterClassMember")
			.field(
				"access_modifier",
				&support::DebugOptionalElement(self.access_modifier()),
			)
			.field(
				"abstract_token",
				&support::DebugOptionalElement(self.abstract_token()),
			)
			.field(
				"static_token",
				&support::DebugOptionalElement(self.static_token()),
			)
			.field("get_token", &support::DebugSyntaxResult(self.get_token()))
			.field("name", &support::DebugSyntaxResult(self.name()))
			.field(
				"l_paren_token",
				&support::DebugSyntaxResult(self.l_paren_token()),
			)
			.field(
				"r_paren_token",
				&support::DebugSyntaxResult(self.r_paren_token()),
			)
			.field(
				"return_type",
				&support::DebugOptionalElement(self.return_type()),
			)
			.field("body", &support::DebugSyntaxResult(self.body()))
			.finish()
	}
}
impl AstNode for JsSetterClassMember {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_SETTER_CLASS_MEMBER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsSetterClassMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsSetterClassMember")
			.field(
				"access_modifier",
				&support::DebugOptionalElement(self.access_modifier()),
			)
			.field(
				"abstract_token",
				&support::DebugOptionalElement(self.abstract_token()),
			)
			.field(
				"static_token",
				&support::DebugOptionalElement(self.static_token()),
			)
			.field("set_token", &support::DebugSyntaxResult(self.set_token()))
			.field("name", &support::DebugSyntaxResult(self.name()))
			.field(
				"l_paren_token",
				&support::DebugSyntaxResult(self.l_paren_token()),
			)
			.field("parameter", &support::DebugSyntaxResult(self.parameter()))
			.field(
				"r_paren_token",
				&support::DebugSyntaxResult(self.r_paren_token()),
			)
			.field("body", &support::DebugSyntaxResult(self.body()))
			.finish()
	}
}
impl AstNode for JsEmptyClassMember {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_EMPTY_CLASS_MEMBER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsEmptyClassMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsEmptyClassMember")
			.field(
				"semicolon_token",
				&support::DebugSyntaxResult(self.semicolon_token()),
			)
			.finish()
	}
}
impl AstNode for TsIndexSignature {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_INDEX_SIGNATURE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsIndexSignature {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsIndexSignature")
			.field(
				"readonly_token",
				&support::DebugOptionalElement(self.readonly_token()),
			)
			.field(
				"l_brack_token",
				&support::DebugSyntaxResult(self.l_brack_token()),
			)
			.field("pat", &support::DebugSyntaxResult(self.pat()))
			.field(
				"colon_token",
				&support::DebugSyntaxResult(self.colon_token()),
			)
			.field("ty", &support::DebugSyntaxResult(self.ty()))
			.field(
				"r_brack_token",
				&support::DebugSyntaxResult(self.r_brack_token()),
			)
			.finish()
	}
}
impl AstNode for TsAccessibility {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_ACCESSIBILITY }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsAccessibility {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsAccessibility")
			.field(
				"private_token",
				&support::DebugSyntaxResult(self.private_token()),
			)
			.field(
				"readonly_token",
				&support::DebugSyntaxResult(self.readonly_token()),
			)
			.finish()
	}
}
impl AstNode for JsConstructorParameterList {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_CONSTRUCTOR_PARAMETER_LIST }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsConstructorParameterList {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsConstructorParameterList")
			.field(
				"l_paren_token",
				&support::DebugSyntaxResult(self.l_paren_token()),
			)
			.field("parameters", &self.parameters())
			.field(
				"r_paren_token",
				&support::DebugSyntaxResult(self.r_paren_token()),
			)
			.finish()
	}
}
impl AstNode for TsConstructorParam {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_CONSTRUCTOR_PARAM }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsConstructorParam {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsConstructorParam")
			.field(
				"readonly_token",
				&support::DebugSyntaxResult(self.readonly_token()),
			)
			.field("pat", &support::DebugSyntaxResult(self.pat()))
			.finish()
	}
}
impl AstNode for JsBindingPatternWithDefault {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_BINDING_PATTERN_WITH_DEFAULT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsBindingPatternWithDefault {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsBindingPatternWithDefault")
			.field("pattern", &support::DebugSyntaxResult(self.pattern()))
			.field("eq_token", &support::DebugSyntaxResult(self.eq_token()))
			.field("default", &support::DebugSyntaxResult(self.default()))
			.finish()
	}
}
impl AstNode for JsEqualValueClause {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_EQUAL_VALUE_CLAUSE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsEqualValueClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsEqualValueClause")
			.field("eq_token", &support::DebugSyntaxResult(self.eq_token()))
			.field("expression", &support::DebugSyntaxResult(self.expression()))
			.finish()
	}
}
impl AstNode for JsModifier {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_MODIFIER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsModifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsModifier")
			.field(
				"declare_token",
				&support::DebugOptionalElement(self.declare_token()),
			)
			.finish()
	}
}
impl AstNode for JsIdentifierAssignment {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_IDENTIFIER_ASSIGNMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsIdentifierAssignment {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsIdentifierAssignment")
			.field("name_token", &support::DebugSyntaxResult(self.name_token()))
			.finish()
	}
}
impl AstNode for JsStaticMemberAssignment {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_STATIC_MEMBER_ASSIGNMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsStaticMemberAssignment {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsStaticMemberAssignment")
			.field("object", &support::DebugSyntaxResult(self.object()))
			.field("dot_token", &support::DebugSyntaxResult(self.dot_token()))
			.field("member", &support::DebugSyntaxResult(self.member()))
			.finish()
	}
}
impl AstNode for JsComputedMemberAssignment {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_COMPUTED_MEMBER_ASSIGNMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsComputedMemberAssignment {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsComputedMemberAssignment")
			.field("object", &support::DebugSyntaxResult(self.object()))
			.field(
				"l_brack_token",
				&support::DebugSyntaxResult(self.l_brack_token()),
			)
			.field("member", &support::DebugSyntaxResult(self.member()))
			.field(
				"r_brack_token",
				&support::DebugSyntaxResult(self.r_brack_token()),
			)
			.finish()
	}
}
impl AstNode for JsParenthesizedAssignment {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_PARENTHESIZED_ASSIGNMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsParenthesizedAssignment {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsParenthesizedAssignment")
			.field(
				"l_paren_token",
				&support::DebugSyntaxResult(self.l_paren_token()),
			)
			.field("assignment", &support::DebugSyntaxResult(self.assignment()))
			.field(
				"r_paren_token",
				&support::DebugSyntaxResult(self.r_paren_token()),
			)
			.finish()
	}
}
impl AstNode for JsAssignmentWithDefault {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_ASSIGNMENT_WITH_DEFAULT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsAssignmentWithDefault {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsAssignmentWithDefault")
			.field("pattern", &support::DebugSyntaxResult(self.pattern()))
			.field("eq_token", &support::DebugSyntaxResult(self.eq_token()))
			.field("default", &support::DebugSyntaxResult(self.default()))
			.finish()
	}
}
impl AstNode for JsArrayAssignmentPattern {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_ARRAY_ASSIGNMENT_PATTERN }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsArrayAssignmentPattern {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsArrayAssignmentPattern")
			.field(
				"l_brack_token",
				&support::DebugSyntaxResult(self.l_brack_token()),
			)
			.field("elements", &self.elements())
			.field(
				"r_brack_token",
				&support::DebugSyntaxResult(self.r_brack_token()),
			)
			.finish()
	}
}
impl AstNode for JsObjectAssignmentPattern {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_OBJECT_ASSIGNMENT_PATTERN }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsObjectAssignmentPattern {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsObjectAssignmentPattern")
			.field(
				"l_curly_token",
				&support::DebugSyntaxResult(self.l_curly_token()),
			)
			.field("properties", &self.properties())
			.field(
				"r_curly_token",
				&support::DebugSyntaxResult(self.r_curly_token()),
			)
			.finish()
	}
}
impl AstNode for JsArrayAssignmentPatternRestElement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_ARRAY_ASSIGNMENT_PATTERN_REST_ELEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsArrayAssignmentPatternRestElement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsArrayAssignmentPatternRestElement")
			.field(
				"dotdotdot_token",
				&support::DebugSyntaxResult(self.dotdotdot_token()),
			)
			.field("pattern", &support::DebugSyntaxResult(self.pattern()))
			.finish()
	}
}
impl AstNode for JsObjectAssignmentPatternShorthandProperty {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == JS_OBJECT_ASSIGNMENT_PATTERN_SHORTHAND_PROPERTY
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsObjectAssignmentPatternShorthandProperty {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsObjectAssignmentPatternShorthandProperty")
			.field("identifier", &support::DebugSyntaxResult(self.identifier()))
			.field("init", &support::DebugOptionalElement(self.init()))
			.finish()
	}
}
impl AstNode for JsObjectAssignmentPatternProperty {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsObjectAssignmentPatternProperty {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsObjectAssignmentPatternProperty")
			.field("member", &support::DebugSyntaxResult(self.member()))
			.field(
				"colon_token",
				&support::DebugSyntaxResult(self.colon_token()),
			)
			.field("pattern", &support::DebugSyntaxResult(self.pattern()))
			.field("init", &support::DebugOptionalElement(self.init()))
			.finish()
	}
}
impl AstNode for JsObjectAssignmentPatternRest {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_OBJECT_ASSIGNMENT_PATTERN_REST }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsObjectAssignmentPatternRest {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsObjectAssignmentPatternRest")
			.field(
				"dotdotdot_token",
				&support::DebugSyntaxResult(self.dotdotdot_token()),
			)
			.field("target", &support::DebugSyntaxResult(self.target()))
			.finish()
	}
}
impl AstNode for JsName {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_NAME }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsName")
			.field(
				"value_token",
				&support::DebugSyntaxResult(self.value_token()),
			)
			.finish()
	}
}
impl AstNode for JsIdentifierBinding {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_IDENTIFIER_BINDING }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsIdentifierBinding {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsIdentifierBinding")
			.field("name_token", &support::DebugSyntaxResult(self.name_token()))
			.finish()
	}
}
impl AstNode for JsArrayBindingPattern {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_ARRAY_BINDING_PATTERN }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsArrayBindingPattern {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsArrayBindingPattern")
			.field(
				"l_brack_token",
				&support::DebugSyntaxResult(self.l_brack_token()),
			)
			.field("elements", &self.elements())
			.field(
				"r_brack_token",
				&support::DebugSyntaxResult(self.r_brack_token()),
			)
			.finish()
	}
}
impl AstNode for JsObjectBindingPattern {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_OBJECT_BINDING_PATTERN }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsObjectBindingPattern {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsObjectBindingPattern")
			.field(
				"l_curly_token",
				&support::DebugSyntaxResult(self.l_curly_token()),
			)
			.field("properties", &self.properties())
			.field(
				"r_curly_token",
				&support::DebugSyntaxResult(self.r_curly_token()),
			)
			.finish()
	}
}
impl AstNode for JsArrayBindingPatternRestElement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_ARRAY_BINDING_PATTERN_REST_ELEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsArrayBindingPatternRestElement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsArrayBindingPatternRestElement")
			.field(
				"dotdotdot_token",
				&support::DebugSyntaxResult(self.dotdotdot_token()),
			)
			.field("pattern", &support::DebugSyntaxResult(self.pattern()))
			.finish()
	}
}
impl AstNode for JsObjectBindingPatternProperty {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_OBJECT_BINDING_PATTERN_PROPERTY }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsObjectBindingPatternProperty {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsObjectBindingPatternProperty")
			.field("member", &support::DebugSyntaxResult(self.member()))
			.field(
				"colon_token",
				&support::DebugSyntaxResult(self.colon_token()),
			)
			.field("pattern", &support::DebugSyntaxResult(self.pattern()))
			.field("init", &support::DebugOptionalElement(self.init()))
			.finish()
	}
}
impl AstNode for JsObjectBindingPatternRest {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_OBJECT_BINDING_PATTERN_REST }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsObjectBindingPatternRest {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsObjectBindingPatternRest")
			.field(
				"dotdotdot_token",
				&support::DebugSyntaxResult(self.dotdotdot_token()),
			)
			.field("binding", &support::DebugSyntaxResult(self.binding()))
			.finish()
	}
}
impl AstNode for JsObjectBindingPatternShorthandProperty {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsObjectBindingPatternShorthandProperty {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsObjectBindingPatternShorthandProperty")
			.field("identifier", &support::DebugSyntaxResult(self.identifier()))
			.field("init", &support::DebugOptionalElement(self.init()))
			.finish()
	}
}
impl AstNode for JsStringLiteralExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_STRING_LITERAL_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsStringLiteralExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsStringLiteralExpression")
			.field(
				"value_token",
				&support::DebugSyntaxResult(self.value_token()),
			)
			.finish()
	}
}
impl AstNode for JsNumberLiteralExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_NUMBER_LITERAL_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsNumberLiteralExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsNumberLiteralExpression")
			.field(
				"value_token",
				&support::DebugSyntaxResult(self.value_token()),
			)
			.finish()
	}
}
impl AstNode for JsBigIntLiteralExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_BIG_INT_LITERAL_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsBigIntLiteralExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsBigIntLiteralExpression")
			.field(
				"value_token",
				&support::DebugSyntaxResult(self.value_token()),
			)
			.finish()
	}
}
impl AstNode for JsBooleanLiteralExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_BOOLEAN_LITERAL_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsBooleanLiteralExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsBooleanLiteralExpression")
			.field(
				"value_token",
				&support::DebugSyntaxResult(self.value_token()),
			)
			.finish()
	}
}
impl AstNode for JsNullLiteralExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_NULL_LITERAL_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsNullLiteralExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsNullLiteralExpression")
			.field(
				"value_token",
				&support::DebugSyntaxResult(self.value_token()),
			)
			.finish()
	}
}
impl AstNode for JsRegexLiteralExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_REGEX_LITERAL_EXPRESSION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsRegexLiteralExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsRegexLiteralExpression")
			.field(
				"value_token",
				&support::DebugSyntaxResult(self.value_token()),
			)
			.finish()
	}
}
impl AstNode for JsVariableDeclarator {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_VARIABLE_DECLARATOR }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsVariableDeclarator {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsVariableDeclarator")
			.field("id", &support::DebugSyntaxResult(self.id()))
			.field("init", &support::DebugOptionalElement(self.init()))
			.finish()
	}
}
impl AstNode for JsImport {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_IMPORT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsImport {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsImport")
			.field(
				"import_token",
				&support::DebugSyntaxResult(self.import_token()),
			)
			.field(
				"import_clause",
				&support::DebugSyntaxResult(self.import_clause()),
			)
			.field(
				"semicolon_token",
				&support::DebugOptionalElement(self.semicolon_token()),
			)
			.finish()
	}
}
impl AstNode for ExportNamed {
	fn can_cast(kind: SyntaxKind) -> bool { kind == EXPORT_NAMED }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for ExportNamed {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ExportNamed")
			.field(
				"export_token",
				&support::DebugSyntaxResult(self.export_token()),
			)
			.field(
				"type_token",
				&support::DebugOptionalElement(self.type_token()),
			)
			.field(
				"from_token",
				&support::DebugOptionalElement(self.from_token()),
			)
			.field(
				"l_curly_token",
				&support::DebugSyntaxResult(self.l_curly_token()),
			)
			.field("specifiers", &self.specifiers())
			.field(
				"r_curly_token",
				&support::DebugSyntaxResult(self.r_curly_token()),
			)
			.finish()
	}
}
impl AstNode for ExportDefaultDecl {
	fn can_cast(kind: SyntaxKind) -> bool { kind == EXPORT_DEFAULT_DECL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for ExportDefaultDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ExportDefaultDecl")
			.field(
				"export_token",
				&support::DebugSyntaxResult(self.export_token()),
			)
			.field(
				"default_token",
				&support::DebugOptionalElement(self.default_token()),
			)
			.field(
				"type_token",
				&support::DebugOptionalElement(self.type_token()),
			)
			.field("decl", &support::DebugSyntaxResult(self.decl()))
			.finish()
	}
}
impl AstNode for ExportDefaultExpr {
	fn can_cast(kind: SyntaxKind) -> bool { kind == EXPORT_DEFAULT_EXPR }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for ExportDefaultExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ExportDefaultExpr")
			.field(
				"export_token",
				&support::DebugSyntaxResult(self.export_token()),
			)
			.field(
				"type_token",
				&support::DebugOptionalElement(self.type_token()),
			)
			.field(
				"default_token",
				&support::DebugOptionalElement(self.default_token()),
			)
			.field("expr", &support::DebugSyntaxResult(self.expr()))
			.finish()
	}
}
impl AstNode for ExportWildcard {
	fn can_cast(kind: SyntaxKind) -> bool { kind == EXPORT_WILDCARD }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for ExportWildcard {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ExportWildcard")
			.field(
				"export_token",
				&support::DebugSyntaxResult(self.export_token()),
			)
			.field(
				"type_token",
				&support::DebugOptionalElement(self.type_token()),
			)
			.field("star_token", &support::DebugSyntaxResult(self.star_token()))
			.field("as_token", &support::DebugOptionalElement(self.as_token()))
			.field("ident", &support::DebugOptionalElement(self.ident()))
			.field("from_token", &support::DebugSyntaxResult(self.from_token()))
			.field(
				"source_token",
				&support::DebugSyntaxResult(self.source_token()),
			)
			.finish()
	}
}
impl AstNode for ExportDecl {
	fn can_cast(kind: SyntaxKind) -> bool { kind == EXPORT_DECL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for ExportDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ExportDecl")
			.field(
				"export_token",
				&support::DebugSyntaxResult(self.export_token()),
			)
			.field(
				"type_token",
				&support::DebugOptionalElement(self.type_token()),
			)
			.field("decl", &support::DebugSyntaxResult(self.decl()))
			.finish()
	}
}
impl AstNode for TsImportEqualsDecl {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_IMPORT_EQUALS_DECL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsImportEqualsDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsImportEqualsDecl")
			.field(
				"import_token",
				&support::DebugSyntaxResult(self.import_token()),
			)
			.field(
				"export_token",
				&support::DebugSyntaxResult(self.export_token()),
			)
			.field("ident", &support::DebugSyntaxResult(self.ident()))
			.field("eq_token", &support::DebugSyntaxResult(self.eq_token()))
			.field("module", &support::DebugSyntaxResult(self.module()))
			.field(
				"semicolon_token",
				&support::DebugOptionalElement(self.semicolon_token()),
			)
			.finish()
	}
}
impl AstNode for TsExportAssignment {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_EXPORT_ASSIGNMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsExportAssignment {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsExportAssignment")
			.field(
				"export_token",
				&support::DebugSyntaxResult(self.export_token()),
			)
			.field("eq_token", &support::DebugSyntaxResult(self.eq_token()))
			.field("expr", &support::DebugSyntaxResult(self.expr()))
			.field(
				"semicolon_token",
				&support::DebugOptionalElement(self.semicolon_token()),
			)
			.finish()
	}
}
impl AstNode for TsNamespaceExportDecl {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_NAMESPACE_EXPORT_DECL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsNamespaceExportDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsNamespaceExportDecl")
			.field(
				"export_token",
				&support::DebugSyntaxResult(self.export_token()),
			)
			.field("as_token", &support::DebugSyntaxResult(self.as_token()))
			.field(
				"namespace_token",
				&support::DebugSyntaxResult(self.namespace_token()),
			)
			.field("ident", &support::DebugOptionalElement(self.ident()))
			.field(
				"semicolon_token",
				&support::DebugOptionalElement(self.semicolon_token()),
			)
			.finish()
	}
}
impl AstNode for JsImportBareClause {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_IMPORT_BARE_CLAUSE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsImportBareClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsImportBareClause")
			.field("source", &support::DebugSyntaxResult(self.source()))
			.field(
				"assertion",
				&support::DebugOptionalElement(self.assertion()),
			)
			.finish()
	}
}
impl AstNode for JsImportNamedClause {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_IMPORT_NAMED_CLAUSE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsImportNamedClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsImportNamedClause")
			.field(
				"default_specifier",
				&support::DebugOptionalElement(self.default_specifier()),
			)
			.field(
				"named_import",
				&support::DebugSyntaxResult(self.named_import()),
			)
			.field("from_token", &support::DebugSyntaxResult(self.from_token()))
			.field("source", &support::DebugSyntaxResult(self.source()))
			.field(
				"assertion",
				&support::DebugOptionalElement(self.assertion()),
			)
			.finish()
	}
}
impl AstNode for JsImportDefaultClause {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_IMPORT_DEFAULT_CLAUSE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsImportDefaultClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsImportDefaultClause")
			.field("local_name", &support::DebugSyntaxResult(self.local_name()))
			.field("from_token", &support::DebugSyntaxResult(self.from_token()))
			.field("source", &support::DebugSyntaxResult(self.source()))
			.field(
				"assertion",
				&support::DebugOptionalElement(self.assertion()),
			)
			.finish()
	}
}
impl AstNode for JsImportNamespaceClause {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_IMPORT_NAMESPACE_CLAUSE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsImportNamespaceClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsImportNamespaceClause")
			.field("star_token", &support::DebugSyntaxResult(self.star_token()))
			.field("as_token", &support::DebugSyntaxResult(self.as_token()))
			.field("local_name", &support::DebugSyntaxResult(self.local_name()))
			.field("from_token", &support::DebugSyntaxResult(self.from_token()))
			.field("source", &support::DebugSyntaxResult(self.source()))
			.field(
				"assertion",
				&support::DebugOptionalElement(self.assertion()),
			)
			.finish()
	}
}
impl AstNode for JsModuleSource {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_MODULE_SOURCE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsModuleSource {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsModuleSource")
			.field(
				"value_token",
				&support::DebugSyntaxResult(self.value_token()),
			)
			.finish()
	}
}
impl AstNode for JsImportAssertion {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_IMPORT_ASSERTION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsImportAssertion {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsImportAssertion")
			.field(
				"assert_token",
				&support::DebugSyntaxResult(self.assert_token()),
			)
			.field(
				"l_curly_token",
				&support::DebugSyntaxResult(self.l_curly_token()),
			)
			.field("assertions", &self.assertions())
			.field(
				"r_curly_token",
				&support::DebugSyntaxResult(self.r_curly_token()),
			)
			.finish()
	}
}
impl AstNode for JsDefaultImportSpecifier {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_DEFAULT_IMPORT_SPECIFIER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsDefaultImportSpecifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsDefaultImportSpecifier")
			.field("local_name", &support::DebugSyntaxResult(self.local_name()))
			.field(
				"trailing_comma_token",
				&support::DebugSyntaxResult(self.trailing_comma_token()),
			)
			.finish()
	}
}
impl AstNode for JsNamedImportSpecifierList {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_NAMED_IMPORT_SPECIFIER_LIST }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsNamedImportSpecifierList {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsNamedImportSpecifierList")
			.field(
				"l_curly_token",
				&support::DebugSyntaxResult(self.l_curly_token()),
			)
			.field("specifiers", &self.specifiers())
			.field(
				"r_curly_token",
				&support::DebugSyntaxResult(self.r_curly_token()),
			)
			.finish()
	}
}
impl AstNode for JsNamespaceImportSpecifier {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_NAMESPACE_IMPORT_SPECIFIER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsNamespaceImportSpecifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsNamespaceImportSpecifier")
			.field("star_token", &support::DebugSyntaxResult(self.star_token()))
			.field("as_token", &support::DebugSyntaxResult(self.as_token()))
			.field("local_name", &support::DebugSyntaxResult(self.local_name()))
			.finish()
	}
}
impl AstNode for JsShorthandNamedImportSpecifier {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_SHORTHAND_NAMED_IMPORT_SPECIFIER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsShorthandNamedImportSpecifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsShorthandNamedImportSpecifier")
			.field("local_name", &support::DebugSyntaxResult(self.local_name()))
			.finish()
	}
}
impl AstNode for JsNamedImportSpecifier {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_NAMED_IMPORT_SPECIFIER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsNamedImportSpecifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsNamedImportSpecifier")
			.field("name", &support::DebugSyntaxResult(self.name()))
			.field("as_token", &support::DebugSyntaxResult(self.as_token()))
			.field("local_name", &support::DebugSyntaxResult(self.local_name()))
			.finish()
	}
}
impl AstNode for JsLiteralExportName {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_LITERAL_EXPORT_NAME }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsLiteralExportName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsLiteralExportName")
			.field("value", &support::DebugSyntaxResult(self.value()))
			.finish()
	}
}
impl AstNode for JsImportAssertionEntry {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_IMPORT_ASSERTION_ENTRY }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsImportAssertionEntry {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsImportAssertionEntry")
			.field("key", &support::DebugSyntaxResult(self.key()))
			.field(
				"colon_token",
				&support::DebugSyntaxResult(self.colon_token()),
			)
			.field(
				"value_token",
				&support::DebugSyntaxResult(self.value_token()),
			)
			.finish()
	}
}
impl AstNode for Specifier {
	fn can_cast(kind: SyntaxKind) -> bool { kind == SPECIFIER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for Specifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Specifier")
			.field("name", &support::DebugSyntaxResult(self.name()))
			.field("as_token", &support::DebugOptionalElement(self.as_token()))
			.field("alias", &support::DebugOptionalElement(self.alias()))
			.finish()
	}
}
impl AstNode for JsPrivateName {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_PRIVATE_NAME }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsPrivateName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsPrivateName")
			.field("hash_token", &support::DebugSyntaxResult(self.hash_token()))
			.field(
				"value_token",
				&support::DebugSyntaxResult(self.value_token()),
			)
			.finish()
	}
}
impl AstNode for JsRestParameter {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_REST_PARAMETER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsRestParameter {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsRestParameter")
			.field(
				"dotdotdot_token",
				&support::DebugSyntaxResult(self.dotdotdot_token()),
			)
			.field("binding", &support::DebugSyntaxResult(self.binding()))
			.finish()
	}
}
impl AstNode for TsExternalModuleRef {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_EXTERNAL_MODULE_REF }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsExternalModuleRef {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsExternalModuleRef")
			.field(
				"require_token",
				&support::DebugSyntaxResult(self.require_token()),
			)
			.field(
				"l_paren_token",
				&support::DebugSyntaxResult(self.l_paren_token()),
			)
			.field(
				"module_token",
				&support::DebugSyntaxResult(self.module_token()),
			)
			.field(
				"r_paren_token",
				&support::DebugSyntaxResult(self.r_paren_token()),
			)
			.finish()
	}
}
impl AstNode for TsAny {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_ANY }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsAny {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsAny")
			.field("any_token", &support::DebugSyntaxResult(self.any_token()))
			.finish()
	}
}
impl AstNode for TsUnknown {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_UNKNOWN }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsUnknown {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsUnknown")
			.field(
				"unknown_token",
				&support::DebugSyntaxResult(self.unknown_token()),
			)
			.finish()
	}
}
impl AstNode for TsNumber {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_NUMBER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsNumber {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsNumber")
			.field("ident", &support::DebugSyntaxResult(self.ident()))
			.finish()
	}
}
impl AstNode for TsObject {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_OBJECT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsObject {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsObject")
			.field("ident", &support::DebugSyntaxResult(self.ident()))
			.finish()
	}
}
impl AstNode for TsBoolean {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_BOOLEAN }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsBoolean {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsBoolean")
			.field("ident", &support::DebugSyntaxResult(self.ident()))
			.finish()
	}
}
impl AstNode for TsBigint {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_BIGINT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsBigint {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsBigint")
			.field("ident", &support::DebugSyntaxResult(self.ident()))
			.finish()
	}
}
impl AstNode for TsString {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_STRING }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsString {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsString")
			.field("ident", &support::DebugSyntaxResult(self.ident()))
			.finish()
	}
}
impl AstNode for TsSymbol {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_SYMBOL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsSymbol {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsSymbol")
			.field("ident", &support::DebugSyntaxResult(self.ident()))
			.finish()
	}
}
impl AstNode for TsVoid {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_VOID }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsVoid {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsVoid")
			.field("void_token", &support::DebugSyntaxResult(self.void_token()))
			.finish()
	}
}
impl AstNode for TsUndefined {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_UNDEFINED }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsUndefined {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsUndefined")
			.field(
				"undefined_token",
				&support::DebugSyntaxResult(self.undefined_token()),
			)
			.finish()
	}
}
impl AstNode for TsNull {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_NULL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsNull {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsNull")
			.field("null_token", &support::DebugSyntaxResult(self.null_token()))
			.finish()
	}
}
impl AstNode for TsNever {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_NEVER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsNever {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsNever")
			.field(
				"never_token",
				&support::DebugSyntaxResult(self.never_token()),
			)
			.finish()
	}
}
impl AstNode for TsThis {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_THIS }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsThis {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsThis")
			.field("this_token", &support::DebugSyntaxResult(self.this_token()))
			.finish()
	}
}
impl AstNode for TsLiteral {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_LITERAL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsLiteral {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsLiteral")
			.field("ident", &support::DebugSyntaxResult(self.ident()))
			.finish()
	}
}
impl AstNode for TsPredicate {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_PREDICATE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsPredicate {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsPredicate")
			.field("lhs", &support::DebugSyntaxResult(self.lhs()))
			.field("rhs", &support::DebugSyntaxResult(self.rhs()))
			.finish()
	}
}
impl AstNode for TsTuple {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_TUPLE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsTuple {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsTuple")
			.field(
				"l_brack_token",
				&support::DebugSyntaxResult(self.l_brack_token()),
			)
			.field("elements", &support::DebugSyntaxResult(self.elements()))
			.field(
				"r_brack_token",
				&support::DebugSyntaxResult(self.r_brack_token()),
			)
			.finish()
	}
}
impl AstNode for TsParen {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_PAREN }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsParen {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsParen")
			.field(
				"l_paren_token",
				&support::DebugSyntaxResult(self.l_paren_token()),
			)
			.field("ty", &support::DebugSyntaxResult(self.ty()))
			.field(
				"r_paren_token",
				&support::DebugSyntaxResult(self.r_paren_token()),
			)
			.finish()
	}
}
impl AstNode for TsTypeRef {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_TYPE_REF }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsTypeRef {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsTypeRef")
			.field("name", &support::DebugSyntaxResult(self.name()))
			.field("type_args", &support::DebugSyntaxResult(self.type_args()))
			.finish()
	}
}
impl AstNode for TsTemplate {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_TEMPLATE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsTemplate {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsTemplate")
			.field("elements", &support::DebugSyntaxResult(self.elements()))
			.finish()
	}
}
impl AstNode for TsMappedType {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_MAPPED_TYPE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsMappedType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsMappedType")
			.field(
				"l_curly_token",
				&support::DebugSyntaxResult(self.l_curly_token()),
			)
			.field(
				"readonly_modifier",
				&support::DebugOptionalElement(self.readonly_modifier()),
			)
			.field(
				"minus_token",
				&support::DebugOptionalElement(self.minus_token()),
			)
			.field(
				"plus_token",
				&support::DebugOptionalElement(self.plus_token()),
			)
			.field(
				"question_mark_token",
				&support::DebugOptionalElement(self.question_mark_token()),
			)
			.field("param", &support::DebugSyntaxResult(self.param()))
			.field(
				"colon_token",
				&support::DebugSyntaxResult(self.colon_token()),
			)
			.field("ty", &support::DebugSyntaxResult(self.ty()))
			.field(
				"r_curly_token",
				&support::DebugSyntaxResult(self.r_curly_token()),
			)
			.field(
				"semicolon_token",
				&support::DebugOptionalElement(self.semicolon_token()),
			)
			.finish()
	}
}
impl AstNode for TsImport {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_IMPORT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsImport {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsImport")
			.field(
				"import_token",
				&support::DebugSyntaxResult(self.import_token()),
			)
			.field("type_args", &support::DebugSyntaxResult(self.type_args()))
			.field(
				"dot_token",
				&support::DebugOptionalElement(self.dot_token()),
			)
			.field(
				"l_paren_token",
				&support::DebugSyntaxResult(self.l_paren_token()),
			)
			.field("qualifier", &support::DebugSyntaxResult(self.qualifier()))
			.field(
				"r_paren_token",
				&support::DebugSyntaxResult(self.r_paren_token()),
			)
			.finish()
	}
}
impl AstNode for TsArray {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_ARRAY }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsArray {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsArray")
			.field(
				"l_brack_token",
				&support::DebugSyntaxResult(self.l_brack_token()),
			)
			.field("ty", &support::DebugSyntaxResult(self.ty()))
			.field(
				"r_brack_token",
				&support::DebugSyntaxResult(self.r_brack_token()),
			)
			.finish()
	}
}
impl AstNode for TsIndexedArray {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_INDEXED_ARRAY }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsIndexedArray {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsIndexedArray")
			.field(
				"l_brack_token",
				&support::DebugSyntaxResult(self.l_brack_token()),
			)
			.field("ty", &support::DebugSyntaxResult(self.ty()))
			.field(
				"r_brack_token",
				&support::DebugSyntaxResult(self.r_brack_token()),
			)
			.finish()
	}
}
impl AstNode for TsTypeOperator {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_TYPE_OPERATOR }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsTypeOperator {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsTypeOperator")
			.field("ty", &support::DebugSyntaxResult(self.ty()))
			.finish()
	}
}
impl AstNode for TsIntersection {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_INTERSECTION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsIntersection {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsIntersection")
			.field("types", &self.types())
			.finish()
	}
}
impl AstNode for TsUnion {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_UNION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsUnion {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsUnion")
			.field("types", &self.types())
			.finish()
	}
}
impl AstNode for TsFnType {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_FN_TYPE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsFnType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsFnType")
			.field("params", &support::DebugSyntaxResult(self.params()))
			.field(
				"fat_arrow_token",
				&support::DebugSyntaxResult(self.fat_arrow_token()),
			)
			.field(
				"return_type",
				&support::DebugOptionalElement(self.return_type()),
			)
			.finish()
	}
}
impl AstNode for TsConstructorType {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_CONSTRUCTOR_TYPE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsConstructorType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsConstructorType")
			.field("new_token", &support::DebugSyntaxResult(self.new_token()))
			.field("params", &support::DebugSyntaxResult(self.params()))
			.field(
				"colon_token",
				&support::DebugSyntaxResult(self.colon_token()),
			)
			.field(
				"return_type",
				&support::DebugOptionalElement(self.return_type()),
			)
			.finish()
	}
}
impl AstNode for TsConditionalType {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_CONDITIONAL_TYPE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsConditionalType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsConditionalType")
			.field("ty", &support::DebugSyntaxResult(self.ty()))
			.field(
				"question_mark_token",
				&support::DebugSyntaxResult(self.question_mark_token()),
			)
			.field(
				"colon_token",
				&support::DebugSyntaxResult(self.colon_token()),
			)
			.field("extends", &support::DebugSyntaxResult(self.extends()))
			.finish()
	}
}
impl AstNode for TsObjectType {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_OBJECT_TYPE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsObjectType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsObjectType")
			.field(
				"l_curly_token",
				&support::DebugSyntaxResult(self.l_curly_token()),
			)
			.field("members", &self.members())
			.field(
				"r_curly_token",
				&support::DebugSyntaxResult(self.r_curly_token()),
			)
			.finish()
	}
}
impl AstNode for TsInfer {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_INFER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsInfer {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsInfer")
			.field(
				"infer_token",
				&support::DebugSyntaxResult(self.infer_token()),
			)
			.field("ident", &support::DebugSyntaxResult(self.ident()))
			.finish()
	}
}
impl AstNode for TsTupleElement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_TUPLE_ELEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsTupleElement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsTupleElement")
			.field("ident", &support::DebugSyntaxResult(self.ident()))
			.field(
				"colon_token",
				&support::DebugSyntaxResult(self.colon_token()),
			)
			.field(
				"question_mark_token",
				&support::DebugSyntaxResult(self.question_mark_token()),
			)
			.field(
				"dotdotdot_token",
				&support::DebugOptionalElement(self.dotdotdot_token()),
			)
			.field("ty", &support::DebugSyntaxResult(self.ty()))
			.finish()
	}
}
impl AstNode for TsEnumMember {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_ENUM_MEMBER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsEnumMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsEnumMember")
			.field("ident", &support::DebugSyntaxResult(self.ident()))
			.field("eq_token", &support::DebugSyntaxResult(self.eq_token()))
			.field("value", &support::DebugSyntaxResult(self.value()))
			.finish()
	}
}
impl AstNode for TsTemplateElement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_TEMPLATE_ELEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsTemplateElement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsTemplateElement")
			.field("ty", &support::DebugSyntaxResult(self.ty()))
			.field(
				"r_curly_token",
				&support::DebugSyntaxResult(self.r_curly_token()),
			)
			.finish()
	}
}
impl AstNode for TsMappedTypeReadonly {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_MAPPED_TYPE_READONLY }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsMappedTypeReadonly {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsMappedTypeReadonly")
			.field(
				"minus_token",
				&support::DebugOptionalElement(self.minus_token()),
			)
			.field(
				"plus_token",
				&support::DebugOptionalElement(self.plus_token()),
			)
			.field(
				"readonly_token",
				&support::DebugOptionalElement(self.readonly_token()),
			)
			.finish()
	}
}
impl AstNode for TsMappedTypeParam {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_MAPPED_TYPE_PARAM }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsMappedTypeParam {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsMappedTypeParam")
			.field(
				"l_brack_token",
				&support::DebugOptionalElement(self.l_brack_token()),
			)
			.field("name", &support::DebugOptionalElement(self.name()))
			.field(
				"r_brack_token",
				&support::DebugOptionalElement(self.r_brack_token()),
			)
			.field("ident", &support::DebugOptionalElement(self.ident()))
			.field("ty", &support::DebugSyntaxResult(self.ty()))
			.finish()
	}
}
impl AstNode for TsTypeName {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_TYPE_NAME }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsTypeName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsTypeName")
			.field("ident", &support::DebugSyntaxResult(self.ident()))
			.finish()
	}
}
impl AstNode for TsExtends {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_EXTENDS }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsExtends {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsExtends")
			.field(
				"extends_token",
				&support::DebugSyntaxResult(self.extends_token()),
			)
			.field("ty", &support::DebugSyntaxResult(self.ty()))
			.finish()
	}
}
impl AstNode for TsModuleBlock {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_MODULE_BLOCK }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsModuleBlock {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsModuleBlock")
			.field(
				"l_curly_token",
				&support::DebugSyntaxResult(self.l_curly_token()),
			)
			.field("items", &support::DebugSyntaxResult(self.items()))
			.field(
				"r_curly_token",
				&support::DebugSyntaxResult(self.r_curly_token()),
			)
			.finish()
	}
}
impl AstNode for TsTypeParam {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_TYPE_PARAM }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsTypeParam {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsTypeParam")
			.field("ident", &support::DebugSyntaxResult(self.ident()))
			.field("constraint", &support::DebugSyntaxResult(self.constraint()))
			.field("default", &support::DebugSyntaxResult(self.default()))
			.finish()
	}
}
impl AstNode for TsConstraint {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_CONSTRAINT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsConstraint {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsConstraint")
			.field(
				"extends_token",
				&support::DebugSyntaxResult(self.extends_token()),
			)
			.field("ty", &support::DebugSyntaxResult(self.ty()))
			.finish()
	}
}
impl AstNode for TsDefault {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_DEFAULT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsDefault {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsDefault")
			.field("eq_token", &support::DebugSyntaxResult(self.eq_token()))
			.field("ty", &support::DebugSyntaxResult(self.ty()))
			.finish()
	}
}
impl AstNode for TsCallSignatureDecl {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_CALL_SIGNATURE_DECL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsCallSignatureDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsCallSignatureDecl")
			.field(
				"type_params",
				&support::DebugSyntaxResult(self.type_params()),
			)
			.field("parameters", &support::DebugSyntaxResult(self.parameters()))
			.field(
				"colon_token",
				&support::DebugSyntaxResult(self.colon_token()),
			)
			.field(
				"return_type",
				&support::DebugSyntaxResult(self.return_type()),
			)
			.finish()
	}
}
impl AstNode for TsConstructSignatureDecl {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_CONSTRUCT_SIGNATURE_DECL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsConstructSignatureDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsConstructSignatureDecl")
			.field("new_token", &support::DebugSyntaxResult(self.new_token()))
			.field(
				"type_params",
				&support::DebugSyntaxResult(self.type_params()),
			)
			.field("parameters", &support::DebugSyntaxResult(self.parameters()))
			.field(
				"colon_token",
				&support::DebugOptionalElement(self.colon_token()),
			)
			.field(
				"return_type",
				&support::DebugSyntaxResult(self.return_type()),
			)
			.finish()
	}
}
impl AstNode for TsPropertySignature {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_PROPERTY_SIGNATURE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsPropertySignature {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsPropertySignature")
			.field(
				"readonly_token",
				&support::DebugOptionalElement(self.readonly_token()),
			)
			.field("prop", &support::DebugSyntaxResult(self.prop()))
			.field(
				"question_mark_token",
				&support::DebugSyntaxResult(self.question_mark_token()),
			)
			.field(
				"colon_token",
				&support::DebugSyntaxResult(self.colon_token()),
			)
			.field("ty", &support::DebugSyntaxResult(self.ty()))
			.finish()
	}
}
impl AstNode for TsMethodSignature {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_METHOD_SIGNATURE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsMethodSignature {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsMethodSignature")
			.field(
				"readonly_token",
				&support::DebugOptionalElement(self.readonly_token()),
			)
			.field("key", &support::DebugSyntaxResult(self.key()))
			.field(
				"type_params",
				&support::DebugSyntaxResult(self.type_params()),
			)
			.field("parameters", &support::DebugSyntaxResult(self.parameters()))
			.field(
				"question_mark_token",
				&support::DebugOptionalElement(self.question_mark_token()),
			)
			.field(
				"colon_token",
				&support::DebugSyntaxResult(self.colon_token()),
			)
			.field(
				"return_type",
				&support::DebugSyntaxResult(self.return_type()),
			)
			.finish()
	}
}
impl AstNode for TsQualifiedPath {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_QUALIFIED_PATH }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsQualifiedPath {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsQualifiedPath")
			.field("lhs", &support::DebugSyntaxResult(self.lhs()))
			.field("dot_token", &support::DebugSyntaxResult(self.dot_token()))
			.field("rhs", &support::DebugSyntaxResult(self.rhs()))
			.finish()
	}
}
impl From<JsScript> for JsAnyRoot {
	fn from(node: JsScript) -> JsAnyRoot { JsAnyRoot::JsScript(node) }
}
impl From<JsModule> for JsAnyRoot {
	fn from(node: JsModule) -> JsAnyRoot { JsAnyRoot::JsModule(node) }
}
impl AstNode for JsAnyRoot {
	fn can_cast(kind: SyntaxKind) -> bool { matches!(kind, JS_SCRIPT | JS_MODULE) }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_SCRIPT => JsAnyRoot::JsScript(JsScript { syntax }),
			JS_MODULE => JsAnyRoot::JsModule(JsModule { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsAnyRoot::JsScript(it) => &it.syntax,
			JsAnyRoot::JsModule(it) => &it.syntax,
		}
	}
}
impl std::fmt::Debug for JsAnyRoot {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			JsAnyRoot::JsScript(it) => std::fmt::Debug::fmt(it, f),
			JsAnyRoot::JsModule(it) => std::fmt::Debug::fmt(it, f),
		}
	}
}
impl From<JsBlockStatement> for JsAnyStatement {
	fn from(node: JsBlockStatement) -> JsAnyStatement { JsAnyStatement::JsBlockStatement(node) }
}
impl From<JsEmptyStatement> for JsAnyStatement {
	fn from(node: JsEmptyStatement) -> JsAnyStatement { JsAnyStatement::JsEmptyStatement(node) }
}
impl From<JsExpressionStatement> for JsAnyStatement {
	fn from(node: JsExpressionStatement) -> JsAnyStatement {
		JsAnyStatement::JsExpressionStatement(node)
	}
}
impl From<JsIfStatement> for JsAnyStatement {
	fn from(node: JsIfStatement) -> JsAnyStatement { JsAnyStatement::JsIfStatement(node) }
}
impl From<JsDoWhileStatement> for JsAnyStatement {
	fn from(node: JsDoWhileStatement) -> JsAnyStatement { JsAnyStatement::JsDoWhileStatement(node) }
}
impl From<JsWhileStatement> for JsAnyStatement {
	fn from(node: JsWhileStatement) -> JsAnyStatement { JsAnyStatement::JsWhileStatement(node) }
}
impl From<ForStmt> for JsAnyStatement {
	fn from(node: ForStmt) -> JsAnyStatement { JsAnyStatement::ForStmt(node) }
}
impl From<ForInStmt> for JsAnyStatement {
	fn from(node: ForInStmt) -> JsAnyStatement { JsAnyStatement::ForInStmt(node) }
}
impl From<ForOfStmt> for JsAnyStatement {
	fn from(node: ForOfStmt) -> JsAnyStatement { JsAnyStatement::ForOfStmt(node) }
}
impl From<JsContinueStatement> for JsAnyStatement {
	fn from(node: JsContinueStatement) -> JsAnyStatement {
		JsAnyStatement::JsContinueStatement(node)
	}
}
impl From<JsBreakStatement> for JsAnyStatement {
	fn from(node: JsBreakStatement) -> JsAnyStatement { JsAnyStatement::JsBreakStatement(node) }
}
impl From<JsReturnStatement> for JsAnyStatement {
	fn from(node: JsReturnStatement) -> JsAnyStatement { JsAnyStatement::JsReturnStatement(node) }
}
impl From<JsWithStatement> for JsAnyStatement {
	fn from(node: JsWithStatement) -> JsAnyStatement { JsAnyStatement::JsWithStatement(node) }
}
impl From<JsLabeledStatement> for JsAnyStatement {
	fn from(node: JsLabeledStatement) -> JsAnyStatement { JsAnyStatement::JsLabeledStatement(node) }
}
impl From<JsSwitchStatement> for JsAnyStatement {
	fn from(node: JsSwitchStatement) -> JsAnyStatement { JsAnyStatement::JsSwitchStatement(node) }
}
impl From<JsThrowStatement> for JsAnyStatement {
	fn from(node: JsThrowStatement) -> JsAnyStatement { JsAnyStatement::JsThrowStatement(node) }
}
impl From<JsTryStatement> for JsAnyStatement {
	fn from(node: JsTryStatement) -> JsAnyStatement { JsAnyStatement::JsTryStatement(node) }
}
impl From<JsTryFinallyStatement> for JsAnyStatement {
	fn from(node: JsTryFinallyStatement) -> JsAnyStatement {
		JsAnyStatement::JsTryFinallyStatement(node)
	}
}
impl From<JsDebuggerStatement> for JsAnyStatement {
	fn from(node: JsDebuggerStatement) -> JsAnyStatement {
		JsAnyStatement::JsDebuggerStatement(node)
	}
}
impl From<JsFunctionDeclaration> for JsAnyStatement {
	fn from(node: JsFunctionDeclaration) -> JsAnyStatement {
		JsAnyStatement::JsFunctionDeclaration(node)
	}
}
impl From<JsClassDeclaration> for JsAnyStatement {
	fn from(node: JsClassDeclaration) -> JsAnyStatement { JsAnyStatement::JsClassDeclaration(node) }
}
impl From<JsVariableDeclarationStatement> for JsAnyStatement {
	fn from(node: JsVariableDeclarationStatement) -> JsAnyStatement {
		JsAnyStatement::JsVariableDeclarationStatement(node)
	}
}
impl From<TsEnum> for JsAnyStatement {
	fn from(node: TsEnum) -> JsAnyStatement { JsAnyStatement::TsEnum(node) }
}
impl From<TsTypeAliasDecl> for JsAnyStatement {
	fn from(node: TsTypeAliasDecl) -> JsAnyStatement { JsAnyStatement::TsTypeAliasDecl(node) }
}
impl From<TsNamespaceDecl> for JsAnyStatement {
	fn from(node: TsNamespaceDecl) -> JsAnyStatement { JsAnyStatement::TsNamespaceDecl(node) }
}
impl From<TsModuleDecl> for JsAnyStatement {
	fn from(node: TsModuleDecl) -> JsAnyStatement { JsAnyStatement::TsModuleDecl(node) }
}
impl From<TsInterfaceDecl> for JsAnyStatement {
	fn from(node: TsInterfaceDecl) -> JsAnyStatement { JsAnyStatement::TsInterfaceDecl(node) }
}
impl From<JsUnknownStatement> for JsAnyStatement {
	fn from(node: JsUnknownStatement) -> JsAnyStatement { JsAnyStatement::JsUnknownStatement(node) }
}
impl AstNode for JsAnyStatement {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(
			kind,
			JS_BLOCK_STATEMENT
				| JS_EMPTY_STATEMENT
				| JS_EXPRESSION_STATEMENT
				| JS_IF_STATEMENT
				| JS_DO_WHILE_STATEMENT
				| JS_WHILE_STATEMENT
				| FOR_STMT | FOR_IN_STMT
				| FOR_OF_STMT | JS_CONTINUE_STATEMENT
				| JS_BREAK_STATEMENT
				| JS_RETURN_STATEMENT
				| JS_WITH_STATEMENT
				| JS_LABELED_STATEMENT
				| JS_SWITCH_STATEMENT
				| JS_THROW_STATEMENT
				| JS_TRY_STATEMENT
				| JS_TRY_FINALLY_STATEMENT
				| JS_DEBUGGER_STATEMENT
				| JS_FUNCTION_DECLARATION
				| JS_CLASS_DECLARATION
				| JS_VARIABLE_DECLARATION_STATEMENT
				| TS_ENUM | TS_TYPE_ALIAS_DECL
				| TS_NAMESPACE_DECL
				| TS_MODULE_DECL | TS_INTERFACE_DECL
				| JS_UNKNOWN_STATEMENT
		)
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_BLOCK_STATEMENT => JsAnyStatement::JsBlockStatement(JsBlockStatement { syntax }),
			JS_EMPTY_STATEMENT => JsAnyStatement::JsEmptyStatement(JsEmptyStatement { syntax }),
			JS_EXPRESSION_STATEMENT => {
				JsAnyStatement::JsExpressionStatement(JsExpressionStatement { syntax })
			}
			JS_IF_STATEMENT => JsAnyStatement::JsIfStatement(JsIfStatement { syntax }),
			JS_DO_WHILE_STATEMENT => {
				JsAnyStatement::JsDoWhileStatement(JsDoWhileStatement { syntax })
			}
			JS_WHILE_STATEMENT => JsAnyStatement::JsWhileStatement(JsWhileStatement { syntax }),
			FOR_STMT => JsAnyStatement::ForStmt(ForStmt { syntax }),
			FOR_IN_STMT => JsAnyStatement::ForInStmt(ForInStmt { syntax }),
			FOR_OF_STMT => JsAnyStatement::ForOfStmt(ForOfStmt { syntax }),
			JS_CONTINUE_STATEMENT => {
				JsAnyStatement::JsContinueStatement(JsContinueStatement { syntax })
			}
			JS_BREAK_STATEMENT => JsAnyStatement::JsBreakStatement(JsBreakStatement { syntax }),
			JS_RETURN_STATEMENT => JsAnyStatement::JsReturnStatement(JsReturnStatement { syntax }),
			JS_WITH_STATEMENT => JsAnyStatement::JsWithStatement(JsWithStatement { syntax }),
			JS_LABELED_STATEMENT => {
				JsAnyStatement::JsLabeledStatement(JsLabeledStatement { syntax })
			}
			JS_SWITCH_STATEMENT => JsAnyStatement::JsSwitchStatement(JsSwitchStatement { syntax }),
			JS_THROW_STATEMENT => JsAnyStatement::JsThrowStatement(JsThrowStatement { syntax }),
			JS_TRY_STATEMENT => JsAnyStatement::JsTryStatement(JsTryStatement { syntax }),
			JS_TRY_FINALLY_STATEMENT => {
				JsAnyStatement::JsTryFinallyStatement(JsTryFinallyStatement { syntax })
			}
			JS_DEBUGGER_STATEMENT => {
				JsAnyStatement::JsDebuggerStatement(JsDebuggerStatement { syntax })
			}
			JS_FUNCTION_DECLARATION => {
				JsAnyStatement::JsFunctionDeclaration(JsFunctionDeclaration { syntax })
			}
			JS_CLASS_DECLARATION => {
				JsAnyStatement::JsClassDeclaration(JsClassDeclaration { syntax })
			}
			JS_VARIABLE_DECLARATION_STATEMENT => {
				JsAnyStatement::JsVariableDeclarationStatement(JsVariableDeclarationStatement {
					syntax,
				})
			}
			TS_ENUM => JsAnyStatement::TsEnum(TsEnum { syntax }),
			TS_TYPE_ALIAS_DECL => JsAnyStatement::TsTypeAliasDecl(TsTypeAliasDecl { syntax }),
			TS_NAMESPACE_DECL => JsAnyStatement::TsNamespaceDecl(TsNamespaceDecl { syntax }),
			TS_MODULE_DECL => JsAnyStatement::TsModuleDecl(TsModuleDecl { syntax }),
			TS_INTERFACE_DECL => JsAnyStatement::TsInterfaceDecl(TsInterfaceDecl { syntax }),
			JS_UNKNOWN_STATEMENT => {
				JsAnyStatement::JsUnknownStatement(JsUnknownStatement { syntax })
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsAnyStatement::JsBlockStatement(it) => &it.syntax,
			JsAnyStatement::JsEmptyStatement(it) => &it.syntax,
			JsAnyStatement::JsExpressionStatement(it) => &it.syntax,
			JsAnyStatement::JsIfStatement(it) => &it.syntax,
			JsAnyStatement::JsDoWhileStatement(it) => &it.syntax,
			JsAnyStatement::JsWhileStatement(it) => &it.syntax,
			JsAnyStatement::ForStmt(it) => &it.syntax,
			JsAnyStatement::ForInStmt(it) => &it.syntax,
			JsAnyStatement::ForOfStmt(it) => &it.syntax,
			JsAnyStatement::JsContinueStatement(it) => &it.syntax,
			JsAnyStatement::JsBreakStatement(it) => &it.syntax,
			JsAnyStatement::JsReturnStatement(it) => &it.syntax,
			JsAnyStatement::JsWithStatement(it) => &it.syntax,
			JsAnyStatement::JsLabeledStatement(it) => &it.syntax,
			JsAnyStatement::JsSwitchStatement(it) => &it.syntax,
			JsAnyStatement::JsThrowStatement(it) => &it.syntax,
			JsAnyStatement::JsTryStatement(it) => &it.syntax,
			JsAnyStatement::JsTryFinallyStatement(it) => &it.syntax,
			JsAnyStatement::JsDebuggerStatement(it) => &it.syntax,
			JsAnyStatement::JsFunctionDeclaration(it) => &it.syntax,
			JsAnyStatement::JsClassDeclaration(it) => &it.syntax,
			JsAnyStatement::JsVariableDeclarationStatement(it) => &it.syntax,
			JsAnyStatement::TsEnum(it) => &it.syntax,
			JsAnyStatement::TsTypeAliasDecl(it) => &it.syntax,
			JsAnyStatement::TsNamespaceDecl(it) => &it.syntax,
			JsAnyStatement::TsModuleDecl(it) => &it.syntax,
			JsAnyStatement::TsInterfaceDecl(it) => &it.syntax,
			JsAnyStatement::JsUnknownStatement(it) => &it.syntax,
		}
	}
}
impl std::fmt::Debug for JsAnyStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			JsAnyStatement::JsBlockStatement(it) => std::fmt::Debug::fmt(it, f),
			JsAnyStatement::JsEmptyStatement(it) => std::fmt::Debug::fmt(it, f),
			JsAnyStatement::JsExpressionStatement(it) => std::fmt::Debug::fmt(it, f),
			JsAnyStatement::JsIfStatement(it) => std::fmt::Debug::fmt(it, f),
			JsAnyStatement::JsDoWhileStatement(it) => std::fmt::Debug::fmt(it, f),
			JsAnyStatement::JsWhileStatement(it) => std::fmt::Debug::fmt(it, f),
			JsAnyStatement::ForStmt(it) => std::fmt::Debug::fmt(it, f),
			JsAnyStatement::ForInStmt(it) => std::fmt::Debug::fmt(it, f),
			JsAnyStatement::ForOfStmt(it) => std::fmt::Debug::fmt(it, f),
			JsAnyStatement::JsContinueStatement(it) => std::fmt::Debug::fmt(it, f),
			JsAnyStatement::JsBreakStatement(it) => std::fmt::Debug::fmt(it, f),
			JsAnyStatement::JsReturnStatement(it) => std::fmt::Debug::fmt(it, f),
			JsAnyStatement::JsWithStatement(it) => std::fmt::Debug::fmt(it, f),
			JsAnyStatement::JsLabeledStatement(it) => std::fmt::Debug::fmt(it, f),
			JsAnyStatement::JsSwitchStatement(it) => std::fmt::Debug::fmt(it, f),
			JsAnyStatement::JsThrowStatement(it) => std::fmt::Debug::fmt(it, f),
			JsAnyStatement::JsTryStatement(it) => std::fmt::Debug::fmt(it, f),
			JsAnyStatement::JsTryFinallyStatement(it) => std::fmt::Debug::fmt(it, f),
			JsAnyStatement::JsDebuggerStatement(it) => std::fmt::Debug::fmt(it, f),
			JsAnyStatement::JsFunctionDeclaration(it) => std::fmt::Debug::fmt(it, f),
			JsAnyStatement::JsClassDeclaration(it) => std::fmt::Debug::fmt(it, f),
			JsAnyStatement::JsVariableDeclarationStatement(it) => std::fmt::Debug::fmt(it, f),
			JsAnyStatement::TsEnum(it) => std::fmt::Debug::fmt(it, f),
			JsAnyStatement::TsTypeAliasDecl(it) => std::fmt::Debug::fmt(it, f),
			JsAnyStatement::TsNamespaceDecl(it) => std::fmt::Debug::fmt(it, f),
			JsAnyStatement::TsModuleDecl(it) => std::fmt::Debug::fmt(it, f),
			JsAnyStatement::TsInterfaceDecl(it) => std::fmt::Debug::fmt(it, f),
			JsAnyStatement::JsUnknownStatement(it) => std::fmt::Debug::fmt(it, f),
		}
	}
}
impl From<JsImport> for JsAnyModuleItem {
	fn from(node: JsImport) -> JsAnyModuleItem { JsAnyModuleItem::JsImport(node) }
}
impl From<ExportNamed> for JsAnyModuleItem {
	fn from(node: ExportNamed) -> JsAnyModuleItem { JsAnyModuleItem::ExportNamed(node) }
}
impl From<ExportDefaultDecl> for JsAnyModuleItem {
	fn from(node: ExportDefaultDecl) -> JsAnyModuleItem { JsAnyModuleItem::ExportDefaultDecl(node) }
}
impl From<ExportDefaultExpr> for JsAnyModuleItem {
	fn from(node: ExportDefaultExpr) -> JsAnyModuleItem { JsAnyModuleItem::ExportDefaultExpr(node) }
}
impl From<ExportWildcard> for JsAnyModuleItem {
	fn from(node: ExportWildcard) -> JsAnyModuleItem { JsAnyModuleItem::ExportWildcard(node) }
}
impl From<ExportDecl> for JsAnyModuleItem {
	fn from(node: ExportDecl) -> JsAnyModuleItem { JsAnyModuleItem::ExportDecl(node) }
}
impl From<TsImportEqualsDecl> for JsAnyModuleItem {
	fn from(node: TsImportEqualsDecl) -> JsAnyModuleItem {
		JsAnyModuleItem::TsImportEqualsDecl(node)
	}
}
impl From<TsExportAssignment> for JsAnyModuleItem {
	fn from(node: TsExportAssignment) -> JsAnyModuleItem {
		JsAnyModuleItem::TsExportAssignment(node)
	}
}
impl From<TsNamespaceExportDecl> for JsAnyModuleItem {
	fn from(node: TsNamespaceExportDecl) -> JsAnyModuleItem {
		JsAnyModuleItem::TsNamespaceExportDecl(node)
	}
}
impl AstNode for JsAnyModuleItem {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			JS_IMPORT
			| EXPORT_NAMED
			| EXPORT_DEFAULT_DECL
			| EXPORT_DEFAULT_EXPR
			| EXPORT_WILDCARD
			| EXPORT_DECL
			| TS_IMPORT_EQUALS_DECL
			| TS_EXPORT_ASSIGNMENT
			| TS_NAMESPACE_EXPORT_DECL => true,
			k if JsAnyStatement::can_cast(k) => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_IMPORT => JsAnyModuleItem::JsImport(JsImport { syntax }),
			EXPORT_NAMED => JsAnyModuleItem::ExportNamed(ExportNamed { syntax }),
			EXPORT_DEFAULT_DECL => JsAnyModuleItem::ExportDefaultDecl(ExportDefaultDecl { syntax }),
			EXPORT_DEFAULT_EXPR => JsAnyModuleItem::ExportDefaultExpr(ExportDefaultExpr { syntax }),
			EXPORT_WILDCARD => JsAnyModuleItem::ExportWildcard(ExportWildcard { syntax }),
			EXPORT_DECL => JsAnyModuleItem::ExportDecl(ExportDecl { syntax }),
			TS_IMPORT_EQUALS_DECL => {
				JsAnyModuleItem::TsImportEqualsDecl(TsImportEqualsDecl { syntax })
			}
			TS_EXPORT_ASSIGNMENT => {
				JsAnyModuleItem::TsExportAssignment(TsExportAssignment { syntax })
			}
			TS_NAMESPACE_EXPORT_DECL => {
				JsAnyModuleItem::TsNamespaceExportDecl(TsNamespaceExportDecl { syntax })
			}
			_ => {
				if let Some(js_any_statement) = JsAnyStatement::cast(syntax) {
					return Some(JsAnyModuleItem::JsAnyStatement(js_any_statement));
				}
				return None;
			}
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsAnyModuleItem::JsImport(it) => &it.syntax,
			JsAnyModuleItem::ExportNamed(it) => &it.syntax,
			JsAnyModuleItem::ExportDefaultDecl(it) => &it.syntax,
			JsAnyModuleItem::ExportDefaultExpr(it) => &it.syntax,
			JsAnyModuleItem::ExportWildcard(it) => &it.syntax,
			JsAnyModuleItem::ExportDecl(it) => &it.syntax,
			JsAnyModuleItem::TsImportEqualsDecl(it) => &it.syntax,
			JsAnyModuleItem::TsExportAssignment(it) => &it.syntax,
			JsAnyModuleItem::TsNamespaceExportDecl(it) => &it.syntax,
			JsAnyModuleItem::JsAnyStatement(it) => it.syntax(),
		}
	}
}
impl std::fmt::Debug for JsAnyModuleItem {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			JsAnyModuleItem::JsAnyStatement(it) => std::fmt::Debug::fmt(it, f),
			JsAnyModuleItem::JsImport(it) => std::fmt::Debug::fmt(it, f),
			JsAnyModuleItem::ExportNamed(it) => std::fmt::Debug::fmt(it, f),
			JsAnyModuleItem::ExportDefaultDecl(it) => std::fmt::Debug::fmt(it, f),
			JsAnyModuleItem::ExportDefaultExpr(it) => std::fmt::Debug::fmt(it, f),
			JsAnyModuleItem::ExportWildcard(it) => std::fmt::Debug::fmt(it, f),
			JsAnyModuleItem::ExportDecl(it) => std::fmt::Debug::fmt(it, f),
			JsAnyModuleItem::TsImportEqualsDecl(it) => std::fmt::Debug::fmt(it, f),
			JsAnyModuleItem::TsExportAssignment(it) => std::fmt::Debug::fmt(it, f),
			JsAnyModuleItem::TsNamespaceExportDecl(it) => std::fmt::Debug::fmt(it, f),
		}
	}
}
impl From<JsArrayExpression> for JsAnyExpression {
	fn from(node: JsArrayExpression) -> JsAnyExpression { JsAnyExpression::JsArrayExpression(node) }
}
impl From<JsArrowFunctionExpression> for JsAnyExpression {
	fn from(node: JsArrowFunctionExpression) -> JsAnyExpression {
		JsAnyExpression::JsArrowFunctionExpression(node)
	}
}
impl From<JsAssignmentExpression> for JsAnyExpression {
	fn from(node: JsAssignmentExpression) -> JsAnyExpression {
		JsAnyExpression::JsAssignmentExpression(node)
	}
}
impl From<JsAwaitExpression> for JsAnyExpression {
	fn from(node: JsAwaitExpression) -> JsAnyExpression { JsAnyExpression::JsAwaitExpression(node) }
}
impl From<JsBinaryExpression> for JsAnyExpression {
	fn from(node: JsBinaryExpression) -> JsAnyExpression {
		JsAnyExpression::JsBinaryExpression(node)
	}
}
impl From<JsClassExpression> for JsAnyExpression {
	fn from(node: JsClassExpression) -> JsAnyExpression { JsAnyExpression::JsClassExpression(node) }
}
impl From<JsConditionalExpression> for JsAnyExpression {
	fn from(node: JsConditionalExpression) -> JsAnyExpression {
		JsAnyExpression::JsConditionalExpression(node)
	}
}
impl From<JsComputedMemberExpression> for JsAnyExpression {
	fn from(node: JsComputedMemberExpression) -> JsAnyExpression {
		JsAnyExpression::JsComputedMemberExpression(node)
	}
}
impl From<JsFunctionExpression> for JsAnyExpression {
	fn from(node: JsFunctionExpression) -> JsAnyExpression {
		JsAnyExpression::JsFunctionExpression(node)
	}
}
impl From<JsImportCallExpression> for JsAnyExpression {
	fn from(node: JsImportCallExpression) -> JsAnyExpression {
		JsAnyExpression::JsImportCallExpression(node)
	}
}
impl From<JsLogicalExpression> for JsAnyExpression {
	fn from(node: JsLogicalExpression) -> JsAnyExpression {
		JsAnyExpression::JsLogicalExpression(node)
	}
}
impl From<JsObjectExpression> for JsAnyExpression {
	fn from(node: JsObjectExpression) -> JsAnyExpression {
		JsAnyExpression::JsObjectExpression(node)
	}
}
impl From<JsParenthesizedExpression> for JsAnyExpression {
	fn from(node: JsParenthesizedExpression) -> JsAnyExpression {
		JsAnyExpression::JsParenthesizedExpression(node)
	}
}
impl From<JsIdentifierExpression> for JsAnyExpression {
	fn from(node: JsIdentifierExpression) -> JsAnyExpression {
		JsAnyExpression::JsIdentifierExpression(node)
	}
}
impl From<JsSequenceExpression> for JsAnyExpression {
	fn from(node: JsSequenceExpression) -> JsAnyExpression {
		JsAnyExpression::JsSequenceExpression(node)
	}
}
impl From<JsStaticMemberExpression> for JsAnyExpression {
	fn from(node: JsStaticMemberExpression) -> JsAnyExpression {
		JsAnyExpression::JsStaticMemberExpression(node)
	}
}
impl From<JsSuperExpression> for JsAnyExpression {
	fn from(node: JsSuperExpression) -> JsAnyExpression { JsAnyExpression::JsSuperExpression(node) }
}
impl From<JsThisExpression> for JsAnyExpression {
	fn from(node: JsThisExpression) -> JsAnyExpression { JsAnyExpression::JsThisExpression(node) }
}
impl From<JsUnaryExpression> for JsAnyExpression {
	fn from(node: JsUnaryExpression) -> JsAnyExpression { JsAnyExpression::JsUnaryExpression(node) }
}
impl From<JsPreUpdateExpression> for JsAnyExpression {
	fn from(node: JsPreUpdateExpression) -> JsAnyExpression {
		JsAnyExpression::JsPreUpdateExpression(node)
	}
}
impl From<JsPostUpdateExpression> for JsAnyExpression {
	fn from(node: JsPostUpdateExpression) -> JsAnyExpression {
		JsAnyExpression::JsPostUpdateExpression(node)
	}
}
impl From<JsYieldExpression> for JsAnyExpression {
	fn from(node: JsYieldExpression) -> JsAnyExpression { JsAnyExpression::JsYieldExpression(node) }
}
impl From<Template> for JsAnyExpression {
	fn from(node: Template) -> JsAnyExpression { JsAnyExpression::Template(node) }
}
impl From<NewExpr> for JsAnyExpression {
	fn from(node: NewExpr) -> JsAnyExpression { JsAnyExpression::NewExpr(node) }
}
impl From<CallExpr> for JsAnyExpression {
	fn from(node: CallExpr) -> JsAnyExpression { JsAnyExpression::CallExpr(node) }
}
impl From<NewTarget> for JsAnyExpression {
	fn from(node: NewTarget) -> JsAnyExpression { JsAnyExpression::NewTarget(node) }
}
impl From<ImportMeta> for JsAnyExpression {
	fn from(node: ImportMeta) -> JsAnyExpression { JsAnyExpression::ImportMeta(node) }
}
impl From<TsNonNull> for JsAnyExpression {
	fn from(node: TsNonNull) -> JsAnyExpression { JsAnyExpression::TsNonNull(node) }
}
impl From<TsAssertion> for JsAnyExpression {
	fn from(node: TsAssertion) -> JsAnyExpression { JsAnyExpression::TsAssertion(node) }
}
impl From<TsConstAssertion> for JsAnyExpression {
	fn from(node: TsConstAssertion) -> JsAnyExpression { JsAnyExpression::TsConstAssertion(node) }
}
impl From<JsUnknownExpression> for JsAnyExpression {
	fn from(node: JsUnknownExpression) -> JsAnyExpression {
		JsAnyExpression::JsUnknownExpression(node)
	}
}
impl AstNode for JsAnyExpression {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			JS_ARRAY_EXPRESSION
			| JS_ARROW_FUNCTION_EXPRESSION
			| JS_ASSIGNMENT_EXPRESSION
			| JS_AWAIT_EXPRESSION
			| JS_BINARY_EXPRESSION
			| JS_CLASS_EXPRESSION
			| JS_CONDITIONAL_EXPRESSION
			| JS_COMPUTED_MEMBER_EXPRESSION
			| JS_FUNCTION_EXPRESSION
			| JS_IMPORT_CALL_EXPRESSION
			| JS_LOGICAL_EXPRESSION
			| JS_OBJECT_EXPRESSION
			| JS_PARENTHESIZED_EXPRESSION
			| JS_IDENTIFIER_EXPRESSION
			| JS_SEQUENCE_EXPRESSION
			| JS_STATIC_MEMBER_EXPRESSION
			| JS_SUPER_EXPRESSION
			| JS_THIS_EXPRESSION
			| JS_UNARY_EXPRESSION
			| JS_PRE_UPDATE_EXPRESSION
			| JS_POST_UPDATE_EXPRESSION
			| JS_YIELD_EXPRESSION
			| TEMPLATE
			| NEW_EXPR
			| CALL_EXPR
			| NEW_TARGET
			| IMPORT_META
			| TS_NON_NULL
			| TS_ASSERTION
			| TS_CONST_ASSERTION
			| JS_UNKNOWN_EXPRESSION => true,
			k if JsAnyLiteralExpression::can_cast(k) => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_ARRAY_EXPRESSION => JsAnyExpression::JsArrayExpression(JsArrayExpression { syntax }),
			JS_ARROW_FUNCTION_EXPRESSION => {
				JsAnyExpression::JsArrowFunctionExpression(JsArrowFunctionExpression { syntax })
			}
			JS_ASSIGNMENT_EXPRESSION => {
				JsAnyExpression::JsAssignmentExpression(JsAssignmentExpression { syntax })
			}
			JS_AWAIT_EXPRESSION => JsAnyExpression::JsAwaitExpression(JsAwaitExpression { syntax }),
			JS_BINARY_EXPRESSION => {
				JsAnyExpression::JsBinaryExpression(JsBinaryExpression { syntax })
			}
			JS_CLASS_EXPRESSION => JsAnyExpression::JsClassExpression(JsClassExpression { syntax }),
			JS_CONDITIONAL_EXPRESSION => {
				JsAnyExpression::JsConditionalExpression(JsConditionalExpression { syntax })
			}
			JS_COMPUTED_MEMBER_EXPRESSION => {
				JsAnyExpression::JsComputedMemberExpression(JsComputedMemberExpression { syntax })
			}
			JS_FUNCTION_EXPRESSION => {
				JsAnyExpression::JsFunctionExpression(JsFunctionExpression { syntax })
			}
			JS_IMPORT_CALL_EXPRESSION => {
				JsAnyExpression::JsImportCallExpression(JsImportCallExpression { syntax })
			}
			JS_LOGICAL_EXPRESSION => {
				JsAnyExpression::JsLogicalExpression(JsLogicalExpression { syntax })
			}
			JS_OBJECT_EXPRESSION => {
				JsAnyExpression::JsObjectExpression(JsObjectExpression { syntax })
			}
			JS_PARENTHESIZED_EXPRESSION => {
				JsAnyExpression::JsParenthesizedExpression(JsParenthesizedExpression { syntax })
			}
			JS_IDENTIFIER_EXPRESSION => {
				JsAnyExpression::JsIdentifierExpression(JsIdentifierExpression { syntax })
			}
			JS_SEQUENCE_EXPRESSION => {
				JsAnyExpression::JsSequenceExpression(JsSequenceExpression { syntax })
			}
			JS_STATIC_MEMBER_EXPRESSION => {
				JsAnyExpression::JsStaticMemberExpression(JsStaticMemberExpression { syntax })
			}
			JS_SUPER_EXPRESSION => JsAnyExpression::JsSuperExpression(JsSuperExpression { syntax }),
			JS_THIS_EXPRESSION => JsAnyExpression::JsThisExpression(JsThisExpression { syntax }),
			JS_UNARY_EXPRESSION => JsAnyExpression::JsUnaryExpression(JsUnaryExpression { syntax }),
			JS_PRE_UPDATE_EXPRESSION => {
				JsAnyExpression::JsPreUpdateExpression(JsPreUpdateExpression { syntax })
			}
			JS_POST_UPDATE_EXPRESSION => {
				JsAnyExpression::JsPostUpdateExpression(JsPostUpdateExpression { syntax })
			}
			JS_YIELD_EXPRESSION => JsAnyExpression::JsYieldExpression(JsYieldExpression { syntax }),
			TEMPLATE => JsAnyExpression::Template(Template { syntax }),
			NEW_EXPR => JsAnyExpression::NewExpr(NewExpr { syntax }),
			CALL_EXPR => JsAnyExpression::CallExpr(CallExpr { syntax }),
			NEW_TARGET => JsAnyExpression::NewTarget(NewTarget { syntax }),
			IMPORT_META => JsAnyExpression::ImportMeta(ImportMeta { syntax }),
			TS_NON_NULL => JsAnyExpression::TsNonNull(TsNonNull { syntax }),
			TS_ASSERTION => JsAnyExpression::TsAssertion(TsAssertion { syntax }),
			TS_CONST_ASSERTION => JsAnyExpression::TsConstAssertion(TsConstAssertion { syntax }),
			JS_UNKNOWN_EXPRESSION => {
				JsAnyExpression::JsUnknownExpression(JsUnknownExpression { syntax })
			}
			_ => {
				if let Some(js_any_literal_expression) = JsAnyLiteralExpression::cast(syntax) {
					return Some(JsAnyExpression::JsAnyLiteralExpression(
						js_any_literal_expression,
					));
				}
				return None;
			}
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsAnyExpression::JsArrayExpression(it) => &it.syntax,
			JsAnyExpression::JsArrowFunctionExpression(it) => &it.syntax,
			JsAnyExpression::JsAssignmentExpression(it) => &it.syntax,
			JsAnyExpression::JsAwaitExpression(it) => &it.syntax,
			JsAnyExpression::JsBinaryExpression(it) => &it.syntax,
			JsAnyExpression::JsClassExpression(it) => &it.syntax,
			JsAnyExpression::JsConditionalExpression(it) => &it.syntax,
			JsAnyExpression::JsComputedMemberExpression(it) => &it.syntax,
			JsAnyExpression::JsFunctionExpression(it) => &it.syntax,
			JsAnyExpression::JsImportCallExpression(it) => &it.syntax,
			JsAnyExpression::JsLogicalExpression(it) => &it.syntax,
			JsAnyExpression::JsObjectExpression(it) => &it.syntax,
			JsAnyExpression::JsParenthesizedExpression(it) => &it.syntax,
			JsAnyExpression::JsIdentifierExpression(it) => &it.syntax,
			JsAnyExpression::JsSequenceExpression(it) => &it.syntax,
			JsAnyExpression::JsStaticMemberExpression(it) => &it.syntax,
			JsAnyExpression::JsSuperExpression(it) => &it.syntax,
			JsAnyExpression::JsThisExpression(it) => &it.syntax,
			JsAnyExpression::JsUnaryExpression(it) => &it.syntax,
			JsAnyExpression::JsPreUpdateExpression(it) => &it.syntax,
			JsAnyExpression::JsPostUpdateExpression(it) => &it.syntax,
			JsAnyExpression::JsYieldExpression(it) => &it.syntax,
			JsAnyExpression::Template(it) => &it.syntax,
			JsAnyExpression::NewExpr(it) => &it.syntax,
			JsAnyExpression::CallExpr(it) => &it.syntax,
			JsAnyExpression::NewTarget(it) => &it.syntax,
			JsAnyExpression::ImportMeta(it) => &it.syntax,
			JsAnyExpression::TsNonNull(it) => &it.syntax,
			JsAnyExpression::TsAssertion(it) => &it.syntax,
			JsAnyExpression::TsConstAssertion(it) => &it.syntax,
			JsAnyExpression::JsUnknownExpression(it) => &it.syntax,
			JsAnyExpression::JsAnyLiteralExpression(it) => it.syntax(),
		}
	}
}
impl std::fmt::Debug for JsAnyExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			JsAnyExpression::JsAnyLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::JsArrayExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::JsArrowFunctionExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::JsAssignmentExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::JsAwaitExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::JsBinaryExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::JsClassExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::JsConditionalExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::JsComputedMemberExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::JsFunctionExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::JsImportCallExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::JsLogicalExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::JsObjectExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::JsParenthesizedExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::JsIdentifierExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::JsSequenceExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::JsStaticMemberExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::JsSuperExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::JsThisExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::JsUnaryExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::JsPreUpdateExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::JsPostUpdateExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::JsYieldExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::Template(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::NewExpr(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::CallExpr(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::NewTarget(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::ImportMeta(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::TsNonNull(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::TsAssertion(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::TsConstAssertion(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::JsUnknownExpression(it) => std::fmt::Debug::fmt(it, f),
		}
	}
}
impl From<JsVariableDeclaration> for ForHead {
	fn from(node: JsVariableDeclaration) -> ForHead { ForHead::JsVariableDeclaration(node) }
}
impl AstNode for ForHead {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			JS_VARIABLE_DECLARATION => true,
			k if JsAnyExpression::can_cast(k) => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_VARIABLE_DECLARATION => {
				ForHead::JsVariableDeclaration(JsVariableDeclaration { syntax })
			}
			_ => {
				if let Some(js_any_expression) = JsAnyExpression::cast(syntax) {
					return Some(ForHead::JsAnyExpression(js_any_expression));
				}
				return None;
			}
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			ForHead::JsVariableDeclaration(it) => &it.syntax,
			ForHead::JsAnyExpression(it) => it.syntax(),
		}
	}
}
impl std::fmt::Debug for ForHead {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ForHead::JsVariableDeclaration(it) => std::fmt::Debug::fmt(it, f),
			ForHead::JsAnyExpression(it) => std::fmt::Debug::fmt(it, f),
		}
	}
}
impl From<JsVariableDeclaration> for ForLeft {
	fn from(node: JsVariableDeclaration) -> ForLeft { ForLeft::JsVariableDeclaration(node) }
}
impl AstNode for ForLeft {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			JS_VARIABLE_DECLARATION => true,
			k if JsAnyAssignment::can_cast(k) => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_VARIABLE_DECLARATION => {
				ForLeft::JsVariableDeclaration(JsVariableDeclaration { syntax })
			}
			_ => {
				if let Some(js_any_assignment) = JsAnyAssignment::cast(syntax) {
					return Some(ForLeft::JsAnyAssignment(js_any_assignment));
				}
				return None;
			}
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			ForLeft::JsVariableDeclaration(it) => &it.syntax,
			ForLeft::JsAnyAssignment(it) => it.syntax(),
		}
	}
}
impl std::fmt::Debug for ForLeft {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ForLeft::JsAnyAssignment(it) => std::fmt::Debug::fmt(it, f),
			ForLeft::JsVariableDeclaration(it) => std::fmt::Debug::fmt(it, f),
		}
	}
}
impl From<JsIdentifierAssignment> for JsAnyAssignment {
	fn from(node: JsIdentifierAssignment) -> JsAnyAssignment {
		JsAnyAssignment::JsIdentifierAssignment(node)
	}
}
impl From<JsStaticMemberAssignment> for JsAnyAssignment {
	fn from(node: JsStaticMemberAssignment) -> JsAnyAssignment {
		JsAnyAssignment::JsStaticMemberAssignment(node)
	}
}
impl From<JsComputedMemberAssignment> for JsAnyAssignment {
	fn from(node: JsComputedMemberAssignment) -> JsAnyAssignment {
		JsAnyAssignment::JsComputedMemberAssignment(node)
	}
}
impl From<JsParenthesizedAssignment> for JsAnyAssignment {
	fn from(node: JsParenthesizedAssignment) -> JsAnyAssignment {
		JsAnyAssignment::JsParenthesizedAssignment(node)
	}
}
impl From<JsUnknownAssignment> for JsAnyAssignment {
	fn from(node: JsUnknownAssignment) -> JsAnyAssignment {
		JsAnyAssignment::JsUnknownAssignment(node)
	}
}
impl AstNode for JsAnyAssignment {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(
			kind,
			JS_IDENTIFIER_ASSIGNMENT
				| JS_STATIC_MEMBER_ASSIGNMENT
				| JS_COMPUTED_MEMBER_ASSIGNMENT
				| JS_PARENTHESIZED_ASSIGNMENT
				| JS_UNKNOWN_ASSIGNMENT
		)
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_IDENTIFIER_ASSIGNMENT => {
				JsAnyAssignment::JsIdentifierAssignment(JsIdentifierAssignment { syntax })
			}
			JS_STATIC_MEMBER_ASSIGNMENT => {
				JsAnyAssignment::JsStaticMemberAssignment(JsStaticMemberAssignment { syntax })
			}
			JS_COMPUTED_MEMBER_ASSIGNMENT => {
				JsAnyAssignment::JsComputedMemberAssignment(JsComputedMemberAssignment { syntax })
			}
			JS_PARENTHESIZED_ASSIGNMENT => {
				JsAnyAssignment::JsParenthesizedAssignment(JsParenthesizedAssignment { syntax })
			}
			JS_UNKNOWN_ASSIGNMENT => {
				JsAnyAssignment::JsUnknownAssignment(JsUnknownAssignment { syntax })
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsAnyAssignment::JsIdentifierAssignment(it) => &it.syntax,
			JsAnyAssignment::JsStaticMemberAssignment(it) => &it.syntax,
			JsAnyAssignment::JsComputedMemberAssignment(it) => &it.syntax,
			JsAnyAssignment::JsParenthesizedAssignment(it) => &it.syntax,
			JsAnyAssignment::JsUnknownAssignment(it) => &it.syntax,
		}
	}
}
impl std::fmt::Debug for JsAnyAssignment {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			JsAnyAssignment::JsIdentifierAssignment(it) => std::fmt::Debug::fmt(it, f),
			JsAnyAssignment::JsStaticMemberAssignment(it) => std::fmt::Debug::fmt(it, f),
			JsAnyAssignment::JsComputedMemberAssignment(it) => std::fmt::Debug::fmt(it, f),
			JsAnyAssignment::JsParenthesizedAssignment(it) => std::fmt::Debug::fmt(it, f),
			JsAnyAssignment::JsUnknownAssignment(it) => std::fmt::Debug::fmt(it, f),
		}
	}
}
impl From<JsCaseClause> for JsAnySwitchClause {
	fn from(node: JsCaseClause) -> JsAnySwitchClause { JsAnySwitchClause::JsCaseClause(node) }
}
impl From<JsDefaultClause> for JsAnySwitchClause {
	fn from(node: JsDefaultClause) -> JsAnySwitchClause { JsAnySwitchClause::JsDefaultClause(node) }
}
impl AstNode for JsAnySwitchClause {
	fn can_cast(kind: SyntaxKind) -> bool { matches!(kind, JS_CASE_CLAUSE | JS_DEFAULT_CLAUSE) }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_CASE_CLAUSE => JsAnySwitchClause::JsCaseClause(JsCaseClause { syntax }),
			JS_DEFAULT_CLAUSE => JsAnySwitchClause::JsDefaultClause(JsDefaultClause { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsAnySwitchClause::JsCaseClause(it) => &it.syntax,
			JsAnySwitchClause::JsDefaultClause(it) => &it.syntax,
		}
	}
}
impl std::fmt::Debug for JsAnySwitchClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			JsAnySwitchClause::JsCaseClause(it) => std::fmt::Debug::fmt(it, f),
			JsAnySwitchClause::JsDefaultClause(it) => std::fmt::Debug::fmt(it, f),
		}
	}
}
impl From<JsArrayBindingPattern> for JsAnyBindingPattern {
	fn from(node: JsArrayBindingPattern) -> JsAnyBindingPattern {
		JsAnyBindingPattern::JsArrayBindingPattern(node)
	}
}
impl From<JsObjectBindingPattern> for JsAnyBindingPattern {
	fn from(node: JsObjectBindingPattern) -> JsAnyBindingPattern {
		JsAnyBindingPattern::JsObjectBindingPattern(node)
	}
}
impl AstNode for JsAnyBindingPattern {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			JS_ARRAY_BINDING_PATTERN | JS_OBJECT_BINDING_PATTERN => true,
			k if JsAnyBinding::can_cast(k) => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_ARRAY_BINDING_PATTERN => {
				JsAnyBindingPattern::JsArrayBindingPattern(JsArrayBindingPattern { syntax })
			}
			JS_OBJECT_BINDING_PATTERN => {
				JsAnyBindingPattern::JsObjectBindingPattern(JsObjectBindingPattern { syntax })
			}
			_ => {
				if let Some(js_any_binding) = JsAnyBinding::cast(syntax) {
					return Some(JsAnyBindingPattern::JsAnyBinding(js_any_binding));
				}
				return None;
			}
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsAnyBindingPattern::JsArrayBindingPattern(it) => &it.syntax,
			JsAnyBindingPattern::JsObjectBindingPattern(it) => &it.syntax,
			JsAnyBindingPattern::JsAnyBinding(it) => it.syntax(),
		}
	}
}
impl std::fmt::Debug for JsAnyBindingPattern {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			JsAnyBindingPattern::JsAnyBinding(it) => std::fmt::Debug::fmt(it, f),
			JsAnyBindingPattern::JsArrayBindingPattern(it) => std::fmt::Debug::fmt(it, f),
			JsAnyBindingPattern::JsObjectBindingPattern(it) => std::fmt::Debug::fmt(it, f),
		}
	}
}
impl From<JsStringLiteralExpression> for JsAnyLiteralExpression {
	fn from(node: JsStringLiteralExpression) -> JsAnyLiteralExpression {
		JsAnyLiteralExpression::JsStringLiteralExpression(node)
	}
}
impl From<JsNumberLiteralExpression> for JsAnyLiteralExpression {
	fn from(node: JsNumberLiteralExpression) -> JsAnyLiteralExpression {
		JsAnyLiteralExpression::JsNumberLiteralExpression(node)
	}
}
impl From<JsBigIntLiteralExpression> for JsAnyLiteralExpression {
	fn from(node: JsBigIntLiteralExpression) -> JsAnyLiteralExpression {
		JsAnyLiteralExpression::JsBigIntLiteralExpression(node)
	}
}
impl From<JsBooleanLiteralExpression> for JsAnyLiteralExpression {
	fn from(node: JsBooleanLiteralExpression) -> JsAnyLiteralExpression {
		JsAnyLiteralExpression::JsBooleanLiteralExpression(node)
	}
}
impl From<JsNullLiteralExpression> for JsAnyLiteralExpression {
	fn from(node: JsNullLiteralExpression) -> JsAnyLiteralExpression {
		JsAnyLiteralExpression::JsNullLiteralExpression(node)
	}
}
impl From<JsRegexLiteralExpression> for JsAnyLiteralExpression {
	fn from(node: JsRegexLiteralExpression) -> JsAnyLiteralExpression {
		JsAnyLiteralExpression::JsRegexLiteralExpression(node)
	}
}
impl AstNode for JsAnyLiteralExpression {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(
			kind,
			JS_STRING_LITERAL_EXPRESSION
				| JS_NUMBER_LITERAL_EXPRESSION
				| JS_BIG_INT_LITERAL_EXPRESSION
				| JS_BOOLEAN_LITERAL_EXPRESSION
				| JS_NULL_LITERAL_EXPRESSION
				| JS_REGEX_LITERAL_EXPRESSION
		)
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_STRING_LITERAL_EXPRESSION => {
				JsAnyLiteralExpression::JsStringLiteralExpression(JsStringLiteralExpression {
					syntax,
				})
			}
			JS_NUMBER_LITERAL_EXPRESSION => {
				JsAnyLiteralExpression::JsNumberLiteralExpression(JsNumberLiteralExpression {
					syntax,
				})
			}
			JS_BIG_INT_LITERAL_EXPRESSION => {
				JsAnyLiteralExpression::JsBigIntLiteralExpression(JsBigIntLiteralExpression {
					syntax,
				})
			}
			JS_BOOLEAN_LITERAL_EXPRESSION => {
				JsAnyLiteralExpression::JsBooleanLiteralExpression(JsBooleanLiteralExpression {
					syntax,
				})
			}
			JS_NULL_LITERAL_EXPRESSION => {
				JsAnyLiteralExpression::JsNullLiteralExpression(JsNullLiteralExpression { syntax })
			}
			JS_REGEX_LITERAL_EXPRESSION => {
				JsAnyLiteralExpression::JsRegexLiteralExpression(JsRegexLiteralExpression {
					syntax,
				})
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsAnyLiteralExpression::JsStringLiteralExpression(it) => &it.syntax,
			JsAnyLiteralExpression::JsNumberLiteralExpression(it) => &it.syntax,
			JsAnyLiteralExpression::JsBigIntLiteralExpression(it) => &it.syntax,
			JsAnyLiteralExpression::JsBooleanLiteralExpression(it) => &it.syntax,
			JsAnyLiteralExpression::JsNullLiteralExpression(it) => &it.syntax,
			JsAnyLiteralExpression::JsRegexLiteralExpression(it) => &it.syntax,
		}
	}
}
impl std::fmt::Debug for JsAnyLiteralExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			JsAnyLiteralExpression::JsStringLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyLiteralExpression::JsNumberLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyLiteralExpression::JsBigIntLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyLiteralExpression::JsBooleanLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyLiteralExpression::JsNullLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyLiteralExpression::JsRegexLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
		}
	}
}
impl From<JsIdentifierBinding> for JsAnyBinding {
	fn from(node: JsIdentifierBinding) -> JsAnyBinding { JsAnyBinding::JsIdentifierBinding(node) }
}
impl From<JsUnknownBinding> for JsAnyBinding {
	fn from(node: JsUnknownBinding) -> JsAnyBinding { JsAnyBinding::JsUnknownBinding(node) }
}
impl AstNode for JsAnyBinding {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(kind, JS_IDENTIFIER_BINDING | JS_UNKNOWN_BINDING)
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_IDENTIFIER_BINDING => {
				JsAnyBinding::JsIdentifierBinding(JsIdentifierBinding { syntax })
			}
			JS_UNKNOWN_BINDING => JsAnyBinding::JsUnknownBinding(JsUnknownBinding { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsAnyBinding::JsIdentifierBinding(it) => &it.syntax,
			JsAnyBinding::JsUnknownBinding(it) => &it.syntax,
		}
	}
}
impl std::fmt::Debug for JsAnyBinding {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			JsAnyBinding::JsIdentifierBinding(it) => std::fmt::Debug::fmt(it, f),
			JsAnyBinding::JsUnknownBinding(it) => std::fmt::Debug::fmt(it, f),
		}
	}
}
impl From<JsParameterList> for JsAnyArrowFunctionParameters {
	fn from(node: JsParameterList) -> JsAnyArrowFunctionParameters {
		JsAnyArrowFunctionParameters::JsParameterList(node)
	}
}
impl AstNode for JsAnyArrowFunctionParameters {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			JS_PARAMETER_LIST => true,
			k if JsAnyBinding::can_cast(k) => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_PARAMETER_LIST => {
				JsAnyArrowFunctionParameters::JsParameterList(JsParameterList { syntax })
			}
			_ => {
				if let Some(js_any_binding) = JsAnyBinding::cast(syntax) {
					return Some(JsAnyArrowFunctionParameters::JsAnyBinding(js_any_binding));
				}
				return None;
			}
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsAnyArrowFunctionParameters::JsParameterList(it) => &it.syntax,
			JsAnyArrowFunctionParameters::JsAnyBinding(it) => it.syntax(),
		}
	}
}
impl std::fmt::Debug for JsAnyArrowFunctionParameters {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			JsAnyArrowFunctionParameters::JsParameterList(it) => std::fmt::Debug::fmt(it, f),
			JsAnyArrowFunctionParameters::JsAnyBinding(it) => std::fmt::Debug::fmt(it, f),
		}
	}
}
impl From<JsFunctionBody> for JsAnyArrowFunctionBody {
	fn from(node: JsFunctionBody) -> JsAnyArrowFunctionBody {
		JsAnyArrowFunctionBody::JsFunctionBody(node)
	}
}
impl AstNode for JsAnyArrowFunctionBody {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			JS_FUNCTION_BODY => true,
			k if JsAnyExpression::can_cast(k) => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_FUNCTION_BODY => JsAnyArrowFunctionBody::JsFunctionBody(JsFunctionBody { syntax }),
			_ => {
				if let Some(js_any_expression) = JsAnyExpression::cast(syntax) {
					return Some(JsAnyArrowFunctionBody::JsAnyExpression(js_any_expression));
				}
				return None;
			}
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsAnyArrowFunctionBody::JsFunctionBody(it) => &it.syntax,
			JsAnyArrowFunctionBody::JsAnyExpression(it) => it.syntax(),
		}
	}
}
impl std::fmt::Debug for JsAnyArrowFunctionBody {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			JsAnyArrowFunctionBody::JsAnyExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyArrowFunctionBody::JsFunctionBody(it) => std::fmt::Debug::fmt(it, f),
		}
	}
}
impl From<JsSpread> for JsAnyArrayElement {
	fn from(node: JsSpread) -> JsAnyArrayElement { JsAnyArrayElement::JsSpread(node) }
}
impl From<JsArrayHole> for JsAnyArrayElement {
	fn from(node: JsArrayHole) -> JsAnyArrayElement { JsAnyArrayElement::JsArrayHole(node) }
}
impl AstNode for JsAnyArrayElement {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			JS_SPREAD | JS_ARRAY_HOLE => true,
			k if JsAnyExpression::can_cast(k) => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_SPREAD => JsAnyArrayElement::JsSpread(JsSpread { syntax }),
			JS_ARRAY_HOLE => JsAnyArrayElement::JsArrayHole(JsArrayHole { syntax }),
			_ => {
				if let Some(js_any_expression) = JsAnyExpression::cast(syntax) {
					return Some(JsAnyArrayElement::JsAnyExpression(js_any_expression));
				}
				return None;
			}
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsAnyArrayElement::JsSpread(it) => &it.syntax,
			JsAnyArrayElement::JsArrayHole(it) => &it.syntax,
			JsAnyArrayElement::JsAnyExpression(it) => it.syntax(),
		}
	}
}
impl std::fmt::Debug for JsAnyArrayElement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			JsAnyArrayElement::JsAnyExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyArrayElement::JsSpread(it) => std::fmt::Debug::fmt(it, f),
			JsAnyArrayElement::JsArrayHole(it) => std::fmt::Debug::fmt(it, f),
		}
	}
}
impl From<JsArrayAssignmentPattern> for JsAnyAssignmentPattern {
	fn from(node: JsArrayAssignmentPattern) -> JsAnyAssignmentPattern {
		JsAnyAssignmentPattern::JsArrayAssignmentPattern(node)
	}
}
impl From<JsObjectAssignmentPattern> for JsAnyAssignmentPattern {
	fn from(node: JsObjectAssignmentPattern) -> JsAnyAssignmentPattern {
		JsAnyAssignmentPattern::JsObjectAssignmentPattern(node)
	}
}
impl AstNode for JsAnyAssignmentPattern {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			JS_ARRAY_ASSIGNMENT_PATTERN | JS_OBJECT_ASSIGNMENT_PATTERN => true,
			k if JsAnyAssignment::can_cast(k) => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_ARRAY_ASSIGNMENT_PATTERN => {
				JsAnyAssignmentPattern::JsArrayAssignmentPattern(JsArrayAssignmentPattern {
					syntax,
				})
			}
			JS_OBJECT_ASSIGNMENT_PATTERN => {
				JsAnyAssignmentPattern::JsObjectAssignmentPattern(JsObjectAssignmentPattern {
					syntax,
				})
			}
			_ => {
				if let Some(js_any_assignment) = JsAnyAssignment::cast(syntax) {
					return Some(JsAnyAssignmentPattern::JsAnyAssignment(js_any_assignment));
				}
				return None;
			}
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsAnyAssignmentPattern::JsArrayAssignmentPattern(it) => &it.syntax,
			JsAnyAssignmentPattern::JsObjectAssignmentPattern(it) => &it.syntax,
			JsAnyAssignmentPattern::JsAnyAssignment(it) => it.syntax(),
		}
	}
}
impl std::fmt::Debug for JsAnyAssignmentPattern {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			JsAnyAssignmentPattern::JsAnyAssignment(it) => std::fmt::Debug::fmt(it, f),
			JsAnyAssignmentPattern::JsArrayAssignmentPattern(it) => std::fmt::Debug::fmt(it, f),
			JsAnyAssignmentPattern::JsObjectAssignmentPattern(it) => std::fmt::Debug::fmt(it, f),
		}
	}
}
impl From<JsName> for JsAnyName {
	fn from(node: JsName) -> JsAnyName { JsAnyName::JsName(node) }
}
impl From<JsPrivateName> for JsAnyName {
	fn from(node: JsPrivateName) -> JsAnyName { JsAnyName::JsPrivateName(node) }
}
impl AstNode for JsAnyName {
	fn can_cast(kind: SyntaxKind) -> bool { matches!(kind, JS_NAME | JS_PRIVATE_NAME) }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_NAME => JsAnyName::JsName(JsName { syntax }),
			JS_PRIVATE_NAME => JsAnyName::JsPrivateName(JsPrivateName { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsAnyName::JsName(it) => &it.syntax,
			JsAnyName::JsPrivateName(it) => &it.syntax,
		}
	}
}
impl std::fmt::Debug for JsAnyName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			JsAnyName::JsName(it) => std::fmt::Debug::fmt(it, f),
			JsAnyName::JsPrivateName(it) => std::fmt::Debug::fmt(it, f),
		}
	}
}
impl From<JsLiteralMemberName> for JsAnyObjectMemberName {
	fn from(node: JsLiteralMemberName) -> JsAnyObjectMemberName {
		JsAnyObjectMemberName::JsLiteralMemberName(node)
	}
}
impl From<JsComputedMemberName> for JsAnyObjectMemberName {
	fn from(node: JsComputedMemberName) -> JsAnyObjectMemberName {
		JsAnyObjectMemberName::JsComputedMemberName(node)
	}
}
impl AstNode for JsAnyObjectMemberName {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(kind, JS_LITERAL_MEMBER_NAME | JS_COMPUTED_MEMBER_NAME)
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_LITERAL_MEMBER_NAME => {
				JsAnyObjectMemberName::JsLiteralMemberName(JsLiteralMemberName { syntax })
			}
			JS_COMPUTED_MEMBER_NAME => {
				JsAnyObjectMemberName::JsComputedMemberName(JsComputedMemberName { syntax })
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsAnyObjectMemberName::JsLiteralMemberName(it) => &it.syntax,
			JsAnyObjectMemberName::JsComputedMemberName(it) => &it.syntax,
		}
	}
}
impl std::fmt::Debug for JsAnyObjectMemberName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			JsAnyObjectMemberName::JsLiteralMemberName(it) => std::fmt::Debug::fmt(it, f),
			JsAnyObjectMemberName::JsComputedMemberName(it) => std::fmt::Debug::fmt(it, f),
		}
	}
}
impl From<JsPropertyObjectMember> for JsAnyObjectMember {
	fn from(node: JsPropertyObjectMember) -> JsAnyObjectMember {
		JsAnyObjectMember::JsPropertyObjectMember(node)
	}
}
impl From<JsMethodObjectMember> for JsAnyObjectMember {
	fn from(node: JsMethodObjectMember) -> JsAnyObjectMember {
		JsAnyObjectMember::JsMethodObjectMember(node)
	}
}
impl From<JsGetterObjectMember> for JsAnyObjectMember {
	fn from(node: JsGetterObjectMember) -> JsAnyObjectMember {
		JsAnyObjectMember::JsGetterObjectMember(node)
	}
}
impl From<JsSetterObjectMember> for JsAnyObjectMember {
	fn from(node: JsSetterObjectMember) -> JsAnyObjectMember {
		JsAnyObjectMember::JsSetterObjectMember(node)
	}
}
impl From<JsShorthandPropertyObjectMember> for JsAnyObjectMember {
	fn from(node: JsShorthandPropertyObjectMember) -> JsAnyObjectMember {
		JsAnyObjectMember::JsShorthandPropertyObjectMember(node)
	}
}
impl From<JsSpread> for JsAnyObjectMember {
	fn from(node: JsSpread) -> JsAnyObjectMember { JsAnyObjectMember::JsSpread(node) }
}
impl From<JsUnknownMember> for JsAnyObjectMember {
	fn from(node: JsUnknownMember) -> JsAnyObjectMember { JsAnyObjectMember::JsUnknownMember(node) }
}
impl AstNode for JsAnyObjectMember {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(
			kind,
			JS_PROPERTY_OBJECT_MEMBER
				| JS_METHOD_OBJECT_MEMBER
				| JS_GETTER_OBJECT_MEMBER
				| JS_SETTER_OBJECT_MEMBER
				| JS_SHORTHAND_PROPERTY_OBJECT_MEMBER
				| JS_SPREAD | JS_UNKNOWN_MEMBER
		)
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_PROPERTY_OBJECT_MEMBER => {
				JsAnyObjectMember::JsPropertyObjectMember(JsPropertyObjectMember { syntax })
			}
			JS_METHOD_OBJECT_MEMBER => {
				JsAnyObjectMember::JsMethodObjectMember(JsMethodObjectMember { syntax })
			}
			JS_GETTER_OBJECT_MEMBER => {
				JsAnyObjectMember::JsGetterObjectMember(JsGetterObjectMember { syntax })
			}
			JS_SETTER_OBJECT_MEMBER => {
				JsAnyObjectMember::JsSetterObjectMember(JsSetterObjectMember { syntax })
			}
			JS_SHORTHAND_PROPERTY_OBJECT_MEMBER => {
				JsAnyObjectMember::JsShorthandPropertyObjectMember(
					JsShorthandPropertyObjectMember { syntax },
				)
			}
			JS_SPREAD => JsAnyObjectMember::JsSpread(JsSpread { syntax }),
			JS_UNKNOWN_MEMBER => JsAnyObjectMember::JsUnknownMember(JsUnknownMember { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsAnyObjectMember::JsPropertyObjectMember(it) => &it.syntax,
			JsAnyObjectMember::JsMethodObjectMember(it) => &it.syntax,
			JsAnyObjectMember::JsGetterObjectMember(it) => &it.syntax,
			JsAnyObjectMember::JsSetterObjectMember(it) => &it.syntax,
			JsAnyObjectMember::JsShorthandPropertyObjectMember(it) => &it.syntax,
			JsAnyObjectMember::JsSpread(it) => &it.syntax,
			JsAnyObjectMember::JsUnknownMember(it) => &it.syntax,
		}
	}
}
impl std::fmt::Debug for JsAnyObjectMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			JsAnyObjectMember::JsPropertyObjectMember(it) => std::fmt::Debug::fmt(it, f),
			JsAnyObjectMember::JsMethodObjectMember(it) => std::fmt::Debug::fmt(it, f),
			JsAnyObjectMember::JsGetterObjectMember(it) => std::fmt::Debug::fmt(it, f),
			JsAnyObjectMember::JsSetterObjectMember(it) => std::fmt::Debug::fmt(it, f),
			JsAnyObjectMember::JsShorthandPropertyObjectMember(it) => std::fmt::Debug::fmt(it, f),
			JsAnyObjectMember::JsSpread(it) => std::fmt::Debug::fmt(it, f),
			JsAnyObjectMember::JsUnknownMember(it) => std::fmt::Debug::fmt(it, f),
		}
	}
}
impl From<JsConstructorClassMember> for JsAnyClassMember {
	fn from(node: JsConstructorClassMember) -> JsAnyClassMember {
		JsAnyClassMember::JsConstructorClassMember(node)
	}
}
impl From<JsPropertyClassMember> for JsAnyClassMember {
	fn from(node: JsPropertyClassMember) -> JsAnyClassMember {
		JsAnyClassMember::JsPropertyClassMember(node)
	}
}
impl From<JsMethodClassMember> for JsAnyClassMember {
	fn from(node: JsMethodClassMember) -> JsAnyClassMember {
		JsAnyClassMember::JsMethodClassMember(node)
	}
}
impl From<JsGetterClassMember> for JsAnyClassMember {
	fn from(node: JsGetterClassMember) -> JsAnyClassMember {
		JsAnyClassMember::JsGetterClassMember(node)
	}
}
impl From<JsSetterClassMember> for JsAnyClassMember {
	fn from(node: JsSetterClassMember) -> JsAnyClassMember {
		JsAnyClassMember::JsSetterClassMember(node)
	}
}
impl From<JsEmptyClassMember> for JsAnyClassMember {
	fn from(node: JsEmptyClassMember) -> JsAnyClassMember {
		JsAnyClassMember::JsEmptyClassMember(node)
	}
}
impl From<TsIndexSignature> for JsAnyClassMember {
	fn from(node: TsIndexSignature) -> JsAnyClassMember { JsAnyClassMember::TsIndexSignature(node) }
}
impl From<JsUnknownMember> for JsAnyClassMember {
	fn from(node: JsUnknownMember) -> JsAnyClassMember { JsAnyClassMember::JsUnknownMember(node) }
}
impl AstNode for JsAnyClassMember {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(
			kind,
			JS_CONSTRUCTOR_CLASS_MEMBER
				| JS_PROPERTY_CLASS_MEMBER
				| JS_METHOD_CLASS_MEMBER
				| JS_GETTER_CLASS_MEMBER
				| JS_SETTER_CLASS_MEMBER
				| JS_EMPTY_CLASS_MEMBER
				| TS_INDEX_SIGNATURE
				| JS_UNKNOWN_MEMBER
		)
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_CONSTRUCTOR_CLASS_MEMBER => {
				JsAnyClassMember::JsConstructorClassMember(JsConstructorClassMember { syntax })
			}
			JS_PROPERTY_CLASS_MEMBER => {
				JsAnyClassMember::JsPropertyClassMember(JsPropertyClassMember { syntax })
			}
			JS_METHOD_CLASS_MEMBER => {
				JsAnyClassMember::JsMethodClassMember(JsMethodClassMember { syntax })
			}
			JS_GETTER_CLASS_MEMBER => {
				JsAnyClassMember::JsGetterClassMember(JsGetterClassMember { syntax })
			}
			JS_SETTER_CLASS_MEMBER => {
				JsAnyClassMember::JsSetterClassMember(JsSetterClassMember { syntax })
			}
			JS_EMPTY_CLASS_MEMBER => {
				JsAnyClassMember::JsEmptyClassMember(JsEmptyClassMember { syntax })
			}
			TS_INDEX_SIGNATURE => JsAnyClassMember::TsIndexSignature(TsIndexSignature { syntax }),
			JS_UNKNOWN_MEMBER => JsAnyClassMember::JsUnknownMember(JsUnknownMember { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsAnyClassMember::JsConstructorClassMember(it) => &it.syntax,
			JsAnyClassMember::JsPropertyClassMember(it) => &it.syntax,
			JsAnyClassMember::JsMethodClassMember(it) => &it.syntax,
			JsAnyClassMember::JsGetterClassMember(it) => &it.syntax,
			JsAnyClassMember::JsSetterClassMember(it) => &it.syntax,
			JsAnyClassMember::JsEmptyClassMember(it) => &it.syntax,
			JsAnyClassMember::TsIndexSignature(it) => &it.syntax,
			JsAnyClassMember::JsUnknownMember(it) => &it.syntax,
		}
	}
}
impl std::fmt::Debug for JsAnyClassMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			JsAnyClassMember::JsConstructorClassMember(it) => std::fmt::Debug::fmt(it, f),
			JsAnyClassMember::JsPropertyClassMember(it) => std::fmt::Debug::fmt(it, f),
			JsAnyClassMember::JsMethodClassMember(it) => std::fmt::Debug::fmt(it, f),
			JsAnyClassMember::JsGetterClassMember(it) => std::fmt::Debug::fmt(it, f),
			JsAnyClassMember::JsSetterClassMember(it) => std::fmt::Debug::fmt(it, f),
			JsAnyClassMember::JsEmptyClassMember(it) => std::fmt::Debug::fmt(it, f),
			JsAnyClassMember::TsIndexSignature(it) => std::fmt::Debug::fmt(it, f),
			JsAnyClassMember::JsUnknownMember(it) => std::fmt::Debug::fmt(it, f),
		}
	}
}
impl From<JsLiteralMemberName> for JsAnyClassMemberName {
	fn from(node: JsLiteralMemberName) -> JsAnyClassMemberName {
		JsAnyClassMemberName::JsLiteralMemberName(node)
	}
}
impl From<JsComputedMemberName> for JsAnyClassMemberName {
	fn from(node: JsComputedMemberName) -> JsAnyClassMemberName {
		JsAnyClassMemberName::JsComputedMemberName(node)
	}
}
impl From<JsPrivateClassMemberName> for JsAnyClassMemberName {
	fn from(node: JsPrivateClassMemberName) -> JsAnyClassMemberName {
		JsAnyClassMemberName::JsPrivateClassMemberName(node)
	}
}
impl AstNode for JsAnyClassMemberName {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(
			kind,
			JS_LITERAL_MEMBER_NAME | JS_COMPUTED_MEMBER_NAME | JS_PRIVATE_CLASS_MEMBER_NAME
		)
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_LITERAL_MEMBER_NAME => {
				JsAnyClassMemberName::JsLiteralMemberName(JsLiteralMemberName { syntax })
			}
			JS_COMPUTED_MEMBER_NAME => {
				JsAnyClassMemberName::JsComputedMemberName(JsComputedMemberName { syntax })
			}
			JS_PRIVATE_CLASS_MEMBER_NAME => {
				JsAnyClassMemberName::JsPrivateClassMemberName(JsPrivateClassMemberName { syntax })
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsAnyClassMemberName::JsLiteralMemberName(it) => &it.syntax,
			JsAnyClassMemberName::JsComputedMemberName(it) => &it.syntax,
			JsAnyClassMemberName::JsPrivateClassMemberName(it) => &it.syntax,
		}
	}
}
impl std::fmt::Debug for JsAnyClassMemberName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			JsAnyClassMemberName::JsLiteralMemberName(it) => std::fmt::Debug::fmt(it, f),
			JsAnyClassMemberName::JsComputedMemberName(it) => std::fmt::Debug::fmt(it, f),
			JsAnyClassMemberName::JsPrivateClassMemberName(it) => std::fmt::Debug::fmt(it, f),
		}
	}
}
impl From<TsConstructorParam> for JsAnyConstructorParameter {
	fn from(node: TsConstructorParam) -> JsAnyConstructorParameter {
		JsAnyConstructorParameter::TsConstructorParam(node)
	}
}
impl From<JsBindingPatternWithDefault> for JsAnyConstructorParameter {
	fn from(node: JsBindingPatternWithDefault) -> JsAnyConstructorParameter {
		JsAnyConstructorParameter::JsBindingPatternWithDefault(node)
	}
}
impl AstNode for JsAnyConstructorParameter {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			TS_CONSTRUCTOR_PARAM | JS_BINDING_PATTERN_WITH_DEFAULT => true,
			k if JsAnyBindingPattern::can_cast(k) => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			TS_CONSTRUCTOR_PARAM => {
				JsAnyConstructorParameter::TsConstructorParam(TsConstructorParam { syntax })
			}
			JS_BINDING_PATTERN_WITH_DEFAULT => {
				JsAnyConstructorParameter::JsBindingPatternWithDefault(
					JsBindingPatternWithDefault { syntax },
				)
			}
			_ => {
				if let Some(js_any_binding_pattern) = JsAnyBindingPattern::cast(syntax) {
					return Some(JsAnyConstructorParameter::JsAnyBindingPattern(
						js_any_binding_pattern,
					));
				}
				return None;
			}
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsAnyConstructorParameter::TsConstructorParam(it) => &it.syntax,
			JsAnyConstructorParameter::JsBindingPatternWithDefault(it) => &it.syntax,
			JsAnyConstructorParameter::JsAnyBindingPattern(it) => it.syntax(),
		}
	}
}
impl std::fmt::Debug for JsAnyConstructorParameter {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			JsAnyConstructorParameter::TsConstructorParam(it) => std::fmt::Debug::fmt(it, f),
			JsAnyConstructorParameter::JsAnyBindingPattern(it) => std::fmt::Debug::fmt(it, f),
			JsAnyConstructorParameter::JsBindingPatternWithDefault(it) => {
				std::fmt::Debug::fmt(it, f)
			}
		}
	}
}
impl From<JsModifier> for JsAnyModifier {
	fn from(node: JsModifier) -> JsAnyModifier { JsAnyModifier::JsModifier(node) }
}
impl From<JsUnknownModifier> for JsAnyModifier {
	fn from(node: JsUnknownModifier) -> JsAnyModifier { JsAnyModifier::JsUnknownModifier(node) }
}
impl AstNode for JsAnyModifier {
	fn can_cast(kind: SyntaxKind) -> bool { matches!(kind, JS_MODIFIER | JS_UNKNOWN_MODIFIER) }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_MODIFIER => JsAnyModifier::JsModifier(JsModifier { syntax }),
			JS_UNKNOWN_MODIFIER => JsAnyModifier::JsUnknownModifier(JsUnknownModifier { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsAnyModifier::JsModifier(it) => &it.syntax,
			JsAnyModifier::JsUnknownModifier(it) => &it.syntax,
		}
	}
}
impl std::fmt::Debug for JsAnyModifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			JsAnyModifier::JsModifier(it) => std::fmt::Debug::fmt(it, f),
			JsAnyModifier::JsUnknownModifier(it) => std::fmt::Debug::fmt(it, f),
		}
	}
}
impl From<JsAssignmentWithDefault> for JsAnyArrayAssignmentPatternElement {
	fn from(node: JsAssignmentWithDefault) -> JsAnyArrayAssignmentPatternElement {
		JsAnyArrayAssignmentPatternElement::JsAssignmentWithDefault(node)
	}
}
impl From<JsArrayAssignmentPatternRestElement> for JsAnyArrayAssignmentPatternElement {
	fn from(node: JsArrayAssignmentPatternRestElement) -> JsAnyArrayAssignmentPatternElement {
		JsAnyArrayAssignmentPatternElement::JsArrayAssignmentPatternRestElement(node)
	}
}
impl From<JsArrayHole> for JsAnyArrayAssignmentPatternElement {
	fn from(node: JsArrayHole) -> JsAnyArrayAssignmentPatternElement {
		JsAnyArrayAssignmentPatternElement::JsArrayHole(node)
	}
}
impl From<JsUnknownAssignment> for JsAnyArrayAssignmentPatternElement {
	fn from(node: JsUnknownAssignment) -> JsAnyArrayAssignmentPatternElement {
		JsAnyArrayAssignmentPatternElement::JsUnknownAssignment(node)
	}
}
impl AstNode for JsAnyArrayAssignmentPatternElement {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			JS_ASSIGNMENT_WITH_DEFAULT
			| JS_ARRAY_ASSIGNMENT_PATTERN_REST_ELEMENT
			| JS_ARRAY_HOLE
			| JS_UNKNOWN_ASSIGNMENT => true,
			k if JsAnyAssignmentPattern::can_cast(k) => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_ASSIGNMENT_WITH_DEFAULT => {
				JsAnyArrayAssignmentPatternElement::JsAssignmentWithDefault(
					JsAssignmentWithDefault { syntax },
				)
			}
			JS_ARRAY_ASSIGNMENT_PATTERN_REST_ELEMENT => {
				JsAnyArrayAssignmentPatternElement::JsArrayAssignmentPatternRestElement(
					JsArrayAssignmentPatternRestElement { syntax },
				)
			}
			JS_ARRAY_HOLE => {
				JsAnyArrayAssignmentPatternElement::JsArrayHole(JsArrayHole { syntax })
			}
			JS_UNKNOWN_ASSIGNMENT => {
				JsAnyArrayAssignmentPatternElement::JsUnknownAssignment(JsUnknownAssignment {
					syntax,
				})
			}
			_ => {
				if let Some(js_any_assignment_pattern) = JsAnyAssignmentPattern::cast(syntax) {
					return Some(JsAnyArrayAssignmentPatternElement::JsAnyAssignmentPattern(
						js_any_assignment_pattern,
					));
				}
				return None;
			}
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsAnyArrayAssignmentPatternElement::JsAssignmentWithDefault(it) => &it.syntax,
			JsAnyArrayAssignmentPatternElement::JsArrayAssignmentPatternRestElement(it) => {
				&it.syntax
			}
			JsAnyArrayAssignmentPatternElement::JsArrayHole(it) => &it.syntax,
			JsAnyArrayAssignmentPatternElement::JsUnknownAssignment(it) => &it.syntax,
			JsAnyArrayAssignmentPatternElement::JsAnyAssignmentPattern(it) => it.syntax(),
		}
	}
}
impl std::fmt::Debug for JsAnyArrayAssignmentPatternElement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			JsAnyArrayAssignmentPatternElement::JsAssignmentWithDefault(it) => {
				std::fmt::Debug::fmt(it, f)
			}
			JsAnyArrayAssignmentPatternElement::JsAnyAssignmentPattern(it) => {
				std::fmt::Debug::fmt(it, f)
			}
			JsAnyArrayAssignmentPatternElement::JsArrayAssignmentPatternRestElement(it) => {
				std::fmt::Debug::fmt(it, f)
			}
			JsAnyArrayAssignmentPatternElement::JsArrayHole(it) => std::fmt::Debug::fmt(it, f),
			JsAnyArrayAssignmentPatternElement::JsUnknownAssignment(it) => {
				std::fmt::Debug::fmt(it, f)
			}
		}
	}
}
impl From<JsObjectAssignmentPatternShorthandProperty> for JsAnyObjectAssignmentPatternMember {
	fn from(
		node: JsObjectAssignmentPatternShorthandProperty,
	) -> JsAnyObjectAssignmentPatternMember {
		JsAnyObjectAssignmentPatternMember::JsObjectAssignmentPatternShorthandProperty(node)
	}
}
impl From<JsObjectAssignmentPatternProperty> for JsAnyObjectAssignmentPatternMember {
	fn from(node: JsObjectAssignmentPatternProperty) -> JsAnyObjectAssignmentPatternMember {
		JsAnyObjectAssignmentPatternMember::JsObjectAssignmentPatternProperty(node)
	}
}
impl From<JsObjectAssignmentPatternRest> for JsAnyObjectAssignmentPatternMember {
	fn from(node: JsObjectAssignmentPatternRest) -> JsAnyObjectAssignmentPatternMember {
		JsAnyObjectAssignmentPatternMember::JsObjectAssignmentPatternRest(node)
	}
}
impl From<JsUnknownAssignment> for JsAnyObjectAssignmentPatternMember {
	fn from(node: JsUnknownAssignment) -> JsAnyObjectAssignmentPatternMember {
		JsAnyObjectAssignmentPatternMember::JsUnknownAssignment(node)
	}
}
impl AstNode for JsAnyObjectAssignmentPatternMember {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(
			kind,
			JS_OBJECT_ASSIGNMENT_PATTERN_SHORTHAND_PROPERTY
				| JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY
				| JS_OBJECT_ASSIGNMENT_PATTERN_REST
				| JS_UNKNOWN_ASSIGNMENT
		)
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_OBJECT_ASSIGNMENT_PATTERN_SHORTHAND_PROPERTY => {
				JsAnyObjectAssignmentPatternMember::JsObjectAssignmentPatternShorthandProperty(
					JsObjectAssignmentPatternShorthandProperty { syntax },
				)
			}
			JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY => {
				JsAnyObjectAssignmentPatternMember::JsObjectAssignmentPatternProperty(
					JsObjectAssignmentPatternProperty { syntax },
				)
			}
			JS_OBJECT_ASSIGNMENT_PATTERN_REST => {
				JsAnyObjectAssignmentPatternMember::JsObjectAssignmentPatternRest(
					JsObjectAssignmentPatternRest { syntax },
				)
			}
			JS_UNKNOWN_ASSIGNMENT => {
				JsAnyObjectAssignmentPatternMember::JsUnknownAssignment(JsUnknownAssignment {
					syntax,
				})
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsAnyObjectAssignmentPatternMember::JsObjectAssignmentPatternShorthandProperty(it) => {
				&it.syntax
			}
			JsAnyObjectAssignmentPatternMember::JsObjectAssignmentPatternProperty(it) => &it.syntax,
			JsAnyObjectAssignmentPatternMember::JsObjectAssignmentPatternRest(it) => &it.syntax,
			JsAnyObjectAssignmentPatternMember::JsUnknownAssignment(it) => &it.syntax,
		}
	}
}
impl std::fmt::Debug for JsAnyObjectAssignmentPatternMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			JsAnyObjectAssignmentPatternMember::JsObjectAssignmentPatternShorthandProperty(it) => {
				std::fmt::Debug::fmt(it, f)
			}
			JsAnyObjectAssignmentPatternMember::JsObjectAssignmentPatternProperty(it) => {
				std::fmt::Debug::fmt(it, f)
			}
			JsAnyObjectAssignmentPatternMember::JsObjectAssignmentPatternRest(it) => {
				std::fmt::Debug::fmt(it, f)
			}
			JsAnyObjectAssignmentPatternMember::JsUnknownAssignment(it) => {
				std::fmt::Debug::fmt(it, f)
			}
		}
	}
}
impl From<JsArrayHole> for JsAnyArrayBindingPatternElement {
	fn from(node: JsArrayHole) -> JsAnyArrayBindingPatternElement {
		JsAnyArrayBindingPatternElement::JsArrayHole(node)
	}
}
impl From<JsBindingPatternWithDefault> for JsAnyArrayBindingPatternElement {
	fn from(node: JsBindingPatternWithDefault) -> JsAnyArrayBindingPatternElement {
		JsAnyArrayBindingPatternElement::JsBindingPatternWithDefault(node)
	}
}
impl From<JsArrayBindingPatternRestElement> for JsAnyArrayBindingPatternElement {
	fn from(node: JsArrayBindingPatternRestElement) -> JsAnyArrayBindingPatternElement {
		JsAnyArrayBindingPatternElement::JsArrayBindingPatternRestElement(node)
	}
}
impl AstNode for JsAnyArrayBindingPatternElement {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			JS_ARRAY_HOLE
			| JS_BINDING_PATTERN_WITH_DEFAULT
			| JS_ARRAY_BINDING_PATTERN_REST_ELEMENT => true,
			k if JsAnyBindingPattern::can_cast(k) => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_ARRAY_HOLE => JsAnyArrayBindingPatternElement::JsArrayHole(JsArrayHole { syntax }),
			JS_BINDING_PATTERN_WITH_DEFAULT => {
				JsAnyArrayBindingPatternElement::JsBindingPatternWithDefault(
					JsBindingPatternWithDefault { syntax },
				)
			}
			JS_ARRAY_BINDING_PATTERN_REST_ELEMENT => {
				JsAnyArrayBindingPatternElement::JsArrayBindingPatternRestElement(
					JsArrayBindingPatternRestElement { syntax },
				)
			}
			_ => {
				if let Some(js_any_binding_pattern) = JsAnyBindingPattern::cast(syntax) {
					return Some(JsAnyArrayBindingPatternElement::JsAnyBindingPattern(
						js_any_binding_pattern,
					));
				}
				return None;
			}
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsAnyArrayBindingPatternElement::JsArrayHole(it) => &it.syntax,
			JsAnyArrayBindingPatternElement::JsBindingPatternWithDefault(it) => &it.syntax,
			JsAnyArrayBindingPatternElement::JsArrayBindingPatternRestElement(it) => &it.syntax,
			JsAnyArrayBindingPatternElement::JsAnyBindingPattern(it) => it.syntax(),
		}
	}
}
impl std::fmt::Debug for JsAnyArrayBindingPatternElement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			JsAnyArrayBindingPatternElement::JsArrayHole(it) => std::fmt::Debug::fmt(it, f),
			JsAnyArrayBindingPatternElement::JsAnyBindingPattern(it) => std::fmt::Debug::fmt(it, f),
			JsAnyArrayBindingPatternElement::JsBindingPatternWithDefault(it) => {
				std::fmt::Debug::fmt(it, f)
			}
			JsAnyArrayBindingPatternElement::JsArrayBindingPatternRestElement(it) => {
				std::fmt::Debug::fmt(it, f)
			}
		}
	}
}
impl From<JsObjectBindingPatternProperty> for JsAnyObjectBindingPatternMember {
	fn from(node: JsObjectBindingPatternProperty) -> JsAnyObjectBindingPatternMember {
		JsAnyObjectBindingPatternMember::JsObjectBindingPatternProperty(node)
	}
}
impl From<JsObjectBindingPatternRest> for JsAnyObjectBindingPatternMember {
	fn from(node: JsObjectBindingPatternRest) -> JsAnyObjectBindingPatternMember {
		JsAnyObjectBindingPatternMember::JsObjectBindingPatternRest(node)
	}
}
impl From<JsObjectBindingPatternShorthandProperty> for JsAnyObjectBindingPatternMember {
	fn from(node: JsObjectBindingPatternShorthandProperty) -> JsAnyObjectBindingPatternMember {
		JsAnyObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(node)
	}
}
impl From<JsIdentifierBinding> for JsAnyObjectBindingPatternMember {
	fn from(node: JsIdentifierBinding) -> JsAnyObjectBindingPatternMember {
		JsAnyObjectBindingPatternMember::JsIdentifierBinding(node)
	}
}
impl From<JsUnknownBinding> for JsAnyObjectBindingPatternMember {
	fn from(node: JsUnknownBinding) -> JsAnyObjectBindingPatternMember {
		JsAnyObjectBindingPatternMember::JsUnknownBinding(node)
	}
}
impl AstNode for JsAnyObjectBindingPatternMember {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(
			kind,
			JS_OBJECT_BINDING_PATTERN_PROPERTY
				| JS_OBJECT_BINDING_PATTERN_REST
				| JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY
				| JS_IDENTIFIER_BINDING
				| JS_UNKNOWN_BINDING
		)
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_OBJECT_BINDING_PATTERN_PROPERTY => {
				JsAnyObjectBindingPatternMember::JsObjectBindingPatternProperty(
					JsObjectBindingPatternProperty { syntax },
				)
			}
			JS_OBJECT_BINDING_PATTERN_REST => {
				JsAnyObjectBindingPatternMember::JsObjectBindingPatternRest(
					JsObjectBindingPatternRest { syntax },
				)
			}
			JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY => {
				JsAnyObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(
					JsObjectBindingPatternShorthandProperty { syntax },
				)
			}
			JS_IDENTIFIER_BINDING => {
				JsAnyObjectBindingPatternMember::JsIdentifierBinding(JsIdentifierBinding { syntax })
			}
			JS_UNKNOWN_BINDING => {
				JsAnyObjectBindingPatternMember::JsUnknownBinding(JsUnknownBinding { syntax })
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsAnyObjectBindingPatternMember::JsObjectBindingPatternProperty(it) => &it.syntax,
			JsAnyObjectBindingPatternMember::JsObjectBindingPatternRest(it) => &it.syntax,
			JsAnyObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(it) => {
				&it.syntax
			}
			JsAnyObjectBindingPatternMember::JsIdentifierBinding(it) => &it.syntax,
			JsAnyObjectBindingPatternMember::JsUnknownBinding(it) => &it.syntax,
		}
	}
}
impl std::fmt::Debug for JsAnyObjectBindingPatternMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			JsAnyObjectBindingPatternMember::JsObjectBindingPatternProperty(it) => {
				std::fmt::Debug::fmt(it, f)
			}
			JsAnyObjectBindingPatternMember::JsObjectBindingPatternRest(it) => {
				std::fmt::Debug::fmt(it, f)
			}
			JsAnyObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(it) => {
				std::fmt::Debug::fmt(it, f)
			}
			JsAnyObjectBindingPatternMember::JsIdentifierBinding(it) => std::fmt::Debug::fmt(it, f),
			JsAnyObjectBindingPatternMember::JsUnknownBinding(it) => std::fmt::Debug::fmt(it, f),
		}
	}
}
impl From<TsAny> for TsType {
	fn from(node: TsAny) -> TsType { TsType::TsAny(node) }
}
impl From<TsUnknown> for TsType {
	fn from(node: TsUnknown) -> TsType { TsType::TsUnknown(node) }
}
impl From<TsNumber> for TsType {
	fn from(node: TsNumber) -> TsType { TsType::TsNumber(node) }
}
impl From<TsObject> for TsType {
	fn from(node: TsObject) -> TsType { TsType::TsObject(node) }
}
impl From<TsBoolean> for TsType {
	fn from(node: TsBoolean) -> TsType { TsType::TsBoolean(node) }
}
impl From<TsBigint> for TsType {
	fn from(node: TsBigint) -> TsType { TsType::TsBigint(node) }
}
impl From<TsString> for TsType {
	fn from(node: TsString) -> TsType { TsType::TsString(node) }
}
impl From<TsSymbol> for TsType {
	fn from(node: TsSymbol) -> TsType { TsType::TsSymbol(node) }
}
impl From<TsVoid> for TsType {
	fn from(node: TsVoid) -> TsType { TsType::TsVoid(node) }
}
impl From<TsUndefined> for TsType {
	fn from(node: TsUndefined) -> TsType { TsType::TsUndefined(node) }
}
impl From<TsNull> for TsType {
	fn from(node: TsNull) -> TsType { TsType::TsNull(node) }
}
impl From<TsNever> for TsType {
	fn from(node: TsNever) -> TsType { TsType::TsNever(node) }
}
impl From<TsThis> for TsType {
	fn from(node: TsThis) -> TsType { TsType::TsThis(node) }
}
impl From<TsLiteral> for TsType {
	fn from(node: TsLiteral) -> TsType { TsType::TsLiteral(node) }
}
impl From<TsPredicate> for TsType {
	fn from(node: TsPredicate) -> TsType { TsType::TsPredicate(node) }
}
impl From<TsTuple> for TsType {
	fn from(node: TsTuple) -> TsType { TsType::TsTuple(node) }
}
impl From<TsParen> for TsType {
	fn from(node: TsParen) -> TsType { TsType::TsParen(node) }
}
impl From<TsTypeRef> for TsType {
	fn from(node: TsTypeRef) -> TsType { TsType::TsTypeRef(node) }
}
impl From<TsTemplate> for TsType {
	fn from(node: TsTemplate) -> TsType { TsType::TsTemplate(node) }
}
impl From<TsMappedType> for TsType {
	fn from(node: TsMappedType) -> TsType { TsType::TsMappedType(node) }
}
impl From<TsImport> for TsType {
	fn from(node: TsImport) -> TsType { TsType::TsImport(node) }
}
impl From<TsArray> for TsType {
	fn from(node: TsArray) -> TsType { TsType::TsArray(node) }
}
impl From<TsIndexedArray> for TsType {
	fn from(node: TsIndexedArray) -> TsType { TsType::TsIndexedArray(node) }
}
impl From<TsTypeOperator> for TsType {
	fn from(node: TsTypeOperator) -> TsType { TsType::TsTypeOperator(node) }
}
impl From<TsIntersection> for TsType {
	fn from(node: TsIntersection) -> TsType { TsType::TsIntersection(node) }
}
impl From<TsUnion> for TsType {
	fn from(node: TsUnion) -> TsType { TsType::TsUnion(node) }
}
impl From<TsFnType> for TsType {
	fn from(node: TsFnType) -> TsType { TsType::TsFnType(node) }
}
impl From<TsConstructorType> for TsType {
	fn from(node: TsConstructorType) -> TsType { TsType::TsConstructorType(node) }
}
impl From<TsConditionalType> for TsType {
	fn from(node: TsConditionalType) -> TsType { TsType::TsConditionalType(node) }
}
impl From<TsObjectType> for TsType {
	fn from(node: TsObjectType) -> TsType { TsType::TsObjectType(node) }
}
impl From<TsInfer> for TsType {
	fn from(node: TsInfer) -> TsType { TsType::TsInfer(node) }
}
impl AstNode for TsType {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(
			kind,
			TS_ANY
				| TS_UNKNOWN | TS_NUMBER
				| TS_OBJECT | TS_BOOLEAN
				| TS_BIGINT | TS_STRING
				| TS_SYMBOL | TS_VOID
				| TS_UNDEFINED | TS_NULL
				| TS_NEVER | TS_THIS
				| TS_LITERAL | TS_PREDICATE
				| TS_TUPLE | TS_PAREN
				| TS_TYPE_REF | TS_TEMPLATE
				| TS_MAPPED_TYPE | TS_IMPORT
				| TS_ARRAY | TS_INDEXED_ARRAY
				| TS_TYPE_OPERATOR
				| TS_INTERSECTION
				| TS_UNION | TS_FN_TYPE
				| TS_CONSTRUCTOR_TYPE
				| TS_CONDITIONAL_TYPE
				| TS_OBJECT_TYPE | TS_INFER
		)
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			TS_ANY => TsType::TsAny(TsAny { syntax }),
			TS_UNKNOWN => TsType::TsUnknown(TsUnknown { syntax }),
			TS_NUMBER => TsType::TsNumber(TsNumber { syntax }),
			TS_OBJECT => TsType::TsObject(TsObject { syntax }),
			TS_BOOLEAN => TsType::TsBoolean(TsBoolean { syntax }),
			TS_BIGINT => TsType::TsBigint(TsBigint { syntax }),
			TS_STRING => TsType::TsString(TsString { syntax }),
			TS_SYMBOL => TsType::TsSymbol(TsSymbol { syntax }),
			TS_VOID => TsType::TsVoid(TsVoid { syntax }),
			TS_UNDEFINED => TsType::TsUndefined(TsUndefined { syntax }),
			TS_NULL => TsType::TsNull(TsNull { syntax }),
			TS_NEVER => TsType::TsNever(TsNever { syntax }),
			TS_THIS => TsType::TsThis(TsThis { syntax }),
			TS_LITERAL => TsType::TsLiteral(TsLiteral { syntax }),
			TS_PREDICATE => TsType::TsPredicate(TsPredicate { syntax }),
			TS_TUPLE => TsType::TsTuple(TsTuple { syntax }),
			TS_PAREN => TsType::TsParen(TsParen { syntax }),
			TS_TYPE_REF => TsType::TsTypeRef(TsTypeRef { syntax }),
			TS_TEMPLATE => TsType::TsTemplate(TsTemplate { syntax }),
			TS_MAPPED_TYPE => TsType::TsMappedType(TsMappedType { syntax }),
			TS_IMPORT => TsType::TsImport(TsImport { syntax }),
			TS_ARRAY => TsType::TsArray(TsArray { syntax }),
			TS_INDEXED_ARRAY => TsType::TsIndexedArray(TsIndexedArray { syntax }),
			TS_TYPE_OPERATOR => TsType::TsTypeOperator(TsTypeOperator { syntax }),
			TS_INTERSECTION => TsType::TsIntersection(TsIntersection { syntax }),
			TS_UNION => TsType::TsUnion(TsUnion { syntax }),
			TS_FN_TYPE => TsType::TsFnType(TsFnType { syntax }),
			TS_CONSTRUCTOR_TYPE => TsType::TsConstructorType(TsConstructorType { syntax }),
			TS_CONDITIONAL_TYPE => TsType::TsConditionalType(TsConditionalType { syntax }),
			TS_OBJECT_TYPE => TsType::TsObjectType(TsObjectType { syntax }),
			TS_INFER => TsType::TsInfer(TsInfer { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			TsType::TsAny(it) => &it.syntax,
			TsType::TsUnknown(it) => &it.syntax,
			TsType::TsNumber(it) => &it.syntax,
			TsType::TsObject(it) => &it.syntax,
			TsType::TsBoolean(it) => &it.syntax,
			TsType::TsBigint(it) => &it.syntax,
			TsType::TsString(it) => &it.syntax,
			TsType::TsSymbol(it) => &it.syntax,
			TsType::TsVoid(it) => &it.syntax,
			TsType::TsUndefined(it) => &it.syntax,
			TsType::TsNull(it) => &it.syntax,
			TsType::TsNever(it) => &it.syntax,
			TsType::TsThis(it) => &it.syntax,
			TsType::TsLiteral(it) => &it.syntax,
			TsType::TsPredicate(it) => &it.syntax,
			TsType::TsTuple(it) => &it.syntax,
			TsType::TsParen(it) => &it.syntax,
			TsType::TsTypeRef(it) => &it.syntax,
			TsType::TsTemplate(it) => &it.syntax,
			TsType::TsMappedType(it) => &it.syntax,
			TsType::TsImport(it) => &it.syntax,
			TsType::TsArray(it) => &it.syntax,
			TsType::TsIndexedArray(it) => &it.syntax,
			TsType::TsTypeOperator(it) => &it.syntax,
			TsType::TsIntersection(it) => &it.syntax,
			TsType::TsUnion(it) => &it.syntax,
			TsType::TsFnType(it) => &it.syntax,
			TsType::TsConstructorType(it) => &it.syntax,
			TsType::TsConditionalType(it) => &it.syntax,
			TsType::TsObjectType(it) => &it.syntax,
			TsType::TsInfer(it) => &it.syntax,
		}
	}
}
impl std::fmt::Debug for TsType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			TsType::TsAny(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsUnknown(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsNumber(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsObject(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsBoolean(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsBigint(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsString(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsSymbol(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsVoid(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsUndefined(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsNull(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsNever(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsThis(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsLiteral(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsPredicate(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsTuple(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsParen(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsTypeRef(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsTemplate(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsMappedType(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsImport(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsArray(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsIndexedArray(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsTypeOperator(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsIntersection(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsUnion(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsFnType(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsConstructorType(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsConditionalType(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsObjectType(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsInfer(it) => std::fmt::Debug::fmt(it, f),
		}
	}
}
impl From<JsImportBareClause> for AnyJsImportClause {
	fn from(node: JsImportBareClause) -> AnyJsImportClause {
		AnyJsImportClause::JsImportBareClause(node)
	}
}
impl From<JsName> for AnyJsImportClause {
	fn from(node: JsName) -> AnyJsImportClause { AnyJsImportClause::JsName(node) }
}
impl From<JsImportNamedClause> for AnyJsImportClause {
	fn from(node: JsImportNamedClause) -> AnyJsImportClause {
		AnyJsImportClause::JsImportNamedClause(node)
	}
}
impl From<JsImportDefaultClause> for AnyJsImportClause {
	fn from(node: JsImportDefaultClause) -> AnyJsImportClause {
		AnyJsImportClause::JsImportDefaultClause(node)
	}
}
impl From<JsImportNamespaceClause> for AnyJsImportClause {
	fn from(node: JsImportNamespaceClause) -> AnyJsImportClause {
		AnyJsImportClause::JsImportNamespaceClause(node)
	}
}
impl AstNode for AnyJsImportClause {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(
			kind,
			JS_IMPORT_BARE_CLAUSE
				| JS_NAME | JS_IMPORT_NAMED_CLAUSE
				| JS_IMPORT_DEFAULT_CLAUSE
				| JS_IMPORT_NAMESPACE_CLAUSE
		)
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_IMPORT_BARE_CLAUSE => {
				AnyJsImportClause::JsImportBareClause(JsImportBareClause { syntax })
			}
			JS_NAME => AnyJsImportClause::JsName(JsName { syntax }),
			JS_IMPORT_NAMED_CLAUSE => {
				AnyJsImportClause::JsImportNamedClause(JsImportNamedClause { syntax })
			}
			JS_IMPORT_DEFAULT_CLAUSE => {
				AnyJsImportClause::JsImportDefaultClause(JsImportDefaultClause { syntax })
			}
			JS_IMPORT_NAMESPACE_CLAUSE => {
				AnyJsImportClause::JsImportNamespaceClause(JsImportNamespaceClause { syntax })
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			AnyJsImportClause::JsImportBareClause(it) => &it.syntax,
			AnyJsImportClause::JsName(it) => &it.syntax,
			AnyJsImportClause::JsImportNamedClause(it) => &it.syntax,
			AnyJsImportClause::JsImportDefaultClause(it) => &it.syntax,
			AnyJsImportClause::JsImportNamespaceClause(it) => &it.syntax,
		}
	}
}
impl std::fmt::Debug for AnyJsImportClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			AnyJsImportClause::JsImportBareClause(it) => std::fmt::Debug::fmt(it, f),
			AnyJsImportClause::JsName(it) => std::fmt::Debug::fmt(it, f),
			AnyJsImportClause::JsImportNamedClause(it) => std::fmt::Debug::fmt(it, f),
			AnyJsImportClause::JsImportDefaultClause(it) => std::fmt::Debug::fmt(it, f),
			AnyJsImportClause::JsImportNamespaceClause(it) => std::fmt::Debug::fmt(it, f),
		}
	}
}
impl From<JsNamedImportSpecifierList> for JsAnyNamedImport {
	fn from(node: JsNamedImportSpecifierList) -> JsAnyNamedImport {
		JsAnyNamedImport::JsNamedImportSpecifierList(node)
	}
}
impl From<JsNamespaceImportSpecifier> for JsAnyNamedImport {
	fn from(node: JsNamespaceImportSpecifier) -> JsAnyNamedImport {
		JsAnyNamedImport::JsNamespaceImportSpecifier(node)
	}
}
impl AstNode for JsAnyNamedImport {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(
			kind,
			JS_NAMED_IMPORT_SPECIFIER_LIST | JS_NAMESPACE_IMPORT_SPECIFIER
		)
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_NAMED_IMPORT_SPECIFIER_LIST => {
				JsAnyNamedImport::JsNamedImportSpecifierList(JsNamedImportSpecifierList { syntax })
			}
			JS_NAMESPACE_IMPORT_SPECIFIER => {
				JsAnyNamedImport::JsNamespaceImportSpecifier(JsNamespaceImportSpecifier { syntax })
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsAnyNamedImport::JsNamedImportSpecifierList(it) => &it.syntax,
			JsAnyNamedImport::JsNamespaceImportSpecifier(it) => &it.syntax,
		}
	}
}
impl std::fmt::Debug for JsAnyNamedImport {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			JsAnyNamedImport::JsNamedImportSpecifierList(it) => std::fmt::Debug::fmt(it, f),
			JsAnyNamedImport::JsNamespaceImportSpecifier(it) => std::fmt::Debug::fmt(it, f),
		}
	}
}
impl From<JsShorthandNamedImportSpecifier> for JsAnyNamedImportSpecifier {
	fn from(node: JsShorthandNamedImportSpecifier) -> JsAnyNamedImportSpecifier {
		JsAnyNamedImportSpecifier::JsShorthandNamedImportSpecifier(node)
	}
}
impl From<JsNamedImportSpecifier> for JsAnyNamedImportSpecifier {
	fn from(node: JsNamedImportSpecifier) -> JsAnyNamedImportSpecifier {
		JsAnyNamedImportSpecifier::JsNamedImportSpecifier(node)
	}
}
impl From<JsUnknownNamedImportSpecifier> for JsAnyNamedImportSpecifier {
	fn from(node: JsUnknownNamedImportSpecifier) -> JsAnyNamedImportSpecifier {
		JsAnyNamedImportSpecifier::JsUnknownNamedImportSpecifier(node)
	}
}
impl AstNode for JsAnyNamedImportSpecifier {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(
			kind,
			JS_SHORTHAND_NAMED_IMPORT_SPECIFIER
				| JS_NAMED_IMPORT_SPECIFIER
				| JS_UNKNOWN_NAMED_IMPORT_SPECIFIER
		)
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_SHORTHAND_NAMED_IMPORT_SPECIFIER => {
				JsAnyNamedImportSpecifier::JsShorthandNamedImportSpecifier(
					JsShorthandNamedImportSpecifier { syntax },
				)
			}
			JS_NAMED_IMPORT_SPECIFIER => {
				JsAnyNamedImportSpecifier::JsNamedImportSpecifier(JsNamedImportSpecifier { syntax })
			}
			JS_UNKNOWN_NAMED_IMPORT_SPECIFIER => {
				JsAnyNamedImportSpecifier::JsUnknownNamedImportSpecifier(
					JsUnknownNamedImportSpecifier { syntax },
				)
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsAnyNamedImportSpecifier::JsShorthandNamedImportSpecifier(it) => &it.syntax,
			JsAnyNamedImportSpecifier::JsNamedImportSpecifier(it) => &it.syntax,
			JsAnyNamedImportSpecifier::JsUnknownNamedImportSpecifier(it) => &it.syntax,
		}
	}
}
impl std::fmt::Debug for JsAnyNamedImportSpecifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			JsAnyNamedImportSpecifier::JsShorthandNamedImportSpecifier(it) => {
				std::fmt::Debug::fmt(it, f)
			}
			JsAnyNamedImportSpecifier::JsNamedImportSpecifier(it) => std::fmt::Debug::fmt(it, f),
			JsAnyNamedImportSpecifier::JsUnknownNamedImportSpecifier(it) => {
				std::fmt::Debug::fmt(it, f)
			}
		}
	}
}
impl From<JsImportAssertionEntry> for JsAnyImportAssertionEntry {
	fn from(node: JsImportAssertionEntry) -> JsAnyImportAssertionEntry {
		JsAnyImportAssertionEntry::JsImportAssertionEntry(node)
	}
}
impl From<JsUnknownImportAssertionEntry> for JsAnyImportAssertionEntry {
	fn from(node: JsUnknownImportAssertionEntry) -> JsAnyImportAssertionEntry {
		JsAnyImportAssertionEntry::JsUnknownImportAssertionEntry(node)
	}
}
impl AstNode for JsAnyImportAssertionEntry {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(
			kind,
			JS_IMPORT_ASSERTION_ENTRY | JS_UNKNOWN_IMPORT_ASSERTION_ENTRY
		)
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_IMPORT_ASSERTION_ENTRY => {
				JsAnyImportAssertionEntry::JsImportAssertionEntry(JsImportAssertionEntry { syntax })
			}
			JS_UNKNOWN_IMPORT_ASSERTION_ENTRY => {
				JsAnyImportAssertionEntry::JsUnknownImportAssertionEntry(
					JsUnknownImportAssertionEntry { syntax },
				)
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsAnyImportAssertionEntry::JsImportAssertionEntry(it) => &it.syntax,
			JsAnyImportAssertionEntry::JsUnknownImportAssertionEntry(it) => &it.syntax,
		}
	}
}
impl std::fmt::Debug for JsAnyImportAssertionEntry {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			JsAnyImportAssertionEntry::JsImportAssertionEntry(it) => std::fmt::Debug::fmt(it, f),
			JsAnyImportAssertionEntry::JsUnknownImportAssertionEntry(it) => {
				std::fmt::Debug::fmt(it, f)
			}
		}
	}
}
impl From<JsFunctionDeclaration> for DefaultDecl {
	fn from(node: JsFunctionDeclaration) -> DefaultDecl { DefaultDecl::JsFunctionDeclaration(node) }
}
impl From<JsClassDeclaration> for DefaultDecl {
	fn from(node: JsClassDeclaration) -> DefaultDecl { DefaultDecl::JsClassDeclaration(node) }
}
impl AstNode for DefaultDecl {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(kind, JS_FUNCTION_DECLARATION | JS_CLASS_DECLARATION)
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_FUNCTION_DECLARATION => {
				DefaultDecl::JsFunctionDeclaration(JsFunctionDeclaration { syntax })
			}
			JS_CLASS_DECLARATION => DefaultDecl::JsClassDeclaration(JsClassDeclaration { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			DefaultDecl::JsFunctionDeclaration(it) => &it.syntax,
			DefaultDecl::JsClassDeclaration(it) => &it.syntax,
		}
	}
}
impl std::fmt::Debug for DefaultDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			DefaultDecl::JsFunctionDeclaration(it) => std::fmt::Debug::fmt(it, f),
			DefaultDecl::JsClassDeclaration(it) => std::fmt::Debug::fmt(it, f),
		}
	}
}
impl From<JsFunctionDeclaration> for JsAnyExportDeclaration {
	fn from(node: JsFunctionDeclaration) -> JsAnyExportDeclaration {
		JsAnyExportDeclaration::JsFunctionDeclaration(node)
	}
}
impl From<JsClassDeclaration> for JsAnyExportDeclaration {
	fn from(node: JsClassDeclaration) -> JsAnyExportDeclaration {
		JsAnyExportDeclaration::JsClassDeclaration(node)
	}
}
impl From<JsVariableDeclarationStatement> for JsAnyExportDeclaration {
	fn from(node: JsVariableDeclarationStatement) -> JsAnyExportDeclaration {
		JsAnyExportDeclaration::JsVariableDeclarationStatement(node)
	}
}
impl From<TsEnum> for JsAnyExportDeclaration {
	fn from(node: TsEnum) -> JsAnyExportDeclaration { JsAnyExportDeclaration::TsEnum(node) }
}
impl From<TsTypeAliasDecl> for JsAnyExportDeclaration {
	fn from(node: TsTypeAliasDecl) -> JsAnyExportDeclaration {
		JsAnyExportDeclaration::TsTypeAliasDecl(node)
	}
}
impl From<TsNamespaceDecl> for JsAnyExportDeclaration {
	fn from(node: TsNamespaceDecl) -> JsAnyExportDeclaration {
		JsAnyExportDeclaration::TsNamespaceDecl(node)
	}
}
impl From<TsModuleDecl> for JsAnyExportDeclaration {
	fn from(node: TsModuleDecl) -> JsAnyExportDeclaration {
		JsAnyExportDeclaration::TsModuleDecl(node)
	}
}
impl From<TsInterfaceDecl> for JsAnyExportDeclaration {
	fn from(node: TsInterfaceDecl) -> JsAnyExportDeclaration {
		JsAnyExportDeclaration::TsInterfaceDecl(node)
	}
}
impl AstNode for JsAnyExportDeclaration {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(
			kind,
			JS_FUNCTION_DECLARATION
				| JS_CLASS_DECLARATION
				| JS_VARIABLE_DECLARATION_STATEMENT
				| TS_ENUM | TS_TYPE_ALIAS_DECL
				| TS_NAMESPACE_DECL
				| TS_MODULE_DECL | TS_INTERFACE_DECL
		)
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_FUNCTION_DECLARATION => {
				JsAnyExportDeclaration::JsFunctionDeclaration(JsFunctionDeclaration { syntax })
			}
			JS_CLASS_DECLARATION => {
				JsAnyExportDeclaration::JsClassDeclaration(JsClassDeclaration { syntax })
			}
			JS_VARIABLE_DECLARATION_STATEMENT => {
				JsAnyExportDeclaration::JsVariableDeclarationStatement(
					JsVariableDeclarationStatement { syntax },
				)
			}
			TS_ENUM => JsAnyExportDeclaration::TsEnum(TsEnum { syntax }),
			TS_TYPE_ALIAS_DECL => {
				JsAnyExportDeclaration::TsTypeAliasDecl(TsTypeAliasDecl { syntax })
			}
			TS_NAMESPACE_DECL => {
				JsAnyExportDeclaration::TsNamespaceDecl(TsNamespaceDecl { syntax })
			}
			TS_MODULE_DECL => JsAnyExportDeclaration::TsModuleDecl(TsModuleDecl { syntax }),
			TS_INTERFACE_DECL => {
				JsAnyExportDeclaration::TsInterfaceDecl(TsInterfaceDecl { syntax })
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsAnyExportDeclaration::JsFunctionDeclaration(it) => &it.syntax,
			JsAnyExportDeclaration::JsClassDeclaration(it) => &it.syntax,
			JsAnyExportDeclaration::JsVariableDeclarationStatement(it) => &it.syntax,
			JsAnyExportDeclaration::TsEnum(it) => &it.syntax,
			JsAnyExportDeclaration::TsTypeAliasDecl(it) => &it.syntax,
			JsAnyExportDeclaration::TsNamespaceDecl(it) => &it.syntax,
			JsAnyExportDeclaration::TsModuleDecl(it) => &it.syntax,
			JsAnyExportDeclaration::TsInterfaceDecl(it) => &it.syntax,
		}
	}
}
impl std::fmt::Debug for JsAnyExportDeclaration {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			JsAnyExportDeclaration::JsFunctionDeclaration(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExportDeclaration::JsClassDeclaration(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExportDeclaration::JsVariableDeclarationStatement(it) => {
				std::fmt::Debug::fmt(it, f)
			}
			JsAnyExportDeclaration::TsEnum(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExportDeclaration::TsTypeAliasDecl(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExportDeclaration::TsNamespaceDecl(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExportDeclaration::TsModuleDecl(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExportDeclaration::TsInterfaceDecl(it) => std::fmt::Debug::fmt(it, f),
		}
	}
}
impl From<JsBindingPatternWithDefault> for JsAnyParameter {
	fn from(node: JsBindingPatternWithDefault) -> JsAnyParameter {
		JsAnyParameter::JsBindingPatternWithDefault(node)
	}
}
impl From<JsRestParameter> for JsAnyParameter {
	fn from(node: JsRestParameter) -> JsAnyParameter { JsAnyParameter::JsRestParameter(node) }
}
impl AstNode for JsAnyParameter {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			JS_BINDING_PATTERN_WITH_DEFAULT | JS_REST_PARAMETER => true,
			k if JsAnyBindingPattern::can_cast(k) => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_BINDING_PATTERN_WITH_DEFAULT => {
				JsAnyParameter::JsBindingPatternWithDefault(JsBindingPatternWithDefault { syntax })
			}
			JS_REST_PARAMETER => JsAnyParameter::JsRestParameter(JsRestParameter { syntax }),
			_ => {
				if let Some(js_any_binding_pattern) = JsAnyBindingPattern::cast(syntax) {
					return Some(JsAnyParameter::JsAnyBindingPattern(js_any_binding_pattern));
				}
				return None;
			}
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsAnyParameter::JsBindingPatternWithDefault(it) => &it.syntax,
			JsAnyParameter::JsRestParameter(it) => &it.syntax,
			JsAnyParameter::JsAnyBindingPattern(it) => it.syntax(),
		}
	}
}
impl std::fmt::Debug for JsAnyParameter {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			JsAnyParameter::JsAnyBindingPattern(it) => std::fmt::Debug::fmt(it, f),
			JsAnyParameter::JsBindingPatternWithDefault(it) => std::fmt::Debug::fmt(it, f),
			JsAnyParameter::JsRestParameter(it) => std::fmt::Debug::fmt(it, f),
		}
	}
}
impl From<TsExternalModuleRef> for TsModuleRef {
	fn from(node: TsExternalModuleRef) -> TsModuleRef { TsModuleRef::TsExternalModuleRef(node) }
}
impl AstNode for TsModuleRef {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			TS_EXTERNAL_MODULE_REF => true,
			k if TsEntityName::can_cast(k) => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			TS_EXTERNAL_MODULE_REF => {
				TsModuleRef::TsExternalModuleRef(TsExternalModuleRef { syntax })
			}
			_ => {
				if let Some(ts_entity_name) = TsEntityName::cast(syntax) {
					return Some(TsModuleRef::TsEntityName(ts_entity_name));
				}
				return None;
			}
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			TsModuleRef::TsExternalModuleRef(it) => &it.syntax,
			TsModuleRef::TsEntityName(it) => it.syntax(),
		}
	}
}
impl std::fmt::Debug for TsModuleRef {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			TsModuleRef::TsExternalModuleRef(it) => std::fmt::Debug::fmt(it, f),
			TsModuleRef::TsEntityName(it) => std::fmt::Debug::fmt(it, f),
		}
	}
}
impl From<TsTypeName> for TsEntityName {
	fn from(node: TsTypeName) -> TsEntityName { TsEntityName::TsTypeName(node) }
}
impl From<TsQualifiedPath> for TsEntityName {
	fn from(node: TsQualifiedPath) -> TsEntityName { TsEntityName::TsQualifiedPath(node) }
}
impl AstNode for TsEntityName {
	fn can_cast(kind: SyntaxKind) -> bool { matches!(kind, TS_TYPE_NAME | TS_QUALIFIED_PATH) }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			TS_TYPE_NAME => TsEntityName::TsTypeName(TsTypeName { syntax }),
			TS_QUALIFIED_PATH => TsEntityName::TsQualifiedPath(TsQualifiedPath { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			TsEntityName::TsTypeName(it) => &it.syntax,
			TsEntityName::TsQualifiedPath(it) => &it.syntax,
		}
	}
}
impl std::fmt::Debug for TsEntityName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			TsEntityName::TsTypeName(it) => std::fmt::Debug::fmt(it, f),
			TsEntityName::TsQualifiedPath(it) => std::fmt::Debug::fmt(it, f),
		}
	}
}
impl From<TsThis> for TsThisOrMore {
	fn from(node: TsThis) -> TsThisOrMore { TsThisOrMore::TsThis(node) }
}
impl From<TsTypeName> for TsThisOrMore {
	fn from(node: TsTypeName) -> TsThisOrMore { TsThisOrMore::TsTypeName(node) }
}
impl AstNode for TsThisOrMore {
	fn can_cast(kind: SyntaxKind) -> bool { matches!(kind, TS_THIS | TS_TYPE_NAME) }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			TS_THIS => TsThisOrMore::TsThis(TsThis { syntax }),
			TS_TYPE_NAME => TsThisOrMore::TsTypeName(TsTypeName { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			TsThisOrMore::TsThis(it) => &it.syntax,
			TsThisOrMore::TsTypeName(it) => &it.syntax,
		}
	}
}
impl std::fmt::Debug for TsThisOrMore {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			TsThisOrMore::TsThis(it) => std::fmt::Debug::fmt(it, f),
			TsThisOrMore::TsTypeName(it) => std::fmt::Debug::fmt(it, f),
		}
	}
}
impl From<TsCallSignatureDecl> for TsTypeElement {
	fn from(node: TsCallSignatureDecl) -> TsTypeElement { TsTypeElement::TsCallSignatureDecl(node) }
}
impl From<TsConstructSignatureDecl> for TsTypeElement {
	fn from(node: TsConstructSignatureDecl) -> TsTypeElement {
		TsTypeElement::TsConstructSignatureDecl(node)
	}
}
impl From<TsPropertySignature> for TsTypeElement {
	fn from(node: TsPropertySignature) -> TsTypeElement { TsTypeElement::TsPropertySignature(node) }
}
impl From<TsMethodSignature> for TsTypeElement {
	fn from(node: TsMethodSignature) -> TsTypeElement { TsTypeElement::TsMethodSignature(node) }
}
impl From<TsIndexSignature> for TsTypeElement {
	fn from(node: TsIndexSignature) -> TsTypeElement { TsTypeElement::TsIndexSignature(node) }
}
impl AstNode for TsTypeElement {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(
			kind,
			TS_CALL_SIGNATURE_DECL
				| TS_CONSTRUCT_SIGNATURE_DECL
				| TS_PROPERTY_SIGNATURE
				| TS_METHOD_SIGNATURE
				| TS_INDEX_SIGNATURE
		)
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			TS_CALL_SIGNATURE_DECL => {
				TsTypeElement::TsCallSignatureDecl(TsCallSignatureDecl { syntax })
			}
			TS_CONSTRUCT_SIGNATURE_DECL => {
				TsTypeElement::TsConstructSignatureDecl(TsConstructSignatureDecl { syntax })
			}
			TS_PROPERTY_SIGNATURE => {
				TsTypeElement::TsPropertySignature(TsPropertySignature { syntax })
			}
			TS_METHOD_SIGNATURE => TsTypeElement::TsMethodSignature(TsMethodSignature { syntax }),
			TS_INDEX_SIGNATURE => TsTypeElement::TsIndexSignature(TsIndexSignature { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			TsTypeElement::TsCallSignatureDecl(it) => &it.syntax,
			TsTypeElement::TsConstructSignatureDecl(it) => &it.syntax,
			TsTypeElement::TsPropertySignature(it) => &it.syntax,
			TsTypeElement::TsMethodSignature(it) => &it.syntax,
			TsTypeElement::TsIndexSignature(it) => &it.syntax,
		}
	}
}
impl std::fmt::Debug for TsTypeElement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			TsTypeElement::TsCallSignatureDecl(it) => std::fmt::Debug::fmt(it, f),
			TsTypeElement::TsConstructSignatureDecl(it) => std::fmt::Debug::fmt(it, f),
			TsTypeElement::TsPropertySignature(it) => std::fmt::Debug::fmt(it, f),
			TsTypeElement::TsMethodSignature(it) => std::fmt::Debug::fmt(it, f),
			TsTypeElement::TsIndexSignature(it) => std::fmt::Debug::fmt(it, f),
		}
	}
}
impl From<TsModuleBlock> for TsNamespaceBody {
	fn from(node: TsModuleBlock) -> TsNamespaceBody { TsNamespaceBody::TsModuleBlock(node) }
}
impl From<TsNamespaceDecl> for TsNamespaceBody {
	fn from(node: TsNamespaceDecl) -> TsNamespaceBody { TsNamespaceBody::TsNamespaceDecl(node) }
}
impl AstNode for TsNamespaceBody {
	fn can_cast(kind: SyntaxKind) -> bool { matches!(kind, TS_MODULE_BLOCK | TS_NAMESPACE_DECL) }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			TS_MODULE_BLOCK => TsNamespaceBody::TsModuleBlock(TsModuleBlock { syntax }),
			TS_NAMESPACE_DECL => TsNamespaceBody::TsNamespaceDecl(TsNamespaceDecl { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			TsNamespaceBody::TsModuleBlock(it) => &it.syntax,
			TsNamespaceBody::TsNamespaceDecl(it) => &it.syntax,
		}
	}
}
impl std::fmt::Debug for TsNamespaceBody {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			TsNamespaceBody::TsModuleBlock(it) => std::fmt::Debug::fmt(it, f),
			TsNamespaceBody::TsNamespaceDecl(it) => std::fmt::Debug::fmt(it, f),
		}
	}
}
impl From<JsUnknownStatement> for AnyNode {
	fn from(node: JsUnknownStatement) -> AnyNode { AnyNode::JsUnknownStatement(node) }
}
impl From<JsUnknownExpression> for AnyNode {
	fn from(node: JsUnknownExpression) -> AnyNode { AnyNode::JsUnknownExpression(node) }
}
impl From<JsUnknownMember> for AnyNode {
	fn from(node: JsUnknownMember) -> AnyNode { AnyNode::JsUnknownMember(node) }
}
impl From<JsUnknownBinding> for AnyNode {
	fn from(node: JsUnknownBinding) -> AnyNode { AnyNode::JsUnknownBinding(node) }
}
impl From<JsUnknownAssignment> for AnyNode {
	fn from(node: JsUnknownAssignment) -> AnyNode { AnyNode::JsUnknownAssignment(node) }
}
impl From<JsUnknownModifier> for AnyNode {
	fn from(node: JsUnknownModifier) -> AnyNode { AnyNode::JsUnknownModifier(node) }
}
impl From<JsUnknownImportAssertionEntry> for AnyNode {
	fn from(node: JsUnknownImportAssertionEntry) -> AnyNode {
		AnyNode::JsUnknownImportAssertionEntry(node)
	}
}
impl From<JsUnknownNamedImportSpecifier> for AnyNode {
	fn from(node: JsUnknownNamedImportSpecifier) -> AnyNode {
		AnyNode::JsUnknownNamedImportSpecifier(node)
	}
}
impl From<List> for AnyNode {
	fn from(node: List) -> AnyNode { AnyNode::List(node) }
}
impl From<Ident> for AnyNode {
	fn from(node: Ident) -> AnyNode { AnyNode::Ident(node) }
}
impl From<JsScript> for AnyNode {
	fn from(node: JsScript) -> AnyNode { AnyNode::JsScript(node) }
}
impl From<JsModule> for AnyNode {
	fn from(node: JsModule) -> AnyNode { AnyNode::JsModule(node) }
}
impl From<JsDirective> for AnyNode {
	fn from(node: JsDirective) -> AnyNode { AnyNode::JsDirective(node) }
}
impl From<JsBlockStatement> for AnyNode {
	fn from(node: JsBlockStatement) -> AnyNode { AnyNode::JsBlockStatement(node) }
}
impl From<JsEmptyStatement> for AnyNode {
	fn from(node: JsEmptyStatement) -> AnyNode { AnyNode::JsEmptyStatement(node) }
}
impl From<JsExpressionStatement> for AnyNode {
	fn from(node: JsExpressionStatement) -> AnyNode { AnyNode::JsExpressionStatement(node) }
}
impl From<JsIfStatement> for AnyNode {
	fn from(node: JsIfStatement) -> AnyNode { AnyNode::JsIfStatement(node) }
}
impl From<JsDoWhileStatement> for AnyNode {
	fn from(node: JsDoWhileStatement) -> AnyNode { AnyNode::JsDoWhileStatement(node) }
}
impl From<JsWhileStatement> for AnyNode {
	fn from(node: JsWhileStatement) -> AnyNode { AnyNode::JsWhileStatement(node) }
}
impl From<ForStmt> for AnyNode {
	fn from(node: ForStmt) -> AnyNode { AnyNode::ForStmt(node) }
}
impl From<ForInStmt> for AnyNode {
	fn from(node: ForInStmt) -> AnyNode { AnyNode::ForInStmt(node) }
}
impl From<ForOfStmt> for AnyNode {
	fn from(node: ForOfStmt) -> AnyNode { AnyNode::ForOfStmt(node) }
}
impl From<JsContinueStatement> for AnyNode {
	fn from(node: JsContinueStatement) -> AnyNode { AnyNode::JsContinueStatement(node) }
}
impl From<JsBreakStatement> for AnyNode {
	fn from(node: JsBreakStatement) -> AnyNode { AnyNode::JsBreakStatement(node) }
}
impl From<JsReturnStatement> for AnyNode {
	fn from(node: JsReturnStatement) -> AnyNode { AnyNode::JsReturnStatement(node) }
}
impl From<JsWithStatement> for AnyNode {
	fn from(node: JsWithStatement) -> AnyNode { AnyNode::JsWithStatement(node) }
}
impl From<JsLabeledStatement> for AnyNode {
	fn from(node: JsLabeledStatement) -> AnyNode { AnyNode::JsLabeledStatement(node) }
}
impl From<JsSwitchStatement> for AnyNode {
	fn from(node: JsSwitchStatement) -> AnyNode { AnyNode::JsSwitchStatement(node) }
}
impl From<JsThrowStatement> for AnyNode {
	fn from(node: JsThrowStatement) -> AnyNode { AnyNode::JsThrowStatement(node) }
}
impl From<JsTryStatement> for AnyNode {
	fn from(node: JsTryStatement) -> AnyNode { AnyNode::JsTryStatement(node) }
}
impl From<JsTryFinallyStatement> for AnyNode {
	fn from(node: JsTryFinallyStatement) -> AnyNode { AnyNode::JsTryFinallyStatement(node) }
}
impl From<JsDebuggerStatement> for AnyNode {
	fn from(node: JsDebuggerStatement) -> AnyNode { AnyNode::JsDebuggerStatement(node) }
}
impl From<JsFunctionDeclaration> for AnyNode {
	fn from(node: JsFunctionDeclaration) -> AnyNode { AnyNode::JsFunctionDeclaration(node) }
}
impl From<JsClassDeclaration> for AnyNode {
	fn from(node: JsClassDeclaration) -> AnyNode { AnyNode::JsClassDeclaration(node) }
}
impl From<JsVariableDeclarationStatement> for AnyNode {
	fn from(node: JsVariableDeclarationStatement) -> AnyNode {
		AnyNode::JsVariableDeclarationStatement(node)
	}
}
impl From<TsEnum> for AnyNode {
	fn from(node: TsEnum) -> AnyNode { AnyNode::TsEnum(node) }
}
impl From<TsTypeAliasDecl> for AnyNode {
	fn from(node: TsTypeAliasDecl) -> AnyNode { AnyNode::TsTypeAliasDecl(node) }
}
impl From<TsNamespaceDecl> for AnyNode {
	fn from(node: TsNamespaceDecl) -> AnyNode { AnyNode::TsNamespaceDecl(node) }
}
impl From<TsModuleDecl> for AnyNode {
	fn from(node: TsModuleDecl) -> AnyNode { AnyNode::TsModuleDecl(node) }
}
impl From<TsInterfaceDecl> for AnyNode {
	fn from(node: TsInterfaceDecl) -> AnyNode { AnyNode::TsInterfaceDecl(node) }
}
impl From<JsElseClause> for AnyNode {
	fn from(node: JsElseClause) -> AnyNode { AnyNode::JsElseClause(node) }
}
impl From<ForStmtInit> for AnyNode {
	fn from(node: ForStmtInit) -> AnyNode { AnyNode::ForStmtInit(node) }
}
impl From<ForStmtTest> for AnyNode {
	fn from(node: ForStmtTest) -> AnyNode { AnyNode::ForStmtTest(node) }
}
impl From<ForStmtUpdate> for AnyNode {
	fn from(node: ForStmtUpdate) -> AnyNode { AnyNode::ForStmtUpdate(node) }
}
impl From<JsVariableDeclaration> for AnyNode {
	fn from(node: JsVariableDeclaration) -> AnyNode { AnyNode::JsVariableDeclaration(node) }
}
impl From<JsCaseClause> for AnyNode {
	fn from(node: JsCaseClause) -> AnyNode { AnyNode::JsCaseClause(node) }
}
impl From<JsDefaultClause> for AnyNode {
	fn from(node: JsDefaultClause) -> AnyNode { AnyNode::JsDefaultClause(node) }
}
impl From<JsCatchClause> for AnyNode {
	fn from(node: JsCatchClause) -> AnyNode { AnyNode::JsCatchClause(node) }
}
impl From<JsFinallyClause> for AnyNode {
	fn from(node: JsFinallyClause) -> AnyNode { AnyNode::JsFinallyClause(node) }
}
impl From<JsCatchDeclaration> for AnyNode {
	fn from(node: JsCatchDeclaration) -> AnyNode { AnyNode::JsCatchDeclaration(node) }
}
impl From<JsArrayExpression> for AnyNode {
	fn from(node: JsArrayExpression) -> AnyNode { AnyNode::JsArrayExpression(node) }
}
impl From<JsArrowFunctionExpression> for AnyNode {
	fn from(node: JsArrowFunctionExpression) -> AnyNode { AnyNode::JsArrowFunctionExpression(node) }
}
impl From<JsAssignmentExpression> for AnyNode {
	fn from(node: JsAssignmentExpression) -> AnyNode { AnyNode::JsAssignmentExpression(node) }
}
impl From<JsAwaitExpression> for AnyNode {
	fn from(node: JsAwaitExpression) -> AnyNode { AnyNode::JsAwaitExpression(node) }
}
impl From<JsBinaryExpression> for AnyNode {
	fn from(node: JsBinaryExpression) -> AnyNode { AnyNode::JsBinaryExpression(node) }
}
impl From<JsClassExpression> for AnyNode {
	fn from(node: JsClassExpression) -> AnyNode { AnyNode::JsClassExpression(node) }
}
impl From<JsConditionalExpression> for AnyNode {
	fn from(node: JsConditionalExpression) -> AnyNode { AnyNode::JsConditionalExpression(node) }
}
impl From<JsComputedMemberExpression> for AnyNode {
	fn from(node: JsComputedMemberExpression) -> AnyNode {
		AnyNode::JsComputedMemberExpression(node)
	}
}
impl From<JsFunctionExpression> for AnyNode {
	fn from(node: JsFunctionExpression) -> AnyNode { AnyNode::JsFunctionExpression(node) }
}
impl From<JsImportCallExpression> for AnyNode {
	fn from(node: JsImportCallExpression) -> AnyNode { AnyNode::JsImportCallExpression(node) }
}
impl From<JsLogicalExpression> for AnyNode {
	fn from(node: JsLogicalExpression) -> AnyNode { AnyNode::JsLogicalExpression(node) }
}
impl From<JsObjectExpression> for AnyNode {
	fn from(node: JsObjectExpression) -> AnyNode { AnyNode::JsObjectExpression(node) }
}
impl From<JsParenthesizedExpression> for AnyNode {
	fn from(node: JsParenthesizedExpression) -> AnyNode { AnyNode::JsParenthesizedExpression(node) }
}
impl From<JsIdentifierExpression> for AnyNode {
	fn from(node: JsIdentifierExpression) -> AnyNode { AnyNode::JsIdentifierExpression(node) }
}
impl From<JsSequenceExpression> for AnyNode {
	fn from(node: JsSequenceExpression) -> AnyNode { AnyNode::JsSequenceExpression(node) }
}
impl From<JsStaticMemberExpression> for AnyNode {
	fn from(node: JsStaticMemberExpression) -> AnyNode { AnyNode::JsStaticMemberExpression(node) }
}
impl From<JsSuperExpression> for AnyNode {
	fn from(node: JsSuperExpression) -> AnyNode { AnyNode::JsSuperExpression(node) }
}
impl From<JsThisExpression> for AnyNode {
	fn from(node: JsThisExpression) -> AnyNode { AnyNode::JsThisExpression(node) }
}
impl From<JsUnaryExpression> for AnyNode {
	fn from(node: JsUnaryExpression) -> AnyNode { AnyNode::JsUnaryExpression(node) }
}
impl From<JsPreUpdateExpression> for AnyNode {
	fn from(node: JsPreUpdateExpression) -> AnyNode { AnyNode::JsPreUpdateExpression(node) }
}
impl From<JsPostUpdateExpression> for AnyNode {
	fn from(node: JsPostUpdateExpression) -> AnyNode { AnyNode::JsPostUpdateExpression(node) }
}
impl From<JsYieldExpression> for AnyNode {
	fn from(node: JsYieldExpression) -> AnyNode { AnyNode::JsYieldExpression(node) }
}
impl From<Template> for AnyNode {
	fn from(node: Template) -> AnyNode { AnyNode::Template(node) }
}
impl From<NewExpr> for AnyNode {
	fn from(node: NewExpr) -> AnyNode { AnyNode::NewExpr(node) }
}
impl From<CallExpr> for AnyNode {
	fn from(node: CallExpr) -> AnyNode { AnyNode::CallExpr(node) }
}
impl From<NewTarget> for AnyNode {
	fn from(node: NewTarget) -> AnyNode { AnyNode::NewTarget(node) }
}
impl From<ImportMeta> for AnyNode {
	fn from(node: ImportMeta) -> AnyNode { AnyNode::ImportMeta(node) }
}
impl From<TsNonNull> for AnyNode {
	fn from(node: TsNonNull) -> AnyNode { AnyNode::TsNonNull(node) }
}
impl From<TsAssertion> for AnyNode {
	fn from(node: TsAssertion) -> AnyNode { AnyNode::TsAssertion(node) }
}
impl From<TsConstAssertion> for AnyNode {
	fn from(node: TsConstAssertion) -> AnyNode { AnyNode::TsConstAssertion(node) }
}
impl From<TsTypeArgs> for AnyNode {
	fn from(node: TsTypeArgs) -> AnyNode { AnyNode::TsTypeArgs(node) }
}
impl From<ArgList> for AnyNode {
	fn from(node: ArgList) -> AnyNode { AnyNode::ArgList(node) }
}
impl From<TsTypeParams> for AnyNode {
	fn from(node: TsTypeParams) -> AnyNode { AnyNode::TsTypeParams(node) }
}
impl From<JsParameterList> for AnyNode {
	fn from(node: JsParameterList) -> AnyNode { AnyNode::JsParameterList(node) }
}
impl From<TsTypeAnnotation> for AnyNode {
	fn from(node: TsTypeAnnotation) -> AnyNode { AnyNode::TsTypeAnnotation(node) }
}
impl From<JsFunctionBody> for AnyNode {
	fn from(node: JsFunctionBody) -> AnyNode { AnyNode::JsFunctionBody(node) }
}
impl From<JsSpread> for AnyNode {
	fn from(node: JsSpread) -> AnyNode { AnyNode::JsSpread(node) }
}
impl From<JsArrayHole> for AnyNode {
	fn from(node: JsArrayHole) -> AnyNode { AnyNode::JsArrayHole(node) }
}
impl From<JsReferenceIdentifier> for AnyNode {
	fn from(node: JsReferenceIdentifier) -> AnyNode { AnyNode::JsReferenceIdentifier(node) }
}
impl From<JsLiteralMemberName> for AnyNode {
	fn from(node: JsLiteralMemberName) -> AnyNode { AnyNode::JsLiteralMemberName(node) }
}
impl From<JsComputedMemberName> for AnyNode {
	fn from(node: JsComputedMemberName) -> AnyNode { AnyNode::JsComputedMemberName(node) }
}
impl From<JsPropertyObjectMember> for AnyNode {
	fn from(node: JsPropertyObjectMember) -> AnyNode { AnyNode::JsPropertyObjectMember(node) }
}
impl From<JsMethodObjectMember> for AnyNode {
	fn from(node: JsMethodObjectMember) -> AnyNode { AnyNode::JsMethodObjectMember(node) }
}
impl From<JsGetterObjectMember> for AnyNode {
	fn from(node: JsGetterObjectMember) -> AnyNode { AnyNode::JsGetterObjectMember(node) }
}
impl From<JsSetterObjectMember> for AnyNode {
	fn from(node: JsSetterObjectMember) -> AnyNode { AnyNode::JsSetterObjectMember(node) }
}
impl From<JsShorthandPropertyObjectMember> for AnyNode {
	fn from(node: JsShorthandPropertyObjectMember) -> AnyNode {
		AnyNode::JsShorthandPropertyObjectMember(node)
	}
}
impl From<TsImplementsClause> for AnyNode {
	fn from(node: TsImplementsClause) -> AnyNode { AnyNode::TsImplementsClause(node) }
}
impl From<JsExtendsClause> for AnyNode {
	fn from(node: JsExtendsClause) -> AnyNode { AnyNode::JsExtendsClause(node) }
}
impl From<TsExprWithTypeArgs> for AnyNode {
	fn from(node: TsExprWithTypeArgs) -> AnyNode { AnyNode::TsExprWithTypeArgs(node) }
}
impl From<JsPrivateClassMemberName> for AnyNode {
	fn from(node: JsPrivateClassMemberName) -> AnyNode { AnyNode::JsPrivateClassMemberName(node) }
}
impl From<JsConstructorClassMember> for AnyNode {
	fn from(node: JsConstructorClassMember) -> AnyNode { AnyNode::JsConstructorClassMember(node) }
}
impl From<JsPropertyClassMember> for AnyNode {
	fn from(node: JsPropertyClassMember) -> AnyNode { AnyNode::JsPropertyClassMember(node) }
}
impl From<JsMethodClassMember> for AnyNode {
	fn from(node: JsMethodClassMember) -> AnyNode { AnyNode::JsMethodClassMember(node) }
}
impl From<JsGetterClassMember> for AnyNode {
	fn from(node: JsGetterClassMember) -> AnyNode { AnyNode::JsGetterClassMember(node) }
}
impl From<JsSetterClassMember> for AnyNode {
	fn from(node: JsSetterClassMember) -> AnyNode { AnyNode::JsSetterClassMember(node) }
}
impl From<JsEmptyClassMember> for AnyNode {
	fn from(node: JsEmptyClassMember) -> AnyNode { AnyNode::JsEmptyClassMember(node) }
}
impl From<TsIndexSignature> for AnyNode {
	fn from(node: TsIndexSignature) -> AnyNode { AnyNode::TsIndexSignature(node) }
}
impl From<TsAccessibility> for AnyNode {
	fn from(node: TsAccessibility) -> AnyNode { AnyNode::TsAccessibility(node) }
}
impl From<JsConstructorParameterList> for AnyNode {
	fn from(node: JsConstructorParameterList) -> AnyNode {
		AnyNode::JsConstructorParameterList(node)
	}
}
impl From<TsConstructorParam> for AnyNode {
	fn from(node: TsConstructorParam) -> AnyNode { AnyNode::TsConstructorParam(node) }
}
impl From<JsBindingPatternWithDefault> for AnyNode {
	fn from(node: JsBindingPatternWithDefault) -> AnyNode {
		AnyNode::JsBindingPatternWithDefault(node)
	}
}
impl From<JsEqualValueClause> for AnyNode {
	fn from(node: JsEqualValueClause) -> AnyNode { AnyNode::JsEqualValueClause(node) }
}
impl From<JsModifier> for AnyNode {
	fn from(node: JsModifier) -> AnyNode { AnyNode::JsModifier(node) }
}
impl From<JsIdentifierAssignment> for AnyNode {
	fn from(node: JsIdentifierAssignment) -> AnyNode { AnyNode::JsIdentifierAssignment(node) }
}
impl From<JsStaticMemberAssignment> for AnyNode {
	fn from(node: JsStaticMemberAssignment) -> AnyNode { AnyNode::JsStaticMemberAssignment(node) }
}
impl From<JsComputedMemberAssignment> for AnyNode {
	fn from(node: JsComputedMemberAssignment) -> AnyNode {
		AnyNode::JsComputedMemberAssignment(node)
	}
}
impl From<JsParenthesizedAssignment> for AnyNode {
	fn from(node: JsParenthesizedAssignment) -> AnyNode { AnyNode::JsParenthesizedAssignment(node) }
}
impl From<JsAssignmentWithDefault> for AnyNode {
	fn from(node: JsAssignmentWithDefault) -> AnyNode { AnyNode::JsAssignmentWithDefault(node) }
}
impl From<JsArrayAssignmentPattern> for AnyNode {
	fn from(node: JsArrayAssignmentPattern) -> AnyNode { AnyNode::JsArrayAssignmentPattern(node) }
}
impl From<JsObjectAssignmentPattern> for AnyNode {
	fn from(node: JsObjectAssignmentPattern) -> AnyNode { AnyNode::JsObjectAssignmentPattern(node) }
}
impl From<JsArrayAssignmentPatternRestElement> for AnyNode {
	fn from(node: JsArrayAssignmentPatternRestElement) -> AnyNode {
		AnyNode::JsArrayAssignmentPatternRestElement(node)
	}
}
impl From<JsObjectAssignmentPatternShorthandProperty> for AnyNode {
	fn from(node: JsObjectAssignmentPatternShorthandProperty) -> AnyNode {
		AnyNode::JsObjectAssignmentPatternShorthandProperty(node)
	}
}
impl From<JsObjectAssignmentPatternProperty> for AnyNode {
	fn from(node: JsObjectAssignmentPatternProperty) -> AnyNode {
		AnyNode::JsObjectAssignmentPatternProperty(node)
	}
}
impl From<JsObjectAssignmentPatternRest> for AnyNode {
	fn from(node: JsObjectAssignmentPatternRest) -> AnyNode {
		AnyNode::JsObjectAssignmentPatternRest(node)
	}
}
impl From<JsName> for AnyNode {
	fn from(node: JsName) -> AnyNode { AnyNode::JsName(node) }
}
impl From<JsIdentifierBinding> for AnyNode {
	fn from(node: JsIdentifierBinding) -> AnyNode { AnyNode::JsIdentifierBinding(node) }
}
impl From<JsArrayBindingPattern> for AnyNode {
	fn from(node: JsArrayBindingPattern) -> AnyNode { AnyNode::JsArrayBindingPattern(node) }
}
impl From<JsObjectBindingPattern> for AnyNode {
	fn from(node: JsObjectBindingPattern) -> AnyNode { AnyNode::JsObjectBindingPattern(node) }
}
impl From<JsArrayBindingPatternRestElement> for AnyNode {
	fn from(node: JsArrayBindingPatternRestElement) -> AnyNode {
		AnyNode::JsArrayBindingPatternRestElement(node)
	}
}
impl From<JsObjectBindingPatternProperty> for AnyNode {
	fn from(node: JsObjectBindingPatternProperty) -> AnyNode {
		AnyNode::JsObjectBindingPatternProperty(node)
	}
}
impl From<JsObjectBindingPatternRest> for AnyNode {
	fn from(node: JsObjectBindingPatternRest) -> AnyNode {
		AnyNode::JsObjectBindingPatternRest(node)
	}
}
impl From<JsObjectBindingPatternShorthandProperty> for AnyNode {
	fn from(node: JsObjectBindingPatternShorthandProperty) -> AnyNode {
		AnyNode::JsObjectBindingPatternShorthandProperty(node)
	}
}
impl From<JsStringLiteralExpression> for AnyNode {
	fn from(node: JsStringLiteralExpression) -> AnyNode { AnyNode::JsStringLiteralExpression(node) }
}
impl From<JsNumberLiteralExpression> for AnyNode {
	fn from(node: JsNumberLiteralExpression) -> AnyNode { AnyNode::JsNumberLiteralExpression(node) }
}
impl From<JsBigIntLiteralExpression> for AnyNode {
	fn from(node: JsBigIntLiteralExpression) -> AnyNode { AnyNode::JsBigIntLiteralExpression(node) }
}
impl From<JsBooleanLiteralExpression> for AnyNode {
	fn from(node: JsBooleanLiteralExpression) -> AnyNode {
		AnyNode::JsBooleanLiteralExpression(node)
	}
}
impl From<JsNullLiteralExpression> for AnyNode {
	fn from(node: JsNullLiteralExpression) -> AnyNode { AnyNode::JsNullLiteralExpression(node) }
}
impl From<JsRegexLiteralExpression> for AnyNode {
	fn from(node: JsRegexLiteralExpression) -> AnyNode { AnyNode::JsRegexLiteralExpression(node) }
}
impl From<JsVariableDeclarator> for AnyNode {
	fn from(node: JsVariableDeclarator) -> AnyNode { AnyNode::JsVariableDeclarator(node) }
}
impl From<JsImport> for AnyNode {
	fn from(node: JsImport) -> AnyNode { AnyNode::JsImport(node) }
}
impl From<ExportNamed> for AnyNode {
	fn from(node: ExportNamed) -> AnyNode { AnyNode::ExportNamed(node) }
}
impl From<ExportDefaultDecl> for AnyNode {
	fn from(node: ExportDefaultDecl) -> AnyNode { AnyNode::ExportDefaultDecl(node) }
}
impl From<ExportDefaultExpr> for AnyNode {
	fn from(node: ExportDefaultExpr) -> AnyNode { AnyNode::ExportDefaultExpr(node) }
}
impl From<ExportWildcard> for AnyNode {
	fn from(node: ExportWildcard) -> AnyNode { AnyNode::ExportWildcard(node) }
}
impl From<ExportDecl> for AnyNode {
	fn from(node: ExportDecl) -> AnyNode { AnyNode::ExportDecl(node) }
}
impl From<TsImportEqualsDecl> for AnyNode {
	fn from(node: TsImportEqualsDecl) -> AnyNode { AnyNode::TsImportEqualsDecl(node) }
}
impl From<TsExportAssignment> for AnyNode {
	fn from(node: TsExportAssignment) -> AnyNode { AnyNode::TsExportAssignment(node) }
}
impl From<TsNamespaceExportDecl> for AnyNode {
	fn from(node: TsNamespaceExportDecl) -> AnyNode { AnyNode::TsNamespaceExportDecl(node) }
}
impl From<JsImportBareClause> for AnyNode {
	fn from(node: JsImportBareClause) -> AnyNode { AnyNode::JsImportBareClause(node) }
}
impl From<JsImportNamedClause> for AnyNode {
	fn from(node: JsImportNamedClause) -> AnyNode { AnyNode::JsImportNamedClause(node) }
}
impl From<JsImportDefaultClause> for AnyNode {
	fn from(node: JsImportDefaultClause) -> AnyNode { AnyNode::JsImportDefaultClause(node) }
}
impl From<JsImportNamespaceClause> for AnyNode {
	fn from(node: JsImportNamespaceClause) -> AnyNode { AnyNode::JsImportNamespaceClause(node) }
}
impl From<JsModuleSource> for AnyNode {
	fn from(node: JsModuleSource) -> AnyNode { AnyNode::JsModuleSource(node) }
}
impl From<JsImportAssertion> for AnyNode {
	fn from(node: JsImportAssertion) -> AnyNode { AnyNode::JsImportAssertion(node) }
}
impl From<JsDefaultImportSpecifier> for AnyNode {
	fn from(node: JsDefaultImportSpecifier) -> AnyNode { AnyNode::JsDefaultImportSpecifier(node) }
}
impl From<JsNamedImportSpecifierList> for AnyNode {
	fn from(node: JsNamedImportSpecifierList) -> AnyNode {
		AnyNode::JsNamedImportSpecifierList(node)
	}
}
impl From<JsNamespaceImportSpecifier> for AnyNode {
	fn from(node: JsNamespaceImportSpecifier) -> AnyNode {
		AnyNode::JsNamespaceImportSpecifier(node)
	}
}
impl From<JsShorthandNamedImportSpecifier> for AnyNode {
	fn from(node: JsShorthandNamedImportSpecifier) -> AnyNode {
		AnyNode::JsShorthandNamedImportSpecifier(node)
	}
}
impl From<JsNamedImportSpecifier> for AnyNode {
	fn from(node: JsNamedImportSpecifier) -> AnyNode { AnyNode::JsNamedImportSpecifier(node) }
}
impl From<JsLiteralExportName> for AnyNode {
	fn from(node: JsLiteralExportName) -> AnyNode { AnyNode::JsLiteralExportName(node) }
}
impl From<JsImportAssertionEntry> for AnyNode {
	fn from(node: JsImportAssertionEntry) -> AnyNode { AnyNode::JsImportAssertionEntry(node) }
}
impl From<Specifier> for AnyNode {
	fn from(node: Specifier) -> AnyNode { AnyNode::Specifier(node) }
}
impl From<JsPrivateName> for AnyNode {
	fn from(node: JsPrivateName) -> AnyNode { AnyNode::JsPrivateName(node) }
}
impl From<JsRestParameter> for AnyNode {
	fn from(node: JsRestParameter) -> AnyNode { AnyNode::JsRestParameter(node) }
}
impl From<TsExternalModuleRef> for AnyNode {
	fn from(node: TsExternalModuleRef) -> AnyNode { AnyNode::TsExternalModuleRef(node) }
}
impl From<TsAny> for AnyNode {
	fn from(node: TsAny) -> AnyNode { AnyNode::TsAny(node) }
}
impl From<TsUnknown> for AnyNode {
	fn from(node: TsUnknown) -> AnyNode { AnyNode::TsUnknown(node) }
}
impl From<TsNumber> for AnyNode {
	fn from(node: TsNumber) -> AnyNode { AnyNode::TsNumber(node) }
}
impl From<TsObject> for AnyNode {
	fn from(node: TsObject) -> AnyNode { AnyNode::TsObject(node) }
}
impl From<TsBoolean> for AnyNode {
	fn from(node: TsBoolean) -> AnyNode { AnyNode::TsBoolean(node) }
}
impl From<TsBigint> for AnyNode {
	fn from(node: TsBigint) -> AnyNode { AnyNode::TsBigint(node) }
}
impl From<TsString> for AnyNode {
	fn from(node: TsString) -> AnyNode { AnyNode::TsString(node) }
}
impl From<TsSymbol> for AnyNode {
	fn from(node: TsSymbol) -> AnyNode { AnyNode::TsSymbol(node) }
}
impl From<TsVoid> for AnyNode {
	fn from(node: TsVoid) -> AnyNode { AnyNode::TsVoid(node) }
}
impl From<TsUndefined> for AnyNode {
	fn from(node: TsUndefined) -> AnyNode { AnyNode::TsUndefined(node) }
}
impl From<TsNull> for AnyNode {
	fn from(node: TsNull) -> AnyNode { AnyNode::TsNull(node) }
}
impl From<TsNever> for AnyNode {
	fn from(node: TsNever) -> AnyNode { AnyNode::TsNever(node) }
}
impl From<TsThis> for AnyNode {
	fn from(node: TsThis) -> AnyNode { AnyNode::TsThis(node) }
}
impl From<TsLiteral> for AnyNode {
	fn from(node: TsLiteral) -> AnyNode { AnyNode::TsLiteral(node) }
}
impl From<TsPredicate> for AnyNode {
	fn from(node: TsPredicate) -> AnyNode { AnyNode::TsPredicate(node) }
}
impl From<TsTuple> for AnyNode {
	fn from(node: TsTuple) -> AnyNode { AnyNode::TsTuple(node) }
}
impl From<TsParen> for AnyNode {
	fn from(node: TsParen) -> AnyNode { AnyNode::TsParen(node) }
}
impl From<TsTypeRef> for AnyNode {
	fn from(node: TsTypeRef) -> AnyNode { AnyNode::TsTypeRef(node) }
}
impl From<TsTemplate> for AnyNode {
	fn from(node: TsTemplate) -> AnyNode { AnyNode::TsTemplate(node) }
}
impl From<TsMappedType> for AnyNode {
	fn from(node: TsMappedType) -> AnyNode { AnyNode::TsMappedType(node) }
}
impl From<TsImport> for AnyNode {
	fn from(node: TsImport) -> AnyNode { AnyNode::TsImport(node) }
}
impl From<TsArray> for AnyNode {
	fn from(node: TsArray) -> AnyNode { AnyNode::TsArray(node) }
}
impl From<TsIndexedArray> for AnyNode {
	fn from(node: TsIndexedArray) -> AnyNode { AnyNode::TsIndexedArray(node) }
}
impl From<TsTypeOperator> for AnyNode {
	fn from(node: TsTypeOperator) -> AnyNode { AnyNode::TsTypeOperator(node) }
}
impl From<TsIntersection> for AnyNode {
	fn from(node: TsIntersection) -> AnyNode { AnyNode::TsIntersection(node) }
}
impl From<TsUnion> for AnyNode {
	fn from(node: TsUnion) -> AnyNode { AnyNode::TsUnion(node) }
}
impl From<TsFnType> for AnyNode {
	fn from(node: TsFnType) -> AnyNode { AnyNode::TsFnType(node) }
}
impl From<TsConstructorType> for AnyNode {
	fn from(node: TsConstructorType) -> AnyNode { AnyNode::TsConstructorType(node) }
}
impl From<TsConditionalType> for AnyNode {
	fn from(node: TsConditionalType) -> AnyNode { AnyNode::TsConditionalType(node) }
}
impl From<TsObjectType> for AnyNode {
	fn from(node: TsObjectType) -> AnyNode { AnyNode::TsObjectType(node) }
}
impl From<TsInfer> for AnyNode {
	fn from(node: TsInfer) -> AnyNode { AnyNode::TsInfer(node) }
}
impl From<TsTupleElement> for AnyNode {
	fn from(node: TsTupleElement) -> AnyNode { AnyNode::TsTupleElement(node) }
}
impl From<TsEnumMember> for AnyNode {
	fn from(node: TsEnumMember) -> AnyNode { AnyNode::TsEnumMember(node) }
}
impl From<TsTemplateElement> for AnyNode {
	fn from(node: TsTemplateElement) -> AnyNode { AnyNode::TsTemplateElement(node) }
}
impl From<TsMappedTypeReadonly> for AnyNode {
	fn from(node: TsMappedTypeReadonly) -> AnyNode { AnyNode::TsMappedTypeReadonly(node) }
}
impl From<TsMappedTypeParam> for AnyNode {
	fn from(node: TsMappedTypeParam) -> AnyNode { AnyNode::TsMappedTypeParam(node) }
}
impl From<TsTypeName> for AnyNode {
	fn from(node: TsTypeName) -> AnyNode { AnyNode::TsTypeName(node) }
}
impl From<TsExtends> for AnyNode {
	fn from(node: TsExtends) -> AnyNode { AnyNode::TsExtends(node) }
}
impl From<TsModuleBlock> for AnyNode {
	fn from(node: TsModuleBlock) -> AnyNode { AnyNode::TsModuleBlock(node) }
}
impl From<TsTypeParam> for AnyNode {
	fn from(node: TsTypeParam) -> AnyNode { AnyNode::TsTypeParam(node) }
}
impl From<TsConstraint> for AnyNode {
	fn from(node: TsConstraint) -> AnyNode { AnyNode::TsConstraint(node) }
}
impl From<TsDefault> for AnyNode {
	fn from(node: TsDefault) -> AnyNode { AnyNode::TsDefault(node) }
}
impl From<TsCallSignatureDecl> for AnyNode {
	fn from(node: TsCallSignatureDecl) -> AnyNode { AnyNode::TsCallSignatureDecl(node) }
}
impl From<TsConstructSignatureDecl> for AnyNode {
	fn from(node: TsConstructSignatureDecl) -> AnyNode { AnyNode::TsConstructSignatureDecl(node) }
}
impl From<TsPropertySignature> for AnyNode {
	fn from(node: TsPropertySignature) -> AnyNode { AnyNode::TsPropertySignature(node) }
}
impl From<TsMethodSignature> for AnyNode {
	fn from(node: TsMethodSignature) -> AnyNode { AnyNode::TsMethodSignature(node) }
}
impl From<TsQualifiedPath> for AnyNode {
	fn from(node: TsQualifiedPath) -> AnyNode { AnyNode::TsQualifiedPath(node) }
}
impl AstNode for AnyNode {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(
			kind,
			JS_UNKNOWN_STATEMENT
				| JS_UNKNOWN_EXPRESSION
				| JS_UNKNOWN_MEMBER
				| JS_UNKNOWN_BINDING
				| JS_UNKNOWN_ASSIGNMENT
				| JS_UNKNOWN_MODIFIER
				| JS_UNKNOWN_IMPORT_ASSERTION_ENTRY
				| JS_UNKNOWN_NAMED_IMPORT_SPECIFIER
				| LIST | IDENT | JS_SCRIPT
				| JS_MODULE | JS_DIRECTIVE
				| JS_BLOCK_STATEMENT
				| JS_EMPTY_STATEMENT
				| JS_EXPRESSION_STATEMENT
				| JS_IF_STATEMENT
				| JS_DO_WHILE_STATEMENT
				| JS_WHILE_STATEMENT
				| FOR_STMT | FOR_IN_STMT
				| FOR_OF_STMT | JS_CONTINUE_STATEMENT
				| JS_BREAK_STATEMENT
				| JS_RETURN_STATEMENT
				| JS_WITH_STATEMENT
				| JS_LABELED_STATEMENT
				| JS_SWITCH_STATEMENT
				| JS_THROW_STATEMENT
				| JS_TRY_STATEMENT
				| JS_TRY_FINALLY_STATEMENT
				| JS_DEBUGGER_STATEMENT
				| JS_FUNCTION_DECLARATION
				| JS_CLASS_DECLARATION
				| JS_VARIABLE_DECLARATION_STATEMENT
				| TS_ENUM | TS_TYPE_ALIAS_DECL
				| TS_NAMESPACE_DECL
				| TS_MODULE_DECL | TS_INTERFACE_DECL
				| JS_ELSE_CLAUSE | FOR_STMT_INIT
				| FOR_STMT_TEST | FOR_STMT_UPDATE
				| JS_VARIABLE_DECLARATION
				| JS_CASE_CLAUSE | JS_DEFAULT_CLAUSE
				| JS_CATCH_CLAUSE
				| JS_FINALLY_CLAUSE
				| JS_CATCH_DECLARATION
				| JS_ARRAY_EXPRESSION
				| JS_ARROW_FUNCTION_EXPRESSION
				| JS_ASSIGNMENT_EXPRESSION
				| JS_AWAIT_EXPRESSION
				| JS_BINARY_EXPRESSION
				| JS_CLASS_EXPRESSION
				| JS_CONDITIONAL_EXPRESSION
				| JS_COMPUTED_MEMBER_EXPRESSION
				| JS_FUNCTION_EXPRESSION
				| JS_IMPORT_CALL_EXPRESSION
				| JS_LOGICAL_EXPRESSION
				| JS_OBJECT_EXPRESSION
				| JS_PARENTHESIZED_EXPRESSION
				| JS_IDENTIFIER_EXPRESSION
				| JS_SEQUENCE_EXPRESSION
				| JS_STATIC_MEMBER_EXPRESSION
				| JS_SUPER_EXPRESSION
				| JS_THIS_EXPRESSION
				| JS_UNARY_EXPRESSION
				| JS_PRE_UPDATE_EXPRESSION
				| JS_POST_UPDATE_EXPRESSION
				| JS_YIELD_EXPRESSION
				| TEMPLATE | NEW_EXPR
				| CALL_EXPR | NEW_TARGET
				| IMPORT_META | TS_NON_NULL
				| TS_ASSERTION | TS_CONST_ASSERTION
				| TS_TYPE_ARGS | ARG_LIST
				| TS_TYPE_PARAMS | JS_PARAMETER_LIST
				| TS_TYPE_ANNOTATION
				| JS_FUNCTION_BODY
				| JS_SPREAD | JS_ARRAY_HOLE
				| JS_REFERENCE_IDENTIFIER
				| JS_LITERAL_MEMBER_NAME
				| JS_COMPUTED_MEMBER_NAME
				| JS_PROPERTY_OBJECT_MEMBER
				| JS_METHOD_OBJECT_MEMBER
				| JS_GETTER_OBJECT_MEMBER
				| JS_SETTER_OBJECT_MEMBER
				| JS_SHORTHAND_PROPERTY_OBJECT_MEMBER
				| TS_IMPLEMENTS_CLAUSE
				| JS_EXTENDS_CLAUSE
				| TS_EXPR_WITH_TYPE_ARGS
				| JS_PRIVATE_CLASS_MEMBER_NAME
				| JS_CONSTRUCTOR_CLASS_MEMBER
				| JS_PROPERTY_CLASS_MEMBER
				| JS_METHOD_CLASS_MEMBER
				| JS_GETTER_CLASS_MEMBER
				| JS_SETTER_CLASS_MEMBER
				| JS_EMPTY_CLASS_MEMBER
				| TS_INDEX_SIGNATURE
				| TS_ACCESSIBILITY
				| JS_CONSTRUCTOR_PARAMETER_LIST
				| TS_CONSTRUCTOR_PARAM
				| JS_BINDING_PATTERN_WITH_DEFAULT
				| JS_EQUAL_VALUE_CLAUSE
				| JS_MODIFIER | JS_IDENTIFIER_ASSIGNMENT
				| JS_STATIC_MEMBER_ASSIGNMENT
				| JS_COMPUTED_MEMBER_ASSIGNMENT
				| JS_PARENTHESIZED_ASSIGNMENT
				| JS_ASSIGNMENT_WITH_DEFAULT
				| JS_ARRAY_ASSIGNMENT_PATTERN
				| JS_OBJECT_ASSIGNMENT_PATTERN
				| JS_ARRAY_ASSIGNMENT_PATTERN_REST_ELEMENT
				| JS_OBJECT_ASSIGNMENT_PATTERN_SHORTHAND_PROPERTY
				| JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY
				| JS_OBJECT_ASSIGNMENT_PATTERN_REST
				| JS_NAME | JS_IDENTIFIER_BINDING
				| JS_ARRAY_BINDING_PATTERN
				| JS_OBJECT_BINDING_PATTERN
				| JS_ARRAY_BINDING_PATTERN_REST_ELEMENT
				| JS_OBJECT_BINDING_PATTERN_PROPERTY
				| JS_OBJECT_BINDING_PATTERN_REST
				| JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY
				| JS_STRING_LITERAL_EXPRESSION
				| JS_NUMBER_LITERAL_EXPRESSION
				| JS_BIG_INT_LITERAL_EXPRESSION
				| JS_BOOLEAN_LITERAL_EXPRESSION
				| JS_NULL_LITERAL_EXPRESSION
				| JS_REGEX_LITERAL_EXPRESSION
				| JS_VARIABLE_DECLARATOR
				| JS_IMPORT | EXPORT_NAMED
				| EXPORT_DEFAULT_DECL
				| EXPORT_DEFAULT_EXPR
				| EXPORT_WILDCARD
				| EXPORT_DECL | TS_IMPORT_EQUALS_DECL
				| TS_EXPORT_ASSIGNMENT
				| TS_NAMESPACE_EXPORT_DECL
				| JS_IMPORT_BARE_CLAUSE
				| JS_IMPORT_NAMED_CLAUSE
				| JS_IMPORT_DEFAULT_CLAUSE
				| JS_IMPORT_NAMESPACE_CLAUSE
				| JS_MODULE_SOURCE
				| JS_IMPORT_ASSERTION
				| JS_DEFAULT_IMPORT_SPECIFIER
				| JS_NAMED_IMPORT_SPECIFIER_LIST
				| JS_NAMESPACE_IMPORT_SPECIFIER
				| JS_SHORTHAND_NAMED_IMPORT_SPECIFIER
				| JS_NAMED_IMPORT_SPECIFIER
				| JS_LITERAL_EXPORT_NAME
				| JS_IMPORT_ASSERTION_ENTRY
				| SPECIFIER | JS_PRIVATE_NAME
				| JS_REST_PARAMETER
				| TS_EXTERNAL_MODULE_REF
				| TS_ANY | TS_UNKNOWN
				| TS_NUMBER | TS_OBJECT
				| TS_BOOLEAN | TS_BIGINT
				| TS_STRING | TS_SYMBOL
				| TS_VOID | TS_UNDEFINED
				| TS_NULL | TS_NEVER
				| TS_THIS | TS_LITERAL
				| TS_PREDICATE | TS_TUPLE
				| TS_PAREN | TS_TYPE_REF
				| TS_TEMPLATE | TS_MAPPED_TYPE
				| TS_IMPORT | TS_ARRAY
				| TS_INDEXED_ARRAY
				| TS_TYPE_OPERATOR
				| TS_INTERSECTION
				| TS_UNION | TS_FN_TYPE
				| TS_CONSTRUCTOR_TYPE
				| TS_CONDITIONAL_TYPE
				| TS_OBJECT_TYPE | TS_INFER
				| TS_TUPLE_ELEMENT
				| TS_ENUM_MEMBER | TS_TEMPLATE_ELEMENT
				| TS_MAPPED_TYPE_READONLY
				| TS_MAPPED_TYPE_PARAM
				| TS_TYPE_NAME | TS_EXTENDS
				| TS_MODULE_BLOCK
				| TS_TYPE_PARAM | TS_CONSTRAINT
				| TS_DEFAULT | TS_CALL_SIGNATURE_DECL
				| TS_CONSTRUCT_SIGNATURE_DECL
				| TS_PROPERTY_SIGNATURE
				| TS_METHOD_SIGNATURE
				| TS_QUALIFIED_PATH
		)
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_UNKNOWN_STATEMENT => AnyNode::JsUnknownStatement(JsUnknownStatement { syntax }),
			JS_UNKNOWN_EXPRESSION => AnyNode::JsUnknownExpression(JsUnknownExpression { syntax }),
			JS_UNKNOWN_MEMBER => AnyNode::JsUnknownMember(JsUnknownMember { syntax }),
			JS_UNKNOWN_BINDING => AnyNode::JsUnknownBinding(JsUnknownBinding { syntax }),
			JS_UNKNOWN_ASSIGNMENT => AnyNode::JsUnknownAssignment(JsUnknownAssignment { syntax }),
			JS_UNKNOWN_MODIFIER => AnyNode::JsUnknownModifier(JsUnknownModifier { syntax }),
			JS_UNKNOWN_IMPORT_ASSERTION_ENTRY => {
				AnyNode::JsUnknownImportAssertionEntry(JsUnknownImportAssertionEntry { syntax })
			}
			JS_UNKNOWN_NAMED_IMPORT_SPECIFIER => {
				AnyNode::JsUnknownNamedImportSpecifier(JsUnknownNamedImportSpecifier { syntax })
			}
			LIST => AnyNode::List(List { syntax }),
			IDENT => AnyNode::Ident(Ident { syntax }),
			JS_SCRIPT => AnyNode::JsScript(JsScript { syntax }),
			JS_MODULE => AnyNode::JsModule(JsModule { syntax }),
			JS_DIRECTIVE => AnyNode::JsDirective(JsDirective { syntax }),
			JS_BLOCK_STATEMENT => AnyNode::JsBlockStatement(JsBlockStatement { syntax }),
			JS_EMPTY_STATEMENT => AnyNode::JsEmptyStatement(JsEmptyStatement { syntax }),
			JS_EXPRESSION_STATEMENT => {
				AnyNode::JsExpressionStatement(JsExpressionStatement { syntax })
			}
			JS_IF_STATEMENT => AnyNode::JsIfStatement(JsIfStatement { syntax }),
			JS_DO_WHILE_STATEMENT => AnyNode::JsDoWhileStatement(JsDoWhileStatement { syntax }),
			JS_WHILE_STATEMENT => AnyNode::JsWhileStatement(JsWhileStatement { syntax }),
			FOR_STMT => AnyNode::ForStmt(ForStmt { syntax }),
			FOR_IN_STMT => AnyNode::ForInStmt(ForInStmt { syntax }),
			FOR_OF_STMT => AnyNode::ForOfStmt(ForOfStmt { syntax }),
			JS_CONTINUE_STATEMENT => AnyNode::JsContinueStatement(JsContinueStatement { syntax }),
			JS_BREAK_STATEMENT => AnyNode::JsBreakStatement(JsBreakStatement { syntax }),
			JS_RETURN_STATEMENT => AnyNode::JsReturnStatement(JsReturnStatement { syntax }),
			JS_WITH_STATEMENT => AnyNode::JsWithStatement(JsWithStatement { syntax }),
			JS_LABELED_STATEMENT => AnyNode::JsLabeledStatement(JsLabeledStatement { syntax }),
			JS_SWITCH_STATEMENT => AnyNode::JsSwitchStatement(JsSwitchStatement { syntax }),
			JS_THROW_STATEMENT => AnyNode::JsThrowStatement(JsThrowStatement { syntax }),
			JS_TRY_STATEMENT => AnyNode::JsTryStatement(JsTryStatement { syntax }),
			JS_TRY_FINALLY_STATEMENT => {
				AnyNode::JsTryFinallyStatement(JsTryFinallyStatement { syntax })
			}
			JS_DEBUGGER_STATEMENT => AnyNode::JsDebuggerStatement(JsDebuggerStatement { syntax }),
			JS_FUNCTION_DECLARATION => {
				AnyNode::JsFunctionDeclaration(JsFunctionDeclaration { syntax })
			}
			JS_CLASS_DECLARATION => AnyNode::JsClassDeclaration(JsClassDeclaration { syntax }),
			JS_VARIABLE_DECLARATION_STATEMENT => {
				AnyNode::JsVariableDeclarationStatement(JsVariableDeclarationStatement { syntax })
			}
			TS_ENUM => AnyNode::TsEnum(TsEnum { syntax }),
			TS_TYPE_ALIAS_DECL => AnyNode::TsTypeAliasDecl(TsTypeAliasDecl { syntax }),
			TS_NAMESPACE_DECL => AnyNode::TsNamespaceDecl(TsNamespaceDecl { syntax }),
			TS_MODULE_DECL => AnyNode::TsModuleDecl(TsModuleDecl { syntax }),
			TS_INTERFACE_DECL => AnyNode::TsInterfaceDecl(TsInterfaceDecl { syntax }),
			JS_ELSE_CLAUSE => AnyNode::JsElseClause(JsElseClause { syntax }),
			FOR_STMT_INIT => AnyNode::ForStmtInit(ForStmtInit { syntax }),
			FOR_STMT_TEST => AnyNode::ForStmtTest(ForStmtTest { syntax }),
			FOR_STMT_UPDATE => AnyNode::ForStmtUpdate(ForStmtUpdate { syntax }),
			JS_VARIABLE_DECLARATION => {
				AnyNode::JsVariableDeclaration(JsVariableDeclaration { syntax })
			}
			JS_CASE_CLAUSE => AnyNode::JsCaseClause(JsCaseClause { syntax }),
			JS_DEFAULT_CLAUSE => AnyNode::JsDefaultClause(JsDefaultClause { syntax }),
			JS_CATCH_CLAUSE => AnyNode::JsCatchClause(JsCatchClause { syntax }),
			JS_FINALLY_CLAUSE => AnyNode::JsFinallyClause(JsFinallyClause { syntax }),
			JS_CATCH_DECLARATION => AnyNode::JsCatchDeclaration(JsCatchDeclaration { syntax }),
			JS_ARRAY_EXPRESSION => AnyNode::JsArrayExpression(JsArrayExpression { syntax }),
			JS_ARROW_FUNCTION_EXPRESSION => {
				AnyNode::JsArrowFunctionExpression(JsArrowFunctionExpression { syntax })
			}
			JS_ASSIGNMENT_EXPRESSION => {
				AnyNode::JsAssignmentExpression(JsAssignmentExpression { syntax })
			}
			JS_AWAIT_EXPRESSION => AnyNode::JsAwaitExpression(JsAwaitExpression { syntax }),
			JS_BINARY_EXPRESSION => AnyNode::JsBinaryExpression(JsBinaryExpression { syntax }),
			JS_CLASS_EXPRESSION => AnyNode::JsClassExpression(JsClassExpression { syntax }),
			JS_CONDITIONAL_EXPRESSION => {
				AnyNode::JsConditionalExpression(JsConditionalExpression { syntax })
			}
			JS_COMPUTED_MEMBER_EXPRESSION => {
				AnyNode::JsComputedMemberExpression(JsComputedMemberExpression { syntax })
			}
			JS_FUNCTION_EXPRESSION => {
				AnyNode::JsFunctionExpression(JsFunctionExpression { syntax })
			}
			JS_IMPORT_CALL_EXPRESSION => {
				AnyNode::JsImportCallExpression(JsImportCallExpression { syntax })
			}
			JS_LOGICAL_EXPRESSION => AnyNode::JsLogicalExpression(JsLogicalExpression { syntax }),
			JS_OBJECT_EXPRESSION => AnyNode::JsObjectExpression(JsObjectExpression { syntax }),
			JS_PARENTHESIZED_EXPRESSION => {
				AnyNode::JsParenthesizedExpression(JsParenthesizedExpression { syntax })
			}
			JS_IDENTIFIER_EXPRESSION => {
				AnyNode::JsIdentifierExpression(JsIdentifierExpression { syntax })
			}
			JS_SEQUENCE_EXPRESSION => {
				AnyNode::JsSequenceExpression(JsSequenceExpression { syntax })
			}
			JS_STATIC_MEMBER_EXPRESSION => {
				AnyNode::JsStaticMemberExpression(JsStaticMemberExpression { syntax })
			}
			JS_SUPER_EXPRESSION => AnyNode::JsSuperExpression(JsSuperExpression { syntax }),
			JS_THIS_EXPRESSION => AnyNode::JsThisExpression(JsThisExpression { syntax }),
			JS_UNARY_EXPRESSION => AnyNode::JsUnaryExpression(JsUnaryExpression { syntax }),
			JS_PRE_UPDATE_EXPRESSION => {
				AnyNode::JsPreUpdateExpression(JsPreUpdateExpression { syntax })
			}
			JS_POST_UPDATE_EXPRESSION => {
				AnyNode::JsPostUpdateExpression(JsPostUpdateExpression { syntax })
			}
			JS_YIELD_EXPRESSION => AnyNode::JsYieldExpression(JsYieldExpression { syntax }),
			TEMPLATE => AnyNode::Template(Template { syntax }),
			NEW_EXPR => AnyNode::NewExpr(NewExpr { syntax }),
			CALL_EXPR => AnyNode::CallExpr(CallExpr { syntax }),
			NEW_TARGET => AnyNode::NewTarget(NewTarget { syntax }),
			IMPORT_META => AnyNode::ImportMeta(ImportMeta { syntax }),
			TS_NON_NULL => AnyNode::TsNonNull(TsNonNull { syntax }),
			TS_ASSERTION => AnyNode::TsAssertion(TsAssertion { syntax }),
			TS_CONST_ASSERTION => AnyNode::TsConstAssertion(TsConstAssertion { syntax }),
			TS_TYPE_ARGS => AnyNode::TsTypeArgs(TsTypeArgs { syntax }),
			ARG_LIST => AnyNode::ArgList(ArgList { syntax }),
			TS_TYPE_PARAMS => AnyNode::TsTypeParams(TsTypeParams { syntax }),
			JS_PARAMETER_LIST => AnyNode::JsParameterList(JsParameterList { syntax }),
			TS_TYPE_ANNOTATION => AnyNode::TsTypeAnnotation(TsTypeAnnotation { syntax }),
			JS_FUNCTION_BODY => AnyNode::JsFunctionBody(JsFunctionBody { syntax }),
			JS_SPREAD => AnyNode::JsSpread(JsSpread { syntax }),
			JS_ARRAY_HOLE => AnyNode::JsArrayHole(JsArrayHole { syntax }),
			JS_REFERENCE_IDENTIFIER => {
				AnyNode::JsReferenceIdentifier(JsReferenceIdentifier { syntax })
			}
			JS_LITERAL_MEMBER_NAME => AnyNode::JsLiteralMemberName(JsLiteralMemberName { syntax }),
			JS_COMPUTED_MEMBER_NAME => {
				AnyNode::JsComputedMemberName(JsComputedMemberName { syntax })
			}
			JS_PROPERTY_OBJECT_MEMBER => {
				AnyNode::JsPropertyObjectMember(JsPropertyObjectMember { syntax })
			}
			JS_METHOD_OBJECT_MEMBER => {
				AnyNode::JsMethodObjectMember(JsMethodObjectMember { syntax })
			}
			JS_GETTER_OBJECT_MEMBER => {
				AnyNode::JsGetterObjectMember(JsGetterObjectMember { syntax })
			}
			JS_SETTER_OBJECT_MEMBER => {
				AnyNode::JsSetterObjectMember(JsSetterObjectMember { syntax })
			}
			JS_SHORTHAND_PROPERTY_OBJECT_MEMBER => {
				AnyNode::JsShorthandPropertyObjectMember(JsShorthandPropertyObjectMember { syntax })
			}
			TS_IMPLEMENTS_CLAUSE => AnyNode::TsImplementsClause(TsImplementsClause { syntax }),
			JS_EXTENDS_CLAUSE => AnyNode::JsExtendsClause(JsExtendsClause { syntax }),
			TS_EXPR_WITH_TYPE_ARGS => AnyNode::TsExprWithTypeArgs(TsExprWithTypeArgs { syntax }),
			JS_PRIVATE_CLASS_MEMBER_NAME => {
				AnyNode::JsPrivateClassMemberName(JsPrivateClassMemberName { syntax })
			}
			JS_CONSTRUCTOR_CLASS_MEMBER => {
				AnyNode::JsConstructorClassMember(JsConstructorClassMember { syntax })
			}
			JS_PROPERTY_CLASS_MEMBER => {
				AnyNode::JsPropertyClassMember(JsPropertyClassMember { syntax })
			}
			JS_METHOD_CLASS_MEMBER => AnyNode::JsMethodClassMember(JsMethodClassMember { syntax }),
			JS_GETTER_CLASS_MEMBER => AnyNode::JsGetterClassMember(JsGetterClassMember { syntax }),
			JS_SETTER_CLASS_MEMBER => AnyNode::JsSetterClassMember(JsSetterClassMember { syntax }),
			JS_EMPTY_CLASS_MEMBER => AnyNode::JsEmptyClassMember(JsEmptyClassMember { syntax }),
			TS_INDEX_SIGNATURE => AnyNode::TsIndexSignature(TsIndexSignature { syntax }),
			TS_ACCESSIBILITY => AnyNode::TsAccessibility(TsAccessibility { syntax }),
			JS_CONSTRUCTOR_PARAMETER_LIST => {
				AnyNode::JsConstructorParameterList(JsConstructorParameterList { syntax })
			}
			TS_CONSTRUCTOR_PARAM => AnyNode::TsConstructorParam(TsConstructorParam { syntax }),
			JS_BINDING_PATTERN_WITH_DEFAULT => {
				AnyNode::JsBindingPatternWithDefault(JsBindingPatternWithDefault { syntax })
			}
			JS_EQUAL_VALUE_CLAUSE => AnyNode::JsEqualValueClause(JsEqualValueClause { syntax }),
			JS_MODIFIER => AnyNode::JsModifier(JsModifier { syntax }),
			JS_IDENTIFIER_ASSIGNMENT => {
				AnyNode::JsIdentifierAssignment(JsIdentifierAssignment { syntax })
			}
			JS_STATIC_MEMBER_ASSIGNMENT => {
				AnyNode::JsStaticMemberAssignment(JsStaticMemberAssignment { syntax })
			}
			JS_COMPUTED_MEMBER_ASSIGNMENT => {
				AnyNode::JsComputedMemberAssignment(JsComputedMemberAssignment { syntax })
			}
			JS_PARENTHESIZED_ASSIGNMENT => {
				AnyNode::JsParenthesizedAssignment(JsParenthesizedAssignment { syntax })
			}
			JS_ASSIGNMENT_WITH_DEFAULT => {
				AnyNode::JsAssignmentWithDefault(JsAssignmentWithDefault { syntax })
			}
			JS_ARRAY_ASSIGNMENT_PATTERN => {
				AnyNode::JsArrayAssignmentPattern(JsArrayAssignmentPattern { syntax })
			}
			JS_OBJECT_ASSIGNMENT_PATTERN => {
				AnyNode::JsObjectAssignmentPattern(JsObjectAssignmentPattern { syntax })
			}
			JS_ARRAY_ASSIGNMENT_PATTERN_REST_ELEMENT => {
				AnyNode::JsArrayAssignmentPatternRestElement(JsArrayAssignmentPatternRestElement {
					syntax,
				})
			}
			JS_OBJECT_ASSIGNMENT_PATTERN_SHORTHAND_PROPERTY => {
				AnyNode::JsObjectAssignmentPatternShorthandProperty(
					JsObjectAssignmentPatternShorthandProperty { syntax },
				)
			}
			JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY => {
				AnyNode::JsObjectAssignmentPatternProperty(JsObjectAssignmentPatternProperty {
					syntax,
				})
			}
			JS_OBJECT_ASSIGNMENT_PATTERN_REST => {
				AnyNode::JsObjectAssignmentPatternRest(JsObjectAssignmentPatternRest { syntax })
			}
			JS_NAME => AnyNode::JsName(JsName { syntax }),
			JS_IDENTIFIER_BINDING => AnyNode::JsIdentifierBinding(JsIdentifierBinding { syntax }),
			JS_ARRAY_BINDING_PATTERN => {
				AnyNode::JsArrayBindingPattern(JsArrayBindingPattern { syntax })
			}
			JS_OBJECT_BINDING_PATTERN => {
				AnyNode::JsObjectBindingPattern(JsObjectBindingPattern { syntax })
			}
			JS_ARRAY_BINDING_PATTERN_REST_ELEMENT => {
				AnyNode::JsArrayBindingPatternRestElement(JsArrayBindingPatternRestElement {
					syntax,
				})
			}
			JS_OBJECT_BINDING_PATTERN_PROPERTY => {
				AnyNode::JsObjectBindingPatternProperty(JsObjectBindingPatternProperty { syntax })
			}
			JS_OBJECT_BINDING_PATTERN_REST => {
				AnyNode::JsObjectBindingPatternRest(JsObjectBindingPatternRest { syntax })
			}
			JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY => {
				AnyNode::JsObjectBindingPatternShorthandProperty(
					JsObjectBindingPatternShorthandProperty { syntax },
				)
			}
			JS_STRING_LITERAL_EXPRESSION => {
				AnyNode::JsStringLiteralExpression(JsStringLiteralExpression { syntax })
			}
			JS_NUMBER_LITERAL_EXPRESSION => {
				AnyNode::JsNumberLiteralExpression(JsNumberLiteralExpression { syntax })
			}
			JS_BIG_INT_LITERAL_EXPRESSION => {
				AnyNode::JsBigIntLiteralExpression(JsBigIntLiteralExpression { syntax })
			}
			JS_BOOLEAN_LITERAL_EXPRESSION => {
				AnyNode::JsBooleanLiteralExpression(JsBooleanLiteralExpression { syntax })
			}
			JS_NULL_LITERAL_EXPRESSION => {
				AnyNode::JsNullLiteralExpression(JsNullLiteralExpression { syntax })
			}
			JS_REGEX_LITERAL_EXPRESSION => {
				AnyNode::JsRegexLiteralExpression(JsRegexLiteralExpression { syntax })
			}
			JS_VARIABLE_DECLARATOR => {
				AnyNode::JsVariableDeclarator(JsVariableDeclarator { syntax })
			}
			JS_IMPORT => AnyNode::JsImport(JsImport { syntax }),
			EXPORT_NAMED => AnyNode::ExportNamed(ExportNamed { syntax }),
			EXPORT_DEFAULT_DECL => AnyNode::ExportDefaultDecl(ExportDefaultDecl { syntax }),
			EXPORT_DEFAULT_EXPR => AnyNode::ExportDefaultExpr(ExportDefaultExpr { syntax }),
			EXPORT_WILDCARD => AnyNode::ExportWildcard(ExportWildcard { syntax }),
			EXPORT_DECL => AnyNode::ExportDecl(ExportDecl { syntax }),
			TS_IMPORT_EQUALS_DECL => AnyNode::TsImportEqualsDecl(TsImportEqualsDecl { syntax }),
			TS_EXPORT_ASSIGNMENT => AnyNode::TsExportAssignment(TsExportAssignment { syntax }),
			TS_NAMESPACE_EXPORT_DECL => {
				AnyNode::TsNamespaceExportDecl(TsNamespaceExportDecl { syntax })
			}
			JS_IMPORT_BARE_CLAUSE => AnyNode::JsImportBareClause(JsImportBareClause { syntax }),
			JS_IMPORT_NAMED_CLAUSE => AnyNode::JsImportNamedClause(JsImportNamedClause { syntax }),
			JS_IMPORT_DEFAULT_CLAUSE => {
				AnyNode::JsImportDefaultClause(JsImportDefaultClause { syntax })
			}
			JS_IMPORT_NAMESPACE_CLAUSE => {
				AnyNode::JsImportNamespaceClause(JsImportNamespaceClause { syntax })
			}
			JS_MODULE_SOURCE => AnyNode::JsModuleSource(JsModuleSource { syntax }),
			JS_IMPORT_ASSERTION => AnyNode::JsImportAssertion(JsImportAssertion { syntax }),
			JS_DEFAULT_IMPORT_SPECIFIER => {
				AnyNode::JsDefaultImportSpecifier(JsDefaultImportSpecifier { syntax })
			}
			JS_NAMED_IMPORT_SPECIFIER_LIST => {
				AnyNode::JsNamedImportSpecifierList(JsNamedImportSpecifierList { syntax })
			}
			JS_NAMESPACE_IMPORT_SPECIFIER => {
				AnyNode::JsNamespaceImportSpecifier(JsNamespaceImportSpecifier { syntax })
			}
			JS_SHORTHAND_NAMED_IMPORT_SPECIFIER => {
				AnyNode::JsShorthandNamedImportSpecifier(JsShorthandNamedImportSpecifier { syntax })
			}
			JS_NAMED_IMPORT_SPECIFIER => {
				AnyNode::JsNamedImportSpecifier(JsNamedImportSpecifier { syntax })
			}
			JS_LITERAL_EXPORT_NAME => AnyNode::JsLiteralExportName(JsLiteralExportName { syntax }),
			JS_IMPORT_ASSERTION_ENTRY => {
				AnyNode::JsImportAssertionEntry(JsImportAssertionEntry { syntax })
			}
			SPECIFIER => AnyNode::Specifier(Specifier { syntax }),
			JS_PRIVATE_NAME => AnyNode::JsPrivateName(JsPrivateName { syntax }),
			JS_REST_PARAMETER => AnyNode::JsRestParameter(JsRestParameter { syntax }),
			TS_EXTERNAL_MODULE_REF => AnyNode::TsExternalModuleRef(TsExternalModuleRef { syntax }),
			TS_ANY => AnyNode::TsAny(TsAny { syntax }),
			TS_UNKNOWN => AnyNode::TsUnknown(TsUnknown { syntax }),
			TS_NUMBER => AnyNode::TsNumber(TsNumber { syntax }),
			TS_OBJECT => AnyNode::TsObject(TsObject { syntax }),
			TS_BOOLEAN => AnyNode::TsBoolean(TsBoolean { syntax }),
			TS_BIGINT => AnyNode::TsBigint(TsBigint { syntax }),
			TS_STRING => AnyNode::TsString(TsString { syntax }),
			TS_SYMBOL => AnyNode::TsSymbol(TsSymbol { syntax }),
			TS_VOID => AnyNode::TsVoid(TsVoid { syntax }),
			TS_UNDEFINED => AnyNode::TsUndefined(TsUndefined { syntax }),
			TS_NULL => AnyNode::TsNull(TsNull { syntax }),
			TS_NEVER => AnyNode::TsNever(TsNever { syntax }),
			TS_THIS => AnyNode::TsThis(TsThis { syntax }),
			TS_LITERAL => AnyNode::TsLiteral(TsLiteral { syntax }),
			TS_PREDICATE => AnyNode::TsPredicate(TsPredicate { syntax }),
			TS_TUPLE => AnyNode::TsTuple(TsTuple { syntax }),
			TS_PAREN => AnyNode::TsParen(TsParen { syntax }),
			TS_TYPE_REF => AnyNode::TsTypeRef(TsTypeRef { syntax }),
			TS_TEMPLATE => AnyNode::TsTemplate(TsTemplate { syntax }),
			TS_MAPPED_TYPE => AnyNode::TsMappedType(TsMappedType { syntax }),
			TS_IMPORT => AnyNode::TsImport(TsImport { syntax }),
			TS_ARRAY => AnyNode::TsArray(TsArray { syntax }),
			TS_INDEXED_ARRAY => AnyNode::TsIndexedArray(TsIndexedArray { syntax }),
			TS_TYPE_OPERATOR => AnyNode::TsTypeOperator(TsTypeOperator { syntax }),
			TS_INTERSECTION => AnyNode::TsIntersection(TsIntersection { syntax }),
			TS_UNION => AnyNode::TsUnion(TsUnion { syntax }),
			TS_FN_TYPE => AnyNode::TsFnType(TsFnType { syntax }),
			TS_CONSTRUCTOR_TYPE => AnyNode::TsConstructorType(TsConstructorType { syntax }),
			TS_CONDITIONAL_TYPE => AnyNode::TsConditionalType(TsConditionalType { syntax }),
			TS_OBJECT_TYPE => AnyNode::TsObjectType(TsObjectType { syntax }),
			TS_INFER => AnyNode::TsInfer(TsInfer { syntax }),
			TS_TUPLE_ELEMENT => AnyNode::TsTupleElement(TsTupleElement { syntax }),
			TS_ENUM_MEMBER => AnyNode::TsEnumMember(TsEnumMember { syntax }),
			TS_TEMPLATE_ELEMENT => AnyNode::TsTemplateElement(TsTemplateElement { syntax }),
			TS_MAPPED_TYPE_READONLY => {
				AnyNode::TsMappedTypeReadonly(TsMappedTypeReadonly { syntax })
			}
			TS_MAPPED_TYPE_PARAM => AnyNode::TsMappedTypeParam(TsMappedTypeParam { syntax }),
			TS_TYPE_NAME => AnyNode::TsTypeName(TsTypeName { syntax }),
			TS_EXTENDS => AnyNode::TsExtends(TsExtends { syntax }),
			TS_MODULE_BLOCK => AnyNode::TsModuleBlock(TsModuleBlock { syntax }),
			TS_TYPE_PARAM => AnyNode::TsTypeParam(TsTypeParam { syntax }),
			TS_CONSTRAINT => AnyNode::TsConstraint(TsConstraint { syntax }),
			TS_DEFAULT => AnyNode::TsDefault(TsDefault { syntax }),
			TS_CALL_SIGNATURE_DECL => AnyNode::TsCallSignatureDecl(TsCallSignatureDecl { syntax }),
			TS_CONSTRUCT_SIGNATURE_DECL => {
				AnyNode::TsConstructSignatureDecl(TsConstructSignatureDecl { syntax })
			}
			TS_PROPERTY_SIGNATURE => AnyNode::TsPropertySignature(TsPropertySignature { syntax }),
			TS_METHOD_SIGNATURE => AnyNode::TsMethodSignature(TsMethodSignature { syntax }),
			TS_QUALIFIED_PATH => AnyNode::TsQualifiedPath(TsQualifiedPath { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			AnyNode::JsUnknownStatement(it) => &it.syntax,
			AnyNode::JsUnknownExpression(it) => &it.syntax,
			AnyNode::JsUnknownMember(it) => &it.syntax,
			AnyNode::JsUnknownBinding(it) => &it.syntax,
			AnyNode::JsUnknownAssignment(it) => &it.syntax,
			AnyNode::JsUnknownModifier(it) => &it.syntax,
			AnyNode::JsUnknownImportAssertionEntry(it) => &it.syntax,
			AnyNode::JsUnknownNamedImportSpecifier(it) => &it.syntax,
			AnyNode::List(it) => &it.syntax,
			AnyNode::Ident(it) => &it.syntax,
			AnyNode::JsScript(it) => &it.syntax,
			AnyNode::JsModule(it) => &it.syntax,
			AnyNode::JsDirective(it) => &it.syntax,
			AnyNode::JsBlockStatement(it) => &it.syntax,
			AnyNode::JsEmptyStatement(it) => &it.syntax,
			AnyNode::JsExpressionStatement(it) => &it.syntax,
			AnyNode::JsIfStatement(it) => &it.syntax,
			AnyNode::JsDoWhileStatement(it) => &it.syntax,
			AnyNode::JsWhileStatement(it) => &it.syntax,
			AnyNode::ForStmt(it) => &it.syntax,
			AnyNode::ForInStmt(it) => &it.syntax,
			AnyNode::ForOfStmt(it) => &it.syntax,
			AnyNode::JsContinueStatement(it) => &it.syntax,
			AnyNode::JsBreakStatement(it) => &it.syntax,
			AnyNode::JsReturnStatement(it) => &it.syntax,
			AnyNode::JsWithStatement(it) => &it.syntax,
			AnyNode::JsLabeledStatement(it) => &it.syntax,
			AnyNode::JsSwitchStatement(it) => &it.syntax,
			AnyNode::JsThrowStatement(it) => &it.syntax,
			AnyNode::JsTryStatement(it) => &it.syntax,
			AnyNode::JsTryFinallyStatement(it) => &it.syntax,
			AnyNode::JsDebuggerStatement(it) => &it.syntax,
			AnyNode::JsFunctionDeclaration(it) => &it.syntax,
			AnyNode::JsClassDeclaration(it) => &it.syntax,
			AnyNode::JsVariableDeclarationStatement(it) => &it.syntax,
			AnyNode::TsEnum(it) => &it.syntax,
			AnyNode::TsTypeAliasDecl(it) => &it.syntax,
			AnyNode::TsNamespaceDecl(it) => &it.syntax,
			AnyNode::TsModuleDecl(it) => &it.syntax,
			AnyNode::TsInterfaceDecl(it) => &it.syntax,
			AnyNode::JsElseClause(it) => &it.syntax,
			AnyNode::ForStmtInit(it) => &it.syntax,
			AnyNode::ForStmtTest(it) => &it.syntax,
			AnyNode::ForStmtUpdate(it) => &it.syntax,
			AnyNode::JsVariableDeclaration(it) => &it.syntax,
			AnyNode::JsCaseClause(it) => &it.syntax,
			AnyNode::JsDefaultClause(it) => &it.syntax,
			AnyNode::JsCatchClause(it) => &it.syntax,
			AnyNode::JsFinallyClause(it) => &it.syntax,
			AnyNode::JsCatchDeclaration(it) => &it.syntax,
			AnyNode::JsArrayExpression(it) => &it.syntax,
			AnyNode::JsArrowFunctionExpression(it) => &it.syntax,
			AnyNode::JsAssignmentExpression(it) => &it.syntax,
			AnyNode::JsAwaitExpression(it) => &it.syntax,
			AnyNode::JsBinaryExpression(it) => &it.syntax,
			AnyNode::JsClassExpression(it) => &it.syntax,
			AnyNode::JsConditionalExpression(it) => &it.syntax,
			AnyNode::JsComputedMemberExpression(it) => &it.syntax,
			AnyNode::JsFunctionExpression(it) => &it.syntax,
			AnyNode::JsImportCallExpression(it) => &it.syntax,
			AnyNode::JsLogicalExpression(it) => &it.syntax,
			AnyNode::JsObjectExpression(it) => &it.syntax,
			AnyNode::JsParenthesizedExpression(it) => &it.syntax,
			AnyNode::JsIdentifierExpression(it) => &it.syntax,
			AnyNode::JsSequenceExpression(it) => &it.syntax,
			AnyNode::JsStaticMemberExpression(it) => &it.syntax,
			AnyNode::JsSuperExpression(it) => &it.syntax,
			AnyNode::JsThisExpression(it) => &it.syntax,
			AnyNode::JsUnaryExpression(it) => &it.syntax,
			AnyNode::JsPreUpdateExpression(it) => &it.syntax,
			AnyNode::JsPostUpdateExpression(it) => &it.syntax,
			AnyNode::JsYieldExpression(it) => &it.syntax,
			AnyNode::Template(it) => &it.syntax,
			AnyNode::NewExpr(it) => &it.syntax,
			AnyNode::CallExpr(it) => &it.syntax,
			AnyNode::NewTarget(it) => &it.syntax,
			AnyNode::ImportMeta(it) => &it.syntax,
			AnyNode::TsNonNull(it) => &it.syntax,
			AnyNode::TsAssertion(it) => &it.syntax,
			AnyNode::TsConstAssertion(it) => &it.syntax,
			AnyNode::TsTypeArgs(it) => &it.syntax,
			AnyNode::ArgList(it) => &it.syntax,
			AnyNode::TsTypeParams(it) => &it.syntax,
			AnyNode::JsParameterList(it) => &it.syntax,
			AnyNode::TsTypeAnnotation(it) => &it.syntax,
			AnyNode::JsFunctionBody(it) => &it.syntax,
			AnyNode::JsSpread(it) => &it.syntax,
			AnyNode::JsArrayHole(it) => &it.syntax,
			AnyNode::JsReferenceIdentifier(it) => &it.syntax,
			AnyNode::JsLiteralMemberName(it) => &it.syntax,
			AnyNode::JsComputedMemberName(it) => &it.syntax,
			AnyNode::JsPropertyObjectMember(it) => &it.syntax,
			AnyNode::JsMethodObjectMember(it) => &it.syntax,
			AnyNode::JsGetterObjectMember(it) => &it.syntax,
			AnyNode::JsSetterObjectMember(it) => &it.syntax,
			AnyNode::JsShorthandPropertyObjectMember(it) => &it.syntax,
			AnyNode::TsImplementsClause(it) => &it.syntax,
			AnyNode::JsExtendsClause(it) => &it.syntax,
			AnyNode::TsExprWithTypeArgs(it) => &it.syntax,
			AnyNode::JsPrivateClassMemberName(it) => &it.syntax,
			AnyNode::JsConstructorClassMember(it) => &it.syntax,
			AnyNode::JsPropertyClassMember(it) => &it.syntax,
			AnyNode::JsMethodClassMember(it) => &it.syntax,
			AnyNode::JsGetterClassMember(it) => &it.syntax,
			AnyNode::JsSetterClassMember(it) => &it.syntax,
			AnyNode::JsEmptyClassMember(it) => &it.syntax,
			AnyNode::TsIndexSignature(it) => &it.syntax,
			AnyNode::TsAccessibility(it) => &it.syntax,
			AnyNode::JsConstructorParameterList(it) => &it.syntax,
			AnyNode::TsConstructorParam(it) => &it.syntax,
			AnyNode::JsBindingPatternWithDefault(it) => &it.syntax,
			AnyNode::JsEqualValueClause(it) => &it.syntax,
			AnyNode::JsModifier(it) => &it.syntax,
			AnyNode::JsIdentifierAssignment(it) => &it.syntax,
			AnyNode::JsStaticMemberAssignment(it) => &it.syntax,
			AnyNode::JsComputedMemberAssignment(it) => &it.syntax,
			AnyNode::JsParenthesizedAssignment(it) => &it.syntax,
			AnyNode::JsAssignmentWithDefault(it) => &it.syntax,
			AnyNode::JsArrayAssignmentPattern(it) => &it.syntax,
			AnyNode::JsObjectAssignmentPattern(it) => &it.syntax,
			AnyNode::JsArrayAssignmentPatternRestElement(it) => &it.syntax,
			AnyNode::JsObjectAssignmentPatternShorthandProperty(it) => &it.syntax,
			AnyNode::JsObjectAssignmentPatternProperty(it) => &it.syntax,
			AnyNode::JsObjectAssignmentPatternRest(it) => &it.syntax,
			AnyNode::JsName(it) => &it.syntax,
			AnyNode::JsIdentifierBinding(it) => &it.syntax,
			AnyNode::JsArrayBindingPattern(it) => &it.syntax,
			AnyNode::JsObjectBindingPattern(it) => &it.syntax,
			AnyNode::JsArrayBindingPatternRestElement(it) => &it.syntax,
			AnyNode::JsObjectBindingPatternProperty(it) => &it.syntax,
			AnyNode::JsObjectBindingPatternRest(it) => &it.syntax,
			AnyNode::JsObjectBindingPatternShorthandProperty(it) => &it.syntax,
			AnyNode::JsStringLiteralExpression(it) => &it.syntax,
			AnyNode::JsNumberLiteralExpression(it) => &it.syntax,
			AnyNode::JsBigIntLiteralExpression(it) => &it.syntax,
			AnyNode::JsBooleanLiteralExpression(it) => &it.syntax,
			AnyNode::JsNullLiteralExpression(it) => &it.syntax,
			AnyNode::JsRegexLiteralExpression(it) => &it.syntax,
			AnyNode::JsVariableDeclarator(it) => &it.syntax,
			AnyNode::JsImport(it) => &it.syntax,
			AnyNode::ExportNamed(it) => &it.syntax,
			AnyNode::ExportDefaultDecl(it) => &it.syntax,
			AnyNode::ExportDefaultExpr(it) => &it.syntax,
			AnyNode::ExportWildcard(it) => &it.syntax,
			AnyNode::ExportDecl(it) => &it.syntax,
			AnyNode::TsImportEqualsDecl(it) => &it.syntax,
			AnyNode::TsExportAssignment(it) => &it.syntax,
			AnyNode::TsNamespaceExportDecl(it) => &it.syntax,
			AnyNode::JsImportBareClause(it) => &it.syntax,
			AnyNode::JsImportNamedClause(it) => &it.syntax,
			AnyNode::JsImportDefaultClause(it) => &it.syntax,
			AnyNode::JsImportNamespaceClause(it) => &it.syntax,
			AnyNode::JsModuleSource(it) => &it.syntax,
			AnyNode::JsImportAssertion(it) => &it.syntax,
			AnyNode::JsDefaultImportSpecifier(it) => &it.syntax,
			AnyNode::JsNamedImportSpecifierList(it) => &it.syntax,
			AnyNode::JsNamespaceImportSpecifier(it) => &it.syntax,
			AnyNode::JsShorthandNamedImportSpecifier(it) => &it.syntax,
			AnyNode::JsNamedImportSpecifier(it) => &it.syntax,
			AnyNode::JsLiteralExportName(it) => &it.syntax,
			AnyNode::JsImportAssertionEntry(it) => &it.syntax,
			AnyNode::Specifier(it) => &it.syntax,
			AnyNode::JsPrivateName(it) => &it.syntax,
			AnyNode::JsRestParameter(it) => &it.syntax,
			AnyNode::TsExternalModuleRef(it) => &it.syntax,
			AnyNode::TsAny(it) => &it.syntax,
			AnyNode::TsUnknown(it) => &it.syntax,
			AnyNode::TsNumber(it) => &it.syntax,
			AnyNode::TsObject(it) => &it.syntax,
			AnyNode::TsBoolean(it) => &it.syntax,
			AnyNode::TsBigint(it) => &it.syntax,
			AnyNode::TsString(it) => &it.syntax,
			AnyNode::TsSymbol(it) => &it.syntax,
			AnyNode::TsVoid(it) => &it.syntax,
			AnyNode::TsUndefined(it) => &it.syntax,
			AnyNode::TsNull(it) => &it.syntax,
			AnyNode::TsNever(it) => &it.syntax,
			AnyNode::TsThis(it) => &it.syntax,
			AnyNode::TsLiteral(it) => &it.syntax,
			AnyNode::TsPredicate(it) => &it.syntax,
			AnyNode::TsTuple(it) => &it.syntax,
			AnyNode::TsParen(it) => &it.syntax,
			AnyNode::TsTypeRef(it) => &it.syntax,
			AnyNode::TsTemplate(it) => &it.syntax,
			AnyNode::TsMappedType(it) => &it.syntax,
			AnyNode::TsImport(it) => &it.syntax,
			AnyNode::TsArray(it) => &it.syntax,
			AnyNode::TsIndexedArray(it) => &it.syntax,
			AnyNode::TsTypeOperator(it) => &it.syntax,
			AnyNode::TsIntersection(it) => &it.syntax,
			AnyNode::TsUnion(it) => &it.syntax,
			AnyNode::TsFnType(it) => &it.syntax,
			AnyNode::TsConstructorType(it) => &it.syntax,
			AnyNode::TsConditionalType(it) => &it.syntax,
			AnyNode::TsObjectType(it) => &it.syntax,
			AnyNode::TsInfer(it) => &it.syntax,
			AnyNode::TsTupleElement(it) => &it.syntax,
			AnyNode::TsEnumMember(it) => &it.syntax,
			AnyNode::TsTemplateElement(it) => &it.syntax,
			AnyNode::TsMappedTypeReadonly(it) => &it.syntax,
			AnyNode::TsMappedTypeParam(it) => &it.syntax,
			AnyNode::TsTypeName(it) => &it.syntax,
			AnyNode::TsExtends(it) => &it.syntax,
			AnyNode::TsModuleBlock(it) => &it.syntax,
			AnyNode::TsTypeParam(it) => &it.syntax,
			AnyNode::TsConstraint(it) => &it.syntax,
			AnyNode::TsDefault(it) => &it.syntax,
			AnyNode::TsCallSignatureDecl(it) => &it.syntax,
			AnyNode::TsConstructSignatureDecl(it) => &it.syntax,
			AnyNode::TsPropertySignature(it) => &it.syntax,
			AnyNode::TsMethodSignature(it) => &it.syntax,
			AnyNode::TsQualifiedPath(it) => &it.syntax,
		}
	}
}
impl std::fmt::Debug for AnyNode {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			AnyNode::JsUnknownStatement(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsUnknownExpression(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsUnknownMember(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsUnknownBinding(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsUnknownAssignment(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsUnknownModifier(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsUnknownImportAssertionEntry(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsUnknownNamedImportSpecifier(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::List(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::Ident(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsScript(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsModule(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsDirective(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsBlockStatement(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsEmptyStatement(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsExpressionStatement(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsIfStatement(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsDoWhileStatement(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsWhileStatement(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::ForStmt(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::ForInStmt(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::ForOfStmt(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsContinueStatement(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsBreakStatement(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsReturnStatement(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsWithStatement(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsLabeledStatement(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsSwitchStatement(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsThrowStatement(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsTryStatement(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsTryFinallyStatement(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsDebuggerStatement(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsFunctionDeclaration(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsClassDeclaration(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsVariableDeclarationStatement(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsEnum(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsTypeAliasDecl(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsNamespaceDecl(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsModuleDecl(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsInterfaceDecl(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsElseClause(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::ForStmtInit(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::ForStmtTest(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::ForStmtUpdate(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsVariableDeclaration(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsCaseClause(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsDefaultClause(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsCatchClause(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsFinallyClause(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsCatchDeclaration(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsArrayExpression(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsArrowFunctionExpression(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsAssignmentExpression(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsAwaitExpression(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsBinaryExpression(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsClassExpression(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsConditionalExpression(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsComputedMemberExpression(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsFunctionExpression(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsImportCallExpression(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsLogicalExpression(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsObjectExpression(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsParenthesizedExpression(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsIdentifierExpression(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsSequenceExpression(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsStaticMemberExpression(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsSuperExpression(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsThisExpression(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsUnaryExpression(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsPreUpdateExpression(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsPostUpdateExpression(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsYieldExpression(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::Template(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::NewExpr(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::CallExpr(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::NewTarget(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::ImportMeta(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsNonNull(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsAssertion(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsConstAssertion(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsTypeArgs(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::ArgList(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsTypeParams(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsParameterList(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsTypeAnnotation(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsFunctionBody(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsSpread(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsArrayHole(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsReferenceIdentifier(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsLiteralMemberName(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsComputedMemberName(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsPropertyObjectMember(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsMethodObjectMember(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsGetterObjectMember(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsSetterObjectMember(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsShorthandPropertyObjectMember(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsImplementsClause(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsExtendsClause(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsExprWithTypeArgs(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsPrivateClassMemberName(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsConstructorClassMember(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsPropertyClassMember(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsMethodClassMember(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsGetterClassMember(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsSetterClassMember(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsEmptyClassMember(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsIndexSignature(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsAccessibility(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsConstructorParameterList(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsConstructorParam(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsBindingPatternWithDefault(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsEqualValueClause(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsModifier(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsIdentifierAssignment(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsStaticMemberAssignment(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsComputedMemberAssignment(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsParenthesizedAssignment(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsAssignmentWithDefault(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsArrayAssignmentPattern(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsObjectAssignmentPattern(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsArrayAssignmentPatternRestElement(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsObjectAssignmentPatternShorthandProperty(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsObjectAssignmentPatternProperty(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsObjectAssignmentPatternRest(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsName(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsIdentifierBinding(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsArrayBindingPattern(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsObjectBindingPattern(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsArrayBindingPatternRestElement(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsObjectBindingPatternProperty(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsObjectBindingPatternRest(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsObjectBindingPatternShorthandProperty(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsStringLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsNumberLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsBigIntLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsBooleanLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsNullLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsRegexLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsVariableDeclarator(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsImport(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::ExportNamed(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::ExportDefaultDecl(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::ExportDefaultExpr(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::ExportWildcard(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::ExportDecl(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsImportEqualsDecl(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsExportAssignment(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsNamespaceExportDecl(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsImportBareClause(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsImportNamedClause(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsImportDefaultClause(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsImportNamespaceClause(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsModuleSource(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsImportAssertion(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsDefaultImportSpecifier(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsNamedImportSpecifierList(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsNamespaceImportSpecifier(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsShorthandNamedImportSpecifier(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsNamedImportSpecifier(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsLiteralExportName(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsImportAssertionEntry(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::Specifier(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsPrivateName(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::JsRestParameter(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsExternalModuleRef(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsAny(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsUnknown(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsNumber(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsObject(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsBoolean(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsBigint(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsString(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsSymbol(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsVoid(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsUndefined(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsNull(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsNever(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsThis(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsLiteral(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsPredicate(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsTuple(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsParen(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsTypeRef(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsTemplate(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsMappedType(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsImport(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsArray(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsIndexedArray(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsTypeOperator(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsIntersection(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsUnion(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsFnType(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsConstructorType(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsConditionalType(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsObjectType(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsInfer(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsTupleElement(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsEnumMember(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsTemplateElement(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsMappedTypeReadonly(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsMappedTypeParam(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsTypeName(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsExtends(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsModuleBlock(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsTypeParam(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsConstraint(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsDefault(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsCallSignatureDecl(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsConstructSignatureDecl(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsPropertySignature(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsMethodSignature(it) => std::fmt::Debug::fmt(it, f),
			AnyNode::TsQualifiedPath(it) => std::fmt::Debug::fmt(it, f),
		}
	}
}
impl std::fmt::Display for JsAnyRoot {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsAnyStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsAnyModuleItem {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsAnyExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ForHead {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ForLeft {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsAnyAssignment {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsAnySwitchClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsAnyBindingPattern {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsAnyLiteralExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsAnyBinding {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsAnyArrowFunctionParameters {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsAnyArrowFunctionBody {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsAnyArrayElement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsAnyAssignmentPattern {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsAnyName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsAnyObjectMemberName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsAnyObjectMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsAnyClassMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsAnyClassMemberName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsAnyConstructorParameter {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsAnyModifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsAnyArrayAssignmentPatternElement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsAnyObjectAssignmentPatternMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsAnyArrayBindingPatternElement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsAnyObjectBindingPatternMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for AnyJsImportClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsAnyNamedImport {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsAnyNamedImportSpecifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsAnyImportAssertionEntry {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for DefaultDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsAnyExportDeclaration {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsAnyParameter {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsModuleRef {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsEntityName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsThisOrMore {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsTypeElement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsNamespaceBody {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for AnyNode {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsUnknownStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsUnknownExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsUnknownMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsUnknownBinding {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsUnknownAssignment {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsUnknownModifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsUnknownImportAssertionEntry {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsUnknownNamedImportSpecifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for List {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Ident {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsScript {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsModule {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsDirective {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsBlockStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsEmptyStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsExpressionStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsIfStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsDoWhileStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsWhileStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ForStmt {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ForInStmt {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ForOfStmt {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsContinueStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsBreakStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsReturnStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsWithStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsLabeledStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsSwitchStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsThrowStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsTryStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsTryFinallyStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsDebuggerStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsFunctionDeclaration {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsClassDeclaration {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsVariableDeclarationStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsEnum {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsTypeAliasDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsNamespaceDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsModuleDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsInterfaceDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsElseClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ForStmtInit {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ForStmtTest {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ForStmtUpdate {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsVariableDeclaration {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsCaseClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsDefaultClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsCatchClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsFinallyClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsCatchDeclaration {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsArrayExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsArrowFunctionExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsAssignmentExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsAwaitExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsBinaryExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsClassExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsConditionalExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsComputedMemberExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsFunctionExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsImportCallExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsLogicalExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsObjectExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsParenthesizedExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsIdentifierExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsSequenceExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsStaticMemberExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsSuperExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsThisExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsUnaryExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsPreUpdateExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsPostUpdateExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsYieldExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Template {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for NewExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for CallExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for NewTarget {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ImportMeta {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsNonNull {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsAssertion {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsConstAssertion {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsTypeArgs {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ArgList {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsTypeParams {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsParameterList {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsTypeAnnotation {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsFunctionBody {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsSpread {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsArrayHole {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsReferenceIdentifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsLiteralMemberName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsComputedMemberName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsPropertyObjectMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsMethodObjectMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsGetterObjectMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsSetterObjectMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsShorthandPropertyObjectMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsImplementsClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsExtendsClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsExprWithTypeArgs {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsPrivateClassMemberName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsConstructorClassMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsPropertyClassMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsMethodClassMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsGetterClassMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsSetterClassMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsEmptyClassMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsIndexSignature {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsAccessibility {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsConstructorParameterList {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsConstructorParam {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsBindingPatternWithDefault {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsEqualValueClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsModifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsIdentifierAssignment {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsStaticMemberAssignment {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsComputedMemberAssignment {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsParenthesizedAssignment {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsAssignmentWithDefault {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsArrayAssignmentPattern {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsObjectAssignmentPattern {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsArrayAssignmentPatternRestElement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsObjectAssignmentPatternShorthandProperty {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsObjectAssignmentPatternProperty {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsObjectAssignmentPatternRest {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsIdentifierBinding {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsArrayBindingPattern {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsObjectBindingPattern {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsArrayBindingPatternRestElement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsObjectBindingPatternProperty {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsObjectBindingPatternRest {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsObjectBindingPatternShorthandProperty {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsStringLiteralExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsNumberLiteralExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsBigIntLiteralExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsBooleanLiteralExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsNullLiteralExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsRegexLiteralExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsVariableDeclarator {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsImport {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ExportNamed {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ExportDefaultDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ExportDefaultExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ExportWildcard {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ExportDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsImportEqualsDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsExportAssignment {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsNamespaceExportDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsImportBareClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsImportNamedClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsImportDefaultClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsImportNamespaceClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsModuleSource {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsImportAssertion {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsDefaultImportSpecifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsNamedImportSpecifierList {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsNamespaceImportSpecifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsShorthandNamedImportSpecifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsNamedImportSpecifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsLiteralExportName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsImportAssertionEntry {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Specifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsPrivateName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsRestParameter {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsExternalModuleRef {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsAny {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsUnknown {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsNumber {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsObject {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsBoolean {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsBigint {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsString {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsSymbol {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsVoid {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsUndefined {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsNull {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsNever {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsThis {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsLiteral {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsPredicate {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsTuple {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsParen {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsTypeRef {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsTemplate {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsMappedType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsImport {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsArray {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsIndexedArray {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsTypeOperator {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsIntersection {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsUnion {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsFnType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsConstructorType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsConditionalType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsObjectType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsInfer {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsTupleElement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsEnumMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsTemplateElement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsMappedTypeReadonly {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsMappedTypeParam {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsTypeName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsExtends {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsModuleBlock {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsTypeParam {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsConstraint {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsDefault {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsCallSignatureDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsConstructSignatureDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsPropertySignature {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsMethodSignature {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsQualifiedPath {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
