// ASYNC AND WAIT
// SERDE SERIALIZE AND DESERIALIZE
// USING MACROS

use std::process::Output;
use serde::{Serialize, Deserialize};

/*
ASYNC AWAIT CODE BLOCK STARTS

It is used to give control back to caller and allow other codes to execute and continue execution from where they left off
*/
async fn my_function(){
    println!("I'm an async function");
    let data = read_from_database().await; // calls wake function (like promise) untill the results are available
    println!("{}",data);
}

async fn read_from_database() -> String{
    "Result from database".to_owned()
}
// The implementation of async looks like this
/*
fn my_function()-> impl Future<Output=()>{
        println!("I'm an async function");
}

enum Poll<T> {
    Ready(T),
    Pending,
}
pub trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
 */
/*
ASYNC AWAIT CODE BLOCK ENDS
*/


/*
SERDE SERIALIZE AND DESERIALIZE
*/
#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u32,
    username: String,
    is_active: bool,
} 

#[tokio::main] // attribute macro to allow main to be async
async fn main() {
    println!("Hello, world!");
    let val = my_function();
    println!("Lazy async function");
    val.await;
    /*
    ASYNC AWAIT CODE BLOCK STARTS
    */
    // Since we can not call the async functions from a non async function we need a executer in top level called Tokio


    /*
    ASYNC AWAIT CODE BLOCK ENDS


    */



    // This attribute macro tells Rust to automatically write 
// the code needed to turn this struct into JSON and back.


    // --- PART 1: SERIALIZATION (Struct -> JSON) ---
    let my_user = User {
        id: 1,
        username: String::from("ferris_the_crab"),
        is_active: true,
    };

    // Convert the Rust struct to a JSON string
    let json_string = serde_json::to_string(&my_user).unwrap();

    println!("As JSON string: {}", json_string);


    // --- PART 2: DESERIALIZATION (JSON -> Struct) ---
    let incoming_json = r#"
        {
            "id": 42,
            "username": "rust_ace",
            "is_active": false
        }
    "#;

    // Convert the JSON string back into a Rust struct
    let parsed_user: User = serde_json::from_str(incoming_json).unwrap();

    println!("As Rust struct: {:?}", parsed_user);
    println!("The username is: {}", parsed_user.username);
}
