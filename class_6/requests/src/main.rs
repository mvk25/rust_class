#[tokio::main]

async fn main() -> Result<(), reqwest::Error> {
    let url = if Some(url) = std::env::args().nth(1) {
        url
    } else {
        println!("No CLI URL provided, using default.");
        "https://hyper.rs".into()
    }

    let res = reqwest::get(url).await?;
}

