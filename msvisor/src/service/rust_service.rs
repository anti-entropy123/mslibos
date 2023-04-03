//! `rust_service` will replace elf_service, because it 
//! will use `dlmopen` or `mmap` to avoid some problems of 
//! `elf_service`.

use std::{fmt::Display, fs::File, io::Read, num::NonZeroUsize, os::unix::prelude::AsRawFd};

use libloading::Symbol;
use ms_hostcall::types::{IsolationID, ServiceName};
use nix::sys::mman::{mmap, MapFlags, ProtFlags};
use xmas_elf::program::{ProgramHeader, Type};

use crate::{round_down, round_up};

pub struct RustService {
    pub name: ServiceName,
}

impl RustService {
    pub fn init(&self, _isol_id: IsolationID) {}

    pub fn run(&self) {}

    pub fn symbol<T>(&self, _symbol: &str) -> Symbol<T> {
        todo!()
    }
}

struct MmapArea {
    base: usize,
    len: usize,
}

impl MmapArea {
    fn _end(&self) -> usize {
        self.base + self.len
    }
}

impl Display for MmapArea {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "MmapArea range: 0x{:x}~0x{:x}",
            self.base,
            self.base + self.len
        ))
    }
}

fn mmap_segment(
    elf_base: usize,
    ph: &ProgramHeader,
    prot: ProtFlags,
    fd: i32,
) -> nix::Result<MmapArea> {
    let base = elf_base + round_down!(ph.virtual_addr() as usize);
    let length = round_up!((ph.virtual_addr() - ph.offset() + ph.mem_size()) as usize);
    let base = unsafe {
        mmap(
            NonZeroUsize::new(base),
            NonZeroUsize::new(length).unwrap(),
            prot,
            MapFlags::MAP_PRIVATE | MapFlags::MAP_DENYWRITE | MapFlags::MAP_FIXED,
            fd,
            round_down!(ph.offset() as usize) as i64,
        )
    }?;
    Ok(MmapArea {
        base: base as usize,
        len: length,
    })
}

/// This is a function used to test load dynlib by mmap. But in current implementation,
/// it still have bugs: free(): invalid pointer.
/// I guess this is because some range will be free twice. 
pub fn load_pic_dynlib() {
    let mut elf_file =
        File::open("/home/yjn/rust_project/mslibos/target/debug/libhello_world.so").unwrap();

    let mut elf_data = Vec::new();
    elf_file
        .read_to_end(&mut elf_data)
        .expect("read elf file failed");

    let elf = { xmas_elf::ElfFile::new(elf_data.as_mut()).unwrap() };
    assert_eq!(
        elf.header.pt1.magic,
        [0x7f, 0x45, 0x4c, 0x46],
        "invalid elf!"
    );
    let (mmap_total, text_idx, data_idx, bss_idx) = {
        let (mut mmap_total_size, mut text_idx, mut data_idx, mut bss_idx) = (0, 0, 0, 0);
        for i in 0..elf.header.pt2.ph_count() {
            let ph = elf.program_header(i).unwrap();
            if ph.get_type() != Ok(Type::Load) {
                continue;
            }
            let flags = ph.flags();
            if flags.is_execute() {
                text_idx = i;
            } else if flags.is_write() {
                bss_idx = i;
            } else {
                data_idx = i;
            }

            mmap_total_size += round_up!(ph.mem_size() as usize);
        }

        let base = unsafe {
            mmap(
                None,
                NonZeroUsize::new(mmap_total_size).unwrap(),
                ProtFlags::PROT_READ,
                MapFlags::MAP_PRIVATE | MapFlags::MAP_DENYWRITE,
                elf_file.as_raw_fd(),
                0,
            )
        }
        .expect("mmap elf file failed");
        (
            MmapArea {
                base: base as usize,
                len: mmap_total_size,
            },
            text_idx,
            data_idx,
            bss_idx,
        )
    };
    log::debug!("mmap_total: {}", mmap_total);

    let mmap_text = mmap_segment(
        mmap_total.base,
        &elf.program_header(text_idx).unwrap(),
        ProtFlags::PROT_READ | ProtFlags::PROT_EXEC,
        elf_file.as_raw_fd(),
    )
    .expect("mmap text segment failed");
    log::debug!("mmap_text: {}", mmap_text);

    let mmap_data = mmap_segment(
        mmap_total.base,
        &elf.program_header(data_idx).unwrap(),
        ProtFlags::PROT_READ,
        elf_file.as_raw_fd(),
    )
    .expect("mmap data segment failed");
    log::debug!("mmap_data: {}", mmap_data);

    let mmap_bss = mmap_segment(
        mmap_total.base,
        &elf.program_header(bss_idx).unwrap(),
        ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
        elf_file.as_raw_fd(),
    )
    .expect("mmap bss segment failed");
    // log::debug!("mmap_bss: {}", mmap_bss)
}

#[test]
fn load_pic_dynlib_test() {
    use crate::logger;
    logger::init();
    load_pic_dynlib()
}
