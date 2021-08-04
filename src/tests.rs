use tokio::time::{sleep, Duration};

pub async fn test_cpu(target: f64) {
    println!("=== TESTING NODE CPU MEASUREMENTS ===\n\n");

    let error_margin = 2.0;

    println!("\tTARGET: {}%", target);
    println!("\tERROR MARGIN: {}%\n", error_margin);

    let res = get_latest_measurement(Duration::from_secs(0)).await;

    if let Some(cpu_data) = res[0]["cpu"]["node"]["collective"].as_f64() {
        println!("\tCOLLECTIVE CPU at: {}%\n", cpu_data);
        // Make sure the measurement is withing the defined interval (with the error margin)
        assert!(target + error_margin >= cpu_data);
        assert!(target - error_margin <= cpu_data);

        println!("=== OK ===\n");
    } else {
        panic!("Test failed: No cpu data found in measurements")
    }

    if let Some(tasks) = res[0]["cpu"]["node"]["taskThreads"].as_object() {
        if let Some(key) = tasks
            .keys()
            .filter(|key| key.contains("test_thread"))
            .next()
        {
            if let Some(thread_cpu) = tasks.get(key).unwrap().as_f64() {
                println!("\tTHREAD CPU at: {}%\n", thread_cpu);
                // Make sure the measurement is withing the defined interval (with the error margin)
                assert!(target + error_margin >= thread_cpu);
                assert!(target - error_margin <= thread_cpu);

                println!("=== OK ===\n");
            }
        } else {
            panic!("No thread named test_thread found")
        }
    } else {
        panic!("Test failed: No thread data found in cpu measurements")
    }
}

pub async fn test_memory(target: u64) {
    println!("=== TESTING NODE MEMORY MEASUREMENT ===\n\n");

    // we need to take into consideration all the other memory allocation (stacks, warp server...)
    // 30 MB
    let error_margin = 31_457_280;

    println!("\tTARGET: {}MB", bytes_to_megabytes(target));
    println!("\tERROR MARGIN: {}MB\n", bytes_to_megabytes(error_margin));

    let res = get_latest_measurement(Duration::from_secs(0)).await;

    if let Some(memory_data) = res[0]["memory"]["node"].as_u64() {
        println!("\tMemory at: {}MB\n", bytes_to_megabytes(memory_data));
        assert!(target + error_margin >= memory_data);
        assert!(target - error_margin <= memory_data);

        println!("=== OK ===\n");
    } else {
        panic!("Test failed: No cpu data found in measurements")
    }
}

pub async fn test_network_and_io(target: u64) {
    println!("=== TESTING NODE IO ===\n\n");

    // there could be many bottlenecks to networking, give it a 20 KB/s error_margin
    // 30 KB/s
    let error_margin = 30_720;

    println!("\tTARGET: {}KB/s", bytes_to_kilobytes(target));
    println!("\tERROR MARGIN: {}KB/s\n", bytes_to_kilobytes(error_margin));

    let res = get_latest_measurement(Duration::from_secs(5)).await;

    if let Some(io_data) = res[0]["io"]["node"]["writtenBytesPerSec"].as_u64() {
        println!("\tDISK WRITE at: {}KB/s\n", bytes_to_kilobytes(io_data));
        assert!(target + error_margin >= io_data);
        assert!(target - error_margin <= io_data);

        println!("=== OK ===\n");
    } else {
        panic!("Test failed: No io data found in measurements")
    }

    println!("=== TESTING NODE NETWORKING ===\n\n");

    let error_margin = 61_440;
    println!("\tTARGET: {}KB/s", bytes_to_kilobytes(target));
    println!("\tERROR MARGIN: {}KB/s\n", bytes_to_kilobytes(error_margin));

    if let Some(network_data) = res[0]["network"]["receivedBytesPerSec"].as_u64() {
        println!(
            "\tNETWORK RECIEVED at: {}KB/s\n",
            bytes_to_kilobytes(network_data)
        );
        assert!(target + error_margin >= network_data);
        assert!(target - error_margin <= network_data);

        println!("=== OK ===\n");
    } else {
        panic!("Test failed: No network data found in measurements")
    }
}

pub async fn test_disk_size(target: u64) {
    println!("=== TESTING DISK SIZE ===\n\n");

    // 30 KB
    let error_margin = 30_720;

    println!("\tTARGET: {}MB", bytes_to_megabytes(target));
    println!("\tERROR MARGIN: {}MB\n", bytes_to_megabytes(error_margin));

    let res = get_latest_measurement(Duration::from_secs(0)).await;

    if let Some(diks_data) = res[0]["disk"]["contextActions"].as_u64() {
        println!(
            "\tcontextActions SIZE at: {}MB\n",
            bytes_to_megabytes(diks_data)
        );
        assert!(target + error_margin >= diks_data);
        assert!(target - error_margin <= diks_data);

        println!("=== OK ===\n");
    } else {
        panic!("Test failed: contextActions data not found in measurements")
    }

    if let Some(diks_data) = res[0]["disk"]["contextIrmin"].as_u64() {
        println!(
            "\tcontextIrmin SIZE at: {}MB\n",
            bytes_to_megabytes(diks_data)
        );
        assert!(target + error_margin >= diks_data);
        assert!(target - error_margin <= diks_data);

        println!("=== OK ===\n");
    } else {
        panic!("Test failed: contextIrmin data not found in measurements")
    }

    if let Some(diks_data) = res[0]["disk"]["blockStorage"].as_u64() {
        println!(
            "\tblockStorage SIZE at: {}MB\n",
            bytes_to_megabytes(diks_data)
        );
        assert!(target + error_margin >= diks_data);
        assert!(target - error_margin <= diks_data);

        println!("=== OK ===\n");
    } else {
        panic!("Test failed: blockStorage data not found in measurements")
    }

    if let Some(diks_data) = res[0]["disk"]["mainDb"].as_u64() {
        println!("\tmainDb SIZE at: {}MB\n", bytes_to_megabytes(diks_data));
        assert!(target + error_margin >= diks_data);
        assert!(target - error_margin <= diks_data);

        println!("=== OK ===\n");
    } else {
        panic!("Test failed: mainDb data not found in measurements")
    }
}

pub async fn get_latest_measurement(delay: Duration) -> serde_json::Value {
    let retries = 100;

    // Network for example needs a few seconds to stabilize on the download speed, so get later measuremnt
    sleep(delay).await;

    for _ in 0..retries {
        match reqwest::get("http://127.0.0.1:38732/resources/tezedge?limit=1").await {
            Ok(result) => return result.json().await.unwrap_or_default(),
            Err(_) => {
                println!("\tMonitoring not yet ready, retrying in 5s");
                sleep(Duration::from_secs(5)).await
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
