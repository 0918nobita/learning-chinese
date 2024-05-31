use clap::{Parser, Subcommand};
use indoc::formatdoc;
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum MessageContent {
    Text { text: String }
}

#[derive(Debug, Deserialize)]
struct MessageResponse {
    content: Vec<MessageContent>,
    #[allow(dead_code)]
    id: String,
}

#[derive(Deserialize)]
struct Env {
    anthropic_api_key: String,
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Generate {
        word: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = envy::from_env::<Env>()?;

    let cli = Cli::parse();

    match &cli.command {
        Commands::Generate { word } => generate_sentences(&env, word).await?,
    }

    Ok(())
}

async fn generate_sentences(env: &Env, word: &str) -> Result<(), reqwest::Error> {
    let json_schema = json!({
        "type": "array",
        "items": {
            "type": "object",
            "properties": {
                "ja": {
                    "type": "string",
                },
                "zh": {
                    "type": "string",
                }
            },
            "required": ["ja", "zh"],
            "additionalProperties": false,
        },
    });

    let json_schema = serde_json::to_string_pretty(&json_schema).unwrap();

    let prompt = formatdoc! {"
        あなたは中国語の教師です。
        たった今私は「{}」という単語を知りました。
        この単語を用いた20文字以内の例文を3つ生成し、バッククォート無しで以下のJSONスキーマに沿ったJSONだけを出力してください。
        ```json
        {}
        ```",
        word,
        json_schema
    };

    let request_body = json!({
        "model": "claude-3-opus-20240229",
        "max_tokens": 1024,
        "messages": [
            {
                "role": "user",
                "content": prompt
            }
        ],
    });

    let resp = reqwest::Client::new()
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", &env.anthropic_api_key)
        .header("anthropic-version", "2023-06-01")
        .json(&request_body)
        .send()
        .await?
        .json::<MessageResponse>()
        .await?;

    let MessageContent::Text { text } = resp.content.first().unwrap();

    let sentences = serde_json::from_str::<serde_json::Value>(&text).unwrap();

    println!("{}", serde_json::to_string_pretty(&sentences).unwrap());

    Ok(())
}
