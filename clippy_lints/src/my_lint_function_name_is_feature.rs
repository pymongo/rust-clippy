use rustc_lint::{EarlyLintPass, EarlyContext};
use rustc_session::{declare_lint_pass, declare_tool_lint};
use rustc_ast::ast::*;
//use rustc_span::span_encoding::Span;
use rustc_ast::visit::FnKind;
use rustc_span::Span;
//use clippy_utils::diagnostics::span_lint_and_help;

declare_clippy_lint! {
    /// **What it does:**
    ///
    /// **Why is this bad?**
    ///
    /// **Known problems:** None.
    ///
    /// **Example:**
    ///
    /// ```rust
    /// // example code where clippy issues a warning
    /// ```
    /// Use instead:
    /// ```rust
    /// // example code which does not raise clippy warning
    /// ```
    pub MY_LINT_FUNCTION_NAME_IS_FEATURE,
    correctness,
    "code `fn feature` generate by tfn should not to commit"
}

declare_lint_pass!(MyLintFunctionNameIsFeature => [MY_LINT_FUNCTION_NAME_IS_FEATURE]);

impl EarlyLintPass for MyLintFunctionNameIsFeature {
    fn check_fn(&mut self, cx: &EarlyContext<'_>, fn_kind: FnKind<'_>, span: Span, _: NodeId) {
        dbg!(line!());
        clippy_utils::diagnostics::span_lint(
            cx,
            MY_LINT_FUNCTION_NAME_IS_FEATURE,
            span,
            "code `fn feature` generate by tfn should not to commit"
        );
        match fn_kind {
            FnKind::Fn(_, ident, ..) => {
                dbg!(line!());
                // check if `fn` name is `function`
                if ident.name.as_str() == "feature" {
                    dbg!(line!());
                    clippy_utils::diagnostics::span_lint(
                        cx,
                        MY_LINT_FUNCTION_NAME_IS_FEATURE,
                        span,
                        "code `fn feature` generate by tfn should not to commit"
                    );
                }
            }
            // ignore closures
            FnKind::Closure(..) => {}
        }
    }
}
