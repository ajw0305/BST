fn bst_case1() {
    let input = vec![50, 15, 62, 80, 7, 54, 11];

    let output = bst::BstNode::bst_sort(&input);
    for it in output {
        print!("{}, ", it);
    }
    println!();
}

fn bst_case2() {
    let input = vec![50, 40, 30, 20, 10, 5, 1];

    let output = bst::BstNode::bst_sort(&input);
    println!();
}

fn main() {
    bst_case1();
    println!();
    bst_case2();
}