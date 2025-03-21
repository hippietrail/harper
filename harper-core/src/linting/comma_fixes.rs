//use itertools::Itertools;

use super::{Lint, LintKind, Linter, Suggestion};
use crate::{Token, TokenKind, TokenStringExt};

/// A linter that fixes common comma errors such as no space after, erroneous
///  space before, etc, Asian commas instead of English commas, etc.
#[derive(Debug, Default)]
pub struct CommaFixes;

impl Linter for CommaFixes {
    fn lint(&mut self, document: &crate::Document) -> Vec<Lint> {
        let mut lints = Vec::new();

        let source = document.get_source();

        for ci in document.iter_comma_indices() {
            let mut toks = (None, None, document.get_token(ci).unwrap(), None, None);
            toks.0 = (ci >= 2).then(|| document.get_token(ci - 2).unwrap());
            toks.1 = (ci >= 1).then(|| document.get_token(ci - 1).unwrap());
            toks.3 = document.get_token(ci + 1);
            toks.4 = document.get_token(ci + 2);


            let comma_kind = toks.2.span.get_content(source).first().unwrap();

            let mut need_to_remove_space_before_comma = false;
            let mut need_to_fix_comma = false;
            let mut need_to_add_space_after_comma = false;

            if comma_kind != &',' {
                need_to_fix_comma = true;
            }
            
            // if let Some(token) = toks.1 {
            //     if matches!(token.kind, TokenKind::Word(_)) {
            //         println!("foo");
            //     }
            // }
            
            // if let (Some(prev), Some(curr)) = (toks.0, toks.1) {
            //     if matches!(curr.kind, TokenKind::Space(_)) && matches!(prev.kind, TokenKind::Word(_)) {
            //         println!("bar");
            //     }
            // }

            // ?w,␣w => ok
            // w␣,␣w => replace ␣, with ,   == replace [1,2]    ,
            // ?w,w? => replace , with ,␣   == replace [2]      ,␣
            // w␣,w? => replace ␣, with ,␣  == replace [1,2]    ,␣
            // ?w@␣w => replace @ with ,    == replace [2]      ,
            // w␣@␣w => replace ␣@ with ,   == replace [1,2]    ,
            // ?w@w? => replace @ with ,␣   == replace [2]      ,␣
            // w␣@w? => replace ␣@ with ,␣  == replace [1,2]    ,␣

            let made_msg = make_message(source, toks, *comma_kind);
            println!("{}", made_msg);

            match (toks.0, toks.1, comma_kind, toks.3, toks.4) {
                // ?w,␣w => ok
                (None | Some(_), Some(t1_w), ',', Some(t3_s), Some(t4_w))
                    if matches!(t1_w.kind, TokenKind::Word(_))
                    && matches!(t3_s.kind, TokenKind::Space(_))
                    && matches!(t4_w.kind, TokenKind::Word(_)) => {
                        println!("`?w,␣w` -> word comma space word -> ok");
                        continue;
                    },

                // ?w@␣w => ok
                (None | Some(_), Some(t1_w), _, Some(t3_s), Some(t4_w))
                    if matches!(t1_w.kind, TokenKind::Word(_))
                    && matches!(t3_s.kind, TokenKind::Space(_))
                    && matches!(t4_w.kind, TokenKind::Word(_)) => {
                        println!("`?w@␣w` -> word asian comma space word -> replace @ with ,");
                        need_to_fix_comma = true;
                    },

                // w␣,␣w => replace ␣, with ,   == replace [1,2]    ,
                (Some(t0_w), Some(t1_s), ',', Some(t3_s), Some(t4_w))
                    if matches!(t0_w.kind, TokenKind::Word(_))
                    && matches!(t1_s.kind, TokenKind::Space(_))
                    && matches!(t3_s.kind, TokenKind::Space(_))
                    && matches!(t4_w.kind, TokenKind::Word(_)) => {
                        println!("`w␣,␣w` -> word space comma space word -> replace ␣, with ,");
                        need_to_remove_space_before_comma = true;
                    },

                // w␣@␣w => replace ␣@ with ,␣  == replace [1,2]    ,␣
                (Some(t0_w), Some(t1_s), _, Some(t3_s), Some(t4_w))
                    if matches!(t0_w.kind, TokenKind::Word(_))
                    && matches!(t1_s.kind, TokenKind::Space(_))
                    && matches!(t3_s.kind, TokenKind::Space(_))
                    && matches!(t4_w.kind, TokenKind::Word(_)) => {
                        println!("`w␣@␣w` -> word space asian comma space word -> replace ␣@ with ,");
                        need_to_remove_space_before_comma = true;
                        need_to_fix_comma = true;
                    },

                // ?w,w? => replace , with ,␣   == replace [2]      ,␣
                (None | Some(_), Some(t1_w), ',', Some(t3_w), None | Some(_))
                    if matches!(t1_w.kind, TokenKind::Word(_))
                    && matches!(t3_w.kind, TokenKind::Word(_)) => {
                        println!("`?w,w?` -> word comma word -> replace , with ,␣");
                        need_to_add_space_after_comma = true;
                    },

                // ?w@w? => replace @ with ,␣   == replace [2]      ,␣
                (None | Some(_), Some(t1_w), _, Some(t3_w), None | Some(_))
                    if matches!(t1_w.kind, TokenKind::Word(_))
                    && matches!(t3_w.kind, TokenKind::Word(_)) => {
                        println!("`?w@w?` -> word asian comma word -> replace @ with ,");
                        need_to_fix_comma = true;
                        need_to_add_space_after_comma = true;
                    },

                // w␣,w? => replace ␣, with ,␣  == replace [1,2]    ,␣
                (Some(t0_w), Some(t1_s), ',', Some(t3_w), None | Some(_))
                    if matches!(t0_w.kind, TokenKind::Word(_))
                    && matches!(t1_s.kind, TokenKind::Space(_))
                    && matches!(t3_w.kind, TokenKind::Word(_)) => {
                        println!("`w␣,w?` -> word space comma word -> replace ␣, with ,");
                        need_to_remove_space_before_comma = true;
                    },

                // w␣@w? => replace ␣@ with ,␣  == replace [1,2]    ,␣
                (Some(t0_w), Some(t1_s), _, Some(t3_w), None | Some(_))
                    if matches!(t0_w.kind, TokenKind::Word(_))
                    && matches!(t1_s.kind, TokenKind::Space(_))
                    && matches!(t3_w.kind, TokenKind::Word(_)) => {
                        println!("`w␣@w?` -> word space asian comma word -> replace ␣@ with ,");
                        need_to_remove_space_before_comma = true;
                        need_to_fix_comma = true;
                        // let range = (t0_w.span.start, t3_w.span.end);
                        // lints.push(Lint::new(
                        //     LintKind::Miscellaneous,
                        //     range,
                        //     Suggestion::ReplaceWith(vec![',']),
                        // ));
                    },

                _ => {
                    println!("anything else. skipping");
                    continue;
                }
            }

            // println!("remove space before? {}, fix comma? {}, add space after? {}", need_to_remove_space_before_comma, need_to_fix_comma, need_to_add_space_after_comma);

            // let span_to_replace = toks.2.span;

            // let replace_with_this = vec!['«', 'c', 'o', 'm', 'm', 'a', '»'];

            // let msg = make_message(source, toks, *comma_kind);

            // lints.push(Lint {
            //     span: span_to_replace,
            //     lint_kind: LintKind::Formatting,
            //     suggestions: vec![Suggestion::ReplaceWith(replace_with_this)],
            //     message: msg,
            //     priority: 32,
            // })
        }

        lints
    }

