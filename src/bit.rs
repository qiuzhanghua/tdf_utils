#[derive(Debug, Default)]
struct BinaryIndexedTree<T> {
    n: usize,
    nodes: Vec<T>,
}

impl<T> BinaryIndexedTree<T>
where
    T: Default + Clone + std::ops::AddAssign + std::ops::Sub<Output = T>,
{
    pub fn new(n: usize) -> BinaryIndexedTree<T> {
        BinaryIndexedTree {
            n,
            nodes: vec![Default::default(); n + 1],
        }
    }

    fn from(v: Vec<T>) -> BinaryIndexedTree<T> {
        let n = v.len();
        let nodes: Vec<T> = vec![Default::default(); n + 1];
        let mut ans = BinaryIndexedTree { n, nodes };
        for i in 0..n {
            ans.update(i, v[i].clone())
        }
        ans
    }
    pub fn update(&mut self, idx: usize, delta: T) {
        let mut i = (idx + 1) as i32;
        while i <= self.n as i32 {
            self.nodes[i as usize] += delta.clone();
            i += i & (-i);
        }
    }

    pub fn sum_to(&mut self, idx: usize) -> T {
        let mut i = (idx + 1) as i32;
        let mut sum = Default::default();
        while i > 0 {
            sum += self.nodes[i as usize].clone();
            i -= i & (-i);
        }
        sum
    }

    pub fn sum_of_range(&mut self, idx: usize, end: usize) -> T {
        self.sum_to(end)
            - if idx == 0 {
                Default::default()
            } else {
                self.sum_to(idx - 1)
            }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_sum() {
        let mut bit = BinaryIndexedTree::from(vec![1, 2, 3, 4, 5, 6, 7, 8]);
        println!("{:?}", bit);
        assert_eq!(bit.sum_to(0), 1);
        assert_eq!(bit.sum_to(1), 3);
        assert_eq!(bit.sum_to(2), 6);
        assert_eq!(bit.sum_to(3), 10);
        assert_eq!(bit.sum_to(4), 15);
        assert_eq!(bit.sum_to(5), 21);
        assert_eq!(bit.sum_to(6), 28);
        assert_eq!(bit.sum_to(7), 36);
    }

    #[test]
    fn test_bit_sum_of_range() {
        let mut bit = BinaryIndexedTree::from(vec![1, 2, 3, 4, 5, 6, 7, 8]);
        println!("{:?}", bit);
        assert_eq!(bit.sum_of_range(0, 0), 1);
        assert_eq!(bit.sum_of_range(1, 1), 2);
        assert_eq!(bit.sum_of_range(0, 1), 3);
        assert_eq!(bit.sum_of_range(0, 2), 6);
        assert_eq!(bit.sum_of_range(2, 3), 7);
        assert_eq!(bit.sum_of_range(0, 7), 36);
        assert_eq!(bit.sum_of_range(6, 7), 15);
    }
}
