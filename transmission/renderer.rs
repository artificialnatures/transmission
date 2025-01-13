use transmission_ui_bevy::bevy_renderer::BevyRenderer;
use transmission_ui_cli::cli_renderer::CliRenderer;

#[derive(Debug, Clone)]
pub enum RendererImplementation {
    Default,
    Cli,
    Bevy
}

impl From<String> for RendererImplementation {
    fn from(implementation_string: String) -> RendererImplementation {
        match implementation_string.as_str() {
            "default" => RendererImplementation::Default,
            "cli" => RendererImplementation::Cli,
            "bevy" => RendererImplementation::Bevy,
            _ => RendererImplementation::Default
        }
    }
}

pub enum Renderer {
    Cli(CliRenderer),
    Bevy(BevyRenderer)
}

impl Renderer {
    pub fn new(implementation: RendererImplementation) -> Renderer {
        match implementation {
            RendererImplementation::Default | RendererImplementation::Cli => {
                Renderer::Cli(CliRenderer::new())
            },
            RendererImplementation::Bevy => {
                Renderer::Bevy(BevyRenderer::new())
            }
        }
    }
}