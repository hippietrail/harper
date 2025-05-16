use super::{Lint, Linter};
use crate::{Document, TokenStringExt};

#[derive(Debug, Default)]
pub struct ApostrophePlurals;

impl Linter for ApostrophePlurals {
    fn lint(&mut self, document: &Document) -> Vec<Lint> {
        let mut lints = Vec::new();

        for tok in document.iter_words() {
            let wch = document.get_span_content(&tok.span);
            let len = wch.len();
            if len >= 2 && wch[len-1] == 's' || wch[len-1] == 'S' {
                if len >= 3 && wch[len-2] == '\'' || wch[len-2] == '’' {
                    eprintln!("¥apos¥{}¥", wch.iter().collect::<String>());
                } else {
                    eprintln!("¥plur¥{}¥", wch.iter().collect::<String>());
                }
            }
        }

        lints
    }

    fn description(&self) -> &'static str {
        "Looks for apostrophes used in words that should be plurals."
    }
}

#[cfg(test)]
mod tests {
    use super::ApostrophePlurals;
    use crate::{ linting::tests::assert_lint_count};

    #[test]
    fn apostrophes_are_hard() {
        assert_lint_count("Apostrophe's are hard!", ApostrophePlurals, 1);
    }

    #[test]
    fn not_even_a_word() {
        assert_lint_count(
        "fafiueb usb't a wurd",
        ApostrophePlurals,
        99);
    }
}