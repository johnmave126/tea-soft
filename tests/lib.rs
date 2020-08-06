//! Test vectors are generated using the same method as
//! http://www.cix.co.uk/~klockstone/teavect.htm
#![no_std]

use block_cipher::new_test;

new_test!(tea16_test, "tea16", tea_soft::Tea16);
new_test!(tea32_test, "tea32", tea_soft::Tea32);
new_test!(tea64_test, "tea64", tea_soft::Tea64);
