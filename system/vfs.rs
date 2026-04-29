#![no_std]
#![allow(dead_code)]
#![allow(unused_variables)]

use crate::filesystem::{Inode, SiberFileSystem};
use core::ptr;
use core::panic::PanicInfo;

// --- ÇEKİRDEK KONFİGÜRASYONU VE LİMİTLER ---
pub const MAX_OPEN_FILES: usize = 512;      // Eşzamanlı dosya limiti
pub const VNODE_TABLE_SIZE: usize = 128;    // Aktif düğüm tablosu
pub const MOUNT_MAX: usize = 32;            // Bağlanabilir disk/bölüm sayısı
pub const PATH_MAX: usize = 4096;           // Maksimum yol uzunluğu (Linux standardı)

// --- DOSYA SİSTEMİ TİPLERİ ---
pub const FSTYPE_NONE: u32  = 0;
pub const FSTYPE_SZFS: u32  = 1;
pub const FSTYPE_FAT32: u32 = 2;
pub const FSTYPE_PROCFS: u32 = 3; // Sistem bilgilerini dosya gibi okumak için

// --- DOSYA ERİŞİM MODLARI VE İZİNLER ---
pub const O_ACCMODE: u32 = 0o003;
pub const O_RDONLY:  u32 = 0o000;
pub const O_WRONLY:  u32 = 0o001;
pub const O_RDWR:    u32 = 0o002;
pub const O_CREAT:   u32 = 0o100;
pub const O_EXCL:    u32 = 0o200;
pub const O_TRUNC:   u32 = 0o1000;
pub const O_APPEND:  u32 = 0o2000;

// --- GÜVENLİK VE YETKİ (ACL) ---
pub const S_IRWXU: u16 = 0o0700; // Kullanıcı tam yetki
pub const S_IRUSR: u16 = 0o0400; // Kullanıcı okuma
pub const S_IWUSR: u16 = 0o0200; // Kullanıcı yazma
pub const S_IXUSR: u16 = 0o0100; // Kullanıcı çalıştırma

// --- VNODE (VIRTUAL NODE) YAPISI ---
// Çekirdeğin belleğinde yaşayan dosya temsilcisi
#[repr(C)]
pub struct VNode {
    pub v_id: u64,
    pub v_type: u16,        // 1: REG, 2: DIR, 3: LNK
    pub v_count: i32,       // Referans sayacı
    pub v_mount: *mut Mount,
    pub v_data: usize,      // Dosya sistemine özel veri (Inode index)
    pub is_locked: bool,
}

// --- MOUNT YAPISI ---
pub struct Mount {
    pub dev_id: u32,
    pub root_vnode: *mut VNode,
    pub fs_type: u32,
}

// --- DOSYA NESNESİ (FILE OBJECT) ---
#[derive(Copy, Clone)]
pub struct File {
    pub f_vnode_idx: usize,
    pub f_pos: u64,
    pub f_flags: u32,
    pub f_count: i32,
    pub f_uid: u16,
}

// --- ANA VFS ÇEKİRDEK YAPISI ---
pub struct VfsManager {
    pub files: [Option<File>; MAX_OPEN_FILES],
    pub vnodes: [VNode; VNODE_TABLE_SIZE],
    pub mounts: [Option<Mount>; MOUNT_MAX],
    pub total_opens: u64,
}

impl VfsManager {
    pub const fn new() -> Self {
        unsafe { core::mem::zeroed() }
    }

    /// Çekirdek Önyükleme (Bootstrap)
    pub fn init(&mut self) {
        // Tüm dosya tablosunu temizle
        for i in 0..MAX_OPEN_FILES {
            self.files[i] = None;
        }
    }

    // --- SİSTEM ÇAĞRILARI (KERNEL API) ---

