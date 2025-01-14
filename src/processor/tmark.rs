use crate::matcher::kmp;
use crate::{Part, Slob, State, Tpair};

struct TmarkProcessor {
  value: Option<String>,
  m2value: Option<String>,
  state: State,
  buffer: String,
  kpairs: [kmp::Kpair; 2],
}

pub fn new(pair: Tpair) -> impl Slob {
  TmarkProcessor {
    value: None,
    m2value: pair.value.clone(),
    state: State::StateQuest,
    buffer: String::new(),
    kpairs: [kmp::new(pair.smark.as_ref()), kmp::new(pair.emark.as_ref())],
  }
}

impl Slob for TmarkProcessor {
  fn process(&mut self, chunk: &str) -> Vec<Part> {
    let mut parts = vec![];
    self.buffer.push_str(chunk);
    while {
      let mut encore = false;
      let kpair = &self.kpairs[self.state as usize];
      let (idx, mark) = kmp::do_match(kpair, self.buffer.as_str());
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
        _ => {
          encore = true;
          match self.state {
            State::StateQuest => {
              self.value = self.m2value.clone();
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
  fn test_tmark_processor() {
    let mut parts = vec![];
    let pair = new_pair("<think>", "</think>").with_value("foo");
    let mut processor = new(pair);
    parts.extend(processor.process("<think>foo</think>"));
    assert_eq!(parts.len(), 3);
    assert_eq!(parts[0].state, State::StateSmark);
    assert_eq!(parts[1].content, "foo");
    assert_eq!(parts[1].state, State::StateMatch);
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
