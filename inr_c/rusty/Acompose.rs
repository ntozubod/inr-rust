use ::libc;
extern "C" {
    fn Error(_: *mut libc::c_char);
    fn U_create() -> U_OBJECT;
    fn U_destroy(_: U_OBJECT);
    fn U_insert(
        _: U_OBJECT,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn U_rec(_: U_OBJECT, _: libc::c_int) -> *mut A_row;
    fn A_create() -> A_OBJECT;
    fn A_destroy(_: A_OBJECT);
    fn A_deems(_: A_OBJECT) -> A_OBJECT;
    fn A_add(_: A_OBJECT, _: libc::c_int, _: libc::c_int, _: libc::c_int) -> A_OBJECT;
    fn A_min(_: A_OBJECT) -> A_OBJECT;
    fn A_lambda() -> A_OBJECT;
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
pub struct U_desc {
    pub Type: libc::c_int,
    pub U_n: libc::c_int,
    pub U_lrec: libc::c_int,
    pub U_lhash: libc::c_int,
    pub U_rec: *mut A_row,
    pub U_hash: *mut SHORT,
}
pub type U_OBJECT = *mut U_desc;
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
pub unsafe extern "C" fn A_compose(mut A1: A_OBJECT, mut A2: A_OBJECT) -> A_OBJECT {
    let mut A: A_OBJECT = 0 as *mut A_desc;
    let mut current: libc::c_int = 0;
    let mut s1: libc::c_int = 0;
    let mut s2: libc::c_int = 0;
    let mut t1: libc::c_int = 0;
    let mut t2: libc::c_int = 0;
    let mut cur_a: libc::c_int = 0;
    let mut cur_b: libc::c_int = 0;
    let mut flag: libc::c_int = 0;
    let mut flag2: libc::c_int = 0;
    t1 = 0 as libc::c_int;
    t2 = 0 as libc::c_int;
    let mut p1: *mut A_row = 0 as *mut A_row;
    let mut p1z: *mut A_row = 0 as *mut A_row;
    let mut p2: *mut A_row = 0 as *mut A_row;
    let mut p2z: *mut A_row = 0 as *mut A_row;
    let mut U: U_OBJECT = 0 as *mut U_desc;
    let mut cur_st: *mut A_row = 0 as *mut A_row;
    if !((*A1).A_ems != 0 && (*A2).A_ems != 0) {
        if (*A1).A_ems != 0 {
            A1 = A_deems(A1);
        }
        if (*A2).A_ems != 0 {
            A2 = A_deems(A2);
        }
    }
    if (*A1).A_nT == 1 as libc::c_int && (*A2).A_nT == 1 as libc::c_int {
        A_destroy(A1);
        A_destroy(A2);
        return A_lambda();
    }
    A1 = A_min(A1);
    A2 = A_min(A2);
    A = A_create();
    (*A).A_nT = (*A1).A_nT + (*A2).A_nT - 2 as libc::c_int;
    U = U_create();
    if U_insert(U, 0 as libc::c_int, 0 as libc::c_int, 1 as libc::c_int)
        != 0 as libc::c_int
    {
        Error(
            b"A_compose: BOTCH 1\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if U_insert(U, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int)
        != 1 as libc::c_int
    {
        Error(
            b"A_compose: BOTCH 2\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    current = 0 as libc::c_int;
    while current < (*U).U_n {
        cur_st = U_rec(U, current);
        cur_a = (*cur_st).A_a;
        cur_b = (*cur_st).A_b;
        flag = (*cur_st).A_c;
        flag2 = 0 as libc::c_int;
        if flag != 0 || (*A2).A_nT > 1 as libc::c_int {
            p2 = *((*A2).A_p).offset(cur_b as isize);
            p2z = *((*A2).A_p).offset((cur_b + 1 as libc::c_int) as isize);
            t2 = -(1 as libc::c_int);
            while p2 < p2z {
                if (*A2).A_nT == 2 as libc::c_int {
                    t2 = (*p2).A_b & 1 as libc::c_int;
                } else {
                    t2 = (*p2).A_b % (*A2).A_nT;
                }
                if (*p2).A_b == 1 as libc::c_int {
                    t2 = 0 as libc::c_int;
                }
                if t2 == 0 as libc::c_int {
                    break;
                }
                p2 = p2.offset(1);
            }
            if t2 != 0 as libc::c_int {
                flag2 = 1 as libc::c_int;
            }
        }
        p1 = *((*A1).A_p).offset(cur_a as isize);
        p1z = *((*A1).A_p).offset((cur_a + 1 as libc::c_int) as isize);
        p2 = *((*A2).A_p).offset(cur_b as isize);
        p2z = *((*A2).A_p).offset((cur_b + 1 as libc::c_int) as isize);
        s2 = -(1 as libc::c_int);
        s1 = s2;
        while p1 < p1z || p2 < p2z {
            if s1 < 0 as libc::c_int {
                if p1 < p1z {
                    if (*A1).A_nT == 1 as libc::c_int {
                        s1 = (*p1).A_b;
                        t1 = 0 as libc::c_int;
                    } else if (*A1).A_nT == 2 as libc::c_int {
                        s1 = (*p1).A_b;
                        t1 = s1 & 1 as libc::c_int;
                        s1 >>= 1 as libc::c_int;
                    } else {
                        s1 = (*p1).A_b / (*A1).A_nT;
                        t1 = (*p1).A_b - s1 * (*A1).A_nT;
                    }
                } else {
                    s1 = 0o17777777777 as libc::c_int;
                }
            }
            if s2 < 0 as libc::c_int {
                if p2 < p2z {
                    if (*A2).A_nT == 1 as libc::c_int {
                        s2 = (*p2).A_b;
                        t2 = 0 as libc::c_int;
                    } else if (*A2).A_nT == 2 as libc::c_int {
                        s2 = (*p2).A_b;
                        t2 = s2 & 1 as libc::c_int;
                        s2 >>= 1 as libc::c_int;
                    } else {
                        s2 = (*p2).A_b / (*A2).A_nT;
                        t2 = (*p2).A_b - s2 * (*A2).A_nT;
                    }
                } else {
                    s2 = 0o17777777777 as libc::c_int;
                }
            }
            if (*p1).A_b == 1 as libc::c_int || (*p2).A_b == 1 as libc::c_int {
                if (*p1).A_b == 1 as libc::c_int && (*p2).A_b == 1 as libc::c_int {
                    A = A_add(A, current, 1 as libc::c_int, 1 as libc::c_int);
                }
                if (*p1).A_b == 1 as libc::c_int {
                    p1 = p1.offset(1);
                    s1 = -(1 as libc::c_int);
                }
                if (*p2).A_b == 1 as libc::c_int {
                    p2 = p2.offset(1);
                    s2 = -(1 as libc::c_int);
                }
            } else if s1 <= s2 && t1 < (*A1).A_nT - 1 as libc::c_int {
                if flag2 == 0 && s1 > 0 as libc::c_int {
                    A = A_add(
                        A,
                        current,
                        if (*A).A_nT == 1 as libc::c_int {
                            s1
                        } else if (*A).A_nT == 2 as libc::c_int {
                            (s1 << 1 as libc::c_int) + t1
                        } else {
                            s1 * (*A).A_nT + t1
                        },
                        U_insert(U, (*p1).A_c, cur_b, 0 as libc::c_int),
                    );
                }
                p1 = p1.offset(1);
                s1 = -(1 as libc::c_int);
            } else if s2 < s1 && t2 != 0 as libc::c_int {
                if flag != 0 && s2 > 0 as libc::c_int {
                    A = A_add(
                        A,
                        current,
                        (if (*A).A_nT == 1 as libc::c_int {
                            s2
                        } else {
                            (if (*A).A_nT == 2 as libc::c_int {
                                s2 << 1 as libc::c_int
                            } else {
                                s2 * (*A).A_nT
                            })
                        }) + t2 + (*A1).A_nT - 2 as libc::c_int,
                        U_insert(U, cur_a, (*p2).A_c, 1 as libc::c_int),
                    );
                }
                p2 = p2.offset(1);
                s2 = -(1 as libc::c_int);
            } else if s1 == s2 && t1 == (*A1).A_nT - 1 as libc::c_int
                && t2 == 0 as libc::c_int
            {
                A = A_add(
                    A,
                    current,
                    0 as libc::c_int,
                    U_insert(U, (*p1).A_c, (*p2).A_c, 1 as libc::c_int),
                );
                p1 = p1.offset(1);
                p2 = p2.offset(1);
                s2 = -(1 as libc::c_int);
                s1 = s2;
            } else if s1 <= s2 {
                p1 = p1.offset(1);
                s1 = -(1 as libc::c_int);
            } else {
                p2 = p2.offset(1);
                s2 = -(1 as libc::c_int);
            }
        }
        current += 1;
    }
    A_destroy(A1);
    A_destroy(A2);
    U_destroy(U);
    return A;
}
