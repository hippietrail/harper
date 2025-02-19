use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

use super::an_a::AnA;
use super::avoid_curses::AvoidCurses;
use super::boring_words::BoringWords;
use super::capitalize_personal_pronouns::CapitalizePersonalPronouns;
use super::chock_full::ChockFull;
use super::closed_compounds::Desktop;
use super::closed_compounds::Furthermore;
use super::closed_compounds::Laptop;
use super::closed_compounds::Overnight;
use super::closed_compounds::{
    Anybody, Anyhow, Anywhere, Backplane, Devops, Everywhere, Henceforth, However, Insofar,
    Instead, Intact, Into, Itself, Middleware, Misunderstand, Misunderstood, Misuse, Misused,
    Multicore, Multimedia, Multithreading, Myself, Nonetheless, Nothing, Notwithstanding, Overall,
    Overclocking, Overload, Postpone, Proofread, Regardless, Somebody, Somehow, Somewhere,
    Therefore, Thereupon, Underclock, Upset, Upward, Whereupon, Widespread, Worldwide,
};
use super::compound_nouns::CompoundNouns;
use super::correct_number_suffix::CorrectNumberSuffix;
use super::despite_of::DespiteOf;
use super::dot_initialisms::DotInitialisms;
use super::ellipsis_length::EllipsisLength;
use super::hereby::Hereby;
use super::hop_hope::HopHope;
use super::hyphenate_number_day::HyphenateNumberDay;
use super::left_right_hand::LeftRightHand;
use super::lets_confusion::LetsConfusion;
use super::likewise::Likewise;
use super::linking_verbs::LinkingVerbs;
use super::long_sentences::LongSentences;
use super::matcher::Matcher;
use super::merge_words::MergeWords;
use super::multiple_sequential_pronouns::MultipleSequentialPronouns;
use super::nobody::Nobody;
use super::number_suffix_capitalization::NumberSuffixCapitalization;
use super::out_of_date::OutOfDate;
use super::pique_interest::PiqueInterest;
use super::plural_conjugate::PluralConjugate;
use super::possessive_your::PossessiveYour;
use super::pronoun_contraction::PronounContraction;
use super::proper_noun_capitalization_linters::DayOneNames;
use super::proper_noun_capitalization_linters::JetpackNames;
use super::proper_noun_capitalization_linters::PocketCastsNames;
use super::proper_noun_capitalization_linters::TumblrNames;
use super::proper_noun_capitalization_linters::{
    AmazonNames, Americas, AppleNames, Australia, AzureNames, Canada, ChineseCommunistParty,
    GoogleNames, Holidays, Koreas, Malaysia, MetaNames, MicrosoftNames, UnitedOrganizations,
};
use super::repeated_words::RepeatedWords;
use super::sentence_capitalization::SentenceCapitalization;
use super::somewhat_something::SomewhatSomething;
use super::spaces::Spaces;
use super::spell_check::SpellCheck;
use super::spelled_numbers::SpelledNumbers;
use super::terminating_conjunctions::TerminatingConjunctions;
use super::that_which::ThatWhich;
use super::then_than::ThenThan;
use super::unclosed_quotes::UnclosedQuotes;
use super::use_genitive::UseGenitive;
use super::was_aloud::WasAloud;
use super::whereas::Whereas;
use super::wordpress_dotcom::WordPressDotcom;
use super::wrong_quotes::WrongQuotes;
use super::Lint;
use super::{CurrencyPlacement, Linter, NoOxfordComma, OxfordComma};
use crate::linting::phrase_corrections;
use crate::Dictionary;
use crate::Document;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(transparent)]
pub struct LintGroupConfig {
    inner: HashMap<String, bool>,
}

impl LintGroupConfig {
    pub fn set_rule_enabled(&mut self, key: impl ToString, val: bool) {
        self.inner.insert(key.to_string(), val);
    }

    /// Remove any configuration attached to a rule.
    /// This allows it to assume its default (curated) state.
    pub fn unset_rule_enabled(&mut self, key: impl AsRef<str>) {
        self.inner.remove_entry(key.as_ref());
    }

    pub fn set_rule_enabled_if_unset(&mut self, key: impl AsRef<str>, val: bool) {
        if self.inner.get(key.as_ref()).is_none() {
            self.set_rule_enabled(key.as_ref().to_string(), val);
        }
    }

    pub fn is_rule_enabled(&self, key: &str) -> bool {
        self.inner.get(key).cloned().unwrap_or(false)
    }

