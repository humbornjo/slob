use std::sync::Arc;

pub struct Kpair {
  pub mark: String,
  pub lps: Arc<[usize]>,
}

pub fn new(mark: &str) -> Kpair {
  Kpair {
    mark: mark.to_owned(),
    lps: gen_lps(mark),
  }
}

pub fn do_match(pat: &str, s: &str, lps: Arc<[usize]>) -> (usize, bool) {
  let mut i = 0;
  let mut j = 0;
  while i < s.len() {
    if pat.bytes().nth(j) == s.bytes().nth(i) {
      i += 1;
      j += 1;
      if j == pat.len() {
        return (i - j, true);
      }
    } else {
      if j == 0 {
        i += 1;
      } else {
        j = lps[j - 1];
      }
    }
  }
  return (i - j, false);
}

pub fn gen_lps(pat: &str) -> Arc<[usize]> {
  let mut i = 1;
  let mut j = 0;
  let mut lps = vec![0 as usize; pat.len()];

  while i < pat.len() {
    if pat.bytes().nth(i) == pat.bytes().nth(j) {
      j += 1;
      lps[i] = j;
      i += 1;
      continue;
    }
    if j != 0 {
      j = lps[j - 1];
    } else {
      lps[i] = 0;
      i += 1
    }
  }

  return Arc::from(lps);
}
