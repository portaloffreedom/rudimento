use renderer::Renderer;

pub struct PixmanRenderer {

}

impl Renderer for PixmanRenderer {

}

impl PixmanRenderer {
    pub fn new() -> Result<Box<PixmanRenderer>,String> {
        Err("PixmanRenderer not implemented yet".to_string())
    }
}
