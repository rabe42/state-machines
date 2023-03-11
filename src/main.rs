use env_logger;
use log::{debug, info, error};
use open_api_matcher::{OpenApiResponse, Value, ValidatedValue};
use std::net::SocketAddr;
use std::fs::File;
use hyper::Method;

#[tokio::main]
pub async fn main() {
    env_logger::init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let file = File::open("StateMachine.yml").unwrap();
    info!("Starting server on {:?}!", addr);
    open_api_matcher::service::start(addr, &file, Box::new(handle)).await;
}


async fn handle(request: open_api_matcher::service::RequestMatch) -> OpenApiResponse
{
    match request.into_match() {
        (&Method::GET, "/state-chart/", _p, op) => {
            let mut response = OpenApiResponse::new(op);
            response.content(Vec::new().into());
            response
        },
        (&Method::POST, "/state-chart/", p, op) => {
            if let ValidatedValue::Object(sc) = p.get_content() {
                debug!("");
            };
            let mut response = OpenApiResponse::new(op);
            response
        },
        (&Method::GET, "/state-chart/{id}", p, op) => {
            let mut response = OpenApiResponse::new(op);
            response
        },
        (&Method::POST, "/action/", p, op) => {
            let mut response = OpenApiResponse::new(op);
            response
        },
        (&Method::POST, "/start/{state-chart-id}", p, op) => {
            let mut response = OpenApiResponse::new(op);
            response
        },
        (&Method::POST, "/send/{state-machine-id}/{event-id}", p, op) => {
            let mut response = OpenApiResponse::new(op);
            response
        },
        (&Method::POST, "/set-var/{state-machine-id}/{variable-id}", p, op) => {
            let mut response = OpenApiResponse::new(op);
            response
        },
        (&Method::GET, "/hello/{name}", p, op) => {
            debug!("Matched '/hello/{{name}}'");
            let answer = format!("Hello {}!", p.get_path_parameter("name"));
            let mut response = OpenApiResponse::new(op);
            response.content(Value::String(answer));
            response
        },
        _ => {
            error!("Unexpected match!");
            OpenApiResponse::error("Unexpected matched request!".into())
        }
    }
}
