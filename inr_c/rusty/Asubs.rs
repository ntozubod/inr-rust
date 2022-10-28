use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn A_clsure(_: A_OBJECT) -> A_OBJECT;
    fn A_close(_: A_OBJECT) -> A_OBJECT;
    fn A_add(_: A_OBJECT, _: libc::c_int, _: libc::c_int, _: libc::c_int) -> A_OBJECT;
    fn A_exchange(_: A_OBJECT, _: A_OBJECT);
    fn A_rept(_: A_OBJECT) -> A_OBJECT;
    fn A_destroy(_: A_OBJECT);
    fn A_create() -> A_OBJECT;
    static mut A_report: libc::c_int;
    fn V_vec(_: V_OBJECT, _: libc::c_int) -> *mut SHORT;
    fn V_insert(_: V_OBJECT, _: *mut SHORT) -> libc::c_int;
    static mut fpout: *mut FILE;
    fn Salloc(_: libc::c_long) -> *mut libc::c_char;
    fn Sfree(_: *mut libc::c_char);
    fn Error(_: *mut libc::c_char);
    fn s_alloc(_: libc::c_int) -> *mut SHORT;
    fn V_create() -> V_OBJECT;
    fn V_destroy(_: V_OBJECT);
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
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
pub unsafe extern "C" fn A_subs(mut A: A_OBJECT) -> A_OBJECT {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut p: *mut A_row = 0 as *mut A_row;
    let mut pz: *mut A_row = 0 as *mut A_row;
    let mut n: libc::c_int = 0;
    let mut hsize: libc::c_int = 0;
    let mut base: libc::c_int = 0;
    let mut head: libc::c_int = 0;
    let mut current: libc::c_int = 0;
    let mut father: libc::c_int = 0;
    let mut son: libc::c_int = 0;
    let mut gap: libc::c_int = 0;
    let mut vlen: libc::c_int = 0;
    let mut insert: *mut A_row = 0 as *mut A_row;
    let mut last: *mut A_row = 0 as *mut A_row;
    let mut heap: *mut *mut A_row = 0 as *mut *mut A_row;
    let mut pnlam: *mut *mut A_row = 0 as *mut *mut A_row;
    let mut set: *mut SHORT = 0 as *mut SHORT;
    let mut vec: *mut SHORT = 0 as *mut SHORT;
    let mut fvec: *mut SHORT = 0 as *mut SHORT;
    let mut An: A_OBJECT = 0 as *mut A_desc;
    let mut V: V_OBJECT = 0 as *mut V_desc;
    if A.is_null() {
        Error(
            b"A_subs: No OBJECT\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if (*A).A_nQ >= 0o17777777777 as libc::c_int - 1 as libc::c_int {
        Error(
            b"A_subs: Too many states\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if (*A).A_mode < 5 as libc::c_int {
        A = A_clsure(A);
    }
    if (*A).A_mode >= 6 as libc::c_int {
        return A;
    }
    if (*A).A_nrows == 0 as libc::c_int {
        (*A).A_mode = 6 as libc::c_int;
        return A;
    }
    if A_report != 0 {
        fprintf(fpout, b"--> A_subs\n\0" as *const u8 as *const libc::c_char);
    }
    set = s_alloc((*A).A_nQ);
    vec = s_alloc((*A).A_nQ + 1 as libc::c_int);
    heap = Salloc(
        (((*A).A_nQ + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut A_row>() as libc::c_ulong)
            as libc::c_long,
    ) as *mut *mut A_row;
    pnlam = Salloc(
        ((*A).A_nQ as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut A_row>() as libc::c_ulong)
            as libc::c_long,
    ) as *mut *mut A_row;
    An = A_create();
    V = V_create();
    (*An).A_nT = (*A).A_nT;
    i = (*A).A_nQ;
    loop {
        i -= 1;
        if !(i >= 0 as libc::c_int) {
            break;
        }
        *set.offset(i as isize) = 0o17777777777 as libc::c_int;
    }
    head = 0o17777777777 as libc::c_int - 1 as libc::c_int;
    vlen = 0 as libc::c_int;
    i = (*A).A_nQ;
    loop {
        i -= 1;
        if !(i >= 0 as libc::c_int) {
            break;
        }
        p = *((*A).A_p).offset(i as isize);
        pz = *((*A).A_p).offset((i + 1 as libc::c_int) as isize);
        while p < pz && (*p).A_b == 0 as libc::c_int {
            p = p.offset(1);
        }
        let ref mut fresh0 = *pnlam.offset(i as isize);
        *fresh0 = p;
    }
    i = 0 as libc::c_int;
    *vec.offset(i as isize) = 0 as libc::c_int;
    pz = *pnlam.offset(0 as libc::c_int as isize);
    p = (*A).A_t;
    while p < pz {
        i += 1;
        *vec.offset(i as isize) = (*p).A_c;
        p = p.offset(1);
    }
    *vec.offset((i + 1 as libc::c_int) as isize) = 0o17777777777 as libc::c_int;
    if V_insert(V, vec) != 0 as libc::c_int {
        Error(
            b"A_subs: BOTCH 1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    *vec.offset(0 as libc::c_int as isize) = 1 as libc::c_int;
    *vec.offset(1 as libc::c_int as isize) = 0o17777777777 as libc::c_int;
    if V_insert(V, vec) != 1 as libc::c_int {
        Error(
            b"A_subs: BOTCH 2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    current = 0 as libc::c_int;
    while current < (*V).V_n {
        hsize = 0 as libc::c_int;
        fvec = V_vec(V, current);
        i = 0 as libc::c_int;
        loop {
            j = *fvec.offset(i as isize);
            if !(j < 0o17777777777 as libc::c_int) {
                break;
            }
            if *pnlam.offset(j as isize)
                != *((*A).A_p).offset((j + 1 as libc::c_int) as isize)
            {
                hsize += 1;
                let ref mut fresh1 = *heap.offset(hsize as isize);
                *fresh1 = *pnlam.offset(j as isize);
            }
            i += 1;
        }
        if !(hsize == 0 as libc::c_int) {
            base = hsize / 2 as libc::c_int;
            while base > 0 as libc::c_int {
                insert = *heap.offset(base as isize);
                father = base;
                loop {
                    son = 2 as libc::c_int * father;
                    if !(son <= hsize) {
                        break;
                    }
                    if son < hsize
                        && (**heap.offset(son as isize)).A_b
                            > (**heap.offset((son + 1 as libc::c_int) as isize)).A_b
                    {
                        son += 1;
                    }
                    if (*insert).A_b <= (**heap.offset(son as isize)).A_b {
                        break;
                    }
                    let ref mut fresh2 = *heap.offset(father as isize);
                    *fresh2 = *heap.offset(son as isize);
                    father = son;
                }
                let ref mut fresh3 = *heap.offset(father as isize);
                *fresh3 = insert;
                base -= 1;
            }
            last = *heap.offset(1 as libc::c_int as isize);
            loop {
                if (*last).A_b != (**heap.offset(1 as libc::c_int as isize)).A_b
                    || hsize == 0 as libc::c_int
                {
                    n = 0 as libc::c_int;
                    if 8 as libc::c_int * vlen < (*A).A_nQ {
                        while head < 0o17777777777 as libc::c_int - 1 as libc::c_int {
                            let fresh4 = n;
                            n = n + 1;
                            *vec.offset(fresh4 as isize) = head;
                            tmp = head;
                            head = *set.offset(head as isize);
                            *set.offset(tmp as isize) = 0o17777777777 as libc::c_int;
                        }
                        gap = n / 2 as libc::c_int;
                        while gap > 0 as libc::c_int {
                            i = gap;
                            while i < n {
                                j = i - gap;
                                while j >= 0 as libc::c_int
                                    && *vec.offset(j as isize) > *vec.offset((j + gap) as isize)
                                {
                                    tmp = *vec.offset(j as isize);
                                    *vec.offset(j as isize) = *vec.offset((j + gap) as isize);
                                    *vec.offset((j + gap) as isize) = tmp;
                                    j -= gap;
                                }
                                i += 1;
                            }
                            gap /= 2 as libc::c_int;
                        }
                    } else {
                        i = 0 as libc::c_int;
                        while i < (*A).A_nQ {
                            if *set.offset(i as isize) != 0o17777777777 as libc::c_int {
                                let fresh5 = n;
                                n = n + 1;
                                let ref mut fresh6 = *vec.offset(fresh5 as isize);
                                *fresh6 = i;
                                *set
                                    .offset(*fresh6 as isize) = 0o17777777777 as libc::c_int;
                            }
                            i += 1;
                        }
                    }
                    head = 0o17777777777 as libc::c_int - 1 as libc::c_int;
                    vlen = 0 as libc::c_int;
                    *vec.offset(n as isize) = 0o17777777777 as libc::c_int;
                    An = A_add(An, current, (*last).A_b, V_insert(V, vec));
                    if hsize == 0 as libc::c_int {
                        break;
                    }
                }
                i = (**heap.offset(1 as libc::c_int as isize)).A_c;
                if *set.offset(i as isize) == 0o17777777777 as libc::c_int {
                    *set.offset(i as isize) = head;
                    head = i;
                    vlen += 1;
                    pz = *pnlam.offset(i as isize);
                    p = *((*A).A_p).offset(i as isize);
                    while p < pz {
                        j = (*p).A_c;
                        if *set.offset(j as isize) == 0o17777777777 as libc::c_int {
                            *set.offset(j as isize) = head;
                            head = j;
                            vlen += 1;
                        }
                        p = p.offset(1);
                    }
                }
                if (*heap.offset(1 as libc::c_int as isize))
                    .offset(1 as libc::c_int as isize)
                    < *((*A).A_p)
                        .offset(
                            ((**heap.offset(1 as libc::c_int as isize)).A_a
                                + 1 as libc::c_int) as isize,
                        )
                {
                    insert = (*heap.offset(1 as libc::c_int as isize))
                        .offset(1 as libc::c_int as isize);
                } else {
                    let fresh7 = hsize;
                    hsize = hsize - 1;
                    insert = *heap.offset(fresh7 as isize);
                }
                last = *heap.offset(1 as libc::c_int as isize);
                father = 1 as libc::c_int;
                loop {
                    son = 2 as libc::c_int * father;
                    if !(son <= hsize) {
                        break;
                    }
                    if son < hsize
                        && (**heap.offset(son as isize)).A_b
                            > (**heap.offset((son + 1 as libc::c_int) as isize)).A_b
                    {
                        son += 1;
                    }
                    if (*insert).A_b <= (**heap.offset(son as isize)).A_b {
                        break;
                    }
                    let ref mut fresh8 = *heap.offset(father as isize);
                    *fresh8 = *heap.offset(son as isize);
                    father = son;
                }
                let ref mut fresh9 = *heap.offset(father as isize);
                *fresh9 = insert;
            }
        }
        current += 1;
    }
    Sfree(set as *mut libc::c_char);
    Sfree(vec as *mut libc::c_char);
    Sfree(heap as *mut libc::c_char);
    Sfree(pnlam as *mut libc::c_char);
    A_exchange(A, An);
    A_destroy(An);
    V_destroy(V);
    A = A_close(A);
    (*A).A_mode = 6 as libc::c_int;
    if A_report != 0 {
        fprintf(fpout, b"<-- A_subs  \0" as *const u8 as *const libc::c_char);
        A_rept(A);
    }
    return A;
}