    /// Merge the contents of another [`LintGroupConfig`] into this one.
    /// The other config will be left empty after this operation.
    ///
    /// Conflicting keys will be overridden by the value in the other group.
    pub fn merge_from(&mut self, other: &mut LintGroupConfig) {
        self.inner.extend(other.inner.drain());
    }

    pub fn fill_with_curated_config(&mut self) {
        self.set_rule_enabled_if_unset(stringify!(WordPressDotcom), true);
        self.set_rule_enabled_if_unset(stringify!(DayOneNames), true);
        self.set_rule_enabled_if_unset(stringify!(PocketCastsNames), true);
        self.set_rule_enabled_if_unset(stringify!(TumblrNames), true);
        self.set_rule_enabled_if_unset(stringify!(JetpackNames), true);
        self.set_rule_enabled_if_unset(stringify!(OutOfDate), true);
        self.set_rule_enabled_if_unset(stringify!(Desktop), true);
        self.set_rule_enabled_if_unset(stringify!(Laptop), true);
        self.set_rule_enabled_if_unset(stringify!(ThenThan), true);
        self.set_rule_enabled_if_unset(stringify!(PiqueInterest), true);
        self.set_rule_enabled_if_unset(stringify!(WasAloud), true);
        self.set_rule_enabled_if_unset(stringify!(HyphenateNumberDay), true);
        self.set_rule_enabled_if_unset(stringify!(LeftRightHand), true);
        self.set_rule_enabled_if_unset(stringify!(HopHope), true);
        self.set_rule_enabled_if_unset(stringify!(Furthermore), true);
        self.set_rule_enabled_if_unset(stringify!(Overnight), true);
        self.set_rule_enabled_if_unset(stringify!(Hereby), true);
        self.set_rule_enabled_if_unset(stringify!(Likewise), true);
        self.set_rule_enabled_if_unset(stringify!(CompoundNouns), true);
        self.set_rule_enabled_if_unset(stringify!(Regardless), true);
        self.set_rule_enabled_if_unset(stringify!(Henceforth), true);
        self.set_rule_enabled_if_unset(stringify!(Upward), true);
        self.set_rule_enabled_if_unset(stringify!(Whereupon), true);
        self.set_rule_enabled_if_unset(stringify!(Insofar), true);
        self.set_rule_enabled_if_unset(stringify!(Thereupon), true);
        self.set_rule_enabled_if_unset(stringify!(Nonetheless), true);
        self.set_rule_enabled_if_unset(stringify!(Anyhow), true);
        self.set_rule_enabled_if_unset(stringify!(Notwithstanding), true);
        self.set_rule_enabled_if_unset(stringify!(Widespread), true);
        self.set_rule_enabled_if_unset(stringify!(Multimedia), true);
        self.set_rule_enabled_if_unset(stringify!(Multicore), true);
        self.set_rule_enabled_if_unset(stringify!(Multithreading), true);
        self.set_rule_enabled_if_unset(stringify!(Devops), true);
        self.set_rule_enabled_if_unset(stringify!(Underclock), true);
        self.set_rule_enabled_if_unset(stringify!(Overload), true);
        self.set_rule_enabled_if_unset(stringify!(Backplane), true);
        self.set_rule_enabled_if_unset(stringify!(Overclocking), true);
        self.set_rule_enabled_if_unset(stringify!(Middleware), true);
        self.set_rule_enabled_if_unset(stringify!(Somewhere), true);
        self.set_rule_enabled_if_unset(stringify!(Instead), true);
        self.set_rule_enabled_if_unset(stringify!(Anywhere), true);
        self.set_rule_enabled_if_unset(stringify!(Nothing), true);
        self.set_rule_enabled_if_unset(stringify!(Anybody), true);
        self.set_rule_enabled_if_unset(stringify!(Somebody), true);
        self.set_rule_enabled_if_unset(stringify!(Nobody), true);
        self.set_rule_enabled_if_unset(stringify!(Into), true);
        self.set_rule_enabled_if_unset(stringify!(Proofread), true);
        self.set_rule_enabled_if_unset(stringify!(Somehow), true);
        self.set_rule_enabled_if_unset(stringify!(Intact), true);
        self.set_rule_enabled_if_unset(stringify!(Upset), true);
        self.set_rule_enabled_if_unset(stringify!(Misunderstood), true);
        self.set_rule_enabled_if_unset(stringify!(However), true);
        self.set_rule_enabled_if_unset(stringify!(Overall), true);
        self.set_rule_enabled_if_unset(stringify!(Worldwide), true);
        self.set_rule_enabled_if_unset(stringify!(Postpone), true);
        self.set_rule_enabled_if_unset(stringify!(Misused), true);
        self.set_rule_enabled_if_unset(stringify!(Misuse), true);
        self.set_rule_enabled_if_unset(stringify!(Misunderstand), true);
        self.set_rule_enabled_if_unset(stringify!(Therefore), true);
        self.set_rule_enabled_if_unset(stringify!(Myself), true);
        self.set_rule_enabled_if_unset(stringify!(Itself), true);
        self.set_rule_enabled_if_unset(stringify!(Whereas), true);
        self.set_rule_enabled_if_unset(stringify!(PossessiveYour), true);
        self.set_rule_enabled_if_unset(stringify!(SpelledNumbers), false);
        self.set_rule_enabled_if_unset(stringify!(AnA), true);
        self.set_rule_enabled_if_unset(stringify!(SentenceCapitalization), true);
        self.set_rule_enabled_if_unset(stringify!(UnclosedQuotes), true);
        self.set_rule_enabled_if_unset(stringify!(WrongQuotes), false);
        self.set_rule_enabled_if_unset(stringify!(LongSentences), true);
        self.set_rule_enabled_if_unset(stringify!(RepeatedWords), true);
        self.set_rule_enabled_if_unset(stringify!(Spaces), true);
        self.set_rule_enabled_if_unset(stringify!(Matcher), true);
        self.set_rule_enabled_if_unset(stringify!(CorrectNumberSuffix), true);
        self.set_rule_enabled_if_unset(stringify!(NumberSuffixCapitalization), true);
        self.set_rule_enabled_if_unset(stringify!(MultipleSequentialPronouns), true);
        self.set_rule_enabled_if_unset(stringify!(LinkingVerbs), false);
        self.set_rule_enabled_if_unset(stringify!(AvoidCurses), true);
        self.set_rule_enabled_if_unset(stringify!(TerminatingConjunctions), true);
        self.set_rule_enabled_if_unset(stringify!(EllipsisLength), true);
        self.set_rule_enabled_if_unset(stringify!(DotInitialisms), true);
        self.set_rule_enabled_if_unset(stringify!(BoringWords), false);
        self.set_rule_enabled_if_unset(stringify!(UseGenitive), false);
        self.set_rule_enabled_if_unset(stringify!(ThatWhich), true);
        self.set_rule_enabled_if_unset(stringify!(CapitalizePersonalPronouns), true);
        self.set_rule_enabled_if_unset(stringify!(Americas), true);
        self.set_rule_enabled_if_unset(stringify!(Australia), true);
        self.set_rule_enabled_if_unset(stringify!(Canada), true);
        self.set_rule_enabled_if_unset(stringify!(Koreas), true);
        self.set_rule_enabled_if_unset(stringify!(Malaysia), true);
        self.set_rule_enabled_if_unset(stringify!(ChineseCommunistParty), true);
        self.set_rule_enabled_if_unset(stringify!(UnitedOrganizations), true);
        self.set_rule_enabled_if_unset(stringify!(Holidays), true);
        self.set_rule_enabled_if_unset(stringify!(AmazonNames), true);
        self.set_rule_enabled_if_unset(stringify!(GoogleNames), true);
        self.set_rule_enabled_if_unset(stringify!(MetaNames), true);
        self.set_rule_enabled_if_unset(stringify!(MicrosoftNames), true);
        self.set_rule_enabled_if_unset(stringify!(AppleNames), true);
        self.set_rule_enabled_if_unset(stringify!(AzureNames), true);
        self.set_rule_enabled_if_unset(stringify!(MergeWords), true);
        self.set_rule_enabled_if_unset(stringify!(PluralConjugate), false);
        self.set_rule_enabled_if_unset(stringify!(OxfordComma), true);
        self.set_rule_enabled_if_unset(stringify!(NoOxfordComma), false);
        self.set_rule_enabled_if_unset(stringify!(PronounContraction), true);
        self.set_rule_enabled_if_unset(stringify!(CurrencyPlacement), true);
        self.set_rule_enabled_if_unset(stringify!(SomewhatSomething), true);
        self.set_rule_enabled_if_unset(stringify!(LetsConfusion), true);
        self.set_rule_enabled_if_unset(stringify!(DespiteOf), true);
        self.set_rule_enabled_if_unset(stringify!(ChockFull), true);
        self.set_rule_enabled_if_unset(stringify!(LoAndBehold), true);
        self.set_rule_enabled_if_unset(stringify!(Everywhere), true);
        self.set_rule_enabled("SpellCheck", true);
    }
}

