use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut fpout: *mut FILE;
    fn Salloc(_: libc::c_long) -> *mut libc::c_char;
    fn Sfree(_: *mut libc::c_char);
    fn Scopy(_: *mut libc::c_char) -> *mut libc::c_char;
    fn Ssize(_: *mut libc::c_char) -> libc::c_long;
    fn Error(_: *mut libc::c_char);
    fn A_close(_: A_OBJECT) -> A_OBJECT;
    fn A_open(_: A_OBJECT) -> A_OBJECT;
    fn A_add(_: A_OBJECT, _: libc::c_int, _: libc::c_int, _: libc::c_int) -> A_OBJECT;
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
#[no_mangle]
pub static mut A_report: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn A_create() -> A_OBJECT {
    let mut A: A_OBJECT = 0 as *mut A_desc;
    A = Salloc(::std::mem::size_of::<A_desc>() as libc::c_ulong as libc::c_long)
        as A_OBJECT;
    (*A).Type = 5 as libc::c_int;
    (*A).A_mode = 0 as libc::c_int;
    (*A).A_ems = 0 as libc::c_int;
    (*A).A_nT = 1 as libc::c_int;
    (*A).A_nQ = 2 as libc::c_int;
    (*A).A_nS = 2 as libc::c_int;
    (*A).A_nrows = 0 as libc::c_int;
    let ref mut fresh0 = (*A).A_p;
    *fresh0 = 0 as *mut *mut A_row;
    let ref mut fresh1 = (*A).A_t;
    *fresh1 = Salloc(
        ((20 as libc::c_int + 2 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<A_row>() as libc::c_ulong)
            as libc::c_long,
    ) as *mut A_row;
    (*A)
        .A_lrows = (Ssize((*A).A_t as *mut libc::c_char) as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<A_row>() as libc::c_ulong)
        .wrapping_sub(2 as libc::c_int as libc::c_ulong) as libc::c_int;
    return A;
}
#[no_mangle]
pub unsafe extern "C" fn A_destroy(mut A: A_OBJECT) {
    if !A.is_null() {
        Sfree((*A).A_p as *mut libc::c_char);
        Sfree((*A).A_t as *mut libc::c_char);
    }
    Sfree(A as *mut libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn A_rept(mut A: A_OBJECT) -> A_OBJECT {
    if A.is_null() {
        fprintf(fpout, b"  NULL Automaton\n\0" as *const u8 as *const libc::c_char);
        return A;
    }
    if (*A).A_nrows == 0 as libc::c_int {
        fprintf(fpout, b"  Empty Automaton\n\0" as *const u8 as *const libc::c_char);
        return A;
    }
    if (*A).A_ems != 0 {
        fprintf(fpout, b"*\0" as *const u8 as *const libc::c_char);
    } else {
        fprintf(fpout, b" \0" as *const u8 as *const libc::c_char);
    }
    match (*A).A_mode {
        0 => {
            fprintf(fpout, b"UPDATE  \0" as *const u8 as *const libc::c_char);
        }
        1 => {
            fprintf(fpout, b"NFA     \0" as *const u8 as *const libc::c_char);
        }
        2 => {
            fprintf(fpout, b"TRIM    \0" as *const u8 as *const libc::c_char);
        }
        3 => {
            fprintf(fpout, b"LAM EQ  \0" as *const u8 as *const libc::c_char);
        }
        4 => {
            fprintf(fpout, b"LAM CM  \0" as *const u8 as *const libc::c_char);
        }
        5 => {
            fprintf(fpout, b"CLOSED  \0" as *const u8 as *const libc::c_char);
        }
        6 => {
            fprintf(fpout, b"DFA     \0" as *const u8 as *const libc::c_char);
        }
        7 => {
            fprintf(fpout, b"DFA MIN \0" as *const u8 as *const libc::c_char);
        }
        8 => {
            fprintf(fpout, b"SSEQ    \0" as *const u8 as *const libc::c_char);
        }
        9 => {
            fprintf(fpout, b"SSEQMIN \0" as *const u8 as *const libc::c_char);
        }
        _ => {}
    }
    fprintf(
        fpout,
        b"States: %-6d Trans: %-6d\0" as *const u8 as *const libc::c_char,
        (*A).A_nQ,
        (*A).A_nrows,
    );
    fprintf(fpout, b" Tapes: %-2d\0" as *const u8 as *const libc::c_char, (*A).A_nT);
    fprintf(
        fpout,
        b" Strg: %ld K\0" as *const u8 as *const libc::c_char,
        (Ssize(A as *mut libc::c_char)
            + (if (*A).A_mode == 0 as libc::c_int {
                0 as libc::c_int as libc::c_long
            } else {
                Ssize((*A).A_p as *mut libc::c_char)
            }) + Ssize((*A).A_t as *mut libc::c_char)
            + 1023 as libc::c_int as libc::c_long) / 1024 as libc::c_int as libc::c_long,
    );
    fprintf(fpout, b"\n\0" as *const u8 as *const libc::c_char);
    fflush(fpout);
    return A;
}
#[no_mangle]
pub unsafe extern "C" fn A_exchange(mut A1: A_OBJECT, mut A2: A_OBJECT) {
    let mut t_int: libc::c_int = 0;
    let mut t_Arpp: *mut *mut A_row = 0 as *mut *mut A_row;
    let mut t_Arp: *mut A_row = 0 as *mut A_row;
    if A1.is_null() {
        A1 = A_create();
        (*A1).A_mode = -(1 as libc::c_int);
    }
    if A2.is_null() {
        A2 = A_create();
        (*A2).A_mode = -(1 as libc::c_int);
    }
    t_int = (*A2).A_mode;
    (*A2).A_mode = (*A1).A_mode;
    (*A1).A_mode = t_int;
    t_int = (*A2).A_ems;
    (*A2).A_ems = (*A1).A_ems;
    (*A1).A_ems = t_int;
    t_int = (*A2).A_nT;
    (*A2).A_nT = (*A1).A_nT;
    (*A1).A_nT = t_int;
    t_int = (*A2).A_nQ;
    (*A2).A_nQ = (*A1).A_nQ;
    (*A1).A_nQ = t_int;
    t_int = (*A2).A_nS;
    (*A2).A_nS = (*A1).A_nS;
    (*A1).A_nS = t_int;
    t_int = (*A2).A_nrows;
    (*A2).A_nrows = (*A1).A_nrows;
    (*A1).A_nrows = t_int;
    t_int = (*A2).A_lrows;
    (*A2).A_lrows = (*A1).A_lrows;
    (*A1).A_lrows = t_int;
    t_Arpp = (*A2).A_p;
    let ref mut fresh2 = (*A2).A_p;
    *fresh2 = (*A1).A_p;
    let ref mut fresh3 = (*A1).A_p;
    *fresh3 = t_Arpp;
    t_Arp = (*A2).A_t;
    let ref mut fresh4 = (*A2).A_t;
    *fresh4 = (*A1).A_t;
    let ref mut fresh5 = (*A1).A_t;
    *fresh5 = t_Arp;
    if (*A1).A_mode == -(1 as libc::c_int) {
        A_destroy(A1);
    }
    if (*A2).A_mode == -(1 as libc::c_int) {
        A_destroy(A2);
    }
}
#[no_mangle]
pub unsafe extern "C" fn A_copy(mut A: A_OBJECT) -> A_OBJECT {
    let mut i: libc::c_int = 0;
    let mut newA: A_OBJECT = 0 as *mut A_desc;
    if A.is_null() {
        return 0 as A_OBJECT;
    }
    newA = Scopy(A as *mut libc::c_char) as A_OBJECT;
    let ref mut fresh6 = (*newA).A_p;
    *fresh6 = 0 as *mut *mut A_row;
    let ref mut fresh7 = (*newA).A_t;
    *fresh7 = Scopy((*A).A_t as *mut libc::c_char) as *mut A_row;
    if (*A).A_mode != 0 as libc::c_int {
        let ref mut fresh8 = (*newA).A_p;
        *fresh8 = Salloc(Ssize((*A).A_p as *mut libc::c_char)) as *mut *mut A_row;
        i = 0 as libc::c_int;
        while i <= (*A).A_nQ {
            let ref mut fresh9 = *((*newA).A_p).offset(i as isize);
            *fresh9 = ((*newA).A_t)
                .offset(
                    (*((*A).A_p).offset(i as isize)).offset_from((*A).A_t)
                        as libc::c_long as isize,
                );
            i += 1;
        }
    }
    return newA;
}
#[no_mangle]
pub unsafe extern "C" fn A_deems(mut A: A_OBJECT) -> A_OBJECT {
    let mut new_mode: libc::c_int = 0;
    let mut lst_em: libc::c_int = 0;
    let mut p: *mut A_row = 0 as *mut A_row;
    if (*A).A_ems == 0 {
        return A;
    }
    lst_em = 2 as libc::c_int * (*A).A_nT - 1 as libc::c_int;
    if (*A).A_mode < 3 as libc::c_int {
        new_mode = (*A).A_mode;
    } else {
        new_mode = 3 as libc::c_int;
    }
    A = A_open(A);
    p = ((*A).A_t).offset((*A).A_nrows as isize);
    loop {
        p = p.offset(-1);
        if !(p >= (*A).A_t) {
            break;
        }
        if (*p).A_b > 1 as libc::c_int && (*p).A_b <= lst_em {
            (*p).A_b = 0 as libc::c_int;
        }
    }
    A = A_close(A);
    (*A).A_mode = new_mode;
    (*A).A_ems = 0 as libc::c_int;
    return A;
}
#[no_mangle]
pub unsafe extern "C" fn A_adems(mut A: A_OBJECT) -> A_OBJECT {
    let mut new_mode: libc::c_int = 0;
    let mut fst_em: libc::c_int = 0;
    let mut lst_em: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut base: libc::c_int = 0;
    let mut p: *mut A_row = 0 as *mut A_row;
    if (*A).A_ems != 0 {
        return A;
    }
    if (*A).A_nT == 1 as libc::c_int {
        Error(
            b"Can't add endmarkers to one tape automaton\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    }
    fst_em = (*A).A_nT;
    lst_em = 2 as libc::c_int * (*A).A_nT - 1 as libc::c_int;
    if (*A).A_mode < 7 as libc::c_int {
        new_mode = (*A).A_mode;
    } else {
        new_mode = 7 as libc::c_int;
    }
    A = A_open(A);
    base = (*A).A_nQ;
    p = ((*A).A_t).offset((*A).A_nrows as isize);
    loop {
        p = p.offset(-1);
        if !(p >= (*A).A_t) {
            break;
        }
        if (*p).A_b == 1 as libc::c_int {
            (*p).A_b = fst_em;
            (*p).A_c = base;
        }
    }
    i = fst_em + 1 as libc::c_int;
    while i <= lst_em {
        A = A_add(A, base + i - fst_em - 1 as libc::c_int, i, base + i - fst_em);
        i += 1;
    }
    A = A_add(A, base + lst_em - fst_em, 1 as libc::c_int, 1 as libc::c_int);
    A = A_close(A);
    (*A).A_mode = new_mode;
    (*A).A_ems = 1 as libc::c_int;
    return A;
}
