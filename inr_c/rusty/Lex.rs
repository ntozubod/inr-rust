use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn yyparse() -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn fileno(_: *mut FILE) -> libc::c_int;
    static mut fpin: *mut FILE;
    static mut fpout: *mut FILE;
    fn Salloc(_: libc::c_long) -> *mut libc::c_char;
    static mut Version: [libc::c_char; 0];
    static mut Date: [libc::c_char; 0];
    fn Error(_: *mut libc::c_char);
    fn pr_time_diff();
    fn Tn_create() -> Tn_OBJECT;
    fn Tn_destroy(_: Tn_OBJECT);
    fn Tn_insert(_: Tn_OBJECT, _: *mut libc::c_char, _: libc::c_int) -> libc::c_int;
    fn Tn_stats();
    fn T2_create() -> T2_OBJECT;
    fn T2_destroy(_: T2_OBJECT);
    fn T2_insert(_: T2_OBJECT, _: *mut libc::c_char, _: libc::c_int) -> libc::c_int;
    fn T2_name(_: T2_OBJECT, _: libc::c_int) -> *mut libc::c_char;
    fn T2_length(_: T2_OBJECT, _: libc::c_int) -> libc::c_int;
    fn P_create(_: libc::c_int, _: *mut libc::c_char) -> P_OBJECT;
    fn V_stats();
    fn R_stats();
    fn U_stats();
    static mut A_report: libc::c_int;
    fn A_create() -> A_OBJECT;
    static mut yylval: YYSTYPE;
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
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct P_desc {
    pub Type: libc::c_int,
    pub P_length: libc::c_int,
    pub P_cstr: *mut libc::c_char,
}
pub type P_OBJECT = *mut P_desc;
#[derive(Copy, Clone)]
#[repr(C)]
pub union YYSTYPE {
    pub up: P_OBJECT,
    pub uA: A_OBJECT,
}
#[no_mangle]
pub static mut A: A_OBJECT = 0 as *const A_desc as *mut A_desc;
#[no_mangle]
pub static mut Atemp: A_OBJECT = 0 as *const A_desc as *mut A_desc;
#[no_mangle]
pub static mut TAlist: Tn_OBJECT = 0 as *const Tn_desc as *mut Tn_desc;
#[no_mangle]
pub static mut Alist: *mut A_OBJECT = 0 as *const A_OBJECT as *mut A_OBJECT;
#[no_mangle]
pub static mut TT2: T2_OBJECT = 0 as *const T2_desc as *mut T2_desc;
#[no_mangle]
pub unsafe extern "C" fn TT2_init() {
    let mut result: libc::c_int = 0;
    let mut ti: libc::c_int = 0;
    let mut tstr: [libc::c_char; 3] = [0; 3];
    TT2 = T2_create();
    result = T2_insert(
        TT2,
        b"^^\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        2 as libc::c_int,
    );
    if result == 0 as libc::c_int {} else {
        __assert_fail(
            b"result == 0\0" as *const u8 as *const libc::c_char,
            b"Lex.c\0" as *const u8 as *const libc::c_char,
            43 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 16],
                &[libc::c_char; 16],
            >(b"void TT2_init()\0"))
                .as_ptr(),
        );
    };
    result = T2_insert(
        TT2,
        b"-|\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        2 as libc::c_int,
    );
    if result == 1 as libc::c_int {} else {
        __assert_fail(
            b"result == 1\0" as *const u8 as *const libc::c_char,
            b"Lex.c\0" as *const u8 as *const libc::c_char,
            45 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 16],
                &[libc::c_char; 16],
            >(b"void TT2_init()\0"))
                .as_ptr(),
        );
    };
    ti = 0 as libc::c_int;
    while ti < 256 as libc::c_int {
        tstr[0 as libc::c_int as usize] = ti as libc::c_char;
        tstr[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        result = T2_insert(TT2, tstr.as_mut_ptr(), 1 as libc::c_int);
        if result == ti + 2 as libc::c_int {} else {
            __assert_fail(
                b"result == ti + 2\0" as *const u8 as *const libc::c_char,
                b"Lex.c\0" as *const u8 as *const libc::c_char,
                50 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"void TT2_init()\0"))
                    .as_ptr(),
            );
        };
        ti += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn pad20(mut s: *mut libc::c_char) -> *mut libc::c_char {
    static mut tmp: [libc::c_char; 41] = [0; 41];
    if strlen(s) >= 20 as libc::c_int as libc::c_ulong {
        return s
    } else {
        strcpy(tmp.as_mut_ptr(), s);
        strcat(
            tmp.as_mut_ptr(),
            b" -------------------\0" as *const u8 as *const libc::c_char,
        );
        tmp[20 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        return tmp.as_mut_ptr();
    };
}
#[no_mangle]
pub static mut ch: libc::c_int = ' ' as i32;
#[no_mangle]
pub static mut token: [libc::c_char; 512] = [0; 512];
#[no_mangle]
pub static mut in_string: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn copyof(mut str: *mut libc::c_char) -> *mut libc::c_char {
    return strcpy(
        Salloc(
            (strlen(str)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_long,
        ),
        str,
    );
}
#[no_mangle]
pub unsafe extern "C" fn yylex() -> libc::c_int {
    let mut li: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut lflag: libc::c_int = 0;
    let mut in_comment: libc::c_int = 0;
    fflush(fpout);
    if in_string != 0 {
        ch = getc(fpin);
        if ch == '\'' as i32 {
            ch = getc(fpin);
            if ch != '\'' as i32 {
                in_string = 0 as libc::c_int;
                return 263 as libc::c_int;
            }
        }
        if ch == '\\' as i32 {
            ch = getc(fpin);
            match ch {
                110 => {
                    ch = '\n' as i32;
                }
                116 => {
                    ch = '\t' as i32;
                }
                95 => {
                    ch = ' ' as i32;
                }
                120 => {
                    d = getc(fpin);
                    if d >= '0' as i32 && d <= '9' as i32 {
                        d = d - '0' as i32;
                    } else if d >= 'a' as i32 && d <= 'f' as i32 {
                        d = d - 'a' as i32 + 10 as libc::c_int;
                    } else if d >= 'A' as i32 && d <= 'F' as i32 {
                        d = d - 'A' as i32 + 10 as libc::c_int;
                    } else {
                        Error(
                            b"Unexpected Hex digit\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        );
                    }
                    ch = d << 4 as libc::c_int;
                    d = getc(fpin);
                    if d >= '0' as i32 && d <= '9' as i32 {
                        d = d - '0' as i32;
                    } else if d >= 'a' as i32 && d <= 'f' as i32 {
                        d = d - 'a' as i32 + 10 as libc::c_int;
                    } else if d >= 'A' as i32 && d <= 'F' as i32 {
                        d = d - 'A' as i32 + 10 as libc::c_int;
                    } else {
                        Error(
                            b"Unexpected Hex digit\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        );
                    }
                    ch += d;
                }
                _ => {}
            }
        }
        if ch == -(1 as libc::c_int) {
            Error(
                b"End of file in string\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        yylval
            .up = P_create(
            T2_length(TT2, ch + 2 as libc::c_int),
            T2_name(TT2, ch + 2 as libc::c_int),
        );
        return 282 as libc::c_int;
    }
    in_comment = 0 as libc::c_int;
    while ch == ' ' as i32 || ch == '\t' as i32 || ch == '\n' as i32 || ch == '#' as i32
        || in_comment != 0
    {
        if in_comment != 0 && isatty(fileno(fpout)) == 0 {
            putc(ch, fpout);
        }
        if ch == '#' as i32 {
            in_comment = 1 as libc::c_int;
        }
        if ch == '\n' as i32 {
            in_comment = 0 as libc::c_int;
        }
        if ch == -(1 as libc::c_int) {
            Error(
                b"End of file in comment\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        ch = getc(fpin);
    }
    if ch == -(1 as libc::c_int) {
        return 0 as libc::c_int;
    }
    d = ch;
    ch = ' ' as i32;
    match d {
        33 => return 258 as libc::c_int,
        36 => return 259 as libc::c_int,
        37 => return 260 as libc::c_int,
        38 => return 261 as libc::c_int,
        39 => {
            in_string = 1 as libc::c_int;
            return 262 as libc::c_int;
        }
        40 => return 262 as libc::c_int,
        41 => return 263 as libc::c_int,
        42 => return 264 as libc::c_int,
        43 => return 265 as libc::c_int,
        44 => return 266 as libc::c_int,
        45 => return 267 as libc::c_int,
        47 => return 268 as libc::c_int,
        58 => return 269 as libc::c_int,
        59 => return 270 as libc::c_int,
        61 => return 271 as libc::c_int,
        63 => return 272 as libc::c_int,
        64 => return 273 as libc::c_int,
        91 => return 274 as libc::c_int,
        92 => return 275 as libc::c_int,
        93 => return 276 as libc::c_int,
        94 => return 277 as libc::c_int,
        123 => return 278 as libc::c_int,
        124 => return 279 as libc::c_int,
        125 => return 280 as libc::c_int,
        34 | 60 | 62 | 126 => {
            fprintf(
                fpout,
                b"Reserved character: %c\n\0" as *const u8 as *const libc::c_char,
                d,
            );
            return d;
        }
        _ => {}
    }
    li = 0 as libc::c_int;
    ch = d;
    lflag = 1 as libc::c_int;
    if ch == '`' as i32 {
        ch = getc(fpin);
        while ch != -(1 as libc::c_int) {
            if ch == '`' as i32 {
                ch = getc(fpin);
                if ch != '`' as i32 {
                    break;
                }
            }
            if ch == '\\' as i32 {
                ch = getc(fpin);
                match ch {
                    110 => {
                        ch = '\n' as i32;
                    }
                    116 => {
                        ch = '\t' as i32;
                    }
                    95 => {
                        ch = ' ' as i32;
                    }
                    120 => {
                        d = getc(fpin);
                        if d >= '0' as i32 && d <= '9' as i32 {
                            d = d - '0' as i32;
                        } else if d >= 'a' as i32 && d <= 'f' as i32 {
                            d = d - 'a' as i32 + 10 as libc::c_int;
                        } else if d >= 'A' as i32 && d <= 'F' as i32 {
                            d = d - 'A' as i32 + 10 as libc::c_int;
                        } else {
                            Error(
                                b"Unexpected Hex digit\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char,
                            );
                        }
                        ch = d << 4 as libc::c_int;
                        d = getc(fpin);
                        if d >= '0' as i32 && d <= '9' as i32 {
                            d = d - '0' as i32;
                        } else if d >= 'a' as i32 && d <= 'f' as i32 {
                            d = d - 'a' as i32 + 10 as libc::c_int;
                        } else if d >= 'A' as i32 && d <= 'F' as i32 {
                            d = d - 'A' as i32 + 10 as libc::c_int;
                        } else {
                            Error(
                                b"Unexpected Hex digit\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char,
                            );
                        }
                        ch += d;
                    }
                    _ => {}
                }
            }
            let fresh0 = li;
            li = li + 1;
            token[fresh0 as usize] = ch as libc::c_char;
            ch = getc(fpin);
        }
        if li == 0 as libc::c_int {
            return 277 as libc::c_int;
        }
        token[li as usize] = 0 as libc::c_int as libc::c_char;
        yylval.up = P_create(li, token.as_mut_ptr());
        return 281 as libc::c_int;
    } else {
        while lflag != 0 && ch != -(1 as libc::c_int) {
            let fresh1 = li;
            li = li + 1;
            token[fresh1 as usize] = ch as libc::c_char;
            ch = getc(fpin);
            if li != 2 as libc::c_int
                || token[1 as libc::c_int as usize] as libc::c_int != '.' as i32
                || *(*__ctype_b_loc())
                    .offset(token[0 as libc::c_int as usize] as libc::c_int as isize)
                    as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
            {
                lflag = 0 as libc::c_int;
                match ch {
                    46 | 95 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 65 | 66
                    | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80
                    | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 | 97 | 98 | 99
                    | 100 | 101 | 102 | 103 | 104 | 105 | 106 | 107 | 108 | 109 | 110
                    | 111 | 112 | 113 | 114 | 115 | 116 | 117 | 118 | 119 | 120 | 121
                    | 122 => {
                        lflag = 1 as libc::c_int;
                    }
                    _ => {}
                }
            }
        }
    }
    token[li as usize] = 0 as libc::c_int as libc::c_char;
    yylval.up = P_create(li, token.as_mut_ptr());
    return 282 as libc::c_int;
}
#[no_mangle]
pub static mut Notice: [libc::c_char; 67] = unsafe {
    *::std::mem::transmute::<
        &[u8; 67],
        &mut [libc::c_char; 67],
    >(b"Copyright (c) 1985, 1988, J Howard Johnson, University of Waterloo\0")
};
#[no_mangle]
pub unsafe extern "C" fn smain(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    let mut file_in: [libc::c_char; 50] = [0; 50];
    let mut file_out: [libc::c_char; 50] = [0; 50];
    let mut rpt_out: [libc::c_char; 50] = [0; 50];
    fpin = stdin;
    fpout = stdout;
    if argc > 3 as libc::c_int {
        printf(
            b"Usage: inr [ input_file ] [ output_file ]\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    if argc > 2 as libc::c_int {
        strcpy(file_out.as_mut_ptr(), *argv.offset(2 as libc::c_int as isize));
        fpout = fopen(file_out.as_mut_ptr(), b"w\0" as *const u8 as *const libc::c_char);
    }
    if argc > 1 as libc::c_int {
        strcpy(file_in.as_mut_ptr(), *argv.offset(1 as libc::c_int as isize));
        fpin = fopen(file_in.as_mut_ptr(), b"r\0" as *const u8 as *const libc::c_char);
    }
    if fpin.is_null() || fpout.is_null() {
        if !fpin.is_null() {
            strcpy(rpt_out.as_mut_ptr(), file_out.as_mut_ptr());
        } else if !fpout.is_null() {
            strcpy(rpt_out.as_mut_ptr(), file_in.as_mut_ptr());
        } else {
            strcpy(rpt_out.as_mut_ptr(), file_in.as_mut_ptr());
            strcat(rpt_out.as_mut_ptr(), b", \0" as *const u8 as *const libc::c_char);
            strcat(rpt_out.as_mut_ptr(), file_out.as_mut_ptr());
        }
        printf(
            b"Problem with %s file(s) opens. -- aborting\n\0" as *const u8
                as *const libc::c_char,
            rpt_out.as_mut_ptr(),
        );
        exit(1 as libc::c_int);
    }
    if isatty(fileno(fpout)) != 0 {
        fprintf(fpout, b"\n\0" as *const u8 as *const libc::c_char);
        fprintf(
            fpout,
            b"II  N     N  RRRRRR    I N R     \0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            fpout,
            b"Version %s (Mar 25, 1988)\n\0" as *const u8 as *const libc::c_char,
            Version.as_mut_ptr(),
        );
        fprintf(fpout, b"II  N N   N  R    RR\0" as *const u8 as *const libc::c_char);
        fprintf(
            fpout,
            b"             Copyright (C) 1988 J Howard Johnson\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            fpout,
            b"II  N  N  N  RRRRRR    modified  %s\n\0" as *const u8
                as *const libc::c_char,
            Date.as_mut_ptr(),
        );
        fprintf(fpout, b"II  N   N N  R    R\n\0" as *const u8 as *const libc::c_char);
        fprintf(fpout, b"II  N    NN  R     R\0" as *const u8 as *const libc::c_char);
        fprintf(
            fpout,
            b"                              (For help type   `:help;')\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            fpout,
            b"This program comes with ABSOLUTELY NO WARRANTY; \0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            fpout,
            b"for details type `:help w;'.\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            fpout,
            b"This is free software, and you are welcome \0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            fpout,
            b"to redistribute it under certain\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            fpout,
            b"conditions; type `:help c;' for details.\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(fpout, b"\n\0" as *const u8 as *const libc::c_char);
    } else {
        fprintf(
            fpout,
            b"I N R -- V %s, modified %s\n\0" as *const u8 as *const libc::c_char,
            Version.as_mut_ptr(),
            Date.as_mut_ptr(),
        );
        fprintf(
            fpout,
            b"Copyright (C) 1988 J Howard Johnson\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            fpout,
            b"Distributed under GPLv3 (see COPYING)\n\0" as *const u8
                as *const libc::c_char,
        );
        if fpin != stdin {
            fprintf(
                fpout,
                b"  (Source file: %s)\0" as *const u8 as *const libc::c_char,
                file_in.as_mut_ptr(),
            );
        }
        fprintf(fpout, b"\n\n\n\0" as *const u8 as *const libc::c_char);
    }
    TT2_init();
    Alist = Salloc(
        (100 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<A_OBJECT>() as libc::c_ulong)
            as libc::c_long,
    ) as *mut A_OBJECT;
    TAlist = Tn_create();
    result = Tn_insert(
        TAlist,
        b"_Last_\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        6 as libc::c_int,
    );
    if result == 0 as libc::c_int {} else {
        __assert_fail(
            b"result == 0\0" as *const u8 as *const libc::c_char,
            b"Lex.c\0" as *const u8 as *const libc::c_char,
            408 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 24],
                &[libc::c_char; 24],
            >(b"int smain(int, char **)\0"))
                .as_ptr(),
        );
    };
    let ref mut fresh2 = *Alist.offset(0 as libc::c_int as isize);
    *fresh2 = A_create();
    pr_time_diff();
    if isatty(fileno(fpin)) != 0 && isatty(fileno(fpout)) != 0 {
        printf(b"--* \0" as *const u8 as *const libc::c_char);
    }
    if yyparse() != 0 as libc::c_int {
        Error(
            b"yyparse returned unexpectedly\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    T2_destroy(TT2);
    Tn_destroy(TAlist);
    if A_report != 0 {
        fprintf(fpout, b"\n\0" as *const u8 as *const libc::c_char);
        Tn_stats();
        V_stats();
        R_stats();
        U_stats();
    }
    exit(0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn yyerror(mut str: *mut libc::c_char) {
    fprintf(fpout, b"*** %s ***\n\0" as *const u8 as *const libc::c_char, str);
}
#[no_mangle]
pub unsafe extern "C" fn tonum(mut p: *mut libc::c_char) -> libc::c_int {
    let mut acum: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    acum = 0 as libc::c_int;
    loop {
        let fresh3 = p;
        p = p.offset(1);
        c = *fresh3 as libc::c_int;
        if !(c != 0) {
            break;
        }
        if c < '0' as i32 || c > '9' as i32 {
            return -(1 as libc::c_int);
        }
        acum = acum * 10 as libc::c_int + c - '0' as i32;
    }
    return acum;
}
