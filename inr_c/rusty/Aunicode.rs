use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn Warning(_: *mut libc::c_char);
    fn T2_insert(_: T2_OBJECT, _: *mut libc::c_char, _: libc::c_int) -> libc::c_int;
    fn T2_name(_: T2_OBJECT, _: libc::c_int) -> *mut libc::c_char;
    fn T2_length(_: T2_OBJECT, _: libc::c_int) -> libc::c_int;
    fn A_create() -> A_OBJECT;
    fn A_open(_: A_OBJECT) -> A_OBJECT;
    fn A_add(_: A_OBJECT, _: libc::c_int, _: libc::c_int, _: libc::c_int) -> A_OBJECT;
    fn A_close(_: A_OBJECT) -> A_OBJECT;
    fn A_min(_: A_OBJECT) -> A_OBJECT;
    fn A_lenmin(_: A_OBJECT) -> A_OBJECT;
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
pub struct T2_desc {
    pub Type: libc::c_int,
    pub T2_int: Tn_OBJECT,
    pub T2_ext: Tn_OBJECT,
}
pub type T2_OBJECT = *mut T2_desc;
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
pub unsafe extern "C" fn A_slurp_octets(
    mut file: *mut libc::c_char,
    mut T2_Sigma: T2_OBJECT,
) -> A_OBJECT {
    let mut A: A_OBJECT = 0 as *mut A_desc;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut c: libc::c_int = 0;
    let mut state: SHORT = 0;
    let mut next_state: SHORT = 0;
    if !file.is_null() {
        fp = fopen(file, b"r\0" as *const u8 as *const libc::c_char);
    }
    if fp.is_null() {
        Warning(
            b"File does not exist\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return 0 as A_OBJECT;
    }
    if !T2_Sigma.is_null() {} else {
        __assert_fail(
            b"T2_Sigma != NULL\0" as *const u8 as *const libc::c_char,
            b"Aunicode.c\0" as *const u8 as *const libc::c_char,
            42 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 43],
                &[libc::c_char; 43],
            >(b"A_OBJECT A_slurp_octets(char *, T2_OBJECT)\0"))
                .as_ptr(),
        );
    }
    if (*(*T2_Sigma).T2_int).Tn_n >= 258 as libc::c_int {} else {
        __assert_fail(
            b"T2_Sigma-> T2_int-> Tn_n >= 258\0" as *const u8 as *const libc::c_char,
            b"Aunicode.c\0" as *const u8 as *const libc::c_char,
            43 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 43],
                &[libc::c_char; 43],
            >(b"A_OBJECT A_slurp_octets(char *, T2_OBJECT)\0"))
                .as_ptr(),
        );
    }
    A = A_create();
    c = getc(fp);
    state = 0 as libc::c_int;
    next_state = 2 as libc::c_int;
    while c != -(1 as libc::c_int) {
        if next_state <= 0o17777777777 as libc::c_int {} else {
            __assert_fail(
                b"next_state <= MAXSHORT\0" as *const u8 as *const libc::c_char,
                b"Aunicode.c\0" as *const u8 as *const libc::c_char,
                50 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 43],
                    &[libc::c_char; 43],
                >(b"A_OBJECT A_slurp_octets(char *, T2_OBJECT)\0"))
                    .as_ptr(),
            );
        }
        A = A_add(A, state, c + 2 as libc::c_int, next_state);
        let fresh0 = next_state;
        next_state = next_state + 1;
        state = fresh0;
        c = getc(fp);
    }
    A = A_add(A, state, 1 as libc::c_int, 1 as libc::c_int);
    A = A_close(A);
    fclose(fp);
    return A;
}
#[no_mangle]
pub unsafe extern "C" fn A_slurp_utf8(
    mut file: *mut libc::c_char,
    mut T2_Sigma: T2_OBJECT,
) -> A_OBJECT {
    let mut A: A_OBJECT = 0 as *mut A_desc;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut i: libc::c_int = 0;
    let mut c1: libc::c_int = 0;
    let mut c2: libc::c_int = 0;
    let mut c3: libc::c_int = 0;
    let mut c4: libc::c_int = 0;
    let mut cp: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut ts: [libc::c_char; 5] = [0; 5];
    if !file.is_null() {
        fp = fopen(file, b"r\0" as *const u8 as *const libc::c_char);
    }
    if fp.is_null() {
        Warning(
            b"File does not exist\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return 0 as A_OBJECT;
    }
    if !T2_Sigma.is_null() {} else {
        __assert_fail(
            b"T2_Sigma != NULL\0" as *const u8 as *const libc::c_char,
            b"Aunicode.c\0" as *const u8 as *const libc::c_char,
            74 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"A_OBJECT A_slurp_utf8(char *, T2_OBJECT)\0"))
                .as_ptr(),
        );
    }
    if (*(*T2_Sigma).T2_int).Tn_n >= 258 as libc::c_int {} else {
        __assert_fail(
            b"T2_Sigma-> T2_int-> Tn_n >= 258\0" as *const u8 as *const libc::c_char,
            b"Aunicode.c\0" as *const u8 as *const libc::c_char,
            75 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"A_OBJECT A_slurp_utf8(char *, T2_OBJECT)\0"))
                .as_ptr(),
        );
    }
    A = A_slurp_octets(file, T2_Sigma);
    A = A_open(A);
    i = 0 as libc::c_int;
    while i < (*A).A_nrows {
        c1 = (*((*A).A_t).offset(i as isize)).A_b - 2 as libc::c_int;
        if c1 & 0x80 as libc::c_int == 0 as libc::c_int {
            type_0 = 0 as libc::c_int;
        } else if c1 & 0xc0 as libc::c_int == 0x80 as libc::c_int {
            type_0 = 1 as libc::c_int;
        } else if c1 & 0xe0 as libc::c_int == 0xc0 as libc::c_int {
            type_0 = 2 as libc::c_int;
        } else if c1 & 0xf0 as libc::c_int == 0xe0 as libc::c_int {
            type_0 = 3 as libc::c_int;
        } else if c1 & 0xf8 as libc::c_int == 0xf0 as libc::c_int {
            type_0 = 4 as libc::c_int;
        } else {
            type_0 = 5 as libc::c_int;
        }
        match type_0 {
            2 => {
                if (i + 1 as libc::c_int) < (*A).A_nrows {} else {
                    __assert_fail(
                        b"i + 1 < A-> A_nrows\0" as *const u8 as *const libc::c_char,
                        b"Aunicode.c\0" as *const u8 as *const libc::c_char,
                        92 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 41],
                            &[libc::c_char; 41],
                        >(b"A_OBJECT A_slurp_utf8(char *, T2_OBJECT)\0"))
                            .as_ptr(),
                    );
                }
                c2 = (*((*A).A_t).offset((i + 1 as libc::c_int) as isize)).A_b
                    - 2 as libc::c_int;
                cp = (c1 & 0x1f as libc::c_int) + (c2 & 0x3f as libc::c_int);
                if c2 & 0xc0 as libc::c_int == 0x80 as libc::c_int
                    && cp > 0x7f as libc::c_int
                {
                    ts[0 as libc::c_int as usize] = c1 as libc::c_char;
                    ts[1 as libc::c_int as usize] = c2 as libc::c_char;
                    ts[2 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
                }
                (*((*A).A_t).offset(i as isize))
                    .A_b = T2_insert(T2_Sigma, ts.as_mut_ptr(), 2 as libc::c_int);
                let ref mut fresh1 = (*((*A).A_t).offset(i as isize)).A_c;
                *fresh1 += 1 as libc::c_int;
            }
            3 => {
                if (i + 2 as libc::c_int) < (*A).A_nrows {} else {
                    __assert_fail(
                        b"i + 2 < A-> A_nrows\0" as *const u8 as *const libc::c_char,
                        b"Aunicode.c\0" as *const u8 as *const libc::c_char,
                        106 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 41],
                            &[libc::c_char; 41],
                        >(b"A_OBJECT A_slurp_utf8(char *, T2_OBJECT)\0"))
                            .as_ptr(),
                    );
                }
                c2 = (*((*A).A_t).offset((i + 1 as libc::c_int) as isize)).A_b
                    - 2 as libc::c_int;
                c3 = (*((*A).A_t).offset((i + 2 as libc::c_int) as isize)).A_b
                    - 2 as libc::c_int;
                cp = ((c1 & 0xf as libc::c_int) << 12 as libc::c_int)
                    + ((c2 & 0x3f as libc::c_int) << 6 as libc::c_int)
                    + (c3 & 0x3f as libc::c_int);
                if c2 & 0xc0 as libc::c_int == 0x80 as libc::c_int
                    && c3 & 0xc0 as libc::c_int == 0x80 as libc::c_int
                    && ((0x3ff as libc::c_int) < cp && cp < 0xd800 as libc::c_int
                        || (0xdfff as libc::c_int) < cp)
                {
                    ts[0 as libc::c_int as usize] = c1 as libc::c_char;
                    ts[1 as libc::c_int as usize] = c2 as libc::c_char;
                    ts[2 as libc::c_int as usize] = c3 as libc::c_char;
                    ts[3 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
                }
                (*((*A).A_t).offset(i as isize))
                    .A_b = T2_insert(T2_Sigma, ts.as_mut_ptr(), 3 as libc::c_int);
                let ref mut fresh2 = (*((*A).A_t).offset(i as isize)).A_c;
                *fresh2 += 2 as libc::c_int;
            }
            4 => {
                if (i + 3 as libc::c_int) < (*A).A_nrows {} else {
                    __assert_fail(
                        b"i + 3 < A-> A_nrows\0" as *const u8 as *const libc::c_char,
                        b"Aunicode.c\0" as *const u8 as *const libc::c_char,
                        126 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 41],
                            &[libc::c_char; 41],
                        >(b"A_OBJECT A_slurp_utf8(char *, T2_OBJECT)\0"))
                            .as_ptr(),
                    );
                }
                c2 = (*((*A).A_t).offset((i + 1 as libc::c_int) as isize)).A_b
                    - 2 as libc::c_int;
                c3 = (*((*A).A_t).offset((i + 2 as libc::c_int) as isize)).A_b
                    - 2 as libc::c_int;
                c4 = (*((*A).A_t).offset((i + 3 as libc::c_int) as isize)).A_b
                    - 2 as libc::c_int;
                cp = ((c1 & 0xf as libc::c_int) << 18 as libc::c_int)
                    + ((c2 & 0x3f as libc::c_int) << 12 as libc::c_int)
                    + ((c3 & 0x3f as libc::c_int) << 6 as libc::c_int)
                    + (c4 & 0x3f as libc::c_int);
                if c2 & 0xc0 as libc::c_int == 0x80 as libc::c_int
                    && c3 & 0xc0 as libc::c_int == 0x80 as libc::c_int
                    && c4 & 0xc0 as libc::c_int == 0x80 as libc::c_int
                    && (0xffff as libc::c_int) < cp && cp <= 0x10ffff as libc::c_int
                {
                    ts[0 as libc::c_int as usize] = c1 as libc::c_char;
                    ts[1 as libc::c_int as usize] = c2 as libc::c_char;
                    ts[2 as libc::c_int as usize] = c3 as libc::c_char;
                    ts[3 as libc::c_int as usize] = c4 as libc::c_char;
                    ts[4 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
                }
                (*((*A).A_t).offset(i as isize))
                    .A_b = T2_insert(T2_Sigma, ts.as_mut_ptr(), 4 as libc::c_int);
                let ref mut fresh3 = (*((*A).A_t).offset(i as isize)).A_c;
                *fresh3 += 3 as libc::c_int;
            }
            0 | 1 | 5 | _ => {}
        }
        i += 1;
    }
    if (*(*T2_Sigma).T2_int).Tn_n > (*A).A_nS {
        (*A).A_nS = (*(*T2_Sigma).T2_int).Tn_n;
    }
    A = A_min(A);
    fclose(fp);
    return A;
}
#[no_mangle]
pub unsafe extern "C" fn A_spit_octets(
    mut A: A_OBJECT,
    mut file: *mut libc::c_char,
    mut T2_Sigma: T2_OBJECT,
) -> A_OBJECT {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut i: libc::c_int = 0;
    let mut s1: libc::c_int = 0;
    let mut c1: libc::c_int = 0;
    if !file.is_null() {
        fp = fopen(file, b"w\0" as *const u8 as *const libc::c_char);
    }
    if fp.is_null() {
        Warning(
            b"Can't open file for write\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return 0 as A_OBJECT;
    }
    if !T2_Sigma.is_null() {} else {
        __assert_fail(
            b"T2_Sigma != NULL\0" as *const u8 as *const libc::c_char,
            b"Aunicode.c\0" as *const u8 as *const libc::c_char,
            172 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 52],
                &[libc::c_char; 52],
            >(b"A_OBJECT A_spit_octets(A_OBJECT, char *, T2_OBJECT)\0"))
                .as_ptr(),
        );
    }
    if (*(*T2_Sigma).T2_int).Tn_n >= 258 as libc::c_int {} else {
        __assert_fail(
            b"T2_Sigma-> T2_int-> Tn_n >= 258\0" as *const u8 as *const libc::c_char,
            b"Aunicode.c\0" as *const u8 as *const libc::c_char,
            173 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 52],
                &[libc::c_char; 52],
            >(b"A_OBJECT A_spit_octets(A_OBJECT, char *, T2_OBJECT)\0"))
                .as_ptr(),
        );
    }
    A = A_gen_min(A);
    i = 0 as libc::c_int;
    while i < (*A).A_nrows {
        s1 = (*((*A).A_t).offset(i as isize)).A_b;
        if s1 < 258 as libc::c_int {} else {
            __assert_fail(
                b"s1 < 258\0" as *const u8 as *const libc::c_char,
                b"Aunicode.c\0" as *const u8 as *const libc::c_char,
                179 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 52],
                    &[libc::c_char; 52],
                >(b"A_OBJECT A_spit_octets(A_OBJECT, char *, T2_OBJECT)\0"))
                    .as_ptr(),
            );
        }
        if s1 >= 2 as libc::c_int {
            c1 = s1 - 2 as libc::c_int;
            fputc(c1, fp);
        }
        i += 1;
    }
    fclose(fp);
    return A;
}
#[no_mangle]
pub unsafe extern "C" fn A_spit_utf8(
    mut A: A_OBJECT,
    mut file: *mut libc::c_char,
    mut T2_Sigma: T2_OBJECT,
) -> A_OBJECT {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut i: libc::c_int = 0;
    let mut s1: libc::c_int = 0;
    let mut c1: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    let mut cstr: *mut libc::c_char = 0 as *mut libc::c_char;
    if !file.is_null() {
        fp = fopen(file, b"w\0" as *const u8 as *const libc::c_char);
    }
    if fp.is_null() {
        Warning(
            b"Can't open file for write\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return 0 as A_OBJECT;
    }
    if !T2_Sigma.is_null() {} else {
        __assert_fail(
            b"T2_Sigma != NULL\0" as *const u8 as *const libc::c_char,
            b"Aunicode.c\0" as *const u8 as *const libc::c_char,
            202 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"A_OBJECT A_spit_utf8(A_OBJECT, char *, T2_OBJECT)\0"))
                .as_ptr(),
        );
    }
    if (*(*T2_Sigma).T2_int).Tn_n >= 258 as libc::c_int {} else {
        __assert_fail(
            b"T2_Sigma-> T2_int-> Tn_n >= 258\0" as *const u8 as *const libc::c_char,
            b"Aunicode.c\0" as *const u8 as *const libc::c_char,
            203 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"A_OBJECT A_spit_utf8(A_OBJECT, char *, T2_OBJECT)\0"))
                .as_ptr(),
        );
    }
    A = A_gen_min(A);
    i = 0 as libc::c_int;
    while i < (*A).A_nrows {
        s1 = (*((*A).A_t).offset(i as isize)).A_b;
        if !(s1 < 2 as libc::c_int) {
            if s1 < 258 as libc::c_int {
                c1 = s1 - 2 as libc::c_int;
                if c1 & 0x80 as libc::c_int == 0 as libc::c_int {
                    type_0 = 0 as libc::c_int;
                } else if c1 & 0xc0 as libc::c_int == 0x80 as libc::c_int {
                    type_0 = 1 as libc::c_int;
                } else if c1 & 0xe0 as libc::c_int == 0xc0 as libc::c_int {
                    type_0 = 2 as libc::c_int;
                } else if c1 & 0xf0 as libc::c_int == 0xe0 as libc::c_int {
                    type_0 = 3 as libc::c_int;
                } else if c1 & 0xf8 as libc::c_int == 0xf0 as libc::c_int {
                    type_0 = 4 as libc::c_int;
                } else {
                    type_0 = 5 as libc::c_int;
                }
                match type_0 {
                    0 | 1 | 2 | 3 | 4 | 5 => {
                        fputc(c1, fp);
                    }
                    _ => {}
                }
            } else {
                cstr = T2_name(T2_Sigma, s1);
                length = T2_length(T2_Sigma, s1);
                j = 0 as libc::c_int;
                while j < length {
                    fputc(*cstr.offset(j as isize) as libc::c_int, fp);
                    j += 1;
                }
            }
        }
        i += 1;
    }
    fclose(fp);
    return A;
}
#[no_mangle]
pub unsafe extern "C" fn A_gen_min(mut A: A_OBJECT) -> A_OBJECT {
    let mut i: libc::c_int = 0;
    let mut dead_state: libc::c_int = 0;
    let mut last_state: libc::c_int = 0;
    let mut state: libc::c_int = 0;
    A = A_lenmin(A);
    A = A_min(A);
    A = A_open(A);
    let ref mut fresh4 = (*A).A_nQ;
    let fresh5 = *fresh4;
    *fresh4 = *fresh4 + 1;
    dead_state = fresh5;
    last_state = -(1 as libc::c_int);
    i = 0 as libc::c_int;
    while i < (*A).A_nrows {
        state = (*((*A).A_t).offset(i as isize)).A_a;
        if state == last_state {
            (*((*A).A_t).offset(i as isize)).A_c = dead_state;
        }
        last_state = state;
        i += 1;
    }
    A = A_min(A);
    return A;
}
#[no_mangle]
pub unsafe extern "C" fn A_valid_utf8_at(
    mut s: *mut libc::c_char,
    mut i: libc::c_int,
    mut l: libc::c_int,
    mut cp_ref: *mut libc::c_int,
) -> libc::c_int {
    let mut c1: libc::c_int = *s.offset(i as isize) as libc::c_int;
    let mut type_0: libc::c_int = 0;
    let mut c2: libc::c_int = 0;
    let mut c3: libc::c_int = 0;
    let mut c4: libc::c_int = 0;
    let mut cp: libc::c_int = 0;
    *cp_ref = -(1 as libc::c_int);
    if c1 & 0x80 as libc::c_int == 0 as libc::c_int {
        type_0 = 0 as libc::c_int;
    } else if c1 & 0xc0 as libc::c_int == 0x80 as libc::c_int {
        type_0 = 1 as libc::c_int;
    } else if c1 & 0xe0 as libc::c_int == 0xc0 as libc::c_int {
        type_0 = 2 as libc::c_int;
    } else if c1 & 0xf0 as libc::c_int == 0xe0 as libc::c_int {
        type_0 = 3 as libc::c_int;
    } else if c1 & 0xf8 as libc::c_int == 0xf0 as libc::c_int {
        type_0 = 4 as libc::c_int;
    } else {
        type_0 = 5 as libc::c_int;
    }
    match type_0 {
        0 => {
            cp = c1;
            *cp_ref = cp;
            return 1 as libc::c_int;
        }
        2 => {
            if i + 1 as libc::c_int >= l {
                return 0 as libc::c_int;
            }
            c2 = *s.offset((i + 1 as libc::c_int) as isize) as libc::c_int;
            cp = ((c1 & 0x1f as libc::c_int) << 6 as libc::c_int)
                + (c2 & 0x3f as libc::c_int);
            if c2 & 0xc0 as libc::c_int == 0x80 as libc::c_int
                && cp > 0x7f as libc::c_int
            {
                *cp_ref = cp;
                return 2 as libc::c_int;
            }
        }
        3 => {
            if i + 2 as libc::c_int >= l {
                return 0 as libc::c_int;
            }
            c2 = *s.offset((i + 1 as libc::c_int) as isize) as libc::c_int;
            c3 = *s.offset((i + 2 as libc::c_int) as isize) as libc::c_int;
            cp = ((c1 & 0xf as libc::c_int) << 12 as libc::c_int)
                + ((c2 & 0x3f as libc::c_int) << 6 as libc::c_int)
                + (c3 & 0x3f as libc::c_int);
            if c2 & 0xc0 as libc::c_int == 0x80 as libc::c_int
                && c3 & 0xc0 as libc::c_int == 0x80 as libc::c_int
                && ((0x3ff as libc::c_int) < cp && cp < 0xd800 as libc::c_int
                    || (0xdfff as libc::c_int) < cp)
            {
                *cp_ref = cp;
                return 3 as libc::c_int;
            }
        }
        4 => {
            if i + 3 as libc::c_int >= l {
                return 0 as libc::c_int;
            }
            c2 = *s.offset((i + 1 as libc::c_int) as isize) as libc::c_int;
            c3 = *s.offset((i + 2 as libc::c_int) as isize) as libc::c_int;
            c4 = *s.offset((i + 3 as libc::c_int) as isize) as libc::c_int;
            cp = ((c1 & 0xf as libc::c_int) << 18 as libc::c_int)
                + ((c2 & 0x3f as libc::c_int) << 12 as libc::c_int)
                + ((c3 & 0x3f as libc::c_int) << 6 as libc::c_int)
                + (c4 & 0x3f as libc::c_int);
            if c2 & 0xc0 as libc::c_int == 0x80 as libc::c_int
                && c3 & 0xc0 as libc::c_int == 0x80 as libc::c_int
                && c4 & 0xc0 as libc::c_int == 0x80 as libc::c_int
                && (0xffff as libc::c_int) < cp && cp <= 0x10ffff as libc::c_int
            {
                *cp_ref = cp;
                return 4 as libc::c_int;
            }
        }
        1 | 5 | _ => {}
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn A_unicode_printable(mut cp: libc::c_int) -> libc::c_int {
    if 0 as libc::c_int <= cp && cp < 0x21 as libc::c_int
        || 0x7f as libc::c_int <= cp && cp < 0xa1 as libc::c_int
        || 0xad as libc::c_int <= cp && cp < 0xae as libc::c_int
        || 0x600 as libc::c_int <= cp && cp < 0x606 as libc::c_int
        || 0x61c as libc::c_int <= cp && cp < 0x61d as libc::c_int
        || 0x6dd as libc::c_int <= cp && cp < 0x6de as libc::c_int
        || 0x70f as libc::c_int <= cp && cp < 0x710 as libc::c_int
        || 0x890 as libc::c_int <= cp && cp < 0x892 as libc::c_int
        || 0x8e2 as libc::c_int <= cp && cp < 0x8e3 as libc::c_int
        || 0x1680 as libc::c_int <= cp && cp < 0x1681 as libc::c_int
        || 0x180e as libc::c_int <= cp && cp < 0x180f as libc::c_int
        || 0x2000 as libc::c_int <= cp && cp < 0x2010 as libc::c_int
        || 0x2028 as libc::c_int <= cp && cp < 0x2030 as libc::c_int
        || 0x205f as libc::c_int <= cp && cp < 0x2065 as libc::c_int
        || 0x2066 as libc::c_int <= cp && cp < 0x2070 as libc::c_int
        || 0x3000 as libc::c_int <= cp && cp < 0x3001 as libc::c_int
        || 0xd800 as libc::c_int <= cp && cp < 0xf900 as libc::c_int
        || 0xfdd0 as libc::c_int <= cp && cp < 0xfdf0 as libc::c_int
        || 0xfeff as libc::c_int <= cp && cp < 0xff00 as libc::c_int
        || 0xfff9 as libc::c_int <= cp && cp < 0xfffc as libc::c_int
        || 0xfffe as libc::c_int <= cp && cp < 0x10000 as libc::c_int
        || 0x110bd as libc::c_int <= cp && cp < 0x110be as libc::c_int
        || 0x110cd as libc::c_int <= cp && cp < 0x110ce as libc::c_int
        || 0x13430 as libc::c_int <= cp && cp < 0x13439 as libc::c_int
        || 0x1bca0 as libc::c_int <= cp && cp < 0x1bca4 as libc::c_int
        || 0x1d173 as libc::c_int <= cp && cp < 0x1d17b as libc::c_int
        || 0x1fffe as libc::c_int <= cp && cp < 0x20000 as libc::c_int
        || 0x2fffe as libc::c_int <= cp && cp < 0x30000 as libc::c_int
        || 0x3fffe as libc::c_int <= cp && cp < 0x40000 as libc::c_int
        || 0x4fffe as libc::c_int <= cp && cp < 0x50000 as libc::c_int
        || 0x5fffe as libc::c_int <= cp && cp < 0x60000 as libc::c_int
        || 0x6fffe as libc::c_int <= cp && cp < 0x70000 as libc::c_int
        || 0x7fffe as libc::c_int <= cp && cp < 0x80000 as libc::c_int
        || 0x8fffe as libc::c_int <= cp && cp < 0x90000 as libc::c_int
        || 0x9fffe as libc::c_int <= cp && cp < 0xa0000 as libc::c_int
        || 0xafffe as libc::c_int <= cp && cp < 0xb0000 as libc::c_int
        || 0xbfffe as libc::c_int <= cp && cp < 0xc0000 as libc::c_int
        || 0xcfffe as libc::c_int <= cp && cp < 0xd0000 as libc::c_int
        || 0xdfffe as libc::c_int <= cp && cp < 0xe0000 as libc::c_int
        || 0xe0001 as libc::c_int <= cp && cp < 0xe0002 as libc::c_int
        || 0xe0020 as libc::c_int <= cp && cp < 0xe0080 as libc::c_int
        || 0xefffe as libc::c_int <= cp && cp <= 0x10ffff as libc::c_int
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
