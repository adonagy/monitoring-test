use std::convert::TryInto;
use std::thread::sleep;
use std::time::Duration;
use std::mem;
use std::env;
use std::io::Write;
use std::fs::File;

pub mod rpc;
pub mod configuration;

use curl::easy::Easy;

use configuration::MonitoringTestEnvironment;

#[tokio::main]
async fn main() {

    let env = MonitoringTestEnvironment::from_args();
    
    if env.cpu_load {
        let port = env::var("RPC_PORT").unwrap_or("18732".to_string()).parse::<u16>().unwrap();
        rpc::spawn_rpc_server(port);
        cpu_load()
    } else if let Some(mem_to_use) = env.memory_load {
        memory_load(mem_to_use)
    } else if let Some(network_and_io_load_to_use) = env.network_and_io_load {
        network_and_io_load(network_and_io_load_to_use);
    } else if let Some(cpu_target) = env.test_cpu {
        test_cpu(cpu_target).await;
    } else if let Some(memory_target) = env.test_memory {
        test_memory(memory_target).await;
    } else if let Some(io_network_target) = env.test_network_and_io {
        test_network_and_io(io_network_target).await;
    }
}

fn cpu_load() {
    loop {}
}

fn memory_load(mem_to_use: usize) {
    // From std::mem doc: In general, the size of a type is not stable across compilations, but specific types such as primitives are.
    // u8 should always be a size of 1 Byte

    println!("=== MEMORY SIMULATION STARTED ===");
    println!("\tALLOCATING {} BYTES", mem_to_use);

    let sizeof_bool = mem::size_of::<u8>();
    let count = mem_to_use / sizeof_bool;
    let mut artificial_memory_load: Vec<u8> = Vec::with_capacity(count);
    artificial_memory_load.resize(count, 0);

    println!("\tMEMORY LOADED, STARTING RPC...");
    let port = env::var("RPC_PORT").unwrap_or("18732".to_string()).parse::<u16>().unwrap();
    rpc::spawn_rpc_server(port);
    sleep(Duration::MAX);
}

fn network_and_io_load(network_and_io_load_to_use: u64) {
    println!("=== NETWORK AND IO SIMULATION STARTED ===");
    let mut file = File::create("downloaded.file").unwrap();

    let mut easy = Easy::new();
    easy.get(true).unwrap();
    // TODO: change this to something more appropriate
    easy.url("http://65.21.165.81:8080/rt-kernel/linux-image-5.10.41-rt42-dbg_5.10.41-rt42-1_amd64.deb").unwrap();
    easy.max_recv_speed(network_and_io_load_to_use).unwrap();
    easy.low_speed_limit(network_and_io_load_to_use.try_into().unwrap()).unwrap();
    println!("\tSETUP COMPLETED, STARTING DOWLOAD AND DISK WRITE");

    easy.write_function(move |data| {
        file.write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();

    let port = env::var("RPC_PORT").unwrap_or("18732".to_string()).parse::<u16>().unwrap();
        rpc::spawn_rpc_server(port);

    easy.perform().unwrap();
}

async fn test_cpu(target: f64) {
    println!("=== TESTING NODE CPU MEASUREMENT ===\n");

    let error_margin = 2.0;

    println!("\tTARGET: {}%", target);
    println!("\tERROR MARGIN: {}%\n", error_margin);

    let res = get_latest_measurement(tokio::time::Duration::from_secs(0)).await;

    if let Some(cpu_data) = res[0]["cpu"]["node"]["collective"].as_f64() {
        println!("\tCPU at: {}%\n", cpu_data);

        // Make sure the measurement is withing the defined interval (with the error margin)
        assert!(target + error_margin >= cpu_data);
        assert!(target - error_margin <= cpu_data);

        println!("=== OK ===");

    } else {
        panic!("Test failed: No cpu data found in measurements")
    }
}

async fn test_memory(target: u64) {
    println!("=== TESTING NODE MEMORY MEASUREMENT ===\n");

    // we need to take into consideration all the other memory allocation (stacks, warp server...)
    // 30 MB
    let error_margin = 31_457_280;

    println!("\tTARGET: {}MB", bytes_to_megabytes(target));
    println!("\tERROR MARGIN: {}MB\n", bytes_to_megabytes(error_margin));

    let res = get_latest_measurement(tokio::time::Duration::from_secs(0)).await;

    if let Some(memory_data) = res[0]["memory"]["node"].as_u64() {
        // DEBUG
        println!("\tMemory at: {}MB\n", bytes_to_megabytes(memory_data));
        assert!(target + error_margin >= memory_data);
        assert!(target - error_margin <= memory_data);

        println!("=== OK ===");

    } else {
        panic!("Test failed: No cpu data found in measurements")
    }
}

async fn test_network_and_io(target: u64) {
    println!("=== TESTING NODE IO ===\n");

    // there could be many bottlenecks to networking, give it a 20 KB/s error_margin
    // 30 KB/s
    let error_margin = 30_720;

    println!("\tTARGET: {}KB/s", bytes_to_kilobytes(target));
    println!("\tERROR MARGIN: {}KB/s\n", bytes_to_kilobytes(error_margin));

    let res = get_latest_measurement(tokio::time::Duration::from_secs(5)).await;

    if let Some(io_data) = res[0]["io"]["node"]["writtenBytesPerSec"].as_u64() {
        // DEBUG
        println!("\tDISK WRITE at: {}KB/s\n", bytes_to_kilobytes(io_data));
        assert!(target + error_margin >= io_data);
        assert!(target - error_margin <= io_data);

        println!("=== OK ===");

    } else {
        panic!("Test failed: No io data found in measurements")
    }

    println!("=== TESTING NODE NETWORKING ===\n");


    let error_margin = 61_440;
    println!("\tTARGET: {}KB/s", bytes_to_kilobytes(target));
    println!("\tERROR MARGIN: {}KB/s\n", bytes_to_kilobytes(error_margin));

    if let Some(network_data) = res[0]["network"]["receivedBytesPerSec"].as_u64() {
        // DEBUG
        println!("\tNETWORK RECIEVED at: {}KB/s\n", bytes_to_kilobytes(network_data));
        assert!(target + error_margin >= network_data);
        assert!(target - error_margin <= network_data);

        println!("=== OK ===");

    } else {
        panic!("Test failed: No network data found in measurements")
    }
}

async fn get_latest_measurement(delay: tokio::time::Duration) -> serde_json::Value {
    let retries = 100;

    // Network for example needs a few seconds to stabilize on the download speed, so get later measuremnt
    tokio::time::sleep(delay).await;

    for _ in 0..retries {
        match reqwest::get(
            "http://127.0.0.1:38732/resources/tezedge?limit=1"
        ).await
        {
            Ok(result) => return result.json().await.unwrap_or_default(),
            Err(_) => {
                println!("\tMonitoring not yet ready, retrying in 5s");
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await
            }
        };
    }
    serde_json::Value::default()
}

fn bytes_to_megabytes(bytes: u64) -> u64 {
    bytes / 1024 / 1024
}

fn bytes_to_kilobytes(bytes: u64) -> u64 {
    bytes / 1024
}