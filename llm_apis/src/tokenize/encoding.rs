use std::io;
use tokenizers::models::bpe::{BpeTrainerBuilder, BPE};
use tokenizers::normalizers::{strip::Strip, unicode::NFC, utils::Sequence};
use tokenizers::pre_tokenizers::byte_level::ByteLevel;
use tokenizers::tokenizer::Tokenizer;
use tokenizers::{AddedToken, Result, TokenizerBuilder};

fn make_string(a: &str, b: &str) -> String {
    format!("{a}/{b}")
}

pub fn test_tokenizer(root_path: &str) -> Result<()> {
    print!("You entered test phase.");

    // let tokenizer = Tokenizer::from_pretrained(make_string(root_path, "/tokenizer.json"), None)?;
    let tokenizer = Tokenizer::from_file(make_string(root_path, "/tokenizer.json"))?;

    loop {
        let mut input: String = String::new();
        print!("Please enter some text: ");
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        let user_input = input.trim();
        // let _ = stdout().flush();
        // stdin()
        //     .read_line(&mut user_input)
        //     .expect("Did not enter a correct string");

        if user_input.is_empty() {
            break;
        }

        let encoding = tokenizer.encode(user_input, false)?;
        println!("{:?}", encoding.get_tokens());
    }

    Ok(())
}

pub fn train_tokenizer(vocab_size: usize, root_path: &str) -> Result<()> {
    print!("You entered train phase.");

    let text_file_fp: String = make_string(root_path, "/test.txt");
    let save_path: String = make_string(root_path, "/test.json");

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
