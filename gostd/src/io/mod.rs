//! This module is waiting to be developed.
#![allow(unused)]
// #![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#[macro_use]
use crate::builtin::*;
use std::io::Error;

pub enum Whence {
    SeekStat,
    SeekCurrent,
    SeekEnd,
}
pub trait Reader {
    fn Read(&mut self, b: Vec<byte>) -> Result<int, Error>
    where
        Self: Sized;
}

pub trait Writer {
    fn Write(&mut self, b: Vec<byte>) -> Result<int, Error>
    where
        Self: Sized;
}

pub trait ReaderAt {
    fn ReadAt(&mut self, b: Vec<byte>, off: int64) -> Result<int, Error>
    where
        Self: Sized;
}

pub trait ByteReader {
    fn ReadByte(&mut self) -> Result<byte, Error>
    where
        Self: Sized;
}

pub trait RuneReader {
    fn ReadRune(&mut self) -> Result<(rune, int), Error>
    where
        Self: Sized;
}

pub trait Seeker {
    fn Seek(&mut self, offset: int64, whence: Whence) -> int64
    where
        Self: Sized;
}

pub trait ByteScanner {
    fn UnreadByte(&mut self) -> Result<int, Error>
    where
        Self: Sized;
}

pub trait WriterTo {
    fn WriteTo(&self, w: Box<dyn Writer>) -> Result<int64, &str>
    where
        Self: Sized;
}
