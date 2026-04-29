#![no_std]
#![no_main]

use::core::panic::PanicInfo;

extern "C" {
        fn ata_write_250gb(lba: u64, count: u16, buffer: *const u16) -> i32;
        }


pub struct SafeDiskManager {
   sector_size: usize,
   disk_limit_lba: u64,
}

impl SafeDiskManager {
  pub const fn new() -> Self {
     Self {
       sector_size: 512,
       disk_limit_lba: 488_397_168,
    }
}

pub fn secure_write(&self, lba: u64, data: &[u16]) -> Result<(), &'static str> {

    if lba > self.disk_limit_lba {
       return Err("Cyber Armor: Disk limit exceeded, permission denied.");
    }

    if data.is_empty() {
       return Err("Error: There is no data to write.");
    }

    let sector_count = (data.len() / 256) as u16;
    let result = unsafe {
      ata_write_250gb(lba, sector_count, data.as_ptr())
    };

   if result == 0 {
     Ok(())
  } else {
      Err("Hardware error: disk write operation failed.")
    }
  }
}

#[panic_handler]
    fn panic(_info: &PanicInfo) -> ! {
       loop{}
   }
