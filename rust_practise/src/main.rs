mod custom_traits;
mod custom_trees;
mod event_system;

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

    let a = match i64::from_str_radix("100", 2) {
        Ok(v) => v,
        Err(e) => {
            println!("{:?}", e);
            std::process::exit(1);
        }
    };
    println!("{}", a);

    event_system::run_simulation();
}
