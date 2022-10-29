use ::libc;
#[no_mangle]
pub static mut Version: [libc::c_char; 13] = unsafe {
    *::std::mem::transmute::<&[u8; 13], &mut [libc::c_char; 13]>(b"2.1.1b-RUSTY\0")
};
#[no_mangle]
pub static mut Date: [libc::c_char; 12] = unsafe {
    *::std::mem::transmute::<&[u8; 12], &mut [libc::c_char; 12]>(b"Oct 29 2022\0")
};
