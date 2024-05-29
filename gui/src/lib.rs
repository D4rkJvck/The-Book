pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn new(components: Vec<Box<dyn Draw>>) -> Self {
        Self { components }
    }

    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw()
        }
    }
}