    /// open(): Dosya sistemine giriş kapısı
    pub fn sys_open(&mut self, fs: &mut SiberFileSystem, path: &str, flags: u32, mode: u16) -> i32 {
        // 1. Dosya var mı kontrol et
        let inode_idx = match fs.find_file(path) {
            Some(idx) => {
                if (flags & O_EXCL) != 0 { return -17; } // EEXIST
                idx
            },
            None => {
                if (flags & O_CREAT) != 0 {
                    fs.create_entry(path, false) as usize
                } else {
                    return -2; // ENOENT
                }
            }
        };

        // 2. Boş bir FD (File Descriptor) ara
        for fd in 0..MAX_OPEN_FILES {
            if self.files[fd].is_none() {
                let mut start_pos = 0;
                if (flags & O_APPEND) != 0 {
                    start_pos = fs.active_inodes[inode_idx].size_bytes;
                }

                self.files[fd] = Some(File {
                    f_vnode_idx: inode_idx,
                    f_pos: start_pos,
                    f_flags: flags,
                    f_count: 1,
                    f_uid: 0, // Root varsayılan
                });
                
                self.total_opens += 1;
                return fd as i32;
            }
        }
        -24 // EMFILE (Çok fazla açık dosya)
    }

    /// read(): Veriyi tampon belleğe çeker
    pub fn sys_read(&mut self, fs: &SiberFileSystem, fd: usize, buf: *mut u16, count: u32) -> i32 {
        if fd >= MAX_OPEN_FILES || self.files[fd].is_none() { return -9; }
        
        let mut file = self.files[fd].as_mut().unwrap();
        let inode = &fs.active_inodes[file.f_vnode_idx];

        if file.f_pos >= inode.size_bytes { return 0; } // EOF

        unsafe {
            let lba = inode.start_block + (file.f_pos / 512);
            let res = crate::drivers::ata::ata_read_250gb(lba, 1, buf);
            if res == 0 {
                file.f_pos += 512;
                return 512;
            }
        }
        -5 // EIO
    }

    /// write(): Siber Zırh mühürleme operasyonu
    pub fn sys_write(&mut self, fs: &mut SiberFileSystem, fd: usize, buf: *const u16) -> i32 {
        if fd >= MAX_OPEN_FILES || self.files[fd].is_none() { return -9; }
        
        let mut file = self.files[fd].as_mut().unwrap();
        if (file.f_flags & O_WRONLY) == 0 && (file.f_flags & O_RDWR) == 0 { return -13; }

        let inode = &fs.active_inodes[file.f_vnode_idx];
        unsafe {
            let lba = inode.start_block + (file.f_pos / 512);
            let res = crate::drivers::ata::ata_write_250gb(lba, 1, buf);
            if res == 0 {
                file.f_pos += 512;
                return 512;
            }
        }
        -5
    }

    /// lseek(): Dosya içinde ışınlanma (Cursor taşıma)
    pub fn sys_lseek(&mut self, fd: usize, offset: i64, whence: i32) -> i64 {
        if fd >= MAX_OPEN_FILES || self.files[fd].is_none() { return -9; }
        
        let mut file = self.files[fd].as_mut().unwrap();
        match whence {
            0 => file.f_pos = offset as u64, // SEEK_SET
            1 => file.f_pos = (file.f_pos as i64 + offset) as u64, // SEEK_CUR
            _ => return -22,
        }
        file.f_pos as i64
    }

    /// sync(): Bekleyen tüm verileri diske zorla basar
    pub fn sys_sync(&self, fs: &SiberFileSystem) {
        for i in 0..fs.active_inodes.len() {
            if fs.active_inodes[i].is_active {
                unsafe {
                    let lba = 20 + i as u64; // Inode Tablosu LBA
                    crate::drivers::ata::ata_write_250gb(lba, 1, &fs.active_inodes[i] as *const _ as *const u16);
                }
            }
        }
    }
}

// --- PANIC HANDLER: ÇEKİRDEK GÜVENLİK SİSTEMİ ---
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Siber Zırh hata tespit ettiğinde donanımı durdurur
    loop {
        unsafe {
            core::arch::asm!("cli", "hlt"); // Kesmeleri kapat ve dur
        }
    }
}
