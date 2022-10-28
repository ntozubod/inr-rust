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
pub struct V_desc {
    pub Type: libc::c_int,
    pub V_n: libc::c_int,
    pub V_lvec: libc::c_int,
    pub V_lhash: libc::c_int,
    pub V_vec: *mut *mut SHORT,
    pub V_hash: *mut SHORT,
}
pub type V_OBJECT = *mut V_desc;
static mut V_vecptr: *mut SHORT = 0 as *const SHORT as *mut SHORT;
static mut V_hashpos: *mut SHORT = 0 as *const SHORT as *mut SHORT;
static mut V_calls: libc::c_int = 0 as libc::c_int;
static mut V_probes: libc::c_int = 0 as libc::c_int;
static mut V_fail: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn veccpy(mut p: *mut SHORT, mut q: *mut SHORT) -> *mut SHORT {
    let mut save: *mut SHORT = p;
    while *q < 0o17777777777 as libc::c_int {
        let fresh0 = q;
        q = q.offset(1);
        let fresh1 = p;
        p = p.offset(1);
        *fresh1 = *fresh0;
    }
    *p = *q;
    return save;
}
#[no_mangle]
pub unsafe extern "C" fn veccmp(mut p: *mut SHORT, mut q: *mut SHORT) -> libc::c_int {
    p = p.offset(-1);
    q = q.offset(-1);
    loop {
        p = p.offset(1);
        q = q.offset(1);
        if !(*p == *q) {
            break;
        }
        if *p == 0o17777777777 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    return *p - *q;
}
#[no_mangle]
pub unsafe extern "C" fn veclen(mut p: *mut SHORT) -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    loop {
        let fresh2 = p;
        p = p.offset(1);
        if !(*fresh2 < 0o17777777777 as libc::c_int) {
            break;
        }
        i += 1;
    }
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn V_create() -> V_OBJECT {
    let mut V: V_OBJECT = 0 as *mut V_desc;
    V = Salloc(::std::mem::size_of::<V_desc>() as libc::c_ulong as libc::c_long)
        as V_OBJECT;
    (*V).Type = 2 as libc::c_int;
    (*V).V_n = 0 as libc::c_int;
    (*V).V_lvec = 0 as libc::c_int;
    (*V).V_lhash = 1 as libc::c_int;
    let ref mut fresh3 = (*V).V_vec;
    *fresh3 = 0 as *mut *mut SHORT;
    let ref mut fresh4 = (*V).V_hash;
    *fresh4 = s_alloc(1 as libc::c_int);
    *((*V).V_hash).offset(0 as libc::c_int as isize) = 0o17777777777 as libc::c_int;
    return V;
}
#[no_mangle]
pub unsafe extern "C" fn V_destroy(mut V: V_OBJECT) {
    let mut p: *mut *mut SHORT = 0 as *mut *mut SHORT;
    let mut pl: *mut *mut SHORT = 0 as *mut *mut SHORT;
    if V.is_null() {
        return;
    }
    p = (*V).V_vec;
    pl = p.offset((*V).V_n as isize);
    while p < pl {
        let fresh5 = p;
        p = p.offset(1);
        Sfree(*fresh5 as *mut libc::c_char);
    }
    Sfree((*V).V_vec as *mut libc::c_char);
    Sfree((*V).V_hash as *mut libc::c_char);
    Sfree(V as *mut libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn V_member(mut V: V_OBJECT, mut vec: *mut SHORT) -> libc::c_int {
    let mut h: libc::c_int = 0;
    let mut na: *mut SHORT = 0 as *mut SHORT;
    let mut p: *mut SHORT = 0 as *mut SHORT;
    V_calls += 1;
    h = 0 as libc::c_int;
    na = vec;
    while *na < 0o17777777777 as libc::c_int {
        let fresh6 = na;
        na = na.offset(1);
        h = (h + *fresh6) * 16807 as libc::c_int & 0o17777777777 as libc::c_int;
    }
    p = ((*V).V_hash).offset((h % (*V).V_lhash) as isize);
    while *p < 0o17777777777 as libc::c_int {
        V_probes += 1;
        if veccmp(*((*V).V_vec).offset(*p as isize), vec) == 0 as libc::c_int {
            return *p;
        }
        p = p.offset(-1);
        if p < (*V).V_hash {
            p = ((*V).V_hash)
                .offset((*V).V_lhash as isize)
                .offset(-(1 as libc::c_int as isize));
        }
    }
    V_fail += 1;
    V_vecptr = na;
    V_hashpos = p;
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn V_grow(mut V: V_OBJECT, mut lvec: libc::c_int) -> V_OBJECT {
    let mut p: *mut SHORT = 0 as *mut SHORT;
    let mut pl: *mut SHORT = 0 as *mut SHORT;
    let mut q: *mut *mut SHORT = 0 as *mut *mut SHORT;
    let mut ql: *mut *mut SHORT = 0 as *mut *mut SHORT;
    let mut i: libc::c_int = 0;
    if lvec < 15 as libc::c_int {
        lvec = 15 as libc::c_int;
    }
    if lvec <= (*V).V_lvec {
        return V;
    }
    Sfree((*V).V_hash as *mut libc::c_char);
    let ref mut fresh7 = (*V).V_vec;
    *fresh7 = Srealloc(
        (*V).V_vec as *mut libc::c_char,
        (lvec as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut SHORT>() as libc::c_ulong)
            as libc::c_long,
    ) as *mut *mut SHORT;
    (*V)
        .V_lvec = (Ssize((*V).V_vec as *mut libc::c_char) as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<*mut SHORT>() as libc::c_ulong)
        as libc::c_int;
    if (*V).V_lvec > 0o17777777777 as libc::c_int {
        (*V).V_lvec = 0o17777777777 as libc::c_int;
    }
    let ref mut fresh8 = (*V).V_hash;
    *fresh8 = s_alloc(2 as libc::c_int * (*V).V_lvec);
    (*V)
        .V_lhash = (Ssize((*V).V_hash as *mut libc::c_char) as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<SHORT>() as libc::c_ulong) as libc::c_int;
    p = (*V).V_hash;
    pl = p.offset((*V).V_lhash as isize);
    while p < pl {
        let fresh9 = p;
        p = p.offset(1);
        *fresh9 = 0o17777777777 as libc::c_int;
    }
    q = (*V).V_vec;
    ql = q.offset((*V).V_n as isize);
    i = 0 as libc::c_int;
    while q < ql {
        let fresh10 = q;
        q = q.offset(1);
        if V_member(V, *fresh10) != -(1 as libc::c_int) {
            Error(
                b"V_grow: BOTCH\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        let fresh11 = i;
        i = i + 1;
        *V_hashpos = fresh11;
    }
    return V;
}
#[no_mangle]
pub unsafe extern "C" fn V_insert(mut V: V_OBJECT, mut vec: *mut SHORT) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if (*V).V_n >= (*V).V_lvec {
        if (*V).V_n >= 0o17777777777 as libc::c_int {
            Error(
                b"V_insert: Table FULL\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        V = V_grow(V, 2 as libc::c_int * (*V).V_lvec);
    }
    i = V_member(V, vec);
    if i >= 0 as libc::c_int {
        return i;
    }
    let ref mut fresh12 = *((*V).V_vec).offset((*V).V_n as isize);
    *fresh12 = veccpy(
        s_alloc(
            (V_vecptr.offset_from(vec) as libc::c_long
                + 1 as libc::c_int as libc::c_long) as libc::c_int,
        ),
        vec,
    );
    let ref mut fresh13 = (*V).V_n;
    let fresh14 = *fresh13;
    *fresh13 = *fresh13 + 1;
    *V_hashpos = fresh14;
    return *V_hashpos;
}
#[no_mangle]
pub unsafe extern "C" fn V_vec(mut V: V_OBJECT, mut i: libc::c_int) -> *mut SHORT {
    if i >= 0 as libc::c_int && i < (*V).V_n {
        return *((*V).V_vec).offset(i as isize)
    } else {
        return 0 as *mut SHORT
    };
}
#[no_mangle]
pub unsafe extern "C" fn V_stats() {
    fprintf(
        fpout,
        b"(V) Calls:%7d  Probes:%7d  Unsuccessful:%7d\n\0" as *const u8
            as *const libc::c_char,
        V_calls,
        V_probes,
        V_fail,
    );
}
