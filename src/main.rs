use hyper::http::StatusCode;
/// This code sets up a simple HTTP server that listens on 127.0.0.1:3000.
/// It responds with "Hello, world!" on the root path ("/") and returns a 404 Not Found status for any other path.
/// To run the server, simply execute cargo run in your terminal.
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::sync::oneshot;

#[tokio::main] // Marks the main function as a Tokio runtime to support asynchronous operations.

async fn main() { // Declares an async main function.
    let addr = SocketAddr::from(([0, 0, 0, 0], 80)); // Creates a new SocketAddr that can be used to bind to IP and port.

    let make_svc = // Creates a new service using a closure that accepts a connection and returns a future of Result<Response<Body>, Error>.
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle_request)) });

    let server = Server::bind(&addr).serve(make_svc); // Starts a new HTTP server bound with the provided socket address.

    println!("Listening on http://{}", addr); // Prints a message indicating the server is ready.
    println!("Press ENTER to exit"); // Prints a message indicating how to stop the server.

    let (tx, rx) = oneshot::channel::<()>(); // Sets up a oneshot channel to communicate between threads.

    tokio::spawn(async move { // Spawns a new thread and provides it with an anonymous async block.
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap(); // Reads input from stdin to allow user to terminate the server.
        let _ = tx.send(()); // Sends a message to the main thread to indicate a termination request.
    });

    let result = tokio::select! { // Waits for either the server to terminate or a termination request to be received.
        result = server => result,
        _ = rx => {
            println!("Shutting down...");
            Ok(())
        }
    };

    if let Err(e) = result { // Prints any errors that occurred during server execution.
        eprintln!("Server error: {}", e);
    }
}

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> { // Defines an async function that handles incoming HTTP requests.
    match (req.method(), req.uri().path()) { // Matches the HTTP method and request path.
        (&hyper::Method::GET, "/") => { // Cases where a GET request is made to the root endpoint.
            let nginx_default_page = r#"<!DOCTYPE html>
<html>
<head>
<title>Welcome to nginx!</title>
<style>
    body {
        width: 35em;
        margin: 0 auto;
        font-family: Tahoma, Verdana, Arial, sans-serif;
    }
</style>
</head>
<body>
<h1>Welcome to nginx!</h1>
<p>If you see this page, the nginx web server is successfully installed and
working. Further configuration is required.</p>

<p>For online documentation and support please refer to
<a href="http://nginx.org/">nginx.org</a>.<br/>
Commercial support is available at
<a href="http://nginx.com/">nginx.com</a>.</p>

<p><em>Thank you for using nginx.</em></p>
</body>
</html>"#; // Stores the HTML content of the default Nginx page.

            Ok(Response::new(Body::from(nginx_default_page))) // Returns a response containing the Nginx default page in the body.
        }
        _ => { // Defaults to handle cases when the requested resource is not available.
            let mut not_found = Response::default(); // Creates a new response struct with default values.
            *not_found.status_mut() = StatusCode::NOT_FOUND; // Sets the status code of the response to NOT FOUND.
            Ok(not_found) // Returns the response.
        }
    }
}

