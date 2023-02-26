pub struct SmartSocket {
    on: bool,
}

impl SmartSocket {
    pub fn new() -> Self {
        Self { on: false }
    }

    pub fn status(&self) -> String {
        match self.on {
            true => "I-on\r\n".to_owned(),
            false => "I-off\r\n".to_owned(),
        }
    }

    pub fn switch(&mut self) {
        self.on = !self.on;
    }
}

impl Default for SmartSocket {
    fn default() -> Self {
        Self::new()
    }
}
