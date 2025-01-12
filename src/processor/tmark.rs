struct TmarkProcessor {
  state: State,
  buffer: String,
  kpairs: [kmp::Kpair; 2],
}

pub fn new_tmark_processor(pair: Tpair) -> impl Slob {
  TmarkProcessor {
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
      let kpair = &self.kpairs[self.state as usize];
      let (idx, encore) = kmp::do_match(kpair, self.buffer.as_str());
      match idx {
        0 => {}
        _ => {
          parts.push(Part {
            content: self.buffer[..idx].to_owned(),
            state: self.state,
            value: None,
          });
        }
      }
      match encore {
        false => {}
        true => {
          parts.push(Part {
            content: "".to_owned(),
            state: match self.state {
              State::StateQuest => State::StateSmark,
              State::StateMatch => State::StateEmark,
              _ => panic!("Invalid state: {}", self.state),
            },
            value: None,
          });
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
