#[derive(Debug)]
pub struct SetIterator {
    num_elements: u8,
    next_counter: u64,
    exhausted: bool,
}

impl SetIterator {
    pub const fn new(num_elems: u8) -> Self {
        SetIterator {
            num_elements: num_elems,
            next_counter: 0,
            exhausted: false,
        }
    }
}

impl Iterator for SetIterator {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.exhausted {
            return None;
        }

        let mut res = vec![];
        let mut bitset = self.next_counter;
        for i in 0..self.num_elements {
            if (bitset & 1) == 1 {
                res.push(i);
            }
            bitset >>= 1;
        }

        self.next_counter = self
            .next_counter
            .checked_add(1)
            .expect("Number of element exceeded 2**64 - 1");

        if self.next_counter == 1_u64 << self.num_elements {
            self.exhausted = true;
        }
        Some(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subset_generator() {
        let mut iter = SetIterator::new(3);
        assert_eq!(iter.next(), Some(vec![]));
        assert_eq!(iter.next(), Some(vec![0]));
        assert_eq!(iter.next(), Some(vec![1]));
        assert_eq!(iter.next(), Some(vec![0, 1]));
        assert_eq!(iter.next(), Some(vec![2]));
        assert_eq!(iter.next(), Some(vec![0, 2]));
        assert_eq!(iter.next(), Some(vec![1, 2]));
        assert_eq!(iter.next(), Some(vec![0, 1, 2]));
        assert_eq!(iter.next(), None);

        assert_eq!(SetIterator::new(10).count(), 1 << 10);
    }
}
