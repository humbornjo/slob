struct TummyProcessor {}

pub fn new_tummy_processor() -> impl Slob {
  TummyProcessor {}
}

impl Slob for TummyProcessor {
  fn process(&mut self, chunk: &str) -> Vec<Part> {
    vec![Part {
      content: chunk.to_owned(),
      state: State::StateQuest,
      value: None,
    }]
  }
}
