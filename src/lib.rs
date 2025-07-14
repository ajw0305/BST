use std::fmt::Error;
use std::fmt::Debug;

#[derive(Debug)]
pub struct BstNode<T> {
    value: T,
    left: Option<Box<BstNode<T>>>,
    right: Option<Box<BstNode<T>>>,
}

impl<T: Ord + Debug> BstNode<T> {
    pub fn build(value: T) -> Self {
        BstNode {
            value,
            left: None,
            right: None,
        }
    }
    
    pub fn insert_node(&mut self, node: BstNode<T>) -> Result<String, String>{        
        if self.value > node.value {
            if let Some(ref mut left_child) = self.left {
                left_child.insert_node(node);
            } else {
                self.left = Some(Box::new(node));
            }
            Ok(String::from("OK"))
        } else if self.value < node.value {
            if let Some(ref mut right_child) = self.right {
                right_child.insert_node(node);
            } else {
                self.right = Some(Box::new(node));
            }
            Ok(String::from("OK"))
        } else {
            Err(String::from("Exist value in Tree"))
        } 
    }

    pub fn insert(&mut self, value: T) {
        let new_node = BstNode::build(value);
        self.insert_node(new_node);
    }

    pub fn in_order_traversal(&self) {
        if let Some(left_child) = &self.left {
            left_child.in_order_traversal();
        }
        println!("{:?}", self.value);
        if let Some(right_child) = &self.right {
            right_child.in_order_traversal();
        }
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
