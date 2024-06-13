use http::{Request, Response, StatusCode};

fn main() {
    let request = Request::builder()
    .uri("https://www/rust-lang.org/")
    .header("User-Agent", "awesome/1.0")
    .body(())
    .unwrap();

    println!("{:?}", request);

    let response = Response::builder()
      .status(StatusCode::MOVED_PERMANENTLY)
      .header("Location", "https://www.rust-lang.org/install.html")
      .body(())
      .unwrap();

    println!("{:?}", response);
}
