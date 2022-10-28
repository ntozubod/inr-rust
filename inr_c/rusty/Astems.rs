use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn Salloc(_: libc::c_long) -> *mut libc::c_char;
    fn Sfree(_: *mut libc::c_char);
    fn Srealloc(_: *mut libc::c_char, _: libc::c_long) -> *mut libc::c_char;
    fn Error(_: *mut libc::c_char);
    fn s_alloc(_: libc::c_int) -> *mut SHORT;
    fn T2_name_pr(_: T2_OBJECT, _: libc::c_int) -> *mut libc::c_char;
    fn A_destroy(_: A_OBJECT);
    fn A_copy(_: A_OBJECT) -> A_OBJECT;
    static mut s_rena: *mut SHORT;
    static mut f_rena: libc::c_int;
    fn A_open(_: A_OBJECT) -> A_OBJECT;
    fn A_lameq(_: A_OBJECT) -> A_OBJECT;
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
static mut GAs: A_OBJECT = 0 as *const A_desc as *mut A_desc;
static mut st_len: *mut SHORT = 0 as *const SHORT as *mut SHORT;
static mut st_closed: *mut SHORT = 0 as *const SHORT as *mut SHORT;
static mut st_work: *mut *mut SHORT = 0 as *const *mut SHORT as *mut *mut SHORT;
static mut st_ptr: *mut *mut SHORT = 0 as *const *mut SHORT as *mut *mut SHORT;
static mut st_c_work: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn A_st_free() {
    let mut i: libc::c_int = 0;
    if st_work.is_null() {
        return;
    }
    if st_ptr != st_work {
        Sfree(st_ptr as *mut libc::c_char);
    }
    i = 0 as libc::c_int;
    while i < st_c_work {
        Sfree(*st_work.offset(i as isize) as *mut libc::c_char);
        i += 1;
    }
    Sfree(st_work as *mut libc::c_char);
    st_work = 0 as *mut *mut SHORT;
    st_ptr = st_work;
}
#[no_mangle]
pub unsafe extern "C" fn A_st_DFS(
    mut q: libc::c_int,
    mut i: libc::c_int,
) -> libc::c_int {
    let mut p: *mut A_row = 0 as *mut A_row;
    let mut try_0: libc::c_int = 0;
    let mut newtry: libc::c_int = 0;
    if i < *st_len.offset(q as isize) {
        return *(*st_work.offset(q as isize)).offset(i as isize)
    } else {
        if *st_closed.offset(q as isize) != 0 {
            return 0o17777777777 as libc::c_int;
        }
    }
    if i > 0 as libc::c_int
        && A_st_DFS(q, i - 1 as libc::c_int) == 0o17777777777 as libc::c_int
    {
        return 0o17777777777 as libc::c_int;
    }
    try_0 = 0o17777777777 as libc::c_int - 1 as libc::c_int;
    p = *((*GAs).A_p).offset(q as isize);
    while p < *((*GAs).A_p).offset((q + 1 as libc::c_int) as isize) {
        if (*p).A_b == 0 as libc::c_int {
            newtry = A_st_DFS((*p).A_c, i);
        } else if i > 0 as libc::c_int {
            newtry = A_st_DFS((*p).A_c, i - 1 as libc::c_int);
        } else {
            newtry = (*p).A_b;
        }
        if try_0 == 0o17777777777 as libc::c_int - 1 as libc::c_int || try_0 == newtry {
            try_0 = newtry;
            p = p.offset(1);
        } else {
            try_0 = 0o17777777777 as libc::c_int;
            *st_closed.offset(q as isize) = 1 as libc::c_int;
            break;
        }
    }
    if try_0 == 0o17777777777 as libc::c_int - 1 as libc::c_int {
        try_0 = 0o17777777777 as libc::c_int;
    }
    let ref mut fresh0 = *st_len.offset(q as isize);
    *fresh0 += 1;
    let ref mut fresh1 = *st_work.offset(q as isize);
    *fresh1 = Srealloc(
        *st_work.offset(q as isize) as *mut libc::c_char,
        (*st_len.offset(q as isize) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<SHORT>() as libc::c_ulong)
            as libc::c_long,
    ) as *mut SHORT;
    if i != *st_len.offset(q as isize) - 1 as libc::c_int {
        Error(
            b"A_stems: Bounds check\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    *(*st_work.offset(q as isize)).offset(i as isize) = try_0;
    return try_0;
}
#[no_mangle]
pub unsafe extern "C" fn A_stems(
    mut A: A_OBJECT,
    mut tape: libc::c_int,
) -> *mut *mut SHORT {
    let mut q: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut Aw: A_OBJECT = 0 as *mut A_desc;
    let mut p: *mut A_row = 0 as *mut A_row;
    A_st_free();
    if A.is_null() {
        Error(
            b"A_stems: No OBJECT\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if tape < 0 as libc::c_int || tape > (*A).A_nT {
        Error(
            b"A_stems: tape out of range\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    Aw = A_copy(A);
    if (*Aw).A_nT > 1 as libc::c_int {
        Aw = A_open(Aw);
        p = ((*Aw).A_t).offset((*Aw).A_nrows as isize);
        loop {
            p = p.offset(-1);
            if !(p >= (*Aw).A_t) {
                break;
            }
            if (*p).A_b > 1 as libc::c_int && (*p).A_b % (*Aw).A_nT != tape {
                (*p).A_b = 0 as libc::c_int;
            } else if (*p).A_b > 1 as libc::c_int {
                (*p).A_b = (*p).A_b / (*Aw).A_nT;
            }
        }
        (*Aw).A_nT = 1 as libc::c_int;
    }
    Sfree(s_rena as *mut libc::c_char);
    s_rena = 0 as *mut SHORT;
    f_rena = 1 as libc::c_int;
    if (*Aw).A_mode < 3 as libc::c_int {
        Aw = A_lameq(Aw);
    }
    f_rena = 0 as libc::c_int;
    st_work = Salloc(
        ((*Aw).A_nQ as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut SHORT>() as libc::c_ulong)
            as libc::c_long,
    ) as *mut *mut SHORT;
    st_c_work = (*Aw).A_nQ;
    st_len = s_alloc((*Aw).A_nQ);
    st_closed = s_alloc((*Aw).A_nQ);
    i = (*Aw).A_nQ;
    loop {
        i -= 1;
        if !(i >= 0 as libc::c_int) {
            break;
        }
        let ref mut fresh2 = *st_work.offset(i as isize);
        *fresh2 = s_alloc(1 as libc::c_int);
        *(*st_work.offset(i as isize))
            .offset(0 as libc::c_int as isize) = 0o17777777777 as libc::c_int;
        *st_len.offset(i as isize) = 0 as libc::c_int;
        *st_closed.offset(i as isize) = 0 as libc::c_int;
    }
    GAs = Aw;
    q = (*Aw).A_nQ;
    loop {
        q -= 1;
        if !(q >= 0 as libc::c_int) {
            break;
        }
        i = 0 as libc::c_int;
        while A_st_DFS(q, i) != 0o17777777777 as libc::c_int {
            i += 1;
        }
    }
    Sfree(st_len as *mut libc::c_char);
    Sfree(st_closed as *mut libc::c_char);
    A_destroy(Aw);
    if s_rena.is_null() {
        st_ptr = st_work;
    } else {
        st_ptr = Salloc(
            ((*A).A_nQ as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut SHORT>() as libc::c_ulong)
                as libc::c_long,
        ) as *mut *mut SHORT;
        i = 0 as libc::c_int;
        while i < (*A).A_nQ {
            if *s_rena.offset(i as isize) < 0o17777777777 as libc::c_int {
                let ref mut fresh3 = *st_ptr.offset(i as isize);
                *fresh3 = *st_work.offset(*s_rena.offset(i as isize) as isize);
            } else {
                let ref mut fresh4 = *st_ptr.offset(i as isize);
                *fresh4 = *st_work.offset(1 as libc::c_int as isize);
            }
            i += 1;
        }
    }
    Sfree(s_rena as *mut libc::c_char);
    s_rena = 0 as *mut SHORT;
    return st_ptr;
}
#[no_mangle]
pub unsafe extern "C" fn A_prstems(
    mut A: A_OBJECT,
    mut T2: T2_OBJECT,
    mut tape: libc::c_int,
) {
    let mut q: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ch: libc::c_int = 0;
    if A.is_null() {
        Error(
            b"A_prstems: No OBJECT\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    A_stems(A, tape);
    let mut current_block_11: u64;
    q = 0 as libc::c_int;
    while q < (*A).A_nQ {
        if q == 0 as libc::c_int {
            printf(b"  (START)  \0" as *const u8 as *const libc::c_char);
            current_block_11 = 8515828400728868193;
        } else if q == 1 as libc::c_int {
            current_block_11 = 17778012151635330486;
        } else {
            printf(b"%9d  \0" as *const u8 as *const libc::c_char, q);
            current_block_11 = 8515828400728868193;
        }
        match current_block_11 {
            8515828400728868193 => {
                i = 0 as libc::c_int;
                while *(*st_ptr.offset(q as isize)).offset(i as isize)
                    < 0o17777777777 as libc::c_int
                {
                    ch = *(*st_ptr.offset(q as isize)).offset(i as isize);
                    printf(
                        b" %s\0" as *const u8 as *const libc::c_char,
                        T2_name_pr(T2, ch),
                    );
                    i += 1;
                }
                printf(b"\n\0" as *const u8 as *const libc::c_char);
            }
            _ => {}
        }
        q += 1;
    }
    A_st_free();
}
