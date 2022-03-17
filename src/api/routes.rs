use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use warp::{filters::BoxedFilter, Filter, Reply};

// Struct to deserialise json into
#[derive(Clone, Debug, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct Hello {
    name: String,
}

impl Hello {
    // Generate a minimal response (HashMap can serialise into json)
    pub fn reply(&self) -> HashMap<String, String> {
        HashMap::from([("message".to_string(), format!("Hello {}!", self.name))])
    }
}

// Return warp filter to process requests and map to handlers
pub fn get_routes() -> BoxedFilter<(impl Reply,)> {
    // curl "http://127.0.0.1:8080/hello"
    //  Hello from Mini API
    let hello_get = warp::path("hello")
        .and(warp::get())
        .map(|| "Hello from Mini API");

    // curl "http://127.0.0.1:8080/hello" -H 'Content-Type: application/json' -X POST -d '{"name":"Bob"}'
    //  {"message":"Hello Bob!"}
    let hello_post = warp::path("hello")
        .and(warp::post())
        .and(warp::body::json::<Hello>())
        .map(|body: Hello| warp::reply::json(&body.reply()));

    // Return routes to server
    let routes = hello_get.or(hello_post);
    routes.boxed()
}
