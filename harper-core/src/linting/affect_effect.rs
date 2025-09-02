use crate::{
    CharStringExt, Document, Token, TokenStringExt,
    linting::{Lint, LintKind, Linter, Suggestion},
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PosStruct {
    pub c: bool,  // conjunction
    pub i: bool,  // pronoun (I)
    pub d: bool,  // determiner
    pub j: bool,  // adjective
    pub n: bool,  // noun
    pub p: bool,  // preposition
    pub r: bool,  // adverb (R for adverb, since A is already used for adjective)
    pub v: bool,  // verb
}

impl PosStruct {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn to_string(&self) -> String {
        let mut s = String::with_capacity(8);
        if self.v { s.push('V'); }
        if self.n { s.push('N'); }
        if self.r { s.push('R'); }
        if self.j { s.push('J'); }
        if self.p { s.push('P'); }
        if self.i { s.push('I'); }
        if self.d { s.push('D'); }
        if self.c { s.push('C'); }
        s
    }
    
    // Bitwise OR operation
    pub fn or(self, other: Self) -> Self {
        Self {
            c: self.c || other.c,
            i: self.i || other.i,
            d: self.d || other.d,
            j: self.j || other.j,
            n: self.n || other.n,
            p: self.p || other.p,
            r: self.r || other.r,
            v: self.v || other.v,
        }
    }
    
    // Bitwise AND operation
    pub fn and(self, other: Self) -> Self {
        Self {
            c: self.c && other.c,
            i: self.i && other.i,
            d: self.d && other.d,
            j: self.j && other.j,
            n: self.n && other.n,
            p: self.p && other.p,
            r: self.r && other.r,
            v: self.v && other.v,
        }
    }
    
    // Check if any of the flags are set
    pub fn any(self) -> bool {
        self.c || self.i || self.d || self.j || self.n || self.p || self.r || self.v
    }
    
    // Check if all flags are set
    pub fn all(self) -> bool {
        self.c && self.i && self.d && self.j && self.n && self.p && self.r && self.v
    }
}

// Implement BitOr and BitAnd for nice syntax
impl std::ops::BitOr for PosStruct {
    type Output = Self;
    
    fn bitor(self, rhs: Self) -> Self::Output {
        self.or(rhs)
    }
}

impl std::ops::BitAnd for PosStruct {
    type Output = Self;
    
    fn bitand(self, rhs: Self) -> Self::Output {
        self.and(rhs)
    }
}

#[derive(Debug, Default)]
pub struct AffectEffect;

impl Linter for AffectEffect {
    fn lint(&mut self, document: &Document) -> Vec<Lint> {
        let mut output = Vec::new();

        for chunk in document.iter_chunks() {
            // for tok in chunk.iter_words() {
            for wix in chunk.iter_word_indices() {
                let tok = &chunk[wix];

                if !tok.kind.is_verb() && !tok.kind.is_noun() {
                    continue;
                }
                // < len of "seem" or > len of "affect" + "ing"
                if tok.span.len() < 4 || tok.span.len() > 6 + 3 {
                    continue;
                }
                let word = tok.span.get_content(document.get_source());
                if !word.eq_any_ignore_ascii_case_str(&[
                    "affect",
                    "affected",
                    "affects",
                    "affecting",
                    "effect",
                    "effected",
                    "effecting",
                    "effects",
                    "seam",
                    "seamed",
                    "seams",
                    "seaming",
                    "seem",
                    "seemed",
                    "seeming",
                    "seems",
                ]) {
                    continue;
                }

                enum Stem {
                    Æffect,
                    Seæm,
                }

                let (stem, stem_len) = match word.first() {
                    Some(&'a' | &'A' | &'e' | &'E') => (Stem::Æffect, 6),
                    Some(&'s' | &'S') => (Stem::Seæm, 4),
                    _ => continue,
                };

                let toks = chunk.widen_slice(wix, 2);

                if toks.len() == 5 {
                    let first_tok = toks.first().and_then(|t| t.kind.as_word());
                    let last_tok = toks.last().and_then(|t| t.kind.as_word());

                    if let (Some(Some(first)), Some(Some(last))) = (first_tok, last_tok) {
                        let first_txt = tokpos(first); //format!("{:#?}", first);
                        let last_txt = tokpos(last); //format!("{:#?}", last);
                        // eprintln!("{}", first_txt);
                        if let Some(span) = toks.span() {
                            eprintln!(
                                "❤️ {} '{}' {}",
                                first_txt,
                                span.get_content_string(document.get_source()),
                                last_txt
                            );
                        }
                        // eprintln!("{}", last_txt);
                    }
                } else {
                    if let Some(span) = toks.span() {
                        eprintln!(
                            "💔 [{}] {}",
                            toks.len(),
                            span.get_content_string(document.get_source())
                        );
                    }
                }

                let ending = &tok.span.get_content(document.get_source())[stem_len..];

                let with_ending = match stem {
                    Stem::Æffect => vec!['æ', 'f', 'f', 'e', 'c', 't'],
                    Stem::Seæm => vec!['s', 'e', 'æ', 'm'],
                }
                .into_iter()
                .chain(ending.iter().copied())
                .collect::<Vec<_>>();
                let message = format!("Did you mean `{}`?", with_ending.iter().collect::<String>());

                output.push(Lint {
                    span: tok.span,
                    lint_kind: LintKind::Spelling,
                    suggestions: vec![Suggestion::replace_with_match_case(with_ending, word)],
                    message,
                    priority: 63,
                })
            }
        }

        output
    }

    fn description(&self) -> &'static str {
        "Fixes mix-ups between `affect` and `effect`."
    }
}

use crate::WordMetadata;

