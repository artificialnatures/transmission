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
    network: String
}

#[tokio::main]
async fn main() {
    let arguments = SandboxArguments::parse();
    let config = TransmissionConfiguration {
        renderer_implementation: RendererImplementation::from(arguments.renderer),
        network_implementation: NetworkImplementation::from(arguments.network)
    };
    match Transmission::new(config).await {
        Ok(_) => {
            println!("Yay!")
        }
        Err(error) => {
            println!("{}", error.description)
        }
    }
}
