use crate::config::{CodeActionConfig, DiagnosticSeverity};
use crate::diagnostics::{lint_to_code_actions, lints_to_diagnostics};
use crate::pos_conv::range_to_span;
use harper_core::linting::{Lint, LintGroup, Linter};
use harper_core::{Document, IgnoredLints, MergedDictionary, MutableDictionary, TokenKind};
use harper_core::{Lrc, Token};
use tower_lsp::lsp_types::{CodeActionOrCommand, Command, Diagnostic, Range, Url};

pub struct DocumentState {
    pub document: Document,
    pub ident_dict: Lrc<MutableDictionary>,
    pub dict: Lrc<MergedDictionary>,
    pub linter: LintGroup,
    pub language_id: Option<String>,
    pub ignored_lints: IgnoredLints,
    pub url: Url,
}

impl DocumentState {
    pub fn ignore_lint(&mut self, lint: &Lint) {
        self.ignored_lints.ignore_lint(lint, &self.document);
    }

    pub fn generate_diagnostics(&mut self, severity: DiagnosticSeverity) -> Vec<Diagnostic> {
        let temp = self.linter.config.clone();
        self.linter.config.fill_with_curated();

        let mut lints = self.linter.lint(&self.document);

        self.linter.config = temp;

        self.ignored_lints
            .remove_ignored(&mut lints, &self.document);

        lints_to_diagnostics(self.document.get_full_content(), &lints, severity)
    }

    /// Generate code actions results for a selected area.
    pub fn generate_code_actions(
        &mut self,
        range: Range,
        code_action_config: &CodeActionConfig,
    ) -> Vec<CodeActionOrCommand> {
        let temp = self.linter.config.clone();
        self.linter.config.fill_with_curated();

        let mut lints = self.linter.lint(&self.document);

        self.linter.config = temp;

        self.ignored_lints
            .remove_ignored(&mut lints, &self.document);

        lints.sort_by_key(|l| l.priority);

        let source_chars = self.document.get_full_content();

        // Find lints whole span overlaps with range
        let span = range_to_span(source_chars, range).with_len(1);

        let mut actions: Vec<CodeActionOrCommand> = lints
            .into_iter()
            .filter(|lint| lint.span.overlaps_with(span))
            .flat_map(|lint| {
                let foo: Vec<CodeActionOrCommand> = lint_to_code_actions(&lint, &self.url, source_chars, code_action_config);

                foo
            })
            .collect();

        actions.dedup_by(|a, b| {
            let a_is_command = matches!(a, CodeActionOrCommand::Command(_));
            let b_is_command = matches!(b, CodeActionOrCommand::Command(_));
            if !a_is_command || !b_is_command {
                // return true;
                // if we always return true like above, that means that...
                // All code actions will be deduplicated (only one will remain)
                // Commands will never be deduplicated
                
                // // but we don't want **all** code actions to be deduped, only matching ones
                // let act1 = match a {
                //     CodeActionOrCommand::CodeAction(cmd) => cmd,
                //     _ => return false,
                //     // _ => return true,
                // };
                // let act2 = match b {
                //     CodeActionOrCommand::CodeAction(cmd) => cmd,
                //     // _ => return false,
                //     _ => return true,
                // };
                // // return act1.title == act2.title; // that doesn't work
                // // return true;
                // return false;
                // let mut found_a_match = false;
                if let (CodeActionOrCommand::CodeAction(a), CodeActionOrCommand::CodeAction(b)) = (a, b) {
                    let ae = a.edit.clone().unwrap().changes;
                    let be = b.edit.clone().unwrap().changes;
                    let acoll = ae.unwrap();
                    let bcoll = be.unwrap();
                    // bloody hell! these are also collections of collections!!
                    // so let's go through all the text edits in the all the vecs
                    for (filea, editsa) in acoll.iter() {
                        for edita in editsa.iter() {
                            for (fileb, editsb) in bcoll.iter() {
                                for editb in editsb.iter() {
                                    if filea == fileb || edita.new_text == editb.new_text {
                                        // found_a_match = true;
                                        // break;
                                        return true;
                                    }
                                }
                            }
                        }
                    }
                    // return found_a_match;
                }
            }
            
            false
        });
        // actions.dedup_by(|a, b| {
        //     match (a, b) {
        //         (CodeActionOrCommand::CodeAction(ca), CodeActionOrCommand::CodeAction(cb)) => {
        //             if ca.title == cb.title {
        //                 // Compare the edit actions too to ensure we only deduplicate if they do the same thing
        //                 match (ca.edit.as_ref(), cb.edit.as_ref()) {
        //                     (Some(edit_a), Some(edit_b)) => {
        //                         if let Some(changes_a) = edit_a.changes.as_ref() {
        //                             if let Some(changes_b) = edit_b.changes.as_ref() {
        //                                 // Compare the actual text changes
        //                                 if changes_a.len() == changes_b.len() {
        //                                     for (file_a, edits_a) in changes_a {
        //                                         if let Some(edits_b) = changes_b.get(file_a) {
        //                                             if edits_a.len() == edits_b.len() {
        //                                                 for (edit_a, edit_b) in edits_a.iter().zip(edits_b.iter()) {
        //                                                     if edit_a.new_text != edit_b.new_text {
        //                                                         return false;
        //                                                     }
        //                                                 }
        //                                                 return true;
        //                                             }
        //                                         }
        //                                     }
        //                                 }
        //                             }
        //                         }
        //                     }
        //                     _ => false,
        //                 }
        //             }
        //             false
        //         }
        //         (CodeActionOrCommand::Command(ca), CodeActionOrCommand::Command(cb)) => 
        //             ca.title == cb.title,
        //         _ => false,
        //     }
        // });

        if let Some(Token {
            kind: TokenKind::Url,
            span,
            ..
        }) = self.document.get_token_at_char_index(span.start)
        {
            actions.push(CodeActionOrCommand::Command(Command::new(
                "Open URL".to_string(),
                "HarperOpen".to_string(),
                Some(vec![self.document.get_span_content_str(span).into()]),
            )))
        }

        actions
    }
}

impl Default for DocumentState {
    fn default() -> Self {
        Self {
            document: Default::default(),
            ident_dict: Default::default(),
            dict: Default::default(),
            linter: Default::default(),
            language_id: Default::default(),
            ignored_lints: Default::default(),
            url: Url::parse("https://example.net").unwrap(),
        }
    }
}
