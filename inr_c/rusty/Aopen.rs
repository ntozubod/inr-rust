use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn copymem(_: libc::c_long, _: *mut libc::c_char, _: *mut libc::c_char);
    fn Salloc(_: libc::c_long) -> *mut libc::c_char;
    fn Sfree(_: *mut libc::c_char);
    fn Srealloc(_: *mut libc::c_char, _: libc::c_long) -> *mut libc::c_char;
    fn Ssize(_: *mut libc::c_char) -> libc::c_long;
    fn s_alloc(_: libc::c_int) -> *mut SHORT;
    static mut fpout: *mut FILE;
    fn R_destroy(_: R_OBJECT);
    fn R_insert(_: R_OBJECT, _: libc::c_int, _: libc::c_int) -> libc::c_int;
    static mut A_report: libc::c_int;
    fn R_create() -> R_OBJECT;
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
pub static mut s_rena: *mut SHORT = 0 as *const SHORT as *mut SHORT;
#[no_mangle]
pub static mut f_rena: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn A_add(
    mut A: A_OBJECT,
    mut a: libc::c_int,
    mut b: libc::c_int,
    mut c: libc::c_int,
) -> A_OBJECT {
    let mut p: *mut A_row = 0 as *mut A_row;
    if !A.is_null() {} else {
        __assert_fail(
            b"A != NULL\0" as *const u8 as *const libc::c_char,
            b"Aopen.c\0" as *const u8 as *const libc::c_char,
            35 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 40],
                &[libc::c_char; 40],
            >(b"A_OBJECT A_add(A_OBJECT, int, int, int)\0"))
                .as_ptr(),
        );
    }
    if (*A).A_mode == 0 as libc::c_int {} else {
        __assert_fail(
            b"A-> A_mode == OPEN\0" as *const u8 as *const libc::c_char,
            b"Aopen.c\0" as *const u8 as *const libc::c_char,
            36 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 40],
                &[libc::c_char; 40],
            >(b"A_OBJECT A_add(A_OBJECT, int, int, int)\0"))
                .as_ptr(),
        );
    }
    if a >= 0 as libc::c_int && b >= 0 as libc::c_int && c >= 0 as libc::c_int {} else {
        __assert_fail(
            b"a >= 0 && b >= 0 && c >= 0\0" as *const u8 as *const libc::c_char,
            b"Aopen.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 40],
                &[libc::c_char; 40],
            >(b"A_OBJECT A_add(A_OBJECT, int, int, int)\0"))
                .as_ptr(),
        );
    }
    if a < 0o17777777777 as libc::c_int && b < 0o17777777777 as libc::c_int
        && c < 0o17777777777 as libc::c_int
    {} else {
        __assert_fail(
            b"a < MAXSHORT && b < MAXSHORT && c < MAXSHORT\0" as *const u8
                as *const libc::c_char,
            b"Aopen.c\0" as *const u8 as *const libc::c_char,
            38 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 40],
                &[libc::c_char; 40],
            >(b"A_OBJECT A_add(A_OBJECT, int, int, int)\0"))
                .as_ptr(),
        );
    }
    if (*A).A_nrows >= (*A).A_lrows {
        let ref mut fresh0 = (*A).A_t;
        *fresh0 = Srealloc(
            (*A).A_t as *mut libc::c_char,
            ((2 as libc::c_int * (*A).A_nrows + 2 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<A_row>() as libc::c_ulong)
                as libc::c_long,
        ) as *mut A_row;
        (*A)
            .A_lrows = (Ssize((*A).A_t as *mut libc::c_char) as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<A_row>() as libc::c_ulong)
            .wrapping_sub(2 as libc::c_int as libc::c_ulong) as libc::c_int;
    }
    let ref mut fresh1 = (*A).A_nrows;
    let fresh2 = *fresh1;
    *fresh1 = *fresh1 + 1;
    p = ((*A).A_t).offset(fresh2 as isize);
    (*p).A_a = a;
    (*p).A_b = b;
    (*p).A_c = c;
    if a >= (*A).A_nQ {
        (*A).A_nQ = a + 1 as libc::c_int;
    }
    if b >= (*A).A_nS * (*A).A_nT {
        (*A).A_nS = b / (*A).A_nT + 1 as libc::c_int;
    }
    if c >= (*A).A_nQ {
        (*A).A_nQ = c + 1 as libc::c_int;
    }
    return A;
}
#[no_mangle]
pub unsafe extern "C" fn A_open(mut A: A_OBJECT) -> A_OBJECT {
    if !A.is_null() {} else {
        __assert_fail(
            b"A != NULL\0" as *const u8 as *const libc::c_char,
            b"Aopen.c\0" as *const u8 as *const libc::c_char,
            56 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 26],
                &[libc::c_char; 26],
            >(b"A_OBJECT A_open(A_OBJECT)\0"))
                .as_ptr(),
        );
    }
    (*A).A_mode = 0 as libc::c_int;
    Sfree((*A).A_p as *mut libc::c_char);
    let ref mut fresh3 = (*A).A_p;
    *fresh3 = 0 as *mut *mut A_row;
    return A;
}
#[no_mangle]
pub unsafe extern "C" fn A_close(mut A: A_OBJECT) -> A_OBJECT {
    let mut i: libc::c_int = 0;
    let mut p: *mut A_row = 0 as *mut A_row;
    let mut q: *mut A_row = 0 as *mut A_row;
    let mut t1: *mut A_row = 0 as *mut A_row;
    let mut t2: *mut A_row = 0 as *mut A_row;
    let mut N: libc::c_int = 0;
    let mut NQ: libc::c_int = 0;
    let mut NS: libc::c_int = 0;
    let mut t1z: *mut A_row = 0 as *mut A_row;
    let mut t2z: *mut A_row = 0 as *mut A_row;
    let mut cnt: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ptr: *mut *mut A_row = 0 as *mut *mut A_row;
    if !A.is_null() {} else {
        __assert_fail(
            b"A != NULL\0" as *const u8 as *const libc::c_char,
            b"Aopen.c\0" as *const u8 as *const libc::c_char,
            73 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"A_OBJECT A_close(A_OBJECT)\0"))
                .as_ptr(),
        );
    }
    if (*A).A_mode != 0 as libc::c_int {
        return A;
    }
    if (*A).A_nrows == 0 as libc::c_int {
        (*A).A_mode = 1 as libc::c_int;
        (*A).A_ems = 0 as libc::c_int;
        (*A).A_nT = 1 as libc::c_int;
        (*A).A_nQ = 2 as libc::c_int;
        (*A).A_nS = 2 as libc::c_int;
        let ref mut fresh4 = (*A).A_p;
        *fresh4 = Salloc(
            (3 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut A_row>() as libc::c_ulong)
                as libc::c_long,
        ) as *mut *mut A_row;
        let ref mut fresh5 = *((*A).A_p).offset(2 as libc::c_int as isize);
        *fresh5 = (*A).A_t;
        let ref mut fresh6 = *((*A).A_p).offset(1 as libc::c_int as isize);
        *fresh6 = *fresh5;
        let ref mut fresh7 = *((*A).A_p).offset(0 as libc::c_int as isize);
        *fresh7 = *fresh6;
        return A;
    }
    if A_report != 0 {
        fprintf(fpout, b"--> A_close\n\0" as *const u8 as *const libc::c_char);
    }
    NQ = (*A).A_nQ;
    NS = (*A).A_nS * (*A).A_nT;
    N = if NQ > NS { NQ } else { NS };
    if N > 0 as libc::c_int {} else {
        __assert_fail(
            b"N > 0\0" as *const u8 as *const libc::c_char,
            b"Aopen.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"A_OBJECT A_close(A_OBJECT)\0"))
                .as_ptr(),
        );
    }
    t1 = (*A).A_t;
    t2 = Salloc(
        (((*A).A_nrows + 2 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<A_row>() as libc::c_ulong)
            as libc::c_long,
    ) as *mut A_row;
    t1z = t1.offset((*A).A_nrows as isize);
    t2z = t2.offset((*A).A_nrows as isize);
    cnt = Salloc(
        (N as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            as libc::c_long,
    ) as *mut libc::c_int;
    ptr = Salloc(
        ((N + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut A_row>() as libc::c_ulong)
            as libc::c_long,
    ) as *mut *mut A_row;
    i = N;
    loop {
        i -= 1;
        if !(i >= 0 as libc::c_int) {
            break;
        }
        *cnt.offset(i as isize) = 0 as libc::c_int;
    }
    p = t1z;
    loop {
        p = p.offset(-1);
        if !(p >= t1) {
            break;
        }
        if (*p).A_c < N {} else {
            __assert_fail(
                b"p-> A_c < N\0" as *const u8 as *const libc::c_char,
                b"Aopen.c\0" as *const u8 as *const libc::c_char,
                104 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 27],
                    &[libc::c_char; 27],
                >(b"A_OBJECT A_close(A_OBJECT)\0"))
                    .as_ptr(),
            );
        }
        let ref mut fresh8 = *cnt.offset((*p).A_c as isize);
        *fresh8 += 1;
    }
    p = t2z;
    i = NQ;
    loop {
        i -= 1;
        if !(i >= 0 as libc::c_int) {
            break;
        }
        let ref mut fresh9 = *ptr.offset(i as isize);
        *fresh9 = p;
        p = p.offset(-(*cnt.offset(i as isize) as isize));
        *cnt.offset(i as isize) = 0 as libc::c_int;
    }
    if p == t2 {} else {
        __assert_fail(
            b"p == t2\0" as *const u8 as *const libc::c_char,
            b"Aopen.c\0" as *const u8 as *const libc::c_char,
            115 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"A_OBJECT A_close(A_OBJECT)\0"))
                .as_ptr(),
        );
    }
    p = t1z;
    loop {
        p = p.offset(-1);
        if !(p >= t1) {
            break;
        }
        let ref mut fresh10 = *ptr.offset((*p).A_c as isize);
        *fresh10 = (*fresh10).offset(-1);
        q = *fresh10;
        (*q).A_a = (*p).A_a;
        if (*p).A_b < N {} else {
            __assert_fail(
                b"p-> A_b < N\0" as *const u8 as *const libc::c_char,
                b"Aopen.c\0" as *const u8 as *const libc::c_char,
                119 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 27],
                    &[libc::c_char; 27],
                >(b"A_OBJECT A_close(A_OBJECT)\0"))
                    .as_ptr(),
            );
        }
        let ref mut fresh11 = (*q).A_b;
        *fresh11 = (*p).A_b;
        let ref mut fresh12 = *cnt.offset(*fresh11 as isize);
        *fresh12 += 1;
        (*q).A_c = (*p).A_c;
    }
    p = t1z;
    i = NS;
    loop {
        i -= 1;
        if !(i >= 0 as libc::c_int) {
            break;
        }
        let ref mut fresh13 = *ptr.offset(i as isize);
        *fresh13 = p;
        p = p.offset(-(*cnt.offset(i as isize) as isize));
        *cnt.offset(i as isize) = 0 as libc::c_int;
    }
    if p == t1 {} else {
        __assert_fail(
            b"p == t1\0" as *const u8 as *const libc::c_char,
            b"Aopen.c\0" as *const u8 as *const libc::c_char,
            131 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"A_OBJECT A_close(A_OBJECT)\0"))
                .as_ptr(),
        );
    }
    p = t2z;
    loop {
        p = p.offset(-1);
        if !(p >= t2) {
            break;
        }
        let ref mut fresh14 = *ptr.offset((*p).A_b as isize);
        *fresh14 = (*fresh14).offset(-1);
        q = *fresh14;
        if (*p).A_a < N {} else {
            __assert_fail(
                b"p-> A_a < N\0" as *const u8 as *const libc::c_char,
                b"Aopen.c\0" as *const u8 as *const libc::c_char,
                134 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 27],
                    &[libc::c_char; 27],
                >(b"A_OBJECT A_close(A_OBJECT)\0"))
                    .as_ptr(),
            );
        }
        let ref mut fresh15 = (*q).A_a;
        *fresh15 = (*p).A_a;
        let ref mut fresh16 = *cnt.offset(*fresh15 as isize);
        *fresh16 += 1;
        (*q).A_b = (*p).A_b;
        (*q).A_c = (*p).A_c;
    }
    p = t2z;
    i = NQ;
    loop {
        i -= 1;
        if !(i >= 0 as libc::c_int) {
            break;
        }
        let ref mut fresh17 = *ptr.offset(i as isize);
        *fresh17 = p;
        p = p.offset(-(*cnt.offset(i as isize) as isize));
        *cnt.offset(i as isize) = 0 as libc::c_int;
    }
    if p == t2 {} else {
        __assert_fail(
            b"p == t2\0" as *const u8 as *const libc::c_char,
            b"Aopen.c\0" as *const u8 as *const libc::c_char,
            147 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"A_OBJECT A_close(A_OBJECT)\0"))
                .as_ptr(),
        );
    }
    p = t1z;
    loop {
        p = p.offset(-1);
        if !(p >= t1) {
            break;
        }
        let ref mut fresh18 = *ptr.offset((*p).A_a as isize);
        *fresh18 = (*fresh18).offset(-1);
        q = *fresh18;
        (*q).A_a = (*p).A_a;
        (*q).A_b = (*p).A_b;
        (*q).A_c = (*p).A_c;
    }
    Sfree((*A).A_t as *mut libc::c_char);
    let ref mut fresh19 = (*A).A_t;
    *fresh19 = t2;
    (*t2z).A_a = (*t2z.offset(-(1 as libc::c_int as isize))).A_a;
    (*t2z).A_b = (*t2z.offset(-(1 as libc::c_int as isize))).A_b;
    (*t2z).A_c = (*t2z.offset(-(1 as libc::c_int as isize))).A_c;
    (*t2z.offset(1 as libc::c_int as isize)).A_c = 0o17777777777 as libc::c_int;
    p = t2;
    t1 = t2.offset(-(1 as libc::c_int as isize));
    while t2 < t2z {
        q = t2;
        let ref mut fresh20 = *cnt.offset((*t2).A_a as isize);
        *fresh20 += 1;
        loop {
            t1 = t1.offset(1);
            t2 = t2.offset(1);
            if !((*t1).A_c != (*t2).A_c || (*t1).A_b != (*t2).A_b
                || (*t1).A_a != (*t2).A_a)
            {
                break;
            }
            let ref mut fresh21 = *cnt.offset((*t2).A_a as isize);
            *fresh21 += 1;
        }
        if p < q {
            copymem(
                (t2.offset_from(q) as libc::c_long as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<A_row>() as libc::c_ulong)
                    as libc::c_long,
                q as *mut libc::c_char,
                p as *mut libc::c_char,
            );
        }
        p = p.offset(t2.offset_from(q) as libc::c_long as isize);
        loop {
            t1 = t1.offset(1);
            t2 = t2.offset(1);
            if !((*t1).A_c == (*t2).A_c && (*t1).A_b == (*t2).A_b
                && (*t1).A_a == (*t2).A_a)
            {
                break;
            }
        }
    }
    (*A).A_mode = 1 as libc::c_int;
    (*A).A_nrows = p.offset_from((*A).A_t) as libc::c_long as libc::c_int;
    (*A)
        .A_lrows = (Ssize((*A).A_t as *mut libc::c_char) as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<A_row>() as libc::c_ulong)
        .wrapping_sub(2 as libc::c_int as libc::c_ulong) as libc::c_int;
    let ref mut fresh22 = *ptr.offset((*A).A_nQ as isize);
    *fresh22 = p;
    i = (*A).A_nQ;
    loop {
        i -= 1;
        if !(i >= 0 as libc::c_int) {
            break;
        }
        p = p.offset(-(*cnt.offset(i as isize) as isize));
        let ref mut fresh23 = *ptr.offset(i as isize);
        *fresh23 = p;
    }
    let ref mut fresh24 = (*A).A_p;
    *fresh24 = ptr;
    Sfree(cnt as *mut libc::c_char);
    if A_report != 0 {
        fprintf(fpout, b"<-- A_close\n\0" as *const u8 as *const libc::c_char);
    }
    return A;
}
#[no_mangle]
pub unsafe extern "C" fn A_rename(mut A: A_OBJECT, mut rena: *mut SHORT) -> A_OBJECT {
    let mut p: *mut A_row = 0 as *mut A_row;
    let mut pz: *mut A_row = 0 as *mut A_row;
    let mut trena: *mut SHORT = 0 as *mut SHORT;
    let mut sp: *mut SHORT = 0 as *mut SHORT;
    let mut nrena: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if !A.is_null() {} else {
        __assert_fail(
            b"A != NULL\0" as *const u8 as *const libc::c_char,
            b"Aopen.c\0" as *const u8 as *const libc::c_char,
            203 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"A_OBJECT A_rename(A_OBJECT, SHORT *)\0"))
                .as_ptr(),
        );
    }
    trena = s_alloc((*A).A_nQ);
    sp = trena.offset((*A).A_nQ as isize);
    loop {
        sp = sp.offset(-1);
        if !(sp >= trena) {
            break;
        }
        *sp = 0o17777777777 as libc::c_int;
    }
    pz = ((*A).A_t).offset((*A).A_nrows as isize);
    if !rena.is_null() {
        p = (*A).A_t;
        while p < pz {
            (*p).A_a = *rena.offset((*p).A_a as isize);
            (*p).A_c = *rena.offset((*p).A_c as isize);
            p = p.offset(1);
        }
        *trena
            .offset(*rena.offset(0 as libc::c_int as isize) as isize) = 0 as libc::c_int;
        *trena
            .offset(*rena.offset(1 as libc::c_int as isize) as isize) = 1 as libc::c_int;
    } else {
        *trena.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
        *trena.offset(1 as libc::c_int as isize) = 1 as libc::c_int;
    }
    nrena = 2 as libc::c_int;
    p = (*A).A_t;
    while p < pz {
        sp = trena.offset((*p).A_a as isize);
        if *sp == 0o17777777777 as libc::c_int {
            let fresh25 = nrena;
            nrena = nrena + 1;
            *sp = fresh25;
        }
        (*p).A_a = *sp;
        p = p.offset(1);
    }
    p = pz;
    loop {
        p = p.offset(-1);
        if !(p >= (*A).A_t) {
            break;
        }
        let ref mut fresh26 = (*p).A_c;
        *fresh26 = *trena.offset((*p).A_c as isize);
        if *fresh26 == 0o17777777777 as libc::c_int
            || (*p).A_a == (*p).A_c && (*p).A_b == 0 as libc::c_int
        {
            pz = pz.offset(-1);
            (*p).A_a = (*pz).A_a;
            (*p).A_b = (*pz).A_b;
            (*p).A_c = (*pz).A_c;
        }
    }
    Sfree(s_rena as *mut libc::c_char);
    s_rena = 0 as *mut SHORT;
    if f_rena != 0 {
        if !rena.is_null() {
            s_rena = s_alloc((*A).A_nQ);
            i = (*A).A_nQ;
            loop {
                i -= 1;
                if !(i >= 0 as libc::c_int) {
                    break;
                }
                if *rena.offset(i as isize) < (*A).A_nQ {
                    *s_rena
                        .offset(
                            i as isize,
                        ) = *trena.offset(*rena.offset(i as isize) as isize);
                } else {
                    *s_rena.offset(i as isize) = 0o17777777777 as libc::c_int;
                }
            }
        } else {
            s_rena = trena;
            trena = 0 as *mut SHORT;
        }
    }
    Sfree(trena as *mut libc::c_char);
    (*A).A_nrows = pz.offset_from((*A).A_t) as libc::c_long as libc::c_int;
    (*A).A_nQ = nrena;
    return A_close(A_open(A));
}
#[no_mangle]
pub unsafe extern "C" fn A_mkdense(mut A: A_OBJECT) -> A_OBJECT {
    let mut p: *mut A_row = 0 as *mut A_row;
    let mut pz: *mut A_row = 0 as *mut A_row;
    let mut R: R_OBJECT = 0 as *mut R_desc;
    if !A.is_null() {} else {
        __assert_fail(
            b"A != NULL\0" as *const u8 as *const libc::c_char,
            b"Aopen.c\0" as *const u8 as *const libc::c_char,
            262 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"A_OBJECT A_mkdense(A_OBJECT)\0"))
                .as_ptr(),
        );
    }
    R = R_create();
    R_insert(R, 0 as libc::c_int, 0 as libc::c_int);
    R_insert(R, 1 as libc::c_int, 0 as libc::c_int);
    pz = ((*A).A_t).offset((*A).A_nrows as isize);
    p = (*A).A_t;
    while p < pz {
        (*p).A_a = R_insert(R, (*p).A_a, 0 as libc::c_int);
        (*p).A_c = R_insert(R, (*p).A_c, 0 as libc::c_int);
        p = p.offset(1);
    }
    (*A).A_nQ = (*R).R_n;
    R_destroy(R);
    return A_close(A_open(A));
}