#[derive(Default)]
pub struct LintGroup {
    pub config: LintGroupConfig,
    inner: HashMap<String, Box<dyn Linter>>,
}

impl LintGroup {
    pub fn empty() -> Self {
        Self {
            config: LintGroupConfig::default(),
            inner: HashMap::new(),
        }
    }

    /// Add a [`Linter`] to the group, returning whether the operation was successful.
    /// If it returns `false`, it is because a linter with that key already existed in the group.
    pub fn add(&mut self, name: impl AsRef<str>, linter: Box<dyn Linter>) -> bool {
        if self.inner.contains_key(name.as_ref()) {
            false
        } else {
            self.inner.insert(name.as_ref().to_string(), linter);
            true
        }
    }

    /// Merge the contents of another [`LintGroup`] into this one.
    /// The other lint group will be left empty after this operation.
    pub fn merge_from(&mut self, other: &mut LintGroup) {
        self.config.merge_from(&mut other.config);
        self.inner.extend(other.inner.drain());
    }

    /// Set all contained rules to a specific value.
    /// Passing `None` will unset that rule, allowing it to assume its default state.
    pub fn set_all_rules_to(&mut self, enabled: Option<bool>) {
        for key in self.inner.keys() {
            match enabled {
                Some(v) => self.config.set_rule_enabled(key, v),
                None => self.config.unset_rule_enabled(key),
            }
        }
    }

