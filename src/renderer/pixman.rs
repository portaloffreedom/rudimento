use renderer::Renderer;

pub struct PixmanRenderer {

}

impl Renderer for PixmanRenderer {

}

impl PixmanRenderer {
    pub fn new() -> Result<Box<PixmanRenderer>,String> {
        Ok(Box::new(PixmanRenderer {

        }))
    }
}
