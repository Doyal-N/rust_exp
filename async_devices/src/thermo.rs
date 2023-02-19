pub struct Thermo {
    pub name: String,
    pub value: f64,
}

impl Thermo {
    pub fn new(name: String, value: f64) -> Self {
        Self { name, value }
    }
}
