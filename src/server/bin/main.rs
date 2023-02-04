use h2::{server, Reason};
use http::{Response, StatusCode};
use tokio::net::TcpListener;

#[tokio::main]
pub async fn main() {
    let listener = TcpListener::bind("127.0.0.1:5928").await.unwrap();

    loop {
        if let Ok((socket, _peer_addr)) = listener.accept().await {
            tokio::spawn(async {
                let mut h2 = server::handshake(socket).await.unwrap();
                h2.abrupt_shutdown(Reason::NO_ERROR);

                while let Some(request) = h2.accept().await {
                    let (request, mut respond) = request.unwrap();
                    println!("Received request: {request:?}");


                    let response = Response::builder().status(StatusCode::OK).body(()).unwrap();
                    respond.send_response(response, true).unwrap();
                }
            });
        }
    }
}
