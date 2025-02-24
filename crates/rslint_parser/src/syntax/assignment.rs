use crate::event::{rewrite_events, RewriteParseEvents};
use crate::parser::{expected_any, ParsedSyntax, ToDiagnostic};
use crate::syntax::class::parse_equal_value_clause;
use crate::syntax::expr::{conditional_expr, expr, is_at_name, parse_name, unary_expr};
use crate::syntax::js_parse_error::{expected_assignment_target, expected_identifier};
use crate::syntax::pattern::{ParseArrayPattern, ParseObjectPattern, ParseWithDefaultPattern};
use crate::ParsedSyntax::{Absent, Present};
use crate::{CompletedMarker, Parser};
use crate::{SyntaxKind::*, *};

// test assignment_target
// foo += bar = b ??= 3;
// a.foo -= bar;
// (foo = bar);
// (((foo))) = bar;
// a["test"] = bar;
// a.call().chain().member = x;
// ++count === 3
// a['b'] = c[d] = "test"

// test_err invalid_assignment_target
// ++a = b;
// (++a) = b;
// (a = b;
// a?.b = b;
// a?.["b"] = b;
// (a +) = b;

/// Converts the passed in lhs expression to an assignment pattern
/// The passed checkpoint allows to restore the parser to the state before it started parsing the expression.
pub(crate) fn expression_to_assignment_pattern(
	p: &mut Parser,
	target: CompletedMarker,
	checkpoint: Checkpoint,
	expr_kind: AssignmentExprPrecedence,
) -> CompletedMarker {
	if let Ok(assignment_target) = try_expression_to_assignment(p, target, checkpoint) {
		return assignment_target;
	}

	let expression_end = p.token_pos();
	p.rewind(checkpoint);

	match parse_assignment_pattern(p, expr_kind) {
		Present(target) => target,
		Absent => wrap_expression_in_invalid_assignment(p, expression_end),
	}
}

pub(crate) fn parse_assignment_pattern(
	p: &mut Parser,
	expression_kind: AssignmentExprPrecedence,
) -> ParsedSyntax<CompletedMarker> {
	match p.cur() {
		T!['['] => ArrayAssignmentPattern.parse_array_pattern(p),
		T!['{'] if p.state.allow_object_expr => ObjectAssignmentPattern.parse_object_pattern(p),
		_ => parse_assignment(p, expression_kind),
	}
}

/// Re-parses an expression as an assignment.
pub(crate) fn expression_to_assignment(
	p: &mut Parser,
	target: CompletedMarker,
	checkpoint: Checkpoint,
) -> CompletedMarker {
	if let Ok(assignment) = try_expression_to_assignment(p, target, checkpoint) {
		assignment
	} else {
		// Doesn't seem to be a valid assignment target. Recover and create an error.
		let expression_end = p.token_pos();
		p.rewind(checkpoint);
		wrap_expression_in_invalid_assignment(p, expression_end)
	}
}

pub(crate) enum AssignmentExprPrecedence {
	Unary,
	Conditional,
	Any,
}

pub(crate) fn parse_assignment(
	p: &mut Parser,
	expr_kind: AssignmentExprPrecedence,
) -> ParsedSyntax<CompletedMarker> {
	let checkpoint = p.checkpoint();

	// TODO remove the rewind inside of the error handle once the `unary_expr` returns a ParsedSyntax
	let assignment_expression = match expr_kind {
		AssignmentExprPrecedence::Unary => unary_expr(p),
		AssignmentExprPrecedence::Conditional => conditional_expr(p),
		AssignmentExprPrecedence::Any => expr(p),
	};

	if let Some(expr) = assignment_expression {
		Present(expression_to_assignment(p, expr, checkpoint))
	} else {
		// Only necessary because `unary_expr` always adds a "expected an expression" error.
		p.rewind(checkpoint);
		Absent
	}
}

struct AssignmentPatternWithDefault;

impl ParseWithDefaultPattern for AssignmentPatternWithDefault {
	#[inline]
	fn pattern_with_default_kind() -> SyntaxKind {
		JS_ASSIGNMENT_WITH_DEFAULT
	}

	#[inline]
	fn expected_pattern_error(p: &Parser, range: Range<usize>) -> Diagnostic {
		expected_assignment_target(p, range)
	}

	#[inline]
	fn parse_pattern(&self, p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
		parse_assignment_pattern(p, AssignmentExprPrecedence::Conditional)
	}
}

struct ArrayAssignmentPattern;

impl ParseArrayPattern<AssignmentPatternWithDefault> for ArrayAssignmentPattern {
	#[inline]
	fn unknown_pattern_kind() -> SyntaxKind {
		JS_UNKNOWN_ASSIGNMENT
	}

