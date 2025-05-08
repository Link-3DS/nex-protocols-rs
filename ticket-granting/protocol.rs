use nex_rs::server::PRUDPServer;

pub enum TicketGrantingMethod {
    protocol_id = 0xA,
    method_login = 0x1,
    method_login_ex = 0x2,
    method_request_ticket = 0x3,
    method_get_pid = 0x4,
    method_get_name = 0x5,
    method_login_with_context = 0x6,
}

pub trait TicketGrantingProtocol: PRUDPServer {

}