#![no_std]
#![no_main]

use core::panic::PanicInfo;


extern "C" {
   fn kprint(message: *const u8);
}

#[no_mangle]
pub extern "C" fn init_scheduler() {
    let msg = b"Rust timer is active.!\0";
    unsafe {
      kprint(msg.as_ptr());
   }
}


#[no_mangle]
pub extern "C" fn schedule_next() {
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
   loop {}
}