	#[inline]
	fn array_pattern_kind() -> SyntaxKind {
		JS_ARRAY_ASSIGNMENT_PATTERN
	}

	// test array_assignment_target_rest
	// ([ ...abcd ] = a);
	// ([ ...(abcd) ] = a);
	// ([ ...m.test ] = c);
	// ([ ...m[call()] ] = c);
	// ([ ...any.expression().b ] = c);
	// ([ ...[x, y] ] = b);
	// ([ ...[ ...a ] ] = c);
	//
	// test_err array_assignment_target_rest_err
	// ([ ... ] = a);
	// ([ ...c = "default" ] = a);
	// ([ ...rest, other_assignment ] = a);
	#[inline]
	fn rest_pattern_kind() -> SyntaxKind {
		JS_ARRAY_ASSIGNMENT_PATTERN_REST_ELEMENT
	}

	#[inline]
	fn expected_element_error(p: &Parser, range: Range<usize>) -> Diagnostic {
		expected_any(&["assignment target", "rest element", "comma"], range).to_diagnostic(p)
	}

	#[inline]
	fn pattern_with_default(&self) -> AssignmentPatternWithDefault {
		AssignmentPatternWithDefault
	}
}

struct ObjectAssignmentPattern;

// test object_assignment_target
// ({} = {});
// ({ bar, baz } = {});
// ({ bar: [baz = "baz"], foo = "foo", ...rest } = {});
impl ParseObjectPattern for ObjectAssignmentPattern {
	#[inline]
	fn unknown_pattern_kind() -> SyntaxKind {
		JS_UNKNOWN_ASSIGNMENT
	}

	#[inline]
	fn object_pattern_kind() -> SyntaxKind {
		JS_OBJECT_ASSIGNMENT_PATTERN
	}

	#[inline]
	fn expected_property_pattern_error(p: &Parser, range: Range<usize>) -> Diagnostic {
		expected_any(&["assignment target", "rest property"], range).to_diagnostic(p)
	}

	// test property_assignment_target
	// ({x}= {});
	// ({x: y}= {});
	// ({x: y.test().z}= {});
	// ({x: ((z))}= {});
	// ({x: z["computed"]}= {});
	// ({x = "default"}= {});
	// ({x: y = "default"}= {});
	//
	// test_err property_assignment_target_err
	// ({:y} = {});
	// ({=y} = {});
	// ({:="test"} = {});
	// ({:=} = {});
	// ({ a b } = {});
	fn parse_property_pattern(&self, p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
		if !is_at_name(p) && !p.at_ts(token_set![T![:], T![=], T![ident], T![await], T![yield]]) {
			return Absent;
		}

		let m = p.start();

		let kind = if p.at(T![:]) || p.nth_at(1, T![:]) {
			parse_name(p).or_missing_with_error(p, expected_identifier);
			p.expect_required(T![:]);
			parse_assignment_pattern(p, AssignmentExprPrecedence::Conditional)
				.or_missing_with_error(p, expected_assignment_target);
			JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY
		} else {
			parse_assignment(p, AssignmentExprPrecedence::Conditional)
				.or_missing_with_error(p, expected_identifier);
			JS_OBJECT_ASSIGNMENT_PATTERN_SHORTHAND_PROPERTY
		};

		parse_equal_value_clause(p).or_missing(p);

		Present(m.complete(p, kind))
	}

	// test rest_property_assignment_target
	// ({ ...abcd } = a);
	// ({ ...(abcd) } = a);
	// ({ ...m.test } = c);
	// ({ ...m[call()] } = c);
	// ({ ...any.expression().b } = c);
	// ({ b: { ...a } } = c);
	//
	// test_err rest_property_assignment_target_err
	// ({ ... } = a);
	// ({ ...c = "default" } = a);
	// ({ ...{a} } = b);
	// ({ ...rest, other_assignment } = a);
	// ({ ...rest, } = a);
	fn parse_rest_property_pattern(&self, p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
		if !p.at(T![...]) {
			return Absent;
		}

		let m = p.start();
		p.bump(T![...]);

		let target = parse_assignment_pattern(p, AssignmentExprPrecedence::Conditional)
			.or_missing_with_error(p, expected_assignment_target);

		if let Some(mut target) = target {
			if matches!(
				target.kind(),
				JS_OBJECT_ASSIGNMENT_PATTERN | JS_ARRAY_ASSIGNMENT_PATTERN
			) {
				target.change_kind(p, JS_UNKNOWN_ASSIGNMENT);
				p.error(
					p.err_builder(
						"object and array assignment targets are not allowed in rest patterns",
					)
					.primary(target.range(p), ""),
				);
			}
		}

		Present(m.complete(p, JS_OBJECT_ASSIGNMENT_PATTERN_REST))
	}
}

