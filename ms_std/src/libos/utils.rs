use crate::libos::USER_HOST_CALL;

pub macro func_type {
    (write) => (ms_hostcall::types::HostWriteFunc),
    (open) => (ms_hostcall::types::HostOpenFunc),
    (fatfs_open) => (ms_hostcall::types::FatfsOpenFunc),
    (stdout) => (ms_hostcall::types::HostStdioFunc),
    (addrinfo) => (ms_hostcall::types::SmoltcpAddrInfoFunc),
    (connect) => (ms_hostcall::types::SmoltcpConnectFunc),
    (send) => (ms_hostcall::types::SmoltcpSendFunc),
    (recv) => (ms_hostcall::types::SmoltcpRecvFunc),
    (buffer_alloc) => (ms_hostcall::types::BufferAllocFunc),
    (access_buffer) => (ms_hostcall::types::AccessBufferFunc),
    (get_time) => (ms_hostcall::types::GetTimeFunc),
}

pub macro hostcall_id {
    (write) => (ms_hostcall::CommonHostCall::Write),
    (open) => (ms_hostcall::CommonHostCall::Open),
    (fatfs_open) => (ms_hostcall::CommonHostCall::FatfsOpen),
    (stdout) => (ms_hostcall::CommonHostCall::Stdout),
    (addrinfo) => (ms_hostcall::CommonHostCall::SmoltcpAddrInfo),
    (connect) => (ms_hostcall::CommonHostCall::SmoltcpConnect),
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
