pub struct Context {
    pub data: String,
}

impl Context {
    pub fn new() -> Self {
        Self {
            data: String::new(),
        }
    }
}

impl juniper::Context for Context {}
