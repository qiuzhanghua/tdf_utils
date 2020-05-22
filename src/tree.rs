use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use std::cmp::Ordering;
use std::marker::PhantomData;

/// Tree with key and adjacent order
/// key for unique
///
#[derive(Debug, Default, Clone)]
pub struct Tree<K, Item>
where
    K: Eq,
    Item: TreeNodeLike<K> + Ord,
{
    root: Option<TreeNode<K, Item>>,
}

impl<K, Item> Serialize for Tree<K, Item>
where
    K: Eq + Serialize,
    Item: TreeNodeLike<K> + Ord + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_some(&self.root)
    }
}

/// Tree should get key and parent
pub trait TreeNodeLike<K>
where
    K: Eq,
{
    fn key(&self) -> K;
    fn parent(&self) -> K;
}

/// Actual TreeNode
#[derive(Debug, Default, Clone, PartialOrd, Eq)]
pub struct TreeNode<K, Item>
where
    K: Eq,
    Item: TreeNodeLike<K> + Ord,
{
    node: Item,
    children: Vec<TreeNode<K, Item>>,
    phantom: PhantomData<K>,
}

impl<K, Item> Serialize for TreeNode<K, Item>
where
    K: Eq + Serialize,
    Item: TreeNodeLike<K> + Ord + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Node", 2)?;
        state.serialize_field("node", &self.node)?;
        if !&self.children.is_empty() {
            state.serialize_field("children", &self.children)?;
        }
        state.end()
    }
}

/// TreeNode methods
impl<K, Item> TreeNode<K, Item>
where
    K: Eq,
    Item: TreeNodeLike<K> + Ord,
{
    pub fn new(val: Item) -> TreeNode<K, Item> {
        TreeNode {
            node: val,
            children: Vec::default(),
            phantom: PhantomData,
        }
    }

    /// find by key recursively
    pub fn get(&mut self, key: &K) -> Option<&mut TreeNode<K, Item>> {
        if self.node.key() == *key {
            return Some(self);
        }
        for node in &mut self.children {
            if let Some(x) = node.get(key) {
                return Some(x);
            }
        }
        None
    }

    /// Add node to TreeNode
    pub fn append(&mut self, val: Item) -> bool {
        if val.parent() == self.node.key() {
            return insert(&mut self.children, TreeNode::new(val));
        }
        false
    }
}

/// method of Tree
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
        let mut p = self.get(&val.parent());
        if val.parent() == p.as_ref().unwrap().node.key() {
            p.as_mut().unwrap().append(val);
            return true;
        }
        false
    }

    pub fn get(&mut self, key: &K) -> Option<&mut TreeNode<K, Item>> {
        if self.root.is_none() {
            return None;
        }
        self.root.as_mut().unwrap().get(key)
    }
}

impl<K, Item> PartialEq for TreeNode<K, Item>
where
    K: Eq,
    Item: TreeNodeLike<K> + Ord,
{
    fn eq(&self, other: &Self) -> bool {
        self.node.key().eq(&other.node.key())
    }
}

impl<K, Item> Ord for TreeNode<K, Item>
where
    K: Eq + PartialOrd,
    Item: TreeNodeLike<K> + Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.node.cmp(&other.node)
    }
}

/// helper method for TreeNode
pub fn insert<K, Item>(v: &mut Vec<TreeNode<K, Item>>, node: TreeNode<K, Item>) -> bool
where
    K: Eq,
    Item: TreeNodeLike<K> + Ord,
{
    if v.is_empty() {
        v.push(node);
        return true;
    }
    if v.contains(&node) {
        return false;
    }; // 保证key不冲突
       // 不检查parent_id
    match v.binary_search_by(|n| n.node.cmp(&node.node)) {
        Ok(pos) => v.insert(pos + 1, node),
        Err(pos) => v.insert(pos, node),
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::tree::{insert, Tree, TreeNode, TreeNodeLike};
    use std::cmp::Ordering;

    #[test]
    fn test_append_to_vec() {
        let mut vec = Vec::<TreeNode<String, Node>>::new();

        let n1 = Node {
            key: "1".to_string(),
            order: 0,
            parent: "".to_string(),
        };
        let n2 = Node {
            key: "2".to_string(),
            order: 0,
            parent: "".to_string(),
        };
        let n3 = Node {
            key: "3".to_string(),
            order: 10,
            parent: "".to_string(),
        };
        let n4 = Node {
            key: "4".to_string(),
            order: 4,
            parent: "".to_string(),
        };
        let n5 = Node {
            key: "5".to_string(),
            order: 3,
            parent: "".to_string(),
        };

        insert(&mut vec, TreeNode::new(n1));
        assert_eq!(vec.len(), 1);
        insert(&mut vec, TreeNode::new(n2));
        assert_eq!(vec.len(), 2);
        insert(&mut vec, TreeNode::new(n3));
        println!("{:?}", vec);
        assert_eq!(vec.len(), 3);
        insert(&mut vec, TreeNode::new(n4));
        println!("{:?}", vec);
        assert_eq!(vec.len(), 4);
        insert(&mut vec, TreeNode::new(n5));
        println!("{:?}", vec);
        assert_eq!(vec.len(), 5);
    }

    #[test]
    pub fn test_tree_01() {
        let mut tree = Tree::<String, Node>::new();
        tree.append(Node {
            key: "key".to_string(),
            order: 0,
            parent: "".to_string(),
        });
        tree.append(Node {
            key: "key2".to_string(),
            order: 1,
            parent: "key".to_string(),
        });
        tree.append(Node {
            key: "key3".to_string(),
            order: 2,
            parent: "key".to_string(),
        });
        tree.append(Node {
            key: "key4".to_string(),
            order: 2,
            parent: "key3".to_string(),
        });

        println!("{:?}", tree);

        let json = serde_json::to_string(&tree).unwrap();
        println!("{}", json);
        assert_eq!(4, 2); //to  show converted json, make test failed
    }

    #[derive(Debug, PartialOrd, Eq, Serialize)]
    struct Node {
        key: String,
        order: i32,
        parent: String,
    }

    impl TreeNodeLike<String> for Node {
        fn key(&self) -> String {
            self.key.clone()
        }

        fn parent(&self) -> String {
            self.parent.clone()
        }

        // fn order(&self) -> i32 {
        //     self.order
        // }
    }

    impl PartialEq for Node {
        fn eq(&self, other: &Self) -> bool {
            self.key.eq(&other.key)
        }
    }

    impl Ord for Node {
        fn cmp(&self, other: &Self) -> Ordering {
            self.order.cmp(&other.order)
        }
    }
}
