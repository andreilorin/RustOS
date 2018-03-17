#![feature(lang_items)]
#![feature(const_fn)]
#![no_std]
#![no_main]

extern crate rlibc;
extern crate volatile;
mod vga_buffer;

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(_msg: core::fmt::Arguments,
    _file: &'static str, _line: u32, _column: u32) -> !
{
    loop {}
}

static HELLO: &[u8] = b"Rust OS";

#[no_mangle]
pub extern fn _start() -> ! {    
   
   let vga_buffer = 0xb8000 as *const u8 as *mut u8;

   for (i, &byte) in HELLO.iter().enumerate() {
	unsafe {
   	    *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
   }
   
   vga_buffer::print_something();

    loop{}
}
