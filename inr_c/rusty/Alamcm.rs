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
    fn A_lameq(_: A_OBJECT) -> A_OBJECT;
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
static mut c_rena: *mut SHORT = 0 as *const SHORT as *mut SHORT;
#[no_mangle]
pub unsafe extern "C" fn A_cm_DFS(mut state: libc::c_int) -> libc::c_int {
    if *c_rena.offset(state as isize) == state {
        return state;
    }
    if *c_rena.offset(state as isize) >= 0o17777777777 as libc::c_int - 1 as libc::c_int
    {
        return state;
    }
    let ref mut fresh0 = *c_rena.offset(state as isize);
    *fresh0 = A_cm_DFS(*c_rena.offset(state as isize));
    return *fresh0;
}
#[no_mangle]
pub unsafe extern "C" fn A_lamcm(mut A: A_OBJECT) -> A_OBJECT {
    let mut p: *mut A_row = 0 as *mut A_row;
    let mut pz: *mut A_row = 0 as *mut A_row;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut n_combine: libc::c_int = 0;
    if A.is_null() {
        Error(
            b"A_lamcm: No OBJECT\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if (*A).A_nQ >= 0o17777777777 as libc::c_int - 1 as libc::c_int {
        Error(
            b"A_lamcm: Too many states\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if (*A).A_mode < 3 as libc::c_int {
        A = A_lameq(A);
    }
    if (*A).A_mode >= 4 as libc::c_int {
        return A;
    }
    if A_report != 0 {
        fprintf(fpout, b"--> A_lamcm\n\0" as *const u8 as *const libc::c_char);
    }
    c_rena = s_alloc((*A).A_nQ);
    n_combine = 1 as libc::c_int;
    while n_combine > 0 as libc::c_int {
        i = (*A).A_nQ;
        loop {
            i -= 1;
            if !(i >= 0 as libc::c_int) {
                break;
            }
            *c_rena.offset(i as isize) = 0o17777777777 as libc::c_int;
        }
        n_combine = 0 as libc::c_int;
        i = (*A).A_nQ;
        loop {
            i -= 1;
            if !(i >= 0 as libc::c_int) {
                break;
            }
            p = *((*A).A_p).offset(i as isize);
            if *((*A).A_p).offset((i + 1 as libc::c_int) as isize)
                == p.offset(1 as libc::c_int as isize) && (*p).A_b == 0 as libc::c_int
            {
                *c_rena.offset(i as isize) = (*p).A_c;
                n_combine += 1;
            }
        }
        if n_combine == 0 as libc::c_int {
            *c_rena
                .offset(
                    0 as libc::c_int as isize,
                ) = 0o17777777777 as libc::c_int - 1 as libc::c_int;
            pz = ((*A).A_t).offset((*A).A_nrows as isize);
            p = (*A).A_t;
            while p < pz {
                j = (*p).A_c;
                if *c_rena.offset(j as isize)
                    < 0o17777777777 as libc::c_int - 1 as libc::c_int
                {
                    *c_rena
                        .offset(
                            j as isize,
                        ) = 0o17777777777 as libc::c_int - 1 as libc::c_int;
                    n_combine -= 1;
                } else if (*p).A_b != 0 as libc::c_int {
                    *c_rena
                        .offset(
                            j as isize,
                        ) = 0o17777777777 as libc::c_int - 1 as libc::c_int;
                } else if *c_rena.offset(j as isize) == 0o17777777777 as libc::c_int {
                    *c_rena.offset(j as isize) = (*p).A_a;
                    n_combine += 1;
                }
                p = p.offset(1);
            }
        }
        i = (*A).A_nQ;
        loop {
            i -= 1;
            if !(i >= 0 as libc::c_int) {
                break;
            }
            if *c_rena.offset(i as isize)
                < 0o17777777777 as libc::c_int - 1 as libc::c_int
            {
                *c_rena.offset(i as isize) = A_cm_DFS(*c_rena.offset(i as isize));
            }
        }
        i = (*A).A_nQ;
        loop {
            i -= 1;
            if !(i >= 0 as libc::c_int) {
                break;
            }
            if *c_rena.offset(i as isize)
                >= 0o17777777777 as libc::c_int - 1 as libc::c_int
            {
                *c_rena.offset(i as isize) = i;
            }
        }
        if n_combine > 0 as libc::c_int {
            A = A_rename(A, c_rena);
        }
    }
    Sfree(c_rena as *mut libc::c_char);
    (*A).A_mode = 4 as libc::c_int;
    if A_report != 0 {
        fprintf(fpout, b"<-- A_lamcm  \0" as *const u8 as *const libc::c_char);
        A_rept(A);
    }
    return A;
}
