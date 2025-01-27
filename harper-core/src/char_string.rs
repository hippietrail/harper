use smallvec::SmallVec;

/// A char sequence that improves cache locality.
/// Most English words are fewer than 12 characters.
pub type CharString = SmallVec<[char; 12]>;

/// Extensions to character sequences that make them easier to wrangle.
pub trait CharStringExt {
    fn to_lower(&self) -> CharString;
    fn to_string(&self) -> String;
}

impl CharStringExt for [char] {
    fn to_lower(&self) -> CharString {
        let mut out = CharString::with_capacity(self.len());

        out.extend(self.iter().flat_map(|v| v.to_lowercase()));

        out
    }
    fn to_string(&self) -> String {
        self.iter().collect()
    }
}
