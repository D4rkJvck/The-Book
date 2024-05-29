use gui::Draw;

#[allow(dead_code)]
pub struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl SelectBox {
    pub fn new(width: u32, height: u32, options: Vec<String>) -> Self {
        Self {
            width,
            height,
            options,
        }
    }
}

impl Draw for SelectBox {
    fn draw(&self) {
        todo!()
    }
}
