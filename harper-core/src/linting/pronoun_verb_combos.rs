use crate::{
    Lint, Token, TokenKind,
    expr::{Expr, FirstMatchOf, SequenceExpr},
    linting::{ExprLinter, debug::format_lint_match, expr_linter::Chunk},
};

type Toks<'a> = &'a [Token];
type Chars<'a> = &'a [char];
type Ctx<'a> = Option<(&'a [Token], &'a [Token])>;
type OptLint = Option<Lint>;

// Combinations of (person_predicate_fn, number, case, label)
// We exhaustively list all valid combinations for proper pronoun-verb agreement testing.
// Each combination generates a subexpression that matches pronoun + whitespace + verb form.
type PersonPredicate = fn(&TokenKind) -> bool;

struct PronounCombination {
    person_pred: PersonPredicate,
    number: &'static str, // "singular" or "plural"
    case: &'static str,   // "subject" or "object"
    label: &'static str,  // for debugging
}

const COMBINATIONS: &[PronounCombination] = &[
    // First person singular (e.g., "I", "me")
    PronounCombination {
        person_pred: TokenKind::is_first_person_singular_pronoun,
        number: "singular",
        case: "subject",
        label: "1sg.subj",
    },
    PronounCombination {
        person_pred: TokenKind::is_first_person_singular_pronoun,
        number: "singular",
        case: "object",
        label: "1sg.obj",
    },
    // First person plural (e.g., "we", "us")
    PronounCombination {
        person_pred: TokenKind::is_first_person_plural_pronoun,
        number: "plural",
        case: "subject",
        label: "1pl.subj",
    },
    PronounCombination {
        person_pred: TokenKind::is_first_person_plural_pronoun,
        number: "plural",
        case: "object",
        label: "1pl.obj",
    },
    // Second person singular (e.g., "you")
    PronounCombination {
        person_pred: TokenKind::is_second_person_pronoun,
        number: "singular",
        case: "subject",
        label: "2sg.subj",
    },
    PronounCombination {
        person_pred: TokenKind::is_second_person_pronoun,
        number: "singular",
        case: "object",
        label: "2sg.obj",
    },
    // Second person plural (e.g., "you all", "y'all")
    PronounCombination {
        person_pred: TokenKind::is_second_person_pronoun,
        number: "plural",
        case: "subject",
        label: "2pl.subj",
    },
    PronounCombination {
        person_pred: TokenKind::is_second_person_pronoun,
        number: "plural",
        case: "object",
        label: "2pl.obj",
    },
    // Third person singular (e.g., "he", "she", "it", "him", "her")
    PronounCombination {
        person_pred: TokenKind::is_third_person_singular_pronoun,
        number: "singular",
        case: "subject",
        label: "3sg.subj",
    },
    PronounCombination {
        person_pred: TokenKind::is_third_person_singular_pronoun,
        number: "singular",
        case: "object",
        label: "3sg.obj",
    },
    // Third person plural (e.g., "they", "them")
    PronounCombination {
        person_pred: TokenKind::is_third_person_plural_pronoun,
        number: "plural",
        case: "subject",
        label: "3pl.subj",
    },
    PronounCombination {
        person_pred: TokenKind::is_third_person_plural_pronoun,
        number: "plural",
        case: "object",
        label: "3pl.obj",
    },
];

pub struct PronounVerbCombos {
    expr: Box<dyn Expr>,
}

impl Default for PronounVerbCombos {
    fn default() -> Self {
        // Define the verb forms we want to check
        type VerbPredicate = fn(&TokenKind) -> bool;

        struct VerbForm {
            pred: VerbPredicate,
            label: &'static str,
        }

        const VERB_FORMS: &[VerbForm] = &[
            VerbForm {
                pred: TokenKind::is_verb_lemma,
                label: "lemma",
            },
            VerbForm {
                pred: TokenKind::is_verb_past_form,
                label: "past",
            },
            VerbForm {
                pred: TokenKind::is_verb_simple_past_form,
                label: "simple_past",
            },
            VerbForm {
                pred: TokenKind::is_verb_third_person_singular_present_form,
                label: "3sg_present",
            },
            VerbForm {
                pred: TokenKind::is_verb_progressive_form,
                label: "progressive",
            },
            VerbForm {
                pred: TokenKind::is_verb_past_participle_form,
                label: "past_participle",
            },
        ];

        // Generate all combinations of pronouns and verb forms
        let exprs = COMBINATIONS
            .iter()
            .flat_map(|pron_combo| {
                VERB_FORMS.iter().map(move |verb_form| {
                    let person_pred = pron_combo.person_pred;
                    let number_pred = match pron_combo.number {
                        "singular" => TokenKind::is_singular_pronoun,
                        "plural" => TokenKind::is_plural_pronoun,
                        _ => unreachable!(),
                    };

                    let case_pred = match pron_combo.case {
                        "subject" => TokenKind::is_subject_pronoun,
                        "object" => TokenKind::is_object_pronoun,
                        _ => unreachable!(),
                    };

                    let verb_pred = verb_form.pred;

                    // Create a sequence: pronoun + whitespace + verb form
                    let pronoun_expr = SequenceExpr::with_kind_where(move |k| {
                        person_pred(k) && number_pred(k) && case_pred(k)
                    });

                    // Add whitespace after pronoun
                    let ws_expr = SequenceExpr::with_kind_where(TokenKind::is_whitespace);

                    // Add verb form
                    let verb_expr = SequenceExpr::with_kind_where(verb_pred);

                    // Combine them: pronoun + whitespace + verb
                    let combined = pronoun_expr.then(ws_expr).then(verb_expr);

                    Box::new(combined) as Box<dyn Expr>
                })
            })
            .collect();

        Self {
            expr: Box::new(FirstMatchOf::new(exprs)),
        }
    }
}

impl ExprLinter for PronounVerbCombos {
    type Unit = Chunk;

    fn expr(&self) -> &dyn Expr {
        self.expr.as_ref()
    }

    fn description(&self) -> &str {
        "Developer tool to find all combinations of personal pronoun + verb form."
    }

    fn match_to_lint_with_context<'a>(&self, t: Toks<'a>, s: Chars<'a>, c: Ctx<'a>) -> OptLint {
        eprintln!("🤯 {}", format_lint_match(t, c, s));
        None
    }
}
