use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    static mut fpout: *mut FILE;
    fn Salloc(_: libc::c_long) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type SHORT = libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn Warning(mut s: *mut libc::c_char) {
    fprintf(fpout, b"*** %s ***\n\0" as *const u8 as *const libc::c_char, s);
}
#[no_mangle]
pub unsafe extern "C" fn Error(mut s: *mut libc::c_char) {
    fprintf(fpout, b"*** %s ***\n\0" as *const u8 as *const libc::c_char, s);
    fflush(fpout);
    exit(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn s_alloc(mut n: libc::c_int) -> *mut SHORT {
    return Salloc(
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<SHORT>() as libc::c_ulong)
            as libc::c_long,
    ) as *mut SHORT;
}
#[no_mangle]
pub unsafe extern "C" fn i_alloc(mut n: libc::c_int) -> *mut libc::c_int {
    return Salloc(
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            as libc::c_long,
    ) as *mut libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn pr_time_diff() {}
