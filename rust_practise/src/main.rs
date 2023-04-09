mod custom_trees;

use custom_trees::BinaryTree;

fn main() {
    println!("Hello, world!");

    let mut my_tree = Box::<BinaryTree<i64>>::new(BinaryTree::Empty);
    my_tree.add(89);
    my_tree.add(2);
    my_tree.add(108);
    my_tree.add(1);
    my_tree.add(5);
    my_tree.add(92);
    my_tree.print();
}
