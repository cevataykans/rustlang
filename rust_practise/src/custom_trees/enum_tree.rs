use std::fmt::Debug;

// An ordered collection of `T`s.
pub enum BinaryTree<T: Debug> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}
// A part of a BinaryTree.
pub struct TreeNode<T: Debug> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

impl<T> BinaryTree<T>
where
    T: Ord + std::fmt::Debug,
{
    pub fn add(&mut self, value: T) {
        match *self {
            BinaryTree::Empty => {
                *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                    element: value,
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty,
                }));
            }
            BinaryTree::NonEmpty(ref mut node) => {
                if node.element > value {
                    node.left.add(value);
                } else {
                    node.right.add(value);
                }
            }
        };
    }
}

impl<T> BinaryTree<T>
where
    T: Debug,
{
    pub fn print(&self) {
        match *self {
            BinaryTree::Empty => {}
            BinaryTree::NonEmpty(ref node) => {
                node.left.print();
                println!("{:?}", node.element);
                node.right.print();
            }
        };
    }
}
