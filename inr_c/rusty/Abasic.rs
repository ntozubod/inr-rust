use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut fpout: *mut FILE;
    fn Error(_: *mut libc::c_char);
    fn R_create() -> R_OBJECT;
    fn R_destroy(_: R_OBJECT);
    fn R_insert(_: R_OBJECT, _: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn R_rec(_: R_OBJECT, _: libc::c_int) -> *mut R_row;
    fn A_create() -> A_OBJECT;
    fn A_destroy(_: A_OBJECT);
    fn A_deems(_: A_OBJECT) -> A_OBJECT;
    fn A_add(_: A_OBJECT, _: libc::c_int, _: libc::c_int, _: libc::c_int) -> A_OBJECT;
    fn A_open(_: A_OBJECT) -> A_OBJECT;
    fn A_close(_: A_OBJECT) -> A_OBJECT;
    fn A_trim(_: A_OBJECT) -> A_OBJECT;
    fn A_min(_: A_OBJECT) -> A_OBJECT;
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
pub struct R_row {
    pub R_a: SHORT,
    pub R_b: SHORT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct A_row {
    pub A_a: SHORT,
    pub A_b: SHORT,
    pub A_c: SHORT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct R_desc {
    pub Type: libc::c_int,
    pub R_n: libc::c_int,
    pub R_lrec: libc::c_int,
    pub R_lhash: libc::c_int,
    pub R_rec: *mut R_row,
    pub R_hash: *mut SHORT,
}
pub type R_OBJECT = *mut R_desc;
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
pub unsafe extern "C" fn A_conform(mut A1: A_OBJECT, mut A2: A_OBJECT) {
    let mut i: libc::c_int = 0;
    let mut p: *mut A_row = 0 as *mut A_row;
    if (*A1).A_nT < (*A2).A_nT {
        p = ((*A1).A_t).offset((*A1).A_nrows as isize);
        loop {
            p = p.offset(-1);
            if !(p >= (*A1).A_t) {
                break;
            }
            i = (*p).A_b;
            if i > 1 as libc::c_int {
                i = i / (*A1).A_nT * (*A2).A_nT + i % (*A1).A_nT;
            }
            (*p).A_b = i;
        }
        (*A1).A_nT = (*A2).A_nT;
    } else if (*A1).A_nT > (*A2).A_nT {
        p = ((*A2).A_t).offset((*A2).A_nrows as isize);
        loop {
            p = p.offset(-1);
            if !(p >= (*A2).A_t) {
                break;
            }
            i = (*p).A_b;
            if i > 1 as libc::c_int {
                i = i / (*A2).A_nT * (*A1).A_nT + i % (*A2).A_nT;
            }
            (*p).A_b = i;
        }
        (*A2).A_nT = (*A1).A_nT;
    }
}
#[no_mangle]
pub unsafe extern "C" fn A_phi() -> A_OBJECT {
    return A_create();
}
#[no_mangle]
pub unsafe extern "C" fn A_lambda() -> A_OBJECT {
    return A_add(A_create(), 0 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn A_letter(mut t: libc::c_int, mut x: libc::c_int) -> A_OBJECT {
    let mut A: A_OBJECT = 0 as *mut A_desc;
    A = A_create();
    (*A).A_nT = t + 1 as libc::c_int;
    if x == 1 as libc::c_int && t == 0 as libc::c_int {
        (*A).A_nT = 2 as libc::c_int;
    }
    return A_add(
        A_add(A, 0 as libc::c_int, x * (*A).A_nT + t, 2 as libc::c_int),
        2 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn A_deecho(
    mut A: A_OBJECT,
    mut ECHO: libc::c_int,
    mut NOECHO: libc::c_int,
) -> A_OBJECT {
    let mut A1: A_OBJECT = 0 as *mut A_desc;
    let mut p: *mut A_row = 0 as *mut A_row;
    let mut e: libc::c_int = 0;
    let mut base: libc::c_int = 0;
    A = A_min(A);
    A1 = A_create();
    (*A1).A_nT = (*A).A_nT;
    e = (*A).A_nQ;
    base = 2 as libc::c_int * e - 1 as libc::c_int;
    p = ((*A).A_t).offset((*A).A_nrows as isize);
    loop {
        p = p.offset(-1);
        if !(p >= (*A).A_t) {
            break;
        }
        if (*p).A_b == ECHO {
            A1 = A_add(A1, (*p).A_a, 0 as libc::c_int, e + (*p).A_c);
            A1 = A_add(A1, e + (*p).A_a, 0 as libc::c_int, e + (*p).A_c);
        } else if (*p).A_b == NOECHO {
            A1 = A_add(A1, (*p).A_a, 0 as libc::c_int, (*p).A_c);
            A1 = A_add(A1, e + (*p).A_a, 0 as libc::c_int, (*p).A_c);
        } else if (*p).A_b == 1 as libc::c_int {
            A1 = A_add(A1, (*p).A_a, 1 as libc::c_int, (*p).A_c);
            A1 = A_add(A1, e + (*p).A_a, 1 as libc::c_int, (*p).A_c);
        } else if (*p).A_b % (*A).A_nT == 0 as libc::c_int {
            A1 = A_add(A1, (*p).A_a, (*p).A_b, (*p).A_c);
            base += 1;
            A1 = A_add(A1, e + (*p).A_a, (*p).A_b, base);
            A1 = A_add(A1, base, (*p).A_b + 1 as libc::c_int, e + (*p).A_c);
        } else {
            A1 = A_add(A1, (*p).A_a, (*p).A_b, (*p).A_c);
            A1 = A_add(A1, e + (*p).A_a, (*p).A_b, e + (*p).A_c);
        }
    }
    A_destroy(A);
    return A1;
}
#[no_mangle]
pub unsafe extern "C" fn A_opt(mut A: A_OBJECT) -> A_OBJECT {
    let mut new_state: libc::c_int = 0;
    let mut p: *mut A_row = 0 as *mut A_row;
    if (*A).A_ems != 0 {
        A = A_deems(A);
    }
    A = A_open(A);
    new_state = (*A).A_nQ;
    p = ((*A).A_t).offset((*A).A_nrows as isize);
    loop {
        p = p.offset(-1);
        if !(p >= (*A).A_t) {
            break;
        }
        if (*p).A_a == 0 as libc::c_int {
            (*p).A_a = new_state;
        }
        if (*p).A_c == 0 as libc::c_int {
            (*p).A_c = new_state;
        }
    }
    A = A_add(A, 0 as libc::c_int, 0 as libc::c_int, new_state);
    A = A_add(A, 0 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int);
    return A;
}
#[no_mangle]
pub unsafe extern "C" fn A_plus(mut A: A_OBJECT) -> A_OBJECT {
    let mut new_state: libc::c_int = 0;
    let mut p: *mut A_row = 0 as *mut A_row;
    if (*A).A_ems != 0 {
        A = A_deems(A);
    }
    A = A_open(A_trim(A));
    new_state = (*A).A_nQ;
    p = ((*A).A_t).offset((*A).A_nrows as isize);
    loop {
        p = p.offset(-1);
        if !(p >= (*A).A_t) {
            break;
        }
        if (*p).A_b == 1 as libc::c_int {
            (*p).A_b = 0 as libc::c_int;
            (*p).A_c = new_state;
        }
    }
    A = A_add(A, new_state, 0 as libc::c_int, 0 as libc::c_int);
    A = A_add(A, new_state, 1 as libc::c_int, 1 as libc::c_int);
    return A;
}
#[no_mangle]
pub unsafe extern "C" fn A_star(mut A: A_OBJECT) -> A_OBJECT {
    return A_opt(A_plus(A));
}
#[no_mangle]
pub unsafe extern "C" fn A_union(mut A1: A_OBJECT, mut A2: A_OBJECT) -> A_OBJECT {
    let mut base: libc::c_int = 0;
    let mut a: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut p: *mut A_row = 0 as *mut A_row;
    if (*A1).A_ems != 0 && (*A2).A_ems == 0 {
        A1 = A_deems(A1);
    }
    if (*A2).A_ems != 0 && (*A1).A_ems == 0 {
        A2 = A_deems(A2);
    }
    A_conform(A1, A2);
    A1 = A_open(A1);
    base = (*A1).A_nQ;
    p = ((*A1).A_t).offset((*A1).A_nrows as isize);
    loop {
        p = p.offset(-1);
        if !(p >= (*A1).A_t) {
            break;
        }
        if (*p).A_a == 0 as libc::c_int {
            (*p).A_a = base + 1 as libc::c_int;
        }
        if (*p).A_c == 0 as libc::c_int {
            (*p).A_c = base + 1 as libc::c_int;
        }
    }
    p = ((*A2).A_t).offset((*A2).A_nrows as isize);
    loop {
        p = p.offset(-1);
        if !(p >= (*A2).A_t) {
            break;
        }
        a = (*p).A_a;
        if a != 1 as libc::c_int {
            a += base;
        }
        c = (*p).A_c;
        if c != 1 as libc::c_int {
            c += base;
        }
        A1 = A_add(A1, a, (*p).A_b, c);
    }
    A1 = A_add(
        A_add(A1, 0 as libc::c_int, 0 as libc::c_int, base),
        0 as libc::c_int,
        0 as libc::c_int,
        base + 1 as libc::c_int,
    );
    A_destroy(A2);
    return A1;
}
#[no_mangle]
pub unsafe extern "C" fn A_percent(mut A1: A_OBJECT, mut A2: A_OBJECT) -> A_OBJECT {
    let mut base: libc::c_int = 0;
    let mut a: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut p: *mut A_row = 0 as *mut A_row;
    if (*A1).A_ems != 0 {
        A1 = A_deems(A1);
    }
    if (*A2).A_ems != 0 {
        A2 = A_deems(A2);
    }
    A_conform(A1, A2);
    A1 = A_open(A1);
    base = (*A1).A_nQ;
    p = ((*A1).A_t).offset((*A1).A_nrows as isize);
    loop {
        p = p.offset(-1);
        if !(p >= (*A1).A_t) {
            break;
        }
        if (*p).A_a <= 1 as libc::c_int {
            let ref mut fresh0 = (*p).A_a;
            *fresh0 += base;
        }
        if (*p).A_c <= 1 as libc::c_int {
            let ref mut fresh1 = (*p).A_c;
            *fresh1 += base;
        }
        if (*p).A_b == 1 as libc::c_int {
            (*p).A_b = 0 as libc::c_int;
        }
    }
    p = ((*A2).A_t).offset((*A2).A_nrows as isize);
    loop {
        p = p.offset(-1);
        if !(p >= (*A2).A_t) {
            break;
        }
        a = (*p).A_a + base + 2 as libc::c_int;
        b = (*p).A_b;
        c = (*p).A_c + base + 2 as libc::c_int;
        if b == 1 as libc::c_int {
            b = 0 as libc::c_int;
        }
        A1 = A_add(A1, a, b, c);
    }
    A1 = A_add(A1, 0 as libc::c_int, 0 as libc::c_int, base);
    A1 = A_add(A1, 0 as libc::c_int, 0 as libc::c_int, base + 2 as libc::c_int);
    A1 = A_add(A1, base + 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int);
    A1 = A_add(A1, base + 3 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int);
    A1 = A_add(A1, base + 1 as libc::c_int, 0 as libc::c_int, base + 2 as libc::c_int);
    A1 = A_add(A1, base + 3 as libc::c_int, 0 as libc::c_int, base);
    A_destroy(A2);
    return A1;
}
#[no_mangle]
pub unsafe extern "C" fn A_concat(mut A1: A_OBJECT, mut A2: A_OBJECT) -> A_OBJECT {
    let mut base: libc::c_int = 0;
    let mut a: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut p: *mut A_row = 0 as *mut A_row;
    if (*A1).A_ems != 0 {
        A1 = A_deems(A1);
    }
    A_conform(A1, A2);
    A1 = A_open(A1);
    base = (*A1).A_nQ - 1 as libc::c_int;
    p = ((*A1).A_t).offset((*A1).A_nrows as isize);
    loop {
        p = p.offset(-1);
        if !(p >= (*A1).A_t) {
            break;
        }
        if (*p).A_b == 1 as libc::c_int && (*p).A_c == 1 as libc::c_int {
            (*p).A_b = 0 as libc::c_int;
            (*p).A_c = base + 1 as libc::c_int;
        }
    }
    (*A1).A_nQ = base + 2 as libc::c_int;
    p = ((*A2).A_t).offset((*A2).A_nrows as isize);
    loop {
        p = p.offset(-1);
        if !(p >= (*A2).A_t) {
            break;
        }
        a = (*p).A_a;
        if a > 1 as libc::c_int {
            a += base;
        } else if a == 0 as libc::c_int {
            a = base + 1 as libc::c_int;
        }
        c = (*p).A_c;
        if c > 1 as libc::c_int {
            c += base;
        } else if c == 0 as libc::c_int {
            c = base + 1 as libc::c_int;
        }
        A1 = A_add(A1, a, (*p).A_b, c);
    }
    A_destroy(A2);
    return A1;
}
#[no_mangle]
pub unsafe extern "C" fn A_intersect(mut A1: A_OBJECT, mut A2: A_OBJECT) -> A_OBJECT {
    let mut A: A_OBJECT = 0 as *mut A_desc;
    let mut current: libc::c_int = 0;
    let mut p1: *mut A_row = 0 as *mut A_row;
    let mut p1z: *mut A_row = 0 as *mut A_row;
    let mut p2: *mut A_row = 0 as *mut A_row;
    let mut p2z: *mut A_row = 0 as *mut A_row;
    let mut R: R_OBJECT = 0 as *mut R_desc;
    let mut cur_st: *mut R_row = 0 as *mut R_row;
    if (*A1).A_ems != 0 && (*A2).A_ems == 0 {
        A1 = A_deems(A1);
    }
    if (*A2).A_ems != 0 && (*A1).A_ems == 0 {
        A2 = A_deems(A2);
    }
    A_conform(A1, A2);
    A1 = A_min(A1);
    A2 = A_min(A2);
    A = A_create();
    (*A).A_nT = (*A1).A_nT;
    if (*A).A_nT > 1 as libc::c_int {
        fprintf(
            fpout,
            b"Warning: & applied to multi-tape automaton\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    R = R_create();
    if R_insert(R, 0 as libc::c_int, 0 as libc::c_int) != 0 as libc::c_int {
        Error(
            b"A_intersect: BOTCH 1\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if R_insert(R, 1 as libc::c_int, 1 as libc::c_int) != 1 as libc::c_int {
        Error(
            b"A_intersect: BOTCH 2\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    current = 0 as libc::c_int;
    while current < (*R).R_n {
        cur_st = R_rec(R, current);
        p1 = *((*A1).A_p).offset((*cur_st).R_a as isize);
        p1z = *((*A1).A_p).offset(((*cur_st).R_a + 1 as libc::c_int) as isize);
        p2 = *((*A2).A_p).offset((*cur_st).R_b as isize);
        p2z = *((*A2).A_p).offset(((*cur_st).R_b + 1 as libc::c_int) as isize);
        while p1 < p1z && p2 < p2z {
            if (*p1).A_b < (*p2).A_b {
                p1 = p1.offset(1);
            } else if (*p1).A_b > (*p2).A_b {
                p2 = p2.offset(1);
            } else {
                A = A_add(A, current, (*p1).A_b, R_insert(R, (*p1).A_c, (*p2).A_c));
                p1 = p1.offset(1);
                p2 = p2.offset(1);
            }
        }
        current += 1;
    }
    A_destroy(A1);
    A_destroy(A2);
    R_destroy(R);
    A = A_trim(A);
    (*A).A_mode = 6 as libc::c_int;
    return A;
}
#[no_mangle]
pub unsafe extern "C" fn A_differ(mut A1: A_OBJECT, mut A2: A_OBJECT) -> A_OBJECT {
    let mut A: A_OBJECT = 0 as *mut A_desc;
    let mut current: libc::c_int = 0;
    let mut dead: libc::c_int = 0;
    let mut p1: *mut A_row = 0 as *mut A_row;
    let mut p1z: *mut A_row = 0 as *mut A_row;
    let mut p2: *mut A_row = 0 as *mut A_row;
    let mut p2z: *mut A_row = 0 as *mut A_row;
    let mut R: R_OBJECT = 0 as *mut R_desc;
    let mut cur_st: *mut R_row = 0 as *mut R_row;
    if (*A1).A_ems != 0 && (*A2).A_ems == 0 {
        A1 = A_deems(A1);
    }
    if (*A2).A_ems != 0 && (*A1).A_ems == 0 {
        A2 = A_deems(A2);
    }
    A_conform(A1, A2);
    A1 = A_min(A1);
    A2 = A_min(A2);
    A = A_create();
    (*A).A_nT = (*A1).A_nT;
    if (*A).A_nT > 1 as libc::c_int {
        fprintf(
            fpout,
            b"Warning: - applied to multi-tape automaton\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    R = R_create();
    dead = (*A2).A_nQ;
    if R_insert(R, 0 as libc::c_int, 0 as libc::c_int) != 0 as libc::c_int {
        Error(
            b"A_differ: BOTCH 1\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if R_insert(R, 1 as libc::c_int, dead) != 1 as libc::c_int {
        Error(
            b"A_differ: BOTCH 2\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    current = 0 as libc::c_int;
    while current < (*R).R_n {
        cur_st = R_rec(R, current);
        p1 = *((*A1).A_p).offset((*cur_st).R_a as isize);
        p1z = *((*A1).A_p).offset(((*cur_st).R_a + 1 as libc::c_int) as isize);
        if (*cur_st).R_b != dead {
            p2 = *((*A2).A_p).offset((*cur_st).R_b as isize);
            p2z = *((*A2).A_p).offset(((*cur_st).R_b + 1 as libc::c_int) as isize);
            while p1 < p1z {
                if p2 == p2z || (*p1).A_b < (*p2).A_b {
                    A = A_add(A, current, (*p1).A_b, R_insert(R, (*p1).A_c, dead));
                    p1 = p1.offset(1);
                } else if (*p2).A_b < (*p1).A_b {
                    p2 = p2.offset(1);
                } else {
                    A = A_add(A, current, (*p1).A_b, R_insert(R, (*p1).A_c, (*p2).A_c));
                    p1 = p1.offset(1);
                    p2 = p2.offset(1);
                }
            }
        } else {
            while p1 < p1z {
                A = A_add(A, current, (*p1).A_b, R_insert(R, (*p1).A_c, dead));
                p1 = p1.offset(1);
            }
        }
        current += 1;
    }
    A_destroy(A1);
    A_destroy(A2);
    R_destroy(R);
    A = A_trim(A);
    (*A).A_mode = 6 as libc::c_int;
    return A;
}
#[no_mangle]
pub unsafe extern "C" fn A_xor(mut A1: A_OBJECT, mut A2: A_OBJECT) -> A_OBJECT {
    let mut A: A_OBJECT = 0 as *mut A_desc;
    let mut current: libc::c_int = 0;
    let mut dead: libc::c_int = 0;
    let mut p1: *mut A_row = 0 as *mut A_row;
    let mut p1z: *mut A_row = 0 as *mut A_row;
    let mut p2: *mut A_row = 0 as *mut A_row;
    let mut p2z: *mut A_row = 0 as *mut A_row;
    let mut R: R_OBJECT = 0 as *mut R_desc;
    let mut cur_st: *mut R_row = 0 as *mut R_row;
    if (*A1).A_ems != 0 && (*A2).A_ems == 0 {
        A1 = A_deems(A1);
    }
    if (*A2).A_ems != 0 && (*A1).A_ems == 0 {
        A2 = A_deems(A2);
    }
    A_conform(A1, A2);
    A1 = A_min(A1);
    A2 = A_min(A2);
    A = A_create();
    (*A).A_nT = (*A1).A_nT;
    R = R_create();
    dead = (*A1).A_nQ;
    if dead < (*A2).A_nQ {
        dead = (*A2).A_nQ;
    }
    if R_insert(R, 0 as libc::c_int, 0 as libc::c_int) != 0 as libc::c_int {
        Error(
            b"A_xor: BOTCH 1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if R_insert(R, dead, dead) != 1 as libc::c_int {
        Error(
            b"A_xor: BOTCH 2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    current = 0 as libc::c_int;
    while current < (*R).R_n {
        cur_st = R_rec(R, current);
        if (*cur_st).R_a != dead {
            p1 = *((*A1).A_p).offset((*cur_st).R_a as isize);
            p1z = *((*A1).A_p).offset(((*cur_st).R_a + 1 as libc::c_int) as isize);
            if (*cur_st).R_b != dead {
                p2 = *((*A2).A_p).offset((*cur_st).R_b as isize);
                p2z = *((*A2).A_p).offset(((*cur_st).R_b + 1 as libc::c_int) as isize);
                while p1 < p1z || p2 < p2z {
                    if p2 == p2z || p1 < p1z && (*p1).A_b < (*p2).A_b {
                        A = A_add(
                            A,
                            current,
                            (*p1).A_b,
                            R_insert(
                                R,
                                if (*p1).A_c == 1 as libc::c_int {
                                    dead
                                } else {
                                    (*p1).A_c
                                },
                                dead,
                            ),
                        );
                        p1 = p1.offset(1);
                    } else if p1 == p1z || p2 < p2z && (*p2).A_b < (*p1).A_b {
                        A = A_add(
                            A,
                            current,
                            (*p2).A_b,
                            R_insert(
                                R,
                                dead,
                                if (*p2).A_c == 1 as libc::c_int { dead } else { (*p2).A_c },
                            ),
                        );
                        p2 = p2.offset(1);
                    } else {
                        A = A_add(
                            A,
                            current,
                            (*p1).A_b,
                            R_insert(R, (*p1).A_c, (*p2).A_c),
                        );
                        p1 = p1.offset(1);
                        p2 = p2.offset(1);
                    }
                }
            } else {
                while p1 < p1z {
                    A = A_add(
                        A,
                        current,
                        (*p1).A_b,
                        R_insert(
                            R,
                            if (*p1).A_c == 1 as libc::c_int { dead } else { (*p1).A_c },
                            dead,
                        ),
                    );
                    p1 = p1.offset(1);
                }
            }
        } else if (*cur_st).R_b != dead {
            p2 = *((*A2).A_p).offset((*cur_st).R_b as isize);
            p2z = *((*A2).A_p).offset(((*cur_st).R_b + 1 as libc::c_int) as isize);
            while p2 < p2z {
                A = A_add(
                    A,
                    current,
                    (*p2).A_b,
                    R_insert(
                        R,
                        dead,
                        if (*p2).A_c == 1 as libc::c_int { dead } else { (*p2).A_c },
                    ),
                );
                p2 = p2.offset(1);
            }
        }
        current += 1;
    }
    A_destroy(A1);
    A_destroy(A2);
    R_destroy(R);
    A = A_trim(A);
    (*A).A_mode = 6 as libc::c_int;
    return A;
}
#[no_mangle]
pub unsafe extern "C" fn A_alph(mut A: A_OBJECT) -> A_OBJECT {
    let mut p: *mut A_row = 0 as *mut A_row;
    if (*A).A_ems != 0 {
        A = A_deems(A);
    }
    A = A_open(A_trim(A));
    p = ((*A).A_t).offset((*A).A_nrows as isize);
    loop {
        p = p.offset(-1);
        if !(p >= (*A).A_t) {
            break;
        }
        if (*p).A_b == 1 as libc::c_int {
            (*p).A_a = 2 as libc::c_int;
            (*p).A_c = 1 as libc::c_int;
        } else if (*p).A_b == 0 as libc::c_int {
            (*p).A_a = 0 as libc::c_int;
            (*p).A_c = 3 as libc::c_int;
        } else {
            (*p).A_a = 0 as libc::c_int;
            (*p).A_c = 2 as libc::c_int;
            let ref mut fresh2 = (*p).A_b;
            *fresh2 /= (*A).A_nT;
        }
    }
    (*A).A_nT = 1 as libc::c_int;
    (*A).A_nQ = 4 as libc::c_int;
    return A_trim(A);
}
#[no_mangle]
pub unsafe extern "C" fn A_rev(mut A: A_OBJECT) -> A_OBJECT {
    let mut p: *mut A_row = 0 as *mut A_row;
    let mut tmp: libc::c_int = 0;
    let mut new_state: libc::c_int = 0;
    let mut old_mode: libc::c_int = 0;
    if (*A).A_ems != 0 {
        A = A_deems(A);
    }
    old_mode = (*A).A_mode;
    A = A_open(A);
    new_state = (*A).A_nQ;
    p = ((*A).A_t).offset((*A).A_nrows as isize);
    loop {
        p = p.offset(-1);
        if !(p >= (*A).A_t) {
            break;
        }
        tmp = (*p).A_a;
        (*p).A_a = (*p).A_c;
        (*p).A_c = tmp;
        if (*p).A_b == 1 as libc::c_int {
            (*p).A_b = 0 as libc::c_int;
        }
        if (*p).A_a == 0 as libc::c_int {
            (*p).A_a = new_state;
        }
        if (*p).A_a == 1 as libc::c_int {
            (*p).A_a = 0 as libc::c_int;
        }
        if (*p).A_c == 0 as libc::c_int {
            (*p).A_c = new_state;
        }
        if (*p).A_c == 1 as libc::c_int {
            (*p).A_c = 0 as libc::c_int;
        }
    }
    A = A_add(A, new_state, 1 as libc::c_int, 1 as libc::c_int);
    if old_mode >= 5 as libc::c_int {
        A = A_close(A);
        (*A).A_mode = 5 as libc::c_int;
    }
    return A;
}
#[no_mangle]
pub unsafe extern "C" fn A_shuffle(mut A1: A_OBJECT, mut A2: A_OBJECT) -> A_OBJECT {
    let mut A: A_OBJECT = 0 as *mut A_desc;
    let mut current: libc::c_int = 0;
    let mut cur_a: libc::c_int = 0;
    let mut cur_b: libc::c_int = 0;
    let mut p1: *mut A_row = 0 as *mut A_row;
    let mut p1z: *mut A_row = 0 as *mut A_row;
    let mut p2: *mut A_row = 0 as *mut A_row;
    let mut p2z: *mut A_row = 0 as *mut A_row;
    let mut R: R_OBJECT = 0 as *mut R_desc;
    let mut cur_st: *mut R_row = 0 as *mut R_row;
    if (*A1).A_ems != 0 && (*A2).A_ems == 0 {
        A1 = A_deems(A1);
    }
    if (*A2).A_ems != 0 && (*A1).A_ems == 0 {
        A2 = A_deems(A2);
    }
    A_conform(A1, A2);
    A1 = A_min(A1);
    A2 = A_min(A2);
    A = A_create();
    (*A).A_nT = (*A1).A_nT;
    R = R_create();
    if R_insert(R, 0 as libc::c_int, 0 as libc::c_int) != 0 as libc::c_int {
        Error(
            b"A_shuffle: BOTCH 1\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if R_insert(R, 1 as libc::c_int, 1 as libc::c_int) != 1 as libc::c_int {
        Error(
            b"A_shuffle: BOTCH 2\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    current = 0 as libc::c_int;
    while current < (*R).R_n {
        cur_st = R_rec(R, current);
        cur_a = (*cur_st).R_a;
        cur_b = (*cur_st).R_b;
        p1 = *((*A1).A_p).offset(cur_a as isize);
        p1z = *((*A1).A_p).offset((cur_a + 1 as libc::c_int) as isize);
        p2 = *((*A2).A_p).offset(cur_b as isize);
        p2z = *((*A2).A_p).offset((cur_b + 1 as libc::c_int) as isize);
        if p1 < p1z && p2 < p2z && (*p1).A_b == 1 as libc::c_int
            && (*p2).A_b == 1 as libc::c_int
        {
            A = A_add(A, current, 1 as libc::c_int, 1 as libc::c_int);
        }
        if p1 < p1z && (*p1).A_b == 1 as libc::c_int {
            p1 = p1.offset(1);
        }
        if p2 < p2z && (*p2).A_b == 1 as libc::c_int {
            p2 = p2.offset(1);
        }
        while p1 < p1z {
            A = A_add(A, current, (*p1).A_b, R_insert(R, (*p1).A_c, cur_b));
            p1 = p1.offset(1);
        }
        while p2 < p2z {
            A = A_add(A, current, (*p2).A_b, R_insert(R, cur_a, (*p2).A_c));
            p2 = p2.offset(1);
        }
        current += 1;
    }
    A_destroy(A1);
    A_destroy(A2);
    R_destroy(R);
    A = A_close(A);
    (*A).A_mode = 5 as libc::c_int;
    return A;
}
