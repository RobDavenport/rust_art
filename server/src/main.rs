extern crate pretty_env_logger;
extern crate warp;

use warp::Filter;

fn main() {
    let files = warp::fs::dir("static");

    warp::serve(files).run(([127, 0, 0, 1], 8888));
}