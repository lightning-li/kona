//! This module contains derivation errors thrown within the pipeline.

use alloy_primitives::B256;
use core::fmt::Display;

use super::SpanBatchError;

/// An error that is thrown within the stages of the derivation pipeline.
#[derive(Debug)]
pub enum StageError {
    /// There is no data to read from the channel bank.
    Eof,
    /// There is not enough data progress, but if we wait, the stage will eventually return data
    /// or produce an EOF error.
    NotEnoughData,
    /// No channels are available in the channel bank.
    NoChannelsAvailable,
    /// Failed to find channel.
    ChannelNotFound,
    /// Missing L1 origin.
    MissingOrigin,
    /// The stage detected a block reorg.
    /// The first argument is the expected block hash.
    /// The second argument is the paren_hash of the next l1 origin block.
    ReorgDetected(B256, B256),
    /// Receipt fetching error.
    ReceiptFetch(anyhow::Error),
    /// [super::BlockInfo] fetching error.
    BlockInfoFetch(anyhow::Error),
    /// [super::SystemConfig] update error.
    SystemConfigUpdate(anyhow::Error),
    /// Other wildcard error.
    Custom(anyhow::Error),
}

impl PartialEq<StageError> for StageError {
    fn eq(&self, other: &StageError) -> bool {
        // if it's a reorg detected check the block hashes
        if let (StageError::ReorgDetected(a, b), StageError::ReorgDetected(c, d)) = (self, other) {
            return a == c && b == d;
        }
        matches!(
            (self, other),
            (StageError::Eof, StageError::Eof) |
                (StageError::NotEnoughData, StageError::NotEnoughData) |
                (StageError::NoChannelsAvailable, StageError::NoChannelsAvailable) |
                (StageError::ChannelNotFound, StageError::ChannelNotFound) |
                (StageError::MissingOrigin, StageError::MissingOrigin) |
                (StageError::ReceiptFetch(_), StageError::ReceiptFetch(_)) |
                (StageError::BlockInfoFetch(_), StageError::BlockInfoFetch(_)) |
                (StageError::SystemConfigUpdate(_), StageError::SystemConfigUpdate(_)) |
                (StageError::Custom(_), StageError::Custom(_))
        )
    }
}

/// A result type for the derivation pipeline stages.
pub type StageResult<T> = Result<T, StageError>;

impl From<anyhow::Error> for StageError {
    fn from(e: anyhow::Error) -> Self {
        StageError::Custom(e)
    }
}

impl Display for StageError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            StageError::Eof => write!(f, "End of file"),
            StageError::NotEnoughData => write!(f, "Not enough data"),
            StageError::NoChannelsAvailable => write!(f, "No channels available"),
            StageError::ChannelNotFound => write!(f, "Channel not found"),
            StageError::MissingOrigin => write!(f, "Missing L1 origin"),
            StageError::ReceiptFetch(e) => write!(f, "Receipt fetch error: {}", e),
            StageError::SystemConfigUpdate(e) => write!(f, "System config update error: {}", e),
            StageError::ReorgDetected(current, next) => {
                write!(f, "Block reorg detected: {} -> {}", current, next)
            }
            StageError::BlockInfoFetch(e) => write!(f, "Block info fetch error: {}", e),
            StageError::Custom(e) => write!(f, "Custom error: {}", e),
        }
    }
}

/// A decoding error.
#[derive(Debug)]
pub enum DecodeError {
    /// The buffer is empty.
    EmptyBuffer,
    /// Alloy RLP Encoding Error.
    AlloyRlpError(alloy_rlp::Error),
    /// Span Batch Error.
    SpanBatchError(SpanBatchError),
}

impl From<alloy_rlp::Error> for DecodeError {
    fn from(e: alloy_rlp::Error) -> Self {
        DecodeError::AlloyRlpError(e)
    }
}

impl PartialEq<DecodeError> for DecodeError {
    fn eq(&self, other: &DecodeError) -> bool {
        matches!(
            (self, other),
            (DecodeError::EmptyBuffer, DecodeError::EmptyBuffer) |
                (DecodeError::AlloyRlpError(_), DecodeError::AlloyRlpError(_))
        )
    }
}

impl Display for DecodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            DecodeError::EmptyBuffer => write!(f, "Empty buffer"),
            DecodeError::AlloyRlpError(e) => write!(f, "Alloy RLP Decoding Error: {}", e),
            DecodeError::SpanBatchError(e) => write!(f, "Span Batch Decoding Error: {:?}", e),
        }
    }
}
