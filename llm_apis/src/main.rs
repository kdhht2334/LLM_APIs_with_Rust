use std::io::{stdin, stdout, Write};

use llm_apis::chat::oneonone::communicate;
use llm_apis::tokenize::encoding::{test_tokenizer, train_tokenizer};

#[tokio::main]
async fn main() {
    let mut input: String = String::new();
    print!("Enter your action: ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut input)
        .expect("Did not enter a correct string");
    let action = input.trim();

    // let mut action: &str = "tokenize";

    if action == "chat" {
        communicate().await;
    } else if action == "test" {
        let root_path: &str = "/path/to/tokenizer";
        let _ = test_tokenizer(root_path);
    } else if action == "train" {
        let vocab_size: usize = 10_000;
        let root_path: &str = "/path/to/tokenizer";
        let _ = train_tokenizer(vocab_size, root_path);
    } else {
        println!("{:?} is not valid action!", action);
    }
}
