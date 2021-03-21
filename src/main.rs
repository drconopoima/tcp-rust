use etherparse::{Ipv4HeaderSlice, TcpHeaderSlice};
use std::collections::HashMap;
use std::io;
use tun_tap;

mod tcp;

macro_rules! skip_unparsable {
    ($res:expr) => {
        match $res {
            Ok(val) => val,
            Err(error) => {
                eprintln!("Skipping packet with errors {:?}:", error);
                continue;
            }
        }
    };
}

fn main() -> io::Result<()> {
    let mut connections: HashMap<tcp::SrcDstQuad, tcp::State> = Default::default();
    let nic = tun_tap::Iface::new("tun%d", tun_tap::Mode::Tun)?;
    let mut buf = [0u8; 1504];
    loop {
        let nbytes = nic.recv(&mut buf[..])?;
        let eth_proto = u16::from_be_bytes([buf[2], buf[3]]);
        if eth_proto != 0x0800 {
            // ipv4 only guard clause
            continue;
        }
        let ipv4_packet_header = skip_unparsable!(Ipv4HeaderSlice::from_slice(&buf[4..nbytes]));
        let ipv4_proto = ipv4_packet_header.protocol();
        if ipv4_proto != 0x06 {
            // tcp only guard clause
            continue;
        }
        let ipv4_header_length = ipv4_packet_header.slice().len();
        let tcp_packet_header = skip_unparsable!(TcpHeaderSlice::from_slice(
            &buf[4 + ipv4_header_length..nbytes]
        ));
        let tcp_header_length = tcp_packet_header.slice().len();
        let data_start_byte = 4 + ipv4_header_length + tcp_header_length;
        let source_address = ipv4_packet_header.source_addr();
        let source_port = tcp_packet_header.source_port();
        let destination_address = ipv4_packet_header.destination_addr();
        let destination_port = tcp_packet_header.destination_port();
        connections
            .entry(tcp::SrcDstQuad {
                src: (source_address, source_port),
                dst: (destination_address, destination_port),
            })
            .or_default()
            .on_packet(
                ipv4_packet_header,
                tcp_packet_header,
                &buf[data_start_byte..nbytes],
            );
        println!(
            "{}:{} -> {}:{} | {} bytes of protocol {:x?}",
            source_address,
            source_port,
            destination_address,
            destination_port,
            tcp_header_length,
            ipv4_proto
        );
    }
}
