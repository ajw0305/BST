pub struct BstNode<T> {
    value: T,
    left: Option<Box<BstNode<T>>>,
    right: Option<Box<BstNode<T>>>,
}

// Type T는 Ord와 Clone trait을 만족해야 함.
impl<T: Ord + Clone + std::fmt::Display> BstNode<T> {
    pub fn build(value: T) -> Self {
        BstNode {
            value,
            left: None,
            right: None,
        }
    }

    pub fn print_tree_horizontal(&self) {
        self.print_subtree_horizontal(0, "Root".to_string(), true);
    }

    fn print_subtree_horizontal(&self, indent: usize, label: String, is_last: bool) {
        // 현재 노드 출력
        let prefix = "    ".repeat(indent);
        let branch = if indent == 0 {
            ""
        } else if is_last {
            "└── "
        } else {
            "├── "
        };
        println!("{}{}{} ({})", prefix, branch, self.value, label);

        // 오른쪽 먼저 출력
        if let Some(ref right) = self.right {
            right.print_subtree_horizontal(indent + 1, "R".to_string(), self.left.is_none());
        }
        // 왼쪽 출력
        if let Some(ref left) = self.left {
            left.print_subtree_horizontal(indent + 1, "L".to_string(), true);
        }
    }

    pub fn insert_node(&mut self, node: BstNode<T>) -> Result<String, String>{        
        if self.value > node.value {
            if let Some(ref mut left_child) = self.left {
                left_child.insert_node(node)
            } else {
                self.left = Some(Box::new(node));
                Ok(String::from("OK"))
            }
        } else if self.value < node.value {
            if let Some(ref mut right_child) = self.right {
                right_child.insert_node(node)
            } else {
                self.right = Some(Box::new(node));
                Ok(String::from("OK"))
            }
        } else {
            Err(String::from("Exist value in Tree"))
        } 
    }

    pub fn insert(&mut self, value: T) {
        let new_node = BstNode::build(value);
        let result = self.insert_node(new_node);
        if let Err(msg) = result {
            println!("Err msg: {}", msg);
        }
    }
    
    fn sort_in_order(&mut self, vec: &mut Vec<T>) {
        if let Some(left_child) = &mut self.left {
            left_child.sort_in_order(vec);
        }
        vec.push(self.value.clone());
        if let Some(right_child) = &mut self.right {
            right_child.sort_in_order(vec);
        }
    }

    pub fn bst_sort(vec: &Vec<T>) -> Vec<T>{
        let mut root: BstNode<T> = BstNode::build(vec[0].clone());
        let mut sorted_vec: Vec<T> = Vec::new();
        // root.print_tree_horizontal();
        for index in 1..vec.len() {
            root.insert(vec[index].clone());
            // root.print_tree_horizontal();
        }
        
        root.sort_in_order(&mut sorted_vec);

        sorted_vec
    }


}

// 테스트 빌드 시에만 컴파일
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_test() {
        let node: BstNode<i32> = BstNode::build(123);
        assert_eq!(node.value, 123);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut root = BstNode::build(10);
        root.insert(5);
        let result = root.insert_node(BstNode::build(5));

        assert_eq!(result, Err(String::from("Exist value in Tree")));
    }

    #[test]
    fn test_bst_sort() {
        let input = vec![10, 5, 15, 3, 7, 12, 18];

        let output = BstNode::bst_sort(&input);

        // 예상되는 오름차순 결과
        let expected = vec![3, 5, 7, 10, 12, 15, 18];
        assert_eq!(output, expected);
    }
    
    #[test]
    fn test_bst_sort2() {
        let input = vec![50, 15, 62, 80, 7, 54, 11];

        let output = BstNode::bst_sort(&input);

        // 예상되는 오름차순 결과
        let expected = vec![7, 11, 15, 50, 54, 62, 80];
        assert_eq!(output, expected);
    }

}
