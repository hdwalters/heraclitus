use heraclitus::*;

#[derive(Debug)]
pub struct Text {
    value: String
}
impl SyntaxModule<SyntaxMetadata> for Text {
    fn new() -> Self {
        Text { value: format!("") }
    }
    fn parse(&mut self, meta: &mut SyntaxMetadata) -> SyntaxResult {
        let mut word = token_by(meta, |word| word.starts_with('\'') && word.ends_with('\''))?;
        let wordlen = word.len() - 1;
        unsafe {
            self.value = word.get_unchecked_mut(1..wordlen).to_string();
        }
        Ok(())
    }
}