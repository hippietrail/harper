use harper_core::{CharStringExt, Mask, Masker, Span};

pub struct LiterateHaskellMasker {
    pub text: bool,
    pub code: bool,
}

impl Masker for LiterateHaskellMasker {
    fn create_mask(&mut self, source: &[char]) -> harper_core::Mask {
        let mut mask = Mask::new_blank();

        let mut location = 0;
        let mut in_code_env = false;
        let mut last_line_blank = false;

        for line in source.split(|c| *c == '\n') {
            let string_form = line.to_string();
            let trimmed = string_form.trim();
            let line_is_bird = line.first().map_or(false, |c| *c == '>');

            // Code fencing
            let code_start = trimmed == r"\begin{code}" || (last_line_blank && line_is_bird);
            let code_end = trimmed == r"\end{code}" || trimmed.is_empty();
            if (!in_code_env && code_start) || (in_code_env && code_end) {
                in_code_env = !in_code_env;
            }

            let end_loc = location + line.len();
            if (!in_code_env && self.text) || (in_code_env && self.code) {
                mask.push_allowed(Span::new(location, end_loc));
            }

            location = end_loc + 1; // +1 for the newline split on
            last_line_blank = trimmed.is_empty();
        }

        mask
    }
}
