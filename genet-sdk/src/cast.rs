//! Cast objects for the usual data structures.

use crate::{
    attr::{Attr, NodeBuilder, SizedField},
    context::Context,
    slice,
};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
pub use genet_abi::cast::{Cast, Map, Nil, Typed};
use genet_abi::slice::TryGet;

use std::{
    io::{Cursor, Error, ErrorKind, Result},
    mem::size_of,
};

/// Cast for 8bit unsigned integer.
#[derive(Clone, Default)]
pub struct UInt8();

impl Typed for UInt8 {
    type Output = u8;

    fn cast(&self, attr: &Attr, data: &slice::ByteSlice) -> Result<u8> {
        Cursor::new(data.try_get(attr.range())?).read_u8()
    }
}

impl SizedField for UInt8 {
    fn bit_size(&self) -> usize {
        size_of::<u8>() * 8
    }
}

impl NodeBuilder<Self> for UInt8 {
    type Builder = Self;

    fn build(_ctx: &Context) -> Self {
        UInt8()
    }
}

/// Cast for 8bit signed integer.
#[derive(Clone, Default)]
pub struct Int8();

impl Typed for Int8 {
    type Output = i8;

    fn cast(&self, attr: &Attr, data: &slice::ByteSlice) -> Result<i8> {
        Cursor::new(data.try_get(attr.range())?).read_i8()
    }
}

impl SizedField for Int8 {
    fn bit_size(&self) -> usize {
        size_of::<i8>() * 8
    }
}

impl NodeBuilder<Self> for Int8 {
    type Builder = Self;

    fn build(_ctx: &Context) -> Self {
        Int8()
    }
}

/// Cast for big-endian 16bit unsigned integer.
#[derive(Clone, Default)]
pub struct UInt16BE();

impl Typed for UInt16BE {
    type Output = u16;

    fn cast(&self, attr: &Attr, data: &slice::ByteSlice) -> Result<u16> {
        Cursor::new(data.try_get(attr.range())?).read_u16::<BigEndian>()
    }
}

impl SizedField for UInt16BE {
    fn bit_size(&self) -> usize {
        size_of::<u16>() * 8
    }
}

impl NodeBuilder<Self> for UInt16BE {
    type Builder = Self;

    fn build(_ctx: &Context) -> Self {
        UInt16BE()
    }
}

/// Cast for big-endian 32bit unsigned integer.
#[derive(Clone, Default)]
pub struct UInt32BE();

impl Typed for UInt32BE {
    type Output = u32;

    fn cast(&self, attr: &Attr, data: &slice::ByteSlice) -> Result<u32> {
        Cursor::new(data.try_get(attr.range())?).read_u32::<BigEndian>()
    }
}

impl SizedField for UInt32BE {
    fn bit_size(&self) -> usize {
        size_of::<u32>() * 8
    }
}

impl NodeBuilder<Self> for UInt32BE {
    type Builder = Self;

    fn build(_ctx: &Context) -> Self {
        UInt32BE()
    }
}

/// Cast for big-endian 64bit unsigned integer.
#[derive(Clone, Default)]
pub struct UInt64BE();

impl Typed for UInt64BE {
    type Output = u64;

    fn cast(&self, attr: &Attr, data: &slice::ByteSlice) -> Result<u64> {
        Cursor::new(data.try_get(attr.range())?).read_u64::<BigEndian>()
    }
}

impl SizedField for UInt64BE {
    fn bit_size(&self) -> usize {
        size_of::<u64>() * 8
    }
}

impl NodeBuilder<Self> for UInt64BE {
    type Builder = Self;

    fn build(_ctx: &Context) -> Self {
        UInt64BE()
    }
}

/// Cast for big-endian 16bit signed integer.
#[derive(Clone, Default)]
pub struct Int16BE();

impl Typed for Int16BE {
    type Output = i16;

    fn cast(&self, attr: &Attr, data: &slice::ByteSlice) -> Result<i16> {
        Cursor::new(data.try_get(attr.range())?).read_i16::<BigEndian>()
    }
}

impl NodeBuilder<Self> for Int16BE {
    type Builder = Self;

    fn build(_ctx: &Context) -> Self {
        Int16BE()
    }
}

impl SizedField for Int16BE {
    fn bit_size(&self) -> usize {
        size_of::<i16>() * 8
    }
}

/// Cast for big-endian 32bit signed integer.
#[derive(Clone, Default)]
pub struct Int32BE();

impl Typed for Int32BE {
    type Output = i32;

    fn cast(&self, attr: &Attr, data: &slice::ByteSlice) -> Result<i32> {
        Cursor::new(data.try_get(attr.range())?).read_i32::<BigEndian>()
    }
}

