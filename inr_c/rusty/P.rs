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
pub unsafe extern "C" fn P_create(
    mut length: libc::c_int,
    mut cstr: *mut libc::c_char,
) -> P_OBJECT {
    let mut P: P_OBJECT = 0 as *mut P_desc;
    P = Salloc(::std::mem::size_of::<P_desc>() as libc::c_ulong as libc::c_long)
        as P_OBJECT;
    (*P).Type = 6 as libc::c_int;
    (*P).P_length = length;
    let ref mut fresh0 = (*P).P_cstr;
    *fresh0 = Salloc((length + 1 as libc::c_int) as libc::c_long);
    copymem(length as libc::c_long, cstr, (*P).P_cstr);
    *((*P).P_cstr).offset(length as isize) = '\0' as i32 as libc::c_char;
    return P;
}
#[no_mangle]
pub unsafe extern "C" fn P_fromQ(mut Q: Q_OBJECT) -> P_OBJECT {
    let mut P: P_OBJECT = 0 as *mut P_desc;
    P = Salloc(::std::mem::size_of::<P_desc>() as libc::c_ulong as libc::c_long)
        as P_OBJECT;
    (*P).Type = 6 as libc::c_int;
    (*P).P_length = (*P).P_length;
    let ref mut fresh1 = (*P).P_cstr;
    *fresh1 = (*P).P_cstr;
    Sfree(Q as *mut libc::c_char);
    return P;
}
#[no_mangle]
pub unsafe extern "C" fn P_destroy(mut P: P_OBJECT) {
    Sfree((*P).P_cstr);
    Sfree(P as *mut libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn P_length(mut P: P_OBJECT) -> libc::c_int {
    return (*P).P_length;
}
#[no_mangle]
pub unsafe extern "C" fn P_cstr(mut P: P_OBJECT) -> *mut libc::c_char {
    return (*P).P_cstr;
}
