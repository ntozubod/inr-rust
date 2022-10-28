use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut fpout: *mut FILE;
    fn Salloc(_: libc::c_long) -> *mut libc::c_char;
    fn Sfree(_: *mut libc::c_char);
    fn Error(_: *mut libc::c_char);
    fn s_alloc(_: libc::c_int) -> *mut SHORT;
    static mut A_report: libc::c_int;
    fn A_rept(_: A_OBJECT) -> A_OBJECT;
    fn A_open(_: A_OBJECT) -> A_OBJECT;
    fn A_close(_: A_OBJECT) -> A_OBJECT;
    fn A_rename(_: A_OBJECT, _: *mut SHORT) -> A_OBJECT;
    fn A_subs(_: A_OBJECT) -> A_OBJECT;
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
pub unsafe extern "C" fn A_min(mut A: A_OBJECT) -> A_OBJECT {
    let mut p: *mut A_row = 0 as *mut A_row;
    let mut f: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ns: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut lo: *mut A_row = 0 as *mut A_row;
    let mut hi: *mut A_row = 0 as *mut A_row;
    let mut mode: libc::c_int = 0;
    let mut nB: libc::c_int = 0;
    let mut wait_H: libc::c_int = 0;
    let mut JL_H: libc::c_int = 0;
    let mut hsize: libc::c_int = 0;
    let mut in_B: *mut SHORT = 0 as *mut SHORT;
    let mut B_card: *mut SHORT = 0 as *mut SHORT;
    let mut B_H: *mut SHORT = 0 as *mut SHORT;
    let mut B_N: *mut SHORT = 0 as *mut SHORT;
    let mut B_L: *mut SHORT = 0 as *mut SHORT;
    let mut in_wait: *mut SHORT = 0 as *mut SHORT;
    let mut wait_N: *mut SHORT = 0 as *mut SHORT;
    let mut in_JL: *mut SHORT = 0 as *mut SHORT;
    let mut JL_N: *mut SHORT = 0 as *mut SHORT;
    let mut int_H: *mut SHORT = 0 as *mut SHORT;
    let mut int_N: *mut SHORT = 0 as *mut SHORT;
    let mut heap: *mut *mut A_row = 0 as *mut *mut A_row;
    if A.is_null() {
        Error(
            b"A_min: No OBJECT\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if (*A).A_mode < 6 as libc::c_int {
        A = A_subs(A);
    }
    mode = (*A).A_mode;
    if mode == 7 as libc::c_int || mode == 9 as libc::c_int {
        return A;
    }
    if mode == 6 as libc::c_int {
        mode = 7 as libc::c_int;
    }
    if mode == 8 as libc::c_int {
        mode = 9 as libc::c_int;
    }
    if (*A).A_nrows == 0 as libc::c_int || (*A).A_nQ <= 2 as libc::c_int {
        (*A).A_mode = mode;
        return A;
    }
    if A_report != 0 {
        fprintf(fpout, b"--> A_min\n\0" as *const u8 as *const libc::c_char);
    }
    A = A_open(A);
    p = ((*A).A_t).offset((*A).A_nrows as isize);
    loop {
        p = p.offset(-1);
        if !(p >= (*A).A_t) {
            break;
        }
        s = (*p).A_a;
        (*p).A_a = (*p).A_c;
        (*p).A_c = s;
    }
    A = A_close(A);
    nB = (*A).A_nQ;
    in_B = s_alloc((*A).A_nQ);
    B_card = s_alloc(nB);
    B_H = s_alloc(nB);
    B_N = s_alloc((*A).A_nQ);
    B_L = s_alloc((*A).A_nQ);
    in_wait = s_alloc(nB);
    wait_N = s_alloc(nB);
    in_JL = s_alloc(nB);
    JL_N = s_alloc(nB);
    int_H = s_alloc(nB);
    int_N = s_alloc((*A).A_nQ);
    heap = Salloc(
        (((*A).A_nQ + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut A_row>() as libc::c_ulong)
            as libc::c_long,
    ) as *mut *mut A_row;
    nB = 2 as libc::c_int;
    let ref mut fresh0 = *in_B.offset(2 as libc::c_int as isize);
    *fresh0 = 0 as libc::c_int;
    *in_B.offset(0 as libc::c_int as isize) = *fresh0;
    *B_card.offset(0 as libc::c_int as isize) = (*A).A_nQ - 1 as libc::c_int;
    *B_H.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    *B_N.offset(0 as libc::c_int as isize) = 2 as libc::c_int;
    *B_L.offset(2 as libc::c_int as isize) = 0 as libc::c_int;
    *B_N.offset(((*A).A_nQ - 1 as libc::c_int) as isize) = 0 as libc::c_int;
    *B_L.offset(0 as libc::c_int as isize) = (*A).A_nQ - 1 as libc::c_int;
    s = (*A).A_nQ;
    loop {
        s -= 1;
        if !(s >= 3 as libc::c_int) {
            break;
        }
        *in_B.offset(s as isize) = 0 as libc::c_int;
        *B_N.offset((s - 1 as libc::c_int) as isize) = s;
        *B_L.offset(s as isize) = s - 1 as libc::c_int;
    }
    *in_B.offset(1 as libc::c_int as isize) = 1 as libc::c_int;
    *B_card.offset(1 as libc::c_int as isize) = 1 as libc::c_int;
    let ref mut fresh1 = *B_L.offset(1 as libc::c_int as isize);
    *fresh1 = 1 as libc::c_int;
    let ref mut fresh2 = *B_N.offset(1 as libc::c_int as isize);
    *fresh2 = *fresh1;
    *B_H.offset(1 as libc::c_int as isize) = *fresh2;
    *in_wait.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    *in_wait.offset(1 as libc::c_int as isize) = 0 as libc::c_int;
    wait_H = 1 as libc::c_int;
    *wait_N.offset(1 as libc::c_int as isize) = 0 as libc::c_int;
    *wait_N
        .offset(
            0 as libc::c_int as isize,
        ) = 0o17777777777 as libc::c_int - 1 as libc::c_int;
    *in_JL.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    *in_JL.offset(1 as libc::c_int as isize) = 0 as libc::c_int;
    s = (*A).A_nQ;
    loop {
        s -= 1;
        if !(s >= 0 as libc::c_int) {
            break;
        }
        *int_N.offset(s as isize) = 0o17777777777 as libc::c_int;
    }
    while wait_H < 0o17777777777 as libc::c_int - 1 as libc::c_int {
        b = wait_H;
        x = *in_wait.offset(b as isize);
        hsize = 0 as libc::c_int;
        p = 0 as *mut A_row;
        s = *B_H.offset(b as isize);
        while s != *B_H.offset(b as isize) || p.is_null() {
            lo = *((*A).A_p).offset(s as isize);
            p = lo;
            hi = (*((*A).A_p).offset((s + 1 as libc::c_int) as isize))
                .offset(-(1 as libc::c_int as isize));
            if lo <= hi && x <= (*hi).A_b {
                while lo < hi {
                    if x <= (*p).A_b {
                        hi = p;
                    } else {
                        lo = p.offset(1 as libc::c_int as isize);
                    }
                    p = lo
                        .offset(
                            (hi.offset_from(lo) as libc::c_long
                                / 2 as libc::c_int as libc::c_long) as isize,
                        );
                }
                hsize += 1;
                let ref mut fresh3 = *heap.offset(hsize as isize);
                *fresh3 = lo;
            }
            s = *B_N.offset(s as isize);
        }
        if hsize == 0 as libc::c_int {
            *in_wait
                .offset(b as isize) = 0o17777777777 as libc::c_int - 1 as libc::c_int;
            wait_H = *wait_N.offset(wait_H as isize);
        } else {
            j = hsize / 2 as libc::c_int;
            while j > 0 as libc::c_int {
                lo = *heap.offset(j as isize);
                f = j;
                loop {
                    s = 2 as libc::c_int * f;
                    if !(s <= hsize) {
                        break;
                    }
                    if s < hsize
                        && (**heap.offset(s as isize)).A_b
                            > (**heap.offset((s + 1 as libc::c_int) as isize)).A_b
                    {
                        s += 1;
                    }
                    if (*lo).A_b <= (**heap.offset(s as isize)).A_b {
                        break;
                    }
                    let ref mut fresh4 = *heap.offset(f as isize);
                    *fresh4 = *heap.offset(s as isize);
                    f = s;
                }
                let ref mut fresh5 = *heap.offset(f as isize);
                *fresh5 = lo;
                j -= 1;
            }
            x = (**heap.offset(1 as libc::c_int as isize)).A_b;
            *in_wait.offset(b as isize) = x + 1 as libc::c_int;
            JL_H = 0o17777777777 as libc::c_int - 1 as libc::c_int;
            loop {
                if x != (**heap.offset(1 as libc::c_int as isize)).A_b
                    || hsize == 0 as libc::c_int
                {
                    j = JL_H;
                    while j < 0o17777777777 as libc::c_int - 1 as libc::c_int {
                        if *in_JL.offset(j as isize) < *B_card.offset(j as isize) {
                            *B_H
                                .offset(
                                    nB as isize,
                                ) = 0o17777777777 as libc::c_int - 1 as libc::c_int;
                            *B_card.offset(nB as isize) = 0 as libc::c_int;
                            *in_wait
                                .offset(
                                    nB as isize,
                                ) = 0o17777777777 as libc::c_int - 1 as libc::c_int;
                            s = *int_H.offset(j as isize);
                            while s < 0o17777777777 as libc::c_int - 1 as libc::c_int {
                                *in_B.offset(s as isize) = nB;
                                let ref mut fresh6 = *B_card.offset(j as isize);
                                *fresh6 -= 1;
                                let ref mut fresh7 = *B_card.offset(nB as isize);
                                *fresh7 += 1;
                                let ref mut fresh8 = *B_N
                                    .offset(*B_L.offset(s as isize) as isize);
                                *fresh8 = *B_N.offset(s as isize);
                                let ref mut fresh9 = *B_H.offset(j as isize);
                                *fresh9 = *fresh8;
                                *B_L.offset(*fresh9 as isize) = *B_L.offset(s as isize);
                                *B_N.offset(s as isize) = *B_H.offset(nB as isize);
                                *B_H.offset(nB as isize) = s;
                                ns = *int_N.offset(s as isize);
                                *int_N.offset(s as isize) = 0o17777777777 as libc::c_int;
                                s = ns;
                            }
                            s = *B_H.offset(nB as isize);
                            loop {
                                ns = *B_N.offset(s as isize);
                                if !(ns < 0o17777777777 as libc::c_int - 1 as libc::c_int) {
                                    break;
                                }
                                *B_L.offset(ns as isize) = s;
                                s = ns;
                            }
                            let ref mut fresh10 = *B_N.offset(s as isize);
                            *fresh10 = *B_H.offset(nB as isize);
                            *B_L.offset(*fresh10 as isize) = s;
                            *in_JL.offset(nB as isize) = 0 as libc::c_int;
                            if *in_wait.offset(j as isize)
                                < 0o17777777777 as libc::c_int - 1 as libc::c_int
                                || *B_card.offset(nB as isize) <= *B_card.offset(j as isize)
                            {
                                *in_wait.offset(nB as isize) = 0 as libc::c_int;
                                *wait_N.offset(nB as isize) = wait_H;
                                wait_H = nB;
                            }
                            if *B_card.offset(nB as isize) > *B_card.offset(j as isize) {
                                if *in_wait.offset(j as isize)
                                    < 0o17777777777 as libc::c_int - 1 as libc::c_int
                                {
                                    *in_wait.offset(nB as isize) = *in_wait.offset(j as isize);
                                    *in_wait.offset(j as isize) = 0 as libc::c_int;
                                    if j == b {
                                        hsize = 0 as libc::c_int;
                                    }
                                } else {
                                    *in_wait.offset(j as isize) = 0 as libc::c_int;
                                    *wait_N.offset(j as isize) = wait_H;
                                    wait_H = j;
                                }
                            }
                            nB += 1;
                        } else {
                            s = *int_H.offset(j as isize);
                            while s < 0o17777777777 as libc::c_int - 1 as libc::c_int {
                                ns = *int_N.offset(s as isize);
                                *int_N.offset(s as isize) = 0o17777777777 as libc::c_int;
                                s = ns;
                            }
                        }
                        *in_JL.offset(j as isize) = 0 as libc::c_int;
                        j = *JL_N.offset(j as isize);
                    }
                    if hsize == 0 as libc::c_int {
                        break;
                    }
                    x = (**heap.offset(1 as libc::c_int as isize)).A_b;
                    *in_wait.offset(b as isize) = x + 1 as libc::c_int;
                    JL_H = 0o17777777777 as libc::c_int - 1 as libc::c_int;
                }
                if *in_B.offset((**heap.offset(1 as libc::c_int as isize)).A_a as isize)
                    == b
                {
                    s = (**heap.offset(1 as libc::c_int as isize)).A_c;
                    if *int_N.offset(s as isize) == 0o17777777777 as libc::c_int {
                        j = *in_B.offset(s as isize);
                        if *in_JL.offset(j as isize) == 0 as libc::c_int {
                            *JL_N.offset(j as isize) = JL_H;
                            JL_H = j;
                            *int_H
                                .offset(
                                    j as isize,
                                ) = 0o17777777777 as libc::c_int - 1 as libc::c_int;
                        }
                        let ref mut fresh11 = *in_JL.offset(j as isize);
                        *fresh11 += 1;
                        *int_N.offset(s as isize) = *int_H.offset(j as isize);
                        *int_H.offset(j as isize) = s;
                    }
                    if (*heap.offset(1 as libc::c_int as isize))
                        .offset(1 as libc::c_int as isize)
                        < *((*A).A_p)
                            .offset(
                                ((**heap.offset(1 as libc::c_int as isize)).A_a
                                    + 1 as libc::c_int) as isize,
                            )
                    {
                        lo = (*heap.offset(1 as libc::c_int as isize))
                            .offset(1 as libc::c_int as isize);
                    } else {
                        let fresh12 = hsize;
                        hsize = hsize - 1;
                        lo = *heap.offset(fresh12 as isize);
                    }
                } else {
                    let fresh13 = hsize;
                    hsize = hsize - 1;
                    lo = *heap.offset(fresh13 as isize);
                }
                f = 1 as libc::c_int;
                loop {
                    s = 2 as libc::c_int * f;
                    if !(s <= hsize) {
                        break;
                    }
                    if s < hsize
                        && (**heap.offset(s as isize)).A_b
                            > (**heap.offset((s + 1 as libc::c_int) as isize)).A_b
                    {
                        s += 1;
                    }
                    if (*lo).A_b <= (**heap.offset(s as isize)).A_b {
                        break;
                    }
                    let ref mut fresh14 = *heap.offset(f as isize);
                    *fresh14 = *heap.offset(s as isize);
                    f = s;
                }
                let ref mut fresh15 = *heap.offset(f as isize);
                *fresh15 = lo;
            }
        }
    }
    Sfree(B_card as *mut libc::c_char);
    Sfree(B_H as *mut libc::c_char);
    Sfree(B_N as *mut libc::c_char);
    Sfree(B_L as *mut libc::c_char);
    Sfree(in_wait as *mut libc::c_char);
    Sfree(wait_N as *mut libc::c_char);
    Sfree(in_JL as *mut libc::c_char);
    Sfree(JL_N as *mut libc::c_char);
    Sfree(int_H as *mut libc::c_char);
    Sfree(int_N as *mut libc::c_char);
    Sfree(heap as *mut libc::c_char);
    A = A_open(A);
    p = ((*A).A_t).offset((*A).A_nrows as isize);
    loop {
        p = p.offset(-1);
        if !(p >= (*A).A_t) {
            break;
        }
        s = (*p).A_a;
        (*p).A_a = (*p).A_c;
        (*p).A_c = s;
    }
    A = A_close(A);
    A = A_rename(A, in_B);
    Sfree(in_B as *mut libc::c_char);
    (*A).A_mode = mode;
    if A_report != 0 {
        fprintf(fpout, b"<-- A_min  \0" as *const u8 as *const libc::c_char);
        A_rept(A);
    }
    return A;
}
