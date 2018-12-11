use std::cell::RefCell;
use std::fmt::Display;
use std::rc::{Rc, Weak};
use utils::*;

pub fn marble_game_winner(num_players: usize, last_marble: usize) -> PuzzleResult {
  let mut buf = CircularList::new();
  buf.insert(0);
  let mut current_marble = 1;
  let mut players = Vec::with_capacity(num_players);
  for _ in 0..num_players {
    players.push(0);
  }
  while current_marble <= last_marble {
    if current_marble % 23 == 0 {
      for _ in 0..7 {
        buf.move_prev();
      }
      let player_idx = current_marble % num_players;
      unsafe {
        let player = players.get_unchecked_mut(player_idx);
        *player += current_marble;
        *player += buf.remove_current().unwrap();
      }
    } else {
      buf.move_next();
      buf.insert(current_marble);
    }
    current_marble += 1;
  }

  Ok(PuzzleSolution::usize(players.into_iter().max().unwrap()))
}

struct Node<T> {
  pub data: T,
  pub prev: Option<Rc<RefCell<Node<T>>>>,
  pub next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
  pub fn new(data: T) -> Self {
    Self { data, prev: None, next: None }
  }

  pub fn split(&self) -> (T, Option<Rc<RefCell<Node<T>>>>, Option<Rc<RefCell<Node<T>>>>) where T: Clone {
    (self.data.clone(), self.prev.clone(), self.next.clone())
  }
}

struct CircularList<T> {
  len: usize,
  current: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> CircularList<T> {
  pub fn new() -> Self {
    Self { len: 0, current: None }
  }

  pub fn move_prev(&mut self) {
    let next = if let Some(ref cur) = self.current {
      cur.borrow().prev.clone()
    } else {
      None
    };
    self.current = next;
  }

  pub fn move_next(&mut self) {
    let prev = if let Some(ref cur) = self.current {
      cur.borrow().next.clone()
    } else {
      None
    };
    self.current = prev;
  }

  /// Removes the current pointer, returning the data there, and sets current to next.
  pub fn remove_current(&mut self) -> Option<T> where T: Clone {
    if self.len == 0 {
      None
    } else {
      self.len -= 1;
      if self.len == 0 {
        let cur = self.current.take();
        let cur = cur.unwrap();
        let cur = cur.borrow();
        let data = cur.data.clone();
        Some(data)
      } else {
        let current = self.current.take();
        if let Some(mut cur) = current {
          let cur = cur.borrow();
          let (data, prev, next) = cur.split();
          if let Some(ref p) = prev {
            p.borrow_mut().next = cur.next.clone();
          }
          if let Some(ref n) = next {
            n.borrow_mut().prev = cur.prev.clone();
          }
          self.current = next;
          Some(data)
        } else {
          None
        }
      }
    }
  }

  /// Inserts data to the right (as next) of current and sets current to point to new data.
  pub fn insert(&mut self, data: T) {
    let mut cur = self.current.take();
    self.current = Some(if let Some(mut cur) = cur {
      let mut new_node = Node::new(data);
      new_node.next = cur.borrow().next.clone();
      new_node.prev = Some(cur.clone());

      let rc = Rc::new(RefCell::new(new_node));

      if self.len == 1 {
        let mut cur = cur.borrow_mut();
        cur.next = Some(rc.clone());
        cur.prev = Some(rc.clone());
      } else {
        if let Some(ref mut n) = cur.borrow_mut().next {
          n.borrow_mut().prev = Some(rc.clone());
        }
        cur.borrow_mut().next = Some(rc.clone());
      }
      rc
    } else {
      let mut node = Node::new(data);
      let rc = Rc::new(RefCell::new(node));
      rc.borrow_mut().next = Some(rc.clone());
      rc.borrow_mut().prev = Some(rc.clone());
      rc
    });
    self.len += 1;
  }
}

impl<T: Display> Display for CircularList<T> {
  fn fmt(&self, w: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
    write!(w, "{} [", self.len)?;
    let mut node = self.current.clone();
    let mut count = 0;
    while let Some(n) = node {
      count += 1;
      write!(w, "{}", n.borrow().data)?;
      node = if count < self.len {
        write!(w, ", ")?;
        n.borrow().next.clone()
      } else {
        None
      }
    }
    write!(w, "]")
  }
}