impl SizedField for Int32BE {
    fn bit_size(&self) -> usize {
        size_of::<i32>() * 8
    }
}

impl NodeBuilder<Self> for Int32BE {
    type Builder = Self;

    fn build(_ctx: &Context) -> Self {
        Int32BE()
    }
}

/// Cast for big-endian 64bit signed integer.
#[derive(Clone, Default)]
pub struct Int64BE();

impl Typed for Int64BE {
    type Output = i64;

    fn cast(&self, attr: &Attr, data: &slice::ByteSlice) -> Result<i64> {
        Cursor::new(data.try_get(attr.range())?).read_i64::<BigEndian>()
    }
}

impl SizedField for Int64BE {
    fn bit_size(&self) -> usize {
        size_of::<i64>() * 8
    }
}

impl NodeBuilder<Self> for Int64BE {
    type Builder = Self;

    fn build(_ctx: &Context) -> Self {
        Int64BE()
    }
}

/// Cast for big-endian 32bit floating point number.
#[derive(Clone, Default)]
pub struct Float32BE();

impl Typed for Float32BE {
    type Output = f32;

    fn cast(&self, attr: &Attr, data: &slice::ByteSlice) -> Result<f32> {
        Cursor::new(data.try_get(attr.range())?).read_f32::<BigEndian>()
    }
}

impl SizedField for Float32BE {
    fn bit_size(&self) -> usize {
        size_of::<f32>() * 8
    }
}

impl NodeBuilder<Self> for Float32BE {
    type Builder = Self;

    fn build(_ctx: &Context) -> Self {
        Float32BE()
    }
}

/// Cast for big-endian 64bit floating point number.
#[derive(Clone, Default)]
pub struct Float64BE();

impl Typed for Float64BE {
    type Output = f64;

    fn cast(&self, attr: &Attr, data: &slice::ByteSlice) -> Result<f64> {
        Cursor::new(data.try_get(attr.range())?).read_f64::<BigEndian>()
    }
}

impl SizedField for Float64BE {
    fn bit_size(&self) -> usize {
        size_of::<f64>() * 8
    }
}

impl NodeBuilder<Self> for Float64BE {
    type Builder = Self;

    fn build(_ctx: &Context) -> Self {
        Float64BE()
    }
}

/// Cast for little-endian 16bit unsigned integer.
#[derive(Clone, Default)]
pub struct UInt16LE();

impl Typed for UInt16LE {
    type Output = u16;

    fn cast(&self, attr: &Attr, data: &slice::ByteSlice) -> Result<u16> {
        Cursor::new(data.try_get(attr.range())?).read_u16::<LittleEndian>()
    }
}

impl SizedField for UInt16LE {
    fn bit_size(&self) -> usize {
        size_of::<u16>() * 8
    }
}

impl NodeBuilder<Self> for UInt16LE {
    type Builder = Self;

    fn build(_ctx: &Context) -> Self {
        UInt16LE()
    }
}

/// Cast for little-endian 32bit unsigned integer.
#[derive(Clone, Default)]
pub struct UInt32LE();

impl Typed for UInt32LE {
    type Output = u32;

    fn cast(&self, attr: &Attr, data: &slice::ByteSlice) -> Result<u32> {
        Cursor::new(data.try_get(attr.range())?).read_u32::<LittleEndian>()
    }
}

impl SizedField for UInt32LE {
    fn bit_size(&self) -> usize {
        size_of::<u32>() * 8
    }
}

impl NodeBuilder<Self> for UInt32LE {
    type Builder = Self;

    fn build(_ctx: &Context) -> Self {
        UInt32LE()
    }
}

/// Cast for little-endian 64bit unsigned integer.
#[derive(Clone, Default)]
pub struct UInt64LE();

impl Typed for UInt64LE {
    type Output = u64;

    fn cast(&self, attr: &Attr, data: &slice::ByteSlice) -> Result<u64> {
        Cursor::new(data.try_get(attr.range())?).read_u64::<LittleEndian>()
    }
}

impl SizedField for UInt64LE {
    fn bit_size(&self) -> usize {
        size_of::<u64>() * 8
    }
}

impl NodeBuilder<Self> for UInt64LE {
    type Builder = Self;

    fn build(_ctx: &Context) -> Self {
        UInt64LE()
    }
}

/// Cast for little-endian 16bit signed integer.
#[derive(Clone, Default)]
pub struct Int16LE();

impl Typed for Int16LE {
    type Output = i16;

    fn cast(&self, attr: &Attr, data: &slice::ByteSlice) -> Result<i16> {
        Cursor::new(data.try_get(attr.range())?).read_i16::<LittleEndian>()
    }
}

impl SizedField for Int16LE {
    fn bit_size(&self) -> usize {
        size_of::<i16>() * 8
    }
}

