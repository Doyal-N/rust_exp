use super::Component;

pub struct Dir {
    name: &'static str,
    components: Vec<Box<dyn Component>>,
}

impl Dir {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            components: vec![],
        }
    }

    pub fn add<T: Component + 'static>(&mut self, component: T) {
        self.components.push(Box::new(component));
    }
}

impl Component for Dir {
    fn search(&self, keyword: &str) {
        println!(
            "Searching recursively for keyword {} in Dir {}",
            keyword, self.name
        );

        for component in self.components.iter() {
            component.search(keyword);
        }
    }
}
