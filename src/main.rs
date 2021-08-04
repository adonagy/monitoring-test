use prctl;
use std::env;
use std::path::PathBuf;

pub mod configuration;
pub mod loads;
pub mod rpc;
pub mod tests;

use crate::loads::*;
use crate::tests::*;
use configuration::MonitoringTestEnvironment;

#[tokio::main]
async fn main() {
    let env = MonitoringTestEnvironment::from_args();

    let volume_path = env::var("VOLUME_PATH")
        .unwrap_or("/tmp/tezedge".to_string())
        .parse::<PathBuf>()
        .expect("Expected PATH");

    if let Some(process_name) = env.process_name {
        prctl::set_name(&process_name)
            .expect(&format!("Cannot change proces name to {}", process_name));
    }

    if env.cpu_load {
        // launch a thread with 100% load (this also loads the main process to 100%)
        cpu_load_on_threads();
        if !env.disable_rpc_server {
            // the rpc server is disabled only when a subprocess is run, so run the subprocess here
            // this allows us to test every aspect of the cpu measurements (collective, thread, subprocess)
            cpu_load_sub_process();
            let port = env::var("RPC_PORT")
                .unwrap_or("18732".to_string())
                .parse::<u16>()
                .expect("Expected u16");
            rpc::spawn_rpc_server(port);
        }
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
            let port = env::var("RPC_PORT")
                .unwrap_or("18732".to_string())
                .parse::<u16>()
                .expect("Expected u16");
            rpc::spawn_rpc_server(port);
        }
        tokio::time::sleep(tokio::time::Duration::MAX).await;
    } else if let Some(disk_target) = env.disk_load {
        disk_load(disk_target, volume_path);
    } else if let Some(target) = env.test_disk {
        test_disk_size(target).await;
    }
}
