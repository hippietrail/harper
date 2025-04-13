use super::{Lint, LintKind, Linter, Suggestion};
use crate::{Dialect, Dictionary, Document, TokenStringExt};

/// Detects nonstandard tenses, negations, and other forms.
pub struct NonstandardForms<T>
where
    T: Dictionary,
{
    dictionary: T,
    dialect: Dialect,
}

impl<T: Dictionary> NonstandardForms<T> {
    pub fn new(dictionary: T, dialect: Dialect) -> Self {
        Self { dictionary, dialect }
    }
    
    fn check_suffix(
        &mut self,
        lints: &mut Vec<Lint>,
        word: &crate::Token,
        word_str: &str,
        suffix: &str,
        stem_len: usize,
        is_verb: bool,
        message: impl Fn(&str) -> String,
    ) {
        if word_str.ends_with(suffix) {
            let stem = &word_str[..word_str.len() - stem_len];
            if let Some(metadata) = self.dictionary.get_word_metadata_str(stem) {
                if (is_verb && metadata.is_verb()) || (!is_verb && metadata.is_noun()) {
                    lints.push(Lint {
                        span: word.span,
                        lint_kind: LintKind::WordChoice,
                        message: message(stem),
                        suggestions: vec![],
                        ..Default::default()
                    });
                }
            }
        }
    }

    fn past(&mut self, lints: &mut Vec<Lint>, word: &crate::Token, word_str: &String) {
        if word_str.len() >= 3 && word_str.ends_with('d') {
            self.check_suffix(
                lints,
                word,
                word_str,
                "ed",
                2,
                true,
                |stem| format!("The past of `{}` is probably irregular and not with -ed", stem),
            );
            self.check_suffix(
                lints,
                word,
                word_str,
                "d",
                1,
                true,
                |stem| format!("The past of `{}` is probably irregular and not with -d", stem),
            );
        }
    }

    fn plural(&mut self, lints: &mut Vec<Lint>, word: &crate::Token, word_str: &String) {
        if word_str.len() >= 3 && word_str.ends_with('s') {
            self.check_suffix(
                lints,
                word,
                word_str,
                "es",
                2,
                false,
                |stem| format!("The plural of `{}` may be irregular or it may have no plural form.", stem),
            );
            self.check_suffix(
                lints,
                word,
                word_str,
                "s",
                1,
                false,
                |stem| format!("The plural of `{}` may be irregular or it may have no plural form.", stem),
            );
        }
    }

    fn negative(&mut self, lints: &mut Vec<Lint>, word: &crate::Token, word_str: &String) {
        let negative_prefixes = ["un", "de", "dis", "non", "in"];
        
        for prefix in negative_prefixes {
            if word_str.len() >= prefix.len() + 1 && word_str.starts_with(prefix) {
                let stem = &word_str[prefix.len()..];
                if let Some(_metadata) = self.dictionary.get_word_metadata_str(stem) {
                    lints.push(Lint {
                        span: word.span,
                        lint_kind: LintKind::WordChoice,
                        message: format!("That's not the negative form of this word. It has a {}- prefix.", prefix),
                        suggestions: vec![],
                        ..Default::default()
                    });
                }
            }
        }
    }
}

impl<T: Dictionary> Linter for NonstandardForms<T> {
    fn lint(&mut self, document: &Document) -> Vec<Lint> {
        let mut lints = Vec::new();

        for word in document.iter_words() {
            if let Some(_word) = word.kind.as_word().unwrap() {
                continue;
            }
            // 1. Is it a nonstandard past tense? eated? runned? thinked? etc. ated? ranned? thoughted? etc.
            // 2. Does it have a nonstandard negative prefix? destandard? disstandard? unstandard? etc.
            // 3. Is it a nonstandard plural? sheeps? childs? etc. informations? knowledges? etc.

            let word_text = document.get_span_content(&word.span);
            let word_str = word_text.iter().collect::<String>();
    
            self.past(&mut lints, word, &word_str);
            self.negative(&mut lints, word, &word_str);
            self.plural(&mut lints, word, &word_str);

            let sug = "suggles!";
            let msg = "meggles!";
            let suggestions = vec![Suggestion::ReplaceWith(sug.chars().collect())];
            let message = msg.to_owned();

            lints.push(Lint {
                span: word.span,
                lint_kind: LintKind::WordChoice,
                suggestions,
                message,
                ..Default::default()
            });
        }

        lints
    }

    fn description(&self) -> &'static str {
        "Detects nonstandard tenses, negations, and other forms."
    }
}

#[cfg(test)]
mod tests {
    use crate::{linting::tests::{assert_lint_count, assert_suggestion_result}, Dialect, FstDictionary};

    use super::NonstandardForms;

    #[test]
    fn markdown_capitalized() {
        assert_suggestion_result(
            "I thinked about the informations.",
            NonstandardForms::new(FstDictionary::curated(), Dialect::American),
            "I thought about the information.",
        );
    }
}
