
use std::fmt::Display;
struct Node<T:Display>{
    data: T,
    next: Option<Box<Node<T>>>,
}

struct LinkedList<T: Display>{
    head: Option<Box<Node<T>>>,
}

impl<T: Display> LinkedList<T>{
    fn new() -> Self{
        LinkedList{
            head: None,
        }
    }

    // head -> new_node , new_node.next = previous head pointer
    fn push(&mut self, data: T){
        let new_node = Node{
            data,
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_node));
    }

    fn pop(&mut self) -> Option<T>{
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }

    fn traverse(&self){
        let mut current = &self.head;
        while let Some(node) = current{
            println!("{}", node.data);
            current = &node.next;
        }
    }
}
fn main() {
  
  let mut list = LinkedList::new();
  list.push(1);
  list.push(2);
  list.push(3);     
  list.traverse();
  list.pop();
  list.traverse();
}
