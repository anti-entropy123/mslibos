use crate::libos::USER_HOST_CALL;

pub macro func_type {
    (write) => (ms_hostcall::types::WriteFunc),
    (open) => (ms_hostcall::types::OpenFunc),
    (read) => (ms_hostcall::types::ReadFunc),
    (close) => (ms_hostcall::types::CloseFunc),
    (connect) => (ms_hostcall::types::ConnectFunc),
    (bind) => (ms_hostcall::types::BindFunc),
    (fatfs_open) => (ms_hostcall::types::FatfsOpenFunc),
    (fatfs_write) => (ms_hostcall::types::FatfsWriteFunc),
    (fatfs_read) => (ms_hostcall::types::FatfsReadFunc),
    (fatfs_close) => (ms_hostcall::types::FatfsCloseFunc),
    (stdout) => (ms_hostcall::types::HostStdioFunc),
    (addrinfo) => (ms_hostcall::types::SmoltcpAddrInfoFunc),
    (smol_connect) => (ms_hostcall::types::SmoltcpConnectFunc),
    (send) => (ms_hostcall::types::SmoltcpSendFunc),
    (recv) => (ms_hostcall::types::SmoltcpRecvFunc),
    (buffer_alloc) => (ms_hostcall::types::BufferAllocFunc),
    (access_buffer) => (ms_hostcall::types::AccessBufferFunc),
    (get_time) => (ms_hostcall::types::GetTimeFunc),
}

pub macro hostcall_id {
    (write) => (ms_hostcall::CommonHostCall::Write),
    (open) => (ms_hostcall::CommonHostCall::Open),
    (read) => (ms_hostcall::CommonHostCall::Read),
    (close) => (ms_hostcall::CommonHostCall::Close),
    (connect) => (ms_hostcall::CommonHostCall::Connect),
    (bind) => (ms_hostcall::CommonHostCall::Bind),
    (fatfs_open) => (ms_hostcall::CommonHostCall::FatfsOpen),
    (fatfs_write) => (ms_hostcall::CommonHostCall::FatfsWrite),
    (fatfs_read) => (ms_hostcall::CommonHostCall::FatfsRead),
    (fatfs_close) => (ms_hostcall::CommonHostCall::FatfsClose),
    (stdout) => (ms_hostcall::CommonHostCall::Stdout),
    (addrinfo) => (ms_hostcall::CommonHostCall::SmoltcpAddrInfo),
    (smol_connect) => (ms_hostcall::CommonHostCall::SmoltcpConnect),
    (send) => (ms_hostcall::CommonHostCall::SmoltcpSend),
    (recv) => (ms_hostcall::CommonHostCall::SmoltcpRecv),
    (buffer_alloc) => (ms_hostcall::CommonHostCall::BufferAlloc),
    (access_buffer) => (ms_hostcall::CommonHostCall::AccessBuffer),
    (get_time) => (ms_hostcall::CommonHostCall::GetTime),
}

pub macro libos {
    ($name:ident($($arg_name:expr),*)) => {
        {
            fn binding() -> func_type!($name) {
                let mut table = USER_HOST_CALL.exclusive_access();
                unsafe { core::mem::transmute(table.get_or_find(hostcall_id!($name))) }
            }
            let $name = binding();
            $name($($arg_name),*)
        }
    }
}
