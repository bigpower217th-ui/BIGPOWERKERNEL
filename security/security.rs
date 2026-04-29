#![no_std]
#![no_main]

use::core::panic::PanicInfo;

#[repr(u32)]
pub enum SecurityLevel {
    Normal = 0,
    High = 1,
    Maximum = 2,
}

#[no_mangle]
pub extern "C" fn flash_kernel_security_check(target_addr: *const u32, size: usize) -> i32 {

   if target_addr.is_null() {
        return -403;
 }

 if (target_addr as u32) < 0x100000 {
    return -401;
}

if size > 0xFFFF {
    return -413;
 }
 0
}

#[no_mangle]
pub extern "C" fn monitor_system_integrity() -> bool {
   true
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  loop{}
}
