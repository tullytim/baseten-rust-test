extern crate baseten;

use baseten::Baseten;
use futures::executor::block_on;
use std::{error::Error, collections::HashMap, env};

#[tokio::main]
async fn test() { 
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: baseten <api_key> <model>");
        return;
    }
    let api_key = &args[1];
    let model  = &args[2];

    let baseten = Baseten {
        api_key: api_key.to_string()
    };

    let prompt = String::from("A tree in a field under the night sky");

    let mut opts = HashMap::new();
    opts.insert(String::from("use_refiner"), String::from("true"));

    let r: Result<String, Box<dyn Error>> = block_on(baseten.call_model_prompt(&model, &prompt, Some(opts)));
    //let r: Result<String, Box<dyn Error>> = block_on(baseten.wake(&model));
    match r {
        Ok(s) => println!("{}", s),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    test();
}
