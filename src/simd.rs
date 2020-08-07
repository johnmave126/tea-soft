use core::ops::{BitXor, Shl, Shr};
use byteorder::{ByteOrder, BigEndian};
use crate::impls::Block8x4;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[allow(non_camel_case_types)]
pub struct u32x4(pub u32, pub u32, pub u32, pub u32);

pub trait WrapArithmetic<T> {
    fn wrapping_add(self, rhs: T) -> Self;
    fn wrapping_sub(self, rhs: T) -> Self;
}

impl WrapArithmetic<u32> for u32 {
    #[inline(always)]
    fn wrapping_add(self, rhs: u32) -> Self {
        u32::wrapping_add(self, rhs)
    }

    #[inline(always)]
    fn wrapping_sub(self, rhs: u32) -> Self {
        u32::wrapping_sub(self, rhs)
    }
}

impl WrapArithmetic<u32> for u32x4 {
    #[inline(always)]
    fn wrapping_add(self, rhs: u32) -> Self {
        u32x4(
            self.0.wrapping_add(rhs),
            self.1.wrapping_add(rhs),
            self.2.wrapping_add(rhs),
            self.3.wrapping_add(rhs),
        )
    }

    #[inline(always)]
    fn wrapping_sub(self, rhs: u32) -> Self {
        u32x4(
            self.0.wrapping_sub(rhs),
            self.1.wrapping_sub(rhs),
            self.2.wrapping_sub(rhs),
            self.3.wrapping_sub(rhs),
        )
    }
}

impl WrapArithmetic<u32x4> for u32x4 {
    #[inline(always)]
    fn wrapping_add(self, rhs: u32x4) -> Self {
        u32x4(
            self.0.wrapping_add(rhs.0),
            self.1.wrapping_add(rhs.1),
            self.2.wrapping_add(rhs.2),
            self.3.wrapping_add(rhs.3),
        )
    }

    #[inline(always)]
    fn wrapping_sub(self, rhs: u32x4) -> Self {
        u32x4(
            self.0.wrapping_sub(rhs.0),
            self.1.wrapping_sub(rhs.1),
            self.2.wrapping_sub(rhs.2),
            self.3.wrapping_sub(rhs.3),
        )
    }
}

impl BitXor for u32x4 {
    type Output = u32x4;

    #[inline(always)]
    fn bitxor(self, rhs: u32x4) -> u32x4 {
        u32x4(
            self.0 ^ rhs.0,
            self.1 ^ rhs.1,
            self.2 ^ rhs.2,
            self.3 ^ rhs.3,
        )
    }
}

impl Shl<usize> for u32x4 {
    type Output = u32x4;

    #[inline(always)]
    fn shl(self, rhs: usize) -> u32x4 {
        u32x4(
            self.0 << rhs,
            self.1 << rhs,
            self.2 << rhs,
            self.3 << rhs,
        )
    }
}

impl Shr<usize> for u32x4 {
    type Output = u32x4;

    #[inline(always)]
    fn shr(self, rhs: usize) -> u32x4 {
        u32x4(
            self.0 >> rhs,
            self.1 >> rhs,
            self.2 >> rhs,
            self.3 >> rhs,
        )
    }
}

#[inline(always)]
pub fn slice_block(blocks: &mut Block8x4) -> (u32x4, u32x4) {
    (u32x4(BigEndian::read_u32(&blocks[0][0..4]),
           BigEndian::read_u32(&blocks[1][0..4]),
           BigEndian::read_u32(&blocks[2][0..4]),
           BigEndian::read_u32(&blocks[3][0..4])),
     u32x4(BigEndian::read_u32(&blocks[0][4..8]),
           BigEndian::read_u32(&blocks[1][4..8]),
           BigEndian::read_u32(&blocks[2][4..8]),
           BigEndian::read_u32(&blocks[3][4..8])))
}

#[inline(always)]
pub fn unslice_block(xs: u32x4, ys: u32x4, blocks: &mut Block8x4) {
    BigEndian::write_u32(&mut blocks[0][0..4], xs.0);
    BigEndian::write_u32(&mut blocks[1][0..4], xs.1);
    BigEndian::write_u32(&mut blocks[2][0..4], xs.2);
    BigEndian::write_u32(&mut blocks[3][0..4], xs.3);

    BigEndian::write_u32(&mut blocks[0][4..8], ys.0);
    BigEndian::write_u32(&mut blocks[1][4..8], ys.1);
    BigEndian::write_u32(&mut blocks[2][4..8], ys.2);
    BigEndian::write_u32(&mut blocks[3][4..8], ys.3);
}
