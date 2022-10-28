use ::libc;
extern "C" {
    fn copymem(_: libc::c_long, _: *mut libc::c_char, _: *mut libc::c_char);
    fn Salloc(_: libc::c_long) -> *mut libc::c_char;
    fn Sfree(_: *mut libc::c_char);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct P_desc {
    pub Type: libc::c_int,
    pub P_length: libc::c_int,
    pub P_cstr: *mut libc::c_char,
}
pub type P_OBJECT = *mut P_desc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Q_desc {
    pub Type: libc::c_int,
    pub Q_tapeno: libc::c_int,
    pub Q_length: libc::c_int,
    pub Q_cstr: *mut libc::c_char,
}
pub type Q_OBJECT = *mut Q_desc;
#[no_mangle]
pub unsafe extern "C" fn Q_create(
    mut tapeno: libc::c_int,
    mut length: libc::c_int,
    mut cstr: *mut libc::c_char,
) -> Q_OBJECT {
    let mut Q: Q_OBJECT = 0 as *mut Q_desc;
    Q = Salloc(::std::mem::size_of::<Q_desc>() as libc::c_ulong as libc::c_long)
        as Q_OBJECT;
    (*Q).Type = 7 as libc::c_int;
    (*Q).Q_tapeno = tapeno;
    (*Q).Q_length = length;
    let ref mut fresh0 = (*Q).Q_cstr;
    *fresh0 = Salloc((length + 1 as libc::c_int) as libc::c_long);
    copymem(length as libc::c_long, cstr, (*Q).Q_cstr);
    *((*Q).Q_cstr).offset(length as isize) = '\0' as i32 as libc::c_char;
    return Q;
}
#[no_mangle]
pub unsafe extern "C" fn Q_fromP(mut P: P_OBJECT) -> Q_OBJECT {
    let mut Q: Q_OBJECT = 0 as *mut Q_desc;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut l: libc::c_int = (*P).P_length;
    let mut s: *mut libc::c_char = (*P).P_cstr;
    let fresh1 = i;
    i = i + 1;
    let mut c: libc::c_int = *s.offset(fresh1 as isize) as libc::c_int;
    let mut tapeno: libc::c_int = 0 as libc::c_int;
    Q = Salloc(::std::mem::size_of::<Q_desc>() as libc::c_ulong as libc::c_long)
        as Q_OBJECT;
    (*Q).Type = 7 as libc::c_int;
    while i <= l && c >= '0' as i32 && c <= '9' as i32 {
        tapeno = tapeno * 10 as libc::c_int + (c - '0' as i32);
        let fresh2 = i;
        i = i + 1;
        c = *s.offset(fresh2 as isize) as libc::c_int;
    }
    if i < l && c == '.' as i32
        && (*s.offset(0 as libc::c_int as isize) as libc::c_int != '0' as i32
            || i <= 2 as libc::c_int)
    {
        l -= i;
        copymem(l as libc::c_long, s.offset(i as isize), s);
        *s.offset(l as isize) = '\0' as i32 as libc::c_char;
    } else {
        tapeno = -(1 as libc::c_int);
    }
    (*Q).Q_tapeno = tapeno;
    (*Q).Q_length = l;
    let ref mut fresh3 = (*Q).Q_cstr;
    *fresh3 = s;
    Sfree(P as *mut libc::c_char);
    return Q;
}
#[no_mangle]
pub unsafe extern "C" fn Q_destroy(mut Q: Q_OBJECT) {
    Sfree((*Q).Q_cstr);
    Sfree(Q as *mut libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn Q_tapeno(mut Q: Q_OBJECT) -> libc::c_int {
    return (*Q).Q_tapeno;
}
#[no_mangle]
pub unsafe extern "C" fn Q_length(mut Q: Q_OBJECT) -> libc::c_int {
    return (*Q).Q_length;
}
#[no_mangle]
pub unsafe extern "C" fn Q_cstr(mut Q: Q_OBJECT) -> *mut libc::c_char {
    return (*Q).Q_cstr;
}
