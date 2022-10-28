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
    fn A_create() -> A_OBJECT;
    fn A_destroy(_: A_OBJECT);
    fn A_rept(_: A_OBJECT) -> A_OBJECT;
    fn A_add(_: A_OBJECT, _: libc::c_int, _: libc::c_int, _: libc::c_int) -> A_OBJECT;
    fn A_open(_: A_OBJECT) -> A_OBJECT;
    fn A_close(_: A_OBJECT) -> A_OBJECT;
    fn A_lamcm(_: A_OBJECT) -> A_OBJECT;
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
static mut GAc: A_OBJECT = 0 as *const A_desc as *mut A_desc;
static mut GAc2: A_OBJECT = 0 as *const A_desc as *mut A_desc;
static mut c_stk: *mut SHORT = 0 as *const SHORT as *mut SHORT;
static mut c_mark: *mut SHORT = 0 as *const SHORT as *mut SHORT;
static mut c_top: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn A_cl_DFS(mut state: libc::c_int) {
    let mut p: *mut A_row = 0 as *mut A_row;
    let mut pz: *mut A_row = 0 as *mut A_row;
    static mut j: libc::c_int = 0;
    static mut next: libc::c_int = 0;
    static mut lo: *mut A_row = 0 as *const A_row as *mut A_row;
    static mut hi: *mut A_row = 0 as *const A_row as *mut A_row;
    static mut mid: *mut A_row = 0 as *const A_row as *mut A_row;
    pz = *((*GAc).A_p).offset((state + 1 as libc::c_int) as isize);
    p = *((*GAc).A_p).offset(state as isize);
    while p < pz && (*p).A_b == 0 as libc::c_int {
        next = (*p).A_c;
        j = c_top;
        loop {
            j -= 1;
            if !(j >= 0 as libc::c_int) {
                break;
            }
            lo = *((*GAc).A_p).offset(*c_stk.offset(j as isize) as isize);
            hi = (*((*GAc).A_p)
                .offset((*c_stk.offset(j as isize) + 1 as libc::c_int) as isize))
                .offset(-(1 as libc::c_int as isize));
            while lo <= hi {
                mid = lo
                    .offset(
                        (hi.offset_from(lo) as libc::c_long
                            / 2 as libc::c_int as libc::c_long) as isize,
                    );
                if (*mid).A_b != 0 as libc::c_int || (*mid).A_c > next {
                    hi = mid.offset(-(1 as libc::c_int as isize));
                } else {
                    if !((*mid).A_c < next) {
                        break;
                    }
                    lo = mid.offset(1 as libc::c_int as isize);
                }
            }
            if hi < lo {
                GAc2 = A_add(GAc2, *c_stk.offset(j as isize), 0 as libc::c_int, next);
            }
        }
        if *c_mark.offset(state as isize) == 0 as libc::c_int {
            let fresh0 = c_top;
            c_top = c_top + 1;
            *c_stk.offset(fresh0 as isize) = state;
            A_cl_DFS(next);
            c_top -= 1;
        } else {
            A_cl_DFS(next);
        }
        p = p.offset(1);
    }
    if *c_mark.offset(state as isize) == 0 as libc::c_int {
        *c_mark.offset(state as isize) = 1 as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn A_clsure(mut A: A_OBJECT) -> A_OBJECT {
    let mut p: *mut A_row = 0 as *mut A_row;
    let mut pz: *mut A_row = 0 as *mut A_row;
    let mut i: libc::c_int = 0;
    let mut n_condemned: libc::c_int = 0;
    if A.is_null() {
        Error(
            b"A_closure: No OBJECT\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if (*A).A_mode < 4 as libc::c_int {
        A = A_lamcm(A);
    }
    if (*A).A_mode >= 5 as libc::c_int {
        return A;
    }
    if (*A).A_nrows == 0 as libc::c_int {
        (*A).A_mode = 5 as libc::c_int;
        return A;
    }
    if A_report != 0 {
        fprintf(fpout, b"--> A_closure\n\0" as *const u8 as *const libc::c_char);
    }
    c_stk = s_alloc((*A).A_nQ);
    c_mark = s_alloc((*A).A_nQ);
    c_top = 0 as libc::c_int;
    n_condemned = 0 as libc::c_int;
    *c_mark.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    *c_mark.offset(1 as libc::c_int as isize) = 0 as libc::c_int;
    i = (*A).A_nQ;
    loop {
        i -= 1;
        if !(i >= 2 as libc::c_int) {
            break;
        }
        p = *((*A).A_p).offset((i + 1 as libc::c_int) as isize);
        if *((*A).A_p).offset(i as isize) != p
            && (*p.offset(-(1 as libc::c_int as isize))).A_b == 0 as libc::c_int
        {
            *c_mark.offset(i as isize) = 2 as libc::c_int;
            n_condemned += 1;
        } else {
            *c_mark.offset(i as isize) = 0 as libc::c_int;
        }
    }
    if n_condemned > 0 as libc::c_int {
        pz = ((*A).A_t).offset((*A).A_nrows as isize);
        p = (*A).A_t;
        while p < pz {
            if (*p).A_b != 0 as libc::c_int
                && *c_mark.offset((*p).A_c as isize) == 2 as libc::c_int
            {
                *c_mark.offset((*p).A_c as isize) = 0 as libc::c_int;
                n_condemned -= 1;
            }
            p = p.offset(1);
        }
    }
    GAc2 = A_create();
    (*GAc2).A_nT = (*A).A_nT;
    GAc = A;
    i = (*A).A_nQ;
    loop {
        i -= 1;
        if !(i >= 0 as libc::c_int) {
            break;
        }
        if *c_mark.offset(i as isize) == 0 as libc::c_int {
            A_cl_DFS(i);
        }
    }
    Sfree(c_stk as *mut libc::c_char);
    if n_condemned > 0 as libc::c_int {
        pz = ((*A).A_t).offset((*A).A_nrows as isize);
        p = pz;
        loop {
            p = p.offset(-1);
            if !(p > (*A).A_t) {
                break;
            }
            if *c_mark.offset((*p).A_a as isize) == 2 as libc::c_int
                || *c_mark.offset((*p).A_c as isize) == 2 as libc::c_int
            {
                pz = pz.offset(-1);
                (*p).A_a = (*pz).A_a;
                (*p).A_b = (*pz).A_b;
                (*p).A_c = (*pz).A_c;
            }
        }
        (*A).A_nrows = pz.offset_from((*A).A_t) as libc::c_long as libc::c_int;
    }
    Sfree(c_mark as *mut libc::c_char);
    if n_condemned > 0 as libc::c_int || (*GAc2).A_nrows > 0 as libc::c_int {
        A = A_open(A);
        pz = ((*GAc2).A_t).offset((*GAc2).A_nrows as isize);
        p = (*GAc2).A_t;
        while p < pz {
            A = A_add(A, (*p).A_a, 0 as libc::c_int, (*p).A_c);
            p = p.offset(1);
        }
        A = A_close(A);
    }
    A_destroy(GAc2);
    (*A).A_mode = 5 as libc::c_int;
    if A_report != 0 {
        fprintf(fpout, b"<-- A_closure  \0" as *const u8 as *const libc::c_char);
        A_rept(A);
    }
    return A;
}