fn try_expression_to_assignment(
	p: &mut Parser,
	target: CompletedMarker,
	checkpoint: Checkpoint,
) -> Result<CompletedMarker, ()> {
	if !matches!(
		target.kind(),
		JS_PARENTHESIZED_EXPRESSION
			| JS_STATIC_MEMBER_EXPRESSION
			| JS_COMPUTED_MEMBER_EXPRESSION
			| JS_IDENTIFIER_EXPRESSION
	) {
		return Err(());
	}

	// At this point it's guaranteed that the root node can be mapped to a assignment
	// but it's not yet guaranteed if it is valid or not (for example, a static member expression
	// is valid, except if it uses optional chaining).
	let mut reparse_assignment = ReparseAssignment::new();
	rewrite_events(&mut reparse_assignment, checkpoint, p);

	Ok(reparse_assignment.result.unwrap())
}

struct ReparseAssignment {
	// Stores the unfinished parents
	// Index 0: Re-mapped kind of the node
	// Index 1: Started marker. A `None` marker means that this node should be dropped
	//          from the re-written tree
	parents: Vec<(SyntaxKind, Option<Marker>)>,
	// Stores the completed assignment node (valid or invalid).
	result: Option<CompletedMarker>,
	// Tracks if the visitor is still inside of an assignment
	inside_assignment: bool,
}

impl ReparseAssignment {
	pub fn new() -> Self {
		Self {
			parents: Vec::default(),
			result: None,
			inside_assignment: true,
		}
	}
}

/// Rewrites expressions to assignments
/// * Converts parenthesized expression to parenthesized assignment
/// * Converts computed/static member expressions to computed/static member assignment.
///   Validates that the operator isn't `?.` .
/// * Converts identifier expressions to identifier assignment, drops the inner reference identifier
impl RewriteParseEvents for ReparseAssignment {
	fn start_node(&mut self, kind: SyntaxKind, p: &mut Parser) {
		if !self.inside_assignment {
			self.parents.push((kind, Some(p.start())));
			return;
		}

		let mapped_kind = match kind {
			JS_PARENTHESIZED_EXPRESSION => JS_PARENTHESIZED_ASSIGNMENT,
			JS_STATIC_MEMBER_EXPRESSION => {
				self.inside_assignment = false;
				JS_STATIC_MEMBER_ASSIGNMENT
			}
			JS_COMPUTED_MEMBER_EXPRESSION => {
				self.inside_assignment = false;
				JS_COMPUTED_MEMBER_ASSIGNMENT
			}
			JS_IDENTIFIER_EXPRESSION => JS_IDENTIFIER_ASSIGNMENT,
			JS_REFERENCE_IDENTIFIER => {
				self.parents.push((kind, None)); // Omit reference identifiers
				return;
			}
			_ => {
				self.inside_assignment = false;
				JS_UNKNOWN_ASSIGNMENT
			}
		};

		self.parents.push((mapped_kind, Some(p.start())));
	}

	fn finish_node(&mut self, p: &mut Parser) {
		let (kind, m) = self.parents.pop().unwrap();

		if let Some(m) = m {
			let completed = m.complete(p, kind);

			if kind == JS_UNKNOWN_ASSIGNMENT {
				p.error(invalid_assignment_error(p, completed.range(p)));
			}
			self.result = Some(completed);
		}
	}

	fn token(&mut self, kind: SyntaxKind, p: &mut Parser) {
		let parent = self.parents.last_mut();

		if let Some((parent_kind, _)) = parent {
			if matches!(
				*parent_kind,
				JS_COMPUTED_MEMBER_ASSIGNMENT | JS_STATIC_MEMBER_ASSIGNMENT
			) && kind == T![?.]
			{
				*parent_kind = JS_UNKNOWN_ASSIGNMENT
			}
		}

		p.bump_remap(kind);
	}
}

fn wrap_expression_in_invalid_assignment(p: &mut Parser, expression_end: usize) -> CompletedMarker {
	let unknown = p.start();
	// Eat all tokens until we reached the end of the original expression. This is better than
	// any other error recovery because it's already know where the expression ends.
	while p.token_pos() < expression_end {
		p.bump_any();
	}

	let completed = unknown.complete(p, JS_UNKNOWN_ASSIGNMENT);

	p.error(invalid_assignment_error(p, completed.range(p)));

	completed
}

fn invalid_assignment_error(p: &Parser, range: TextRange) -> Diagnostic {
	p.err_builder(&format!("Invalid assignment to `{}`", p.source(range)))
		.primary(range, "This expression cannot be assigned to")
}
