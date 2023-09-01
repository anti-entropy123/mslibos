use derive_more::Display;

#[derive(Debug, Display)]
pub enum LibOSErr {
    Unknown,

    BadFileDescriptor,
    NoReadPerm,
    NoWritePerm,

    TcpListenErr,
    PhyWaitErr,
    WrongSockState,
}

pub type LibOSResult<T> = Result<T, LibOSErr>;
