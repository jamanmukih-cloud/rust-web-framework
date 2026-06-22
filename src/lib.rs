pub mod router;
pub mod middleware;
pub mod request;
pub mod response;

pub struct App {
    router: Router,
}

impl App {
    pub fn new() -> Self {
        Self { router: Router::new() }
    }
    
    pub fn route<F>(mut self, path: &str, handler: F) -> Self
    where
        F: Fn() -> std::future::Ready<&'static str> + Send + Sync + 'static,
    {
        self.router.add(path, handler);
        self
    }
    
    pub async fn listen(self, addr: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("Listening on {}", addr);
        Ok(())
    }
}
