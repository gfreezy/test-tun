use std::io;
use std::task::{Context, Poll};

use async_std::net::driver::Watcher;

mod sys;

pub(crate) struct TunSocket {
    mtu: usize,
    name: String,
    watcher: Watcher<sys::TunSocket>,
}

#[allow(dead_code)]
impl TunSocket {
    pub fn new(name: &str) -> TunSocket {
        let watcher = Watcher::new(sys::TunSocket::new(name).expect("TunSocket::new"));
        TunSocket {
            name: watcher.get_ref().name().expect("get name"),
            mtu: watcher.get_ref().mtu().expect("get mut"),
            watcher,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn mtu(&self) -> usize {
        self.mtu
    }

    pub fn poll_recvmsg(&self, cx: &mut Context<'_>, buf: &mut [u8]) -> Poll<io::Result<usize>> {
        self.watcher.poll_read_with(cx, |inner| inner.recvmsg(buf))
    }

    pub fn poll_recvmmsg(
        &self,
        cx: &mut Context<'_>,
        bufs: &[&mut [u8]],
    ) -> Poll<io::Result<Vec<usize>>> {
        self.watcher
            .poll_read_with(cx, |inner| {
                eprintln!("poll_read_with");
                inner.recvmmsg(bufs)
            })
    }

    pub fn poll_sendmsg(&self, cx: &mut Context<'_>, buf: &[u8]) -> Poll<io::Result<usize>> {
        self.watcher.poll_write_with(cx, |inner| inner.sendmsg(buf))
    }

    pub fn poll_sendmmsg(
        &self,
        cx: &mut Context<'_>,
        bufs: &[&[u8]],
    ) -> Poll<io::Result<Vec<usize>>> {
        self.watcher
            .poll_write_with(cx, |inner| inner.sendmmsg(bufs))
    }
}
