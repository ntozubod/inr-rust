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
static mut GAt: A_OBJECT = 0 as *const A_desc as *mut A_desc;
static mut t_stk: *mut SHORT = 0 as *const SHORT as *mut SHORT;
static mut t_low: *mut SHORT = 0 as *const SHORT as *mut SHORT;
static mut t_cnt: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn A_tr_DFS(mut l_state: SHORT) -> SHORT {
    let mut state: *mut SHORT = s_alloc((*GAt).A_nQ);
    let mut dfn: *mut SHORT = s_alloc((*GAt).A_nQ);
    let mut p: *mut *mut A_row = Salloc(
        ((*GAt).A_nQ as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut A_row>() as libc::c_ulong)
            as libc::c_long,
    ) as *mut *mut A_row;
    let mut comp: *mut SHORT = t_stk;
    let mut low: *mut SHORT = t_low;
    let mut l_dfn: SHORT = 0;
    let mut l_p: *mut A_row = 0 as *mut A_row;
    let mut l_next: SHORT = 0;
    let mut return_val: SHORT = 0;
    let mut rec_stk_idx: libc::c_int = 0 as libc::c_int;
    let mut cmp_stk_idx: libc::c_int = 0 as libc::c_int;
    'c_1918: loop {
        *state.offset(rec_stk_idx as isize) = l_state;
        let fresh0 = t_cnt;
        t_cnt = t_cnt + 1;
        l_dfn = fresh0;
        *dfn.offset(rec_stk_idx as isize) = l_dfn;
        *low.offset(l_state as isize) = l_dfn;
        let fresh1 = cmp_stk_idx;
        cmp_stk_idx = cmp_stk_idx + 1;
        *comp.offset(fresh1 as isize) = l_state;
        l_p = *((*GAt).A_p).offset((l_state + 1 as libc::c_int) as isize);
        let ref mut fresh2 = *p.offset(rec_stk_idx as isize);
        *fresh2 = l_p;
        loop {
            let ref mut fresh3 = *p.offset(rec_stk_idx as isize);
            *fresh3 = (*fresh3).offset(-1);
            l_p = *fresh3;
            l_state = *state.offset(rec_stk_idx as isize);
            if l_p < *((*GAt).A_p).offset(l_state as isize) {
                l_state = *state.offset(rec_stk_idx as isize);
                l_dfn = *dfn.offset(rec_stk_idx as isize);
                if *low.offset(l_state as isize) == l_dfn {
                    l_next = 0o17777777777 as libc::c_int;
                    while l_next != l_state {
                        cmp_stk_idx -= 1;
                        l_next = *comp.offset(cmp_stk_idx as isize);
                        *low
                            .offset(
                                l_next as isize,
                            ) = 0o17777777777 as libc::c_int - 1 as libc::c_int;
                    }
                }
                l_state = *state.offset(rec_stk_idx as isize);
                return_val = l_state;
                if !(rec_stk_idx > 0 as libc::c_int) {
                    break 'c_1918;
                }
                rec_stk_idx -= 1;
                l_next = return_val;
            } else {
                l_next = (*l_p).A_c;
                if *low.offset(l_next as isize) == 0o17777777777 as libc::c_int {
                    break;
                }
            }
            l_state = *state.offset(rec_stk_idx as isize);
            if *low.offset(l_next as isize) < *low.offset(l_state as isize) {
                *low.offset(l_state as isize) = *low.offset(l_next as isize);
            }
        }
        l_state = l_next;
        rec_stk_idx += 1;
    }
    Sfree(state as *mut libc::c_char);
    Sfree(dfn as *mut libc::c_char);
    Sfree(p as *mut libc::c_char);
    return return_val;
}
#[no_mangle]
pub unsafe extern "C" fn A_trim(mut A: A_OBJECT) -> A_OBJECT {
    let mut p: *mut A_row = 0 as *mut A_row;
    let mut pz: *mut A_row = 0 as *mut A_row;
    let mut i: libc::c_int = 0;
    let mut must_reopen: libc::c_int = 0;
    let mut stk_rem: *mut SHORT = 0 as *mut SHORT;
    if A.is_null() {
        Error(
            b"A_trim: No OBJECT\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if (*A).A_nQ >= 0o17777777777 as libc::c_int - 1 as libc::c_int {
        Error(
            b"A_trim: Too many states\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if (*A).A_mode < 1 as libc::c_int {
        A = A_close(A);
    }
    if (*A).A_mode >= 2 as libc::c_int {
        return A;
    }
    if (*A).A_nrows == 0 as libc::c_int {
        (*A).A_mode = 2 as libc::c_int;
        return A;
    }
    if A_report != 0 {
        fprintf(fpout, b"--> A_trim\n\0" as *const u8 as *const libc::c_char);
    }
    stk_rem = s_alloc((*A).A_nQ);
    t_stk = stk_rem;
    t_low = s_alloc((*A).A_nQ);
    i = (*A).A_nQ;
    loop {
        i -= 1;
        if !(i >= 0 as libc::c_int) {
            break;
        }
        *t_low.offset(i as isize) = 0o17777777777 as libc::c_int;
    }
    *t_low.offset(1 as libc::c_int as isize) = 0 as libc::c_int;
    t_cnt = 1 as libc::c_int;
    GAt = A;
    if A_tr_DFS(0 as libc::c_int) != 0 as libc::c_int {
        Error(
            b"A_trim: BOTCH\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    Sfree(stk_rem as *mut libc::c_char);
    *t_low.offset(1 as libc::c_int as isize) = *t_low.offset(0 as libc::c_int as isize);
    must_reopen = 0 as libc::c_int;
    pz = ((*A).A_t).offset((*A).A_nrows as isize);
    p = pz;
    loop {
        p = p.offset(-1);
        if !(p >= (*A).A_t) {
            break;
        }
        if *t_low.offset((*p).A_a as isize)
            >= 0o17777777777 as libc::c_int - 1 as libc::c_int
            || *t_low.offset((*p).A_c as isize)
                >= 0o17777777777 as libc::c_int - 1 as libc::c_int
        {
            pz = pz.offset(-1);
            (*p).A_a = (*pz).A_a;
            (*p).A_b = (*pz).A_b;
            (*p).A_c = (*pz).A_c;
            must_reopen = 1 as libc::c_int;
        }
    }
    (*A).A_nrows = pz.offset_from((*A).A_t) as libc::c_long as libc::c_int;
    Sfree(t_low as *mut libc::c_char);
    if must_reopen != 0 {
        A = A_close(A_open(A));
    }
    (*A).A_mode = 2 as libc::c_int;
    pz = ((*A).A_t).offset((*A).A_nrows as isize);
    p = pz;
    loop {
        p = p.offset(-1);
        if !(p >= (*A).A_t) {
            break;
        }
        if (*p).A_b == 0 as libc::c_int {
            break;
        }
    }
    if p < (*A).A_t {
        (*A).A_mode = 5 as libc::c_int;
        p = pz;
        loop {
            p = p.offset(-1);
            if !(p > (*A).A_t) {
                break;
            }
            if (*p.offset(-(1 as libc::c_int as isize))).A_b == (*p).A_b
                && (*p.offset(-(1 as libc::c_int as isize))).A_a == (*p).A_a
            {
                break;
            }
        }
        if p == (*A).A_t {
            (*A).A_mode = 6 as libc::c_int;
        }
    }
    if A_report != 0 {
        fprintf(fpout, b"<-- A_trim  \0" as *const u8 as *const libc::c_char);
        A_rept(A);
    }
    return A;
}