    /*fn oldlint(&mut self, document: &crate::Document) -> Vec<Lint> {
        let mut lints: Vec<Lint> = Vec::new();

        for tok in document.iter_commas() {
            let tok_content = document.get_span_content(tok.span);

            if tok_content.is_empty() || tok_content.first().cloned() == Some(',') {
                continue;
            }

            lints.push(Lint {
                span: tok.span,
                lint_kind: LintKind::Formatting,
                suggestions: vec![Suggestion::ReplaceWith(vec![','])],
                message: "Avoid East Asian commas in English contexts.".to_string(),
                priority: 32,
            })
        }

        lints
    }*/

    fn description(&self) -> &'static str {
        "Fix common comma errors such as no space after, erroneous space before, etc, Asian commas instead of English commas, etc."
    }
}

fn make_message(
    source: &[char],
    toks: (
        Option<Token>,
        Option<Token>,
        Token,
        Option<Token>,
        Option<Token>,
    ),
    which_comma: char,
) -> String {
    let mut msg = String::new();

    let tok_str = |tok: Option<Token>| match tok {
        None => "❌".to_string(),
        Some(Token {
            kind: TokenKind::Word(_),
            ..
        }) => "Word".to_string(),
        Some(Token {
            kind: TokenKind::Punctuation(_),
            ..
        }) => "Punc".to_string(),
        Some(Token {
            kind: TokenKind::Decade,
            ..
        }) => "Decade".to_string(),
        Some(Token {
            kind: TokenKind::Number(_),
            ..
        }) => format!(
            "#{}",
            tok.unwrap()
                .span
                .get_content(source)
                .iter()
                .collect::<String>()
        ),
        Some(Token {
            kind: TokenKind::Space(_),
            ..
        }) => "␣".to_string(),
        Some(Token {
            kind: TokenKind::Newline(_),
            ..
        }) => "⏎".to_string(),
        Some(Token {
            kind: TokenKind::EmailAddress,
            ..
        }) => "@".to_string(),
        Some(Token {
            kind: TokenKind::Url,
            ..
        }) => "🔗".to_string(),
        Some(Token {
            kind: TokenKind::Hostname,
            ..
        }) => "Host".to_string(),
        Some(Token {
            kind: TokenKind::Unlintable,
            ..
        }) => "🚫".to_string(),
        Some(Token {
            kind: TokenKind::ParagraphBreak,
            ..
        }) => "¶".to_string(),
        Some(Token {
            kind: TokenKind::Regexish,
            ..
        }) => "[A-Z]".to_string(),
    };

    // add the type of each token to the message
    msg.push_str(&tok_str(toks.0));
    msg.push(' ');
    msg.push_str(&tok_str(toks.1));
    let comma_str = if which_comma == ',' {
        " ●̧ ".to_string()
    } else {
        format!(" ●̧<U+{:04X}> ", which_comma as u32)
    };
    msg.push_str(&comma_str);
    msg.push_str(&tok_str(toks.3));
    msg.push(' ');
    msg.push_str(&tok_str(toks.4));

    msg
}

