#![doc = include_str!("../README.md")]

use std::convert::Into;
use std::sync::Arc;

use harper_core::language_detection::is_doc_likely_english;
use harper_core::linting::{LintGroup, LintGroupConfig, Linter as _};
use harper_core::parsers::{IsolateEnglish, Markdown, Parser, PlainEnglish};
use harper_core::{
    remove_overlaps, CharString, Dictionary, Document, FstDictionary, IgnoredLints, Lrc,
    MergedDictionary, MutableDictionary, WordMetadata,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

/// Setup the WebAssembly module's logging.
///
///
/// painful.
#[wasm_bindgen(start)]
pub fn setup() {
    console_error_panic_hook::set_once();

    // If `setup` gets called more than once, we want to allow this error to fall through.
    let _ = tracing_wasm::try_set_as_global_default();
}

macro_rules! make_serialize_fns_for {
    ($name:ident) => {
        #[wasm_bindgen]
        impl $name {
            pub fn to_json(&self) -> String {
                serde_json::to_string(&self).unwrap()
            }

            pub fn from_json(json: String) -> Result<Self, String> {
                serde_json::from_str(&json).map_err(|err| err.to_string())
            }
        }
    };
}

make_serialize_fns_for!(Suggestion);
make_serialize_fns_for!(Lint);
make_serialize_fns_for!(Span);

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum Language {
    Plain,
    Markdown,
}

impl Language {
    fn create_parser(&self) -> Box<dyn Parser> {
        match self {
            Language::Plain => Box::new(PlainEnglish),
            // TODO: Have a way to configure the Markdown parser
            Language::Markdown => Box::new(Markdown::default()),
        }
    }
}

#[wasm_bindgen]
pub struct Linter {
    lint_group: LintGroup<Arc<MergedDictionary>>,
    /// The user-supplied dictionary.
    ///
    /// To make changes affect linting, run [`Self::synchronize_lint_dict`].
    user_dictionary: MutableDictionary,
    dictionary: Arc<MergedDictionary>,
    ignored_lints: IgnoredLints,
}

#[wasm_bindgen]
impl Linter {
    /// Construct a new `Linter`.
    /// Note that this can mean constructing the curated dictionary, which is the most expensive operation
    /// in Harper.
    pub fn new() -> Self {
        let dictionary = Self::construct_merged_dict(MutableDictionary::default());

        Self {
            lint_group: LintGroup::new(LintGroupConfig::default(), dictionary.clone()),
            user_dictionary: MutableDictionary::new(),
            dictionary,
            ignored_lints: IgnoredLints::default(),
        }
    }

    /// Update the dictionary inside [`Self::lint_group`] to include [`Self::user_dictionary`].
    /// This clears any linter caches, so use it sparingly.
    fn synchronize_lint_dict(&mut self) {
        let lint_config = self.lint_group.config;
        self.dictionary = Self::construct_merged_dict(self.user_dictionary.clone());
        self.lint_group = LintGroup::new(lint_config, self.dictionary.clone());
    }

    /// Construct the actual dictionary to be used for linting and parsing from the curated dictionary
    /// and [`Self::user_dictionary`].
    fn construct_merged_dict(user_dictionary: MutableDictionary) -> Arc<MergedDictionary> {
        let mut lint_dict = MergedDictionary::new();

        lint_dict.add_dictionary(FstDictionary::curated());
        lint_dict.add_dictionary(Arc::new(user_dictionary.clone()));

        Arc::new(lint_dict)
    }

    /// Helper method to quickly check if a plain string is likely intended to be English
    pub fn is_likely_english(&self, text: String) -> bool {
        let document = Document::new_plain_english(&text, &self.dictionary);
        is_doc_likely_english(&document, &self.dictionary)
    }

    /// Helper method to remove non-English text from a plain English document.
    pub fn isolate_english(&self, text: String) -> String {
        let document = Document::new(
            &text,
            &IsolateEnglish::new(Box::new(PlainEnglish), self.dictionary.clone()),
            &self.dictionary,
        );

        document.to_string()
    }

    /// Get a JSON map containing the descriptions of all the linting rules.
    pub fn get_lint_descriptions_as_json(&self) -> String {
        serde_json::to_string(&self.lint_group.all_descriptions()).unwrap()
    }

    pub fn get_lint_config_as_json(&self) -> String {
        serde_json::to_string(&self.lint_group.config).unwrap()
    }

    pub fn set_lint_config_from_json(&mut self, json: String) -> Result<(), String> {
        self.lint_group.config = serde_json::from_str(&json).map_err(|v| v.to_string())?;
        Ok(())
    }

    /// Get a Record containing the descriptions of all the linting rules.
    pub fn get_lint_descriptions_as_object(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.lint_group.all_descriptions()).unwrap()
    }

    pub fn get_lint_config_as_object(&self) -> JsValue {
        // Important for downstream JSON serialization
        let serializer = serde_wasm_bindgen::Serializer::json_compatible();

        self.lint_group.config.serialize(&serializer).unwrap()
    }

    pub fn set_lint_config_from_object(&mut self, object: JsValue) -> Result<(), String> {
        self.lint_group.config =
            serde_wasm_bindgen::from_value(object).map_err(|v| v.to_string())?;
        Ok(())
    }

    pub fn ignore_lint(&mut self, lint: Lint) {
        let document = Document::new_from_vec(
            lint.source.into(),
            &lint.language.create_parser(),
            &self.dictionary,
        );

        self.ignored_lints.ignore_lint(&lint.inner, &document);
    }

    /// Perform the configured linting on the provided text.
    pub fn lint(&mut self, text: String, language: Language) -> Vec<Lint> {
        let source: Vec<_> = text.chars().collect();
        let source = Lrc::new(source);

        let parser = language.create_parser();

        let document =
            Document::new_from_vec(source.clone(), &parser, &MutableDictionary::curated());

        let mut lints = self.lint_group.lint(&document);

        remove_overlaps(&mut lints);

        self.ignored_lints.remove_ignored(&mut lints, &document);

        lints
            .into_iter()
            .map(|l| Lint::new(l, source.to_vec(), language))
            .collect()
    }

    /// Export the linter's ignored lints as a privacy-respecting JSON list of hashes.
    pub fn export_ignored_lints(&self) -> String {
        serde_json::to_string(&self.ignored_lints).unwrap()
    }

    /// Import into the linter's ignored lints from a privacy-respecting JSON list of hashes.
    pub fn import_ignored_lints(&mut self, json: String) -> Result<(), String> {
        let list: IgnoredLints = serde_json::from_str(&json).map_err(|err| err.to_string())?;

        self.ignored_lints.append(list);

        Ok(())
    }

    pub fn clear_ignored_lints(&mut self) {
        self.ignored_lints = IgnoredLints::new();
    }

    /// Import words into the dictionary.
    pub fn import_words(&mut self, additional_words: Vec<String>) {
        let init_len = self.user_dictionary.word_count();

        self.user_dictionary
            .extend_words(additional_words.iter().map(|word| {
                (
                    word.chars().collect::<CharString>(),
                    WordMetadata::default(),
                )
            }));

        // Only synchronize if we added words that were not there before.
        if self.user_dictionary.word_count() > init_len {
            self.synchronize_lint_dict();
        }
    }

    /// Export words from the dictionary.
    /// Note: this will only return words previously added by [`Self::import_words`].
    pub fn export_words(&mut self) -> Vec<String> {
        self.user_dictionary
            .words_iter()
            .map(|v| v.iter().collect())
            .collect()
    }
}

