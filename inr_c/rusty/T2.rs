use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn Salloc(_: libc::c_long) -> *mut libc::c_char;
    fn Sfree(_: *mut libc::c_char);
    fn Srealloc(_: *mut libc::c_char, _: libc::c_long) -> *mut libc::c_char;
    fn Ssize(_: *mut libc::c_char) -> libc::c_long;
    fn Tn_create() -> Tn_OBJECT;
    fn Tn_destroy(_: Tn_OBJECT);
    fn Tn_member(_: Tn_OBJECT, _: *mut libc::c_char, _: libc::c_int) -> libc::c_int;
    fn Tn_insert(_: Tn_OBJECT, _: *mut libc::c_char, _: libc::c_int) -> libc::c_int;
    fn Tn_name(_: Tn_OBJECT, _: libc::c_int) -> *mut libc::c_char;
    fn Tn_length(_: Tn_OBJECT, _: libc::c_int) -> libc::c_int;
    fn Tn_Pstr(_: Tn_OBJECT, _: libc::c_int) -> P_OBJECT;
    fn Tn_stats();
    fn A_unicode_printable(_: libc::c_int) -> libc::c_int;
    fn A_valid_utf8_at(
        _: *mut libc::c_char,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut libc::c_int,
    ) -> libc::c_int;
}
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
pub struct T2_desc {
    pub Type: libc::c_int,
    pub T2_int: Tn_OBJECT,
    pub T2_ext: Tn_OBJECT,
}
pub type T2_OBJECT = *mut T2_desc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct P_desc {
    pub Type: libc::c_int,
    pub P_length: libc::c_int,
    pub P_cstr: *mut libc::c_char,
}
pub type P_OBJECT = *mut P_desc;
#[no_mangle]
pub unsafe extern "C" fn T2_create() -> T2_OBJECT {
    let mut T2: T2_OBJECT = 0 as *mut T2_desc;
    T2 = Salloc(::std::mem::size_of::<T2_desc>() as libc::c_ulong as libc::c_long)
        as T2_OBJECT;
    let ref mut fresh0 = (*T2).T2_int;
    *fresh0 = Tn_create();
    let ref mut fresh1 = (*T2).T2_ext;
    *fresh1 = Tn_create();
    return T2;
}
#[no_mangle]
pub unsafe extern "C" fn T2_destroy(mut T2: T2_OBJECT) {
    if T2.is_null() {
        return;
    }
    Tn_destroy((*T2).T2_int);
    Tn_destroy((*T2).T2_ext);
    Sfree(T2 as *mut libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn T2_member(
    mut T2: T2_OBJECT,
    mut name: *mut libc::c_char,
    mut length: libc::c_int,
) -> libc::c_int {
    return Tn_member((*T2).T2_int, name, length);
}
#[no_mangle]
pub unsafe extern "C" fn T2_insert(
    mut T2: T2_OBJECT,
    mut name: *mut libc::c_char,
    mut length: libc::c_int,
) -> libc::c_int {
    return Tn_insert((*T2).T2_int, name, length);
}
#[no_mangle]
pub unsafe extern "C" fn T2_name(
    mut T2: T2_OBJECT,
    mut i: libc::c_int,
) -> *mut libc::c_char {
    return Tn_name((*T2).T2_int, i);
}
#[no_mangle]
pub unsafe extern "C" fn T2_length(
    mut T2: T2_OBJECT,
    mut i: libc::c_int,
) -> libc::c_int {
    return Tn_length((*T2).T2_int, i);
}
#[no_mangle]
pub unsafe extern "C" fn T2_Pstr(mut T2: T2_OBJECT, mut i: libc::c_int) -> P_OBJECT {
    return Tn_Pstr((*T2).T2_int, i);
}
#[no_mangle]
pub unsafe extern "C" fn T2_name_pr(
    mut T2: T2_OBJECT,
    mut i: libc::c_int,
) -> *mut libc::c_char {
    T2_sync(T2);
    return Tn_name((*T2).T2_ext, i);
}
#[no_mangle]
pub unsafe extern "C" fn T2_length_pr(
    mut T2: T2_OBJECT,
    mut i: libc::c_int,
) -> libc::c_int {
    T2_sync(T2);
    return Tn_length((*T2).T2_ext, i);
}
#[no_mangle]
pub unsafe extern "C" fn T2_Pstr_pr(mut T2: T2_OBJECT, mut i: libc::c_int) -> P_OBJECT {
    T2_sync(T2);
    return Tn_Pstr((*T2).T2_ext, i);
}
#[no_mangle]
pub unsafe extern "C" fn T2_stats() {
    Tn_stats();
}
#[no_mangle]
pub unsafe extern "C" fn T2_sync(mut T2: T2_OBJECT) {
    let mut hexmap: [libc::c_char; 17] = *::std::mem::transmute::<
        &[u8; 17],
        &mut [libc::c_char; 17],
    >(b"0123456789ABCDEF\0");
    let mut pr_str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    let mut next_ch: libc::c_int = 0;
    let mut ii: libc::c_int = 0;
    let mut pr_str_length: libc::c_int = 0;
    let mut j2: libc::c_int = 0;
    let mut utf8_len: libc::c_int = 0;
    let mut cp: libc::c_int = 0;
    let mut TT: Tn_OBJECT = (*T2).T2_int;
    let mut TT_print: Tn_OBJECT = (*T2).T2_ext;
    pr_str = Salloc(100 as libc::c_int as libc::c_long);
    pr_str_length = Ssize(pr_str) as libc::c_int;
    i = (*TT_print).Tn_n;
    while i < (*TT).Tn_n {
        length = Tn_length(TT, i);
        if pr_str_length < length * 4 as libc::c_int + 1 as libc::c_int {
            pr_str = Srealloc(
                pr_str,
                (length * 4 as libc::c_int + 1 as libc::c_int) as libc::c_long,
            );
        }
        cstr = Tn_name(TT, i);
        k = 0 as libc::c_int;
        j = 0 as libc::c_int;
        while j < length {
            next_ch = *cstr.offset(j as isize) as libc::c_int;
            utf8_len = A_valid_utf8_at(cstr, j, length, &mut cp);
            if utf8_len > 0 as libc::c_int && A_unicode_printable(cp) != 0 {
                let fresh2 = k;
                k = k + 1;
                *pr_str.offset(fresh2 as isize) = next_ch as libc::c_char;
                j2 = 1 as libc::c_int;
                while j2 < utf8_len {
                    j += 1;
                    next_ch = *cstr.offset(j as isize) as libc::c_int;
                    let fresh3 = k;
                    k = k + 1;
                    *pr_str.offset(fresh3 as isize) = next_ch as libc::c_char;
                    j2 += 1;
                }
            } else if next_ch == ' ' as i32 {
                let fresh4 = k;
                k = k + 1;
                *pr_str.offset(fresh4 as isize) = '\\' as i32 as libc::c_char;
                let fresh5 = k;
                k = k + 1;
                *pr_str.offset(fresh5 as isize) = '_' as i32 as libc::c_char;
            } else if next_ch == '\t' as i32 {
                let fresh6 = k;
                k = k + 1;
                *pr_str.offset(fresh6 as isize) = '\\' as i32 as libc::c_char;
                let fresh7 = k;
                k = k + 1;
                *pr_str.offset(fresh7 as isize) = 't' as i32 as libc::c_char;
            } else if next_ch == '\n' as i32 {
                let fresh8 = k;
                k = k + 1;
                *pr_str.offset(fresh8 as isize) = '\\' as i32 as libc::c_char;
                let fresh9 = k;
                k = k + 1;
                *pr_str.offset(fresh9 as isize) = 'n' as i32 as libc::c_char;
            } else if next_ch == '\\' as i32 {
                let fresh10 = k;
                k = k + 1;
                *pr_str.offset(fresh10 as isize) = '\\' as i32 as libc::c_char;
                let fresh11 = k;
                k = k + 1;
                *pr_str.offset(fresh11 as isize) = '\\' as i32 as libc::c_char;
            } else if next_ch == '.' as i32 {
                let fresh12 = k;
                k = k + 1;
                *pr_str.offset(fresh12 as isize) = '\\' as i32 as libc::c_char;
                let fresh13 = k;
                k = k + 1;
                *pr_str.offset(fresh13 as isize) = '.' as i32 as libc::c_char;
            } else {
                let fresh14 = k;
                k = k + 1;
                *pr_str.offset(fresh14 as isize) = '\\' as i32 as libc::c_char;
                let fresh15 = k;
                k = k + 1;
                *pr_str.offset(fresh15 as isize) = 'x' as i32 as libc::c_char;
                let fresh16 = k;
                k = k + 1;
                *pr_str
                    .offset(
                        fresh16 as isize,
                    ) = hexmap[(next_ch >> 4 as libc::c_int & 0xf as libc::c_int)
                    as usize];
                let fresh17 = k;
                k = k + 1;
                *pr_str
                    .offset(
                        fresh17 as isize,
                    ) = hexmap[(next_ch & 0xf as libc::c_int) as usize];
            }
            j += 1;
        }
        *pr_str.offset(k as isize) = '\0' as i32 as libc::c_char;
        ii = Tn_insert(TT_print, pr_str, k);
        if i == ii {} else {
            __assert_fail(
                b"i == ii\0" as *const u8 as *const libc::c_char,
                b"T2.c\0" as *const u8 as *const libc::c_char,
                148 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 24],
                    &[libc::c_char; 24],
                >(b"void T2_sync(T2_OBJECT)\0"))
                    .as_ptr(),
            );
        }
        i += 1;
    }
    Sfree(pr_str);
}
