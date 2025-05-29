use serde_json::json;

use super::response_model::ApiResponse;

pub fn get_recomender_ia(mensaje: &str, api_key: &str, pront: &str, url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::blocking::Client::new();

    let request_body = json!({
        "model": "deepseek/deepseek-r1:free",
        "messages": [
            {
                "role": "user",
                "content": format!("{} {}",mensaje, pront)
            }
        ]
    });

    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()?
        .text()?;

    Ok(response)
}

pub fn parse_response(json_str: &str) -> Result<ApiResponse, serde_json::Error> {
    serde_json::from_str(json_str)
}
