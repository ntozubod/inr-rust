use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut fpout: *mut FILE;
    fn Sfree(_: *mut libc::c_char);
    fn Error(_: *mut libc::c_char);
    fn s_alloc(_: libc::c_int) -> *mut SHORT;
    fn i_alloc(_: libc::c_int) -> *mut libc::c_int;
    fn T2_name_pr(_: T2_OBJECT, _: libc::c_int) -> *mut libc::c_char;
    fn A_destroy(_: A_OBJECT);
    fn A_copy(_: A_OBJECT) -> A_OBJECT;
    fn A_add(_: A_OBJECT, _: libc::c_int, _: libc::c_int, _: libc::c_int) -> A_OBJECT;
    fn A_open(_: A_OBJECT) -> A_OBJECT;
    fn A_close(_: A_OBJECT) -> A_OBJECT;
    fn A_subs(_: A_OBJECT) -> A_OBJECT;
    fn A_min(_: A_OBJECT) -> A_OBJECT;
    fn A_differ(_: A_OBJECT, _: A_OBJECT) -> A_OBJECT;
    fn A_lenmin(_: A_OBJECT) -> A_OBJECT;
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
static mut GAe: A_OBJECT = 0 as *const A_desc as *mut A_desc;
static mut GTe: T2_OBJECT = 0 as *const T2_desc as *mut T2_desc;
static mut e_vec: *mut SHORT = 0 as *const SHORT as *mut SHORT;
static mut c_vec: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
static mut e_lev: libc::c_int = 0;
static mut en_cnt: libc::c_int = 0;
static mut en_max: libc::c_int = 0;
static mut en_str: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn A_en_DFS(mut state: SHORT) -> libc::c_int {
    let mut p: *mut A_row = 0 as *mut A_row;
    static mut i: SHORT = 0;
    p = *((*GAe).A_p).offset(state as isize);
    while p < *((*GAe).A_p).offset((state + 1 as libc::c_int) as isize) {
        if (*p).A_b == 1 as libc::c_int {
            fprintf(fpout, b"    \0" as *const u8 as *const libc::c_char);
            if e_lev == 0 as libc::c_int {
                fprintf(fpout, b"^ \0" as *const u8 as *const libc::c_char);
            } else {
                i = 0 as libc::c_int;
                while i < e_lev {
                    en_cnt += 1;
                    if (*GAe).A_nT == 1 as libc::c_int {
                        en_str = T2_name_pr(GTe, *e_vec.offset(i as isize));
                        fprintf(
                            fpout,
                            b"%s \0" as *const u8 as *const libc::c_char,
                            en_str,
                        );
                    } else {
                        en_str = T2_name_pr(
                            GTe,
                            *e_vec.offset(i as isize) / (*GAe).A_nT,
                        );
                        fprintf(
                            fpout,
                            b"%1d.%s \0" as *const u8 as *const libc::c_char,
                            *e_vec.offset(i as isize) % (*GAe).A_nT,
                            en_str,
                        );
                    }
                    i += 1;
                }
            }
            fprintf(fpout, b"\n\0" as *const u8 as *const libc::c_char);
        } else {
            if e_lev == en_max {
                return 1 as libc::c_int;
            }
            if en_cnt > en_max {
                return 1 as libc::c_int;
            }
            let fresh0 = e_lev;
            e_lev = e_lev + 1;
            *e_vec.offset(fresh0 as isize) = (*p).A_b;
            if A_en_DFS((*p).A_c) != 0 {
                return 1 as libc::c_int;
            }
            e_lev -= 1;
        }
        p = p.offset(1);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn A_enum(
    mut A: A_OBJECT,
    mut T2: T2_OBJECT,
    mut max: libc::c_int,
) -> A_OBJECT {
    let mut Ar: A_OBJECT = 0 as *mut A_desc;
    let mut Am: A_OBJECT = 0 as *mut A_desc;
    if A.is_null() {
        Error(
            b"A_enum: No OBJECT\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    Ar = A_copy(A);
    en_max = max;
    e_vec = s_alloc(en_max);
    e_lev = 0 as libc::c_int;
    GTe = T2;
    en_cnt = 0 as libc::c_int;
    while (*Ar).A_nrows > 0 as libc::c_int {
        Am = A_subs(A_lenmin(A_copy(Ar)));
        GAe = Am;
        if A_en_DFS(0 as libc::c_int) != 0 {
            fprintf(fpout, b"Enum terminated\n\0" as *const u8 as *const libc::c_char);
            A_destroy(Am);
            break;
        } else {
            Ar = A_differ(Ar, Am);
        }
    }
    Sfree(e_vec as *mut libc::c_char);
    A_destroy(Ar);
    return A;
}
#[no_mangle]
pub unsafe extern "C" fn A_cd_DFS(mut state: SHORT) -> libc::c_int {
    let mut p: *mut A_row = 0 as *mut A_row;
    static mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    count = *c_vec.offset(state as isize);
    if count != 0 as libc::c_int {
        return count;
    }
    *c_vec.offset(state as isize) = -(1 as libc::c_int);
    p = *((*GAe).A_p).offset(state as isize);
    while p < *((*GAe).A_p).offset((state + 1 as libc::c_int) as isize) {
        if (*p).A_b == 1 as libc::c_int {
            count += 1;
        } else {
            i = A_cd_DFS((*p).A_c);
            if i < 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
            count += i;
        }
        p = p.offset(1);
    }
    let ref mut fresh1 = *c_vec.offset(state as isize);
    *fresh1 = count;
    return *fresh1;
}
#[no_mangle]
pub unsafe extern "C" fn A_card(mut A: A_OBJECT) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if A.is_null() {
        Error(
            b"A_card: No OBJECT\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    A = A_subs(A);
    c_vec = i_alloc((*A).A_nQ);
    i = 0 as libc::c_int;
    while i < (*A).A_nQ {
        *c_vec.offset(i as isize) = 0 as libc::c_int;
        i += 1;
    }
    GAe = A;
    i = A_cd_DFS(0 as libc::c_int);
    Sfree(c_vec as *mut libc::c_char);
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn A_pref(mut A: A_OBJECT) -> A_OBJECT {
    let mut i: libc::c_int = 0;
    A = A_open(A_min(A));
    i = (*A).A_nQ;
    loop {
        i -= 1;
        if !(i >= 2 as libc::c_int) {
            break;
        }
        A = A_add(A, i, 1 as libc::c_int, 1 as libc::c_int);
    }
    A = A_add(A, 0 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int);
    A = A_close(A);
    (*A).A_mode = 6 as libc::c_int;
    return A;
}
#[no_mangle]
pub unsafe extern "C" fn A_suff(mut A: A_OBJECT) -> A_OBJECT {
    let mut i: libc::c_int = 0;
    A = A_open(A_min(A));
    i = (*A).A_nQ;
    loop {
        i -= 1;
        if !(i >= 2 as libc::c_int) {
            break;
        }
        A = A_add(A, 0 as libc::c_int, 0 as libc::c_int, i);
    }
    A = A_add(A, 0 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int);
    A = A_close(A);
    (*A).A_mode = 5 as libc::c_int;
    return A;
}
