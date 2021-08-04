
use std::convert::TryInto;
use std::thread::sleep;
use std::time::Duration;
use std::mem;
use std::io::Write;
use std::fs::File;
use std::env;
use std::process::Command;

use curl::easy::Easy;

use crate::rpc;

pub fn cpu_load(disable_rpc_server: bool) {
    println!("=== CPU SIMULATION STARTED ===\n");
    println!("\tUSING INFINITE LOOP TO GENERATE 100% load on one CPU");
    if !disable_rpc_server {
        let port = env::var("RPC_PORT").unwrap_or("18732".to_string()).parse::<u16>().unwrap();
        rpc::spawn_rpc_server(port);
    }
    loop {
        let _: u128 = 100000 * 255745; 
    }
}

pub fn memory_load(mem_to_use: usize, disable_rpc_server: bool) {
    // From std::mem doc: In general, the size of a type is not stable across compilations, but specific types such as primitives are.
    // u8 should always be a size of 1 Byte

    println!("=== MEMORY SIMULATION STARTED ===\n");
    println!("\tALLOCATING {} BYTES", mem_to_use);

    let sizeof_bool = mem::size_of::<u8>();
    let count = mem_to_use / sizeof_bool;
    let mut artificial_memory_load: Vec<u8> = Vec::with_capacity(count);
    artificial_memory_load.resize(count, 0);

    println!("\tMEMORY LOADED, STARTING RPC...");
    if !disable_rpc_server {
        let port = env::var("RPC_PORT").unwrap_or("18732".to_string()).parse::<u16>().unwrap();
        rpc::spawn_rpc_server(port);
    }
    sleep(Duration::MAX);
}

pub fn network_and_io_load(network_and_io_load_to_use: u64, disable_rpc_server: bool) {
    println!("=== NETWORK AND IO SIMULATION STARTED ===\n");
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

    if !disable_rpc_server {
        let port = env::var("RPC_PORT").unwrap_or("18732".to_string()).parse::<u16>().unwrap();
        rpc::spawn_rpc_server(port);
    }

    easy.perform().unwrap();
}

pub fn cpu_load_on_threads() {
    std::thread::Builder::new().name("test_thread".to_string()).spawn( move || {
        cpu_load(true)
    }).unwrap();
}

pub fn cpu_load_sub_process() {
    Command::new("target/debug/monitoring-test").args(&["--cpu-load", "--disable-rpc-server", "--process-name", "protocol-runner"]).spawn().expect("Cannot run subprocess");
}