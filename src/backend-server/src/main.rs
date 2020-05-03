use std::net::SocketAddr;
use tonic::transport::Server;
use backend_engine::grpc_server::frj_server::FrjServer;
use backend_framework::wire_api::proto_frj_ngn::proto_fridge_game_engine_server::ProtoFridgeGameEngineServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli_args = cli::CliArgs::parse();

    println!("INFO: Starting local server and dependencies.");
    let frj_server = FrjServer::start()?;

    let socket_address: SocketAddr = format!("[::]:{}", cli_args.port)
        .parse()
        .expect("This should never happen. It's a valid IP address, dammit.");
    println!("INFO: Going to listen on '{:?}'", socket_address);

    Server::builder()
        .add_service(ProtoFridgeGameEngineServer::new(frj_server))
        .serve(socket_address)
        .await?;

    Ok(())
}

mod cli {
    use std::{env, process};

    const DEFAULT_PORT: u16 = 8051;

    pub struct CliArgs {
        pub port: u16,
    }

    impl CliArgs {
    pub fn parse() -> Self {
        let mut cli_args = env::args();

        // Arg 0
        let program_name = cli_args.next().unwrap_or_else(|| {
            eprintln!("Program name is somehow missing? You should never see this.");
            process::exit(1);
        });

        // Arg 1
        let port = cli_args.next()
            .map(|port_str| port_str.parse().unwrap_or_else(|_| {
                CliArgs::print_usage_exit(&program_name);
            }))
            .unwrap_or_else(|| {
                println!("Using default port '{}'", DEFAULT_PORT);
                DEFAULT_PORT
            });

        CliArgs {
            port,
        }
    }

    fn print_usage_exit(program_name: &str) -> ! {
        eprintln!();
        eprintln!("Usage:  \t{} <server port>", program_name);
        eprintln!("Example:\t{} 3000", program_name);
        eprintln!();
        process::exit(1);
    }
}
}
