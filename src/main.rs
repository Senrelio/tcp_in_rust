fn main() -> std::io::Result<()> {
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    let mut buf = [0u8; 1504];
    loop {
        let nbytes = nic.recv(&mut buf[..])?;
        let proto = u16::from_be_bytes([buf[2], buf[3]]);
        if proto != 0x0800 {
            continue;
        }
        match etherparse::Ipv4HeaderSlice::from_slice(&buf[4..nbytes]) {
            Ok(p) => {
                let src = p.source_addr();
                let dst = p.destination_addr();
                let proto = p.protocol();
                if proto != 0x06 {
                    // not tcp
                    continue;
                }
                match etherparse::TcpHeaderSlice::from_slice(&buf[4 + p.slice().len()..]) {
                    Ok(p) => {
                        eprintln!(
                            "{} -> {} {}b of tcp to port {}",
                            src,
                            dst,
                            p.slice().len(),
                            p.destination_port()
                        );
                    }
                    Err(e) => {
                        eprintln!("invalid packet: {:#?}", e);
                    }
                }
            }
            Err(err) => {
                eprintln!("invalid packet: {:#?}", err);
            }
        }
    }
    Ok(())
}
