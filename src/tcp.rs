use etherparse::{Ipv4HeaderSlice, TcpHeaderSlice};
use std::net::Ipv4Addr;
pub struct State {}
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct SrcDstQuad {
    pub src: (Ipv4Addr, u16),
    pub dst: (Ipv4Addr, u16),
}

impl Default for State {
    fn default() -> Self {
        State {}
    }
}

impl State {
    pub fn on_packet<'a>(
        &mut self,
        ip_header: Ipv4HeaderSlice<'a>,
        tcp_header: TcpHeaderSlice<'a>,
        data: &'a [u8],
    ) {
        eprintln!(
            "{}:{} -> {}:{} | {} bytes of tcp",
            ip_header.source_addr(),
            tcp_header.source_port(),
            ip_header.destination_addr(),
            tcp_header.destination_port(),
            data.len()
        )
    }
}
