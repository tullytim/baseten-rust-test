extern crate baseten;

use baseten::Baseten;
use futures::executor::block_on;
use std::{error::Error, collections::HashMap, env};

#[tokio::main]
 async fn test() { 

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: baseten <api_key>");
        return;
    }
    let  api_key = &args[1];

    let baseten = Baseten {
        api_key: api_key.to_string()
    };

    let model = String::from("yqe4MdP");
    let prompt = String::from("A tree in a field under the night sky");

    let mut opts = HashMap::new();
    opts.insert(String::from("use_refiner"), String::from("true"));

    let r: Result<String, Box<dyn Error>> = block_on(baseten.call_model_prompt(&model, &prompt, Some(opts)));
    match r {
        Ok(s) => println!("{}", s),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    test();
}
