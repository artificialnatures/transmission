pub trait Renderer {
    fn initialize(&mut self) -> ();
    fn start(&mut self) -> ();
}