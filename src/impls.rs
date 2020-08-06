pub use block_cipher::{BlockCipher, NewBlockCipher};

use byteorder::{ByteOrder, BigEndian};

use block_cipher::consts::{U1, U8, U16};
use block_cipher::generic_array::GenericArray;

const TEA_DELTA: u32 = 0x9E3779B9;

pub type Block8 = GenericArray<u8, U8>;

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
            type ParBlocks = U1;

            #[inline]
            fn encrypt_block(&self, block: &mut Block8) {
                let mut sum: u32 = 0;
                let mut x = BigEndian::read_u32(&block[0..4]);
                let mut y = BigEndian::read_u32(&block[4..8]);

                let k0 = self.key[0];
                let k1 = self.key[1];
                let k2 = self.key[2];
                let k3 = self.key[3];

                for _ in 0..$rounds {
                    sum = sum.wrapping_add(TEA_DELTA);
                    x = x.wrapping_add((y << 4).wrapping_add(k0) ^
                                       y.wrapping_add(sum) ^
                                       (y >> 5).wrapping_add(k1));
                    y = y.wrapping_add((x << 4).wrapping_add(k2) ^
                                       x.wrapping_add(sum) ^
                                       (x >> 5).wrapping_add(k3));
                }

                BigEndian::write_u32(&mut block[0..4], x);
                BigEndian::write_u32(&mut block[4..8], y);
            }

            #[inline]
            fn decrypt_block(&self, block: &mut Block8) {
                let mut sum: u32 = TEA_DELTA << $shift;
                let mut x = BigEndian::read_u32(&block[0..4]);
                let mut y = BigEndian::read_u32(&block[4..8]);

                let k0 = self.key[0];
                let k1 = self.key[1];
                let k2 = self.key[2];
                let k3 = self.key[3];

                for _ in 0..$rounds {
                    y = y.wrapping_sub((x << 4).wrapping_add(k2) ^
                                       x.wrapping_add(sum) ^
                                       (x >> 5).wrapping_add(k3));
                    x = x.wrapping_sub((y << 4).wrapping_add(k0) ^
                                       y.wrapping_add(sum) ^
                                       (y >> 5).wrapping_add(k1));
                    sum = sum.wrapping_sub(TEA_DELTA);
                }

                BigEndian::write_u32(&mut block[0..4], x);
                BigEndian::write_u32(&mut block[4..8], y);
            }
        }
    }
}

define_tea_impl!(Tea16, 16, 4, "TEA block cipher instance of 16 rounds");
define_tea_impl!(Tea32, 32, 5, "TEA block cipher instance of 32 rounds");
define_tea_impl!(Tea64, 64, 6, "TEA block cipher instance of 64 rounds");
