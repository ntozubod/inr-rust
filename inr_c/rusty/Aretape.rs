use ::libc;
extern "C" {
    fn Error(_: *mut libc::c_char);
    fn T2_name(_: T2_OBJECT, _: libc::c_int) -> *mut libc::c_char;
    fn A_destroy(_: A_OBJECT);
    fn A_deems(_: A_OBJECT) -> A_OBJECT;
    fn A_add(_: A_OBJECT, _: libc::c_int, _: libc::c_int, _: libc::c_int) -> A_OBJECT;
    fn A_open(_: A_OBJECT) -> A_OBJECT;
    fn A_close(_: A_OBJECT) -> A_OBJECT;
    fn A_min(_: A_OBJECT) -> A_OBJECT;
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
pub unsafe extern "C" fn A_retape(
    mut A1: A_OBJECT,
    mut A2: A_OBJECT,
    mut T2: T2_OBJECT,
) -> A_OBJECT {
    let mut i: libc::c_int = 0;
    let mut p: *mut A_row = 0 as *mut A_row;
    let mut sigma: libc::c_int = 0;
    let mut tape: libc::c_int = 0;
    let mut ntapes: libc::c_int = 0;
    let mut old: libc::c_int = 0;
    let mut base: libc::c_int = 0;
    let mut i1: libc::c_int = 0;
    let mut i2: libc::c_int = 0;
    let mut chp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut trans: [libc::c_int; 100] = [0; 100];
    let mut tp: [libc::c_int; 100] = [0; 100];
    if (*A1).A_ems != 0 {
        A1 = A_deems(A1);
    }
    if A1.is_null() || A2.is_null() {
        Error(
            b"A_rename: NULL Automaton\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if (*A1).A_nT > 10 as libc::c_int || (*A2).A_nT > 10 as libc::c_int {
        Error(
            b"A_rename: Too many tapes\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    A2 = A_min(A2);
    let ref mut fresh0 = (*((*A2).A_t).offset(0 as libc::c_int as isize)).A_a;
    *fresh0 += 1 as libc::c_int;
    let ref mut fresh1 = (*((*A2).A_t)
        .offset(((*A2).A_nrows - 1 as libc::c_int) as isize))
        .A_c;
    *fresh1 += (*A2).A_nrows;
    i = 0 as libc::c_int;
    while i < (*A2).A_nrows {
        if (*((*A2).A_t).offset(i as isize)).A_a != i + 1 as libc::c_int
            || (*((*A2).A_t).offset(i as isize)).A_c != i + 2 as libc::c_int
        {
            Error(
                b"A_retape: improper form for second argument\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        i += 1;
    }
    ntapes = (*A2).A_nT;
    tp[0 as libc::c_int as usize] = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*A1).A_nT {
        tp[(i + 1 as libc::c_int) as usize] = tp[i as usize];
        p = (*A2).A_t;
        while p
            < ((*A2).A_t)
                .offset((*A2).A_nrows as isize)
                .offset(-(1 as libc::c_int as isize))
        {
            sigma = (*p).A_b / (*A2).A_nT;
            tape = (*p).A_b - sigma * (*A2).A_nT;
            chp = T2_name(T2, sigma);
            sigma = *chp.offset(0 as libc::c_int as isize) as libc::c_int - '0' as i32;
            if *chp.offset(1 as libc::c_int as isize) as libc::c_int != 0
                || sigma < 0 as libc::c_int || sigma > 9 as libc::c_int
            {
                Error(
                    b"A_retape: Second argument\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
            if !(sigma != i) {
                trans[tp[(i + 1 as libc::c_int) as usize] as usize] = tape;
                tp[(i + 1 as libc::c_int) as usize] += 1;
            }
            p = p.offset(1);
        }
        i += 1;
    }
    A_destroy(A2);
    A1 = A_open(A1);
    i = (*A1).A_nrows;
    loop {
        i -= 1;
        if !(i >= 0 as libc::c_int) {
            break;
        }
        p = ((*A1).A_t).offset(i as isize);
        if (*p).A_b <= 1 as libc::c_int {
            continue;
        }
        sigma = (*p).A_b / (*A1).A_nT;
        tape = (*p).A_b - sigma * (*A1).A_nT;
        if sigma == 1 as libc::c_int
            || tp[tape as usize] == tp[(tape + 1 as libc::c_int) as usize]
        {
            (*p).A_b = 0 as libc::c_int;
        } else {
            i1 = tp[tape as usize];
            i2 = tp[(tape + 1 as libc::c_int) as usize] - 1 as libc::c_int;
            (*p).A_b = sigma * ntapes + trans[i1 as usize];
            if i1 < i2 {
                old = (*p).A_c;
                base = (*A1).A_nQ - i1;
                (*p).A_c = base + i1;
                loop {
                    i1 += 1;
                    if !(i1 < i2) {
                        break;
                    }
                    A1 = A_add(
                        A1,
                        base + i1 - 1 as libc::c_int,
                        sigma * ntapes + trans[i1 as usize],
                        base + i1,
                    );
                }
                A1 = A_add(
                    A1,
                    base + i1 - 1 as libc::c_int,
                    sigma * ntapes + trans[i1 as usize],
                    old,
                );
            }
        }
    }
    (*A1).A_nT = ntapes;
    A1 = A_close(A1);
    return A1;
}
#[no_mangle]
pub unsafe extern "C" fn A_comma(mut A1: A_OBJECT, mut A2: A_OBJECT) -> A_OBJECT {
    let mut p: *mut A_row = 0 as *mut A_row;
    let mut sigma: libc::c_int = 0;
    let mut tape: libc::c_int = 0;
    let mut ntapes: libc::c_int = 0;
    if (*A1).A_ems != 0 {
        A1 = A_deems(A1);
    }
    if (*A2).A_ems != 0 {
        A2 = A_deems(A2);
    }
    if A1.is_null() || A2.is_null() {
        Error(
            b"A_comma: NULL Automaton\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    ntapes = (*A1).A_nT + (*A2).A_nT;
    if ntapes > 10 as libc::c_int {
        Error(
            b"A_comma: Too many tapes\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    A2 = A_open(A2);
    p = ((*A2).A_t).offset((*A2).A_nrows as isize);
    loop {
        p = p.offset(-1);
        if !(p >= (*A2).A_t) {
            break;
        }
        if (*p).A_b <= 1 as libc::c_int {
            continue;
        }
        sigma = (*p).A_b / (*A2).A_nT;
        tape = (*p).A_b - sigma * (*A2).A_nT + (*A1).A_nT;
        (*p).A_b = sigma * ntapes + tape;
    }
    (*A2).A_nT = ntapes;
    return A_concat(A1, A2);
}
