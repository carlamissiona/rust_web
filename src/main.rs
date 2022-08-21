use warp::Filter;
// use warp::{http::Uri, Filter};

#[tokio::main]

async fn main() {
    pretty_env_logger::init();

    let hello_world = warp::path::end().map(|| "Hello, World at root!");
    
    let staticspa = warp::path("static").and(warp::fs::dir("src/html"));
    
    let routes = warp::get().and(
        hello_world
            .or(staticspa)
    );
 
 
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

}

