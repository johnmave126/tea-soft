//! TEA (Tiny Encryption Algorithm) Block Cipher
//!
//! The `tea-soft` crate implements the TEA algorithm completely in software
//! with 16 rounds, 32 rounds, or 64 rounds.
//!
//! # Usage example
//! ```
//! use tea_soft::block_cipher::generic_array::GenericArray;
//! use tea_soft::block_cipher::{BlockCipher, NewBlockCipher};
//! use tea_soft::Tea32;
//! 
//! let key = GenericArray::from_slice(&[0u8; 16]);
//! let mut block = GenericArray::clone_from_slice(&[0u8; 8]);
//! // Initialize cipher
//! let cipher = tea_soft::Tea32::new(&key);
//!
//! let block_copy = block.clone();
//! // Encrypt block in-place
//! cipher.encrypt_block(&mut block);
//! // And decrypt it back
//! cipher.decrypt_block(&mut block);
//! assert_eq!(block, block_copy);
//! ```

#![deny(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms)]

pub use block_cipher;

mod impls;

pub use crate::impls::{Tea16, Tea32, Tea64};
