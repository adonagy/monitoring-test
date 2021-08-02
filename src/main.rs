use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};
use std::thread::sleep;
use std::time::Duration;
use std::mem;
use std::env;

pub mod rpc;

use curl::easy::{Easy2, Handler, WriteError};

struct Collector(Vec<u8>);

#[tokio::main]
async fn main() {

    let port = env::var("RPC_PORT").unwrap_or("18732".to_string()).parse::<u16>().unwrap();

    rpc::spawn_rpc_server(port);
    network_load();
    sleep(Duration::MAX)
}

fn cpu_load() {
    loop {}
}

fn memory_load() {
    // 1 GB
    const ALLOCATE: usize = 1_073_741_824;

    // 512 MB
    // const ALLOCATE: usize = 1_073_741_824 / 2;

    // 2 GB
    // const ALLOCATE: usize = 1_073_741_824 * 2;

    // From std::mem doc: In general, the size of a type is not stable across compilations, but specific types such as primitives are
    // bool should always be a size of 1 Byte
    let sizeof_bool = mem::size_of::<bool>();
    let count = ALLOCATE / sizeof_bool;
    let mut artificial_memory_load: Vec<bool> = Vec::with_capacity(count);

    for _ in 0..count {
        artificial_memory_load.push(true)
    }
    sleep(Duration::MAX)
}

fn network_load() {
    impl Handler for Collector {
        fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
            self.0.extend_from_slice(data);
            Ok(data.len())
        }
    }

    let mut easy = Easy2::new(Collector(Vec::new()));
    easy.get(true).unwrap();
    // TODO: change this to something more appropriate
    easy.url("http://65.21.165.81:8080/rt-kernel/linux-image-5.10.41-rt42-dbg_5.10.41-rt42-1_amd64.deb").unwrap();
    easy.max_recv_speed(102400).unwrap();
    easy.low_speed_limit(102400).unwrap();
    println!("Download starting");
    easy.perform().unwrap();
}

fn io_load() {
    
}