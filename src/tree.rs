use serde::{Deserialize, Serialize, Serializer};
use std::collections::{BTreeMap, BTreeSet};
use std::marker::PhantomData;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Tree<K, Item>
where
    K: Eq,
    Item: TreeNodeLike<K> + Ord,
{
    root: Option<TreeNode<K, Item>>,
}

pub trait TreeNodeLike<K>
where
    K: Eq,
{
    fn key(&self) -> K;
    fn parent(&self) -> K;
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct TreeNode<K, Item>
where
    K: Eq,
    Item: TreeNodeLike<K> + Ord,
{
    val: Item,
    children: BTreeSet<Item>,
    phantom: PhantomData<K>,
}

impl<K, Item> TreeNode<K, Item>
where
    K: Eq,
    Item: TreeNodeLike<K> + Ord,
{
    pub fn new(val: Item) -> TreeNode<K, Item> {
        TreeNode {
            val,
            children: BTreeSet::default(),
            phantom: PhantomData,
        }
    }

    pub fn append(&mut self, val: Item) -> bool {
        if val.parent() == self.val.key() {
            self.children.insert(val);
            return true;
        }
        false
    }
}

impl<K, Item> Tree<K, Item>
where
    K: Eq,
    Item: TreeNodeLike<K> + Ord,
{
    pub fn new() -> Tree<K, Item> {
        Tree { root: None }
    }

    pub fn append(&mut self, val: Item) -> bool {
        if self.root.is_none() {
            self.root = Some(TreeNode::new(val));
            return true;
        }
        let mut p = self.get(val.parent());
        if val.parent() == p.as_ref().unwrap().val.key() {
            p.as_mut().unwrap().append(val);
            return true;
        }
        false
    }

    pub fn get(&mut self, key: K) -> Option<&mut TreeNode<K, Item>> {
        self.root.as_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::tree::{Tree, TreeNode, TreeNodeLike};
    use std::cmp::Ordering;

    #[test]
    pub fn test_tree_01() {
        let mut tree = Tree::<String, Node>::new();
        tree.append(Node {
            key: "key".to_string(),
            parent: "".to_string(),
        });
        tree.append(Node {
            key: "key2".to_string(),
            parent: "key".to_string(),
        });
        println!("{:?}", tree);
        assert_eq!(1, 2)
    }

    #[derive(Debug, PartialOrd, PartialEq, Eq)]
    struct Node {
        key: String,
        parent: String,
    }

    impl TreeNodeLike<String> for Node {
        fn key(&self) -> String {
            self.key.clone()
        }

        fn parent(&self) -> String {
            self.parent.clone()
        }
    }

    impl Ord for Node {
        fn cmp(&self, other: &Self) -> Ordering {
            self.key.cmp(&other.key)
        }
    }
}
