use super::Component;

pub struct File {
    name: &'static str,
}

impl File {
    pub fn new(name: &'static str) -> Self {
        Self { name }
    }
}

impl Component for File {
    fn search(&self, keyword: &str) {
        let is_contain = self.name.contains(keyword);
        println!("File is contain: {}, keyword {}", is_contain, keyword);
    }
}
