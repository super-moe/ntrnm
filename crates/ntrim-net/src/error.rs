use std::error::Error;
use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("DNS query error")]
    DnsQueryError,
    #[error("TCP connect error")]
    TcpConnectError,
    #[error("TCP not connected")]
    TcpNotConnectedError,
    #[error("TCP write error: {0}")]
    TcpWriteError(Box<dyn Error>),
    #[error("TCP read error: {0}")]
    TcpReadError(Box<dyn Error>),
}

#[derive(Error, Debug)]
pub enum CodecError {
    #[error("Packet encode error: {0}")]
    EncodeError(Box<dyn Error>),
    #[error("Packet decode error: {0}")]
    DecodeError(Box<dyn Error>),
}