fn tokpos(tok: &WordMetadata) -> PosStruct {
    // build a string such as VNP (meaning verb & noun & preposition)
    let mut pos = PosStruct::new();
    if tok.is_conjunction() {
        pos.c = true;
    }
    if tok.is_pronoun() {
        pos.i = true;
    }
    if tok.is_determiner() {
        pos.d = true;
    }
    if tok.is_adjective() {
        pos.j = true;
    }
    if tok.is_noun() {
        pos.n = true;
    }
    if tok.preposition {
        pos.p = true;
    }
    if tok.is_adverb() {
        pos.r = true;
    }
    if tok.is_verb() {
        pos.v = true;
    }
    pos
}

#[cfg(test)]
mod tests {
    use super::AffectEffect;

    // legit affect - verb - lemma

    #[test]
    fn all_legit_affect_sentences() {
        use crate::Document;
        use crate::linting::Linter;

        let sentences = [
            "every code change might affect anything else",
            "probably you’ll never be aware which are your slowest code parts under real-world scenario and how these affect the UX",
            "How do null values affect performance?",
            "changes made by this category only affect things that use [Schannel SSP]",
            "you might want to be aware of the options that can affect performance",
            "but can sometimes affect gameplay too",
            "Preview how CSS3 border-radius values affect an element",
            "This modification may temporarily affect network connectivity",
            "Modifying this registry key may affect:",
        ];

        for s in sentences {
            AffectEffect.lint(&Document::new_markdown_default_curated(s));
        }
    }

    #[test]
    fn all_bad_affect_sentences() {
        use crate::Document;
        use crate::linting::Linter;

        let sentences = [
            "Since, raw response is the only option for unstable search commands Val() and Result() calls wouldn't have any affect on them:",
        ];

        for s in sentences {
            AffectEffect.lint(&Document::new_markdown_default_curated(s));
        }
    }

    // legit affected - verb - past participle

    #[test]
    fn all_legit_affected_sentences() {
        use crate::Document;
        use crate::linting::Linter;

        let sentences = [
            "Local RDP such as for Hyper-V enhanced session is not affected.",
            "your help will be immensely valuable for the people who were affected by this tragedy",
        ];

        for s in sentences {
            AffectEffect.lint(&Document::new_markdown_default_curated(s));
        }
    }

    // legit affects - verb - 3rd person singular

    #[test]
    fn all_legit_affects_sentences() {
        use crate::Document;
        use crate::linting::Linter;

        let sentences = [
            "This value directly affects the execution time of this workflow",
            "This affects me as well, with version 2015.01.01.",
        ];

        for s in sentences {
            AffectEffect.lint(&Document::new_markdown_default_curated(s));
        }
    }

    // legit affecting - verb - present participle

    #[test]
    fn all_legit_affecting_sentences() {
        use crate::Document;
        use crate::linting::Linter;

        let sentences = [
            "The Microsoft Security Response Center (MSRC) investigates all reports of security vulnerabilities affecting Microsoft products and services",
        ];

        for s in sentences {
            AffectEffect.lint(&Document::new_markdown_default_curated(s));
        }
    }

    // legit effects - noun - plural

    #[test]
    fn all_legit_effects_sentences() {
        use crate::Document;
        use crate::linting::Linter;

        let sentences = [
            "Avoid effects outside of functions.",
            "Avoid putting code with effects like network or DB calls outside of functions.",
            "side effects influence your application",
            "so that the store can run the reducer and effects, and you can observe state changes in the store",
            "The reducer is also responsible for returning any effects that should be",
            "current state to the next state, and what effects need to be executed",
            "execute effects, and they can return `.none`",
            "And we can immediately test this logic, including the effects, without",
            "Create specialized effects and transformations for video generation",
        ];

        for s in sentences {
            AffectEffect.lint(&Document::new_markdown_default_curated(s));
        }
    }

    // legit effect - noun - singular

    #[test]
    fn all_legit_effect_sentences() {
        use crate::Document;
        use crate::linting::Linter;

        let sentences = [
            "They do not have any effect on security.",
            "seeing the source code won't have any effect on you because you aren't able to understand nor verify it.",
            "Setting the variables after sourcing the script will have no effect",
            "if a step causes an effect to be executed",
            "Currently our reducer is using an effect that reaches out into the real world to hit an API server",
            "please redeploy the project for the changes to take effect.",
            "in effect, forming one big layer",
        ];

        for s in sentences {
            AffectEffect.lint(&Document::new_markdown_default_curated(s));
        }
    }

    // legit seem - verb - lemma

    #[test]
    fn all_legit_seem_sentences() {
        use crate::Document;
        use crate::linting::Linter;

        let sentences = [
            "If you find something which doesn't make sense, or something doesn't seem right, please make a pull request",
            "Ubuntu packages do not seem to get updated anymore.",
            "Some of our users seem to think there is a limit of issues they can or should open.",
            "Do not post features because they seem like a good idea.",
        ];

        for s in sentences {
            AffectEffect.lint(&Document::new_markdown_default_curated(s));
        }
    }

    // legit seems - verb - 3rd person singular

    #[test]
    fn all_legit_seems_sentences() {
        use crate::Document;
        use crate::linting::Linter;

        let sentences = [
            "it seems to be able to handle a bit of general tasks such as",
            "What seems like a bug might be intended behaviour.",
            "Even if it seems self-evident, please",
            "(seems discontinued)",
        ];

        for s in sentences {
            AffectEffect.lint(&Document::new_markdown_default_curated(s));
        }
    }

    // legit seemed - verb - past

    #[test]
    fn all_legit_seemed_sentences() {
        use crate::Document;
        use crate::linting::Linter;

        let sentences = [
            "These libraries were placed here because none of the other categories seemed to fit.",
        ];

        for s in sentences {
            AffectEffect.lint(&Document::new_markdown_default_curated(s));
        }
    }
}
