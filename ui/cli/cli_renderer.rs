use transmission::renderer::Renderer;

pub struct CliRenderer {}

impl Renderer for CliRenderer {
    fn initialize(&mut self) {
        println!("Transmission initialized.")
    }

    fn start(&mut self) {
        println!("Transmission started.")
    }
}

pub fn create() -> Box<dyn Renderer> {
    Box::new(CliRenderer {})
}