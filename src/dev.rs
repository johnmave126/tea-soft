/// Define benchmarks. Add parallel benchmark on top of benchmarks provided by block-cipher
#[macro_export]
#[cfg_attr(docsrs, doc(cfg(feature = "dev")))]
macro_rules! bench {
    ($cipher:path, $key_len:expr) => {
        block_cipher::bench!($cipher, $key_len);

		#[bench]
        pub fn encrypt_blocks(bh: &mut Bencher) {
            let state = <$cipher>::new_varkey(&[1u8; $key_len]).unwrap();
            let mut blocks = Default::default();

            bh.iter(|| {
                state.encrypt_blocks(&mut blocks);
                test::black_box(&blocks);
            });
            bh.bytes = (blocks[0].len() * blocks.len()) as u64;
        }

        #[bench]
        pub fn decrypt_blocks(bh: &mut Bencher) {
            let state = <$cipher>::new_varkey(&[1u8; $key_len]).unwrap();
            let mut blocks = Default::default();

            bh.iter(|| {
                state.decrypt_blocks(&mut blocks);
                test::black_box(&blocks);
            });
            bh.bytes = (blocks[0].len() * blocks.len()) as u64;
        }
    };
}
