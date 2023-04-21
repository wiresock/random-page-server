// Importing required libraries for building a web server.
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use hyper::http::{StatusCode};
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::sync::oneshot;
use tokio::runtime::Builder;

// This async function runs the main logic of the program.
async fn main_async() {
    // Initializes an address to which the server will listen on.
    let addr = SocketAddr::from(([0, 0, 0, 0], 80));

    // Defines a `make_service` function that accepts a Connection and returns a new service to handle requests from the connection.
    let make_svc = make_service_fn(|_conn| {
        async { Ok::<_, Infallible>(service_fn(handle_request)) }
    });

    // initializes a server instance that listens on the defined address, and passes the `make_service` function as input.
    let server = Server::bind(&addr).serve(make_svc);

    // Prints a message to indicate the server has started and is listening on a specific port number.
    println!("Listening on http://{}", addr);
    // Prompts the user to press ENTER to exit.
    println!("Press ENTER to exit");

    // Initializes a channel for communication between this function and a spawned thread.
    let (tx, rx) = oneshot::channel::<()>();
    // Spawns a new thread using Tokio's runtime, that waits for the user to press ENTER.
    tokio::spawn(async move {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let _ = tx.send(());
    });

    // Waits for either the server to return a result or the thread with `rx` to receive a message
    let result = tokio::select! {
        result = server => result, // Server returns Ok() if it completes successfully.
        _ = rx => {
            println!("Shutting down...");
            Ok(()) // Thread returns Ok() if it receives the message successfully.
        }
    };

    // Prints any errors that may have occurred during the program's execution.
    if let Err(e) = result {
        eprintln!("Server error: {}", e);
    }
}

// The main function of the program.
fn main() {
    // Builds a new Tokio runtime instance for executing async functions.
    let runtime = Builder::new_multi_thread()
        .worker_threads(num_cpus::get()) // Sets the number of worker threads to the number of CPUs available.
        .enable_all()
        .build()
        .unwrap();
    
    // Executes the `main_async()` function using the previously defined runtime instance.
    runtime.block_on(main_async());
}

// Defines an async function that handles incoming HTTP requests.
async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    // Matches the HTTP method and request path.
    match (req.method(), req.uri().path()) {
        // Cases where a GET request is made to the root endpoint.
        (&hyper::Method::GET, "/") => {
            // Stores the HTML content of the default Nginx page.
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
</html>"#;

            // Returns a response containing the Nginx default page in the body.
            Ok(Response::new(Body::from(nginx_default_page)))
        }
        // Defaults to handle cases when the requested resource is not available.
        _ => {
            // Creates a new response struct with default values.
            let mut not_found = Response::default();
            // Sets the status code of the response to NOT FOUND.
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            // Returns the response.
            Ok(not_found)
        }
    }
}




