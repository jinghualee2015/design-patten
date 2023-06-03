mod nginx;
mod application;
pub use nginx::NginxServer;
pub trait Server {
    fn handle_request(&mut self, url: &str, method: &str) -> (u16, String);
}