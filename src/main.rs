use reqwest::Client;
use std;

pub async fn test() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let test_url = "https://www.google.com";
    let request = client.get(test_url);
    let response = request.send().await?;

    if response.status().is_success() {
        println!("request succeeded");
        let content = response.text().await?;
        println!("request content {:#?}", content);
        Ok(())
    } else {
        let status = response.status();
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Failed to read response text".to_string());
        eprintln!("Request failed with status: {} {:#?}", status, error_text);

        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Request failed",
        )))?
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    test().await?;
    Ok(())
}