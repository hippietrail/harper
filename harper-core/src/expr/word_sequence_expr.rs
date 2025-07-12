use crate::{expr::{Expr, SequenceExpr}, patterns::WordSet, Span, Token};

#[derive(Default)]
pub struct WordSequenceExpr {
    seq: SequenceExpr,
}

impl Expr for WordSequenceExpr {
    fn run(&self, cursor: usize, tokens: &[Token], source: &[char]) -> Option<Span> {
        self.seq.run(cursor, tokens, source)
    }
}

impl WordSequenceExpr {
    // Constructors

    /// Construct a new sequence with a [`Word`] at the beginning of the operation list.
    pub fn any_capitalization_of(word: &'static str) -> WordSequenceExpr {
        WordSequenceExpr {
            seq: SequenceExpr::any_capitalization_of(word),
        }
    }

    /// Construct a new sequence with an initial [expression](Expr).
    pub fn with(expr: impl Expr + 'static) -> Self {
        Self {
            seq: SequenceExpr::default().then(expr),
        }
    }

    // Builder methods

    /// Push an [expression](Expr) to the operation list.
    pub fn then(mut self, expr: impl Expr + 'static) -> Self {
        self.seq = self.seq.t_ws();
        self.seq = self.seq.then(expr);
        self
    }

    /// Appends the steps in `other` onto the end of `self`.
    /// This is more efficient than [`Self::then`] because it avoids pointer redirection.
    pub fn then_seq(mut self, other: SequenceExpr) -> Self {
        self.seq = self.seq.t_ws();
        self.seq = self.seq.then_seq(other);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::WordSequenceExpr;
    use crate::expr::{ExprExt, FixedPhrase};
    use crate::patterns::WordSet;
    use crate::{Document, Span};

    trait SpanVecExt {
        fn to_strings(&self, doc: &Document) -> Vec<String>;
    }

    impl SpanVecExt for Vec<Span> {
        fn to_strings(&self, doc: &Document) -> Vec<String> {
            self.iter()
                .map(|sp| {
                    doc.get_tokens()[sp.start..sp.end]
                        .iter()
                        .map(|tok| doc.get_span_content_str(&tok.span))
                        .collect::<String>()
                })
                .collect()
        }
    }

    #[test]
    fn matches_foo() {
        let doc = Document::new_markdown_default_curated("foo bar");
        let matches = WordSequenceExpr::any_capitalization_of("foo").iter_matches_in_doc(&doc)
            .collect::<Vec<_>>();

        assert_eq!(matches.to_strings(&doc), vec!["foo"]);
    }

    #[test]
    fn matches_foo_bar() {
        let doc = Document::new_markdown_default_curated("foo bar");
        let matches = WordSequenceExpr::any_capitalization_of("foo").then(WordSequenceExpr::any_capitalization_of("bar")).iter_matches_in_doc(&doc)
            .collect::<Vec<_>>();

        assert_eq!(matches.to_strings(&doc), vec!["foo bar"]);
    }

    #[test]
    fn matches_rise_rises_rising_rised_rose_the_question() {
        let wexpr = WordSequenceExpr::with(WordSet::new(&["rise", "rises", "rising", "rised", "rose"]))
            .then(FixedPhrase::from_phrase("the question"));

        let doc = Document::new_markdown_default_curated("I rised the question to Emax Support and they just came back to me inmediately with the below response.");

        let matches = wexpr
            .iter_matches_in_doc(&doc)
            .collect::<Vec<_>>();

        assert_eq!(matches.to_strings(&doc), vec!["rised the question"]);

        let doc = Document::new_markdown_default_curated("Because, my initial point in rising the question at all mainly was - adding cache control is (very) handy");

        let matches = wexpr
            .iter_matches_in_doc(&doc)
            .collect::<Vec<_>>();

        assert_eq!(matches.to_strings(&doc), vec!["rising the question"]);
    }
}