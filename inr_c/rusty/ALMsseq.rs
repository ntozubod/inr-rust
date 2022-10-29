use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn Sfree(_: *mut libc::c_char);
    static mut fpout: *mut FILE;
    fn Salloc(_: libc::c_long) -> *mut libc::c_char;
    fn Error(_: *mut libc::c_char);
    fn s_alloc(_: libc::c_int) -> *mut SHORT;
    fn T2_name_pr(_: T2_OBJECT, _: libc::c_int) -> *mut libc::c_char;
    fn veccpy(_: *mut SHORT, _: *mut SHORT) -> *mut SHORT;
    fn veccmp(_: *mut SHORT, _: *mut SHORT) -> libc::c_int;
    fn veclen(_: *mut SHORT) -> libc::c_int;
    fn V_create() -> V_OBJECT;
    fn V_destroy(_: V_OBJECT);
    fn V_insert(_: V_OBJECT, _: *mut SHORT) -> libc::c_int;
    fn V_vec(_: V_OBJECT, _: libc::c_int) -> *mut SHORT;
    static mut A_report: libc::c_int;
    fn A_create() -> A_OBJECT;
    fn A_destroy(_: A_OBJECT);
    fn A_rept(_: A_OBJECT) -> A_OBJECT;
    fn A_exchange(_: A_OBJECT, _: A_OBJECT);
    fn A_add(_: A_OBJECT, _: libc::c_int, _: libc::c_int, _: libc::c_int) -> A_OBJECT;
    fn A_open(_: A_OBJECT) -> A_OBJECT;
    fn A_close(_: A_OBJECT) -> A_OBJECT;
    fn A_mkdense(_: A_OBJECT) -> A_OBJECT;
    fn Srealloc(_: *mut libc::c_char, _: libc::c_long) -> *mut libc::c_char;
    fn A_min(_: A_OBJECT) -> A_OBJECT;
    fn A_st_free();
    fn A_stems(_: A_OBJECT, _: libc::c_int) -> *mut *mut SHORT;
    static mut TT2: T2_OBJECT;
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
pub unsafe extern "C" fn A_LMsseq(mut A: A_OBJECT) -> A_OBJECT {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut p: *mut A_row = 0 as *mut A_row;
    let mut pz: *mut A_row = 0 as *mut A_row;
    let mut tt: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut hsize: libc::c_int = 0;
    let mut base: libc::c_int = 0;
    let mut head: libc::c_int = 0;
    let mut current: libc::c_int = 0;
    let mut father: libc::c_int = 0;
    let mut son: libc::c_int = 0;
    let mut gap: libc::c_int = 0;
    let mut vlen: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut sig_lim: libc::c_int = 0;
    let mut aa: libc::c_int = 0;
    let mut bb: libc::c_int = 0;
    let mut cc: libc::c_int = 0;
    let mut nq: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut from: libc::c_int = 0;
    let mut to: libc::c_int = 0;
    let mut label: libc::c_int = 0;
    let mut hi_next: libc::c_int = 0;
    let mut try_0: libc::c_int = 0;
    label = 0 as libc::c_int;
    from = 0 as libc::c_int;
    let mut insert: *mut A_row = 0 as *mut A_row;
    let mut last: *mut A_row = 0 as *mut A_row;
    let mut heap: *mut *mut A_row = 0 as *mut *mut A_row;
    let mut set: *mut SHORT = 0 as *mut SHORT;
    let mut vec: *mut SHORT = 0 as *mut SHORT;
    let mut fvec: *mut SHORT = 0 as *mut SHORT;
    let mut fr_coeff: *mut *mut SHORT = 0 as *mut *mut SHORT;
    let mut to_coeff: *mut *mut SHORT = 0 as *mut *mut SHORT;
    let mut queue: *mut SHORT = 0 as *mut SHORT;
    let mut st_len: *mut SHORT = 0 as *mut SHORT;
    let mut st_ptr: *mut *mut SHORT = 0 as *mut *mut SHORT;
    let mut save_coeff: *mut SHORT = 0 as *mut SHORT;
    let mut sig: *mut SHORT = 0 as *mut SHORT;
    let mut back: *mut SHORT = 0 as *mut SHORT;
    let mut An: A_OBJECT = 0 as *mut A_desc;
    let mut V: V_OBJECT = 0 as *mut V_desc;
    let mut Vs: V_OBJECT = 0 as *mut V_desc;
    let mut Vsig: V_OBJECT = 0 as *mut V_desc;
    if A.is_null() {
        Error(
            b"A_sseq: No OBJECT\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if (*A).A_nQ >= 0o17777777777 as libc::c_int - 1 as libc::c_int {
        Error(
            b"A_sseq: Too many states\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    A = A_close(A);
    if (*A).A_mode < 7 as libc::c_int {
        A = A_min(A);
    }
    if (*A).A_nrows == 0 as libc::c_int {
        (*A).A_mode = 6 as libc::c_int;
        return A;
    }
    if (*A).A_nT != 2 as libc::c_int {
        Error(
            b"A_sseq: Not transducer\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    tmp = 0 as libc::c_int;
    i = (*A).A_nrows;
    loop {
        i -= 1;
        if !(i >= 0 as libc::c_int) {
            break;
        }
        if (*((*A).A_t).offset(i as isize)).A_b / 2 as libc::c_int == 1 as libc::c_int {
            tmp = 1 as libc::c_int;
            (*((*A).A_t).offset(i as isize)).A_b = 0 as libc::c_int;
        }
    }
    if tmp != 0 {
        A = A_min(A_open(A));
    }
    if A_report != 0 {
        fprintf(fpout, b"--> A_sseq\n\0" as *const u8 as *const libc::c_char);
    }
    set = s_alloc((*A).A_nQ);
    vec = s_alloc(2 as libc::c_int * (*A).A_nQ + 1 as libc::c_int);
    heap = Salloc(
        (((*A).A_nQ + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut A_row>() as libc::c_ulong)
            as libc::c_long,
    ) as *mut *mut A_row;
    fr_coeff = Salloc(
        ((*A).A_nQ as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut SHORT>() as libc::c_ulong)
            as libc::c_long,
    ) as *mut *mut SHORT;
    to_coeff = Salloc(
        ((*A).A_nQ as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut SHORT>() as libc::c_ulong)
            as libc::c_long,
    ) as *mut *mut SHORT;
    queue = s_alloc((*A).A_nQ);
    st_len = s_alloc((*A).A_nQ);
    sig_lim = 20 as libc::c_int;
    sig = s_alloc(sig_lim);
    back = s_alloc(sig_lim);
    An = A_create();
    V = V_create();
    Vs = V_create();
    Vsig = V_create();
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
    st_ptr = A_stems(A, 1 as libc::c_int);
    k = 0 as libc::c_int;
    while k < (*A).A_nQ {
        *st_len.offset(k as isize) = veclen(*st_ptr.offset(k as isize));
        k += 1;
    }
    hi_next = 0o17777777777 as libc::c_int - 2 as libc::c_int;
    *set.offset(0 as libc::c_int as isize) = head;
    head = 0 as libc::c_int;
    vlen += 1;
    len = veclen(*st_ptr.offset(0 as libc::c_int as isize));
    let ref mut fresh0 = *to_coeff.offset(0 as libc::c_int as isize);
    *fresh0 = s_alloc(len + 1 as libc::c_int);
    veccpy(
        *to_coeff.offset(0 as libc::c_int as isize),
        *st_ptr.offset(0 as libc::c_int as isize),
    );
    *queue.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    nq = 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i < nq {
        pz = *((*A).A_p).offset((*queue.offset(i as isize) + 1 as libc::c_int) as isize);
        p = *((*A).A_p).offset(*queue.offset(i as isize) as isize);
        while p < pz {
            if (*p).A_b != 1 as libc::c_int
                && (*p).A_b % 2 as libc::c_int == 1 as libc::c_int
            {
                if !((*p).A_a == (*p).A_c) {
                    if *set.offset((*p).A_c as isize) == 0o17777777777 as libc::c_int {
                        *set.offset((*p).A_c as isize) = head;
                        head = (*p).A_c;
                        vlen += 1;
                        save_coeff = 0 as *mut SHORT;
                        let fresh1 = nq;
                        nq = nq + 1;
                        *queue.offset(fresh1 as isize) = (*p).A_c;
                    } else {
                        save_coeff = *to_coeff.offset((*p).A_c as isize);
                    }
                    len = veclen(*to_coeff.offset((*p).A_a as isize)) + 1 as libc::c_int
                        - *st_len.offset((*p).A_a as isize)
                        + *st_len.offset((*p).A_c as isize);
                    let ref mut fresh2 = *to_coeff.offset((*p).A_c as isize);
                    *fresh2 = s_alloc(len + 1 as libc::c_int);
                    veccpy(
                        *to_coeff.offset((*p).A_c as isize),
                        *to_coeff.offset((*p).A_a as isize),
                    );
                    if *st_len.offset((*p).A_a as isize) == 0 as libc::c_int {
                        *(*to_coeff.offset((*p).A_c as isize))
                            .offset(
                                (len - *st_len.offset((*p).A_c as isize) - 1 as libc::c_int)
                                    as isize,
                            ) = (*p).A_b / 2 as libc::c_int;
                        veccpy(
                            (*to_coeff.offset((*p).A_c as isize))
                                .offset((len - *st_len.offset((*p).A_c as isize)) as isize),
                            *st_ptr.offset((*p).A_c as isize),
                        );
                    }
                    if !save_coeff.is_null() {
                        if veccmp(save_coeff, *to_coeff.offset((*p).A_c as isize)) != 0
                            && veclen(save_coeff)
                                == veclen(*to_coeff.offset((*p).A_c as isize))
                        {
                            Error(
                                b"A_sseq: Relation not single-valued (S3)\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char,
                            );
                        }
                        if veclen(save_coeff)
                            >= veclen(*to_coeff.offset((*p).A_c as isize))
                        {
                            Sfree(save_coeff as *mut libc::c_char);
                        } else {
                            Sfree(
                                *to_coeff.offset((*p).A_c as isize) as *mut libc::c_char,
                            );
                            let ref mut fresh3 = *to_coeff.offset((*p).A_c as isize);
                            *fresh3 = save_coeff;
                        }
                    }
                }
            }
            p = p.offset(1);
        }
        i += 1;
    }
    n = 0 as libc::c_int;
    head = 0o17777777777 as libc::c_int - 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*A).A_nQ {
        if *set.offset(i as isize) != 0o17777777777 as libc::c_int {
            *set.offset(i as isize) = 0o17777777777 as libc::c_int;
            pz = *((*A).A_p).offset((i + 1 as libc::c_int) as isize);
            p = *((*A).A_p).offset(i as isize);
            while p < pz {
                if (*p).A_b == 1 as libc::c_int
                    || (*p).A_b % 2 as libc::c_int == 0 as libc::c_int
                {
                    break;
                }
                p = p.offset(1);
            }
            if i == 1 as libc::c_int || p < pz {
                let fresh4 = n;
                n = n + 1;
                *vec.offset(fresh4 as isize) = i;
            } else {
                Sfree(*to_coeff.offset(i as isize) as *mut libc::c_char);
            }
        }
        i += 1;
    }
    vlen = n;
    i = 0 as libc::c_int;
    while i < vlen {
        *vec
            .offset(
                (i + vlen) as isize,
            ) = V_insert(Vs, *to_coeff.offset(*vec.offset(i as isize) as isize));
        i += 1;
    }
    *vec.offset((vlen + vlen) as isize) = 0o17777777777 as libc::c_int;
    if V_insert(V, vec) != 0 as libc::c_int {
        Error(
            b"A_sseq: BOTCH 1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    current = 0 as libc::c_int;
    if V_insert(V, vec.offset(vlen as isize).offset(vlen as isize)) != 1 as libc::c_int {
        Error(
            b"A_sseq: BOTCH 2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    to = 0 as libc::c_int;
    j = 0 as libc::c_int;
    's_571: loop {
        try_0 = *(*to_coeff.offset(*vec.offset(0 as libc::c_int as isize) as isize))
            .offset(j as isize);
        if try_0 == 0o17777777777 as libc::c_int {
            break;
        }
        i = 1 as libc::c_int;
        while i < vlen {
            if *(*to_coeff.offset(*vec.offset(i as isize) as isize)).offset(j as isize)
                != try_0
            {
                break 's_571;
            }
            i += 1;
        }
        if to != 0 as libc::c_int {
            An = A_add(An, from, label, to);
        }
        from = to;
        label = 2 as libc::c_int * try_0 + 1 as libc::c_int;
        let fresh5 = hi_next;
        hi_next = hi_next - 1;
        to = fresh5;
        j += 1;
    }
    if j > 0 as libc::c_int {
        hi_next += 1;
        i = 0 as libc::c_int;
        while i < vlen {
            *vec
                .offset(
                    (i + vlen) as isize,
                ) = V_insert(
                Vs,
                (*to_coeff.offset(*vec.offset(i as isize) as isize)).offset(j as isize),
            );
            i += 1;
        }
        *vec.offset((vlen + vlen) as isize) = 0o17777777777 as libc::c_int;
        current = V_insert(V, vec);
        An = A_add(An, from, label, current);
    }
    i = 0 as libc::c_int;
    while i < vlen {
        Sfree(*to_coeff.offset(*vec.offset(i as isize) as isize) as *mut libc::c_char);
        i += 1;
    }
    vlen = 0 as libc::c_int;
    *vec.offset(0 as libc::c_int as isize) = 1 as libc::c_int;
    *vec.offset(2 as libc::c_int as isize) = 0o17777777777 as libc::c_int;
    *vec
        .offset(
            1 as libc::c_int as isize,
        ) = V_insert(Vs, vec.offset(2 as libc::c_int as isize));
    An = A_add(An, V_insert(V, vec), 1 as libc::c_int, 1 as libc::c_int);
    let ref mut fresh6 = *sig.offset(2 as libc::c_int as isize);
    *fresh6 = 0o17777777777 as libc::c_int;
    let ref mut fresh7 = *sig.offset(1 as libc::c_int as isize);
    *fresh7 = *fresh6;
    *sig.offset(0 as libc::c_int as isize) = *fresh7;
    let ref mut fresh8 = *back.offset(2 as libc::c_int as isize);
    *fresh8 = 0 as libc::c_int;
    let ref mut fresh9 = *back.offset(1 as libc::c_int as isize);
    *fresh9 = *fresh8;
    *back.offset(0 as libc::c_int as isize) = *fresh9;
    while current < (*V).V_n {
        if !(current == 1 as libc::c_int) {
            hsize = 0 as libc::c_int;
            fvec = V_vec(V, current);
            len = veclen(fvec) / 2 as libc::c_int;
            i = 0 as libc::c_int;
            while i < len {
                j = *fvec.offset(i as isize);
                if *((*A).A_p).offset(j as isize)
                    != *((*A).A_p).offset((j + 1 as libc::c_int) as isize)
                {
                    hsize += 1;
                    let ref mut fresh10 = *heap.offset(hsize as isize);
                    *fresh10 = *((*A).A_p).offset(j as isize);
                }
                let ref mut fresh11 = *fr_coeff.offset(j as isize);
                *fresh11 = V_vec(Vs, *fvec.offset((len + i) as isize));
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
                        let ref mut fresh12 = *heap.offset(father as isize);
                        *fresh12 = *heap.offset(son as isize);
                        father = son;
                    }
                    let ref mut fresh13 = *heap.offset(father as isize);
                    *fresh13 = insert;
                    base -= 1;
                }
                last = *heap.offset(1 as libc::c_int as isize);
                loop {
                    if ((*last).A_b != (**heap.offset(1 as libc::c_int as isize)).A_b
                        || hsize == 0 as libc::c_int) && vlen > 0 as libc::c_int
                    {
                        n = 0 as libc::c_int;
                        if 8 as libc::c_int * vlen < (*A).A_nQ {
                            vlen = 0 as libc::c_int;
                            while head < 0o17777777777 as libc::c_int - 1 as libc::c_int
                            {
                                pz = *((*A).A_p).offset((head + 1 as libc::c_int) as isize);
                                p = *((*A).A_p).offset(head as isize);
                                while p < pz {
                                    if (*p).A_b == 1 as libc::c_int
                                        || (*p).A_b % 2 as libc::c_int == 0 as libc::c_int
                                    {
                                        break;
                                    }
                                    p = p.offset(1);
                                }
                                if head == 1 as libc::c_int || p < pz {
                                    let fresh14 = n;
                                    n = n + 1;
                                    *vec.offset(fresh14 as isize) = head;
                                } else {
                                    Sfree(*to_coeff.offset(head as isize) as *mut libc::c_char);
                                }
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
                                    *set.offset(i as isize) = 0o17777777777 as libc::c_int;
                                    pz = *((*A).A_p).offset((i + 1 as libc::c_int) as isize);
                                    p = *((*A).A_p).offset(i as isize);
                                    while p < pz {
                                        if (*p).A_b == 1 as libc::c_int
                                            || (*p).A_b % 2 as libc::c_int == 0 as libc::c_int
                                        {
                                            break;
                                        }
                                        p = p.offset(1);
                                    }
                                    if i == 1 as libc::c_int || p < pz {
                                        let fresh15 = n;
                                        n = n + 1;
                                        *vec.offset(fresh15 as isize) = i;
                                    } else {
                                        Sfree(*to_coeff.offset(i as isize) as *mut libc::c_char);
                                    }
                                }
                                i += 1;
                            }
                        }
                        vlen = n;
                        head = 0o17777777777 as libc::c_int - 1 as libc::c_int;
                        from = current;
                        label = (*last).A_b;
                        if label == 1 as libc::c_int {
                            label = 2 as libc::c_int;
                        }
                        to = hi_next;
                        j = 0 as libc::c_int;
                        's_1069: loop {
                            try_0 = *(*to_coeff
                                .offset(*vec.offset(0 as libc::c_int as isize) as isize))
                                .offset(j as isize);
                            if try_0 == 0o17777777777 as libc::c_int {
                                break;
                            }
                            i = 1 as libc::c_int;
                            while i < vlen {
                                if *(*to_coeff.offset(*vec.offset(i as isize) as isize))
                                    .offset(j as isize) != try_0
                                {
                                    break 's_1069;
                                }
                                i += 1;
                            }
                            An = A_add(An, from, label, to);
                            from = to;
                            label = 2 as libc::c_int * try_0 + 1 as libc::c_int;
                            hi_next -= 1;
                            to = hi_next;
                            if to < (*V).V_n {
                                Error(
                                    b"A_sseq: Overflow\0" as *const u8 as *const libc::c_char
                                        as *mut libc::c_char,
                                );
                            }
                            j += 1;
                        }
                        i = 0 as libc::c_int;
                        while i < vlen {
                            *vec
                                .offset(
                                    (i + vlen) as isize,
                                ) = V_insert(
                                Vs,
                                (*to_coeff.offset(*vec.offset(i as isize) as isize))
                                    .offset(j as isize),
                            );
                            Sfree(
                                *to_coeff.offset(*vec.offset(i as isize) as isize)
                                    as *mut libc::c_char,
                            );
                            i += 1;
                        }
                        *vec
                            .offset(
                                (vlen + vlen) as isize,
                            ) = 0o17777777777 as libc::c_int;
                        tmp = (*V).V_n;
                        to = V_insert(V, vec);
                        An = A_add(An, from, label, to);
                        if to == tmp {
                            *vec.offset(vlen as isize) = 0o17777777777 as libc::c_int;
                            tmp = (*Vsig).V_n;
                            i = V_insert(Vsig, vec);
                            if to >= sig_lim {
                                sig_lim *= 2 as libc::c_int;
                                sig = Srealloc(
                                    sig as *mut libc::c_char,
                                    (sig_lim as libc::c_ulong)
                                        .wrapping_mul(
                                            ::std::mem::size_of::<SHORT>() as libc::c_ulong,
                                        ) as libc::c_long,
                                ) as *mut SHORT;
                                back = Srealloc(
                                    back as *mut libc::c_char,
                                    (sig_lim as libc::c_ulong)
                                        .wrapping_mul(
                                            ::std::mem::size_of::<SHORT>() as libc::c_ulong,
                                        ) as libc::c_long,
                                ) as *mut SHORT;
                            }
                            *sig.offset(to as isize) = i;
                            *back.offset(to as isize) = current;
                            if i < tmp {
                                j = 1 as libc::c_int;
                                k = current;
                                while k > 0 as libc::c_int {
                                    if *sig.offset(k as isize) == i {
                                        j += 1;
                                    }
                                    k = *back.offset(k as isize);
                                }
                                if j > vlen {
                                    Error(
                                        b"A_sseq: Not subsequential (?)\0" as *const u8
                                            as *const libc::c_char as *mut libc::c_char,
                                    );
                                }
                            }
                        }
                        vlen = 0 as libc::c_int;
                    }
                    if hsize == 0 as libc::c_int {
                        break;
                    }
                    aa = (**heap.offset(1 as libc::c_int as isize)).A_a;
                    bb = (**heap.offset(1 as libc::c_int as isize)).A_b;
                    cc = (**heap.offset(1 as libc::c_int as isize)).A_c;
                    if bb == 1 as libc::c_int
                        || bb % 2 as libc::c_int == 0 as libc::c_int
                    {
                        if *set.offset(cc as isize) == 0o17777777777 as libc::c_int {
                            *set.offset(cc as isize) = head;
                            head = cc;
                            vlen += 1;
                            save_coeff = 0 as *mut SHORT;
                        } else {
                            save_coeff = *to_coeff.offset(cc as isize);
                        }
                        len = veclen(*fr_coeff.offset(aa as isize))
                            - *st_len.offset(aa as isize) + *st_len.offset(cc as isize);
                        if bb == 1 as libc::c_int {
                            len += 1;
                        }
                        if len < 0 as libc::c_int {
                            Error(
                                b"A_sseq: len error\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                        }
                        let ref mut fresh16 = *to_coeff.offset(cc as isize);
                        *fresh16 = s_alloc(len + 1 as libc::c_int);
                        veccpy(
                            *to_coeff.offset(cc as isize),
                            *fr_coeff.offset(aa as isize),
                        );
                        if *st_len.offset(aa as isize) < *st_len.offset(cc as isize) {
                            veccpy(
                                (*to_coeff.offset(cc as isize))
                                    .offset(veclen(*to_coeff.offset(cc as isize)) as isize),
                                (*st_ptr.offset(cc as isize))
                                    .offset(*st_len.offset(aa as isize) as isize),
                            );
                        }
                        if bb == 1 as libc::c_int && len > 0 as libc::c_int {
                            *(*to_coeff.offset(cc as isize))
                                .offset(
                                    (len - 1 as libc::c_int) as isize,
                                ) = 1 as libc::c_int;
                            *(*to_coeff.offset(cc as isize))
                                .offset(len as isize) = 0o17777777777 as libc::c_int;
                        }
                        if !save_coeff.is_null() {
                            if veccmp(save_coeff, *to_coeff.offset(cc as isize)) != 0
                                && veclen(save_coeff)
                                    == veclen(*to_coeff.offset(cc as isize))
                            {
                                Error(
                                    b"A_sseq: Relation not single-valued (1)\0" as *const u8
                                        as *const libc::c_char as *mut libc::c_char,
                                );
                            }
                            if veclen(save_coeff)
                                >= veclen(*to_coeff.offset(cc as isize))
                            {
                                Sfree(save_coeff as *mut libc::c_char);
                            } else {
                                Sfree(*to_coeff.offset(cc as isize) as *mut libc::c_char);
                                let ref mut fresh17 = *to_coeff.offset(cc as isize);
                                *fresh17 = save_coeff;
                            }
                        } else {
                            *queue.offset(0 as libc::c_int as isize) = cc;
                            nq = 1 as libc::c_int;
                            i = 0 as libc::c_int;
                            while i < nq {
                                pz = *((*A).A_p)
                                    .offset(
                                        (*queue.offset(i as isize) + 1 as libc::c_int) as isize,
                                    );
                                p = *((*A).A_p).offset(*queue.offset(i as isize) as isize);
                                while p < pz {
                                    if (*p).A_b != 1 as libc::c_int
                                        && (*p).A_b % 2 as libc::c_int == 1 as libc::c_int
                                    {
                                        if !((*p).A_a == (*p).A_c) {
                                            if *set.offset((*p).A_c as isize)
                                                == 0o17777777777 as libc::c_int
                                            {
                                                *set.offset((*p).A_c as isize) = head;
                                                head = (*p).A_c;
                                                vlen += 1;
                                                save_coeff = 0 as *mut SHORT;
                                                let fresh18 = nq;
                                                nq = nq + 1;
                                                *queue.offset(fresh18 as isize) = (*p).A_c;
                                            } else {
                                                save_coeff = *to_coeff.offset((*p).A_c as isize);
                                            }
                                            len = veclen(*to_coeff.offset((*p).A_a as isize))
                                                + 1 as libc::c_int - *st_len.offset((*p).A_a as isize)
                                                + *st_len.offset((*p).A_c as isize);
                                            let ref mut fresh19 = *to_coeff.offset((*p).A_c as isize);
                                            *fresh19 = s_alloc(len + 1 as libc::c_int);
                                            veccpy(
                                                *to_coeff.offset((*p).A_c as isize),
                                                *to_coeff.offset((*p).A_a as isize),
                                            );
                                            tt = len - *st_len.offset((*p).A_c as isize)
                                                - 1 as libc::c_int;
                                            if tt >= 0 as libc::c_int && tt < len {
                                                *(*to_coeff.offset((*p).A_c as isize))
                                                    .offset(tt as isize) = (*p).A_b / 2 as libc::c_int;
                                            }
                                            tt = len - *st_len.offset((*p).A_c as isize);
                                            if tt < 0 as libc::c_int {
                                                tt = 0 as libc::c_int;
                                            }
                                            if tt <= len {
                                                veccpy(
                                                    (*to_coeff.offset((*p).A_c as isize)).offset(tt as isize),
                                                    (*st_ptr.offset((*p).A_c as isize))
                                                        .offset(tt as isize)
                                                        .offset(
                                                            -((len - *st_len.offset((*p).A_c as isize)) as isize),
                                                        ),
                                                );
                                            }
                                            if !save_coeff.is_null() {
                                                if veccmp(save_coeff, *to_coeff.offset((*p).A_c as isize))
                                                    != 0
                                                    && veclen(save_coeff)
                                                        == veclen(*to_coeff.offset((*p).A_c as isize))
                                                {
                                                    fprintf(
                                                        fpout,
                                                        b"Error detected at state %d:\n\0" as *const u8
                                                            as *const libc::c_char,
                                                        (*p).A_c,
                                                    );
                                                    fprintf(
                                                        fpout,
                                                        b"coeffs are\n\0" as *const u8 as *const libc::c_char,
                                                    );
                                                    k = 0 as libc::c_int;
                                                    while *(*to_coeff.offset((*p).A_c as isize))
                                                        .offset(k as isize) < 0o17777777777 as libc::c_int
                                                    {
                                                        fprintf(
                                                            fpout,
                                                            b"%s \0" as *const u8 as *const libc::c_char,
                                                            T2_name_pr(
                                                                TT2,
                                                                *(*to_coeff.offset((*p).A_c as isize)).offset(k as isize),
                                                            ),
                                                        );
                                                        k += 1;
                                                    }
                                                    fprintf(fpout, b"\n\0" as *const u8 as *const libc::c_char);
                                                    k = 0 as libc::c_int;
                                                    while *save_coeff.offset(k as isize)
                                                        < 0o17777777777 as libc::c_int
                                                    {
                                                        fprintf(
                                                            fpout,
                                                            b"%s \0" as *const u8 as *const libc::c_char,
                                                            T2_name_pr(TT2, *save_coeff.offset(k as isize)),
                                                        );
                                                        k += 1;
                                                    }
                                                    fprintf(fpout, b"\n\0" as *const u8 as *const libc::c_char);
                                                    Error(
                                                        b"A_sseq: Relation not single-valued (3)\0" as *const u8
                                                            as *const libc::c_char as *mut libc::c_char,
                                                    );
                                                }
                                                if veclen(save_coeff)
                                                    >= veclen(*to_coeff.offset((*p).A_c as isize))
                                                {
                                                    Sfree(save_coeff as *mut libc::c_char);
                                                } else {
                                                    Sfree(
                                                        *to_coeff.offset((*p).A_c as isize) as *mut libc::c_char,
                                                    );
                                                    let ref mut fresh20 = *to_coeff.offset((*p).A_c as isize);
                                                    *fresh20 = save_coeff;
                                                }
                                            }
                                        }
                                    }
                                    p = p.offset(1);
                                }
                                i += 1;
                            }
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
                        let fresh21 = hsize;
                        hsize = hsize - 1;
                        insert = *heap.offset(fresh21 as isize);
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
                        let ref mut fresh22 = *heap.offset(father as isize);
                        *fresh22 = *heap.offset(son as isize);
                        father = son;
                    }
                    let ref mut fresh23 = *heap.offset(father as isize);
                    *fresh23 = insert;
                }
            }
        }
        current += 1;
    }
    Sfree(set as *mut libc::c_char);
    Sfree(vec as *mut libc::c_char);
    Sfree(sig as *mut libc::c_char);
    Sfree(back as *mut libc::c_char);
    Sfree(heap as *mut libc::c_char);
    Sfree(fr_coeff as *mut libc::c_char);
    Sfree(to_coeff as *mut libc::c_char);
    Sfree(queue as *mut libc::c_char);
    Sfree(st_len as *mut libc::c_char);
    A_st_free();
    A_exchange(A, An);
    A_destroy(An);
    V_destroy(V);
    V_destroy(Vs);
    A = A_mkdense(A);
    A = A_close(A);
    (*A).A_mode = 8 as libc::c_int;
    (*A).A_ems = 1 as libc::c_int;
    if A_report != 0 {
        fprintf(fpout, b"<-- A_sseq  \0" as *const u8 as *const libc::c_char);
        A_rept(A);
    }
    return A;
}
