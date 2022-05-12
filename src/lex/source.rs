use std::collections::HashMap;

pub struct Source<'a> {
    pub newlines: Vec<u32>,
    pub buff: &'a str
}

impl<'a> Source<'a> {
    pub fn at(&self, offset: usize) -> char {
		// pub fn at(&self, offset: usize) -> Option<(char, usize)> {
        // self.buff[offset..].chars().nth(0).map(|ch| { (ch, offset + ch.len_utf8()) })
		self.buff[offset..].chars().nth(0).unwrap_or('\0')
    }

    pub fn slice(&self, start: usize, end: usize) -> &'a str {
        //
        &self.buff[start..end]
    }


    pub fn new(contents: String) -> Self {
        let this = Source {
            newlines: vec![],
            buff: &contents,
        };

        this
    }
}

#[derive(Default)]
pub struct Sources<'a> {
    files: HashMap<String, Source<'a>>,
}

impl<'a> Sources<'a> {
    pub fn has_source(&self, file: String) -> bool {
        self.files.contains_key(&file)
    }

    pub fn new_source(&mut self, file: String, contents: String) -> &Source<'a> {
        //! replaces any existing source with same name
        if self.has_source(file.clone()) {
            self.remove_source(file.clone());
        }

        self.files.insert(file.clone(), Source::new(contents));
        self.files.get(&file).unwrap()
    }

    pub fn remove_source(&mut self, file: String) {
        if self.has_source(file.clone()) {
            self.files.remove(&file);
        }
    }
}


pub static mut __SOURCES: Option<Sources> = None;

#[macro_export]
macro_rules! SOURCES {
    () => { 
        unsafe { 
            if let None = crate::lex::source::__SOURCES {
                crate::lex::source::__SOURCES = Some(crate::lex::source::Sources::default());
            }
            crate::lex::source::__SOURCES.unwrap()
        }
    };
}