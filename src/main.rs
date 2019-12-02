mod phy;

use async_std::prelude::*;
use async_std::task;

use sysconfig::setup_ip;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let mut socket = phy::TunSocket::new("utun");
    let name = socket.name();
    setup_ip(&name, "192.168.10.1", "192.168.10.0/24");
    println!("utun name is {}", name);
    let mut buf = [0u8; 100];
    let _: std::io::Result<()> = task::block_on(async {
        loop {
            let size = socket.read(&mut buf).await?;
            println!("read {} bytes: {:?}", size, &buf[..size]);
        }
        Ok(())
    });

    Ok(())
}
