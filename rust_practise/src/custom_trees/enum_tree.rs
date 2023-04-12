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

    pub fn iter(&self) -> TreeIter<T> {
        let mut iter = TreeIter {
            unvisited: Vec::new(),
        };
        iter.push_left_edge(&self);
        iter
    }
}

pub struct TreeIter<'a, T>
where
    T: Debug,
{
    unvisited: Vec<&'a TreeNode<T>>,
}

impl<'a, T: 'a> TreeIter<'a, T>
where
    T: Debug,
{
    fn push_left_edge(&mut self, mut tree: &'a BinaryTree<T>) {
        while let BinaryTree::NonEmpty(ref node) = *tree {
            self.unvisited.push(node);
            tree = &node.left;
        }
    }
}

impl<'a, T: 'a> IntoIterator for &'a BinaryTree<T>
where
    T: Debug + Ord,
{
    type Item = &'a T;
    type IntoIter = TreeIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T: 'a> Iterator for TreeIter<'a, T>
where
    T: Debug,
{
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let node = self.unvisited.pop()?;

        self.push_left_edge(&node.right);

        Some(&node.element)
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
