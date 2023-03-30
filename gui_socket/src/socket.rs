#[derive(Copy, Clone)]
pub struct SmartSocket {
    on: bool,
    intake: i16,
}

impl SmartSocket {
    pub fn new() -> Self {
        Self {
            on: false,
            intake: 0,
        }
    }

    pub fn status(&self) -> String {
        match self.on {
            true => format!("Socket ON, intake {} V", self.intake),
            false => format!("Socket OFF, intake {} V", self.intake),
        }
    }

    pub fn switch(&mut self) {
        self.on = !self.on;

        if self.on {
            self.intake = 220
        } else {
            self.intake = 0
        }
    }
}

impl Default for SmartSocket {
    fn default() -> Self {
        Self::new()
    }
}
