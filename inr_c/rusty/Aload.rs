use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn A_sseq(_: A_OBJECT) -> A_OBJECT;
    fn A_union(_: A_OBJECT, _: A_OBJECT) -> A_OBJECT;
    fn A_min(_: A_OBJECT) -> A_OBJECT;
    fn A_load_save(_: *mut libc::c_char, _: T2_OBJECT) -> A_OBJECT;
    fn A_load_pr(_: *mut libc::c_char, _: T2_OBJECT) -> A_OBJECT;
    fn A_close(_: A_OBJECT) -> A_OBJECT;
    fn A_add(_: A_OBJECT, _: libc::c_int, _: libc::c_int, _: libc::c_int) -> A_OBJECT;
    fn A_destroy(_: A_OBJECT);
    fn A_create() -> A_OBJECT;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn Warning(_: *mut libc::c_char);
    fn Error(_: *mut libc::c_char);
    fn T2_insert(_: T2_OBJECT, _: *mut libc::c_char, _: libc::c_int) -> libc::c_int;
    fn T2_name_pr(_: T2_OBJECT, _: libc::c_int) -> *mut libc::c_char;
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
static mut fp: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut fpin: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut fpout: *mut FILE = 0 as *const FILE as *mut FILE;
static mut c: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn get_name() -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    static mut token: [libc::c_char; 1024] = [0; 1024];
    while c == ' ' as i32 || c == '\t' as i32 {
        c = getc(fp);
    }
    i = 0 as libc::c_int;
    while c != ' ' as i32 && c != '\t' as i32 && c != '\n' as i32 {
        if c == -(1 as libc::c_int) {
            Warning(
                b"get_name: Unexpected EOF\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            return 0 as *mut libc::c_char;
        }
        if c == '\\' as i32 {
            c = getc(fp);
            match c {
                110 => {
                    c = '\n' as i32;
                }
                116 => {
                    c = '\t' as i32;
                }
                95 => {
                    c = ' ' as i32;
                }
                _ => {}
            }
        }
        let fresh0 = i;
        i = i + 1;
        token[fresh0 as usize] = c as libc::c_char;
        c = getc(fp);
        if i >= 1024 as libc::c_int {
            Warning(
                b"get_name: Name too long\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            return 0 as *mut libc::c_char;
        }
    }
    if i == 0 as libc::c_int {
        Warning(
            b"get_name: Zero length name\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return 0 as *mut libc::c_char;
    }
    token[i as usize] = 0 as libc::c_int as libc::c_char;
    return token.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn get_nl() -> libc::c_int {
    while c == ' ' as i32 || c == '\t' as i32 {
        c = getc(fp);
    }
    if c != '\n' as i32 {
        Warning(
            b"get_nl: New line expected\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return 0 as libc::c_int;
    }
    c = getc(fp);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn A_load(
    mut file: *mut libc::c_char,
    mut T2_Sigma: T2_OBJECT,
) -> A_OBJECT {
    let mut from: libc::c_int = 0;
    let mut symb: libc::c_int = 0;
    let mut to: libc::c_int = 0;
    let mut tape: libc::c_int = 0;
    let mut ntapes: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut A: A_OBJECT = 0 as *mut A_desc;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut A_row = 0 as *mut A_row;
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
    ntapes = 1 as libc::c_int;
    (*A).A_nT = ntapes;
    c = getc(fp);
    if c == 'I' as i32 {
        A_destroy(A);
        fclose(fp);
        return A_load_save(file, T2_Sigma);
    } else if c >= 0 as libc::c_int {
        A_destroy(A);
        fclose(fp);
        return A_load_pr(file, T2_Sigma);
    } else {
        ntapes = c + 1 as libc::c_int;
        (*A).A_nT = ntapes;
        c = getc(fp);
        if get_nl() == 0 {
            A_destroy(A);
            return 0 as A_OBJECT;
        }
        while c != -(1 as libc::c_int) {
            from = (c & 0o377 as libc::c_int) * 256 as libc::c_int;
            c = getc(fp);
            from += c & 0o377 as libc::c_int;
            c = getc(fp);
            to = (c & 0o377 as libc::c_int) * 256 as libc::c_int;
            c = getc(fp);
            to += c & 0o377 as libc::c_int;
            c = getc(fp);
            t = get_name();
            if t.is_null() {
                A_destroy(A);
                return 0 as A_OBJECT;
            }
            if *t.offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32
                && *t.offset(0 as libc::c_int as isize) as libc::c_int >= '0' as i32
                && *t.offset(0 as libc::c_int as isize) as libc::c_int <= '9' as i32
            {
                tape = *t.offset(0 as libc::c_int as isize) as libc::c_int - '0' as i32;
                if tape >= ntapes {
                    p = ((*A).A_t).offset((*A).A_nrows as isize);
                    loop {
                        p = p.offset(-1);
                        if !(p >= (*A).A_t) {
                            break;
                        }
                        i = (*p).A_b;
                        if i > 1 as libc::c_int {
                            i = i / ntapes * (tape + 1 as libc::c_int) + i % ntapes;
                        }
                        (*p).A_b = i;
                    }
                    ntapes = tape + 1 as libc::c_int;
                    (*A).A_nT = ntapes;
                }
                symb = T2_insert(
                    T2_Sigma,
                    t.offset(2 as libc::c_int as isize),
                    strlen(t.offset(2 as libc::c_int as isize)) as libc::c_int,
                );
                if symb == 1 as libc::c_int && ntapes > 1 as libc::c_int {
                    (*A).A_ems = 1 as libc::c_int;
                }
                symb = symb * ntapes + tape;
            } else {
                symb = T2_insert(T2_Sigma, t, strlen(t) as libc::c_int);
                if symb == 1 as libc::c_int && ntapes > 1 as libc::c_int {
                    (*A).A_ems = 1 as libc::c_int;
                }
                if symb != 1 as libc::c_int {
                    symb *= ntapes;
                }
            }
            A = A_add(A, from, symb, to);
            if get_nl() == 0 {
                A_destroy(A);
                return 0 as A_OBJECT;
            }
        }
        if !file.is_null() {
            fclose(fp);
        }
        A = A_close(A);
        (*A).A_mode = 7 as libc::c_int;
        return A;
    };
}
#[no_mangle]
pub unsafe extern "C" fn A_lwds(
    mut file: *mut libc::c_char,
    mut T2_Sigma: T2_OBJECT,
) -> A_OBJECT {
    let mut A: A_OBJECT = 0 as *mut A_desc;
    let mut As: A_OBJECT = 0 as *mut A_desc;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut nQ: libc::c_int = 0;
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
    if !T2_Sigma.is_null() {} else {
        __assert_fail(
            b"T2_Sigma != NULL\0" as *const u8 as *const libc::c_char,
            b"Aload.c\0" as *const u8 as *const libc::c_char,
            182 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"A_OBJECT A_lwds(char *, T2_OBJECT)\0"))
                .as_ptr(),
        );
    }
    if (*(*T2_Sigma).T2_int).Tn_n >= 258 as libc::c_int {} else {
        __assert_fail(
            b"T2_Sigma-> T2_int-> Tn_n >= 258\0" as *const u8 as *const libc::c_char,
            b"Aload.c\0" as *const u8 as *const libc::c_char,
            183 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"A_OBJECT A_lwds(char *, T2_OBJECT)\0"))
                .as_ptr(),
        );
    }
    A = A_create();
    As = A_create();
    c = getc(fp);
    while c != -(1 as libc::c_int) {
        if (*A).A_nQ > 64000 as libc::c_int || (*A).A_nrows > 100000 as libc::c_int {
            fprintf(
                fpout,
                b"Pause to compress ...\n\0" as *const u8 as *const libc::c_char,
            );
            A = A_close(A);
            (*A).A_mode = 5 as libc::c_int;
            As = A_union(A_min(A), A_min(As));
            As = A_close(As);
            (*As).A_mode = 5 as libc::c_int;
            A = A_create();
            fprintf(
                fpout,
                b"Done Compress (states %d transitions %d)\n\0" as *const u8
                    as *const libc::c_char,
                (*As).A_nQ,
                (*As).A_nrows,
            );
        }
        nQ = (*A).A_nQ;
        p = get_name();
        if p.is_null() {
            A_destroy(A);
            A_destroy(As);
            return 0 as A_OBJECT;
        }
        i = 0 as libc::c_int;
        while *p.offset(i as isize) as libc::c_int != 0 as libc::c_int {
            A = A_add(
                A,
                if i == 0 as libc::c_int {
                    0 as libc::c_int
                } else {
                    nQ + i - 1 as libc::c_int
                },
                (*p.offset(i as isize) as libc::c_int & 0xff as libc::c_int)
                    + 2 as libc::c_int,
                nQ + i,
            );
            i += 1;
        }
        A = A_add(A, nQ + i - 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int);
        if get_nl() == 0 {
            A_destroy(A);
            A_destroy(As);
            return 0 as A_OBJECT;
        }
    }
    if !file.is_null() {
        fclose(fp);
    }
    A = A_close(A);
    (*A).A_mode = 5 as libc::c_int;
    As = A_union(A_min(A), A_min(As));
    As = A_close(As);
    (*As).A_mode = 5 as libc::c_int;
    return As;
}
#[no_mangle]
pub unsafe extern "C" fn A_prsseq(
    mut A: A_OBJECT,
    mut file: *mut libc::c_char,
    mut T2_Sigma: T2_OBJECT,
) -> A_OBJECT {
    let mut t: libc::c_int = 0;
    let mut p: *mut A_row = 0 as *mut A_row;
    let mut pz: *mut A_row = 0 as *mut A_row;
    let mut i: libc::c_int = 0;
    let mut n_read: libc::c_int = 0;
    let mut n_write: libc::c_int = 0;
    let mut ss_states: libc::c_int = 0 as libc::c_int;
    if A.is_null() || T2_Sigma.is_null() {
        return A;
    }
    if (*A).A_nT != 2 as libc::c_int {
        Error(
            b"A_prsseq: Not two tapes\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if (*A).A_mode < 8 as libc::c_int {
        A = A_min(A_sseq(A));
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
    i = 0 as libc::c_int;
    while i < (*A).A_nQ {
        n_read = 0 as libc::c_int;
        n_write = 0 as libc::c_int;
        pz = *((*A).A_p).offset((i + 1 as libc::c_int) as isize);
        p = *((*A).A_p).offset(i as isize);
        while p < pz {
            if (*p).A_b % 2 as libc::c_int == 0 as libc::c_int {
                n_read += 1;
            } else {
                n_write += 1;
            }
            p = p.offset(1);
        }
        if n_read > 0 as libc::c_int && n_write > 0 as libc::c_int
            || n_write > 1 as libc::c_int
        {
            Error(
                b"A_prsseq: Not Subsequential\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        if i == 0 as libc::c_int || n_write == 0 as libc::c_int {
            ss_states += 1;
            pz = *((*A).A_p).offset((i + 1 as libc::c_int) as isize);
            p = *((*A).A_p).offset(i as isize);
            while p < pz {
                t = (*p).A_a;
                if t == 0 as libc::c_int {
                    fprintf(fp, b"(START)  \0" as *const u8 as *const libc::c_char);
                } else if t == 1 as libc::c_int {
                    fprintf(fp, b"(FINAL)  \0" as *const u8 as *const libc::c_char);
                } else {
                    fprintf(fp, b"%-7d  \0" as *const u8 as *const libc::c_char, t);
                }
                if n_write == 0 as libc::c_int {
                    fprintf(
                        fp,
                        b"%s\0" as *const u8 as *const libc::c_char,
                        T2_name_pr(T2_Sigma, (*p).A_b / 2 as libc::c_int),
                    );
                    t = (*p).A_c;
                    fprintf(fp, b" [\0" as *const u8 as *const libc::c_char);
                } else {
                    t = (*p).A_a;
                    fprintf(fp, b"[\0" as *const u8 as *const libc::c_char);
                }
                while (*((*A).A_p).offset(t as isize)).offset(1 as libc::c_int as isize)
                    == *((*A).A_p).offset((t + 1 as libc::c_int) as isize)
                    && (**((*A).A_p).offset(t as isize)).A_b % 2 as libc::c_int
                        == 1 as libc::c_int
                {
                    if (**((*A).A_p).offset(t as isize)).A_b != 1 as libc::c_int {
                        fprintf(
                            fp,
                            b" %s\0" as *const u8 as *const libc::c_char,
                            T2_name_pr(
                                T2_Sigma,
                                (**((*A).A_p).offset(t as isize)).A_b / 2 as libc::c_int,
                            ),
                        );
                    }
                    t = (**((*A).A_p).offset(t as isize)).A_c;
                }
                fprintf(fp, b" ]  \t\0" as *const u8 as *const libc::c_char);
                if t == 0 as libc::c_int {
                    fprintf(fp, b"(START)\n\0" as *const u8 as *const libc::c_char);
                } else if t == 1 as libc::c_int {
                    fprintf(fp, b"(FINAL)\n\0" as *const u8 as *const libc::c_char);
                } else {
                    fprintf(fp, b"%d\n\0" as *const u8 as *const libc::c_char, t);
                }
                p = p.offset(1);
            }
        }
        i += 1;
    }
    fprintf(
        fp,
        b"\n%d states in subsequential transducer\n\0" as *const u8
            as *const libc::c_char,
        ss_states,
    );
    if fp != stdout {
        printf(
            b"%d states in subsequential transducer\n\0" as *const u8
                as *const libc::c_char,
            ss_states,
        );
    }
    if !file.is_null() {
        fclose(fp);
    } else if fflush(stdout) == -(1 as libc::c_int) {
        Error(
            b"A_store: fflush\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    return A;
}
