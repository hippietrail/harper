use crate::{
    CharStringExt,
    Document,
    TokenKind,
    TokenStringExt, // Add TokenStringExt
    linting::{Lint, Linter},
};

use std::fmt;

#[derive(Debug)]
struct LineData {
    prev_chars: String,
    prev_pos: String,
    target: String,
    next_chars: String,
    next_pos: String,
}

impl LineData {
    fn new(
        prev_chars: &[char],
        prev_pos: String,
        target: &[char],
        next_chars: &[char],
        next_pos: String,
    ) -> Self {
        Self {
            prev_chars: prev_chars.iter().collect(),
            prev_pos,
            target: target.iter().collect(),
            next_chars: next_chars.iter().collect(),
            next_pos,
        }
    }
}

impl fmt::Display for LineData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let left_side = format!("| {}.{} |", self.prev_chars, self.prev_pos);
        let right_side = format!("| {}.{} |", self.next_chars, self.next_pos);
        let target = format!(" {} ", self.target);

        // Calculate the width of each part
        let left_width = left_side.chars().count();
        let target_width = target.chars().count();
        let right_width = right_side.chars().count();

        // Calculate the total width needed (left + target + right)
        let total_width = left_width + target_width + right_width;

        // Calculate the center position of the target
        let target_center = left_width + target_width / 2;
        let desired_center = 40; // Adjust this value to set the center column

        // Calculate the padding needed to center the target
        let padding = if target_center < desired_center {
            desired_center - target_center
        } else {
            0
        };

        // Build the line with proper padding
        write!(
            f,
            "{:padding$}{} {}{}",
            "",
            left_side,
            target,
            right_side,
            padding = padding,
        )
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ThingThink;

impl Linter for ThingThink {
    fn lint(&mut self, doc: &Document) -> Vec<Lint> {
        // Changed back to &mut self
        let mut lines = Vec::new();

        // First pass: collect all lines
        for sentoks in doc.iter_sentences() {
            sentoks.iter_noun_indices().for_each(|ii| {
                let chars = &sentoks[ii].span.get_content(doc.get_source());
                if chars.eq_ignore_ascii_case_chars(&['t', 'h', 'i', 'n', 'g']) {
                    let mut prev_chars = &['x'][..];
                    let mut next_chars = &['x'][..];
                    let mut prev_pos = String::new();
                    let mut next_pos = String::new();

                    if let Some(prev_word) = doc.get_next_word_from_offset(ii, -1) {
                        prev_chars = &prev_word.span.get_content(doc.get_source());
                        prev_pos = getpos(&prev_word.kind);
                    }
                    if let Some(next_word) = doc.get_next_word_from_offset(ii, 1) {
                        next_chars = &next_word.span.get_content(doc.get_source());
                        next_pos = getpos(&next_word.kind);
                    }

                    lines.push(LineData::new(
                        prev_chars, prev_pos, chars, next_chars, next_pos,
                    ));
                }
            });
        }

        // Print aligned output
        for line in &lines {
            eprintln!("{}", line);
        }

        Vec::new()
    }

    fn description(&self) -> &'static str {
        "This linter checks for [thing](cci:1://file:///Users/hippietrail/harper-the-second/harper/harper-core/src/linting/thing_think.rs:145:4-202:5) mistakenly used as the verb [think](cci:1://file:///Users/hippietrail/harper-the-second/harper/harper-core/src/linting/thing_think.rs:145:4-202:5)"
    }
}

fn getpos(prev_word: &TokenKind) -> String {
    let checks = [
        ("N", TokenKind::is_noun as fn(&TokenKind) -> bool),
        ("V", TokenKind::is_verb),
        ("J", TokenKind::is_adjective),
        ("R", TokenKind::is_adverb),
        ("D", TokenKind::is_determiner),
        ("I", TokenKind::is_pronoun),
        ("P", TokenKind::is_preposition),
        ("C", TokenKind::is_conjunction),
    ];

    let pos: String = checks
        .iter()
        .filter_map(|(flag, check_fn)| {
            if check_fn(prev_word) {
                Some(*flag)
            } else {
                None
            }
        })
        .collect();

    match pos.len() {
        0 => "?".to_string(),
        1 => pos,
        _ => format!("[{pos}]"),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Document,
        linting::{Linter, ThingThink},
    };

    #[test]
    #[ignore = "this is actually a tool, not a test"]
    fn test_thing_think() {
        let text = r#"
Whad do you thing about tinygo?
bcz i thing hugging face embeddings and models are very complex
one should always thing of whether the efforts are better targeted to the improvement
terraform will always thing it has changed
As I always thing in 'sets of devices of the same kind'
which information we thing to be missing
So far we really like it featurewise and we thing it is really ...
On second thought, I thing my ROIs are off
One thing that I sometimes thing would be nice is if I could make different instances
But when I think about composition, I often thing about Bartok. And when I think about performance, I often thing about Coltrane.
When working with workflows on many forms I often thing I need to do the same over and over
Sub builders don't always thing about ...
otherwise without it I thing I would have to pass it as parameter
I thing documentation is not enough and I have to do a lot of things with Superset.
The exe file dosen't work allways, because antivirus can thing it is a virus.
I thing lowering fog density and lowed detail spread somewhat mitigates the issue
I thing something going wrong with permission.
... they thing something is a good idea.
I thing it's faster to just type the date
I thing something like “Controversial topics” would be much ...
Only thing i can thing of is 'Number' or 'Number of Bits', but that doesn't look right
Trying to thing about it a bit more, its the same issue with all the initial props.
Here I'm trying to thing about the following questions:
Fix comparison of config.xml strings that would sometimes thing XMLs were different in cases when they were not.
I thing this is not the correct thread to this discussion but to have a library that supports your request ...
I thing it could be related with the way stardist ...
Reason I thing this is useful is when I tried scramble on my vapor hosted site, it would fail.
When I thing about it again, after writing this, Yii3 does not feel like Yii anymore
I thing Addons are the solution, with addons you can include
I thing idea of these panels is nonsense anyway.
I always thing that less is more
It's good practice to always thing in the context of the data
I thing you are doing same way
I thing my issue is the combination of several technologies:
I thing I found a way to achieve it, not the best one, but it should work.
I always thing the orthographic view in Cura looks weird.
I always thing that ebook2audiobookxtts means the old version
I always thing listening to and including the voices of those you are. trying to help is necessary.
        "#;
        for (i, line) in text.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let doc = Document::new_plain_english_curated(line);
            let mut linter = ThingThink;
            let lints = linter.lint(&doc);

            for lint in lints {
                eprintln!("  {:?}", lint);
            }
        }
    }
}
