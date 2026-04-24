#![no_std]

use::core::panic::PanicInfo;
extern "C" {
        fn kmalloc_ext(size: usize) -> *mut u8;
        }



pub struct SecurityManager {
    total_memory: usize,
    used_memory: usize,
    threat_level: u8,
}

impl SecurityManager {
   pub const fn new(total: usize) -> Self {
      Self { total_memory: total, used_memory: 0, threat_level: 0 }
   }

  pub fn allocate_secure(&mut self, size: usize, align: usize) -> Result<u32, &'static str> {

      if self.used_memory + size > self.total_memory {
           return Err("OUT_OF_MEMORY Cyber ​​armor memory limit has been reached.");
        }

        if size < 16 && self.threat_level > 5 {
            return Err("SECURİTY_ALERT: Heap Spraying suspected attack!");
            }

            let secure_addr = unsafe { kmalloc_ext(size) };
            self.used_memory += size;

            Ok(secure_addr as u32)
         }

         pub fn wipe_memory(&self, addr: u32, size: usize) {
            
             let ptr = addr as *mut u8;
            for i in 0..size {
                unsafe { *ptr.add(i) = 0; }
            }
        }
  }

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
    }
