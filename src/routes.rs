use warp::Filter;
use super::handlers;

// A function to build our routes
pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_order()
}

// A route to handle GET requests for a specific post
fn get_order() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("posts" / usize)
        .and(warp::get())
        .and_then(handlers::get_order)
}
// fn add_order() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
//     warp::path!("posts")
//         .and(warp::post())
//         .and(warp::body::json())
//         .and_then(handlers::add_order)
// }