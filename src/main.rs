#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    let mut list = SinglyLinkedList::new();
    list.add_start(20);
    list.add_start(50);
    list.add_start(60);
    list.add_end(70);
    list.add_end(100);
    
    let vec = list.as_vec();
    println!("{:?}", vec);

}

struct SinglyLinkedList {
    head: Option<Box<SinglyLinkedNode>>,
}

impl SinglyLinkedList {
    pub fn new() -> SinglyLinkedList {
        SinglyLinkedList {head: None}
    }

    pub fn add_end(&mut self, data: i32) {
        SinglyLinkedNode::add_end(&mut self.head, data);
    }

    pub fn add_start(&mut self, data: i32) {
        SinglyLinkedNode::add_start(&mut self.head, data);
    }

    pub fn remove_start(&mut self) {
        SinglyLinkedNode::remove_start(&mut self.head);
    }

    pub fn as_vec(&self) -> Vec<i32> {
        SinglyLinkedNode::as_vec(&self.head)
    }

}

#[derive(Debug)] #[derive(PartialEq)]
struct SinglyLinkedNode {
    data: i32,
    next: Option<Box<SinglyLinkedNode>>,
}

impl SinglyLinkedNode {
    pub fn new(data: i32) -> Option<Box<SinglyLinkedNode>> {
        let node = SinglyLinkedNode{data, next: None,};
        let boxed_node = Box::new(node);
        Some(boxed_node)
    }

    pub fn add_end(head: &mut Option<Box<SinglyLinkedNode>>, data: i32) {
        match head {
            Some(boxed_node) => {
                let mut iter = boxed_node;
                let mut last_next = &mut iter.next; // iter.next auto-dereferences.
                // loop while last_next is not None.
                while let Some(next_node) = last_next { 
                    iter = next_node; // continue to next node.
                    last_next = &mut iter.next; // set last_next in case of loop termination.
                }
                // in this case we know that last_next is a reference to None. dereference and add new.
                *last_next = SinglyLinkedNode::new(data);
            }
            None => *head = SinglyLinkedNode::new(data),
        }
    }

    pub fn as_vec(head: &Option<Box<SinglyLinkedNode>>) -> Vec<i32> {
        let mut vec = Vec::new(); // return vector of i32

        if let Some(boxed_node) = head {
                let mut iter = boxed_node;

                while let Some(next_node) = &iter.next {
                    vec.push(iter.data);
                    iter = next_node;
                }
                // since we stopped at last node, we need to add last node's data as well.
                vec.push(iter.data);
        }

        return vec;
    }

    pub fn add_start(head: &mut Option<Box<SinglyLinkedNode>>, data: i32) {
        match head { 
            Some(boxed_node) => {
                let new_head = SinglyLinkedNode::new(data).unwrap();
                let old_head = std::mem::replace(boxed_node, new_head); // replace the content of head with new_head
                boxed_node.next = Some(old_head); // set head.next to replaced old_head content
            }
            None =>  {
                *head = SinglyLinkedNode::new(data);
            }
        }
    }

    pub fn remove_start(head: &mut Option<Box<SinglyLinkedNode>>) {
        if let Some(mut boxed_node) = head.take() {
            *head = boxed_node.next.take();
        }
    }

    pub fn remove_end(head: &mut Option<Box<SinglyLinkedNode>>) { todo!(); }

        
// end of impl node
}

// -------------------------- TESTS ---------------------------
#[test]
fn add_start_once() {
    let mut node = SinglyLinkedNode::new(20);
    SinglyLinkedNode::add_start(&mut node, 40);
    let vec = SinglyLinkedNode::as_vec(&node);
    assert_eq!(vec, vec![40,20]);
}

#[test]
fn add_start_twice() {
    let mut node = SinglyLinkedNode::new(20);
    SinglyLinkedNode::add_start(&mut node, 40);
    SinglyLinkedNode::add_start(&mut node, 60);
    let vec = SinglyLinkedNode::as_vec(&node);
    assert_eq!(vec, vec![60, 40,20]);
}

#[test]
fn add_start_thrice() {
    let mut node = SinglyLinkedNode::new(20);
    SinglyLinkedNode::add_start(&mut node, 40);
    SinglyLinkedNode::add_start(&mut node, 60);
    SinglyLinkedNode::add_start(&mut node, 100);
    let vec = SinglyLinkedNode::as_vec(&node);
    assert_eq!(vec, vec![100, 60, 40,20]);
}

