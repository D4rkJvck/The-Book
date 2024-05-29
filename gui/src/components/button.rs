use gui::Draw;

#[allow(dead_code)]
pub struct Button {
    width: u32,
    height: u32,
    label: String,
}

impl Button {
    pub fn new(width: u32, height: u32, label: String) -> Self {
        Self {
            width,
            height,
            label,
        }
    }
}

impl Draw for Button {
    fn draw(&self) {
        todo!();
    }
}
