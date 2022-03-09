use hyper::server::conn::AddrStream;
use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::service::{service_fn, make_service_fn};
use futures::future::{self, Future};
use serde_json::{Result, Value};
use std::net::SocketAddr;
use std::str::FromStr;
use std::fs;

/* use std::convert::Infallible;

type BoxFut = Box<dyn Future<Item=Response<Body>, Error=hyper::Error> + Send>;

fn debug_request(req: Request<Body>) -> BoxFut {
    let body_str = format!("{:?}", req);
    let response = Response::new(Body::from(body_str));
    Box::new(future::ok(response))
} */

fn read_config() -> Result<Value> {
    let config_path = format!(
        "{}/{}",
        project_root::get_project_root().unwrap().to_str().unwrap(),
        "config/config.json"
    );

    println!("[+] Reading Config File: {}", config_path);

    let json_str = std::fs::read_to_string(config_path).unwrap();

    // println!("[+] Found Config: {}", json_str);

    return serde_json::from_str(json_str.as_str());
}

fn match_routes(req: &Request<Body>, routes: &serde_json::Value) -> (String, i64) {
    let mut i: i64 = 0;

    let host_header = req.headers().get("host").unwrap().to_str().unwrap();

    println!("\nHOST_HEADER: {}", host_header);

    for route in routes.as_array().unwrap() {
        // println!("ROUTE: {:?}", route);

        let match_rule = route.get("match").unwrap().as_str().unwrap();
        let destination = route.get("forward_to").unwrap().as_str().unwrap();
        let type_rules = route.get("type").unwrap().as_object().unwrap();
        let what = type_rules.get("what").unwrap().as_str().unwrap();
        // let who = type_rules.get("who").unwrap().as_str().unwrap();
        println!("ROUTE: {} -> {}", match_rule, destination);
        println!("ROUTE DEBUG: WHAT -> {}", what);

        if what.eq("header") {
            println!("MATCH_TYPE_HEADER !");

            if host_header.starts_with(match_rule) {

                println!("ROUTE MATCHED !");
                println!(
                    "MATCH: {}\nFORWARD_TO: {}\nWHAT: {}\n\n", match_rule, destination, what
                );

                return (String::from(destination), i);
            }
        } else if what.eq("path") {
            println!("MATCH_TYPE_HEADER !");

            let path = req.uri().path();

            if path.starts_with(match_rule) {
                println!("ROUTE MATCHED !");
                println!(
                    "MATCH: {}\nFORWARD_TO: {}\nWHAT: {}\n\n", match_rule, destination, what
                );
            }

            return (String::from(destination), i);
        }

        i += 1;
    }

    (String::from(""), i)
}

fn main() {

    let config = read_config().unwrap();

    let bind_to_host: &str = config.get("bind_to_host").unwrap().as_str().unwrap();
    let bind_to_port: &str = config.get("bind_to_port").unwrap().as_str().unwrap();

    // println!("PARSED CONFIG: {:?}", config);
    // println!("bind_to_host: {:?}", bind_to_host);
    // println!("bind_to_port: {:?}", bind_to_port);

    
    let bind_to_addr: String = format!(
        "{}:{}",
        bind_to_host,
        bind_to_port
    );

    println!("bind_to_addr: {:?}", bind_to_addr);

    // This is our socket address...
    let addr: SocketAddr = SocketAddr::from_str(
        bind_to_addr.as_str()
    ).unwrap();

    

    // A `Service` is needed for every connection.
    let make_svc = make_service_fn(move |socket: &AddrStream| {
        let remote_addr = socket.remote_addr();

        let routes: serde_json::Value = config.get("routes").unwrap().clone();
        
        service_fn(move |req: Request<Body>| { // returns BoxFut

            let host_header = req.headers().get("host").unwrap().to_str().unwrap();
            let (forward_to, i) = match_routes(&req, &routes);
            

            println!("NEW REQUEST FROM: {:?}\n", req);
            println!("REQUESTING HOST: {}", host_header);
            println!("FORWARDING TO: {}\n", forward_to);

            if i > 0 {
                // will forward requests to port 13901
                return hyper_reverse_proxy::call(
                    remote_addr.ip(), 
                    forward_to.as_str(), 
                    req
                )
            } else {
                // debug_request(req)
                println!("NO MATCH !\nSENDING 404 NOT FOUND !");
                let body_str = fs::read_to_string(
                    format!(
                        "{}/{}",
                        project_root::get_project_root().unwrap().to_str().unwrap(),
                        "html/404.html"
                    )
                ).unwrap();

                let response = Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .header("Content-Type", "text/html")
                    .body(Body::from(body_str));

                Box::new(future::ok(
                    response.unwrap()
                ))
            }
        })
    });

    let server = Server::bind(&addr)
        .serve(make_svc)
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Running server on {:?}", addr);

    // Run this server for... forever!
    hyper::rt::run(server);
}