use tiny_http::{Server, Response};

fn main() {
    let server = Server::http("0.0.0.0:8000").unwrap();
    
    for request in server.incoming_requests() {
        println!("received request! method: {:?}, url: {:?}",
            request.method(),
            request.url(),
        );
    
        let response = Response::from_string("hello world");
        request.respond(response);
    }
}
