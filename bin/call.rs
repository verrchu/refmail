use reqwest::blocking::Client;
use serde_json::json;

fn main() {
    dotenv::dotenv().expect("dotenv");

    let client = Client::new();

    let token = std::env::var("TOKEN").expect("token");

    let body = {
        let text = std::fs::read_to_string(std::env::args().nth(1).expect("text file"))
            .expect("read input");

        json!({
          "model": "text-davinci-002",
          "prompt": format!("rewrite: {}", text),
          "temperature": 0.7,
          "max_tokens": 256,
          "top_p": 1,
          "frequency_penalty": 0,
          "presence_penalty": 0
        })
    };

    let resp: serde_json::Value = client
        .post("https://api.openai.com/v1/completions")
        .bearer_auth(token)
        .json(&body)
        .send()
        .expect("send req")
        .json()
        .expect("json resp");

    serde_json::to_writer_pretty(std::io::stdout(), &resp).expect("print resp");
}
