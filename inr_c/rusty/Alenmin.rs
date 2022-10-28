use ::libc;
extern "C" {
    fn Sfree(_: *mut libc::c_char);
    fn s_alloc(_: libc::c_int) -> *mut SHORT;
    fn A_open(_: A_OBJECT) -> A_OBJECT;
    fn A_trim(_: A_OBJECT) -> A_OBJECT;
    fn A_subs(_: A_OBJECT) -> A_OBJECT;
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
pub unsafe extern "C" fn A_lenmin(mut A: A_OBJECT) -> A_OBJECT {
    let mut lo: *mut A_row = 0 as *mut A_row;
    let mut hi: *mut A_row = 0 as *mut A_row;
    let mut p: *mut A_row = 0 as *mut A_row;
    let mut i: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut cur: libc::c_int = 0;
    let mut dead: libc::c_int = 0;
    let mut bfs: *mut SHORT = 0 as *mut SHORT;
    let mut dis: *mut SHORT = 0 as *mut SHORT;
    if (*A).A_nrows == 0 as libc::c_int {
        return A;
    }
    A = A_subs(A);
    bfs = s_alloc((*A).A_nQ);
    dis = s_alloc((*A).A_nQ);
    i = 0 as libc::c_int;
    while i < (*A).A_nQ {
        *dis.offset(i as isize) = 0o17777777777 as libc::c_int;
        i += 1;
    }
    *bfs.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    l = 1 as libc::c_int;
    *dis.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < l {
        cur = *bfs.offset(i as isize);
        lo = *((*A).A_p).offset(cur as isize);
        hi = *((*A).A_p).offset((cur + 1 as libc::c_int) as isize);
        p = lo;
        while p < hi {
            if *dis.offset((*p).A_c as isize)
                > *dis.offset(cur as isize) + 1 as libc::c_int
            {
                *dis
                    .offset(
                        (*p).A_c as isize,
                    ) = *dis.offset(cur as isize) + 1 as libc::c_int;
                let fresh0 = l;
                l = l + 1;
                *bfs.offset(fresh0 as isize) = (*p).A_c;
            }
            p = p.offset(1);
        }
        i += 1;
    }
    A = A_open(A);
    lo = (*A).A_t;
    hi = lo.offset((*A).A_nrows as isize);
    let ref mut fresh1 = (*A).A_nQ;
    let fresh2 = *fresh1;
    *fresh1 = *fresh1 + 1;
    dead = fresh2;
    p = lo;
    while p < hi {
        if *dis.offset((*p).A_a as isize) >= *dis.offset((*p).A_c as isize) {
            (*p).A_c = dead;
        }
        p = p.offset(1);
    }
    Sfree(bfs as *mut libc::c_char);
    Sfree(dis as *mut libc::c_char);
    return A_trim(A);
}
#[no_mangle]
pub unsafe extern "C" fn A_minlen(mut A: A_OBJECT) -> libc::c_int {
    let mut lo: *mut A_row = 0 as *mut A_row;
    let mut hi: *mut A_row = 0 as *mut A_row;
    let mut p: *mut A_row = 0 as *mut A_row;
    let mut i: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut cur: libc::c_int = 0;
    let mut bfs: *mut SHORT = 0 as *mut SHORT;
    let mut dis: *mut SHORT = 0 as *mut SHORT;
    if (*A).A_nrows == 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    A = A_subs(A);
    bfs = s_alloc((*A).A_nQ);
    dis = s_alloc((*A).A_nQ);
    i = 0 as libc::c_int;
    while i < (*A).A_nQ {
        *dis.offset(i as isize) = 0o17777777777 as libc::c_int;
        i += 1;
    }
    *bfs.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    l = 1 as libc::c_int;
    *dis.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < l {
        cur = *bfs.offset(i as isize);
        lo = *((*A).A_p).offset(cur as isize);
        hi = *((*A).A_p).offset((cur + 1 as libc::c_int) as isize);
        p = lo;
        while p < hi {
            if *dis.offset((*p).A_c as isize) == 0o17777777777 as libc::c_int {
                *dis
                    .offset(
                        (*p).A_c as isize,
                    ) = *dis.offset(cur as isize) + 1 as libc::c_int;
                let fresh3 = l;
                l = l + 1;
                *bfs.offset(fresh3 as isize) = (*p).A_c;
            }
            p = p.offset(1);
        }
        i += 1;
    }
    i = *dis.offset(1 as libc::c_int as isize) - 1 as libc::c_int;
    Sfree(bfs as *mut libc::c_char);
    Sfree(dis as *mut libc::c_char);
    return i;
}
