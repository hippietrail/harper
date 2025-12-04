use crate::expr::{Expr, SequenceExpr};
use crate::{Span, Token};

#[derive(Default)]
pub struct NominalPhrase;

impl Expr for NominalPhrase {
    fn run(&self, cursor: usize, tokens: &[Token], source: &[char]) -> Option<Span<Token>> {
        // Match a noun phrase (optional determiner + optional adjectives + one or more nouns)
        let noun_phrase = SequenceExpr::default()
            .then_optional(SequenceExpr::default().then_determiner().t_ws())
            .then_zero_or_more(SequenceExpr::default().then_adjective().t_ws())
            .then(
                SequenceExpr::default().then_noun().then_zero_or_more(
                    SequenceExpr::default()
                        .t_ws()
                        .then(SequenceExpr::default().then_noun()),
                ),
            );
        let nominal_phrase = SequenceExpr::any_of(vec![
            Box::new(noun_phrase),
            Box::new(SequenceExpr::default().then_pronoun()),
        ]);

        nominal_phrase.run(cursor, tokens, source)
    }
}

#[cfg(test)]
mod tests {
    use super::NominalPhrase;
    use crate::Document;
    use crate::expr::ExprExt;
    use crate::linting::tests::SpanVecExt;

    #[test]
    fn thing() {
        let doc = Document::new_markdown_default_curated("thing");
        let matches = NominalPhrase.iter_matches_in_doc(&doc).collect::<Vec<_>>();

        assert_eq!(matches.to_strings(&doc), vec!["thing"])
    }

    #[test]
    fn a_thing() {
        let doc = Document::new_markdown_default_curated("a thing");
        let matches = NominalPhrase.iter_matches_in_doc(&doc).collect::<Vec<_>>();

        assert_eq!(matches.to_strings(&doc), vec!["a thing"])
    }

    #[test]
    fn red_thing() {
        let doc = Document::new_markdown_default_curated("red thing");
        let matches = NominalPhrase.iter_matches_in_doc(&doc).collect::<Vec<_>>();

        assert_eq!(matches.to_strings(&doc), vec!["red thing"])
    }

    #[test]
    fn big_red_thing() {
        let doc = Document::new_markdown_default_curated("big red thing");
        let matches = NominalPhrase.iter_matches_in_doc(&doc).collect::<Vec<_>>();

        assert_eq!(matches.to_strings(&doc), vec!["big red thing"])
    }

    #[test]
    fn a_red_thing() {
        let doc = Document::new_markdown_default_curated("a red thing");
        let matches = NominalPhrase.iter_matches_in_doc(&doc).collect::<Vec<_>>();

        assert_eq!(matches.to_strings(&doc), vec!["a red thing"])
    }

    #[test]
    fn a_big_red_thing() {
        let doc = Document::new_markdown_default_curated("a big red thing");
        let matches = NominalPhrase.iter_matches_in_doc(&doc).collect::<Vec<_>>();

        assert_eq!(matches.to_strings(&doc), vec!["a big red thing"])
    }

    #[test]
    fn test_present_participle_and_plural() {
        let doc = Document::new_markdown_default_curated("the falling rocks");
        let matches = NominalPhrase.iter_matches_in_doc(&doc).collect::<Vec<_>>();

        assert_eq!(matches.to_strings(&doc), vec!["the falling rocks"])
    }

    #[test]
    fn test_gerund() {
        let doc = Document::new_markdown_default_curated("a spate of vomiting");
        let matches = NominalPhrase.iter_matches_in_doc(&doc).collect::<Vec<_>>();

        assert_eq!(matches.to_strings(&doc), vec!["a spate", "vomiting"])
    }

    #[test]
    fn test_compound_nouns() {
        let doc = Document::new_markdown_default_curated(
            "the new car park next to the old train station",
        );
        let matches = NominalPhrase.iter_matches_in_doc(&doc).collect::<Vec<_>>();

        assert_eq!(
            matches.to_strings(&doc),
            vec!["the new car park", "the old train station"]
        )
    }

    #[test]
    fn test_pronouns() {
        let doc = Document::new_markdown_default_curated("Me, myself, and I.");
        let matches = NominalPhrase.iter_matches_in_doc(&doc).collect::<Vec<_>>();

        assert_eq!(matches.to_strings(&doc), vec!["Me", "myself", "I"])
    }

    #[test]
    fn test_noun_and_pronoun() {
        let doc = Document::new_markdown_default_curated("Me and my dog.");
        let matches = NominalPhrase.iter_matches_in_doc(&doc).collect::<Vec<_>>();

        assert_eq!(matches.to_strings(&doc), vec!["Me", "my dog"])
    }

    // From the `NominalPhrase` `Pattern`

    #[test]
    fn simple_apple() {
        let doc = Document::new_markdown_default_curated("A red apple");
        let matches = NominalPhrase.iter_matches_in_doc(&doc).collect::<Vec<_>>();

        assert_eq!(matches.to_strings(&doc), vec!["A red apple"])
    }

    #[test]
    fn complex_apple() {
        let doc = Document::new_markdown_default_curated("A red apple with a long stem");
        let matches = NominalPhrase.iter_matches_in_doc(&doc).collect::<Vec<_>>();

        assert_eq!(matches.to_strings(&doc), vec!["A red apple", "a long stem"])
    }

    #[test]
    fn list_fruit() {
        let doc = Document::new_markdown_default_curated("An apple, a banana and a pear");
        let matches = NominalPhrase.iter_matches_in_doc(&doc).collect::<Vec<_>>();

        assert_eq!(
            matches.to_strings(&doc),
            vec!["An apple", "a banana", "a pear"]
        )
    }

    #[test]
    fn simplest_banana() {
        let doc = Document::new_markdown_default_curated("a banana");
        assert!(NominalPhrase.iter_matches_in_doc(&doc).next().is_some());
    }

    #[test]
    fn food() {
        let doc = Document::new_markdown_default_curated(
            "My favorite foods are pizza, sushi, tacos and burgers.",
        );
        let matches = NominalPhrase.iter_matches_in_doc(&doc).collect::<Vec<_>>();

        dbg!(&matches);
        dbg!(matches.to_strings(&doc));

        for span in &matches {
            let gc = span
                .to_char_span(doc.get_tokens())
                .get_content(doc.get_source());
            dbg!(gc);
        }

        assert_eq!(
            matches.to_strings(&doc),
            vec!["My favorite foods", "pizza", "sushi", "tacos", "burgers"]
        )
    }

    #[test]
    fn simplest_way() {
        let doc = Document::new_markdown_default_curated("a way");
        assert!(NominalPhrase.iter_matches_in_doc(&doc).next().is_some());
    }

    #[test]
    fn present_participle_way() {
        let doc = Document::new_markdown_default_curated("a winning way");
        assert!(NominalPhrase.iter_matches_in_doc(&doc).next().is_some());
    }

    #[test]
    fn perfect_participle_way() {
        let doc = Document::new_markdown_default_curated("a failed way");
        assert!(NominalPhrase.iter_matches_in_doc(&doc).next().is_some());
    }
}
