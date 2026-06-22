use std::collections::HashMap;

pub struct Router {
    routes: HashMap<String, Box<dyn Fn()>>,
}

impl Router {
    pub fn new() -> Self {
        Self { routes: HashMap::new() }
    }
    
    pub fn add<F>(&mut self, path: &str, _handler: F)
    where
        F: Send + Sync + 'static,
    {
        self.routes.insert(path.to_string(), Box::new(|| {}));
    }
    
    pub fn match_route(&self, path: &str) -> Option<&str> {
        self.routes.get(path).map(|_| path)
    }
}
