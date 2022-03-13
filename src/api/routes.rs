use warp::{filters::BoxedFilter, Filter, Reply};

// Return warp filter to process requests and map to handlers
pub fn get_routes() -> BoxedFilter<(impl Reply,)> {
    let routes = warp::path("hello")
        .and(warp::get())
        .map(|| "Hello from Mini API");
    routes.boxed()
}
