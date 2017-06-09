extern crate nix;

use nix::sys::wait::{waitpid};
use nix::unistd::{fork};

use std::collections::{BTreeMap, BTreeSet};

pub trait ResourceSet {
  type Elem;

  fn alloc(&mut self) -> Option<Self::Elem>;
  fn reclaim(&mut self, elem: Self::Elem);
}

pub trait Process {
}

pub struct CounterSet {
  inner:    usize,
}

impl CounterSet {
  pub fn new() -> Self {
    CounterSet{inner: 0}
  }
}

impl ResourceSet for CounterSet {
  type Elem = usize;

  fn alloc(&mut self) -> Option<usize> {
    let count = self.inner;
    self.inner += 1;
    Some(count)
  }

  fn reclaim(&mut self, _elem: usize) {
    // Do nothing.
  }
}

pub struct RangeSet {
  free:     BTreeSet<usize>,
  used:     BTreeSet<usize>,
}

impl RangeSet {
  pub fn new(count: usize) -> Self {
    let mut range = BTreeSet::new();
    for idx in 0 .. count {
      range.insert(idx);
    }
    RangeSet{
      free:     range,
      used:     BTreeSet::new(),
    }
  }
}

impl ResourceSet for RangeSet {
  type Elem = usize;

  fn alloc(&mut self) -> Option<usize> {
    let mut elem = None;
    for &el in self.free.iter() {
      elem = Some(el);
      break;
    }
    if let Some(el) = elem {
      self.free.remove(&el);
      self.used.insert(el);
    }
    elem
  }

  fn reclaim(&mut self, elem: usize) {
    self.used.remove(&elem);
    self.free.insert(elem);
  }
}
