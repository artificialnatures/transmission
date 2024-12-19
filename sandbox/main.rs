use clap::{Parser, ValueEnum};
use transmission_ui_bevy::bevy_renderer;
use transmission_ui_cli::cli_renderer;

#[derive(ValueEnum, Debug, Clone)]
enum RendererImplementation {
    Default,
    Cli,
    Bevy
}

#[derive(Parser, Debug)]
struct SandboxArguments {
    #[arg(short, long)]
    renderer: Option<RendererImplementation>
}

fn main() {
    let arguments = SandboxArguments::parse();
    let renderer_implementation =
        match arguments.renderer {
            None => RendererImplementation::Default,
            Some(implementation) => implementation
        };
    let mut renderer =
        match renderer_implementation {
            RendererImplementation::Default => cli_renderer::create(),
            RendererImplementation::Cli => cli_renderer::create(),
            RendererImplementation::Bevy => bevy_renderer::create()
        };
    renderer.initialize();
    renderer.start();
}
