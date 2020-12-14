struct MyLinkedList {
    head: Option<Box<Node>>,
}

struct Node{
    val: i32,
    next: Option<Box<Node>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
use std::mem;

impl MyLinkedList {

    /** Initialize your data structure here. */
    fn new() -> Self {
        MyLinkedList{
            head: None,
        }        
    }
    
    fn print_list(&self) {
        let mut i = 0;
        let mut cur = &self.head;
        println!("start of the list:");
        
        while let Some(node) = cur{
            print!("{} -> ", node.val);
            cur = &node.next;
        }
        
        println!("end of the list");
        
    }
    
    /** Get the value of the index-th node in the linked list. If the index is invalid, return -1. */
    fn get(&self, index: i32) -> i32 {
        //println!("get index {}", index);
        //self.print_list();
        let mut i = 0;
        let mut cur = &self.head;
        
        while i < index {
            i += 1;
            match cur {
                None => return -1,
                Some(node) => {
                    //println!("index {}, value {}", index-1, node.val);
                    cur = &node.next;
                },
            }       
        }
        
        if cur.is_some(){
            return cur.as_ref().unwrap().val
        }      
        -1        
    }
    
    /** Add a node of value val before the first element of the linked list. After the insertion, the new node will be the first node of the linked list. */
    fn add_at_head(&mut self, val: i32) {
        let new_node = Box::new(Node{
            val: val,
            next: mem::replace(&mut self.head, None),
            });
        
        self.head = Some(new_node); 
        //println!("add at head {}", val);
        //self.print_list();
    }
    
    /** Append a node of value val to the last element of the linked list. */
    fn add_at_tail(&mut self, val: i32) {
        let mut cur = self.head.as_mut();  
        if cur.is_none(){
            return self.add_at_head(val);
        }
        
        while let Some(node) = cur {
            //let mut next_node = node.next.take();       
            if node.next.is_some(){
                cur = node.next.as_mut();
            }             
            else{
                let next_node = Some(Box::new(Node{
                    val: val,
                    next: None,
                }));
                node.next = next_node;
                break;
            }           
        } 
        
        //println!("add at tail {}", val);
        //self.print_list();
    }
    
    /** Add a node of value val before the index-th node in the linked list. If index equals to the length of linked list, the node will be appended to the end of linked list. If index is greater than the length, the node will not be inserted. */
    fn add_at_index(&mut self, index: i32, val: i32) {
        let mut i = 0;
        let mut cur = self.head.as_mut();
        
        if index == 0 {
            return self.add_at_head(val)
        }
        
        while i < index-1 {
            i += 1;
            match cur {
                None => break,
                Some(node) => cur = node.next.as_mut(),
            }                  
        }
        
        if cur.is_some(){
            let mut cur_node = cur.unwrap();
            let new_node = Some(Box::new(Node{
                val: val,
                next: cur_node.next.take(),
            }));
            cur_node.next = new_node;
        } 
        //println!("add at index {} val {}", index, val);
        //self.print_list();
    }
    
    /** Delete the index-th node in the linked list, if the index is valid. */
    fn delete_at_index(&mut self, index: i32) {
        let mut i = 0;
        let mut cur = self.head.as_mut();
        
        if index == 0 {
            //delete head
            let next_node = self.head.take(); 
            self.head = next_node.unwrap().next;
            self.print_list();
            return            
        }
        
        while i < index-1 {
            i += 1;
            match cur {
                None => break,
                Some(node) => cur = node.next.as_mut(),
            }                  
        }
        
        if cur.is_some(){
            
            let mut cur_node = cur.unwrap();
            let mut delete_node = cur_node.next.take();
            //println!("delete: cur is {}", cur_node.val);
            if delete_node.is_some(){
                cur_node.next = delete_node.unwrap().next;
                //println!("next after delete: {}", cur_node.next.as_ref().unwrap().val);
            }
        } 
        
        //println!("delete at index {}", index);
        //self.print_list();
        
    }
}

/**
 * Your MyLinkedList object will be instantiated and called as such:
 * let obj = MyLinkedList::new();
 * let ret_1: i32 = obj.get(index);
 * obj.add_at_head(val);
 * obj.add_at_tail(val);
 * obj.add_at_index(index, val);
 * obj.delete_at_index(index);
 */
