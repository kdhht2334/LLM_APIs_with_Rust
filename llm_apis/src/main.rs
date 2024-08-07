use kalosm::language::*;
use tokenizers::models::bpe::{BpeTrainerBuilder, BPE};
use tokenizers::normalizers::{strip::Strip, unicode::NFC, utils::Sequence};
use tokenizers::pre_tokenizers::byte_level::ByteLevel;
use tokenizers::tokenizer::Tokenizer;
use tokenizers::{AddedToken, Result, TokenizerBuilder};

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

fn train() -> Result<()> {
    let vocab_size: usize = 100;
    let text_file_fp: &str = "path/to/vocab.txt";
    let save_path: &str = "path/to/tokenizer.json";

    let mut trainer = BpeTrainerBuilder::new()
        .show_progress(true)
        .vocab_size(vocab_size)
        .min_frequency(0)
        .special_tokens(vec![
            AddedToken::from(String::from("<s>"), true),
            AddedToken::from(String::from("<pad>"), true),
            AddedToken::from(String::from("</s>"), true),
            AddedToken::from(String::from("<unk>"), true),
            AddedToken::from(String::from("<mask>"), true),
        ])
        .build();

    let mut tokenizer = TokenizerBuilder::new()
        .with_model(BPE::default())
        .with_normalizer(Some(Sequence::new(vec![
            Strip::new(true, true).into(),
            NFC.into(),
        ])))
        .with_pre_tokenizer(Some(ByteLevel::default()))
        .with_post_processor(Some(ByteLevel::default()))
        .with_decoder(Some(ByteLevel::default()))
        .build()?;

    let pretty = false;
    tokenizer
        .train_from_files(&mut trainer, vec![text_file_fp.to_string()])?
        .save(save_path, pretty)?;

    Ok(())
}

#[tokio::main]
async fn main() {
    let action: &str = "train";

    if action == "chat" {
        chat().await;
    } else if action == "tokenize" {
        let _ = token_encoding();
    } else if action == "train" {
        let _ = train();
    } else {
        println!("{:?} is not valid action!", action);
    }
}
