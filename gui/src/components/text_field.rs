use gui::Draw;

#[allow(dead_code)]
pub struct TextField {
    width: u32,
    height: u32,
    label: String,
    placeholder: String,
}

impl TextField {
    pub fn new(width: u32, height: u32, label: String, placeholder: String) -> Self {
        Self {
            width,
            height,
            label,
            placeholder,
        }
    }
}

impl Draw for TextField {
    fn draw(&self) {
        todo!()
    }
}
