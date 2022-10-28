use ::libc;
extern "C" {
    fn Sfree(_: *mut libc::c_char);
    fn Error(_: *mut libc::c_char);
    fn s_alloc(_: libc::c_int) -> *mut SHORT;
    fn veccpy(_: *mut SHORT, _: *mut SHORT) -> *mut SHORT;
    fn veclen(_: *mut SHORT) -> libc::c_int;
    fn V_create() -> V_OBJECT;
    fn V_destroy(_: V_OBJECT);
    fn V_insert(_: V_OBJECT, _: *mut SHORT) -> libc::c_int;
    fn V_vec(_: V_OBJECT, _: libc::c_int) -> *mut SHORT;
    fn A_create() -> A_OBJECT;
    fn A_destroy(_: A_OBJECT);
    fn A_exchange(_: A_OBJECT, _: A_OBJECT);
    fn A_add(_: A_OBJECT, _: libc::c_int, _: libc::c_int, _: libc::c_int) -> A_OBJECT;
    fn A_mkdense(_: A_OBJECT) -> A_OBJECT;
    fn A_min(_: A_OBJECT) -> A_OBJECT;
    fn A_sseq(_: A_OBJECT) -> A_OBJECT;
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
pub struct V_desc {
    pub Type: libc::c_int,
    pub V_n: libc::c_int,
    pub V_lvec: libc::c_int,
    pub V_lhash: libc::c_int,
    pub V_vec: *mut *mut SHORT,
    pub V_hash: *mut SHORT,
}
pub type V_OBJECT = *mut V_desc;
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
pub unsafe extern "C" fn A_clsseq(mut A1: A_OBJECT) -> A_OBJECT {
    let mut A: A_OBJECT = 0 as *mut A_desc;
    let mut current: libc::c_int = 0;
    let mut end_st: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut bb: libc::c_int = 0;
    let mut last_label: libc::c_int = 0;
    let mut label: libc::c_int = 0;
    let mut hi_next: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    last_label = 0 as libc::c_int;
    let mut vec: *mut SHORT = 0 as *mut SHORT;
    let mut cur_vec: *mut SHORT = 0 as *mut SHORT;
    let mut V: V_OBJECT = 0 as *mut V_desc;
    let mut p: *mut A_row = 0 as *mut A_row;
    let mut pz: *mut A_row = 0 as *mut A_row;
    let mut lo: *mut A_row = 0 as *mut A_row;
    let mut hi: *mut A_row = 0 as *mut A_row;
    let mut mid: *mut A_row = 0 as *mut A_row;
    A1 = A_sseq(A1);
    A = A_create();
    (*A).A_nT = 2 as libc::c_int;
    V = V_create();
    vec = s_alloc(101 as libc::c_int);
    *vec.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    *vec.offset(1 as libc::c_int as isize) = 0o17777777777 as libc::c_int;
    if V_insert(V, vec) != 0 as libc::c_int {
        Error(
            b"A_clsseq: BOTCH 1\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    *vec.offset(0 as libc::c_int as isize) = 1 as libc::c_int;
    *vec.offset(1 as libc::c_int as isize) = 1 as libc::c_int;
    *vec.offset(2 as libc::c_int as isize) = 0o17777777777 as libc::c_int;
    if V_insert(V, vec) != 1 as libc::c_int {
        Error(
            b"A_clsseq: BOTCH 2\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    hi_next = 0o17777777777 as libc::c_int - 1 as libc::c_int;
    current = 0 as libc::c_int;
    while current < (*V).V_n {
        if !(current == 1 as libc::c_int) {
            cur_vec = V_vec(V, current);
            end_st = *cur_vec.offset((veclen(cur_vec) - 1 as libc::c_int) as isize);
            pz = *((*A1).A_p)
                .offset(
                    (*cur_vec.offset(0 as libc::c_int as isize) + 1 as libc::c_int)
                        as isize,
                );
            p = *((*A1).A_p).offset(*cur_vec.offset(0 as libc::c_int as isize) as isize);
            while p < pz {
                if (*p).A_b == 1 as libc::c_int {
                    A = A_add(A, current, 1 as libc::c_int, 1 as libc::c_int);
                } else {
                    veccpy(vec, cur_vec);
                    label = (*p).A_b / 2 as libc::c_int;
                    i = 0 as libc::c_int;
                    k = 0 as libc::c_int;
                    if label != 1 as libc::c_int {
                        k = 2 as libc::c_int * label;
                    }
                    let fresh0 = hi_next;
                    hi_next = hi_next - 1;
                    A = A_add(A, current, k, fresh0);
                    if hi_next < (*V).V_n {
                        Error(
                            b"A_clsseq: Too many states\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        );
                    }
                    while i >= 0 as libc::c_int {
                        lo = *((*A1).A_p).offset(*vec.offset(i as isize) as isize);
                        hi = (*((*A1).A_p)
                            .offset(
                                (*vec.offset(i as isize) + 1 as libc::c_int) as isize,
                            ))
                            .offset(-(1 as libc::c_int as isize));
                        if lo > hi || (*lo).A_b == 1 as libc::c_int
                            || (*lo).A_b % 2 as libc::c_int == 0 as libc::c_int
                        {
                            if label != 0 {
                                bb = 2 as libc::c_int * label;
                                while lo < hi {
                                    mid = lo
                                        .offset(
                                            (hi.offset_from(lo) as libc::c_long
                                                / 2 as libc::c_int as libc::c_long) as isize,
                                        );
                                    if bb <= (*mid).A_b {
                                        hi = mid;
                                    } else {
                                        lo = mid.offset(1 as libc::c_int as isize);
                                    }
                                }
                                if lo == hi && (*lo).A_b == bb {
                                    *vec.offset(i as isize) = (*lo).A_c;
                                } else {
                                    *vec.offset(i as isize) = 1 as libc::c_int;
                                }
                                last_label = label;
                                label = 0 as libc::c_int;
                            } else {
                                if i >= 98 as libc::c_int {
                                    Error(
                                        b"A_clsseq: State explosion\0" as *const u8
                                            as *const libc::c_char as *mut libc::c_char,
                                    );
                                }
                                if *vec.offset((i + 1 as libc::c_int) as isize)
                                    == 0o17777777777 as libc::c_int
                                {
                                    *vec.offset((i + 1 as libc::c_int) as isize) = end_st;
                                    *vec
                                        .offset(
                                            (i + 2 as libc::c_int) as isize,
                                        ) = 0o17777777777 as libc::c_int;
                                }
                                i -= 1;
                            }
                        } else {
                            *vec.offset(i as isize) = (*lo).A_c;
                            i += 1;
                            if i >= 100 as libc::c_int {
                                Error(
                                    b"A_clsseq: State explosion\0" as *const u8
                                        as *const libc::c_char as *mut libc::c_char,
                                );
                            }
                            label = (*lo).A_b / 2 as libc::c_int;
                            if *vec.offset(i as isize) == 0o17777777777 as libc::c_int {
                                if i >= 2 as libc::c_int
                                    && *vec.offset((i - 1 as libc::c_int) as isize)
                                        == *vec.offset((i - 2 as libc::c_int) as isize)
                                    && label == last_label
                                {
                                    k = 0 as libc::c_int;
                                    if label != 1 as libc::c_int {
                                        k = 2 as libc::c_int * label + 1 as libc::c_int;
                                    }
                                    A = A_add(A, hi_next + 1 as libc::c_int, k, hi_next);
                                    hi_next -= 1;
                                    if hi_next < (*V).V_n {
                                        Error(
                                            b"A_clsseq: Too many states\0" as *const u8
                                                as *const libc::c_char as *mut libc::c_char,
                                        );
                                    }
                                    i -= 1;
                                    label = 0 as libc::c_int;
                                } else {
                                    *vec.offset(i as isize) = end_st;
                                    *vec
                                        .offset(
                                            (i + 1 as libc::c_int) as isize,
                                        ) = 0o17777777777 as libc::c_int;
                                }
                            }
                        }
                    }
                    i = veclen(vec);
                    while i > 1 as libc::c_int
                        && *vec.offset((i - 1 as libc::c_int) as isize)
                            == *vec.offset((i - 2 as libc::c_int) as isize)
                    {
                        i -= 1;
                    }
                    *vec.offset(i as isize) = 0o17777777777 as libc::c_int;
                    i = 0 as libc::c_int;
                    while *vec.offset(i as isize)
                        == *vec.offset((i + 1 as libc::c_int) as isize)
                    {
                        i += 1;
                    }
                    k = V_insert(V, vec.offset(i as isize));
                    A = A_add(A, hi_next + 1 as libc::c_int, 0 as libc::c_int, k);
                }
                p = p.offset(1);
            }
        }
        current += 1;
    }
    A_exchange(A, A1);
    A_destroy(A);
    V_destroy(V);
    Sfree(vec as *mut libc::c_char);
    A1 = A_min(A_mkdense(A1));
    (*A1).A_mode = 9 as libc::c_int;
    (*A1).A_ems = 1 as libc::c_int;
    return A1;
}
