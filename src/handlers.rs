use warp::Filter;
use crate::models::NftOrder;


const NFTS: Vec<&NftOrder> = Vec::<&NftOrder>::new();

// A function to handle GET requests at /posts/{id}
pub async fn get_order(id: usize) -> Result<impl warp::Reply, warp::Rejection> {
    // For simplicity, let's say we are returning a static post
    let order = &NFTS[id];
    Ok(warp::reply::json(&order))
}

pub async fn add_order( nft_name: &str, body: &str) -> Result<impl warp::Reply, warp::Rejection> {
    // For simplicity, let's say we are returning a static post
    let order = NftOrder {
        nft_name: String::from(nft_name),
        body: String::from(body),
    };
    NFTS.push(&order);
    Ok(warp::reply::json(&order))
}

