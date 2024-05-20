#[derive(Debug)]
pub struct CustomSmartPointer {
    data: String,
}

impl CustomSmartPointer {
    pub fn new(data: String) -> Self {
        Self { data }
    }

    pub fn get_data(&self) -> &str {
        &self.data
    }
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("\nDropping CustumSmartPointer with data {:?} !", self.data)
    }
}
