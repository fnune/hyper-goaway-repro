use hyper::{Client, Uri};

async fn make_request() -> Result<(), hyper::Error> {
    let client = Client::new();
    let res = client.get(Uri::from_static("http://127.0.0.1:5928")).await?;

    println!("status: {}", res.status());

    let buf = hyper::body::to_bytes(res).await?;

    println!("body: {buf:?}");

    Ok(())
}

#[tokio::main]
async fn main() {
    make_request().await.unwrap();
}
