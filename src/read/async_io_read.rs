use crate::{
    error::ErrorCode,
    read::{Read, Reference},
    Error, Result,
};
use std::{
    cmp::min,
    io::{self, ErrorKind},
    pin::Pin,
    task::{Context, Poll},
};
use tokio::io::{AsyncRead, ReadBuf};

pub struct AsyncIoRead<'o, 'i, 'r, R>
where
    R: AsyncRead,
{
    reader: Pin<&'r mut R>,
    context: &'o mut Context<'i>,
    peek: Option<u8>,
}

impl<'o, 'i, 'r, R> AsyncIoRead<'o, 'i, 'r, R>
where
    R: AsyncRead,
{
    pub fn new(reader: Pin<&'r mut R>, context: &'o mut Context<'i>) -> Self {
        Self {
            reader,
            context,
            peek: None,
        }
    }

    fn read_u8(&mut self) -> Result<Option<u8>> {
        let mut buf = [0; 1];
        let mut buf = ReadBuf::new(&mut buf);
        match self.reader.as_mut().poll_read(self.context, &mut buf) {
            Poll::Pending => Err(Error::new(ErrorCode::Io(ErrorKind::WouldBlock.into()))),
            Poll::Ready(Ok(_)) => {
                if buf.filled().is_empty() {
                    return Ok(None);
                }

                Ok(Some(buf.filled()[0]))
            }
            Poll::Ready(Err(err)) => Err(err.into()),
        }
    }
}

impl<'de, 'o, 'i, 'r, R> Read<'de> for AsyncIoRead<'o, 'i, 'r, R>
where
    R: AsyncRead,
{
    fn next(&mut self) -> Result<Option<u8>> {
        match self.peek.take() {
            Some(byte) => Ok(Some(byte)),
            None => self.read_u8(),
        }
    }

    fn peek(&mut self) -> Result<Option<u8>> {
        match self.peek {
            Some(byte) => Ok(Some(byte)),
            None => {
                self.peek = self.read_u8()?;
                Ok(self.peek)
            }
        }
    }

    fn discard(&mut self) {
        self.peek = None;
    }

    fn parse_slice_until<'s>(
        &mut self,
        scratch: &'s mut Vec<u8>,
        until: u8,
    ) -> Result<Reference<'de, 's, [u8]>> {
        loop {
            let Some(peek) = self.peek()? else {
                return Err(Error::new(ErrorCode::UnexpectedEof));
            };

            if peek != until {
                scratch.push(peek)
            } else {
                break Ok(Reference::Copied(scratch.as_slice()));
            }
        }
    }

    fn parse_slice_len<'s>(
        &mut self,
        scratch: &'s mut Vec<u8>,
        len: usize,
    ) -> std::result::Result<Reference<'de, 's, [u8]>, (usize, crate::Error)> {
        scratch.reserve(len);
        let empty_bytes = scratch.spare_capacity_mut();
        let alloc_len = empty_bytes.len();

        let buf = &mut empty_bytes[..min(len, alloc_len)];
        let mut buf = ReadBuf::uninit(buf);

        while buf.filled().len() < len {
            match poll_read_into(self.reader.as_mut(), self.context, &mut buf) {
                Ok(0) => {
                    if buf.filled().len() == len {
                        break;
                    } else {
                        return Err((buf.filled().len(), Error::new(ErrorCode::UnexpectedEof)));
                    }
                }
                Ok(_) => {}
                Err(err) => return Err((buf.filled().len(), Error::new(ErrorCode::Io(err)))),
            }
        }

        Ok(Reference::Copied(scratch.as_slice()))
    }
}

/// Reads bytes into `buf` using [`AsyncRead::poll_read()`] and returns the amounts of bytes read.
/// When [`AsyncRead::poll_read()`] returns [`Poll::Pending`], returns [`ErrorKind::WouldBlock`] instead
/// and registers repolling in the [`Context`]
fn poll_read_into<R>(
    reader: Pin<&mut R>,
    cx: &mut Context<'_>,
    buf: &mut ReadBuf<'_>,
) -> io::Result<usize>
where
    R: AsyncRead,
{
    let old_len = buf.filled().len();
    match reader.poll_read(cx, buf) {
        Poll::Pending => Err(io::ErrorKind::WouldBlock.into()),
        Poll::Ready(Ok(_)) => Ok(buf.filled().len() - old_len),
        Poll::Ready(Err(err)) => Err(err),
    }
}
