use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut fpout: *mut FILE;
    fn Sfree(_: *mut libc::c_char);
    fn Error(_: *mut libc::c_char);
    fn s_alloc(_: libc::c_int) -> *mut SHORT;
    static mut A_report: libc::c_int;
    fn A_rept(_: A_OBJECT) -> A_OBJECT;
    fn A_rename(_: A_OBJECT, _: *mut SHORT) -> A_OBJECT;
    fn A_trim(_: A_OBJECT) -> A_OBJECT;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct A_row {
    pub A_a: SHORT,
    pub A_b: SHORT,
    pub A_c: SHORT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct A_desc {
    pub Type: libc::c_int,
    pub A_mode: SHORT,
    pub A_ems: SHORT,
    pub A_nT: libc::c_int,
    pub A_nQ: libc::c_int,
    pub A_nS: libc::c_int,
    pub A_nrows: libc::c_int,
    pub A_lrows: libc::c_int,
    pub A_p: *mut *mut A_row,
    pub A_t: *mut A_row,
}
pub type A_OBJECT = *mut A_desc;
static mut GAl: A_OBJECT = 0 as *const A_desc as *mut A_desc;
static mut l_stk: *mut SHORT = 0 as *const SHORT as *mut SHORT;
static mut l_low: *mut SHORT = 0 as *const SHORT as *mut SHORT;
static mut l_cnt: libc::c_int = 0;
static mut l_top: libc::c_int = 0;
static mut l_reopen: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn A_la_DFS(mut state: libc::c_int) -> libc::c_int {
    static mut next: libc::c_int = 0;
    let mut dfn: libc::c_int = 0;
    let mut p: *mut A_row = 0 as *mut A_row;
    let mut pz: *mut A_row = 0 as *mut A_row;
    let fresh0 = l_cnt;
    l_cnt = l_cnt + 1;
    dfn = fresh0;
    *l_low.offset(state as isize) = dfn;
    *l_stk.offset(state as isize) = l_top;
    l_top = state;
    pz = *((*GAl).A_p).offset((state + 1 as libc::c_int) as isize);
    p = *((*GAl).A_p).offset(state as isize);
    while p < pz && (*p).A_b == 0 as libc::c_int {
        next = (*p).A_c;
        if *l_stk.offset(next as isize) == 0o17777777777 as libc::c_int {
            next = A_la_DFS(next);
            if *l_low.offset(next as isize) < *l_low.offset(state as isize) {
                *l_low.offset(state as isize) = *l_low.offset(next as isize);
            }
        } else if *l_stk.offset(next as isize)
            < 0o17777777777 as libc::c_int - 1 as libc::c_int
            && *l_low.offset(next as isize) <= *l_low.offset(state as isize)
        {
            *l_low.offset(state as isize) = *l_low.offset(next as isize);
            l_reopen = 1 as libc::c_int;
        }
        p = p.offset(1);
    }
    if *l_low.offset(state as isize) == dfn {
        next = -(1 as libc::c_int);
        while next != state {
            next = l_top;
            l_top = *l_stk.offset(next as isize);
            *l_stk
                .offset(next as isize) = 0o17777777777 as libc::c_int - 1 as libc::c_int;
            *l_low.offset(next as isize) = dfn;
        }
    }
    return state;
}
#[no_mangle]
pub unsafe extern "C" fn A_lameq(mut A: A_OBJECT) -> A_OBJECT {
    let mut i: libc::c_int = 0;
    if A.is_null() {
        Error(
            b"A_lameq: No OBJECT\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if (*A).A_nQ >= 0o17777777777 as libc::c_int - 1 as libc::c_int {
        Error(
            b"A_lameq: Too many states\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if (*A).A_mode < 2 as libc::c_int {
        A = A_trim(A);
    }
    if (*A).A_mode >= 3 as libc::c_int {
        return A;
    }
    if (*A).A_nrows == 0 as libc::c_int {
        (*A).A_mode = 3 as libc::c_int;
        return A;
    }
    if A_report != 0 {
        fprintf(fpout, b"--> A_lameq\n\0" as *const u8 as *const libc::c_char);
    }
    l_stk = s_alloc((*A).A_nQ);
    l_low = s_alloc((*A).A_nQ);
    l_cnt = 0 as libc::c_int;
    l_top = 0 as libc::c_int;
    l_reopen = 0 as libc::c_int;
    i = (*A).A_nQ;
    loop {
        i -= 1;
        if !(i >= 0 as libc::c_int) {
            break;
        }
        *l_stk.offset(i as isize) = 0o17777777777 as libc::c_int;
    }
    GAl = A;
    i = (*A).A_nQ;
    loop {
        i -= 1;
        if !(i >= 0 as libc::c_int) {
            break;
        }
        if *l_stk.offset(i as isize) == 0o17777777777 as libc::c_int {
            if A_la_DFS(i) != i {
                Error(
                    b"A_lameq: BOTCH\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
        }
    }
    Sfree(l_stk as *mut libc::c_char);
    if l_reopen != 0 {
        A = A_rename(A, l_low);
    }
    Sfree(l_low as *mut libc::c_char);
    (*A).A_mode = 3 as libc::c_int;
    if A_report != 0 {
        fprintf(fpout, b"<-- A_lameq  \0" as *const u8 as *const libc::c_char);
        A_rept(A);
    }
    return A;
}
