use crate::matcher::{aca, kmp};
use crate::{Part, Slob, State, Tpair};

use std::collections::HashMap;

pub fn new(pairs: Vec<Tpair>) -> impl Slob {
  TacamProcessor {
    state: State::StateQuest,
    buffer: String::new(),
    acam: aca::new(pairs),
    value: None,
    kpairs: [kmp::new(""), kmp::new("")],
    m2value: HashMap::new(),
    m2kpairs: HashMap::new(),
  }
}

struct TacamProcessor {
  state: State,
  buffer: String,
  acam: aca::Acam,
  value: Option<String>,
  kpairs: [kmp::Kpair; 2],
  m2value: HashMap<String, String>,
  m2kpairs: HashMap<String, [kmp::Kpair; 2]>,
}

impl Slob for TacamProcessor {
  fn process(&mut self, chunk: &str) -> Vec<Part> {
    let mut parts = vec![];
    self.buffer.push_str(chunk);
    while {
      let mut encore = false;
      let kpair = self.kpairs[self.state as usize].clone();
      let (idx, mark) = match self.state {
        State::StateQuest => aca::do_match(&self.acam, &self.buffer),
        State::StateMatch => kmp::do_match(&kpair, &self.buffer),
        _ => panic!("Invalid state: {}", self.state),
      };
      match idx {
        0 => {}
        _ => {
          parts.push(Part {
            content: self.buffer[..idx].to_owned(),
            state: self.state,
            value: self.value.clone(),
          });
        }
      }
      match mark {
        None => {}
        Some(m) => {
          if self.state == State::StateQuest {
            self.kpairs = self.m2kpairs[&m].clone();
            self.value = Some(self.m2value[&m].clone());
          }
          encore = true;
          parts.push(Part {
            content: "".to_owned(),
            state: match self.state {
              State::StateQuest => State::StateSmark,
              State::StateMatch => State::StateEmark,
              _ => panic!("Invalid state: {}", self.state),
            },
            value: self.value.clone(),
          });
          if self.state == State::StateMatch {
            self.value = None;
          }
          self.state = match self.state {
            State::StateQuest => State::StateMatch,
            State::StateMatch => State::StateQuest,
            _ => panic!("Invalid state: {}", self.state),
          }
        }
      }
      self.buffer = self.buffer[idx + (kpair.mark.len() * encore as usize)..].to_owned();
      encore
    } {}
    parts
  }
}
