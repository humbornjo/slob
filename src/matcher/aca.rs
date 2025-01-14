use std::{borrow::Borrow, collections::VecDeque};

use crate::Tpair;

pub fn new(pairs: Vec<Tpair>) -> Ahoca {
  let mut acam = Ahoca {
    cnt: 0,
    span: vec![0],
    ncnt: vec![0],
    fail: vec![0],
    trie: vec![[0; 256]],
    exist: vec![0],
  };
  pairs
    .into_iter()
    .for_each(|x| acam.insert(x.smark.borrow()));
  acam.build();
  return acam;
}

#[derive(Default)]
pub struct Ahoca {
  cnt: usize,
  span: Vec<usize>,
  ncnt: Vec<usize>,
  fail: Vec<usize>,
  trie: Vec<[usize; 256]>,
  exist: Vec<usize>,
}

pub fn do_match(acam: &Ahoca, chunk: &str) -> (usize, Option<String>) {
  acam.query(chunk)
}

impl Ahoca {
  fn insert(&mut self, pat: &str) {
    let mut p = 0;
    let mut span = 0;
    pat.bytes().into_iter().for_each(|x| {
      span += 1;
      if self.trie[p][x as usize] == 0 {
        self.cnt += 1;
        self.span.push(span);
        self.fail.push(0);
        self.ncnt.push(0);
        self.trie.push([0; 256]);
        self.exist.push(0);
        self.trie[p][x as usize] = self.cnt;
      }
      self.ncnt[p] += 1;
      p = self.trie[p][x as usize];
    });
    self.exist[p] = pat.len();
  }

  fn build(&mut self) {
    let mut q = VecDeque::new();
    self.trie[0].into_iter().for_each(|x| {
      if x != 0 {
        q.push_back(x);
      }
    });

    while !q.is_empty() {
      let k = q.pop_front().unwrap();
      self.trie[k]
        .into_iter()
        .enumerate()
        .for_each(|(i, x)| match x {
          0 => self.trie[k][i] = self.trie[self.fail[k]][i],
          _ => {
            self.fail[x] = self.trie[self.fail[k]][i];
            q.push_back(x);
          }
        });
    }
  }

  fn query(&self, chunk: &str) -> (usize, Option<String>) {
    let mut t = 0;
    for (i, x) in chunk.bytes().enumerate() {
      t = self.trie[t][x as usize];
      while t != 0 && self.ncnt[t] == 0 {
        match self.exist[t] {
          0 => t = self.fail[t],
          _ => {
            let idx = i + 1 - self.exist[t];
            return (idx, Some(chunk[idx..=i].to_owned()));
          }
        }
      }
    }
    (chunk.len() - self.span[t], None)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::new_pair;
  #[test]
  fn acam_functionality() {
    let acam = new(vec![
      new_pair("oof", "bar"),
      new_pair("我试试", "你试试"),
      new_pair("helle", "world"),
      new_pair("我试哥", "你试试"),
      new_pair("helloo", "world"),
      new_pair("start", "end"),
    ]);
    let (idx, mark) = acam.query("ohellooabc");
    println!("{}, {}", idx, mark.unwrap_or("empty".to_owned()));
  }
}
