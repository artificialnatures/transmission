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

pub struct Renderer {
    pub initialize: fn() -> (),
    pub start: fn() -> ()
}

impl Renderer {
    pub fn new(implementation: RendererImplementation) -> Renderer {
        Renderer {
            initialize: || {},
            start: || {}
        }
    }
}