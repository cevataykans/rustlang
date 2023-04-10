#[cfg(test)]
use crate::custom_trees::BinaryTree;

#[test]
fn test_empty_tree() {
    let empty_tree = Box::<BinaryTree<i64>>::new(BinaryTree::Empty);
    empty_tree.print();
}
