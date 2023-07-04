// Importing required libraries for building a web server.
use hyper::http::StatusCode;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use rand::seq::SliceRandom;
use rand::Rng;
use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::runtime::Builder;
use tokio::sync::oneshot;

const DEFAULT_HTTP_PORT: u16 = 80;

// Function to generate a random alphanumeric string of a certain size
fn generate_random_alphanumeric(size: usize) -> String {
    let alphanumeric: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
        .chars()
        .collect();
    let mut rng = rand::thread_rng();
    (0..size)
        .map(|i| {
            let c = *alphanumeric.choose(&mut rng).unwrap();
            if i > 0 && i % 60 == 0 {
                format!("\n{}", c)
            } else {
                format!("{}", c)
            }
        })
        .collect()
}

// This async function runs the main logic of the program.
async fn main_async(port: u16, random_text: Arc<String>) {
    // Initializes an address to which the server will listen on.
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    // Defines a `make_service` function that accepts a Connection and returns a new service to handle requests from the connection.
    let make_svc = make_service_fn(move |_conn| {
        let random_text = Arc::clone(&random_text);
        async {
            Ok::<_, Infallible>(service_fn(move |req| {
                handle_request(req, Arc::clone(&random_text))
            }))
        }
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
    // Collects command line argument values into a vector of strings.
    let args: Vec<String> = env::args().collect();
    // Initializes port variable.
    let mut port = DEFAULT_HTTP_PORT;
    let mut random_text_size = 20; // 20KB

    match args.len() {
        3 => {
            match args[1].parse::<u16>() {
                Ok(p) => port = p,
                Err(_) => eprintln!("Error: Invalid port. Using default: {}", DEFAULT_HTTP_PORT),
            }
            match args[2].parse::<usize>() {
                Ok(size) => random_text_size = size,
                Err(_) => eprintln!("Error: Invalid random text size. Using default: 20KB"),
            }
        }
        1 | 2 => eprintln!("Error: Not enough arguments. Usage: random-page-server <port> <random_text_size_in_kb>"),
        _ => eprintln!("Error: Too many arguments. Usage: random-page-server <port> <random_text_size_in_kb>"),
    }
    
    // Builds a new Tokio runtime instance for executing async functions.
    let runtime = Builder::new_multi_thread()
        .worker_threads(num_cpus::get()) // Sets the number of worker threads to the number of CPUs available.
        .enable_all()
        .build()
        .unwrap();

    // Generate 20KB of random alphanumeric text
    let random_text = generate_random_alphanumeric(random_text_size * 1024);
    let random_text = Arc::new(random_text);

    // Executes the `main_async()` function using the previously defined runtime instance.
    runtime.block_on(main_async(port, random_text));
}

// Defines an async function that handles incoming HTTP requests.
async fn handle_request(
    req: Request<Body>,
    random_text: Arc<String>,
) -> Result<Response<Body>, Infallible> {
    // Matches the HTTP method and request path.
    match (req.method(), req.uri().path()) {
        // Cases where a GET request is made to the root endpoint.
        (&hyper::Method::GET, "/") => {
            // Stores the HTML content of the default Nginx page.
            let mut random_page = r#"<!DOCTYPE html>
<html>
<head>
<title>Random Page!</title>
<style>
    body {
        width: 35em;
        margin: 0 auto;
        font-family: Tahoma, Verdana, Arial, sans-serif;
    }
</style>
</head>
<body>
<h1>Random Page!</h1>
<p>This is a random page generated by Random Page Server.</p>
</body>
</html>"#
                .to_string();

            // Generate a random length of random text to be inserted
            let mut rng = rand::thread_rng();
            let rand_text_len = rng.gen_range(0..=random_text.len());
            let random_insert = format!("<p>Page size: {} Bytes</p><p>{}</p>", rand_text_len, &random_text[..rand_text_len]);

            // Find the end of the body tag where the random text will be inserted
            let pos = random_page
                .find("</body>")
                .unwrap_or(random_page.len());

            // Insert the random text
            random_page.insert_str(pos, &random_insert);

            // Returns a response containing the modified Nginx default page in the body.
            Ok(Response::new(Body::from(random_page)))
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
