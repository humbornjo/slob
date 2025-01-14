use std::borrow::Cow;
use std::collections::HashMap;

use crate::matcher::{aca, kmp};
use crate::{Part, Slob, State, Tpair};

struct TacamProcessor<'a> {
  state: State,
  buffer: String,
  acam: aca::Ahoca,
  value: Option<String>,
  kpairs: Cow<'a, [kmp::Kpair; 2]>,
  m2value: HashMap<String, Option<String>>, // Use Cow here
  m2kpairs: HashMap<String, Cow<'a, [kmp::Kpair; 2]>>, // And here
}

pub fn new(pairs: Vec<Tpair>) -> impl Slob {
  let mut m2v = HashMap::new();
  let mut m2k = HashMap::new();

  for pair in &pairs {
    let kpairs_cow = Cow::Owned([kmp::new(&pair.smark), kmp::new(&pair.emark)]); // Cow for kpairs
    m2v.insert(pair.smark.clone().into(), pair.value.clone()); // Clone smark_string here
    m2k.insert(pair.smark.clone().into(), kpairs_cow);
  }

  TacamProcessor {
    state: State::StateQuest,
    buffer: String::new(),
    acam: aca::new(pairs),
    value: None,
    kpairs: Cow::Owned([kmp::new(""), kmp::new("")]),
    m2value: m2v,
    m2kpairs: m2k,
  }
}

impl Slob for TacamProcessor<'_> {
  fn process(&mut self, chunk: &str) -> Vec<Part> {
    let mut parts = vec![];
    self.buffer.push_str(chunk);
    while {
      let mut encore = false;
      let mut kpair = &self.kpairs[self.state as usize];
      let (idx, mark) = match self.state {
        State::StateQuest => aca::do_match(&self.acam, &self.buffer),
        State::StateMatch => kmp::do_match(kpair, &self.buffer),
        _ => unreachable!(),
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
        None => _ = self.buffer.drain(..idx),
        Some(m) => {
          encore = true;
          match self.state {
            State::StateQuest => {
              self.value = self.m2value[&m].clone();
              self.kpairs = self.m2kpairs[&m].clone();
              kpair = &self.kpairs[self.state as usize];
              parts.push(Part {
                content: "".to_owned(),
                state: State::StateSmark,
                value: self.value.clone(),
              });
              self.state = State::StateMatch;
            }
            State::StateMatch => {
              self.value = None;
              parts.push(Part {
                content: "".to_owned(),
                state: State::StateEmark,
                value: self.value.clone(),
              });
              self.state = State::StateQuest;
            }
            _ => unreachable!(),
          }
          self.buffer.drain(..idx + kpair.mark.len());
        }
      }
      encore
    } {}
    parts
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::new_pair;

  #[test]
  fn test_tacam_processor() {
    let mut parts = vec![];
    let pair = new_pair("<think>", "</think>").with_value("foo");
    let mut processor = new(vec![pair]);
    parts.extend(processor.process("<think>foo</think>"));
    assert_eq!(parts.len(), 3);
    assert_eq!(parts[0].state, State::StateSmark);
    assert_eq!(parts[1].state, State::StateMatch);
    assert_eq!(parts[1].content, "foo");
    assert_eq!(parts[2].state, State::StateEmark);

    parts.clear();

    parts.extend(processor.process("<th"));
    parts.extend(processor.process("ink>"));
    parts.extend(processor.process("foo"));
    parts.extend(processor.process("</th"));
    parts.extend(processor.process("ink>"));
    assert_eq!(parts.len(), 3);
    assert_eq!(parts[0].state, State::StateSmark);
    assert_eq!(parts[0].value.clone().unwrap(), "foo");
    assert_eq!(parts[1].content, "foo");
    assert_eq!(parts[1].state, State::StateMatch);
    assert_eq!(parts[1].value.clone().unwrap(), "foo");
    assert_eq!(parts[2].state, State::StateEmark);
    assert_eq!(parts[2].value.clone(), None);
  }
}
