use std::env;
use prctl;

pub mod rpc;
pub mod configuration;
pub mod tests;
pub mod loads;

use configuration::MonitoringTestEnvironment;
use crate::loads::*;
use crate::tests::*;

#[tokio::main]
async fn main() {

    let env = MonitoringTestEnvironment::from_args();

    if let Some(process_name) = env.process_name {
        prctl::set_name(&process_name).expect(&format!("Cannot change proces name to {}", process_name));
    }
    
    if env.cpu_load {
        // cpu_load()
        cpu_load_on_threads();
        if !env.disable_rpc_server {
            let port = env::var("RPC_PORT").unwrap_or("18732".to_string()).parse::<u16>().unwrap();
            rpc::spawn_rpc_server(port);
        }
        // cpu_load(env.disable_rpc_server);
        tokio::time::sleep(tokio::time::Duration::MAX).await;
    } else if let Some(mem_to_use) = env.memory_load {
        memory_load(mem_to_use, env.disable_rpc_server)
    } else if let Some(network_and_io_load_to_use) = env.network_and_io_load {
        network_and_io_load(network_and_io_load_to_use, env.disable_rpc_server);
    } else if let Some(cpu_target) = env.test_cpu {
        test_cpu(cpu_target).await;
    } else if let Some(memory_target) = env.test_memory {
        test_memory(memory_target).await;
    } else if let Some(io_network_target) = env.test_network_and_io {
        test_network_and_io(io_network_target).await;
    } else if env.cpu_load_with_subprocess {
        cpu_load_sub_process();
        // cpu_load_on_threads();
        if !env.disable_rpc_server {
            let port = env::var("RPC_PORT").unwrap_or("18732".to_string()).parse::<u16>().unwrap();
            rpc::spawn_rpc_server(port);
        }
        tokio::time::sleep(tokio::time::Duration::MAX).await;
    }
}