#[test]
fn add_end_once() {
    let mut node = SinglyLinkedNode::new(20);
    SinglyLinkedNode::add_end(&mut node, 40);
    let vec = SinglyLinkedNode::as_vec(&node);
    assert_eq!(vec, vec![20,40]);
}

#[test]
fn add_end_twice() {
    let mut node = SinglyLinkedNode::new(1);
    SinglyLinkedNode::add_end(&mut node, 2);
    SinglyLinkedNode::add_end(&mut node, 3);
    let vec = SinglyLinkedNode::as_vec(&node);
    assert_eq!(vec, vec![1,2,3]);
}

#[test]
fn add_end_thrice() {
    let mut node = SinglyLinkedNode::new(1);
    SinglyLinkedNode::add_end(&mut node, 2);
    SinglyLinkedNode::add_end(&mut node, 3);
    SinglyLinkedNode::add_end(&mut node, 4);
    let vec = SinglyLinkedNode::as_vec(&node);
    assert_eq!(vec, vec![1,2,3,4]);
}

#[test]
fn add_twice_end_twice_start() {
    let mut node = SinglyLinkedNode::new(11);
    SinglyLinkedNode::add_end(&mut node, 2);
    SinglyLinkedNode::add_end(&mut node, 3);
    SinglyLinkedNode::add_start(&mut node, 40);
    SinglyLinkedNode::add_start(&mut node, 60);
    let vec = SinglyLinkedNode::as_vec(&node);
    assert_eq!(vec, vec![60,40,11,2,3]);
}

#[test]
fn add_twice_start_twice_end() {
    let mut node = SinglyLinkedNode::new(11);
    SinglyLinkedNode::add_start(&mut node, 60);
    SinglyLinkedNode::add_start(&mut node, 40);
    SinglyLinkedNode::add_end(&mut node, 3);
    SinglyLinkedNode::add_end(&mut node, 2);
    let vec = SinglyLinkedNode::as_vec(&node);
    assert_eq!(vec, vec![40,60,11,3,2]);
}

#[test]
fn remove_start_once() {
    let mut node = SinglyLinkedNode::new(1);
    SinglyLinkedNode::add_end(&mut node, 2);
    SinglyLinkedNode::add_end(&mut node, 3);
    SinglyLinkedNode::add_end(&mut node, 4);
    SinglyLinkedNode::add_end(&mut node, 5);
    SinglyLinkedNode::add_end(&mut node, 6);
    SinglyLinkedNode::remove_start(&mut node);

    let vec = SinglyLinkedNode::as_vec(&node);
    assert_eq!(vec, vec![2,3,4,5,6]);
}

#[test]
fn remove_start_thrice() {
    let mut node = SinglyLinkedNode::new(1);
    SinglyLinkedNode::add_end(&mut node, 2);
    SinglyLinkedNode::add_end(&mut node, 3);
    SinglyLinkedNode::add_end(&mut node, 4);
    SinglyLinkedNode::add_end(&mut node, 5);
    SinglyLinkedNode::add_end(&mut node, 6);
    SinglyLinkedNode::remove_start(&mut node);
    SinglyLinkedNode::remove_start(&mut node);
    SinglyLinkedNode::remove_start(&mut node);

    let vec = SinglyLinkedNode::as_vec(&node);
    assert_eq!(vec, vec![4,5,6]);
}

#[test]
fn remove_start_thrice_before_add_end_thrice() {
    let mut node = SinglyLinkedNode::new(1);
    SinglyLinkedNode::add_end(&mut node, 2);
    SinglyLinkedNode::add_end(&mut node, 3);
    SinglyLinkedNode::add_end(&mut node, 4);
    SinglyLinkedNode::add_end(&mut node, 5);
    SinglyLinkedNode::add_end(&mut node, 6);

    let vec = SinglyLinkedNode::as_vec(&node);
    assert_eq!(vec, vec![1,2,3,4,5,6]);

    SinglyLinkedNode::remove_start(&mut node);
    SinglyLinkedNode::remove_start(&mut node);
    SinglyLinkedNode::remove_start(&mut node);

    let vec = SinglyLinkedNode::as_vec(&node);
    assert_eq!(vec, vec![4,5,6]);

    SinglyLinkedNode::add_end(&mut node, 7);
    SinglyLinkedNode::add_end(&mut node, 8);
    SinglyLinkedNode::add_end(&mut node, 9);

    let vec = SinglyLinkedNode::as_vec(&node);
    assert_eq!(vec, vec![4,5,6,7,8,9]);
}

