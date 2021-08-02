
use tokio::task::JoinHandle;

pub mod filters;
pub mod handlers;

pub const MEASUREMENTS_MAX_CAPACITY: usize = 40320;

pub fn spawn_rpc_server(
    rpc_port: u16,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        // let api = filters::filters(log.clone(), resource_utilization.clone());
        let api = filters::filters();

        warp::serve(api).run(([0, 0, 0, 0], rpc_port)).await;
    })
}