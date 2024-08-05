use kalosm::language::*;
use tokenizers::tokenizer::{Result, Tokenizer};

async fn chat() {
    let mut model = Llama::new_chat();
    let mut chat = Chat::builder(&mut model)
        .with_system_prompt("The assistant help answer questions based on the context given by the user. The model knows that the information the user gives it is always true.")
        .build();

    loop {
        chat.add_message(prompt_input("\n> ").unwrap())
            .await
            .unwrap()
            .to_std_out()
            .await
            .unwrap();
    }
}

fn token_encoding() -> Result<()> {
    use std::io::{stdin, stdout, Write};

    let tokenizer = Tokenizer::from_pretrained("bert-base-cased", None)?;

    loop {
        let mut user_input = String::new();
        print!("Please enter some text: ");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut user_input)
            .expect("Did not enter a correct string");

        if user_input.is_empty() {
            break;
        }

        let encoding = tokenizer.encode(user_input, false)?;
        println!("{:?}", encoding.get_tokens());
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let action: &str = "token";

    if action == "chat" {
        chat().await;
    } else if action == "token" {
        token_encoding();
    } else {
        println!("{:?} is not valid action!", action);
    }
}
