use warp::Filter;
// use warp::{http::Uri, Filter};

mod libpages;

#[tokio::main]

async fn main() {
    pretty_env_logger::init();
    // basic hello
    let hello_world = warp::path::end().map(|| "Hello, World at root!");
    // serve static files
    let staticspa = warp::path("static").and(warp::fs::dir("src/html"));
    // url param
    let urlpractice = warp::path("url").map(|| libpages::check_url(1) );
    // json serde
    let jsonbody = warp::path("json").map(|| libpages::json_serve() );
    
    let routes = warp::get().and(
        hello_world
            .or(staticspa)
            .or(urlpractice)
            .or(jsonbody)
    );
 
 
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

}
 