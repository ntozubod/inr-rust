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
    static mut fpout: *mut FILE;
    fn Salloc(_: libc::c_long) -> *mut libc::c_char;
    fn Sfree(_: *mut libc::c_char);
    fn Srealloc(_: *mut libc::c_char, _: libc::c_long) -> *mut libc::c_char;
    fn Ssize(_: *mut libc::c_char) -> libc::c_long;
    fn Error(_: *mut libc::c_char);
    fn s_alloc(_: libc::c_int) -> *mut SHORT;
    fn P_create(_: libc::c_int, _: *mut libc::c_char) -> P_OBJECT;
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
pub struct Tn_desc {
    pub Type: libc::c_int,
    pub Tn_n: libc::c_int,
    pub Tn_lname: libc::c_int,
    pub Tn_lhash: libc::c_int,
    pub Tn_lstor: libc::c_int,
    pub Tn_idxs: *mut libc::c_int,
    pub Tn_hash: *mut SHORT,
    pub Tn_stor: *mut libc::c_char,
}
pub type Tn_OBJECT = *mut Tn_desc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct P_desc {
    pub Type: libc::c_int,
    pub P_length: libc::c_int,
    pub P_cstr: *mut libc::c_char,
}
pub type P_OBJECT = *mut P_desc;
static mut Tn_hashpos: *mut SHORT = 0 as *const SHORT as *mut SHORT;
static mut Tn_calls: libc::c_int = 0 as libc::c_int;
static mut Tn_probes: libc::c_int = 0 as libc::c_int;
static mut Tn_fail: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn Tn_create() -> Tn_OBJECT {
    let mut Tn: Tn_OBJECT = 0 as *mut Tn_desc;
    Tn = Salloc(::std::mem::size_of::<Tn_desc>() as libc::c_ulong as libc::c_long)
        as Tn_OBJECT;
    (*Tn).Type = 1 as libc::c_int;
    (*Tn).Tn_n = 0 as libc::c_int;
    (*Tn).Tn_lname = 0 as libc::c_int;
    (*Tn).Tn_lhash = 1 as libc::c_int;
    (*Tn).Tn_lstor = 100 as libc::c_int;
    let ref mut fresh0 = (*Tn).Tn_idxs;
    *fresh0 = Salloc(
        (100 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            as libc::c_long,
    ) as *mut libc::c_int;
    *((*Tn).Tn_idxs).offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    let ref mut fresh1 = (*Tn).Tn_hash;
    *fresh1 = s_alloc(1 as libc::c_int);
    *((*Tn).Tn_hash).offset(0 as libc::c_int as isize) = 0o17777777777 as libc::c_int;
    let ref mut fresh2 = (*Tn).Tn_stor;
    *fresh2 = Salloc(100 as libc::c_int as libc::c_long);
    return Tn;
}
#[no_mangle]
pub unsafe extern "C" fn Tn_destroy(mut Tn: Tn_OBJECT) {
    if Tn.is_null() {
        return;
    }
    Sfree((*Tn).Tn_idxs as *mut libc::c_char);
    Sfree((*Tn).Tn_hash as *mut libc::c_char);
    Sfree((*Tn).Tn_stor);
    Sfree(Tn as *mut libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn Tn_member(
    mut Tn: Tn_OBJECT,
    mut name: *mut libc::c_char,
    mut length: libc::c_int,
) -> libc::c_int {
    let mut h: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut len_k: libc::c_int = 0;
    let mut na: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut SHORT = 0 as *mut SHORT;
    if *name.offset(length as isize) as libc::c_int == '\0' as i32 {} else {
        __assert_fail(
            b"name[ length ] == '\\0'\0" as *const u8 as *const libc::c_char,
            b"Tn.c\0" as *const u8 as *const libc::c_char,
            68 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"int Tn_member(Tn_OBJECT, char *, int)\0"))
                .as_ptr(),
        );
    }
    Tn_calls += 1;
    h = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < length {
        h = ((h as libc::c_uint)
            .wrapping_add(*name.offset(i as isize) as libc::c_uint)
            .wrapping_mul(16807 as libc::c_int as libc::c_uint)
            & 0o17777777777 as libc::c_int as libc::c_uint) as libc::c_int;
        i += 1;
    }
    p = ((*Tn).Tn_hash).offset((h % (*Tn).Tn_lhash) as isize);
    while *p < 0o17777777777 as libc::c_int {
        Tn_probes += 1;
        k = *p;
        len_k = *((*Tn).Tn_idxs).offset((k + 1 as libc::c_int) as isize)
            - *((*Tn).Tn_idxs).offset(k as isize) - 1 as libc::c_int;
        if length == len_k {
            na = ((*Tn).Tn_stor).offset(*((*Tn).Tn_idxs).offset(k as isize) as isize);
            i = 0 as libc::c_int;
            while i < length {
                if *na.offset(i as isize) as libc::c_int
                    != *name.offset(i as isize) as libc::c_int
                {
                    break;
                }
                i += 1;
            }
            if i == length {
                return k;
            }
        }
        p = p.offset(-1);
        if p < (*Tn).Tn_hash {
            p = ((*Tn).Tn_hash)
                .offset((*Tn).Tn_lhash as isize)
                .offset(-(1 as libc::c_int as isize));
        }
    }
    Tn_fail += 1;
    Tn_hashpos = p;
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Tn_grow(
    mut Tn: Tn_OBJECT,
    mut lname: libc::c_int,
) -> Tn_OBJECT {
    let mut i: libc::c_int = 0;
    let mut idx: libc::c_int = 0;
    let mut idx_next: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    if lname < 15 as libc::c_int {
        lname = 15 as libc::c_int;
    }
    if lname <= (*Tn).Tn_lname {
        return Tn;
    }
    Sfree((*Tn).Tn_hash as *mut libc::c_char);
    let ref mut fresh3 = (*Tn).Tn_idxs;
    *fresh3 = Srealloc(
        (*Tn).Tn_idxs as *mut libc::c_char,
        ((lname + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            as libc::c_long,
    ) as *mut libc::c_int;
    (*Tn)
        .Tn_lname = (Ssize((*Tn).Tn_idxs as *mut libc::c_char) as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    if (*Tn).Tn_lname > 0o17777777777 as libc::c_int {
        (*Tn).Tn_lname = 0o17777777777 as libc::c_int;
    }
    let ref mut fresh4 = (*Tn).Tn_hash;
    *fresh4 = s_alloc(2 as libc::c_int * (*Tn).Tn_lname);
    (*Tn)
        .Tn_lhash = (Ssize((*Tn).Tn_hash as *mut libc::c_char) as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<SHORT>() as libc::c_ulong) as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*Tn).Tn_lhash {
        *((*Tn).Tn_hash).offset(i as isize) = 0o17777777777 as libc::c_int;
        i += 1;
    }
    idx_next = *((*Tn).Tn_idxs).offset(0 as libc::c_int as isize);
    if idx_next == 0 as libc::c_int {} else {
        __assert_fail(
            b"idx_next == 0\0" as *const u8 as *const libc::c_char,
            b"Tn.c\0" as *const u8 as *const libc::c_char,
            115 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"Tn_OBJECT Tn_grow(Tn_OBJECT, int)\0"))
                .as_ptr(),
        );
    }
    i = 0 as libc::c_int;
    while i < (*Tn).Tn_n {
        idx = idx_next;
        idx_next = *((*Tn).Tn_idxs).offset((i + 1 as libc::c_int) as isize);
        len = idx_next - idx - 1 as libc::c_int;
        if Tn_member(Tn, ((*Tn).Tn_stor).offset(idx as isize), len)
            != -(1 as libc::c_int)
        {
            Error(
                b"Tn_grow: BOTCH\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        *Tn_hashpos = i;
        i += 1;
    }
    return Tn;
}
#[no_mangle]
pub unsafe extern "C" fn Tn_insert(
    mut Tn: Tn_OBJECT,
    mut name: *mut libc::c_char,
    mut length: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut current_stor_size: libc::c_int = 0;
    let mut next_stor_size: libc::c_int = 0;
    let mut na: *mut libc::c_char = 0 as *mut libc::c_char;
    if *name.offset(length as isize) as libc::c_int == '\0' as i32 {} else {
        __assert_fail(
            b"name[ length ] == '\\0'\0" as *const u8 as *const libc::c_char,
            b"Tn.c\0" as *const u8 as *const libc::c_char,
            132 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"int Tn_insert(Tn_OBJECT, char *, int)\0"))
                .as_ptr(),
        );
    }
    if (*Tn).Tn_n >= (*Tn).Tn_lname {
        if (*Tn).Tn_n >= 0o17777777777 as libc::c_int {
            Error(
                b"Tn_insert: Table FULL\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        Tn = Tn_grow(Tn, 2 as libc::c_int * (*Tn).Tn_lname);
    }
    i = Tn_member(Tn, name, length);
    if i >= 0 as libc::c_int {
        return i;
    }
    k = (*Tn).Tn_n;
    current_stor_size = *((*Tn).Tn_idxs).offset(k as isize);
    next_stor_size = current_stor_size + length + 1 as libc::c_int;
    if next_stor_size > (*Tn).Tn_lstor {
        let ref mut fresh5 = (*Tn).Tn_stor;
        *fresh5 = Srealloc(
            (*Tn).Tn_stor,
            (2 as libc::c_int * next_stor_size) as libc::c_long,
        );
    }
    na = ((*Tn).Tn_stor).offset(current_stor_size as isize);
    i = 0 as libc::c_int;
    while i < length {
        *na.offset(i as isize) = *name.offset(i as isize);
        i += 1;
    }
    *na.offset(length as isize) = '\0' as i32 as libc::c_char;
    *((*Tn).Tn_idxs).offset((k + 1 as libc::c_int) as isize) = next_stor_size;
    let ref mut fresh6 = (*Tn).Tn_n;
    let fresh7 = *fresh6;
    *fresh6 = *fresh6 + 1;
    *Tn_hashpos = fresh7;
    return *Tn_hashpos;
}
#[no_mangle]
pub unsafe extern "C" fn Tn_name(
    mut Tn: Tn_OBJECT,
    mut i: libc::c_int,
) -> *mut libc::c_char {
    if i >= 0 as libc::c_int && i < (*Tn).Tn_n {
        return ((*Tn).Tn_stor).offset(*((*Tn).Tn_idxs).offset(i as isize) as isize)
    } else {
        return 0 as *mut libc::c_char
    };
}
#[no_mangle]
pub unsafe extern "C" fn Tn_length(
    mut Tn: Tn_OBJECT,
    mut i: libc::c_int,
) -> libc::c_int {
    if i >= 0 as libc::c_int && i < (*Tn).Tn_n {
        return *((*Tn).Tn_idxs).offset((i + 1 as libc::c_int) as isize)
            - *((*Tn).Tn_idxs).offset(i as isize) - 1 as libc::c_int
    } else {
        return -(1 as libc::c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn Tn_Pstr(mut Tn: Tn_OBJECT, mut i: libc::c_int) -> P_OBJECT {
    if i >= 0 as libc::c_int && i < (*Tn).Tn_n {
        return P_create(
            *((*Tn).Tn_idxs).offset((i + 1 as libc::c_int) as isize)
                - *((*Tn).Tn_idxs).offset(i as isize) - 1 as libc::c_int,
            ((*Tn).Tn_stor).offset(*((*Tn).Tn_idxs).offset(i as isize) as isize),
        )
    } else {
        return 0 as P_OBJECT
    };
}
#[no_mangle]
pub unsafe extern "C" fn Tn_stats() {
    fprintf(
        fpout,
        b"(Tn) Calls:%7d  Probes:%7d  Unsuccessful:%7d\n\0" as *const u8
            as *const libc::c_char,
        Tn_calls,
        Tn_probes,
        Tn_fail,
    );
}
