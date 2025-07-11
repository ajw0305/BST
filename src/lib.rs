struct BstNode<T> {
    value: T,
    left: Option<Box<BstNode<T>>>,
    right: Option<Box<BstNode<T>>>,
}

impl<T: PartialEq + PartialOrd> BstNode<T> {
    fn build(value: T) -> Self {
        BstNode {
            value,
            left: None,
            right: None,
        }
    }
    
    fn insert_node(&self, node: BstNode<T>) -> Result<T, String>{
        let mut cur_node = self;
        
        if cur_node.value > node.value {
            if let None = cur_node.left {
                cur_node.left = node;
            }
            Ok(node.value)
        } else if(cur_node.value < node.value) {
            Ok(node.value)
        } else {
            Err(String::from("Exist value in Tree"))
        } 
    }

    fn insert(&self, value: T) {
        let new_node = BstNode::build(value);
        
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_test() {
        let node: BstNode<i32> = BstNode::build(123);
        assert_eq!(node.value, 123);
    }


}
