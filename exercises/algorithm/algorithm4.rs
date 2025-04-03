/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

//I AM NOT DONE
use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T)
    where
        T: Ord + Copy,
    {
        match &mut self.root {
            None => {
                self.root = Some(Box::new(TreeNode::new(value)));
                return;
            },
            Some(root) => root.insert(value),
        }
        let node = Some(Box::new(TreeNode::new(value)));
        let mut current_ptr = self.root.as_mut();
        while let Some(ptr) = current_ptr {
            if value < ptr.value {
                if ptr.left.is_none() {
                    ptr.left = node;
                    break;
                }
                current_ptr = ptr.left.as_mut();
            } else if value > ptr.value {
                if ptr.right.is_none() {
                    ptr.right = node;
                    break;
                }
                current_ptr = ptr.right.as_mut();
            } else {
                break;
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool
    where
        T: std::cmp::PartialOrd + std::fmt::Debug + Copy,
    {
        if self.root.is_none() {
            return false;
        }
        let mut current = self.root.as_ref();
        while let Some(current_ptr) = current {
            if value < current_ptr.value {
                if current_ptr.left.is_none() {
                    return false;
                }
                current = current_ptr.left.as_ref();
            } else if value > current_ptr.value {
                if current_ptr.right.is_none() {
                    return false;
                }
                current = current_ptr.right.as_ref();
            } else { return true; }
        }
        return false;
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {

    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();


        assert_eq!(bst.search(1), false);


        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);


        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);


        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();


        bst.insert(1);
        bst.insert(1);


        assert_eq!(bst.search(1), true);


        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


