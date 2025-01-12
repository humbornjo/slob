mod matcher;
mod processor;

use std::borrow::Cow;
use std::fmt::Display;

pub trait Slob {
    fn process(&mut self, pattern: &str) -> Vec<Part>;
}

#[repr(usize)]
#[derive(Debug, Copy, Clone)]
pub enum State {
    StateQuest,
    StateMatch,
    StateSmark,
    StateEmark,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::StateQuest => write!(f, "StateQuest"),
            State::StateMatch => write!(f, "StateMatch"),
            State::StateSmark => write!(f, "StateSmark"),
            State::StateEmark => write!(f, "StateEmark"),
        }
    }
}

#[derive(Debug)]
pub struct Part {
    pub content: String,
    pub state: State,
    pub value: Option<String>,
}

#[derive(Clone)]
pub struct Tpair<'a> {
    smark: Cow<'a, str>,
    emark: Cow<'a, str>,
    value: Option<String>,
}

pub fn new_pair<'a>(start_mark: &'a str, end_mark: &'a str) -> Tpair<'a> {
    Tpair {
        smark: Cow::Borrowed(start_mark),
        emark: Cow::Borrowed(end_mark),
        value: None,
    }
}

impl Tpair<'_> {
    pub fn with_value(&mut self, value: &str) -> &mut Self {
        self.value = Some(value.to_owned());
        self
    }
}

pub fn new_processor(mut pairs: Vec<Tpair>) -> Box<dyn Slob> {
    match pairs.len() {
        0 => Box::from(processor::new_tummy_processor()),
        1 => Box::from(processor::new_tmark_processor(pairs.pop().unwrap())),
        _ => Box::from(processor::new_tacam_processor(pairs)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tmark_functionality() {
        let pairs = vec![new_pair("oof", "bar")];
        let mut processor = new_processor(pairs);
        let parts = processor.process("foo");
        assert_eq!(parts.len(), 1);
        assert_eq!(parts[0].content, "f");
    }
}