    pub fn all_descriptions(&self) -> HashMap<&str, &str> {
        self.inner
            .iter()
            .map(|(key, value)| (key.as_str(), value.description()))
            .collect()
    }

    pub fn new_curated(config: LintGroupConfig, dictionary: impl Dictionary + 'static) -> Self {
        let mut out = Self::empty();

        macro_rules! insert_struct_rule {
            ($rule:ident) => {
                out.add(stringify!($rule), Box::new($rule::default()));
            };
        }

        out.merge_from(&mut phrase_corrections::lint_group());

        insert_struct_rule!(WordPressDotcom);
        insert_struct_rule!(DayOneNames);
        insert_struct_rule!(PocketCastsNames);
        insert_struct_rule!(TumblrNames);
        insert_struct_rule!(JetpackNames);
        insert_struct_rule!(OutOfDate);
        insert_struct_rule!(Desktop);
        insert_struct_rule!(Laptop);
        insert_struct_rule!(ThenThan);
        insert_struct_rule!(PiqueInterest);
        insert_struct_rule!(WasAloud);
        insert_struct_rule!(HyphenateNumberDay);
        insert_struct_rule!(LeftRightHand);
        insert_struct_rule!(HopHope);
        insert_struct_rule!(Furthermore);
        insert_struct_rule!(Overnight);
        insert_struct_rule!(Hereby);
        insert_struct_rule!(Likewise);
        insert_struct_rule!(CompoundNouns);
        insert_struct_rule!(Regardless);
        insert_struct_rule!(Henceforth);
        insert_struct_rule!(Upward);
        insert_struct_rule!(Whereupon);
        insert_struct_rule!(Insofar);
        insert_struct_rule!(Thereupon);
        insert_struct_rule!(Nonetheless);
        insert_struct_rule!(Anyhow);
        insert_struct_rule!(Notwithstanding);
        insert_struct_rule!(Widespread);
        insert_struct_rule!(Multimedia);
        insert_struct_rule!(Multicore);
        insert_struct_rule!(Multithreading);
        insert_struct_rule!(Devops);
        insert_struct_rule!(Underclock);
        insert_struct_rule!(Overload);
        insert_struct_rule!(Backplane);
        insert_struct_rule!(Overclocking);
        insert_struct_rule!(Middleware);
        insert_struct_rule!(Somewhere);
        insert_struct_rule!(Instead);
        insert_struct_rule!(Anywhere);
        insert_struct_rule!(Nothing);
        insert_struct_rule!(Anybody);
        insert_struct_rule!(Somebody);
        insert_struct_rule!(Nobody);
        insert_struct_rule!(Into);
        insert_struct_rule!(Proofread);
        insert_struct_rule!(Somehow);
        insert_struct_rule!(Intact);
        insert_struct_rule!(Upset);
        insert_struct_rule!(Misunderstood);
        insert_struct_rule!(However);
        insert_struct_rule!(Overall);
        insert_struct_rule!(Worldwide);
        insert_struct_rule!(Postpone);
        insert_struct_rule!(Misused);
        insert_struct_rule!(Misuse);
        insert_struct_rule!(Misunderstand);
        insert_struct_rule!(Therefore);
        insert_struct_rule!(Myself);
        insert_struct_rule!(Itself);
        insert_struct_rule!(Whereas);
        insert_struct_rule!(PossessiveYour);
        insert_struct_rule!(SpelledNumbers);
        insert_struct_rule!(AnA);
        insert_struct_rule!(SentenceCapitalization);
        insert_struct_rule!(UnclosedQuotes);
        insert_struct_rule!(WrongQuotes);
        insert_struct_rule!(LongSentences);
        insert_struct_rule!(RepeatedWords);
        insert_struct_rule!(Spaces);
        insert_struct_rule!(Matcher);
        insert_struct_rule!(CorrectNumberSuffix);
        insert_struct_rule!(NumberSuffixCapitalization);
        insert_struct_rule!(MultipleSequentialPronouns);
        insert_struct_rule!(LinkingVerbs);
        insert_struct_rule!(AvoidCurses);
        insert_struct_rule!(TerminatingConjunctions);
        insert_struct_rule!(EllipsisLength);
        insert_struct_rule!(DotInitialisms);
        insert_struct_rule!(BoringWords);
        insert_struct_rule!(UseGenitive);
        insert_struct_rule!(ThatWhich);
        insert_struct_rule!(CapitalizePersonalPronouns);
        insert_struct_rule!(Americas);
        insert_struct_rule!(Australia);
        insert_struct_rule!(Canada);
        insert_struct_rule!(Koreas);
        insert_struct_rule!(Malaysia);
        insert_struct_rule!(ChineseCommunistParty);
        insert_struct_rule!(UnitedOrganizations);
        insert_struct_rule!(Holidays);
        insert_struct_rule!(AmazonNames);
        insert_struct_rule!(GoogleNames);
        insert_struct_rule!(MetaNames);
        insert_struct_rule!(MicrosoftNames);
        insert_struct_rule!(AppleNames);
        insert_struct_rule!(AzureNames);
        insert_struct_rule!(MergeWords);
        insert_struct_rule!(PluralConjugate);
        insert_struct_rule!(OxfordComma);
        insert_struct_rule!(NoOxfordComma);
        insert_struct_rule!(PronounContraction);
        insert_struct_rule!(CurrencyPlacement);
        insert_struct_rule!(SomewhatSomething);
        insert_struct_rule!(LetsConfusion);
        insert_struct_rule!(DespiteOf);
        insert_struct_rule!(ChockFull);
        insert_struct_rule!(Everywhere);

        out.add("SpellCheck", Box::new(SpellCheck::new(dictionary)));

        out.config = config;

        out
    }
}

