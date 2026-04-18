use std::{async_iter::AsyncIterator, future::poll_fn, pin::Pin};

pub trait AsyncIteratorNext: AsyncIterator {
    fn next(&mut self) -> impl std::future::Future<Output = Option<Self::Item>> + Send
    where
        Self: Unpin + Send,
    {
        async { poll_fn(|cx| Pin::new(&mut *self).poll_next(cx)).await }
    }
}

impl<T: AsyncIterator> AsyncIteratorNext for T {}

pub(crate) fn parse_sse_data_frames(buf: &str) -> impl Iterator<Item = &str> {
    buf.trim()
        .split("\n\n")
        .filter(|&frame| frame != "")
        .map(|frame| frame.strip_prefix("data: ").unwrap())
        .take_while(|data| *data != "[DONE]")
}
