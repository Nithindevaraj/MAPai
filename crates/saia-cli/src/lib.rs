pub mod mcp_server;
pub mod fti;
pub mod ui;
pub mod acp;

pub use fti::{FirstTimeInstaller, DeviceProfile};
pub use ui::WebStudio;
pub use acp::AcpServer;

#[cfg(test)]
mod tests {
    use super::mcp_server::McpServer;

    #[test]
    fn test_generate_badge() {
        let badge = McpServer::generate_badge();
        assert!(badge.contains("MAPai-SAiA ACTIVE SOVEREIGN SIDECAR"));
        assert!(badge.contains("Truth-to-Materials"));
    }
}