impl Linter for LintGroup {
    fn lint(&mut self, document: &Document) -> Vec<Lint> {
        let mut results = Vec::new();

        let mut config = self.config.clone();
        config.fill_with_curated_config();

        for (key, linter) in &mut self.inner {
            if config.is_rule_enabled(key) {
                results.extend(linter.lint(document));
            }
        }

        results
    }

    fn description(&self) -> &str {
        "A collection of linters that can be run as one."
    }
}

#[cfg(test)]
mod tests {
    use crate::{linting::Linter, Document, FstDictionary, MutableDictionary};

    use super::{LintGroup, LintGroupConfig};

    #[test]
    fn can_get_all_descriptions() {
        let group =
            LintGroup::new_curated(LintGroupConfig::default(), MutableDictionary::default());
        group.all_descriptions();
    }

    #[test]
    fn lint_descriptions_are_clean() {
        let mut group =
            LintGroup::new_curated(LintGroupConfig::default(), FstDictionary::curated());
        let pairs: Vec<_> = group
            .all_descriptions()
            .into_iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect();

        for (key, value) in pairs {
            let doc = Document::new_markdown_default_curated(&value);
            eprintln!("{key}: {value}");

            if !group.lint(&doc).is_empty() {
                dbg!(&group.lint(&doc));
                panic!();
            }
        }
    }
}
