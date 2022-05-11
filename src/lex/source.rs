pub struct Source<'a> {
    pub buff: &'a str
}

impl<'a> Source<'a> {
    pub fn at(&self, offset: usize) -> char {
		// pub fn at(&self, offset: usize) -> Option<(char, usize)> {
        // self.buff[offset..].chars().nth(0).map(|ch| { (ch, offset + ch.len_utf8()) })
		self.buff[offset..].chars().nth(0).unwrap()
    }

    pub fn slice(&self, start: usize, end: usize) -> &'a str {
        &self.buff[start..end]
    }
}