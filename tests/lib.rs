//! Test vectors are generated using the same method as
//! http://www.cix.co.uk/~klockstone/teavect.htm
#![no_std]

use cipher::block_cipher_test;

block_cipher_test!(tea16_test, "tea16", tea_soft::Tea16);
block_cipher_test!(tea32_test, "tea32", tea_soft::Tea32);
block_cipher_test!(tea64_test, "tea64", tea_soft::Tea64);
