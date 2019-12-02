use async_std::io::{Read, Write};
use async_std::net::driver::Watcher;
use std::io;
use std::io::{Read as _, Write as _};
use std::pin::Pin;
use std::task::{Context, Poll};
mod sys;

pub(crate) struct TunSocket {
    mtu: usize,
    name: String,
    watcher: Watcher<sys::TunSocket>,
}

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
}

impl Read for TunSocket {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut &*self).poll_read(cx, buf)
    }
}

impl Read for &TunSocket {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        self.watcher.poll_read_with(cx, |mut inner| {
            let ret = inner.read(buf);
            eprintln!("inner.read: {:?}", ret);
            ret
        })
    }
}

impl Write for TunSocket {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut &*self).poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut &*self).poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut &*self).poll_close(cx)
    }
}

impl Write for &TunSocket {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        self.watcher
            .poll_write_with(cx, |mut inner| inner.write(buf))
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.watcher.poll_write_with(cx, |mut inner| inner.flush())
    }

    fn poll_close(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }
}
