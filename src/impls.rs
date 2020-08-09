pub use block_cipher::{BlockCipher, NewBlockCipher};
use core::ops::{BitXor, Shl, Shr};

use byteorder::{BigEndian, ByteOrder};

use block_cipher::consts::{U16, U4, U8};
use block_cipher::generic_array::GenericArray;

use crate::simd::{slice_block, unslice_block, WrapArithmetic};

const TEA_DELTA: u32 = 0x9E3779B9;

pub type Block8 = GenericArray<u8, U8>;
pub type Block8x4 = GenericArray<Block8, U4>;

macro_rules! define_tea_impl {
    (
        $name:ident,
        $rounds:expr,
        $shift:expr,
        $doc:expr
    ) => {
        #[doc=$doc]
        #[derive(Clone)]
        pub struct $name {
            key: [u32; 4],
        }

        impl $name {
            #[inline]
            fn encrypt_core<T>(&self, x: T, y: T) -> (T, T)
            where T: Copy +
                     WrapArithmetic<T> + WrapArithmetic<u32> +
                     BitXor<Output = T> +
                     Shl<usize, Output = T> + Shr<usize, Output = T> {
                let mut sum: u32 = 0;
                let (mut x, mut y) = (x, y);
                let k0 = self.key[0];
                let k1 = self.key[1];
                let k2 = self.key[2];
                let k3 = self.key[3];

                for _ in 0..$rounds {
                    sum = sum.wrapping_add(TEA_DELTA);
                    x = WrapArithmetic::wrapping_add(x,
                                                     WrapArithmetic::wrapping_add(y << 4, k0) ^
                                                     WrapArithmetic::wrapping_add(y, sum) ^
                                                     WrapArithmetic::wrapping_add(y >> 5, k1));
                    y = WrapArithmetic::wrapping_add(y,
                                                     WrapArithmetic::wrapping_add(x << 4, k2) ^
                                                     WrapArithmetic::wrapping_add(x, sum) ^
                                                     WrapArithmetic::wrapping_add(x >> 5, k3));
                }

                (x, y)
            }

            #[inline]
            fn decrypt_core<T>(&self, x: T, y: T) -> (T, T)
            where T: Copy +
                     WrapArithmetic<T> + WrapArithmetic<u32> +
                     BitXor<Output = T> +
                     Shl<usize, Output = T> + Shr<usize, Output = T> {
                let mut sum: u32 = TEA_DELTA << $shift;
                let (mut x, mut y) = (x, y);
                let k0 = self.key[0];
                let k1 = self.key[1];
                let k2 = self.key[2];
                let k3 = self.key[3];

                for _ in 0..$rounds {
                    y = WrapArithmetic::wrapping_sub(y,
                                                     WrapArithmetic::wrapping_add(x << 4, k2) ^
                                                     WrapArithmetic::wrapping_add(x, sum) ^
                                                     WrapArithmetic::wrapping_add(x >> 5, k3));
                    x = WrapArithmetic::wrapping_sub(x,
                                                     WrapArithmetic::wrapping_add(y << 4, k0) ^
                                                     WrapArithmetic::wrapping_add(y, sum) ^
                                                     WrapArithmetic::wrapping_add(y >> 5, k1));
                    sum = sum.wrapping_sub(TEA_DELTA);
                }

                (x, y)
            }
        }

        impl NewBlockCipher for $name {
            type KeySize = U16;

            #[inline]
            fn new(key: &GenericArray<u8, U16>) -> Self {
                Self {
                    key: [
                        BigEndian::read_u32(&key[0..4]),
                        BigEndian::read_u32(&key[4..8]),
                        BigEndian::read_u32(&key[8..12]),
                        BigEndian::read_u32(&key[12..16]),
                    ]
                }
            }
        }

        impl BlockCipher for $name {
            type BlockSize = U8;
            type ParBlocks = U4;

            #[inline]
            fn encrypt_block(&self, block: &mut Block8) {
                let x = BigEndian::read_u32(&block[0..4]);
                let y = BigEndian::read_u32(&block[4..8]);

                let (x, y) = self.encrypt_core(x, y);

                BigEndian::write_u32(&mut block[0..4], x);
                BigEndian::write_u32(&mut block[4..8], y);
            }

            #[inline]
            fn decrypt_block(&self, block: &mut Block8) {
                let x = BigEndian::read_u32(&block[0..4]);
                let y = BigEndian::read_u32(&block[4..8]);

                let (x, y) = self.decrypt_core(x, y);

                BigEndian::write_u32(&mut block[0..4], x);
                BigEndian::write_u32(&mut block[4..8], y);
            }

            #[inline]
            fn encrypt_blocks(&self, blocks: &mut Block8x4) {
                let (xs, ys) = slice_block(blocks);
                let (xs, ys) = self.encrypt_core(xs, ys);
                unslice_block(xs, ys, blocks);
            }

            #[inline]
            fn decrypt_blocks(&self, blocks: &mut Block8x4) {
                let (xs, ys) = slice_block(blocks);
                let (xs, ys) = self.decrypt_core(xs, ys);
                unslice_block(xs, ys, blocks);
            }
        }
    }
}

define_tea_impl!(Tea16, 16, 4, "TEA block cipher instance of 16 rounds");
define_tea_impl!(Tea32, 32, 5, "TEA block cipher instance of 32 rounds");
define_tea_impl!(Tea64, 64, 6, "TEA block cipher instance of 64 rounds");
