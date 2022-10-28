use ::libc;
extern "C" {
    fn Error(_: *mut libc::c_char);
    fn A_create() -> A_OBJECT;
    fn A_destroy(_: A_OBJECT);
    fn A_copy(_: A_OBJECT) -> A_OBJECT;
    fn A_add(_: A_OBJECT, _: libc::c_int, _: libc::c_int, _: libc::c_int) -> A_OBJECT;
    fn A_min(_: A_OBJECT) -> A_OBJECT;
    fn A_lambda() -> A_OBJECT;
    fn A_star(_: A_OBJECT) -> A_OBJECT;
    fn A_alph(_: A_OBJECT) -> A_OBJECT;
    fn A_compose(_: A_OBJECT, _: A_OBJECT) -> A_OBJECT;
    fn A_concat(_: A_OBJECT, _: A_OBJECT) -> A_OBJECT;
}
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
pub unsafe extern "C" fn A_catpow(mut A: A_OBJECT, mut n: libc::c_int) -> A_OBJECT {
    let mut Aprod: A_OBJECT = 0 as *mut A_desc;
    if n < 0 as libc::c_int {
        Error(
            b"A_catpow: negative power\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    A = A_min(A);
    if n % 2 as libc::c_int != 0 {
        Aprod = A_copy(A);
    } else {
        Aprod = A_lambda();
    }
    n /= 2 as libc::c_int;
    while n > 0 as libc::c_int {
        A = A_min(A_concat(A, A_copy(A)));
        if n % 2 as libc::c_int != 0 {
            Aprod = A_min(A_concat(Aprod, A_copy(A)));
        }
        n /= 2 as libc::c_int;
    }
    A_destroy(A);
    return Aprod;
}
#[no_mangle]
pub unsafe extern "C" fn A_ident(mut A: A_OBJECT) -> A_OBJECT {
    let mut A2: A_OBJECT = 0 as *mut A_desc;
    let mut i: libc::c_int = 0;
    let mut sigma: libc::c_int = 0;
    let mut last: libc::c_int = 0;
    A = A_min(A_alph(A));
    A2 = A_create();
    (*A2).A_nT = 2 as libc::c_int;
    last = (*A).A_nrows + 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*A).A_nrows - 1 as libc::c_int {
        sigma = (*((*A).A_t).offset(i as isize)).A_b;
        A2 = A_add(A2, 0 as libc::c_int, sigma * 2 as libc::c_int, i + 2 as libc::c_int);
        A2 = A_add(
            A2,
            i + 2 as libc::c_int,
            sigma * 2 as libc::c_int + 1 as libc::c_int,
            last,
        );
        i += 1;
    }
    A2 = A_add(A2, last, 1 as libc::c_int, 1 as libc::c_int);
    A_destroy(A);
    return A_star(A2);
}
#[no_mangle]
pub unsafe extern "C" fn A_cmpow(mut A: A_OBJECT, mut n: libc::c_int) -> A_OBJECT {
    let mut Aprod: A_OBJECT = 0 as *mut A_desc;
    if n < 0 as libc::c_int {
        Error(
            b"A_cmpow: negative power\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    A = A_min(A);
    if n % 2 as libc::c_int != 0 {
        Aprod = A_copy(A);
    } else {
        Aprod = A_ident(A_copy(A));
    }
    n /= 2 as libc::c_int;
    while n > 0 as libc::c_int {
        A = A_min(A_compose(A, A_copy(A)));
        if n % 2 as libc::c_int != 0 {
            Aprod = A_min(A_compose(Aprod, A_copy(A)));
        }
        n /= 2 as libc::c_int;
    }
    A_destroy(A);
    return Aprod;
}