impl Default for Linter {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
pub fn to_title_case(text: String) -> String {
    harper_core::make_title_case_str(&text, &PlainEnglish, &FstDictionary::curated())
}

#[wasm_bindgen]
pub fn apply_suggestion(
    text: String,
    span: Span,
    suggestion: &Suggestion,
) -> Result<String, String> {
    let mut source: Vec<_> = text.chars().collect();
    let span: harper_core::Span = span.into();

    suggestion.inner.apply(span, &mut source);

    Ok(source.iter().collect())
}

/// A suggestion to fix a Lint.
#[derive(Debug, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct Suggestion {
    inner: harper_core::linting::Suggestion,
}

/// Tags the variant of suggestion.
#[derive(Debug, Serialize, Deserialize)]
#[wasm_bindgen]
pub enum SuggestionKind {
    /// Replace the problematic text.
    Replace = 0,
    /// Remove the problematic text.
    Remove = 1,
    /// Insert additional text after the error.
    InsertAfter = 2,
}

#[wasm_bindgen]
impl Suggestion {
    pub(crate) fn new(inner: harper_core::linting::Suggestion) -> Self {
        Self { inner }
    }

    /// Get the text that is going to replace the problematic section.
    /// If [`Self::kind`] is `SuggestionKind::Remove`, this will return an empty
    /// string.
    pub fn get_replacement_text(&self) -> String {
        match &self.inner {
            harper_core::linting::Suggestion::Remove => "".to_string(),
            harper_core::linting::Suggestion::ReplaceWith(chars) => chars.iter().collect(),
            harper_core::linting::Suggestion::InsertAfter(chars) => chars.iter().collect(),
        }
    }

