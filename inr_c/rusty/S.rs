use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn bcopy(__src: *const libc::c_void, __dest: *mut libc::c_void, __n: size_t);
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut fpout: *mut FILE;
    fn Error(_: *mut libc::c_char);
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
pub type S_ft = S_f;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct S_f {
    pub fill_1: libc::c_uchar,
    pub fill_2: libc::c_uchar,
    pub S_kval: libc::c_uchar,
    pub S_tag: libc::c_uchar,
    pub fill_3: libc::c_uint,
    pub fill_4: *mut S_f,
    pub S_linkf: *mut S_f,
    pub S_linkb: *mut S_f,
}
#[no_mangle]
pub unsafe extern "C" fn copymem(
    mut n: libc::c_long,
    mut from: *mut libc::c_char,
    mut to: *mut libc::c_char,
) {
    if from.offset(n as isize) <= to || to.offset(n as isize) <= from {
        bcopy(from as *const libc::c_void, to as *mut libc::c_void, n as size_t);
        return;
    }
    if from >= to {
        loop {
            n -= 1;
            if !(n >= 0 as libc::c_int as libc::c_long) {
                break;
            }
            let fresh0 = from;
            from = from.offset(1);
            let fresh1 = to;
            to = to.offset(1);
            *fresh1 = *fresh0;
        }
    } else {
        from = from.offset(n as isize);
        to = to.offset(n as isize);
        loop {
            n -= 1;
            if !(n >= 0 as libc::c_int as libc::c_long) {
                break;
            }
            from = from.offset(-1);
            to = to.offset(-1);
            *to = *from;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn scribble(mut p: *mut libc::c_char, mut q: *mut libc::c_char) {
    while p < q {
        let fresh2 = p;
        p = p.offset(1);
        *fresh2 = 0x55 as libc::c_int as libc::c_char;
    }
}
static mut S_lo: *mut S_ft = 0 as *const S_ft as *mut S_ft;
static mut S_hi: *mut S_ft = 0 as *const S_ft as *mut S_ft;
static mut S_avail: [S_ft; 29] = [S_ft {
    fill_1: 0,
    fill_2: 0,
    S_kval: 0,
    S_tag: 0,
    fill_3: 0,
    fill_4: 0 as *const S_f as *mut S_f,
    S_linkf: 0 as *const S_f as *mut S_f,
    S_linkb: 0 as *const S_f as *mut S_f,
}; 29];
static mut S_alld_cnt: [libc::c_int; 28] = [0; 28];
#[no_mangle]
pub static mut LINUXmem: libc::c_long = 0 as libc::c_int as libc::c_long;
#[no_mangle]
pub unsafe extern "C" fn S_init() {
    let mut p: *mut S_ft = 0 as *mut S_ft;
    let mut i: libc::c_int = 0;
    if S_lo.is_null() {
        let mut mem: libc::c_long = 0;
        mem = (512 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int)
            as libc::c_long;
        while S_lo.is_null() {
            LINUXmem = mem;
            S_hi = malloc((mem + 16 as libc::c_int as libc::c_long) as libc::c_ulong)
                as *mut S_ft;
            S_lo = S_hi;
            mem /= 2 as libc::c_int as libc::c_long;
        }
        fflush(fpout);
        p = &mut *S_avail.as_mut_ptr().offset(28 as libc::c_int as isize) as *mut S_ft;
        let ref mut fresh3 = (*p).S_linkf;
        *fresh3 = 0 as *mut S_f;
        loop {
            p = p.offset(-1);
            if !(p >= S_avail.as_mut_ptr()) {
                break;
            }
            let ref mut fresh4 = (*p).S_linkf;
            *fresh4 = p;
            let ref mut fresh5 = (*p).S_linkb;
            *fresh5 = p;
        }
        i = 0 as libc::c_int;
        while i < 28 as libc::c_int {
            S_alld_cnt[i as usize] = 0 as libc::c_int;
            i += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn S_free(mut l: *mut S_ft, mut k: libc::c_int) {
    let mut p: *mut S_ft = 0 as *mut S_ft;
    if l as libc::c_long & 7 as libc::c_int as libc::c_long != 0 {
        Error(
            b"S_free: l not divisible by 8\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if l < S_lo || l.offset(((1 as libc::c_int) << k) as isize) > S_hi {
        Error(
            b"S_free: bounds\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if l.offset_from(S_lo) as libc::c_long
        & (((1 as libc::c_int) << k) - 1 as libc::c_int) as libc::c_long != 0
    {
        Error(
            b"S_free: l improper\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if (*l).S_tag != 0 {
        Error(
            b"S_free: attempt to free unallocated block\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    }
    S_alld_cnt[k as usize] -= 1;
    while k < 28 as libc::c_int - 1 as libc::c_int {
        p = S_lo
            .offset(
                (l.offset_from(S_lo) as libc::c_long
                    ^ ((1 as libc::c_int) << k) as libc::c_long) as isize,
            );
        if p >= S_hi || (*p).S_tag == 0 || (*p).S_kval as libc::c_int != k {
            break;
        }
        let ref mut fresh6 = (*(*p).S_linkb).S_linkf;
        *fresh6 = (*p).S_linkf;
        let ref mut fresh7 = (*(*p).S_linkf).S_linkb;
        *fresh7 = (*p).S_linkb;
        if p < l {
            l = p;
        }
        k += 1;
    }
    (*l).S_tag = 1 as libc::c_int as libc::c_uchar;
    (*l).S_kval = k as libc::c_uchar;
    p = S_avail[k as usize].S_linkf;
    let ref mut fresh8 = (*l).S_linkf;
    *fresh8 = p;
    let ref mut fresh9 = (*p).S_linkb;
    *fresh9 = l;
    p = &mut *S_avail.as_mut_ptr().offset(k as isize) as *mut S_ft;
    let ref mut fresh10 = (*l).S_linkb;
    *fresh10 = p;
    let ref mut fresh11 = (*p).S_linkf;
    *fresh11 = l;
}
#[no_mangle]
pub unsafe extern "C" fn S_morecore(mut k: libc::c_int) {
    let mut a: libc::c_long = 0;
    let mut b: libc::c_long = 0;
    if S_hi != S_lo {
        Error(
            b"S_morecore: Out of Memory\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    a = 0 as libc::c_int as libc::c_long;
    S_hi = S_lo
        .offset(
            (LINUXmem as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<S_ft>() as libc::c_ulong) as isize,
        );
    b = S_hi.offset_from(S_lo) as libc::c_long;
    while a < b {
        k = 0 as libc::c_int;
        while a >> k & 1 as libc::c_int as libc::c_long == 0
            && b - a >> k + 1 as libc::c_int != 0
        {
            k += 1;
        }
        S_alld_cnt[k as usize] += 1;
        (*S_lo.offset(a as isize)).S_tag = 0 as libc::c_int as libc::c_uchar;
        S_free(S_lo.offset(a as isize), k);
        a += ((1 as libc::c_int) << k) as libc::c_long;
    }
}
#[no_mangle]
pub unsafe extern "C" fn S_malloc(mut k: libc::c_int) -> *mut S_ft {
    let mut j: libc::c_int = 0;
    let mut p: *mut S_ft = 0 as *mut S_ft;
    let mut l: *mut S_ft = 0 as *mut S_ft;
    let mut q: *mut S_ft = 0 as *mut S_ft;
    if k >= 28 as libc::c_int {
        Error(
            b"S_malloc: Argument constraint error\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    S_alld_cnt[k as usize] += 1;
    loop {
        j = k;
        while S_avail[j as usize].S_linkf
            == &mut *S_avail.as_mut_ptr().offset(j as isize) as *mut S_ft
        {
            j += 1;
        }
        if j < 28 as libc::c_int {
            break;
        }
        S_morecore(k);
    }
    p = &mut *S_avail.as_mut_ptr().offset(j as isize) as *mut S_ft;
    l = (*p).S_linkf;
    let ref mut fresh12 = (*p).S_linkf;
    *fresh12 = (*l).S_linkf;
    let ref mut fresh13 = (*(*l).S_linkf).S_linkb;
    *fresh13 = p;
    (*l).S_tag = 0 as libc::c_int as libc::c_uchar;
    (*l).S_kval = k as libc::c_uchar;
    let ref mut fresh14 = (*l).S_linkf;
    *fresh14 = 0 as *mut S_f;
    let ref mut fresh15 = (*l).S_linkb;
    *fresh15 = 0 as *mut S_f;
    loop {
        j -= 1;
        if !(j >= k) {
            break;
        }
        p = l.offset(((1 as libc::c_int) << j) as isize);
        (*p).S_tag = 1 as libc::c_int as libc::c_uchar;
        (*p).S_kval = j as libc::c_uchar;
        q = &mut *S_avail.as_mut_ptr().offset(j as isize) as *mut S_ft;
        let ref mut fresh16 = (*p).S_linkf;
        *fresh16 = q;
        let ref mut fresh17 = (*p).S_linkb;
        *fresh17 = q;
        let ref mut fresh18 = (*q).S_linkf;
        *fresh18 = p;
        let ref mut fresh19 = (*q).S_linkb;
        *fresh19 = p;
    }
    return l;
}
#[no_mangle]
pub unsafe extern "C" fn S_realloc(
    mut l: *mut S_ft,
    mut k1: libc::c_int,
    mut k2: libc::c_int,
) -> *mut S_ft {
    let mut k0: libc::c_int = 0;
    let mut p: *mut S_ft = 0 as *mut S_ft;
    let mut q: *mut S_ft = 0 as *mut S_ft;
    S_alld_cnt[k1 as usize] -= 1;
    S_alld_cnt[k2 as usize] += 1;
    if k1 >= k2 {
        loop {
            k1 -= 1;
            if !(k1 >= k2) {
                break;
            }
            p = l.offset(((1 as libc::c_int) << k1) as isize);
            (*p).S_tag = 1 as libc::c_int as libc::c_uchar;
            (*p).S_kval = k1 as libc::c_uchar;
            q = S_avail[k1 as usize].S_linkb;
            let ref mut fresh20 = (*p).S_linkb;
            *fresh20 = q;
            let ref mut fresh21 = (*q).S_linkf;
            *fresh21 = p;
            q = &mut *S_avail.as_mut_ptr().offset(k1 as isize) as *mut S_ft;
            let ref mut fresh22 = (*p).S_linkf;
            *fresh22 = q;
            let ref mut fresh23 = (*q).S_linkb;
            *fresh23 = p;
        }
        (*l).S_kval = k2 as libc::c_uchar;
        return l;
    } else {
        k0 = k1;
        while k1 < k2 {
            p = S_lo
                .offset(
                    (l.offset_from(S_lo) as libc::c_long
                        ^ ((1 as libc::c_int) << k1) as libc::c_long) as isize,
                );
            if p < l || p >= S_hi || (*p).S_tag == 0 || (*p).S_kval as libc::c_int != k1
            {
                break;
            }
            let ref mut fresh24 = (*(*p).S_linkb).S_linkf;
            *fresh24 = (*p).S_linkf;
            let ref mut fresh25 = (*(*p).S_linkf).S_linkb;
            *fresh25 = (*p).S_linkb;
            k1 += 1;
        }
        if k1 == k2 {
            (*l).S_kval = k2 as libc::c_uchar;
            return l;
        }
        S_alld_cnt[k2 as usize] -= 1;
        p = S_malloc(k2);
        copymem(
            ((::std::mem::size_of::<S_ft>() as libc::c_ulong) << k0) as libc::c_long,
            l as *mut libc::c_char,
            p as *mut libc::c_char,
        );
        S_alld_cnt[k1 as usize] += 1;
        S_free(l, k1);
        (*p).S_kval = k2 as libc::c_uchar;
        return p;
    };
}
#[no_mangle]
pub unsafe extern "C" fn S_copy(mut l: *mut S_ft, mut k: libc::c_int) -> *mut S_ft {
    let mut p: *mut S_ft = 0 as *mut S_ft;
    p = S_malloc(k);
    copymem(
        ((::std::mem::size_of::<S_ft>() as libc::c_ulong) << k) as libc::c_long,
        l as *mut libc::c_char,
        p as *mut libc::c_char,
    );
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn S_arena() {
    let mut i: libc::c_int = 0;
    let mut cnt: libc::c_int = 0;
    let mut grand: libc::c_long = 0;
    let mut gran2: libc::c_long = 0;
    let mut size: libc::c_long = 0;
    let mut p: *mut S_ft = 0 as *mut S_ft;
    let mut q: *mut S_ft = 0 as *mut S_ft;
    S_init();
    fprintf(
        fpout,
        b"Size      Free      Allocated\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        fpout,
        b"      Number Total Number Total\n\0" as *const u8 as *const libc::c_char,
    );
    grand = 0 as libc::c_int as libc::c_long;
    gran2 = 0 as libc::c_int as libc::c_long;
    i = 0 as libc::c_int;
    while i < 28 as libc::c_int {
        q = &mut *S_avail.as_mut_ptr().offset(i as isize) as *mut S_ft;
        p = (*q).S_linkf;
        if p != q || S_alld_cnt[i as usize] != 0 {
            size = ((::std::mem::size_of::<S_ft>() as libc::c_ulong) << i)
                as libc::c_long;
            if p != q {
                cnt = 1 as libc::c_int;
                loop {
                    p = (*p).S_linkf;
                    if !(p != q) {
                        break;
                    }
                    cnt += 1;
                }
            } else {
                cnt = 0 as libc::c_int;
            }
            if size < 1024 as libc::c_int as libc::c_long {
                fprintf(fpout, b"%4ld \0" as *const u8 as *const libc::c_char, size);
            } else if size < (1024 as libc::c_int * 1024 as libc::c_int) as libc::c_long
            {
                fprintf(
                    fpout,
                    b"%4ldK\0" as *const u8 as *const libc::c_char,
                    size / 1024 as libc::c_int as libc::c_long,
                );
            } else {
                fprintf(
                    fpout,
                    b"%4ldM\0" as *const u8 as *const libc::c_char,
                    size / 1024 as libc::c_int as libc::c_long
                        / 1024 as libc::c_int as libc::c_long,
                );
            }
            fprintf(fpout, b"%7d\0" as *const u8 as *const libc::c_char, cnt);
            fprintf(
                fpout,
                b"%5ldM\0" as *const u8 as *const libc::c_char,
                (cnt as libc::c_long * size + 1023 as libc::c_int as libc::c_long)
                    / 1024 as libc::c_int as libc::c_long
                    / 1024 as libc::c_int as libc::c_long,
            );
            fprintf(
                fpout,
                b"%7d\0" as *const u8 as *const libc::c_char,
                S_alld_cnt[i as usize],
            );
            fprintf(
                fpout,
                b"%5ldM\n\0" as *const u8 as *const libc::c_char,
                (S_alld_cnt[i as usize] as libc::c_long * size
                    + 1023 as libc::c_int as libc::c_long)
                    / 1024 as libc::c_int as libc::c_long
                    / 1024 as libc::c_int as libc::c_long,
            );
            grand += cnt as libc::c_long * size;
            gran2 += S_alld_cnt[i as usize] as libc::c_long * size;
        }
        i += 1;
    }
    fprintf(
        fpout,
        b"            %5ldM\0" as *const u8 as *const libc::c_char,
        (grand + 1023 as libc::c_int as libc::c_long)
            / 1024 as libc::c_int as libc::c_long / 1024 as libc::c_int as libc::c_long,
    );
    fprintf(
        fpout,
        b"       %5ldM\n\0" as *const u8 as *const libc::c_char,
        (gran2 + 1023 as libc::c_int as libc::c_long)
            / 1024 as libc::c_int as libc::c_long / 1024 as libc::c_int as libc::c_long,
    );
    size = LINUXmem;
    fprintf(
        fpout,
        b"Memory Size %5ldM\n\0" as *const u8 as *const libc::c_char,
        size / 1024 as libc::c_int as libc::c_long / 1024 as libc::c_int as libc::c_long,
    );
    if size % 1024 as libc::c_int as libc::c_long != 0 as libc::c_int as libc::c_long {
        fprintf(
            fpout,
            b"Excess %ld bytes\n\0" as *const u8 as *const libc::c_char,
            size % 1024 as libc::c_int as libc::c_long,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn S_find(mut p: *mut libc::c_char) -> *mut S_ft {
    if p < S_lo as *mut libc::c_char || p >= S_hi as *mut libc::c_char {
        Error(
            b"S_find: BOTCH 1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    let mut l: *mut S_ft = S_lo;
    let mut incr: libc::c_long = (p.offset_from(S_lo as *mut libc::c_char)
        as libc::c_long as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<S_ft>() as libc::c_ulong) as libc::c_long;
    let mut offset: libc::c_long = (p.offset_from(S_lo as *mut libc::c_char)
        as libc::c_long as libc::c_ulong)
        .wrapping_rem(::std::mem::size_of::<S_ft>() as libc::c_ulong) as libc::c_long;
    if (&mut *l.offset(incr as isize) as *mut S_ft as *mut libc::c_char)
        .offset(offset as isize) != p
    {
        Error(
            b"S_find: BOTCH 2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if offset < 0 as libc::c_int as libc::c_long
        || offset as libc::c_ulong >= ::std::mem::size_of::<S_ft>() as libc::c_ulong
    {
        Error(
            b"S_find: BOTCH 3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    let mut base: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut base_k: libc::c_int = (*l.offset(base as isize)).S_kval as libc::c_int;
    let mut right: libc::c_long = S_hi.offset_from(S_lo) as libc::c_long;
    let mut k: libc::c_int = 0;
    k = 0 as libc::c_int;
    while incr >> k != 0 {
        k += 1;
    }
    k -= 1;
    right = ((1 as libc::c_int) << k) as libc::c_long;
    while base_k <= k {
        base += right;
        incr -= right;
        base_k = (*l.offset(base as isize)).S_kval as libc::c_int;
        k = 0 as libc::c_int;
        while incr >> k != 0 {
            k += 1;
        }
        k -= 1;
        right = ((1 as libc::c_int) << k) as libc::c_long;
    }
    return &mut *l.offset(base as isize) as *mut S_ft;
}
#[no_mangle]
pub unsafe extern "C" fn Salloc(mut n: libc::c_long) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut k: libc::c_int = 0;
    S_init();
    if n < 0 as libc::c_int as libc::c_long {
        Error(
            b"Salloc: Argument constraint error\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    n = (n as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<S_ft>() as libc::c_ulong)
        .wrapping_add(3 as libc::c_int as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<S_ft>() as libc::c_ulong) as libc::c_long;
    k = 0 as libc::c_int;
    while n > ((1 as libc::c_int) << k) as libc::c_long {
        k += 1;
    }
    p = S_malloc(k) as *mut libc::c_char;
    *p.offset(0 as libc::c_int as isize) = 0x7f as libc::c_int as libc::c_char;
    *p.offset(1 as libc::c_int as isize) = k as libc::c_char;
    return p.offset(4 as libc::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn Sfree(mut p: *mut libc::c_char) {
    if p.is_null() {
        return;
    }
    p = p.offset(-(4 as libc::c_int as isize));
    if *p.offset(0 as libc::c_int as isize) as libc::c_int != 0x7f as libc::c_int {
        Error(
            b"Sfree: Invalid free\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    S_free(p as *mut S_ft, *p.offset(1 as libc::c_int as isize) as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Srealloc(
    mut p: *mut libc::c_char,
    mut n: libc::c_long,
) -> *mut libc::c_char {
    let mut k: libc::c_int = 0;
    if n < 0 as libc::c_int as libc::c_long {
        Error(
            b"Srealloc: Argument constraint error\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if p.is_null() {
        return Salloc(n);
    }
    n = (n as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<S_ft>() as libc::c_ulong)
        .wrapping_add(3 as libc::c_int as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<S_ft>() as libc::c_ulong) as libc::c_long;
    k = 0 as libc::c_int;
    while n > ((1 as libc::c_int) << k) as libc::c_long {
        k += 1;
    }
    p = p.offset(-(4 as libc::c_int as isize));
    p = S_realloc(p as *mut S_ft, *p.offset(1 as libc::c_int as isize) as libc::c_int, k)
        as *mut libc::c_char;
    *p.offset(0 as libc::c_int as isize) = 0x7f as libc::c_int as libc::c_char;
    *p.offset(1 as libc::c_int as isize) = k as libc::c_char;
    return p.offset(4 as libc::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn Scopy(mut p: *mut libc::c_char) -> *mut libc::c_char {
    if p.is_null() {
        return 0 as *mut libc::c_char;
    }
    p = p.offset(-(4 as libc::c_int as isize));
    return (S_copy(p as *mut S_ft, *p.offset(1 as libc::c_int as isize) as libc::c_int)
        as *mut libc::c_char)
        .offset(4 as libc::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn Ssize(mut p: *mut libc::c_char) -> libc::c_long {
    if !p.is_null() {} else {
        __assert_fail(
            b"p != NULL\0" as *const u8 as *const libc::c_char,
            b"S.c\0" as *const u8 as *const libc::c_char,
            402 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 19],
                &[libc::c_char; 19],
            >(b"long Ssize(char *)\0"))
                .as_ptr(),
        );
    }
    return ((::std::mem::size_of::<S_ft>() as libc::c_ulong)
        << *p.offset(-(3 as libc::c_int) as isize) as libc::c_int)
        .wrapping_sub(4 as libc::c_int as libc::c_ulong) as libc::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn Sarena() {
    S_arena();
}
#[no_mangle]
pub unsafe extern "C" fn Saudit() {
    let mut p: *mut S_ft = 0 as *mut S_ft;
    let mut last_p: *mut S_ft = 0 as *mut S_ft;
    let mut pc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    printf(b"<<< Audit >>>\n\0" as *const u8 as *const libc::c_char);
    last_p = 0 as *mut S_ft;
    p = S_lo;
    while p < S_hi {
        pc = p as *mut libc::c_char;
        if p.offset_from(S_lo) as libc::c_long
            & (((1 as libc::c_int) << (*p).S_kval as libc::c_int) - 1 as libc::c_int)
                as libc::c_long != 0
        {
            printf(
                b"Block alignment error at %lx\n\0" as *const u8 as *const libc::c_char,
                p as libc::c_ulong,
            );
            printf(
                b"Block size %d Offset %lx\n\0" as *const u8 as *const libc::c_char,
                (*p).S_kval as libc::c_int,
                p.offset_from(S_lo) as libc::c_long as libc::c_ulong,
            );
            if !last_p.is_null() {
                printf(
                    b"Last good block %lx kval %d\n\0" as *const u8
                        as *const libc::c_char,
                    last_p as libc::c_ulong,
                    (*last_p).S_kval as libc::c_int,
                );
            }
            return;
        }
        if (*p).S_tag == 0 {
            if *pc.offset(0 as libc::c_int as isize) as libc::c_int
                != 0x7f as libc::c_int
            {
                printf(
                    b"Audit anomaly in busy block at %lx:\n\0" as *const u8
                        as *const libc::c_char,
                    p as libc::c_ulong,
                );
                printf(
                    b"Size code %d\n\0" as *const u8 as *const libc::c_char,
                    *pc.offset(1 as libc::c_int as isize) as libc::c_int,
                );
                printf(
                    b"S_lo %lx S_hi %lx S_avail %lx\n\0" as *const u8
                        as *const libc::c_char,
                    S_lo as libc::c_ulong,
                    S_hi as libc::c_ulong,
                    S_avail.as_mut_ptr() as libc::c_ulong,
                );
                if *pc.offset(0 as libc::c_int as isize) as libc::c_int
                    != 0x7f as libc::c_int
                {
                    printf(
                        b"Mask is %lx\n\0" as *const u8 as *const libc::c_char,
                        (*pc.offset(0 as libc::c_int as isize) as libc::c_int
                            & 0xff as libc::c_int) as libc::c_ulong,
                    );
                }
                if !last_p.is_null() {
                    printf(
                        b"Last good block %lx kval %d\n\0" as *const u8
                            as *const libc::c_char,
                        last_p as libc::c_ulong,
                        (*last_p).S_kval as libc::c_int,
                    );
                }
                return;
            }
        } else {
            k = (*p).S_kval as libc::c_int;
            if k >= 30 as libc::c_int
                || ((*p).S_linkf < S_lo || (*p).S_linkf >= S_hi)
                    && ((*p).S_linkf < S_avail.as_mut_ptr()
                        || (*p).S_linkf
                            >= S_avail.as_mut_ptr().offset(28 as libc::c_int as isize))
                || ((*p).S_linkb < S_lo || (*p).S_linkb >= S_hi)
                    && ((*p).S_linkb < S_avail.as_mut_ptr()
                        || (*p).S_linkb
                            >= S_avail.as_mut_ptr().offset(28 as libc::c_int as isize))
                || (*(*p).S_linkf).S_linkb != p || (*(*p).S_linkb).S_linkf != p
            {
                printf(
                    b"Audit anomaly in free block at %lx:\n\0" as *const u8
                        as *const libc::c_char,
                    p as libc::c_ulong,
                );
                printf(
                    b"S_lo %lx S_hi %lx S_avail %lx\n\0" as *const u8
                        as *const libc::c_char,
                    S_lo as libc::c_ulong,
                    S_hi as libc::c_ulong,
                    S_avail.as_mut_ptr() as libc::c_ulong,
                );
                printf(b"kval %d\n\0" as *const u8 as *const libc::c_char, k);
                printf(
                    b"linkf %lx linkb %lx\n\0" as *const u8 as *const libc::c_char,
                    (*p).S_linkf as libc::c_ulong,
                    (*p).S_linkb as libc::c_ulong,
                );
                if (*p).S_linkf >= S_lo && (*p).S_linkf < S_hi
                    || (*p).S_linkf >= S_avail.as_mut_ptr()
                        && (*p).S_linkf
                            < S_avail.as_mut_ptr().offset(28 as libc::c_int as isize)
                {
                    printf(
                        b"linkb(linkf(p)) %lx\n\0" as *const u8 as *const libc::c_char,
                        (*(*p).S_linkf).S_linkb as libc::c_ulong,
                    );
                }
                if (*p).S_linkb >= S_lo && (*p).S_linkb < S_hi
                    || (*p).S_linkb >= S_avail.as_mut_ptr()
                        && (*p).S_linkb
                            < S_avail.as_mut_ptr().offset(28 as libc::c_int as isize)
                {
                    printf(
                        b"linkf(linkb(p)) %lx\n\0" as *const u8 as *const libc::c_char,
                        (*(*p).S_linkb).S_linkf as libc::c_ulong,
                    );
                }
                i = 0 as libc::c_int;
                while i < 28 as libc::c_int {
                    if (*S_avail.as_mut_ptr().offset(i as isize)).S_linkf == p {
                        printf(
                            b"linkf(S_avail+i) %lx\n\0" as *const u8
                                as *const libc::c_char,
                            (*S_avail.as_mut_ptr().offset(i as isize)).S_linkf
                                as libc::c_ulong,
                        );
                    }
                    if (*S_avail.as_mut_ptr().offset(i as isize)).S_linkb == p {
                        printf(
                            b"linkb(S_avail+i) %lx\n\0" as *const u8
                                as *const libc::c_char,
                            (*S_avail.as_mut_ptr().offset(i as isize)).S_linkb
                                as libc::c_ulong,
                        );
                    }
                    i += 1;
                }
                if !last_p.is_null() {
                    printf(
                        b"Last good block %lx kval %d\n\0" as *const u8
                            as *const libc::c_char,
                        last_p as libc::c_ulong,
                        (*last_p).S_kval as libc::c_int,
                    );
                }
                return;
            }
        }
        last_p = p;
        p = p.offset(((1 as libc::c_int) << (*p).S_kval as libc::c_int) as isize);
    }
}
