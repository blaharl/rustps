use std::{
    cell::RefCell,
    fmt::Debug,
    rc::{Rc, Weak},
};

struct Node<T> {
    value: T,
    prev: Option<Weak<RefCell<Node<T>>>>,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    #[must_use]
    fn new(value: T) -> Self {
        Self {
            value,
            prev: None,
            next: None,
        }
    }
}

struct Deque<T> {
    front: Option<Rc<RefCell<Node<T>>>>,
    back: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Deque<T> {
    #[must_use]
    fn new() -> Self {
        Self {
            front: None,
            back: None,
        }
    }

    fn push_front(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node::new(value)));
        if let Some(front) = &self.front {
            new_node.borrow_mut().next = Some(front.clone());
            front.borrow_mut().prev = Some(Rc::downgrade(&new_node));
        } else {
            self.back = Some(new_node.clone());
        }
        self.front = Some(new_node);
    }

    fn push_back(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node::new(value)));
        if let Some(back) = &self.back {
            new_node.borrow_mut().prev = Some(Rc::downgrade(back));
            back.borrow_mut().next = Some(new_node.clone());
        } else {
            self.front = Some(new_node.clone());
        }
        self.back = Some(new_node);
    }

    fn pop_front(&mut self) -> Option<T> {
        let pop = self.front.take()?;

        match pop.borrow_mut().next.take() {
            Some(next) => {
                next.borrow_mut().prev = None;
                self.front = Some(next);
            }
            None => self.back = None,
        }

        let node = Rc::into_inner(pop)?.into_inner();
        Some(node.value)
    }

    fn pop_back(&mut self) -> Option<T> {
        let pop = self.back.take()?;

        match pop.borrow_mut().prev.take().and_then(|prev| prev.upgrade()) {
            Some(prev) => {
                prev.borrow_mut().next = None;
                self.back = Some(prev);
            }
            None => self.front = None,
        }

        let node = Rc::into_inner(pop)?.into_inner();
        Some(node.value)
    }

    fn eprint(&self)
    where
        T: Debug,
    {
        eprint!("front <-");
        let mut iter = self.front.clone();

        while let Some(curr) = iter {
            let node = curr.borrow();
            eprint!("-> {:?} <-", node.value);
            iter = node.next.clone();
        }

        eprintln!("-> back");
    }
}

impl<T> Default for Deque<T> {
    fn default() -> Self {
        Self::new()
    }
}
