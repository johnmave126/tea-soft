#![feature(test)]
use block_cipher::bench;

bench!(tea_soft::Tea32, 16);
