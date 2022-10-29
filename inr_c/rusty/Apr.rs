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
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut fpin: *mut FILE;
    static mut fpout: *mut FILE;
    fn Salloc(_: libc::c_long) -> *mut libc::c_char;
    fn Sfree(_: *mut libc::c_char);
    fn Srealloc(_: *mut libc::c_char, _: libc::c_long) -> *mut libc::c_char;
    fn Ssize(_: *mut libc::c_char) -> libc::c_long;
    fn Warning(_: *mut libc::c_char);
    fn Error(_: *mut libc::c_char);
    fn Tn_create() -> Tn_OBJECT;
    fn Tn_destroy(_: Tn_OBJECT);
    fn Tn_insert(_: Tn_OBJECT, _: *mut libc::c_char, _: libc::c_int) -> libc::c_int;
    fn T2_insert(_: T2_OBJECT, _: *mut libc::c_char, _: libc::c_int) -> libc::c_int;
    fn T2_name_pr(_: T2_OBJECT, _: libc::c_int) -> *mut libc::c_char;
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
pub unsafe extern "C" fn A_pr(
    mut A: A_OBJECT,
    mut file: *mut libc::c_char,
    mut T2_Sigma: T2_OBJECT,
) -> A_OBJECT {
    let mut t: libc::c_int = 0;
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
    pz = ((*A).A_t).offset((*A).A_nrows as isize);
    p = (*A).A_t;
    while p < pz {
        t = (*p).A_a;
        if t == 0 as libc::c_int {
            fprintf(fp, b"(START) \0" as *const u8 as *const libc::c_char);
        } else if t == 1 as libc::c_int {
            fprintf(fp, b"(FINAL) \0" as *const u8 as *const libc::c_char);
        } else {
            fprintf(fp, b"%d \0" as *const u8 as *const libc::c_char, t);
        }
        t = (*p).A_b;
        if t <= 1 as libc::c_int || (*A).A_nT == 1 as libc::c_int {
            fprintf(
                fp,
                b"%s\0" as *const u8 as *const libc::c_char,
                T2_name_pr(T2_Sigma, t),
            );
        } else {
            fprintf(
                fp,
                b"%1d.%s\0" as *const u8 as *const libc::c_char,
                t % (*A).A_nT,
                T2_name_pr(T2_Sigma, t / (*A).A_nT),
            );
        }
        t = (*p).A_c;
        if t == 0 as libc::c_int {
            fprintf(fp, b" (START)\n\0" as *const u8 as *const libc::c_char);
        } else if t == 1 as libc::c_int {
            fprintf(fp, b" (FINAL)\n\0" as *const u8 as *const libc::c_char);
        } else {
            fprintf(fp, b" %d\n\0" as *const u8 as *const libc::c_char, t);
        }
        p = p.offset(1);
    }
    if !file.is_null() {
        fclose(fp);
    } else if fflush(stdout) == -(1 as libc::c_int) {
        Error(
            b"A_pr: fflush\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    return A;
}
#[no_mangle]
pub unsafe extern "C" fn A_load_pr(
    mut file: *mut libc::c_char,
    mut T2_Sigma: T2_OBJECT,
) -> A_OBJECT {
    let mut from_state: libc::c_int = 0;
    let mut to_state: libc::c_int = 0;
    let mut tape_number: libc::c_int = 0;
    let mut ntapes: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut transition_label: libc::c_int = 0;
    let mut A: A_OBJECT = 0 as *mut A_desc;
    let mut TQ: Tn_OBJECT = 0 as *mut Tn_desc;
    let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut A_row = 0 as *mut A_row;
    let mut fp: *mut FILE = 0 as *mut FILE;
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
    ntapes = 1 as libc::c_int;
    (*A).A_nT = ntapes;
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
    TQ = Tn_create();
    if Tn_insert(
        TQ,
        b"(START)\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        7 as libc::c_int,
    ) != 0 as libc::c_int
        || Tn_insert(
            TQ,
            b"(FINAL)\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            7 as libc::c_int,
        ) != 1 as libc::c_int
    {
        Error(
            b"A_load: BOTCH 2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    c = getc(fp);
    if Ssize(buffer) >= 100 as libc::c_int as libc::c_long {} else {
        __assert_fail(
            b"Ssize( buffer ) >= 100\0" as *const u8 as *const libc::c_char,
            b"Apr.c\0" as *const u8 as *const libc::c_char,
            104 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"A_OBJECT A_load_pr(char *, T2_OBJECT)\0"))
                .as_ptr(),
        );
    };
    'c_1925: loop {
        if c == ' ' as i32 || c == '\t' as i32 || c == '\n' as i32
            || c == -(1 as libc::c_int)
        {
            error_code = 1 as libc::c_int;
            break;
        } else {
            l = 0 as libc::c_int;
            if l as libc::c_long >= Ssize(buffer) {
                buffer = Srealloc(buffer, (2 as libc::c_int * l) as libc::c_long);
            }
            let fresh0 = l;
            l = l + 1;
            *buffer.offset(fresh0 as isize) = c as libc::c_char;
            c = getc(fp);
            while c != ' ' as i32 && c != '\t' as i32 && c != '\n' as i32
                && c != -(1 as libc::c_int)
            {
                if l as libc::c_long >= Ssize(buffer) {
                    buffer = Srealloc(buffer, (2 as libc::c_int * l) as libc::c_long);
                }
                let fresh1 = l;
                l = l + 1;
                *buffer.offset(fresh1 as isize) = c as libc::c_char;
                c = getc(fp);
            }
            *buffer.offset(l as isize) = '\0' as i32 as libc::c_char;
            from_state = Tn_insert(TQ, buffer, l);
            if c == '\n' as i32 || c == -(1 as libc::c_int) {
                error_code = 2 as libc::c_int;
                break;
            } else {
                if c == ' ' as i32 || c == '\t' as i32 {} else {
                    __assert_fail(
                        b"c == ' ' || c == '\\t'\0" as *const u8 as *const libc::c_char,
                        b"Apr.c\0" as *const u8 as *const libc::c_char,
                        131 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 38],
                            &[libc::c_char; 38],
                        >(b"A_OBJECT A_load_pr(char *, T2_OBJECT)\0"))
                            .as_ptr(),
                    );
                };
                c = getc(fp);
                l = 0 as libc::c_int;
                tape_number = 0 as libc::c_int;
                while c >= '0' as i32 && c <= '9' as i32 {
                    tape_number = tape_number * 10 as libc::c_int + (c - '0' as i32);
                    if l as libc::c_long >= Ssize(buffer) {
                        buffer = Srealloc(
                            buffer,
                            (2 as libc::c_int * l) as libc::c_long,
                        );
                    }
                    let fresh2 = l;
                    l = l + 1;
                    *buffer.offset(fresh2 as isize) = c as libc::c_char;
                    c = getc(fp);
                }
                if l > 0 as libc::c_int && c == '.' as i32 {
                    l = 0 as libc::c_int;
                    c = getc(fp);
                } else {
                    tape_number = -(1 as libc::c_int);
                }
                if tape_number >= ntapes {
                    p = ((*A).A_t).offset((*A).A_nrows as isize);
                    loop {
                        p = p.offset(-1);
                        if !(p >= (*A).A_t) {
                            break;
                        }
                        j = (*p).A_b;
                        if j > 1 as libc::c_int {
                            j = j / ntapes * (tape_number + 1 as libc::c_int)
                                + j % ntapes;
                        }
                        (*p).A_b = j;
                    }
                    ntapes = tape_number + 1 as libc::c_int;
                    (*A).A_nT = ntapes;
                }
                if c == ' ' as i32 || c == '\t' as i32 || c == '\n' as i32
                    || c == -(1 as libc::c_int)
                {
                    error_code = 3 as libc::c_int;
                    break;
                } else {
                    while c != ' ' as i32 && c != '\t' as i32 && c != '\n' as i32
                        && c != -(1 as libc::c_int)
                    {
                        if l as libc::c_long >= Ssize(buffer) {
                            buffer = Srealloc(
                                buffer,
                                (2 as libc::c_int * l) as libc::c_long,
                            );
                        }
                        if c == '\\' as i32 {
                            c = getc(fp);
                            if c == '_' as i32 {
                                c = ' ' as i32;
                            } else if c == 't' as i32 {
                                c = '\t' as i32;
                            } else if c == 'n' as i32 {
                                c = '\n' as i32;
                            } else if c == 'r' as i32 {
                                c = '\r' as i32;
                            } else if c == 'x' as i32 {
                                c = getc(fp);
                                if c >= '0' as i32 && c <= '9' as i32 {
                                    d = c - '0' as i32;
                                } else if c >= 'A' as i32 && c <= 'F' as i32 {
                                    d = c + 10 as libc::c_int - 'A' as i32;
                                } else if c >= 'a' as i32 && c <= 'f' as i32 {
                                    d = c + 10 as libc::c_int - 'a' as i32;
                                } else {
                                    error_code = 4 as libc::c_int;
                                    break 'c_1925;
                                }
                                d *= 16 as libc::c_int;
                                c = getc(fp);
                                if c >= '0' as i32 && c <= '9' as i32 {
                                    d += c - '0' as i32;
                                } else if c >= 'A' as i32 && c <= 'F' as i32 {
                                    d += c + 10 as libc::c_int - 'A' as i32;
                                } else if c >= 'a' as i32 && c <= 'f' as i32 {
                                    d += c + 10 as libc::c_int - 'a' as i32;
                                } else {
                                    error_code = 5 as libc::c_int;
                                    break 'c_1925;
                                }
                                c = d;
                            } else if c != -(1 as libc::c_int) {
                                error_code = 6 as libc::c_int;
                                break 'c_1925;
                            }
                        }
                        let fresh3 = l;
                        l = l + 1;
                        *buffer.offset(fresh3 as isize) = c as libc::c_char;
                        c = getc(fp);
                    }
                    *buffer.offset(l as isize) = '\0' as i32 as libc::c_char;
                    transition_label = T2_insert(T2_Sigma, buffer, l);
                    if c != ' ' as i32 && c != '\t' as i32 && c != '\n' as i32
                        && c != -(1 as libc::c_int)
                    {
                        error_code = 7 as libc::c_int;
                        break;
                    } else {
                        c = getc(fp);
                        l = 0 as libc::c_int;
                        while c != ' ' as i32 && c != '\t' as i32 && c != '\n' as i32
                            && c != -(1 as libc::c_int)
                        {
                            if l as libc::c_long >= Ssize(buffer) {
                                buffer = Srealloc(
                                    buffer,
                                    (2 as libc::c_int * l) as libc::c_long,
                                );
                            }
                            let fresh4 = l;
                            l = l + 1;
                            *buffer.offset(fresh4 as isize) = c as libc::c_char;
                            c = getc(fp);
                        }
                        *buffer.offset(l as isize) = '\0' as i32 as libc::c_char;
                        to_state = Tn_insert(TQ, buffer, l);
                        if c != '\n' as i32 && c != -(1 as libc::c_int) {
                            error_code = 8 as libc::c_int;
                            break;
                        } else {
                            c = getc(fp);
                            if tape_number >= 0 as libc::c_int {
                                A = A_add(
                                    A,
                                    from_state,
                                    transition_label * ntapes + tape_number,
                                    to_state,
                                );
                            } else {
                                A = A_add(A, from_state, transition_label, to_state);
                            }
                            if c != -(1 as libc::c_int) {
                                continue;
                            }
                            Sfree(buffer);
                            if !file.is_null() {
                                fclose(fp);
                            }
                            Tn_destroy(TQ);
                            return A;
                        }
                    }
                }
            }
        }
    }
    fprintf(
        fpout,
        b"A_load_pr: Error code = %d\n\0" as *const u8 as *const libc::c_char,
        error_code,
    );
    Error(
        b"Illegal pr format\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    Sfree(buffer);
    if !file.is_null() {
        fclose(fp);
    }
    A_destroy(A);
    Tn_destroy(TQ);
    return 0 as A_OBJECT;
}
