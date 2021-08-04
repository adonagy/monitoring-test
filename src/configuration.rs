use clap::{App, Arg};

#[derive(Clone, Debug)]
pub struct MonitoringTestEnvironment {
    pub cpu_load: bool,

    pub memory_load: Option<usize>,

    pub network_and_io_load: Option<u64>,

    pub test_cpu: Option<f64>,

    pub test_memory: Option<u64>,

    pub test_network_and_io: Option<u64>,

    pub disable_rpc_server: bool,

    pub cpu_load_with_subprocess: bool,

    pub process_name: Option<String>,
}

impl MonitoringTestEnvironment {
    pub fn from_args() -> Self {
        let app = monitoring_test_app();
        let args = app.clone().get_matches();

        Self {
            cpu_load: args.is_present("cpu-load"),
            disable_rpc_server: args.is_present("disable-rpc-server"),
            cpu_load_with_subprocess: args.is_present("cpu-load-with-subprocess"),
            process_name: args
                .value_of("process-name")
                .map(|process_name| {
                    process_name.to_string()
                }),
            memory_load: args
                .value_of("memory-load")
                .map(|memory_load| {
                    memory_load
                        .parse::<usize>()
                        .expect("Was expecting NUM [u64]")
                }),
            network_and_io_load: args
                .value_of("network-and-io-load")
                .map(|network_and_io_load| {
                    network_and_io_load
                        .parse::<u64>()
                        .expect("Was expecting NUM [u64]")
                }),
            test_cpu: args
                .value_of("test-cpu")
                .map(|target| {
                    target
                        .parse::<f64>()
                        .expect("Was expecting FLOAT [f64]")
                }),
            test_memory: args
                .value_of("test-memory")
                .map(|target| {
                    target
                        .parse::<u64>()
                        .expect("Was expecting NUM [u64]")
                }),
            test_network_and_io: args
            .value_of("test-networking-and-io")
            .map(|target| {
                target
                    .parse::<u64>()
                    .expect("Was expecting NUM [u64]")
            })
        }
    }
}

fn monitoring_test_app() -> App<'static, 'static> {

    let app = App::new("Tezedge node monitoring app")
        .version("1.7.0")
        .author("SimpleStaking and the project contributors")
        .setting(clap::AppSettings::AllArgsOverrideSelf)
        .arg(
            Arg::with_name("disable-rpc-server")
                .long("disable-rpc-server")
                .help("Launches the app with RPC server")
        )
        .arg(
            Arg::with_name("process-name")
                .long("process-name")
                .takes_value(true)
                .value_name("STRING")
                .help("Sets the process' name")
        )
        .arg(
            Arg::with_name("cpu-load")
                .long("cpu-load")
                .help("Launches the app with 100% cpu load")
        )
        .arg(
            Arg::with_name("cpu-load-with-subprocess")
                .long("cpu-load-with-subprocess")
                .help("Launches the app with 100% cpu load and a subprocess with also 100% cpu load")
        )
        .arg(
            Arg::with_name("memory-load")
                .long("memory-load")
                .takes_value(true)
                .value_name("NUM")
                .help("Launches the app with the hardcoded load")
        )
        .arg(
            Arg::with_name("network-and-io-load")
                .long("network-and-io-load")
                .takes_value(true)
                .value_name("NUM")
                .help("Launches the app with the newtwork and io load")
        )
        .arg(
            Arg::with_name("test-cpu")
                .long("test-cpu")
                .takes_value(true)
                .value_name("FLOAT")
                .help("Launches test to assert cpu measurent")
        )
        .arg(
            Arg::with_name("test-memory")
                .long("test-memory")
                .takes_value(true)
                .value_name("NUM")
                .help("Launches test to assert memory measurent")
        )
        .arg(
            Arg::with_name("test-networking-and-io")
                .long("test-networking-and-io")
                .takes_value(true)
                .value_name("NUM")
                .help("Launches test to assert networking and io measurent")
        );
    app
}