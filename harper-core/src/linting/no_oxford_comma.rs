use crate::{
    patterns::{Pattern, SequencePattern, WordSet},
    Document, Token, TokenStringExt,
};

use super::{Lint, LintKind, Linter, Suggestion};

pub struct NoOxfordComma {
    pattern: SequencePattern,
}

impl NoOxfordComma {
    pub fn new() -> Self {
        Self {
            pattern: SequencePattern::default()
                .then_noun_phrase()
                .then_comma()
                .then_whitespace()
                .then_noun_phrase()
                .then_comma()
                .then_whitespace()
                .then(Box::new(WordSet::all(&["and", "or", "nor"]))),
        }
    }

    fn match_to_lint(&self, matched_toks: &[Token], _source: &[char]) -> Lint {
        let last_comma_index = matched_toks.last_comma_index().unwrap();
        let offender = matched_toks[last_comma_index];

        Lint {
            span: offender.span,
            lint_kind: LintKind::Style,
            suggestions: vec![Suggestion::Remove],
            message: "Remove the Oxford comma here.".to_owned(),
            priority: 31,
        }
    }
}

impl Default for NoOxfordComma {
    fn default() -> Self {
        Self::new()
    }
}

impl Linter for NoOxfordComma {
    fn lint(&mut self, document: &Document) -> Vec<Lint> {
        let mut lints = Vec::new();

        for sentence in document.iter_sentences() {
            let mut tok_cursor = 0;

            loop {
                if tok_cursor >= sentence.len() {
                    break;
                }

                let match_len = self
                    .pattern
                    .matches(&sentence[tok_cursor..], document.get_source());

                if match_len != 0 {
                    let lint = self.match_to_lint(
                        &sentence[tok_cursor..tok_cursor + match_len],
                        document.get_source(),
                    );

                    lints.push(lint);
                    tok_cursor += match_len;
                } else {
                    tok_cursor += 1;
                }
            }
        }

        lints
    }

    fn description(&self) -> &str {
        "The Oxford comma is one of the more controversial rules in common use today. Enabling this lint checks that there is no comma before `and`, `or` or `nor` when listing out more than two ideas."
    }
}

#[cfg(test)]
mod tests {
    use crate::linting::tests::{assert_lint_count, assert_suggestion_result};

    use super::NoOxfordComma;

    #[test]
    fn t1() {
        assert_suggestion_result("Car, bus, and", NoOxfordComma::default(), "Car, bus and");
    }

    #[test]
    fn t2() {
        assert_suggestion_result(
            "Cars, buses, and trucks",
            NoOxfordComma::default(),
            "Cars, buses and trucks",
        );
    }

    #[test]
    fn t3() {
        assert_suggestion_result(
            "A dog, a cat, and a pig.",
            NoOxfordComma::default(),
            "A dog, a cat and a pig.",
        );
    }

    #[test]
    fn t4() {
        assert_suggestion_result(
            "A bat, a rat, and a hog.",
            NoOxfordComma::default(),
            "A bat, a rat and a hog.",
        );
    }

    #[test]
    fn fruits() {
        assert_lint_count(
            "An apple, a banana, and a pear",
            NoOxfordComma::default(),
            1,
        );
    }

    #[test]
    fn people() {
        assert_suggestion_result(
            "Nancy, Steve, and Carl are going to the coffee shop.",
            NoOxfordComma::default(),
            "Nancy, Steve and Carl are going to the coffee shop.",
        );
    }

    #[test]
    fn places() {
        assert_suggestion_result(
            "I've always wanted to visit Paris, Tokyo, and Rome.",
            NoOxfordComma::default(),
            "I've always wanted to visit Paris, Tokyo and Rome.",
        );
    }

    #[test]
    fn foods() {
        assert_suggestion_result(
            "My favorite foods are pizza, sushi, tacos, and burgers.",
            NoOxfordComma::default(),
            "My favorite foods are pizza, sushi, tacos and burgers.",
        );
    }

    #[test]
    fn allows_clean_music() {
        assert_lint_count(
            "I enjoy listening to pop music, rock, hip-hop, electronic dance and classical music.",
            NoOxfordComma::default(),
            0,
        );
    }

    #[test]
    fn allows_clean_nations() {
        assert_lint_count("The team consists of players from different countries: France, Germany, Italy and Spain.", NoOxfordComma::default(), 0);
    }

    #[test]
    fn or_writing() {
        assert_suggestion_result("Harper can be a lifesaver when writing technical documents, emails, or other formal forms of communication.", NoOxfordComma::default(), "Harper can be a lifesaver when writing technical documents, emails or other formal forms of communication.",);
    }

    #[test]
    fn sports() {
        assert_suggestion_result(
            "They enjoy playing soccer, basketball, or tennis.",
            NoOxfordComma::default(),
            "They enjoy playing soccer, basketball or tennis.",
        );
    }

    #[test]
    fn nor_vegetables() {
        assert_suggestion_result(
            "I like carrots, kale, nor broccoli.",
            NoOxfordComma::default(),
            "I like carrots, kale nor broccoli.",
        );
    }
}