impl NodeBuilder<Self> for Int16LE {
    type Builder = Self;

    fn build(_ctx: &Context) -> Self {
        Int16LE()
    }
}

/// Cast for little-endian 32bit signed integer.
#[derive(Clone, Default)]
pub struct Int32LE();

impl Typed for Int32LE {
    type Output = i32;

    fn cast(&self, attr: &Attr, data: &slice::ByteSlice) -> Result<i32> {
        Cursor::new(data.try_get(attr.range())?).read_i32::<LittleEndian>()
    }
}

impl SizedField for Int32LE {
    fn bit_size(&self) -> usize {
        size_of::<i32>() * 8
    }
}

impl NodeBuilder<Self> for Int32LE {
    type Builder = Self;

    fn build(_ctx: &Context) -> Self {
        Int32LE()
    }
}

/// Cast for little-endian 64bit signed integer.
#[derive(Clone, Default)]
pub struct Int64LE();

impl Typed for Int64LE {
    type Output = i64;

    fn cast(&self, attr: &Attr, data: &slice::ByteSlice) -> Result<i64> {
        Cursor::new(data.try_get(attr.range())?).read_i64::<LittleEndian>()
    }
}

impl SizedField for Int64LE {
    fn bit_size(&self) -> usize {
        size_of::<i64>() * 8
    }
}

impl NodeBuilder<Self> for Int64LE {
    type Builder = Self;

    fn build(_ctx: &Context) -> Self {
        Int64LE()
    }
}

/// Cast for little-endian 32bit floating point number.
#[derive(Clone, Default)]
pub struct Float32LE();

impl Typed for Float32LE {
    type Output = f32;

    fn cast(&self, attr: &Attr, data: &slice::ByteSlice) -> Result<f32> {
        Cursor::new(data.try_get(attr.range())?).read_f32::<LittleEndian>()
    }
}

impl SizedField for Float32LE {
    fn bit_size(&self) -> usize {
        size_of::<f32>() * 8
    }
}

impl NodeBuilder<Self> for Float32LE {
    type Builder = Self;

    fn build(_ctx: &Context) -> Self {
        Float32LE()
    }
}

/// Cast for little-endian 64bit floating point number.
#[derive(Clone, Default)]
pub struct Float64LE();

impl Typed for Float64LE {
    type Output = f64;

    fn cast(&self, attr: &Attr, data: &slice::ByteSlice) -> Result<f64> {
        Cursor::new(data.try_get(attr.range())?).read_f64::<LittleEndian>()
    }
}

impl SizedField for Float64LE {
    fn bit_size(&self) -> usize {
        size_of::<f64>() * 8
    }
}

impl NodeBuilder<Self> for Float64LE {
    type Builder = Self;

    fn build(_ctx: &Context) -> Self {
        Float64LE()
    }
}

/// Cast for UTF-8 string.
#[derive(Clone, Default)]
pub struct Utf8();

impl Typed for Utf8 {
    type Output = Box<str>;

    fn cast(&self, attr: &Attr, data: &slice::ByteSlice) -> Result<Box<str>> {
        if let Ok(s) = String::from_utf8(data.try_get(attr.range())?.to_vec()) {
            Ok(s.into_boxed_str())
        } else {
            Err(Error::new(ErrorKind::InvalidData, "Invalid UTF-8"))
        }
    }
}

impl NodeBuilder<Self> for Utf8 {
    type Builder = Self;

    fn build(_ctx: &Context) -> Self {
        Utf8()
    }
}

/// Cast for ByteSlice.
#[derive(Clone, Default)]
pub struct ByteSlice();

impl Typed for ByteSlice {
    type Output = slice::ByteSlice;

    fn cast(&self, attr: &Attr, data: &slice::ByteSlice) -> Result<slice::ByteSlice> {
        Ok(data.try_get(attr.range())?)
    }
}

impl NodeBuilder<Self> for ByteSlice {
    type Builder = Self;

    fn build(_ctx: &Context) -> Self {
        ByteSlice()
    }
}

#[derive(Clone, Default)]
pub struct BitFlag();

impl Typed for BitFlag {
    type Output = bool;

    fn cast(&self, attr: &Attr, data: &slice::ByteSlice) -> Result<bool> {
        let byte = Cursor::new(data.try_get(attr.range())?).read_u8()?;
        Ok((byte & (0b1000_0000 >> (attr.bit_range().start % 8))) != 0)
    }
}

impl SizedField for BitFlag {
    fn bit_size(&self) -> usize {
        1
    }
}

impl NodeBuilder<Self> for BitFlag {
    type Builder = Self;

    fn build(_ctx: &Context) -> Self {
        BitFlag()
    }
}
