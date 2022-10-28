use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut fpout: *mut FILE;
    fn Salloc(_: libc::c_long) -> *mut libc::c_char;
    fn Sfree(_: *mut libc::c_char);
    fn Srealloc(_: *mut libc::c_char, _: libc::c_long) -> *mut libc::c_char;
    fn Ssize(_: *mut libc::c_char) -> libc::c_long;
    fn Error(_: *mut libc::c_char);
    fn s_alloc(_: libc::c_int) -> *mut SHORT;
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
pub struct U_desc {
    pub Type: libc::c_int,
    pub U_n: libc::c_int,
    pub U_lrec: libc::c_int,
    pub U_lhash: libc::c_int,
    pub U_rec: *mut A_row,
    pub U_hash: *mut SHORT,
}
pub type U_OBJECT = *mut U_desc;
static mut U_hashpos: *mut SHORT = 0 as *const SHORT as *mut SHORT;
static mut U_calls: libc::c_int = 0 as libc::c_int;
static mut U_probes: libc::c_int = 0 as libc::c_int;
static mut U_fail: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn U_create() -> U_OBJECT {
    let mut U: U_OBJECT = 0 as *mut U_desc;
    U = Salloc(::std::mem::size_of::<U_desc>() as libc::c_ulong as libc::c_long)
        as U_OBJECT;
    (*U).Type = 4 as libc::c_int;
    (*U).U_n = 0 as libc::c_int;
    (*U).U_lrec = 0 as libc::c_int;
    (*U).U_lhash = 1 as libc::c_int;
    let ref mut fresh0 = (*U).U_rec;
    *fresh0 = 0 as *mut A_row;
    let ref mut fresh1 = (*U).U_hash;
    *fresh1 = s_alloc(1 as libc::c_int);
    *((*U).U_hash).offset(0 as libc::c_int as isize) = 0o17777777777 as libc::c_int;
    return U;
}
#[no_mangle]
pub unsafe extern "C" fn U_destroy(mut U: U_OBJECT) {
    if U.is_null() {
        return;
    }
    Sfree((*U).U_rec as *mut libc::c_char);
    Sfree((*U).U_hash as *mut libc::c_char);
    Sfree(U as *mut libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn U_member(
    mut U: U_OBJECT,
    mut reca: libc::c_int,
    mut recb: libc::c_int,
    mut recc: libc::c_int,
) -> libc::c_int {
    let mut p: *mut SHORT = 0 as *mut SHORT;
    U_calls += 1;
    p = ((*U).U_hash)
        .offset(
            (((16807 as libc::c_int as libc::c_uint)
                .wrapping_mul(
                    (16807 as libc::c_int as libc::c_uint)
                        .wrapping_mul(reca as libc::c_uint)
                        .wrapping_add(recb as libc::c_uint)
                        & 0o17777777777 as libc::c_int as libc::c_uint,
                )
                .wrapping_add(recc as libc::c_uint)
                & 0o17777777777 as libc::c_int as libc::c_uint)
                .wrapping_mul(16807 as libc::c_int as libc::c_uint)
                & 0o17777777777 as libc::c_int as libc::c_uint)
                .wrapping_rem((*U).U_lhash as libc::c_uint) as isize,
        );
    while *p < 0o17777777777 as libc::c_int {
        U_probes += 1;
        if (*((*U).U_rec).offset(*p as isize)).A_a == reca
            && (*((*U).U_rec).offset(*p as isize)).A_b == recb
            && (*((*U).U_rec).offset(*p as isize)).A_c == recc
        {
            return *p;
        }
        p = p.offset(-1);
        if p < (*U).U_hash {
            p = ((*U).U_hash)
                .offset((*U).U_lhash as isize)
                .offset(-(1 as libc::c_int as isize));
        }
    }
    U_fail += 1;
    U_hashpos = p;
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn U_grow(mut U: U_OBJECT, mut lrec: libc::c_int) -> U_OBJECT {
    let mut p: *mut SHORT = 0 as *mut SHORT;
    let mut pl: *mut SHORT = 0 as *mut SHORT;
    let mut q: *mut A_row = 0 as *mut A_row;
    let mut ql: *mut A_row = 0 as *mut A_row;
    let mut i: libc::c_int = 0;
    if lrec < 15 as libc::c_int {
        lrec = 15 as libc::c_int;
    }
    if lrec <= (*U).U_lrec {
        return U;
    }
    Sfree((*U).U_hash as *mut libc::c_char);
    let ref mut fresh2 = (*U).U_rec;
    *fresh2 = Srealloc(
        (*U).U_rec as *mut libc::c_char,
        (lrec as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<A_row>() as libc::c_ulong)
            as libc::c_long,
    ) as *mut A_row;
    (*U)
        .U_lrec = (Ssize((*U).U_rec as *mut libc::c_char) as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<A_row>() as libc::c_ulong) as libc::c_int;
    if (*U).U_lrec > 0o17777777777 as libc::c_int {
        (*U).U_lrec = 0o17777777777 as libc::c_int;
    }
    let ref mut fresh3 = (*U).U_hash;
    *fresh3 = s_alloc(2 as libc::c_int * (*U).U_lrec);
    (*U)
        .U_lhash = (Ssize((*U).U_hash as *mut libc::c_char) as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<SHORT>() as libc::c_ulong) as libc::c_int;
    p = (*U).U_hash;
    pl = p.offset((*U).U_lhash as isize);
    while p < pl {
        let fresh4 = p;
        p = p.offset(1);
        *fresh4 = 0o17777777777 as libc::c_int;
    }
    q = (*U).U_rec;
    ql = q.offset((*U).U_n as isize);
    i = 0 as libc::c_int;
    while q < ql {
        if U_member(U, (*q).A_a, (*q).A_b, (*q).A_c) != -(1 as libc::c_int) {
            Error(
                b"U_grow: BOTCH\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        q = q.offset(1);
        let fresh5 = i;
        i = i + 1;
        *U_hashpos = fresh5;
    }
    return U;
}
#[no_mangle]
pub unsafe extern "C" fn U_insert(
    mut U: U_OBJECT,
    mut reca: libc::c_int,
    mut recb: libc::c_int,
    mut recc: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if (*U).U_n >= (*U).U_lrec {
        if (*U).U_n >= 0o17777777777 as libc::c_int {
            Error(
                b"U_insert: Table FULL\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        U = U_grow(U, 2 as libc::c_int * (*U).U_lrec);
    }
    i = U_member(U, reca, recb, recc);
    if i >= 0 as libc::c_int {
        return i;
    }
    (*((*U).U_rec).offset((*U).U_n as isize)).A_a = reca;
    (*((*U).U_rec).offset((*U).U_n as isize)).A_b = recb;
    (*((*U).U_rec).offset((*U).U_n as isize)).A_c = recc;
    let ref mut fresh6 = (*U).U_n;
    let fresh7 = *fresh6;
    *fresh6 = *fresh6 + 1;
    *U_hashpos = fresh7;
    return *U_hashpos;
}
#[no_mangle]
pub unsafe extern "C" fn U_rec(mut U: U_OBJECT, mut i: libc::c_int) -> *mut A_row {
    if i >= 0 as libc::c_int && i < (*U).U_n {
        return ((*U).U_rec).offset(i as isize)
    } else {
        return 0 as *mut A_row
    };
}
#[no_mangle]
pub unsafe extern "C" fn U_stats() {
    fprintf(
        fpout,
        b"(U) Calls:%7d  Probes:%7d  Unsuccessful:%7d\n\0" as *const u8
            as *const libc::c_char,
        U_calls,
        U_probes,
        U_fail,
    );
}
