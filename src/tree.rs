use serde::{Deserialize, Serialize, Serializer};
use std::collections::{BTreeMap, BTreeSet};

pub trait HasKey<K, V>
where
    K: Ord,
{
    fn key(val: V) -> K;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Tree<K, V>
where
    K: Ord,
    V: HasKey<K, V>
{
    val: V,
    children: BTreeMap<K, V>,
}

impl<K, V> Tree<K, V>
where
    K: Ord,
    V: HasKey<K, V>,
{
    pub fn new(val: V) -> Tree<K, V> {
        Tree {
            val,
            children: BTreeMap::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn test_tree_01() {}
}
