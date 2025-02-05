use clap::Parser;
use transmission::{
    transmission::{
        TransmissionConfiguration,
        Transmission
    },
    network::NetworkImplementation, 
    renderer::RendererImplementation
};

#[derive(Parser, Debug)]
struct SandboxArguments {
    #[arg(short, long, default_value="default")]
    renderer: String,
    #[arg(short, long, default_value="default")]
    network: String,
    #[arg(short, long, default_value=None)]
    join: Option<String>
}

#[tokio::main]
async fn main() {
    let arguments = SandboxArguments::parse();
    let config = TransmissionConfiguration {
        renderer_implementation: RendererImplementation::from(arguments.renderer),
        network_implementation: NetworkImplementation::from(arguments.network)
    };
    match Transmission::new(config).await {
        Ok(transmission) => {
            println!("Transmission started.");
            match arguments.join {
                Some(invite) => {
                    match transmission.network.accept_invite(invite).await {
                        Ok(_) => println!("Successfully accepted invite!"),
                        Err(error) => eprintln!("{}", error)
                    }
                }
                None => {
                    match transmission.network.generate_invite().await {
                        Ok(invite) => println!("Document invite:\n{}", invite),
                        Err(error) => eprintln!("{}", error)
                    }
                }
            }
        }
        Err(error) => {
            println!("{}", error.description)
        }
    }
}