    pub fn kind(&self) -> SuggestionKind {
        match &self.inner {
            harper_core::linting::Suggestion::Remove => SuggestionKind::Remove,
            harper_core::linting::Suggestion::ReplaceWith(_) => SuggestionKind::Replace,
            harper_core::linting::Suggestion::InsertAfter(_) => SuggestionKind::InsertAfter,
        }
    }
}

/// An error found in provided text.
///
/// May include zero or more suggestions that may fix the problematic text.
#[derive(Debug, Deserialize, Serialize)]
#[wasm_bindgen]
pub struct Lint {
    inner: harper_core::linting::Lint,
    source: Vec<char>,
    language: Language,
}

#[wasm_bindgen]
impl Lint {
    pub(crate) fn new(
        inner: harper_core::linting::Lint,
        source: Vec<char>,
        language: Language,
    ) -> Self {
        Self {
            inner,
            source,
            language,
        }
    }

    /// Get the content of the source material pointed to by [`Self::span`]
    pub fn get_problem_text(&self) -> String {
        self.inner.span.get_content_string(&self.source)
    }

    /// Get a string representing the general category of the lint.
    pub fn lint_kind(&self) -> String {
        self.inner.lint_kind.to_string_key()
    }

    /// Get a string representing the general category of the lint.
    pub fn lint_kind_pretty(&self) -> String {
        self.inner.lint_kind.to_string()
    }

    /// Equivalent to calling `.length` on the result of `suggestions()`.
    pub fn suggestion_count(&self) -> usize {
        self.inner.suggestions.len()
    }

    /// Get an array of any suggestions that may resolve the issue.
    pub fn suggestions(&self) -> Vec<Suggestion> {
        self.inner
            .suggestions
            .iter()
            .map(|s| Suggestion::new(s.clone()))
            .collect()
    }

    /// Get the location of the problematic text.
    pub fn span(&self) -> Span {
        self.inner.span.into()
    }

    /// Get a description of the error.
    pub fn message(&self) -> String {
        self.inner.message.clone()
    }
}

#[wasm_bindgen]
pub fn get_default_lint_config_as_json() -> String {
    let mut config = LintGroupConfig::default();
    config.fill_default_values();

    serde_json::to_string(&config).unwrap()
}

#[wasm_bindgen]
pub fn get_default_lint_config() -> JsValue {
    let mut config = LintGroupConfig::default();
    config.fill_default_values();

    // Important for downstream JSON serialization
    let serializer = serde_wasm_bindgen::Serializer::json_compatible();

    config.serialize(&serializer).unwrap()
}

/// A struct that represents two character indices in a string: a start and an end.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[wasm_bindgen]
impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        Into::<harper_core::Span>::into(*self).len()
    }
}

impl From<Span> for harper_core::Span {
    fn from(value: Span) -> Self {
        harper_core::Span::new(value.start, value.end)
    }
}

impl From<harper_core::Span> for Span {
    fn from(value: harper_core::Span) -> Self {
        Span::new(value.start, value.end)
    }
}
