// Copyright 2020 - developers of the `grammers` project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Implementation of the several [MTProto transports].
//!
//! [MTProto transports]: https://core.telegram.org/mtproto#mtproto-transport
mod abridged;
mod full;
mod intermediate;

use crate::errors::TransportError;
pub use abridged::TransportAbridged;
pub use full::TransportFull;
pub use intermediate::TransportIntermediate;

/// The trait used by the transports to create instances of themselves.
pub trait Transport {
    type Encoder: Encoder;
    type Decoder: Decoder;

    fn instance() -> (Self::Encoder, Self::Decoder);
}

/// The trait used by the encoder part of a concrete transport.
pub trait Encoder {
    /// How much overhead does the transport incur, at a maximum.
    fn max_overhead(&self) -> usize;

    /// Write the protocol's magic into `output`.
    ///
    /// On success, return how many bytes were written.
    ///
    /// On failure, return how many bytes long the output buffer should have been.
    fn write_magic(&mut self, output: &mut [u8]) -> Result<usize, usize>;

    /// Write the packet from `input` into `output`.
    ///
    /// On success, return how many bytes were written.
    ///
    /// On failure, return how many bytes long the output buffer should have been.
    ///
    /// # Panics
    ///
    /// The input length must be a multiple of 4, or else a panic will occur.
    fn write_into<'a>(&mut self, input: &[u8], output: &mut [u8]) -> Result<usize, usize>;
}

/// The trait used by the decoder part of a concrete transport.
pub trait Decoder {
    /// Read a packet from `input` and return the body subslice.
    ///
    /// On success, return how many bytes were written.
    ///
    /// On failure, return either how many bytes long the input buffer should
    /// have been or decoding failure in which case the connection should end.
    fn read<'a>(&mut self, input: &'a [u8]) -> Result<&'a [u8], TransportError>;
}
