use crate::{
    read::{Read, Reference},
    Error,
};

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct InteruptSlice<'a, 'de> {
    pub src: &'a mut [&'de [u8]],
}

impl<'a, 'de> Read<'de> for InteruptSlice<'a, 'de> {
    fn peek(&mut self) -> crate::Result<Option<u8>> {
        let slice = std::mem::take(&mut self.src);
        match slice {
            slice @ [&[peek, ..], ..] => {
                self.src = slice;
                Ok(Some(peek))
            }
            [[], rest @ ..] => {
                self.src = rest;
                Err(Error::Pending)
            }
            [] => {
                self.src = Default::default();
                Ok(None)
            }
        }
    }
    fn next(&mut self) -> crate::Result<Option<u8>> {
        let slice = std::mem::take(&mut self.src);
        match slice {
            [[], tail @ ..] => {
                self.src = tail;
                Err(Error::Pending)
            }
            [head @ &[next, ..], ..] => {
                if let [_, tail @ ..] = head {
                    *head = tail;
                } else {
                    unreachable!("We literally just checked this three lines above this");
                }

                self.src = slice;
                Ok(Some(next))
            }
            [] => {
                self.src = Default::default();
                Ok(None)
            }
        }
    }
    fn discard(&mut self) {
        let slice = std::mem::take(&mut self.src);
        match slice {
            [[], tail @ ..] => self.src = tail,
            [head @ &[_, ..], ..] => {
                if let [_, tail @ ..] = head {
                    *head = tail;
                } else {
                    unreachable!("We literally just checked this three lines above this");
                }
            }
            [] => {}
        }
    }

    fn read_until<'s>(
        &mut self,
        scratch: &'s mut Vec<u8>,
        until: u8,
    ) -> crate::Result<Reference<'de, 's, [u8]>> {
        if let Some(head) = self.src.first_mut() {
            if let Some((index, _)) = head.iter().enumerate().find(|(_, byte)| **byte == until) {
                let (bytes, tail) = head.split_at(index);
                let (_, tail) = tail
                    .split_first()
                    .expect("expected the `until` byte to exist");
                *head = tail;

                if scratch.is_empty() {
                    Ok(Reference::Borrowed(bytes))
                } else {
                    scratch.extend_from_slice(bytes);
                    Ok(Reference::Copied(scratch))
                }
            } else {
                let (head, tail) = std::mem::take(&mut self.src)
                    .split_first_mut()
                    .expect("expected head to exist");
                scratch.extend_from_slice(head);
                self.src = tail;
                Err(Error::Pending)
            }
        } else {
            Err(Error::Eof)
        }
    }
    fn read_len<'s>(
        &mut self,
        scratch: &'s mut Vec<u8>,
        len: usize,
    ) -> std::result::Result<Reference<'de, 's, [u8]>, (usize, Error)> {
        if let Some(head) = self.src.first_mut() {
            if len <= head.len() {
                let (bytes, tail) = head.split_at(len);
                *head = tail;

                if scratch.is_empty() {
                    Ok(Reference::Borrowed(bytes))
                } else {
                    scratch.extend_from_slice(bytes);
                    Ok(Reference::Copied(scratch))
                }
            } else {
                let (head, tail) = std::mem::take(&mut self.src)
                    .split_first_mut()
                    .expect("expected the `until` byte to exist");
                scratch.extend_from_slice(head);
                self.src = tail;
                Err((head.len(), (Error::Pending)))
            }
        } else {
            Err((0, Error::Eof))
        }
    }

    fn skip_until(&mut self, until: u8) -> crate::Result<()> {
        if let Some(head) = self.src.first_mut() {
            if let Some((index, _)) = head.iter().enumerate().find(|(_, byte)| **byte == until) {
                let (_, tail) = head.split_at(index + 1);
                *head = tail;
                Ok(())
            } else {
                let (_, tail) = std::mem::take(&mut self.src)
                    .split_first_mut()
                    .expect("expected head to exist");
                self.src = tail;
                Err(Error::Pending)
            }
        } else {
            Err(Error::Eof)
        }
    }

    fn starts_with(
        &mut self,
        scratch: &mut Vec<u8>,
        mut starts_with: &[u8],
    ) -> crate::Result<bool> {
        if !scratch.is_empty() {
            if let Some(remaining) = starts_with.strip_prefix(scratch.as_slice()) {
                if remaining.is_empty() {
                    scratch.clear();
                    return Ok(true);
                }
                starts_with = remaining;
            } else {
                return Ok(false);
            }
        }

        if let Some(head) = self.src.first_mut() {
            if head.len() < starts_with.len() {
                if *head != &starts_with[..head.len()] {
                    Ok(false)
                } else {
                    let (head, tail) = std::mem::take(&mut self.src)
                        .split_first_mut()
                        .expect("expected head to exist");

                    scratch.extend_from_slice(head);
                    self.src = tail;

                    Err(Error::Pending)
                }
            } else {
                if let Some(remaining) = head.strip_prefix(starts_with) {
                    *head = remaining;
                    scratch.clear();
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
        } else {
            Err(Error::Eof)
        }
    }
}
