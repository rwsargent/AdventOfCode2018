use std::cell::RefCell;
use std::fmt::Display;
use std::rc::{Rc, Weak};
use utils::*;

pub fn marble_game_winner(num_players: usize, last_marble: usize) -> PuzzleResult {
  let mut buf = List::new();
  buf.append(0);
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

// The node type stores the data and two pointers.
//
// It uses Option to represent nullability in safe Rust. It has zero overhead
// over a null pointer due to the NonZero optimization.
//
// It uses an Rc (Reference Counted) pointer to give ownership of the next node
// to the current node. And a Weak (weak Reference Counted) pointer to reference
// the previous node without owning it.
//
// It uses RefCell for interior mutability. It allows mutation through
// shared references.
struct Node<T> {
  pub data: T,
  pub prev: Option<Weak<RefCell<Node<T>>>>,
  pub next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
  // Constructs a node with some `data` initializing prev and next to null.
  pub fn new(data: T) -> Self {
    Self { data, prev: None, next: None }
  }

  pub fn remove(node: &mut Rc<RefCell<Node<T>>>) -> (T, Option<Rc<RefCell<Node<T>>>>) where T: Clone {
    let node = node.borrow_mut();
    if let Some(ref mut next) = node.next {
      let next = next.borrow_mut();
      next.prev = node.prev;
    }

    if let Some(ref mut prev) = node.prev {
      let prev = prev.upgrade().unwrap().borrow_mut();
      prev.next = node.next.clone();
    }

    (node.data.clone(), node.next)
  }

  pub fn insert(node: &mut Rc<RefCell<Node<T>>>, data: T) -> Option<Rc<RefCell<Node<T>>>> {
    let mut new_node = Node::new(data);
    new_node.prev = Some(Rc::downgrade(&node));
    let rc = Rc::new(RefCell::new(new_node));
    if let Some(ref mut n) = node.borrow_mut().next {
      n.borrow_mut().prev = Some(Rc::downgrade(&rc));
    }

    new_node.next = node.borrow_mut().next.replace(rc.clone());
    Some(rc)
  }

  // Appends `data` to the chain of nodes. The implementation is recursive
  // but one could rewrite it to use a while-let imperative loop instead
  // without too much effort.
  pub fn append(node: &mut Rc<RefCell<Node<T>>>, data: T) -> Option<Rc<RefCell<Node<T>>>> {
    let is_last = node.borrow().next.is_none();
    if is_last {
      // If the current node is the last one, create a new node,
      // set its prev pointer to the current node, and store it as
      // the node after the current one.
      let mut new_node = Node::new(data);
      new_node.prev = Some(Rc::downgrade(&node));
      let rc = Rc::new(RefCell::new(new_node));
      node.borrow_mut().next = Some(rc.clone());
      Some(rc)
    } else {
      // Not the last node, just continue traversing the list:
      if let Some(ref mut next) = node.borrow_mut().next {
        Self::append(next, data)
      } else { None }
    }
  }
}

// The doubly-linked list with pointers to the first and last nodes in the list.
struct List<T> {
  first: Option<Rc<RefCell<Node<T>>>>,
  last: Option<Rc<RefCell<Node<T>>>>,
  current: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> List<T> {
  // Constructs an empty list.
  pub fn new() -> Self {
    Self { first: None, last: None, current: None }
  }

  pub fn move_prev(&mut self) {
    if let Some(ref cur) = self.current {
      if cur.borrow().prev.is_none() {
        if cur.borrow().next.is_some() {
          self.current = self.last.clone();
        }
      } else {
        self.current = cur.borrow().prev;
      }
    }
  }

  pub fn move_next(&mut self) {
    if let Some(ref cur) = self.current {
      if cur.borrow().next.is_none() {
        if cur.borrow().prev.is_some() {
          self.current = self.first.clone();
        }
      } else {
        self.current = cur.borrow().next;
      }
    }
  }

  pub fn remove_current(&mut self) -> Option<T> {
    if let Some(ref mut cur) = self.current {
      *cur = Node::remove(cur);
      Some(cur.borrow().data)
    } else {
      None
    }
  }

  pub fn insert(&mut self, data: T) {
    if let Some(ref mut cur) = self.current {
      self.current = Node::insert(cur, data);
    } else {
      let f = Rc::new(RefCell::new(Node::new(data)));
      self.first = Some(f.clone());
      self.last = Some(f.clone());
      self.current = Some(f);
    }
  }
  // Appends a new node to the list, handling the case where the list is empty.
  pub fn append(&mut self, data: T) {
    if let Some(ref mut next) = self.first {
      self.last = Node::append(next, data);
    } else {
      let f = Rc::new(RefCell::new(Node::new(data)));
      self.first = Some(f.clone());
      self.last = Some(f.clone());
      self.current = Some(f);
    }
  }
}
