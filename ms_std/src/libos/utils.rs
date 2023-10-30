use crate::libos::USER_HOST_CALL;

pub macro func_type {
    (metric) => (ms_hostcall::types::MetricFunc),
    (fs_image) => (ms_hostcall::types::FsImageFunc),
    (write) => (ms_hostcall::types::WriteFunc),
    (open) => (ms_hostcall::types::OpenFunc),
    (read) => (ms_hostcall::types::ReadFunc),
    (close) => (ms_hostcall::types::CloseFunc),
    (connect) => (ms_hostcall::types::ConnectFunc),
    (bind) => (ms_hostcall::types::BindFunc),
    (accept) => (ms_hostcall::types::AcceptFunc),
    (fatfs_open) => (ms_hostcall::types::FatfsOpenFunc),
    (fatfs_write) => (ms_hostcall::types::FatfsWriteFunc),
    (fatfs_read) => (ms_hostcall::types::FatfsReadFunc),
    (fatfs_close) => (ms_hostcall::types::FatfsCloseFunc),
    (stdout) => (ms_hostcall::types::HostStdioFunc),
    (addrinfo) => (ms_hostcall::types::SmoltcpAddrInfoFunc),
    (smol_connect) => (ms_hostcall::types::SmoltcpConnectFunc),
    (send) => (ms_hostcall::types::SmoltcpSendFunc),
    (recv) => (ms_hostcall::types::SmoltcpRecvFunc),
    (smol_bind) => (ms_hostcall::types::SmoltcpBindFunc),
    (smol_accept) => (ms_hostcall::types::SmoltcpAcceptFunc),
    (smol_close) => (ms_hostcall::types::SmoltcpCloseFunc),
    (buffer_alloc) => (ms_hostcall::types::BufferAllocFunc),
    (access_buffer) => (ms_hostcall::types::AccessBufferFunc),
    (buffer_dealloc) => (ms_hostcall::types::BufferDeallocFunc),
    (get_time) => (ms_hostcall::types::GetTimeFunc),
}

pub macro hostcall_id {
    (metric) => (ms_hostcall::CommonHostCall::Metric),
    (fs_image) => (ms_hostcall::CommonHostCall::FsImage),
    (write) => (ms_hostcall::CommonHostCall::Write),
    (open) => (ms_hostcall::CommonHostCall::Open),
    (read) => (ms_hostcall::CommonHostCall::Read),
    (close) => (ms_hostcall::CommonHostCall::Close),
    (connect) => (ms_hostcall::CommonHostCall::Connect),
    (bind) => (ms_hostcall::CommonHostCall::Bind),
    (accept) => (ms_hostcall::CommonHostCall::Accept),
    (fatfs_open) => (ms_hostcall::CommonHostCall::FatfsOpen),
    (fatfs_write) => (ms_hostcall::CommonHostCall::FatfsWrite),
    (fatfs_read) => (ms_hostcall::CommonHostCall::FatfsRead),
    (fatfs_close) => (ms_hostcall::CommonHostCall::FatfsClose),
    (stdout) => (ms_hostcall::CommonHostCall::Stdout),
    (addrinfo) => (ms_hostcall::CommonHostCall::SmoltcpAddrInfo),
    (smol_connect) => (ms_hostcall::CommonHostCall::SmoltcpConnect),
    (send) => (ms_hostcall::CommonHostCall::SmoltcpSend),
    (recv) => (ms_hostcall::CommonHostCall::SmoltcpRecv),
    (smol_bind) => (ms_hostcall::CommonHostCall::SmoltcpBind),
    (smol_accept) => (ms_hostcall::CommonHostCall::SmoltcpAccept),
    (smol_close) => (ms_hostcall::CommonHostCall::SmoltcpClose),
    (buffer_alloc) => (ms_hostcall::CommonHostCall::BufferAlloc),
    (access_buffer) => (ms_hostcall::CommonHostCall::AccessBuffer),
    (buffer_dealloc) => (ms_hostcall::CommonHostCall::BufferDealloc),
    (get_time) => (ms_hostcall::CommonHostCall::GetTime),
}

pub macro libos {
    ($name:ident($($arg_name:expr),*)) => {
        {
            fn binding() -> func_type!($name) {
                let mut table = USER_HOST_CALL.lock();
                unsafe { core::mem::transmute(table.get_or_find(hostcall_id!($name))) }
            }
            let $name = binding();
            $name($($arg_name),*)
        }
    }
}
