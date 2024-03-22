
type AllocationNumber = u32;

#[derive(Clone, Copy)]
pub struct Token {
    spot:usize,
    alloc_number:AllocationNumber,
}

struct AllocCell<T> {
    data:Option<T>,
    alloc_number:AllocationNumber,
}

pub struct MonoHeap<T> {
    block:Vec<AllocCell<T>>,
}

impl<T> MonoHeap<T> {

    // public =====

    pub fn iter(&self) -> MonoHeapIterator<T> {
        return MonoHeapIterator::<T> {
            heap:self,
            idx:0,
        };
    }

    pub fn new() -> Self {
        return Self {
            block:Vec::new(),
        };
    }

    pub fn with_capacity(capacity:usize) -> Self {
        return Self {
            block:Vec::with_capacity(capacity),
        };
    }

    pub fn insert(&mut self, value:T) -> Token {
        let token = self.reserve();
        self.block[token.spot].data = Some(value);
        return token;
    }

    pub fn remove(&mut self, token:Token) -> bool {
        if ! self.is_token_valid(token) {
            return false;
        }
        else {
            self.block[token.spot].data = None;
            return true;
        }
    }

    pub fn get_mut(&mut self, token:Token) -> Option<&mut T> {
        if !self.is_token_valid(token) {
            return  None;
        }
        else {
            return match self.block[token.spot].data.as_mut() {
                Some(v) => Some(v),
                None => None,
            }
        }
    }

    pub fn get(&self, token:Token) -> Option<&T> {
        if !self.is_token_valid(token) {
            return None;
        }
        else {
            return Some(self.block[token.spot].data.as_ref().unwrap());
        }
    }

    pub fn is_token_valid(&self, token:Token) -> bool {
        return token.spot < self.block.len() &&
               self.block[token.spot].alloc_number == token.alloc_number &&
               self.block[token.spot].data.is_some();
    }

    // private =====
    fn reserve(&mut self) -> Token {

        for i in 0..self.block.len() {
            let cell = &mut self.block[i];
            if cell.data.is_none() {
                cell.alloc_number += 1;
                return Token {
                    spot:i,
                    alloc_number:cell.alloc_number,
                };
            }
        }

        self.block.push(AllocCell {
            data:None,
            alloc_number:0,
        });

        return Token {
            spot: self.block.len() - 1,
            alloc_number: 0,
        };

    }
}

pub struct MonoHeapIterator<'a,T> {
    heap:&'a MonoHeap<T>,
    idx:usize,
}

impl<'a,T:Clone> Iterator for MonoHeapIterator<'a,T> {
    type Item =&'a T;
    fn next(&mut self) -> Option<Self::Item> {
        #[cfg(test)]
        println!("Iteration {}", self.idx);
        while self.idx < self.heap.block.len() {
            if let Some(v) = &self.heap.block[self.idx].data {
                self.idx += 1;
                return Some(v);
            }
            self.idx += 1;
        }
        return None;
    }
}

mod test {
    use std::{collections::HashMap, hash::Hash};

    use crate::MonoHeap;

    #[test]
    fn create() {
        let _ = MonoHeap::<i32>::new();
    }
    
    #[test]
    fn add_retrieve() {
        let mut h = MonoHeap::<i32>::new();
        let t1 = h.insert(4);
        let t2 = h.insert(6);
        assert_eq!(*h.get(t1).unwrap(), 4);
        assert_eq!(*h.get(t2).unwrap(), 6);
    }

    #[test]
    fn add_delete_retrieve() {
        let mut h = MonoHeap::<i32>::new();
        let t1 = h.insert(4);
        let t2 = h.insert(6);
        let t3 = h.insert(12);
        h.remove(t2);
        assert_eq!(*h.get(t1).unwrap(), 4);
        assert_eq!(h.get(t2), None);
        assert_eq!(*h.get(t3).unwrap(), 12);
    }

    #[test]
    fn iterator_test() {
        let mut h = MonoHeap::<i32>::new();
        h.insert(4);
        h.insert(6);
        h.insert(12);
        let t1 = h.insert(14);
        h.remove(t1);
        let mut check_set:HashMap<i32, bool> = HashMap::from([
            (4,false),
            (6,false),
            (12,false),
        ]);

        for a in h.iter() {
            
            if let Some(v) = check_set.get_mut(a) {
                *v = true;
            }
            else {
                assert!(false)
            }
        }

        for (_,v) in check_set {
            assert!(v);
        }

    }

}