mod structure;
mod tool;

use crate::structure::bst::BstNode;
use crate::structure::tree::Node;
use crate::structure::tree::NodeLink;
use crate::structure::bst::BstNodeLink;
use crate::tool::generate_dotfile;
use crate::tool::generate_dotfile_bst;

fn main() {
    // test_binary_tree();
    test_binary_search_tree();
}

fn test_binary_search_tree() {
    let mut root: Option<BstNodeLink> = Some(BstNode::new_bst_nodelink(15));

    // Insert nodes
    BstNode::tree_insert(&mut root, 6);
    BstNode::tree_insert(&mut root, 18);
    BstNode::tree_insert(&mut root, 17);
    BstNode::tree_insert(&mut root, 20);
    BstNode::tree_insert(&mut root, 3);
    BstNode::tree_insert(&mut root, 7);
    BstNode::tree_insert(&mut root, 2);
    BstNode::tree_insert(&mut root, 4);
    BstNode::tree_insert(&mut root, 13);
    BstNode::tree_insert(&mut root, 9);

    // Simpan hasil BST setelah insert
    if let Some(ref root_unwrapped) = root {
        generate_dotfile_bst(&BstNode::get_root(root_unwrapped), "bst_after_insert.dot");
    }

    // Simpan hasil BST ke file (opsional tambahan)
    let rootlink = root.clone().unwrap();
    generate_dotfile_bst(&rootlink, "bst_graph.dot");

    // Tree search
    let search_keys = vec![15, 9, 22];
    for &key in &search_keys {
        print!("tree search result of key {} is ", key);
        if let Some(node_result) = BstNode::tree_search(&root, key) {
            println!("found -> {:?}", node_result.borrow().key);
        } else {
            println!("not found");
        }
    }

    // Min & Max
    let min_node = BstNode::minimum(&rootlink);
    println!("minimum result {:?}", min_node.borrow().key);

    let max_node = BstNode::maximum(&rootlink);
    println!("maximum result {:?}", max_node.borrow().key);

    // Root node
    let root_node = BstNode::get_root(&max_node);
    println!("root node {:?}", root_node.borrow().key);

    // Successor
    let query_keys = vec![2, 20, 15, 13, 9, 7, 22];
    for &key in &query_keys {
        if let Some(node) = BstNode::tree_search(&root, key) {
            print!("successor of node ({}) is ", key);
            if let Some(successor) = BstNode::tree_successor(&node) {
                println!("{:?}", successor.borrow().key);
            } else {
                println!("not found");
            }
        } else {
            println!("node with key of {} does not exist, failed to get successor", key);
        }
    }

    // ===== Transplant Test =====
    println!("\n===== Testing transplant =====");
    if let Some(node_6) = BstNode::tree_search(&root, 6) {
        if let Some(node_6_right) = node_6.borrow().right.clone() {
            // Perbaiki pemanggilan transplant sesuai dengan signature fungsi
            BstNode::transplant(&node_6, Some(node_6_right));
            println!("Transplanted node 6 with its right child.");
            // Simpan hasil BST setelah transplant
            if let Some(ref root_unwrapped) = root {
                generate_dotfile_bst(&BstNode::get_root(root_unwrapped), "bst_after_transplant.dot");
            }
        } else {
            println!("Node 6 has no right child; cannot perform transplant.");
        }
    } else {
        println!("Node 6 not found; cannot perform transplant.");
    }

    // Delete test
    println!("\n===== Testing tree_delete =====");
    if let Some(node_to_delete) = BstNode::tree_search(&root, 13) {
        BstNode::tree_delete(&mut root, &node_to_delete);
        println!("Node with key 13 deleted.");
    } else {
        println!("Node with key 13 not found, cannot delete.");
    }

    // Simpan hasil BST setelah delete
    if let Some(ref root_unwrapped) = root {
        generate_dotfile_bst(&BstNode::get_root(root_unwrapped), "bst_after_delete.dot");
    }
}

#[allow(dead_code)]
fn test_binary_tree() {
    let rootlink: NodeLink = Node::new_nodelink(5);

    rootlink.borrow_mut().add_left_child(&rootlink, 3);
    rootlink.borrow_mut().add_right_child(&rootlink, 7);

    let mut main_tree_path = "prime.dot";
    generate_dotfile(&rootlink, main_tree_path);

    let left_subtree = rootlink.borrow().left.clone();
    if let Some(left_tree_extract) = left_subtree {
        left_tree_extract.borrow_mut().add_left_child(&left_tree_extract, 2);
        left_tree_extract.borrow_mut().add_right_child(&left_tree_extract, 4);
    }

    let right_subtree = rootlink.borrow().right.clone();
    if let Some(ref right_tree_extract) = right_subtree {
        right_tree_extract.borrow_mut().add_right_child(&right_tree_extract, 10);
    }

    main_tree_path = "prime_t2.dot";
    generate_dotfile(&rootlink, main_tree_path);

    let recorded_depth = rootlink.borrow().tree_depth();
    println!("Current tree depth: {0}", recorded_depth);

    let total_nodes = rootlink.borrow().count_nodes();
    println!("Amount of nodes in current tree: {0}", total_nodes);

    if let Some(ref right_subtree_unwrapped) = right_subtree {
        let subtree_count = Node::count_nodes_by_nodelink(right_subtree_unwrapped, 0);
        println!("Amount of nodes in current subtree: {0}", subtree_count);
    }

    let left_subtree_found = rootlink.borrow().get_node_by_value(3);
    println!("left subtree seek by value {:?}", left_subtree_found);

    let another_left_subtree =
        rootlink.borrow().get_node_by_full_property(&left_subtree_found.as_ref().unwrap());
    println!(
        "left subtree seek by full property {:?}",
        another_left_subtree
    );

    let rootlink2 = rootlink.borrow().get_nodelink_copy();

    let flag = rootlink2.borrow_mut().discard_node_by_value(3);
    println!("status of node deletion: {0}", flag);

    main_tree_path = "prime_t3.dot";
    generate_dotfile(&rootlink2, main_tree_path);

    let depth_now = rootlink2.borrow().tree_depth();
    println!("Depth after discard {0}", depth_now);

    let count_now = rootlink2.borrow().count_nodes();
    println!("Count nodes after discard {0}", count_now);

    main_tree_path = "prime_t4.dot";
    generate_dotfile(&rootlink, main_tree_path);
}
