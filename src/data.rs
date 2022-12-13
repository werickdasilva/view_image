use druid::{Data, Lens};

#[derive(Clone, Data, Lens)]
pub struct ViewImageData {
    pub url_image: Option<String>,
}

impl ViewImageData {
    pub fn new() -> Self {
        Self { url_image: None }
    }
}
