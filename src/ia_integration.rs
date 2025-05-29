pub get_recomender_ia(mensaje: &str) -> String {
    let client = reqwest::Client::new();
    let api_key = "tu_api_key_aqui";
    let url = "https://openrouter.ai/api/v1/chat/completions";

    let request_body = json!({
        "model": "deepseek/deepseek-r1:free",
        "messages": [
            {
                "role": "user",
                "content": format!("{}",mensaje)
            }
        ]
    });

    format!("toen").to_string() // Placeholder for the actual request body|
    
    // match client.post(url)
    //     .header("Authorization
    //     ", format!("Bearer {}", api_key))
    //     .header("Content-Type", "application/json")
    //     .json(request_body)
    //     .send(){
    //         Ok(response) => {
    //             if response.status().is_success() {
    //             let response_text = response.text()?;
    //             let json_response: serde_json::Value = serde_json::from_str(&response_text)?;
    //             if let Some(content) = json_response["choices"][0]["message"]["content"].as_str() {
    //                 Ok(content.to_string())
    //             } else {
    //                 format!( "No content found in response")
    //
    //             }
    //             } else {
    //             format!("Failed to get a successful response")
    //             }
    //     }
    //         Err(e) =>  {
    //             format!("no response")
    //         }
    //      }
    //
}

