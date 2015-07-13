use std::io::Error as IoError;
use std::fs::File;
use std::os::unix::io::RawFd;
use nix::unistd::{pipe};
use nix::Error::{Sys, InvalidPath};
use nix::errno::Errno::EPIPE;

use libc::{c_int, c_void};
use libc::funcs::posix88::unistd::{close, write};
use libc::consts::os::posix88::{EINTR, EAGAIN};


pub struct CPipe {
    reader: RawFd,
    writer: RawFd,
}

impl CPipe {
    pub fn new() -> Result<CPipe, IoError> {
        match pipe() {
            Ok((reader, writer)) => Ok(CPipe {
                reader: reader, writer: writer
            }),
            Err(Sys(code)) => Err(IoError::from_raw_os_error(code as i32)),
            Err(InvalidPath) => unreachable!(),
        }
    }
    pub fn read(self) -> Result<Vec<u8>, IoError> {
        let CPipe {reader, writer} = self;
        let mut buf = Vec::new();
        close(reader);
        let res = File::from_raw_fd(self.reader).read_to_end(&mut buf);
        close(writer);
        try!(res);
        Ok(buf)
    }
}

impl Drop for CPipe {
    fn drop(&mut self) {
        unsafe {
            close(self.reader);
            close(self.writer);
        }
    }
}
