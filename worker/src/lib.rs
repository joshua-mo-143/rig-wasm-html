mod utils;

use rig::completion::Prompt;
use rig::providers::openai::GPT_4O;
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {}

#[wasm_bindgen]
pub async fn send_prompt(openai_api_key: String) -> JsValue {
    set_panic_hook();

    let client = rig::providers::openai::Client::new(&openai_api_key);

    let agent = client.agent(GPT_4O).build();

    // Prompt the model and print its response
    let response = agent
        .prompt("Who are you?")
        .await
        .expect("Failed to prompt GPT-4");

    response.into()
}
