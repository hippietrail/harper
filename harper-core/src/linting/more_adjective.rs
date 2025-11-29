use itertools::Itertools;

use crate::expr::{Expr, SequenceExpr};
use crate::linting::{ExprLinter, LintKind, Suggestion, expr_linter::Chunk};
use crate::spell::Dictionary;
use crate::{CharStringExt, Lint, Token, TokenStringExt};

pub struct MoreAdjective<D> {
    expr: Box<dyn Expr>,
    dict: D,
}

impl<D> MoreAdjective<D>
where
    D: Dictionary,
{
    pub fn new(dict: D) -> Self {
        Self {
            expr: Box::new(
                SequenceExpr::word_set(&["more", "most"])
                    .t_ws()
                    .then_positive_adjective(),
            ),
            dict,
        }
    }
}

impl<D> ExprLinter for MoreAdjective<D>
where
    D: Dictionary,
{
    type Unit = Chunk;

    fn expr(&self) -> &dyn Expr {
        self.expr.as_ref()
    }

    fn match_to_lint(&self, toks: &[Token], src: &[char]) -> Option<Lint> {
        // eprintln!("🍏 '{}'", toks.span()?.get_content_string(src));
        // Check invariants just in case the Expr changes
        if toks.len() != 3 || !toks[1].kind.is_whitespace() || !toks[2].kind.is_positive_adjective()
        {
            return None;
        }

        let phrase_span = toks.span()?;

        enum Degree {
            Comparative,
            Superlative,
        }

        let degree_tok = &toks[0];
        let degree_span = degree_tok.span;
        let degree_chars = degree_span.get_content(src);
        let degree_str = degree_span.get_content_string(src);

        let degree = if degree_chars.eq_ignore_ascii_case_str("more") {
            Degree::Comparative
        } else if degree_chars.eq_ignore_ascii_case_str("most") {
            Degree::Superlative
        } else {
            return None;
        };

        let ending = match degree {
            Degree::Comparative => "er",
            Degree::Superlative => "est",
        };

        let adj_tok = &toks[2];
        let adj_span = adj_tok.span;
        let adj_chars = adj_span.get_content(src);
        let adj_str = adj_span.get_content_string(src);

        if adj_chars.len() < 2 {
            return None;
        }

        let mut candidates: Vec<String> = vec![];

        // Only a handful of adjectives are irregular
        // if adj_str == "good" {
        let candidate = match adj_str.as_str() {
            "good" => match degree {
                Degree::Comparative => Some("better"),
                Degree::Superlative => Some("best"),
            },
            _ => None,
        };
        if let Some(candidate) = candidate {
            eprintln!("🌈 '{} {}' 🔜 '{}'", degree_str, adj_str, candidate);
            candidates.push(candidate.to_string());
        }

        // Just add the ending: smart -> smarter/smartest
        let candidate = format!("{}{}", adj_str, ending);
        if let Some(metadata) = self.dict.get_word_metadata_str(&candidate)
            && (metadata.is_comparative_adjective() || metadata.is_superlative_adjective())
        {
            eprintln!("🍊 '{} {}' 🔜 '{}'", degree_str, adj_str, candidate);
            candidates.push(candidate);
        }

        // Double consonant: big -> bigger/biggest
        let penult = adj_chars[adj_chars.len() - 2];
        let last = adj_chars[adj_chars.len() - 1];
        let vowels = ['a', 'e', 'i', 'o', 'u'];
        if vowels.contains(&penult) && !vowels.contains(&last) {
            let candidate = format!("{}{}{}", adj_str, last, ending);
            if let Some(metadata) = self.dict.get_word_metadata_str(&candidate)
                && (metadata.is_comparative_adjective() || metadata.is_superlative_adjective())
            {
                eprintln!("🍎 '{} {}' 🔜 '{}'", degree_str, adj_str, candidate);
                candidates.push(candidate);
            }
        }

        if last == 'y' {
            // smelly -> smellier/smelliest
            let candidate = format!(
                "{}i{}",
                &adj_chars[0..adj_chars.len() - 1].iter().collect::<String>(),
                ending
            );
            if let Some(metadata) = self.dict.get_word_metadata_str(&candidate)
                && (metadata.is_comparative_adjective() || metadata.is_superlative_adjective())
            {
                eprintln!("🎾 '{} {}' 🔜 '{}'", degree_str, adj_str, candidate);
                candidates.push(candidate);
            }
        } else if last == 'e' {
            // cute -> cuter/cutest
            let candidate = format!(
                "{}{}",
                &adj_chars[0..adj_chars.len() - 1].iter().collect::<String>(),
                ending
            );
            if let Some(metadata) = self.dict.get_word_metadata_str(&candidate)
                && (metadata.is_comparative_adjective() || metadata.is_superlative_adjective())
            {
                eprintln!("🍋 '{} {}' 🔜 '{}'", degree_str, adj_str, candidate);
                candidates.push(candidate);
            }
        }

        if candidates.is_empty() {
            return None;
        }

        let suggestions = candidates
            .iter()
            .map(|c| {
                Suggestion::replace_with_match_case(
                    c.chars().collect_vec(),
                    phrase_span.get_content(src), // template - char slice
                )
            })
            .collect::<Vec<Suggestion>>();

        Some(Lint {
            span: toks.span()?,
            // Not `LintKind::Style` or `LintKind::Usage` since those can imply that the inflected form
            // is always preferred over the two-word phrase. `LintKind::WordChoice` leaves it up to you.
            lint_kind: LintKind::WordChoice,
            suggestions,
            message: "This is not an error, but an inflected form of this adjective also exists"
                .to_string(),
            ..Default::default()
        })
    }

    fn description(&self) -> &str {
        "Looks for comparative adjective constructions with `more` than could use inflected forms."
    }
}
