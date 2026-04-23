#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn sys_security_audit(syscall_id: u32) -> i32 {
    if syscall_id == 0x80 {
        return -1;
    }
    0
 }
 
 

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
