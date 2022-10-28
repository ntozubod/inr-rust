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
pub struct R_row {
    pub R_a: SHORT,
    pub R_b: SHORT,
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
static mut R_hashpos: *mut SHORT = 0 as *const SHORT as *mut SHORT;
static mut R_calls: libc::c_int = 0 as libc::c_int;
static mut R_probes: libc::c_int = 0 as libc::c_int;
static mut R_fail: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn R_create() -> R_OBJECT {
    let mut R: R_OBJECT = 0 as *mut R_desc;
    R = Salloc(::std::mem::size_of::<R_desc>() as libc::c_ulong as libc::c_long)
        as R_OBJECT;
    (*R).Type = 3 as libc::c_int;
    (*R).R_n = 0 as libc::c_int;
    (*R).R_lrec = 0 as libc::c_int;
    (*R).R_lhash = 1 as libc::c_int;
    let ref mut fresh0 = (*R).R_rec;
    *fresh0 = 0 as *mut R_row;
    let ref mut fresh1 = (*R).R_hash;
    *fresh1 = s_alloc(1 as libc::c_int);
    *((*R).R_hash).offset(0 as libc::c_int as isize) = 0o17777777777 as libc::c_int;
    return R;
}
#[no_mangle]
pub unsafe extern "C" fn R_destroy(mut R: R_OBJECT) {
    if R.is_null() {
        return;
    }
    Sfree((*R).R_rec as *mut libc::c_char);
    Sfree((*R).R_hash as *mut libc::c_char);
    Sfree(R as *mut libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn R_member(
    mut R: R_OBJECT,
    mut reca: libc::c_int,
    mut recb: libc::c_int,
) -> libc::c_int {
    let mut p: *mut SHORT = 0 as *mut SHORT;
    R_calls += 1;
    p = ((*R).R_hash)
        .offset(
            (((16807 as libc::c_int as libc::c_uint)
                .wrapping_mul(reca as libc::c_uint)
                .wrapping_add(recb as libc::c_uint)
                & 0o17777777777 as libc::c_int as libc::c_uint)
                .wrapping_mul(16807 as libc::c_int as libc::c_uint)
                & 0o17777777777 as libc::c_int as libc::c_uint)
                .wrapping_rem((*R).R_lhash as libc::c_uint) as isize,
        );
    while *p < 0o17777777777 as libc::c_int {
        R_probes += 1;
        if (*((*R).R_rec).offset(*p as isize)).R_a == reca
            && (*((*R).R_rec).offset(*p as isize)).R_b == recb
        {
            return *p;
        }
        p = p.offset(-1);
        if p < (*R).R_hash {
            p = ((*R).R_hash)
                .offset((*R).R_lhash as isize)
                .offset(-(1 as libc::c_int as isize));
        }
    }
    R_fail += 1;
    R_hashpos = p;
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn R_grow(mut R: R_OBJECT, mut lrec: libc::c_int) -> R_OBJECT {
    let mut p: *mut SHORT = 0 as *mut SHORT;
    let mut pl: *mut SHORT = 0 as *mut SHORT;
    let mut q: *mut R_row = 0 as *mut R_row;
    let mut ql: *mut R_row = 0 as *mut R_row;
    let mut i: libc::c_int = 0;
    if lrec < 15 as libc::c_int {
        lrec = 15 as libc::c_int;
    }
    if lrec <= (*R).R_lrec {
        return R;
    }
    Sfree((*R).R_hash as *mut libc::c_char);
    let ref mut fresh2 = (*R).R_rec;
    *fresh2 = Srealloc(
        (*R).R_rec as *mut libc::c_char,
        (lrec as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<R_row>() as libc::c_ulong)
            as libc::c_long,
    ) as *mut R_row;
    (*R)
        .R_lrec = (Ssize((*R).R_rec as *mut libc::c_char) as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<R_row>() as libc::c_ulong) as libc::c_int;
    if (*R).R_lrec > 0o17777777777 as libc::c_int {
        (*R).R_lrec = 0o17777777777 as libc::c_int;
    }
    let ref mut fresh3 = (*R).R_hash;
    *fresh3 = s_alloc(2 as libc::c_int * (*R).R_lrec);
    (*R)
        .R_lhash = (Ssize((*R).R_hash as *mut libc::c_char) as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<SHORT>() as libc::c_ulong) as libc::c_int;
    p = (*R).R_hash;
    pl = p.offset((*R).R_lhash as isize);
    while p < pl {
        let fresh4 = p;
        p = p.offset(1);
        *fresh4 = 0o17777777777 as libc::c_int;
    }
    q = (*R).R_rec;
    ql = q.offset((*R).R_n as isize);
    i = 0 as libc::c_int;
    while q < ql {
        if R_member(R, (*q).R_a, (*q).R_b) != -(1 as libc::c_int) {
            Error(
                b"R_grow: BOTCH\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        q = q.offset(1);
        let fresh5 = i;
        i = i + 1;
        *R_hashpos = fresh5;
    }
    return R;
}
#[no_mangle]
pub unsafe extern "C" fn R_insert(
    mut R: R_OBJECT,
    mut reca: libc::c_int,
    mut recb: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if (*R).R_n >= (*R).R_lrec {
        if (*R).R_n >= 0o17777777777 as libc::c_int {
            Error(
                b"R_insert: Table FULL\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        R = R_grow(R, 2 as libc::c_int * (*R).R_lrec);
    }
    i = R_member(R, reca, recb);
    if i >= 0 as libc::c_int {
        return i;
    }
    (*((*R).R_rec).offset((*R).R_n as isize)).R_a = reca;
    (*((*R).R_rec).offset((*R).R_n as isize)).R_b = recb;
    let ref mut fresh6 = (*R).R_n;
    let fresh7 = *fresh6;
    *fresh6 = *fresh6 + 1;
    *R_hashpos = fresh7;
    return *R_hashpos;
}
#[no_mangle]
pub unsafe extern "C" fn R_rec(mut R: R_OBJECT, mut i: libc::c_int) -> *mut R_row {
    if i >= 0 as libc::c_int && i < (*R).R_n {
        return ((*R).R_rec).offset(i as isize)
    } else {
        return 0 as *mut R_row
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_stats() {
    fprintf(
        fpout,
        b"(R) Calls:%7d  Probes:%7d  Unsuccessful:%7d\n\0" as *const u8
            as *const libc::c_char,
        R_calls,
        R_probes,
        R_fail,
    );
}
