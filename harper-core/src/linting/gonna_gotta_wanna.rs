use crate::{
    Lint, Token, TokenStringExt,
    expr::{Expr, SequenceExpr},
    linting::{
        ExprLinter, LintGroup, LintKind, Suggestion, debug::format_lint_match, expr_linter::Chunk,
    },
};

struct GonnaGottaWanna {
    expr: SequenceExpr, // TODO
    informal: &'static str,
    formal: &'static str,
}

impl GonnaGottaWanna {
    fn new(informal: &'static str, formal: &'static str) -> Self {
        Self {
            expr: SequenceExpr::aco(informal)
                .then_optional(SequenceExpr::whitespace().t_set(["to", "a"])),
            informal,
            formal,
        }
    }
}

impl ExprLinter for GonnaGottaWanna {
    type Unit = Chunk;

    fn expr(&self) -> &dyn Expr {
        &self.expr
    }

    fn match_to_lint_with_context(
        &self,
        toks: &[Token],
        src: &[char],
        ctx: Option<(&[Token], &[Token])>,
    ) -> Option<Lint> {
        eprintln!("🚨 {}", format_lint_match(toks, ctx, src));
        let span = toks.span()?;
        let suggestions = vec![Suggestion::replace_with_match_case_str(
            self.formal,
            span.get_content(src),
        )];
        Some(Lint {
            span,
            lint_kind: LintKind::Style,
            suggestions,
            message: format!("Use '{}' instead of '{}'", self.formal, self.informal),
            ..Default::default()
        })
    }

    fn description(&self) -> &str {
        "gonna gotta wanna"
    }
}

pub fn lint_group() -> LintGroup {
    let mut group = LintGroup::empty();

    macro_rules! add_ggw {
        ($group:expr, { $($name:expr => ($informal:expr, $formal:expr)),+ $(,)? }) => {
            $(
                $group.add(
                    $name,
                    Box::new(GonnaGottaWanna::new($informal, $formal)),
                );
            )+
        };
    }

    add_ggw!(group, {
        "Gonna" => ("gonna", "going"),
        "Gotta" => ("gotta", "got"),
        "Wanna" => ("wanna", "want"),
    });

    group.set_all_rules_to(Some(true));
    group
}

#[cfg(test)]
mod tests {
    use crate::linting::tests::{assert_good_and_bad_suggestions, assert_suggestion_result};

    use super::lint_group;

    #[test]
    fn fix_gonna() {
        // We think you're gonna like it here."
        assert_suggestion_result(
            "We think you're gonna like it here.",
            lint_group(),
            // fix the grammar and the informality
            "We think you're going to like it here.",
        )
    }

    #[test]
    fn fix_gonna_to() {
        // "gonna" already means "going to", so adding "to" is redundant
        assert_good_and_bad_suggestions(
            "I am gonna to create the region by clicking and dragging the mouse on waveform but I couldn't find the way to do that.",
            lint_group(),
            &[
                // fix the grammar but remain informal
                "I am gonna create the region by clicking and dragging the mouse on waveform but I couldn't find the way to do that.",
                // fix the grammar and the informality
                "I am going to create the region by clicking and dragging the mouse on waveform but I couldn't find the way to do that.",
            ],
            &[],
        )
    }

    #[test]
    fn fix_gotta() {
        assert_suggestion_result(
            "Gotta Hear Them All: Towards Sound Source Aware Audio Generation.",
            lint_group(),
            "Got to Hear Them All: Towards Sound Source Aware Audio Generation.",
            // What about "have to"?
        )
    }

    #[test]
    fn fix_gotta_a_error() {
        // "gotta" only means "(have) got to", using it as "got a" is incorrect
        // but adding "a" is also redundant
        assert_good_and_bad_suggestions(
            "when I try to create a c/c++ database,I gotta a error",
            lint_group(),
            &[
                // fix the grammar but remain informal
                "when I try to create a c/c++ database,I got a error",
                // fix the grammar and the informality
                "when I try to create a c/c++ database,I got an error",
            ],
            &[],
        )
    }

    // You gotta a 20% OFF coupom.
    #[test]
    fn fix_gotta_a_coupom() {
        // "gotta" only means "(have) got to", using it as "got a" is incorrect
        // but adding "a" is also redundant
        assert_good_and_bad_suggestions(
            "You gotta a 20% OFF coupom.",
            lint_group(),
            &[
                // fix the grammar but remain informal
                "You got a 20% OFF coupom.",
                // what about "you get", "you receive", "you received"?
            ],
            &[],
        )
    }

    #[test]
    // it works well enough for me and gotta to get back working on other stuff
    fn fix_gotta_to() {
        // "gotta" already means "got to", so adding "to" is redundant
        assert_good_and_bad_suggestions(
            "it works well enough for me and gotta to get back working on other stuff",
            lint_group(),
            &[
                // fix the grammar but remain informal
                "it works well enough for me and gotta get back working on other stuff",
                // fix the grammar and the informality
                "it works well enough for me and have to get back working on other stuff",
                // What about "has to", "had to"?
            ],
            &[],
        )
    }

    #[test]
    fn fix_wanna_control() {
        assert_suggestion_result(
            "I wanna control G1 arms while walking using gamepad. But ...",
            lint_group(),
            "I want to control G1 arms while walking using gamepad. But ...",
        )
    }

    #[test]
    fn fix_wanna_a() {
        // "wanna" only means "want to", using it as "want a" is incorrect
        // Only way to fix fixes both the informality and the grammar goether
        assert_suggestion_result(
            "And then I remember from the beginning I just wanna a very simple array with some simple opreations.",
            lint_group(),
            "And then I remember from the beginning I just want a very simple array with some simple opreations.",
        )
    }

    #[test]
    fn fix_wanna_to() {
        // "wanna" already means "want to", so adding "to" is redundant
        assert_good_and_bad_suggestions(
            "it is stoped but i wanna to continue",
            lint_group(),
            &[
                // fix the grammar but remain informal
                "it is stoped but i wanna continue",
                // fix the grammar and the informality
                "it is stoped but i want to continue",
            ],
            &[],
        )
    }
}