#[test]
fn remove_start_thrice_before_add_start_thrice() {
    let mut node = SinglyLinkedNode::new(1);
    SinglyLinkedNode::add_end(&mut node, 2);
    SinglyLinkedNode::add_end(&mut node, 3);
    SinglyLinkedNode::add_end(&mut node, 4);
    SinglyLinkedNode::add_end(&mut node, 5);
    SinglyLinkedNode::add_end(&mut node, 6);

    let vec = SinglyLinkedNode::as_vec(&node);
    assert_eq!(vec, vec![1,2,3,4,5,6]);

    SinglyLinkedNode::remove_start(&mut node);
    SinglyLinkedNode::remove_start(&mut node);
    SinglyLinkedNode::remove_start(&mut node);

    let vec = SinglyLinkedNode::as_vec(&node);
    assert_eq!(vec, vec![4,5,6]);

    SinglyLinkedNode::add_start(&mut node, -10);
    SinglyLinkedNode::add_start(&mut node, -20);
    SinglyLinkedNode::add_start(&mut node, -30);

    let vec = SinglyLinkedNode::as_vec(&node);
    assert_eq!(vec, vec![-30,-20,-10,4,5,6]);
}

#[test]
fn remove_start_thrice_before_add_start_thrice_before_add_end_thrice() {
    let mut node = SinglyLinkedNode::new(1);
    SinglyLinkedNode::add_end(&mut node, 2);
    SinglyLinkedNode::add_end(&mut node, 3);
    SinglyLinkedNode::add_end(&mut node, 4);
    SinglyLinkedNode::add_end(&mut node, 5);
    SinglyLinkedNode::add_end(&mut node, 6);

    let vec = SinglyLinkedNode::as_vec(&node);
    assert_eq!(vec, vec![1,2,3,4,5,6]);

    SinglyLinkedNode::remove_start(&mut node);
    SinglyLinkedNode::remove_start(&mut node);
    SinglyLinkedNode::remove_start(&mut node);

    let vec = SinglyLinkedNode::as_vec(&node);
    assert_eq!(vec, vec![4,5,6]);

    SinglyLinkedNode::add_start(&mut node, -10);
    SinglyLinkedNode::add_start(&mut node, -20);
    SinglyLinkedNode::add_start(&mut node, -30);

    let vec = SinglyLinkedNode::as_vec(&node);
    assert_eq!(vec, vec![-30,-20,-10,4,5,6]);

    SinglyLinkedNode::add_end(&mut node, -40);
    SinglyLinkedNode::add_end(&mut node, -50);
    SinglyLinkedNode::add_end(&mut node, -60);

    let vec = SinglyLinkedNode::as_vec(&node);
    assert_eq!(vec, vec![-30,-20,-10,4,5,6,-40,-50,-60]);
}

#[test]
fn remove_start_none_head() {
    let mut node: Option<Box<SinglyLinkedNode>> = None;
    SinglyLinkedNode::remove_start(&mut node);
    
    let vec = SinglyLinkedNode::as_vec(&node);
    assert_eq!(vec, vec![]);
}

#[test]
fn remove_start_none_head_before_add_end() {
    let mut node: Option<Box<SinglyLinkedNode>> = None;
    SinglyLinkedNode::remove_start(&mut node);
    
    let vec = SinglyLinkedNode::as_vec(&node);
    assert_eq!(vec, vec![]);

    SinglyLinkedNode::add_end(&mut node, 10);

    let vec = SinglyLinkedNode::as_vec(&node);
    assert_eq!(vec, vec![10]);
}


#[test]
fn remove_start_none_head_before_add_start() {
    let mut node: Option<Box<SinglyLinkedNode>> = None;
    SinglyLinkedNode::remove_start(&mut node);
    
    let vec = SinglyLinkedNode::as_vec(&node);
    assert_eq!(vec, vec![]);

    SinglyLinkedNode::add_start(&mut node, -10);

    let vec = SinglyLinkedNode::as_vec(&node);
    assert_eq!(vec, vec![-10]);
}





