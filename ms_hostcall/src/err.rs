pub enum LibOSErr {
    Unknown,

    BadFileDescriptor,
    NoReadPerm,
    NoWritePerm,

    TcpBindErr,
}

pub type LibOSResult<T> = Result<T, LibOSErr>;
