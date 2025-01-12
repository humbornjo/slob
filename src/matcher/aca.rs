use crate::Tpair;

pub fn new(pairs: Vec<Tpair>) -> Acam {
  Acam {}
}

pub struct Acam {}

pub fn do_match(acam: &Acam, chunk: &str) -> (usize, bool) {
  (0, false)
}
