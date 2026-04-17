use maestro_http::{get, post};

#[tokio::main]
async fn main() {
    // GET
    let html = get("https://example.com")
        .timeout(10)
        .text()
        .await
        .unwrap();

    println!("HTML:\n{}", html);

    // POST JSON
    let data = serde_json::json!({
        "name": "Maestro"
    });

    let res = post("https://httpbin.org/post")
        .header("Content-Type", "application/json")
        .json(&data)
        .await
        .unwrap();

    println!("POST response:\n{}", res);

  }