#[cfg(test)]
mod tests {
    use super::CommaFixes;
    use crate::linting::tests::{assert_lint_count, assert_suggestion_result};

    #[test]
    fn allows_english_comma_atomic() {
        assert_lint_count(",", CommaFixes, 0);
    }

    #[test]
    fn flags_fullwidth_comma_atomic() {
        assert_lint_count("，", CommaFixes, 1);
    }

    #[test]
    fn flags_ideographic_comma_atomic() {
        assert_lint_count("、", CommaFixes, 1);
    }

    #[test]
    fn corrects_fullwidth_comma_real_world() {
        assert_suggestion_result(
            "higher 2 bits of the number of nodes， whether abandoned or not decided by .index section",
            CommaFixes,
            "higher 2 bits of the number of nodes, whether abandoned or not decided by .index section",
        );
    }

    #[test]
    fn corrects_ideographic_comma_real_world() {
        assert_suggestion_result("cout、endl、string", CommaFixes, "cout,endl,string")
    }

    #[test]
    fn word_comma_space_word() {
        assert_lint_count("foo, bar", CommaFixes, 0);
    }

    #[test]
    fn word_fullwidth_comma_space_word() {
        assert_lint_count("foo， bar", CommaFixes, 1);
    }

    #[test]
    fn word_ideographic_comma_space_word() {
        assert_lint_count("foo、 bar", CommaFixes, 1);
    }

    #[test]
    fn word_semicolon_space_word() {
        assert_lint_count("foo; bar", CommaFixes, 0);
    }

    #[test]
    fn word_comma_word() {
        assert_suggestion_result("foo,bar", CommaFixes, "foo, bar")
    }

    #[test]
    fn word_space_comma_word() {
        assert_suggestion_result("foo ,bar", CommaFixes, "foo, bar")
    }

    #[test]
    fn word_space_asian_comma_word() {
        assert_suggestion_result("foo ，bar", CommaFixes, "foo, bar")
    }

    #[test]
    fn word_space_comma_space_word() {
        assert_suggestion_result("foo , bar", CommaFixes, "foo, bar")
    }

    #[test]
    fn word_space_asian_comma_space_word() {
        assert_suggestion_result("foo 、 bar", CommaFixes, "foo, bar")
    }
}
