#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let url = if let Some() = std::env::args().nth(1) {
        url
    } else {
        println!("No CLI URL provided, using default.");
        "https://hyper.rs".into()
    };
}

// use reqwest::Client;

// #[tokio::main]
// async fn main() {
//     // Create a new HTTP client
//     let client = Client::new();

//     // Send a GET request to the specified URL
//     let res = client.get("https://www.rust-lang.org")
//         .send()
//         .await
//         .unwrap();

//     // Check the response status
//     if res.status().is_success() {
//         // Read the response body as text
//         let body = res.text().await.unwrap();
//         println!("Response body:\n{}", body);
//     } else {
//         println!("Request failed with status code: {}", res.status());
//     }
// }


// use error_chain::error_chain;
use reqwest::Error;

// error_chain! {
//     foreign_links {
//         Io(std::io::Error);
//         HttpRequest(reqwest::Error);
//     }
// }