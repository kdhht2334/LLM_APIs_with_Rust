use kalosm::language::*;

#[tokio::main]
async fn main() {
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

// use kalosm::language::{prompt_input, TextStream};
// use llm_apis::chat::loop_chat;

// // run below ;)
// // LIBCLANG_PATH=D:/clang+llvm-18.1.8-x86_64-pc-windows-msvc/clang+llvm-18.1.8-x86_64-pc-windows-msvc/lib cargo run --release

// #[tokio::main]
// async fn main() {
//     let mut chat = loop_chat();

//     loop {
//         chat.add_message(prompt_input("\n> ").unwrap())
//             .await
//             .unwrap()
//             .to_std_out()
//             .await
//             .unwrap();
//     }
// }
