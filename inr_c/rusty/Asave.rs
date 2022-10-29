use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    static mut fpin: *mut FILE;
    static mut fpout: *mut FILE;
    fn Salloc(_: libc::c_long) -> *mut libc::c_char;
    fn Sfree(_: *mut libc::c_char);
    fn Srealloc(_: *mut libc::c_char, _: libc::c_long) -> *mut libc::c_char;
    fn Ssize(_: *mut libc::c_char) -> libc::c_long;
    fn Warning(_: *mut libc::c_char);
    fn Error(_: *mut libc::c_char);
    fn T2_insert(_: T2_OBJECT, _: *mut libc::c_char, _: libc::c_int) -> libc::c_int;
    fn T2_name(_: T2_OBJECT, _: libc::c_int) -> *mut libc::c_char;
    fn T2_length(_: T2_OBJECT, _: libc::c_int) -> libc::c_int;
    fn A_create() -> A_OBJECT;
    fn A_destroy(_: A_OBJECT);
    fn A_add(_: A_OBJECT, _: libc::c_int, _: libc::c_int, _: libc::c_int) -> A_OBJECT;
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
static mut error_code: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn A_save(
    mut A: A_OBJECT,
    mut file: *mut libc::c_char,
    mut T2_Sigma: T2_OBJECT,
) -> A_OBJECT {
    let mut t: libc::c_int = 0;
    let mut tape_number: libc::c_int = 0;
    let mut label_length: libc::c_int = 0;
    let mut tt: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut label_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut A_row = 0 as *mut A_row;
    let mut pz: *mut A_row = 0 as *mut A_row;
    let mut fp: *mut FILE = 0 as *mut FILE;
    if A.is_null() || T2_Sigma.is_null() {
        return A;
    }
    if !file.is_null() {
        if strcmp(file, b"devnull\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            return A
        } else {
            fp = fopen(file, b"w\0" as *const u8 as *const libc::c_char);
        }
    } else {
        fp = fpout;
        if fp.is_null() {
            fp = stdin;
        }
    }
    if fp.is_null() {
        Warning(
            b"Cannot open file\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return A;
    }
    fprintf(
        fp,
        b"INR210\t%d\t%d\t%d\t%d\n\0" as *const u8 as *const libc::c_char,
        (*A).A_nT,
        (*A).A_nrows,
        (*A).A_nQ,
        (*A).A_nS,
    );
    pz = ((*A).A_t).offset((*A).A_nrows as isize);
    p = (*A).A_t;
    while p < pz {
        t = (*p).A_b;
        if t == 0 as libc::c_int {
            tape_number = -(1 as libc::c_int);
            label_length = 0 as libc::c_int;
            label_name = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        } else if t == 1 as libc::c_int {
            tape_number = 0 as libc::c_int;
            label_length = 0 as libc::c_int;
            label_name = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        } else if (*A).A_nT == 1 as libc::c_int {
            tape_number = 0 as libc::c_int;
            label_length = T2_length(T2_Sigma, t);
            label_name = T2_name(T2_Sigma, t);
        } else {
            tape_number = t % (*A).A_nT;
            tt = t / (*A).A_nT;
            if tt == 1 as libc::c_int {
                label_length = 0 as libc::c_int;
                label_name = b"\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            } else {
                label_length = T2_length(T2_Sigma, tt);
                label_name = T2_name(T2_Sigma, tt);
            }
        }
        fprintf(
            fp,
            b"%d\t%d\t%d\t%d\t\0" as *const u8 as *const libc::c_char,
            (*p).A_a,
            (*p).A_c,
            tape_number,
            label_length,
        );
        i = 0 as libc::c_int;
        while i < label_length {
            putc(*label_name.offset(i as isize) as libc::c_int, fp);
            i += 1;
        }
        putc('\n' as i32, fp);
        p = p.offset(1);
    }
    if !file.is_null() {
        fclose(fp);
    } else if fflush(stdout) == -(1 as libc::c_int) {
        Error(
            b"A_save: fflush\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    return A;
}
#[no_mangle]
pub unsafe extern "C" fn A_load_save(
    mut file: *mut libc::c_char,
    mut T2_Sigma: T2_OBJECT,
) -> A_OBJECT {
    let mut current_block: u64;
    let mut c: libc::c_int = 0;
    let mut number_tapes: libc::c_int = 0;
    let mut number_rows: libc::c_int = 0;
    let mut number_states: libc::c_int = 0;
    let mut number_symbols: libc::c_int = 0;
    let mut from_state: libc::c_int = 0;
    let mut to_state: libc::c_int = 0;
    let mut row_no: libc::c_int = 0;
    let mut tape_no: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    let mut label: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut A: A_OBJECT = 0 as *mut A_desc;
    buffer = Salloc(100 as libc::c_int as libc::c_long);
    if !file.is_null() {
        fp = fopen(file, b"r\0" as *const u8 as *const libc::c_char);
    } else if !fpin.is_null() {
        fp = fpin;
    } else {
        fp = stdin;
    }
    if fp.is_null() {
        Warning(
            b"File does not exist\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return 0 as A_OBJECT;
    }
    A = A_create();
    if T2_Sigma.is_null()
        || T2_insert(
            T2_Sigma,
            b"^^\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            2 as libc::c_int,
        ) != 0 as libc::c_int
        || T2_insert(
            T2_Sigma,
            b"-|\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            2 as libc::c_int,
        ) != 1 as libc::c_int
    {
        Error(
            b"A_load: BOTCH 1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    c = getc(fp);
    if c != 'I' as i32 {
        error_code = 1 as libc::c_int;
    } else {
        c = getc(fp);
        if c != 'N' as i32 {
            error_code = 2 as libc::c_int;
        } else {
            c = getc(fp);
            if c != 'R' as i32 {
                error_code = 3 as libc::c_int;
            } else {
                c = getc(fp);
                if c != '2' as i32 {
                    error_code = 4 as libc::c_int;
                } else {
                    c = getc(fp);
                    if c != '1' as i32 {
                        error_code = 5 as libc::c_int;
                    } else {
                        c = getc(fp);
                        if c != '0' as i32 {
                            error_code = 6 as libc::c_int;
                        } else {
                            c = getc(fp);
                            if c != '\t' as i32 {
                                error_code = 7 as libc::c_int;
                            } else {
                                c = getc(fp);
                                if c < '0' as i32 || c > '9' as i32 {
                                    error_code = 8 as libc::c_int;
                                } else {
                                    number_tapes = c - '0' as i32;
                                    c = getc(fp);
                                    loop {
                                        if !(c != '\t' as i32) {
                                            current_block = 5028470053297453708;
                                            break;
                                        }
                                        if c < '0' as i32 || c > '9' as i32 {
                                            error_code = 9 as libc::c_int;
                                            current_block = 2846881917047551707;
                                            break;
                                        } else {
                                            number_tapes = number_tapes * 10 as libc::c_int
                                                + (c - '0' as i32);
                                            c = getc(fp);
                                            if !(number_tapes >= 0o17777777777 as libc::c_int) {
                                                continue;
                                            }
                                            error_code = 10 as libc::c_int;
                                            current_block = 2846881917047551707;
                                            break;
                                        }
                                    }
                                    match current_block {
                                        2846881917047551707 => {}
                                        _ => {
                                            (*A).A_nT = number_tapes;
                                            if c != '\t' as i32 {
                                                error_code = 11 as libc::c_int;
                                            } else {
                                                c = getc(fp);
                                                if c < '0' as i32 || c > '9' as i32 {
                                                    error_code = 12 as libc::c_int;
                                                } else {
                                                    number_rows = c - '0' as i32;
                                                    c = getc(fp);
                                                    loop {
                                                        if !(c != '\t' as i32) {
                                                            current_block = 6471821049853688503;
                                                            break;
                                                        }
                                                        if c < '0' as i32 || c > '9' as i32 {
                                                            error_code = 13 as libc::c_int;
                                                            current_block = 2846881917047551707;
                                                            break;
                                                        } else {
                                                            number_rows = number_rows * 10 as libc::c_int
                                                                + (c - '0' as i32);
                                                            c = getc(fp);
                                                            if !(number_rows >= 0o17777777777 as libc::c_int) {
                                                                continue;
                                                            }
                                                            error_code = 14 as libc::c_int;
                                                            current_block = 2846881917047551707;
                                                            break;
                                                        }
                                                    }
                                                    match current_block {
                                                        2846881917047551707 => {}
                                                        _ => {
                                                            if c != '\t' as i32 {
                                                                error_code = 15 as libc::c_int;
                                                            } else {
                                                                c = getc(fp);
                                                                if c < '0' as i32 || c > '9' as i32 {
                                                                    error_code = 16 as libc::c_int;
                                                                } else {
                                                                    number_states = c - '0' as i32;
                                                                    c = getc(fp);
                                                                    loop {
                                                                        if !(c != '\t' as i32) {
                                                                            current_block = 12079920068676227593;
                                                                            break;
                                                                        }
                                                                        if c < '0' as i32 || c > '9' as i32 {
                                                                            error_code = 17 as libc::c_int;
                                                                            current_block = 2846881917047551707;
                                                                            break;
                                                                        } else {
                                                                            number_states = number_states * 10 as libc::c_int
                                                                                + (c - '0' as i32);
                                                                            c = getc(fp);
                                                                            if !(number_states >= 0o17777777777 as libc::c_int) {
                                                                                continue;
                                                                            }
                                                                            error_code = 18 as libc::c_int;
                                                                            current_block = 2846881917047551707;
                                                                            break;
                                                                        }
                                                                    }
                                                                    match current_block {
                                                                        2846881917047551707 => {}
                                                                        _ => {
                                                                            if c != '\t' as i32 {
                                                                                error_code = 19 as libc::c_int;
                                                                            } else {
                                                                                c = getc(fp);
                                                                                if c < '0' as i32 || c > '9' as i32 {
                                                                                    error_code = 20 as libc::c_int;
                                                                                } else {
                                                                                    number_symbols = c - '0' as i32;
                                                                                    c = getc(fp);
                                                                                    loop {
                                                                                        if !(c != '\n' as i32) {
                                                                                            current_block = 10763371041174037105;
                                                                                            break;
                                                                                        }
                                                                                        if c < '0' as i32 || c > '9' as i32 {
                                                                                            error_code = 21 as libc::c_int;
                                                                                            current_block = 2846881917047551707;
                                                                                            break;
                                                                                        } else {
                                                                                            number_symbols = number_symbols * 10 as libc::c_int
                                                                                                + (c - '0' as i32);
                                                                                            c = getc(fp);
                                                                                            if !(number_symbols >= 0o17777777777 as libc::c_int) {
                                                                                                continue;
                                                                                            }
                                                                                            error_code = 22 as libc::c_int;
                                                                                            current_block = 2846881917047551707;
                                                                                            break;
                                                                                        }
                                                                                    }
                                                                                    match current_block {
                                                                                        2846881917047551707 => {}
                                                                                        _ => {
                                                                                            if c != '\n' as i32 {
                                                                                                error_code = 23 as libc::c_int;
                                                                                            } else {
                                                                                                c = getc(fp);
                                                                                                row_no = -(1 as libc::c_int);
                                                                                                'c_2044: loop {
                                                                                                    row_no += 1;
                                                                                                    if row_no >= number_rows {
                                                                                                        error_code = 24 as libc::c_int;
                                                                                                        break;
                                                                                                    } else if c < '0' as i32 || c > '9' as i32 {
                                                                                                        error_code = 25 as libc::c_int;
                                                                                                        break;
                                                                                                    } else {
                                                                                                        from_state = c - '0' as i32;
                                                                                                        c = getc(fp);
                                                                                                        while c != '\t' as i32 {
                                                                                                            if c < '0' as i32 || c > '9' as i32 {
                                                                                                                error_code = 26 as libc::c_int;
                                                                                                                break 'c_2044;
                                                                                                            } else {
                                                                                                                from_state = from_state * 10 as libc::c_int
                                                                                                                    + (c - '0' as i32);
                                                                                                                c = getc(fp);
                                                                                                                if !(from_state >= 0o17777777777 as libc::c_int) {
                                                                                                                    continue;
                                                                                                                }
                                                                                                                error_code = 27 as libc::c_int;
                                                                                                                break 'c_2044;
                                                                                                            }
                                                                                                        }
                                                                                                        if c != '\t' as i32 {
                                                                                                            error_code = 28 as libc::c_int;
                                                                                                            break;
                                                                                                        } else {
                                                                                                            c = getc(fp);
                                                                                                            if from_state >= number_states {
                                                                                                                error_code = 29 as libc::c_int;
                                                                                                                break;
                                                                                                            } else if c < '0' as i32 || c > '9' as i32 {
                                                                                                                error_code = 30 as libc::c_int;
                                                                                                                break;
                                                                                                            } else {
                                                                                                                to_state = c - '0' as i32;
                                                                                                                c = getc(fp);
                                                                                                                while c != '\t' as i32 {
                                                                                                                    if c < '0' as i32 || c > '9' as i32 {
                                                                                                                        error_code = 31 as libc::c_int;
                                                                                                                        break 'c_2044;
                                                                                                                    } else {
                                                                                                                        to_state = to_state * 10 as libc::c_int + (c - '0' as i32);
                                                                                                                        c = getc(fp);
                                                                                                                        if !(to_state >= 0o17777777777 as libc::c_int) {
                                                                                                                            continue;
                                                                                                                        }
                                                                                                                        error_code = 32 as libc::c_int;
                                                                                                                        break 'c_2044;
                                                                                                                    }
                                                                                                                }
                                                                                                                if c != '\t' as i32 {
                                                                                                                    error_code = 33 as libc::c_int;
                                                                                                                    break;
                                                                                                                } else {
                                                                                                                    c = getc(fp);
                                                                                                                    if to_state >= number_states {
                                                                                                                        error_code = 34 as libc::c_int;
                                                                                                                        break;
                                                                                                                    } else {
                                                                                                                        if c == '-' as i32 {
                                                                                                                            c = getc(fp);
                                                                                                                            if c != '1' as i32 {
                                                                                                                                error_code = 35 as libc::c_int;
                                                                                                                                break;
                                                                                                                            } else {
                                                                                                                                c = getc(fp);
                                                                                                                                tape_no = -(1 as libc::c_int);
                                                                                                                            }
                                                                                                                        } else if c < '0' as i32 || c > '9' as i32 {
                                                                                                                            error_code = 36 as libc::c_int;
                                                                                                                            break;
                                                                                                                        } else {
                                                                                                                            tape_no = c - '0' as i32;
                                                                                                                            c = getc(fp);
                                                                                                                            while c != '\t' as i32 {
                                                                                                                                if c < '0' as i32 || c > '9' as i32 {
                                                                                                                                    error_code = 37 as libc::c_int;
                                                                                                                                    break 'c_2044;
                                                                                                                                } else {
                                                                                                                                    tape_no = tape_no * 10 as libc::c_int + (c - '0' as i32);
                                                                                                                                    c = getc(fp);
                                                                                                                                    if !(tape_no >= 0o17777777777 as libc::c_int) {
                                                                                                                                        continue;
                                                                                                                                    }
                                                                                                                                    error_code = 38 as libc::c_int;
                                                                                                                                    break 'c_2044;
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                        if c != '\t' as i32 {
                                                                                                                            error_code = 39 as libc::c_int;
                                                                                                                            break;
                                                                                                                        } else {
                                                                                                                            c = getc(fp);
                                                                                                                            if tape_no >= number_tapes {
                                                                                                                                error_code = 40 as libc::c_int;
                                                                                                                                break;
                                                                                                                            } else if c < '0' as i32 || c > '9' as i32 {
                                                                                                                                error_code = 41 as libc::c_int;
                                                                                                                                break;
                                                                                                                            } else {
                                                                                                                                length = c - '0' as i32;
                                                                                                                                c = getc(fp);
                                                                                                                                while c != '\t' as i32 {
                                                                                                                                    if c < '0' as i32 || c > '9' as i32 {
                                                                                                                                        error_code = 42 as libc::c_int;
                                                                                                                                        break 'c_2044;
                                                                                                                                    } else {
                                                                                                                                        length = length * 10 as libc::c_int + (c - '0' as i32);
                                                                                                                                        c = getc(fp);
                                                                                                                                        if !(length >= 0o17777777777 as libc::c_int) {
                                                                                                                                            continue;
                                                                                                                                        }
                                                                                                                                        error_code = 43 as libc::c_int;
                                                                                                                                        break 'c_2044;
                                                                                                                                    }
                                                                                                                                }
                                                                                                                                if c != '\t' as i32 {
                                                                                                                                    error_code = 44 as libc::c_int;
                                                                                                                                    break;
                                                                                                                                } else {
                                                                                                                                    c = getc(fp);
                                                                                                                                    if (length + 1 as libc::c_int) as libc::c_long
                                                                                                                                        >= Ssize(buffer)
                                                                                                                                    {
                                                                                                                                        buffer = Srealloc(
                                                                                                                                            buffer,
                                                                                                                                            (length + 1 as libc::c_int) as libc::c_long,
                                                                                                                                        );
                                                                                                                                    }
                                                                                                                                    i = 0 as libc::c_int;
                                                                                                                                    while i < length {
                                                                                                                                        let fresh0 = i;
                                                                                                                                        i = i + 1;
                                                                                                                                        *buffer.offset(fresh0 as isize) = c as libc::c_char;
                                                                                                                                        c = getc(fp);
                                                                                                                                        if !(c == -(1 as libc::c_int)) {
                                                                                                                                            continue;
                                                                                                                                        }
                                                                                                                                        error_code = 45 as libc::c_int;
                                                                                                                                        break 'c_2044;
                                                                                                                                    }
                                                                                                                                    *buffer.offset(i as isize) = '\0' as i32 as libc::c_char;
                                                                                                                                    if c != '\n' as i32 {
                                                                                                                                        error_code = 46 as libc::c_int;
                                                                                                                                        break;
                                                                                                                                    } else {
                                                                                                                                        c = getc(fp);
                                                                                                                                        if tape_no == -(1 as libc::c_int) {
                                                                                                                                            A = A_add(A, from_state, 0 as libc::c_int, to_state);
                                                                                                                                            if length != 0 as libc::c_int {
                                                                                                                                                error_code = 47 as libc::c_int;
                                                                                                                                                break;
                                                                                                                                            }
                                                                                                                                        } else if length == 0 as libc::c_int {
                                                                                                                                            if to_state == 1 as libc::c_int {
                                                                                                                                                label = 1 as libc::c_int;
                                                                                                                                            } else {
                                                                                                                                                label = 1 as libc::c_int * number_tapes + tape_no;
                                                                                                                                            }
                                                                                                                                            A = A_add(A, from_state, label, to_state);
                                                                                                                                        } else {
                                                                                                                                            index = T2_insert(T2_Sigma, buffer, length);
                                                                                                                                            if index >= number_symbols {
                                                                                                                                                error_code = 48 as libc::c_int;
                                                                                                                                                break;
                                                                                                                                            } else {
                                                                                                                                                label = index * number_tapes + tape_no;
                                                                                                                                                A = A_add(A, from_state, label, to_state);
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                        if c != -(1 as libc::c_int) {
                                                                                                                                            continue;
                                                                                                                                        }
                                                                                                                                        Sfree(buffer);
                                                                                                                                        return A;
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    fprintf(
        fpout,
        b"A_load_save: Error code = %d\n\0" as *const u8 as *const libc::c_char,
        error_code,
    );
    Error(
        b"Illegal save format\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    Sfree(buffer);
    A_destroy(A);
    return 0 as A_OBJECT;
}
