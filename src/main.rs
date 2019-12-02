mod phy;

use async_std::future;
use async_std::task;

use sysconfig::setup_ip;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let socket = phy::TunSocket::new("utun");
    let name = socket.name();
    setup_ip(&name, "192.168.10.1", "192.168.10.0/24");
    println!("utun name is {}", name);
    let mut buf1 = [0u8; 100];
    let mut buf2 = [0u8; 200];
    let mut buf3 = [0u8; 400];
    let mut buf4 = [0u8; 800];
    let bufs = [&mut buf1[..], &mut buf2[..], &mut buf3[..], &mut buf4[..]];
    let _: std::io::Result<()> = task::block_on(async {
        loop {
            eprintln!("loop poll");
            let sizes = future::poll_fn(|cx| {
                eprintln!("future:poll_fn");
                socket.poll_recvmmsg(cx, &bufs[..])
            }).await?;
            println!("recv {} packets", sizes.len());
            for (i, size) in sizes.iter().enumerate() {
                println!("\tsize: {}, bytes: {:?}", size, &bufs[i][..*size]);
            }
        }
        Ok(())
    });

    Ok(())
}
