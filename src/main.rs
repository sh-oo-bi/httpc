#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res = client
        .get("https://shoobi.ir/randnum")
        .send()
        .await?
        .text()
        .await?;
    println!("{}", res);
    Ok(())
}
