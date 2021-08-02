
use warp::Filter;

use warp::reject;
use warp::http::StatusCode;
use lazy_static::lazy_static;
use serde_json;

// const DUMMY_HEADER: &'static str = r#"{"hash":"BM2EMxsQsjCqud7g3VKE4qK8hXwpjnqjUWUHqNtDtwwVcD5cm1c","chain_id":"NetXxkAx4woPLyu","level":2,"proto":1,"predecessor":"BMAP2ZWJAhqCidHvTAxZ9d3vaL9winSaPQ7tnnbjEoAB59uzdAF","timestamp":"2021-03-04T20:02:09Z","validation_pass":4,"operations_hash":"LLoa7bxRTKaQN2bLYoitYB6bU2DvLnBAqrVjZcvJ364cTcX2PZYKU","fitness":["01","0000000000000001"],"context":"CoV3UQeXxNiWY4HT5KUoQbjYzEEGnKvKwg4X1rNtrwzZVSeZp3vU","protocol":"PsFLorenaUUuikDWvMDr6fGBRG8kt3e3D3fHoXK1j1BFRxeSH4i","signature":"sigPKUeqqrL3NBqz5AMwjRt6BESvkoPm8Tv84UKvY4NF2MThLrSzjBhUouPJreCrkcvVv6uXhZ1YKiZVW9jjBCKYaHxwVYhK","priority":0,"proof_of_work_nonce":"08351e3dc4590300"}"#;
// 

pub fn filters() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // Allow cors from any origin

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_methods(vec!["GET"]);

    let header_path = 
        warp::path!("chains" / "main" / "blocks" / "head" / "header")
            .and(warp::get())
            .and_then(reply_with_header);

    let metadata_path =
        warp::path!("chains" / "main" / "blocks" / "head" / "metadata")
            .and(warp::get())
            .and_then(reply_with_metadata);

    header_path.or(metadata_path).with(cors)
}

async fn reply_with_metadata() -> Result<impl warp::Reply, reject::Rejection> {
    lazy_static!{
        static ref DUMMY_METADATA: &'static str = r#"{"balance_updates":[{"change":"-512000000","contract":"tz1SwJwrKe8H1yi6KnYKCYkVHPApJRnZcHsa","kind":"contract","origin":"block"},{"category":"deposits","change":"512000000","cycle":0,"delegate":"tz1SwJwrKe8H1yi6KnYKCYkVHPApJRnZcHsa","kind":"freezer","origin":"block"}],"protocol":"PsFLorenaUUuikDWvMDr6fGBRG8kt3e3D3fHoXK1j1BFRxeSH4i","level_info":{"cycle":0,"cycle_position":1,"expected_commitment":false,"level":2,"level_position":1},"max_operation_list_length":[{"max_op":2048,"max_size":4194304},{"max_size":32768},{"max_op":132,"max_size":135168},{"max_size":524288}],"max_block_header_length":238,"nonce_hash":null,"level":{"cycle":0,"cycle_position":1,"expected_commitment":false,"level":2,"level_position":1,"voting_period":0,"voting_period_position":1},"test_chain_status":{"status":"not_running"},"consumed_gas":"0","next_protocol":"PsFLorenaUUuikDWvMDr6fGBRG8kt3e3D3fHoXK1j1BFRxeSH4i","voting_period_info":{"position":1,"remaining":1022,"voting_period":{"index":0,"kind":"proposal","start_position":0}},"max_operation_data_length":32768,"baker":"tz1SwJwrKe8H1yi6KnYKCYkVHPApJRnZcHsa","max_operations_ttl":2,"deactivated":[],"voting_period_kind":"proposal"}"#;
    }

    let json: serde_json::Value = serde_json::from_str(&DUMMY_METADATA).unwrap();
    Ok(warp::reply::with_status(warp::reply::json(&json), StatusCode::OK))
}

async fn reply_with_header() -> Result<impl warp::Reply, reject::Rejection> {
    lazy_static!{
        static ref DUMMY_HEADER: String = r#"{"hash":"BM2EMxsQsjCqud7g3VKE4qK8hXwpjnqjUWUHqNtDtwwVcD5cm1c","chain_id":"NetXxkAx4woPLyu","level":2,"proto":1,"predecessor":"BMAP2ZWJAhqCidHvTAxZ9d3vaL9winSaPQ7tnnbjEoAB59uzdAF","timestamp":"2021-03-04T20:02:09Z","validation_pass":4,"operations_hash":"LLoa7bxRTKaQN2bLYoitYB6bU2DvLnBAqrVjZcvJ364cTcX2PZYKU","fitness":["01","0000000000000001"],"context":"CoV3UQeXxNiWY4HT5KUoQbjYzEEGnKvKwg4X1rNtrwzZVSeZp3vU","protocol":"PsFLorenaUUuikDWvMDr6fGBRG8kt3e3D3fHoXK1j1BFRxeSH4i","signature":"sigPKUeqqrL3NBqz5AMwjRt6BESvkoPm8Tv84UKvY4NF2MThLrSzjBhUouPJreCrkcvVv6uXhZ1YKiZVW9jjBCKYaHxwVYhK","priority":0,"proof_of_work_nonce":"08351e3dc4590300"}"#.to_string();
    }

    let json: serde_json::Value = serde_json::from_str(&DUMMY_HEADER).unwrap();
    Ok(warp::reply::with_status(warp::reply::json(&json), StatusCode::OK))
}