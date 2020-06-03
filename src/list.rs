use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Default)]
pub struct List<T>(Option<(T, Box<List<T>>)>);

impl<T> List<T> {
    pub fn new() -> Self {
        List(None)
    }

    pub fn push_front(&mut self, data: T) {
        let t = self.0.take();
        self.0 = Some((data, Box::new(List(t))))
    }

    pub fn pop_front(&mut self) -> Option<T> {
        match self.0.take() {
            None => None,
            Some((val, next)) => {
                self.0 = next.0;
                Some(val)
            }
        }
    }

    pub fn push_back(&mut self, data: T) {
        match self.0 {
            Some((_, ref mut next)) => next.push_back(data),
            None => self.push_front(data),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    pub fn len(&self) -> usize {
        let mut ans = 0;
        let mut p = self;
        while let Some((_, next)) = &p.0 {
            ans += 1;
            unsafe {
                p = std::mem::transmute_copy(next);
            }
        }
        ans
        // match &self.0 {
        //     Some((_, next)) => 1 + next.len(),
        //     None => 0
        // }
    }
}

#[derive(Clone, Default)]
pub struct ListNode<T> {
    val: T,
    next: ListLink<T>,
    prev: ListLink<T>,
}

pub type ListLink<T> = Option<Rc<RefCell<ListNode<T>>>>;

#[derive(Clone, Default)]
pub struct DoubleLinkedList<T: Clone> {
    head: ListLink<T>,
    tail: ListLink<T>,
    pub length: u64,
}

impl<T: Clone> DoubleLinkedList<T> {
    pub fn append(&mut self, val: T) {
        let n = Rc::new(RefCell::new(ListNode {
            val,
            next: None,
            prev: None,
        }));
        match self.tail.take() {
            Some(old) => {
                old.borrow_mut().next = Some(n.clone());
                n.borrow_mut().prev = Some(old);
            }
            None => self.head = Some(n.clone()),
        }
        self.length += 1;
        self.tail = Some(n)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                next.borrow_mut().prev = None;
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.length -= 1;
            Rc::try_unwrap(head)
                .ok()
                .unwrap() // .expect("fatal error")
                .into_inner()
                .val
        })
    }

    pub fn iter(&self) -> DoubleLinkedListIterator<T> {
        DoubleLinkedListIterator::new(self.head.clone())
    }
}

pub struct DoubleLinkedListIterator<T: Clone> {
    current: ListLink<T>,
}

impl<T: Clone> DoubleLinkedListIterator<T> {
    fn new(start_at: ListLink<T>) -> Self {
        DoubleLinkedListIterator { current: start_at }
    }
}

impl<T: Clone> Iterator for DoubleLinkedListIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let current = &self.current;
        let mut result = None;
        self.current = match current {
            Some(ref current) => {
                let current = current.borrow();
                result = Some(current.val.clone());
                current.next.clone()
            }
            None => None,
        };
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::list::{DoubleLinkedList, List};

    #[test]
    pub fn list_test_01() {
        let mut list = List::default();
        assert_eq!(list.len(), 0);
        list.push_front(3);
        list.push_back(4);
        list.push_front(9);
        assert_eq!(list.len(), 3);
    }

    #[test]
    pub fn list_test_02() {
        let mut list: List<i32> = List::default();
        let p = list.pop_front();
        assert_eq!(p.is_none(), true);
        assert_eq!(list.len(), 0);
        list.push_front(3);
        list.push_back(4);
        list.push_front(9);
        println!("{:?}", list);
        let p = list.pop_front();
        println!("{:?}", p);
        assert_eq!(p, Some(9));
        println!("{:?}", list);
        let p = list.pop_front();
        println!("{:?}", p);
        println!("{:?}", list);
        assert_eq!(p, Some(3));
        let p = list.pop_front();
        println!("{:?}", p);
        println!("{:?}", list);
        assert_eq!(p, Some(4));
        let p = list.pop_front();
        println!("{:?}", p);
        assert_eq!(p, None);
    }

    #[test]
    pub fn list_test_03() {
        let mut dl = DoubleLinkedList::<i32>::default();
        dl.append(1);
        dl.append(2);
        dl.append(3);
        println!("{:?}", dl.length);
        //println!("{:?}", dl);
        let o = dl.pop();
        println!("{:?}", o);
        let o = dl.pop();
        println!("{:?}", o);
        let o = dl.pop();
        println!("{:?}", o);
        let o = dl.pop();
        println!("{:?}", o);
        assert!(o.is_none())
    }

    #[test]
    pub fn list_test_04() {
        let mut dl = DoubleLinkedList::<i32>::default();
        dl.append(1);
        dl.append(2);
        dl.append(3);
        println!("{:?}", dl.length);

        for o in dl.iter() {
            println!("{}", o)
        }

        assert_eq!(dl.length, 3)
    }
}
