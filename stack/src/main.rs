#[derive(Debug)]
struct Item{
    id: i32,
    name: String,
    price: f32,
    quantity: i32,
}



#[derive(Debug)]
struct Stack{
    items: Vec<Item>,
    size: i32,
    base: i32,
    top: i32
}


impl Stack {
    fn new() -> Stack {
        Stack {
            items: Vec::new(),
            size: 0,
            base: 0,
            top: 0
        }
    }
    fn push(&mut self,item: Item){
        self.items.push(item); // adding new item to the vector stack
        self.size += 1;
        self.top += 1;
    }
    
    fn pop(&mut self) -> Option<Item>{
        if self.size == 0{
            return None;
        }
        self.size -= 1;
        self.top -= 1;
        self.items.pop()
    }
    fn display(&self){
        for item in &self.items{
            println!("{:?}",item);
        }
    }
}


fn main() {
    let mut stack = Stack::new();
    stack.push(Item{id: 1, name: "xxx".to_string(), price: 10.0, quantity: 1});
    stack.push(Item{id: 2, name: "asd".to_string(), price: 10.0, quantity: 1});
    stack.display();
    stack.pop();
    stack.display();

}
