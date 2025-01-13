use clap::Parser;
use transmission::{
    transmission::Transmission,
    network::NetworkImplementation, 
    renderer::RendererImplementation, 
    transmission::TransmissionConfiguration
};

#[derive(Parser, Debug)]
struct SandboxArguments {
    #[arg(short, long, default_value="default")]
    renderer: String,
    #[arg(short, long, default_value="default")]
    network: String
}

fn main() {
    let arguments = SandboxArguments::parse();
    let config = TransmissionConfiguration {
        renderer_implementation: RendererImplementation::from(arguments.renderer),
        network_implementation: NetworkImplementation::from(arguments.network)
    };
    let transmission = Transmission::new(config);
    (transmission.renderer.initialize)();
    (transmission.renderer.start)();
}
