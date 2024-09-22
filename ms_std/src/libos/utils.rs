use crate::libos::USER_HOST_CALL;

pub macro func_type {
    (metric) => (ms_hostcall::types::MetricFunc),
    (fs_image) => (ms_hostcall::types::FsImageFunc),
    (spawn_fault_handler) => (ms_hostcall::types::SpawnFaultThreadFunc),
    (write) => (ms_hostcall::fdtab::WriteFunc),
    (open) => (ms_hostcall::fdtab::OpenFunc),
    (read) => (ms_hostcall::fdtab::ReadFunc),
    (close) => (ms_hostcall::fdtab::CloseFunc),
    (lseek) => (ms_hostcall::fdtab::LseekFunc),
    (stat) => (ms_hostcall::fdtab::StatFunc),
    (connect) => (ms_hostcall::fdtab::ConnectFunc),
    (bind) => (ms_hostcall::fdtab::BindFunc),
    (accept) => (ms_hostcall::fdtab::AcceptFunc),
    (fatfs_open) => (ms_hostcall::fatfs::FatfsOpenFunc),
    (fatfs_write) => (ms_hostcall::fatfs::FatfsWriteFunc),
    (fatfs_read) => (ms_hostcall::fatfs::FatfsReadFunc),
    (fatfs_close) => (ms_hostcall::fatfs::FatfsCloseFunc),
    (fatfs_seek) => (ms_hostcall::fatfs::FatfsSeekFunc),
    (fatfs_stat) => (ms_hostcall::fatfs::FatfsStatFunc),
    (stdout) => (ms_hostcall::types::HostStdioFunc),
    (addrinfo) => (ms_hostcall::socket::SmoltcpAddrInfoFunc),
    (smol_connect) => (ms_hostcall::socket::SmoltcpConnectFunc),
    (send) => (ms_hostcall::socket::SmoltcpSendFunc),
    (recv) => (ms_hostcall::socket::SmoltcpRecvFunc),
    (smol_bind) => (ms_hostcall::socket::SmoltcpBindFunc),
    (smol_accept) => (ms_hostcall::socket::SmoltcpAcceptFunc),
    (smol_close) => (ms_hostcall::socket::SmoltcpCloseFunc),
    (buffer_alloc) => (ms_hostcall::mm::BufferAllocFunc),
    (access_buffer) => (ms_hostcall::mm::AccessBufferFunc),
    (buffer_dealloc) => (ms_hostcall::mm::BufferDeallocFunc),
    (mmap) => (ms_hostcall::mm::MemmapFunc),
    (munmap) => (ms_hostcall::mm::MemunmapFunc),
    (register_file_backend) => (ms_hostcall::mmap_file_backend::RegisterFileBackendFunc),
    (unregister_file_backend) => (ms_hostcall::mmap_file_backend::UnregisterFileBackendFunc),
    (get_time) => (ms_hostcall::types::GetTimeFunc),
    (nanosleep) => (ms_hostcall::types::NanoSleepFunc),
}

pub macro hostcall_id {
    (metric) => (ms_hostcall::CommonHostCall::Metric),
    (fs_image) => (ms_hostcall::CommonHostCall::FsImage),
    (spawn_fault_handler) => (ms_hostcall::CommonHostCall::SpawnFaultThread),
    (write) => (ms_hostcall::CommonHostCall::Write),
    (open) => (ms_hostcall::CommonHostCall::Open),
    (read) => (ms_hostcall::CommonHostCall::Read),
    (close) => (ms_hostcall::CommonHostCall::Close),
    (lseek) => (ms_hostcall::CommonHostCall::Lseek),
    (stat) => (ms_hostcall::CommonHostCall::Stat),
    (connect) => (ms_hostcall::CommonHostCall::Connect),
    (bind) => (ms_hostcall::CommonHostCall::Bind),
    (accept) => (ms_hostcall::CommonHostCall::Accept),
    (fatfs_open) => (ms_hostcall::CommonHostCall::FatfsOpen),
    (fatfs_write) => (ms_hostcall::CommonHostCall::FatfsWrite),
    (fatfs_read) => (ms_hostcall::CommonHostCall::FatfsRead),
    (fatfs_close) => (ms_hostcall::CommonHostCall::FatfsClose),
    (fatfs_seek) => (ms_hostcall::CommonHostCall::FatfsSeek),
    (fatfs_stat) => (ms_hostcall::CommonHostCall::FatfsStat),
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
    (mmap) => (ms_hostcall::CommonHostCall::Mmap),
    (munmap) => (ms_hostcall::CommonHostCall::Munmap),
    (register_file_backend) => (ms_hostcall::CommonHostCall::RegisterFileBackend),
    (unregister_file_backend) => (ms_hostcall::CommonHostCall::UnregisterFileBackend),
    (get_time) => (ms_hostcall::CommonHostCall::GetTime),
    (nanosleep) => (ms_hostcall::CommonHostCall::NanoSleep),
}

pub macro libos {
    ($name:ident($($arg_name:expr),*)) => {
        {
            fn binding() -> func_type!($name){
                let mut table = USER_HOST_CALL.lock();
                unsafe { core::mem::transmute(table.get_or_find(hostcall_id!($name))) }
            }
            let $name = binding();
            let res = $name($($arg_name),*);
            res
        }
    }
}

#[cfg(feature = "mpk")]
pub macro libos_with_switch_mpk {
    ($name:ident($($arg_name:expr),*)) => {
        {
            use core::arch::asm;
            use crate::mpk;
            let pkru = mpk::pkey_read();
            let is_user_level = (pkru & 0b11 != 0);
            if is_user_level {
                unsafe{
                    asm!(
                        "mov eax, 0x55555550",
                        "xor rcx, rcx",
                        "mov rdx, rcx",
                        "wrpkru"
                    );
                }
            }

            fn binding() -> func_type!($name){
                let mut table = USER_HOST_CALL.lock();
                unsafe { core::mem::transmute(table.get_or_find(hostcall_id!($name))) }
            }
            let $name = binding();    
            let res = $name($($arg_name),*);

            if is_user_level {
                unsafe{
                    asm!(
                        "mov eax, 0x55555553",
                        "xor rcx, rcx",
                        "mov rdx, rcx",
                        "wrpkru"
                    );
                }
            }

            res
        }
    }
}
