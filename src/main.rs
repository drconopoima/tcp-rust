use etherparse::Ipv4HeaderSlice;
use std::io;
use tun_tap;

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
    let nic = tun_tap::Iface::new("tun%d", tun_tap::Mode::Tun)?;
    let mut buf = [0u8; 1504];
    loop {
        let nbytes = nic.recv(&mut buf[..])?;
        let eth_proto = u16::from_be_bytes([buf[2], buf[3]]);
        if eth_proto == 0x0800 {
            // ipv4 only
            let packet = skip_unparsable!(Ipv4HeaderSlice::from_slice(&buf[4..nbytes]));
            let source = packet.source_addr();
            let destination = packet.destination_addr();
            let ipv4_proto = packet.protocol();
            println!(
                "{} -> {}: {} bytes of protocol {:x?}",
                source,
                destination,
                packet.payload_len(),
                ipv4_proto
            );
        }
    }
}
