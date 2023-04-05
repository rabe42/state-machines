mod error;
mod ids;
mod state_charts;
mod state_machine;
mod state_machine_log;
mod warehouse;

use env_logger;
use hyper::Method;
use log::{debug, error, info};
use open_api_matcher::{OpenApiResponse, Value, RequestParamters, OpenApiOperation};
use std::fs::File;
use std::net::SocketAddr;

use crate::state_charts::Node;
use crate::error::StateChartError;

#[tokio::main]
pub async fn main() {
    env_logger::init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let file = File::open("StateMachine.yml").unwrap();
    info!("Starting server on {:?}!", addr);
    open_api_matcher::service::start(addr, &file, Box::new(handle)).await;
}

/// The central function, where all request must be handled.
async fn handle(request: open_api_matcher::service::RequestMatch) -> OpenApiResponse {
    match request.into_match() {
        (&Method::GET, "/state-chart/", _p, op) => {
            let mut response = OpenApiResponse::new(op);
            response.content(Vec::new().into());
            response
        },
        (&Method::POST, "/state-chart/", p, op) => {
            create_state_chart(p, op).await
        },
        (&Method::GET, "/state-chart/{id}", _p, op) => {
            let response = OpenApiResponse::new(op);
            response
        },
        (&Method::POST, "/action/", _p, op) => {
            let response = OpenApiResponse::new(op);
            response
        },
        (&Method::POST, "/start/{state-chart-id}", p, op) => {
            start_state_machine(p, op).await
        },
        (&Method::POST, "/send/{state-machine-id}/{event-id}", _p, op) => {
            let response = OpenApiResponse::new(op);
            response
        },
        (&Method::POST, "/set-var/{state-machine-id}/{variable-id}", _p, op) => {
            let response = OpenApiResponse::new(op);
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
            OpenApiResponse::fall_through()
        },
    }
}

/// Creates and saves a new state chart, from the already validated parameters. The only validation
/// which needs to happen here, is the validation, based on semantic level. This might be a wrong
/// defintion of the state chart, where the start state might be missing.
async fn create_state_chart(p: &RequestParamters, op: &OpenApiOperation) -> OpenApiResponse {
    debug!("[main::create_state_chart()]");
    let mut response = OpenApiResponse::new(op);
    let node_result: Result<Node, StateChartError> = p.get_content().try_into();
    match node_result {
        Ok(state_chart) => {
            debug!("Received state chart successfully:\n{:?}", state_chart);
            response.content(state_chart.id().into());
        },
        Err(err) => {
            error!("[main::create_state_chart()]: {}", err);
            response.set_mime_type("application/json".into());
            response.content(err.into());
        }
    }
    response
}

async fn start_state_machine(p: &RequestParamters, op: &OpenApiOperation) -> OpenApiResponse {
    let response = OpenApiResponse::new(op);
    response
}
