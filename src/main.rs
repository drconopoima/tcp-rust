use std::io;
use tun_tap;

fn main() -> io::Result<()> {
    let nic = tun_tap::Iface::new("tun%d", tun_tap::Mode::Tun)?;
    let mut buf = [0u8; 1504];
    let nbytes = nic.recv(&mut buf[..])?;
    eprintln!("read {} bytes: {:02x?}", nbytes, &buf[..nbytes]);
    Ok(())
}
