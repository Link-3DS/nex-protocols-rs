use nex_rs::server::PRUDPServer;

pub enum MonitoringMethod {
    protocol_id = 0x13,
    method_ping_daemon = 0x1,
    method_get_cluster_members = 0x2,
}

pub trait MonitoringProtocol: PRUDPServer {

}