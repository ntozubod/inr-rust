use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut fpout: *mut FILE;
    fn Sarena();
    fn Saudit();
    fn Warning(_: *mut libc::c_char);
    fn Error(_: *mut libc::c_char);
    fn pr_time_diff();
    fn Tn_member(_: Tn_OBJECT, _: *mut libc::c_char, _: libc::c_int) -> libc::c_int;
    fn Tn_name(_: Tn_OBJECT, _: libc::c_int) -> *mut libc::c_char;
    fn T2_insert(_: T2_OBJECT, _: *mut libc::c_char, _: libc::c_int) -> libc::c_int;
    fn T2_name(_: T2_OBJECT, _: libc::c_int) -> *mut libc::c_char;
    static mut A_report: libc::c_int;
    fn A_rept(_: A_OBJECT) -> A_OBJECT;
    fn A_copy(_: A_OBJECT) -> A_OBJECT;
    fn A_add(_: A_OBJECT, _: libc::c_int, _: libc::c_int, _: libc::c_int) -> A_OBJECT;
    fn A_open(_: A_OBJECT) -> A_OBJECT;
    fn A_close(_: A_OBJECT) -> A_OBJECT;
    fn A_load(_: *mut libc::c_char, _: T2_OBJECT) -> A_OBJECT;
    fn A_lwds(_: *mut libc::c_char, _: T2_OBJECT) -> A_OBJECT;
    fn A_prsseq(_: A_OBJECT, _: *mut libc::c_char, _: T2_OBJECT) -> A_OBJECT;
    fn A_pr(_: A_OBJECT, _: *mut libc::c_char, _: T2_OBJECT) -> A_OBJECT;
    fn A_save(_: A_OBJECT, _: *mut libc::c_char, _: T2_OBJECT) -> A_OBJECT;
    fn A_trim(_: A_OBJECT) -> A_OBJECT;
    fn A_lameq(_: A_OBJECT) -> A_OBJECT;
    fn A_lamcm(_: A_OBJECT) -> A_OBJECT;
    fn A_clsure(_: A_OBJECT) -> A_OBJECT;
    fn A_subs(_: A_OBJECT) -> A_OBJECT;
    fn A_min(_: A_OBJECT) -> A_OBJECT;
    fn A_letter(_: libc::c_int, _: libc::c_int) -> A_OBJECT;
    fn A_deecho(_: A_OBJECT, _: libc::c_int, _: libc::c_int) -> A_OBJECT;
    fn A_star(_: A_OBJECT) -> A_OBJECT;
    fn A_differ(_: A_OBJECT, _: A_OBJECT) -> A_OBJECT;
    fn A_alph(_: A_OBJECT) -> A_OBJECT;
    fn A_rev(_: A_OBJECT) -> A_OBJECT;
    fn A_enum(_: A_OBJECT, _: T2_OBJECT, _: libc::c_int) -> A_OBJECT;
    fn A_card(_: A_OBJECT) -> libc::c_int;
    fn A_pref(_: A_OBJECT) -> A_OBJECT;
    fn A_suff(_: A_OBJECT) -> A_OBJECT;
    fn A_join(_: A_OBJECT, _: A_OBJECT) -> A_OBJECT;
    fn A_retape(_: A_OBJECT, _: A_OBJECT, _: T2_OBJECT) -> A_OBJECT;
    fn A_catpow(_: A_OBJECT, _: libc::c_int) -> A_OBJECT;
    fn A_lenmin(_: A_OBJECT) -> A_OBJECT;
    fn A_minlen(_: A_OBJECT) -> libc::c_int;
    fn A_prstems(_: A_OBJECT, _: T2_OBJECT, _: libc::c_int);
    fn A_sseq(_: A_OBJECT) -> A_OBJECT;
    fn A_LMsseq(_: A_OBJECT) -> A_OBJECT;
    fn A_GMsseq(_: A_OBJECT) -> A_OBJECT;
    fn A_clsseq(_: A_OBJECT) -> A_OBJECT;
    static mut TT2: T2_OBJECT;
    static mut TAlist: Tn_OBJECT;
    static mut Alist: *mut A_OBJECT;
    fn pad20(_: *mut libc::c_char) -> *mut libc::c_char;
    fn tonum(_: *mut libc::c_char) -> libc::c_int;
    fn A_gen_min(_: A_OBJECT) -> A_OBJECT;
    static mut Atemp: A_OBJECT;
    fn A_spit_utf8(_: A_OBJECT, _: *mut libc::c_char, _: T2_OBJECT) -> A_OBJECT;
    fn A_spit_octets(_: A_OBJECT, _: *mut libc::c_char, _: T2_OBJECT) -> A_OBJECT;
    fn A_slurp_utf8(_: *mut libc::c_char, _: T2_OBJECT) -> A_OBJECT;
    fn A_slurp_octets(_: *mut libc::c_char, _: T2_OBJECT) -> A_OBJECT;
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
#[no_mangle]
pub static mut disp_flag: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn do_n_i(mut op: *mut libc::c_char) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if strcmp(b"alph\0" as *const u8 as *const libc::c_char, op) == 0 {
        fprintf(fpout, b"\n\0" as *const u8 as *const libc::c_char);
        i = 2 as libc::c_int;
        while i < (*(*TT2).T2_int).Tn_n {
            fprintf(fpout, b"%s\0" as *const u8 as *const libc::c_char, T2_name(TT2, i));
            fprintf(fpout, b" \0" as *const u8 as *const libc::c_char);
            i += 1;
        }
        fprintf(fpout, b"\n\0" as *const u8 as *const libc::c_char);
    } else if strcmp(b"free\0" as *const u8 as *const libc::c_char, op) == 0 {
        fprintf(fpout, b"\n\0" as *const u8 as *const libc::c_char);
        Sarena();
        Saudit();
        fprintf(fpout, b"\n\0" as *const u8 as *const libc::c_char);
    } else if strcmp(b"list\0" as *const u8 as *const libc::c_char, op) == 0 {
        fprintf(fpout, b"\n\0" as *const u8 as *const libc::c_char);
        i = 0 as libc::c_int;
        while i < (*TAlist).Tn_n {
            fprintf(
                fpout,
                b"%s  \0" as *const u8 as *const libc::c_char,
                pad20(Tn_name(TAlist, i)),
            );
            A_rept(*Alist.offset(i as isize));
            i += 1;
        }
        fprintf(fpout, b"\n\0" as *const u8 as *const libc::c_char);
    } else if strcmp(b"noreport\0" as *const u8 as *const libc::c_char, op) == 0 {
        A_report = 0 as libc::c_int;
    } else if strcmp(b"pr\0" as *const u8 as *const libc::c_char, op) == 0 {
        i = 1 as libc::c_int;
        while i < (*TAlist).Tn_n {
            if (**Alist.offset(i as isize)).A_nrows > 0 as libc::c_int {
                A_pr(*Alist.offset(i as isize), Tn_name(TAlist, i), TT2);
            }
            i += 1;
        }
    } else if strcmp(b"report\0" as *const u8 as *const libc::c_char, op) == 0 {
        A_report = 1 as libc::c_int;
    } else if strcmp(b"save\0" as *const u8 as *const libc::c_char, op) == 0 {
        i = 1 as libc::c_int;
        while i < (*TAlist).Tn_n {
            if (**Alist.offset(i as isize)).A_nrows > 0 as libc::c_int {
                let ref mut fresh0 = *Alist.offset(i as isize);
                *fresh0 = A_save(*Alist.offset(i as isize), Tn_name(TAlist, i), TT2);
            }
            i += 1;
        }
    } else if strcmp(b"time\0" as *const u8 as *const libc::c_char, op) == 0 {
        pr_time_diff();
    } else if strcmp(b"help\0" as *const u8 as *const libc::c_char, op) == 0 {
        fprintf(
            fpout,
            b"\nBasic Help\n\nTo terminate session type       :bye;\n\nTo get additional help type\n\n     :help ops;          (Priority and meaning of basic operators)\n     :help colonops;     (Colon operators)\n     :help ioops;        (IO operators)\n     :help stats;        (Form of statements)\n\n\0"
                as *const u8 as *const libc::c_char,
        );
    } else if strcmp(b"done\0" as *const u8 as *const libc::c_char, op) == 0
        || strcmp(b"quit\0" as *const u8 as *const libc::c_char, op) == 0
        || strcmp(b"stop\0" as *const u8 as *const libc::c_char, op) == 0
        || strcmp(b"bye\0" as *const u8 as *const libc::c_char, op) == 0
    {
        return 1 as libc::c_int
    } else {
        Warning(
            b"Unknown function\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn do_an_a(
    mut A: A_OBJECT,
    mut op: *mut libc::c_char,
) -> A_OBJECT {
    let mut i: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    disp_flag = 0 as libc::c_int;
    if strcmp(b"pr\0" as *const u8 as *const libc::c_char, op) == 0 {
        A_rept(A);
        fprintf(fpout, b"\n\0" as *const u8 as *const libc::c_char);
        A = A_pr(A, 0 as *mut libc::c_void as *mut libc::c_char, TT2);
        fprintf(fpout, b"\n\0" as *const u8 as *const libc::c_char);
    } else if strcmp(b"prsseq\0" as *const u8 as *const libc::c_char, op) == 0 {
        fprintf(fpout, b"\n\0" as *const u8 as *const libc::c_char);
        A = A_prsseq(A, 0 as *mut libc::c_void as *mut libc::c_char, TT2);
        fprintf(fpout, b"\n\0" as *const u8 as *const libc::c_char);
    } else if strcmp(b"spit_octets\0" as *const u8 as *const libc::c_char, op) == 0 {
        fprintf(fpout, b"\n\0" as *const u8 as *const libc::c_char);
        A = A_spit_octets(A, 0 as *mut libc::c_void as *mut libc::c_char, TT2);
        fprintf(fpout, b"\n\0" as *const u8 as *const libc::c_char);
    } else if strcmp(b"spit_utf8\0" as *const u8 as *const libc::c_char, op) == 0 {
        fprintf(fpout, b"\n\0" as *const u8 as *const libc::c_char);
        A = A_spit_utf8(A, 0 as *mut libc::c_void as *mut libc::c_char, TT2);
        fprintf(fpout, b"\n\0" as *const u8 as *const libc::c_char);
    } else if strcmp(b"acomp\0" as *const u8 as *const libc::c_char, op) == 0 {
        if A_report != 0 {
            fprintf(fpout, b"(acomp)\n\0" as *const u8 as *const libc::c_char);
        }
        Atemp = A_star(A_alph(A_copy(A)));
        A = A_differ(Atemp, A);
        disp_flag = 2 as libc::c_int;
    } else if strcmp(b"alph\0" as *const u8 as *const libc::c_char, op) == 0 {
        if A_report != 0 {
            fprintf(fpout, b"(alph)\n\0" as *const u8 as *const libc::c_char);
        }
        A = A_alph(A);
        disp_flag = 2 as libc::c_int;
    } else if strcmp(b"card\0" as *const u8 as *const libc::c_char, op) == 0 {
        i = A_card(A);
        if i < 0 as libc::c_int {
            fprintf(fpout, b"Infinite\n\0" as *const u8 as *const libc::c_char);
        } else {
            fprintf(fpout, b"%d word\0" as *const u8 as *const libc::c_char, i);
            if i != 1 as libc::c_int {
                fprintf(fpout, b"s\0" as *const u8 as *const libc::c_char);
            }
            fprintf(fpout, b"\n\0" as *const u8 as *const libc::c_char);
        }
    } else if strcmp(b"closed\0" as *const u8 as *const libc::c_char, op) == 0 {
        A = A_clsure(A);
        disp_flag = 1 as libc::c_int;
    } else if strcmp(b"clsseq\0" as *const u8 as *const libc::c_char, op) == 0 {
        A = A_clsseq(A);
        disp_flag = 2 as libc::c_int;
    } else if strcmp(b"comp\0" as *const u8 as *const libc::c_char, op) == 0 {
        if A_report != 0 {
            fprintf(fpout, b"(comp)\n\0" as *const u8 as *const libc::c_char);
        }
        i = Tn_member(
            TAlist,
            b"_Sigma_\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            7 as libc::c_int,
        );
        if i >= 0 as libc::c_int {
            Atemp = A_star(A_alph(A_copy(*Alist.offset(i as isize))));
            A = A_differ(Atemp, A);
        } else {
            fprintf(
                fpout,
                b"Error in comp: _Sigma_ not defined\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        disp_flag = 2 as libc::c_int;
    } else if strcmp(b"deecho\0" as *const u8 as *const libc::c_char, op) == 0 {
        A = A_deecho(
            A,
            T2_insert(
                TT2,
                b"_Echo_\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                6 as libc::c_int,
            ) * (*A).A_nT + 1 as libc::c_int,
            T2_insert(
                TT2,
                b"_Noecho_\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                8 as libc::c_int,
            ) * (*A).A_nT + 1 as libc::c_int,
        );
        disp_flag = 2 as libc::c_int;
    } else if strcmp(b"dfa\0" as *const u8 as *const libc::c_char, op) == 0 {
        A = A_subs(A);
        disp_flag = 1 as libc::c_int;
    } else if strcmp(b"dfamin\0" as *const u8 as *const libc::c_char, op) == 0 {
        A = A_min(A);
        disp_flag = 1 as libc::c_int;
    } else if strcmp(b"enum\0" as *const u8 as *const libc::c_char, op) == 0 {
        A = A_enum(A, TT2, 1000 as libc::c_int);
    } else if strcmp(b"limit\0" as *const u8 as *const libc::c_char, op) == 0 {
        if A_report != 0 {
            fprintf(fpout, b"(limit)\n\0" as *const u8 as *const libc::c_char);
        }
        if (*A).A_nT != 2 as libc::c_int {
            Error(
                b"Wrong number of tapes (limit)\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        Atemp = A_differ(
            A_retape(
                A_copy(A),
                A_letter(
                    0 as libc::c_int,
                    T2_insert(
                        TT2,
                        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        1 as libc::c_int,
                    ),
                ),
                TT2,
            ),
            A_retape(
                A_copy(A),
                A_letter(
                    0 as libc::c_int,
                    T2_insert(
                        TT2,
                        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        1 as libc::c_int,
                    ),
                ),
                TT2,
            ),
        );
        A = A_join(A, Atemp);
        disp_flag = 2 as libc::c_int;
    } else if strcmp(b"lamcm\0" as *const u8 as *const libc::c_char, op) == 0 {
        A = A_lamcm(A);
        disp_flag = 1 as libc::c_int;
    } else if strcmp(b"lameq\0" as *const u8 as *const libc::c_char, op) == 0 {
        A = A_lameq(A);
        disp_flag = 1 as libc::c_int;
    } else if strcmp(b"lenmin\0" as *const u8 as *const libc::c_char, op) == 0 {
        A = A_lenmin(A);
        disp_flag = 2 as libc::c_int;
    } else if strcmp(b"gen_min\0" as *const u8 as *const libc::c_char, op) == 0 {
        A = A_gen_min(A);
        disp_flag = 2 as libc::c_int;
    } else if strcmp(b"min\0" as *const u8 as *const libc::c_char, op) == 0 {
        A = A_min(A);
        disp_flag = 1 as libc::c_int;
    } else if strcmp(b"length\0" as *const u8 as *const libc::c_char, op) == 0 {
        i = A_minlen(A);
        if i < 0 as libc::c_int {
            fprintf(fpout, b"Empty Set\n\0" as *const u8 as *const libc::c_char);
        } else {
            fprintf(fpout, b"%d letter\0" as *const u8 as *const libc::c_char, i);
            if i != 1 as libc::c_int {
                fprintf(fpout, b"s\0" as *const u8 as *const libc::c_char);
            }
            fprintf(fpout, b"\n\0" as *const u8 as *const libc::c_char);
        }
    } else if strcmp(b"nfa\0" as *const u8 as *const libc::c_char, op) == 0 {
        A = A_close(A);
        disp_flag = 1 as libc::c_int;
    } else if strcmp(b"pref\0" as *const u8 as *const libc::c_char, op) == 0 {
        if A_report != 0 {
            fprintf(fpout, b"(pref)\n\0" as *const u8 as *const libc::c_char);
        }
        A = A_pref(A);
        disp_flag = 2 as libc::c_int;
    } else if strcmp(b"report\0" as *const u8 as *const libc::c_char, op) == 0 {
        A = A_rept(A);
    } else if strcmp(b"rev\0" as *const u8 as *const libc::c_char, op) == 0 {
        if A_report != 0 {
            fprintf(fpout, b"(rev)\n\0" as *const u8 as *const libc::c_char);
        }
        A = A_rev(A);
        disp_flag = 2 as libc::c_int;
    } else if strcmp(b"sseq\0" as *const u8 as *const libc::c_char, op) == 0 {
        A = A_sseq(A);
        disp_flag = 2 as libc::c_int;
    } else if strcmp(b"LMsseq\0" as *const u8 as *const libc::c_char, op) == 0 {
        A = A_LMsseq(A);
        disp_flag = 2 as libc::c_int;
    } else if strcmp(b"GMsseq\0" as *const u8 as *const libc::c_char, op) == 0 {
        A = A_GMsseq(A);
        disp_flag = 2 as libc::c_int;
    } else if strcmp(b"stems\0" as *const u8 as *const libc::c_char, op) == 0 {
        A_prstems(A, TT2, 0 as libc::c_int);
    } else if strcmp(b"suff\0" as *const u8 as *const libc::c_char, op) == 0 {
        if A_report != 0 {
            fprintf(fpout, b"(suff)\n\0" as *const u8 as *const libc::c_char);
        }
        A = A_suff(A);
        disp_flag = 2 as libc::c_int;
    } else if strcmp(b"time\0" as *const u8 as *const libc::c_char, op) == 0 {
        pr_time_diff();
        disp_flag = 2 as libc::c_int;
    } else if strcmp(b"trim\0" as *const u8 as *const libc::c_char, op) == 0 {
        A = A_trim(A);
        disp_flag = 1 as libc::c_int;
    } else if strcmp(b"update\0" as *const u8 as *const libc::c_char, op) == 0 {
        A = A_open(A);
        disp_flag = 1 as libc::c_int;
    } else {
        num = tonum(op);
        if num >= 0 as libc::c_int {
            if A_report != 0 {
                fprintf(fpout, b"(pow)\n\0" as *const u8 as *const libc::c_char);
            }
            A = A_catpow(A, num);
            disp_flag = 2 as libc::c_int;
        } else {
            Warning(
                b"Unknown function\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
    }
    return A;
}
#[no_mangle]
pub unsafe extern "C" fn do_ann_a(
    mut A: A_OBJECT,
    mut op: *mut libc::c_char,
    mut arg: *mut libc::c_char,
) -> A_OBJECT {
    let mut num: libc::c_int = 0;
    disp_flag = 0 as libc::c_int;
    if strcmp(b"enum\0" as *const u8 as *const libc::c_char, op) == 0
        && {
            num = tonum(arg);
            num >= 0 as libc::c_int
        }
    {
        A = A_enum(A, TT2, num);
    } else if strcmp(b"save\0" as *const u8 as *const libc::c_char, op) == 0 {
        A = A_save(A, arg, TT2);
    } else if strcmp(b"pr\0" as *const u8 as *const libc::c_char, op) == 0 {
        A = A_pr(A, arg, TT2);
    } else if strcmp(b"prsseq\0" as *const u8 as *const libc::c_char, op) == 0 {
        A = A_prsseq(A, arg, TT2);
    } else if strcmp(b"spit_octets\0" as *const u8 as *const libc::c_char, op) == 0 {
        A = A_spit_octets(A, arg, TT2);
    } else if strcmp(b"spit_utf8\0" as *const u8 as *const libc::c_char, op) == 0 {
        A = A_spit_utf8(A, arg, TT2);
    } else if strcmp(b"stems\0" as *const u8 as *const libc::c_char, op) == 0 {
        A_prstems(A, TT2, tonum(arg));
    } else if strcmp(b"surgery\0" as *const u8 as *const libc::c_char, op) == 0 {
        num = tonum(arg);
        if num < 2 as libc::c_int || num >= (*A).A_nrows {
            Warning(
                b"Illegal surgery\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        } else {
            A = A_open(A);
            A = A_add(
                A,
                num,
                T2_insert(
                    TT2,
                    b"_Incision_\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    10 as libc::c_int,
                ) * (*A).A_nT,
                num,
            );
        }
    } else {
        Warning(
            b"Unknown function\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    return A;
}
#[no_mangle]
pub unsafe extern "C" fn do_nn_a(
    mut op: *mut libc::c_char,
    mut arg: *mut libc::c_char,
) -> A_OBJECT {
    let mut i: libc::c_int = 0;
    let mut A: A_OBJECT = 0 as *mut A_desc;
    disp_flag = 0 as libc::c_int;
    A = 0 as A_OBJECT;
    if strcmp(b"get\0" as *const u8 as *const libc::c_char, op) == 0 {
        i = Tn_member(TAlist, arg, strlen(arg) as libc::c_int);
        if i >= 0 as libc::c_int {
            A = A_copy(*Alist.offset(i as isize));
        }
        disp_flag = 1 as libc::c_int;
    } else if strcmp(b"read\0" as *const u8 as *const libc::c_char, op) == 0
        || strcmp(b"load\0" as *const u8 as *const libc::c_char, op) == 0
    {
        A = A_load(arg, TT2);
        disp_flag = 1 as libc::c_int;
    } else if strcmp(b"readwords\0" as *const u8 as *const libc::c_char, op) == 0 {
        A = A_lwds(arg, TT2);
        disp_flag = 1 as libc::c_int;
    } else if strcmp(b"slurp_octets\0" as *const u8 as *const libc::c_char, op) == 0 {
        A = A_slurp_octets(arg, TT2);
        disp_flag = 1 as libc::c_int;
    } else if strcmp(b"slurp_utf8\0" as *const u8 as *const libc::c_char, op) == 0 {
        A = A_slurp_utf8(arg, TT2);
        disp_flag = 1 as libc::c_int;
    } else if strcmp(b"save\0" as *const u8 as *const libc::c_char, op) == 0 {
        i = Tn_member(TAlist, arg, strlen(arg) as libc::c_int);
        if i >= 0 as libc::c_int {
            A = A_save(A_copy(*Alist.offset(i as isize)), arg, TT2);
        } else {
            fprintf(
                fpout,
                b"Warning: %s undefined\n\0" as *const u8 as *const libc::c_char,
                arg,
            );
        }
    } else if strcmp(b"pr\0" as *const u8 as *const libc::c_char, op) == 0 {
        i = Tn_member(TAlist, arg, strlen(arg) as libc::c_int);
        if i >= 0 as libc::c_int {
            A = A_pr(*Alist.offset(i as isize), arg, TT2);
        } else {
            fprintf(
                fpout,
                b"Warning: %s undefined\n\0" as *const u8 as *const libc::c_char,
                arg,
            );
        }
    } else if strcmp(b"help\0" as *const u8 as *const libc::c_char, op) == 0 {
        if strcmp(b"ops\0" as *const u8 as *const libc::c_char, arg) == 0 {
            fprintf(
                fpout,
                b"\nOperations by priority (highest to lowest):\n\n+   *   ?               postfix operators for 1 or more, 0 or more, 0 or 1\n<concatenation>         no explicit operator\n\\   /                   left factor, right factor\n&   -                   intersection, set difference\n|   !   ||  !!          union, exclusive or, elseor, shuffle\n$                       project\n@   @@                  composition, join\n<all colon operators>   see :help colonops; for details\n,                       Cartesian product within (), union within {}\n\nAll operators associate from left to right.\nParentheses may be used to indicate a specific order of evaluation.\n{,,,} is a set constructor.\n(,,,) is a tuple construtor.\n[   ] is the tape-shifting operator\n'   ' is a string of single letter tokens.\n`   ` is a token containing arbitrary symbols.\n^     indicates the empty word (or an explicit concatenation operator).\n\n\0"
                    as *const u8 as *const libc::c_char,
            );
        } else if strcmp(b"colonops\0" as *const u8 as *const libc::c_char, arg) == 0 {
            fprintf(
                fpout,
                b"\nColon operations (postfix operators at lowest priority)\n\nTransformation Operators               Displaying Operators\n:acomp      Active complement          :card       Print cardinality\n:alph       Active alphabet            :enum       Enumerate language\n:clsseq     Subsequential closure      :length     Display min word length\n:comp       Complement w.r.t. _Sigma_* :pr         Display automaton\n:lenmin     Words of min length        :prsseq     Subsequential display\n:pref       Set of prefixes            :report     Display report line\n:rev        Reverse                    :stems #    Print tape # stems\n:sseq       Subsequential transducer\n:LMsseq     LM Subsequential transducer\n:GMsseq     GM Subsequential transducer\n:suff       Set of suffixes            Coercing operators\n:<number>   Concatenation power        :update :nfa :trim :lameq\n:(<number>) Composition power          :lamcm :closed :dfa :dfamin\n\n:enum may take an optional argument to specify the quantity of output.\n\n\0"
                    as *const u8 as *const libc::c_char,
            );
        } else if strcmp(b"ioops\0" as *const u8 as *const libc::c_char, arg) == 0 {
            fprintf(
                fpout,
                b"\nIO operations\n\n:pr <filename>        Postfix operator to display automaton into a file\n:save <filename>      Postfix operator to save automaton in compressed form\n:load <filename>      Operator without left argument to get value from a file\n:readwords <filename> Operator with no argument to load a word file\n\n:get <variable>       Operator with no arguments to get value from a variable\n\n<var> = :load;        Short for <var> = :load <var>;\n:save <var>;          Short for <var> :save <var>;\n\n\0"
                    as *const u8 as *const libc::c_char,
            );
        } else if strcmp(b"stats\0" as *const u8 as *const libc::c_char, arg) == 0 {
            fprintf(
                fpout,
                b"\nStatement forms\n\n:bye;           Terminate session\n\n:alph;          Display token symbol table\n:free;          Display storage management report\n:list;          Display report lines for variables\n:noreport;      Turn off debug reporting\n:pr;            Save each variable into a file with the variable as file name\n:report;        Turn on debug reporting\n:save;          Save in compressed form all variables into files\n\n<var> = <exp>;  Assign value of expression to variable\n<exp>;          Compute and display expression\n<exp>:;         Compute and enumerate words in expression\n\nThe action of <exp>; depends on the last operator evaluated:\nCoercing Operator:  display the value using :pr implicitly\nPrinting Operator:  do not perform additional display\nOther:              coerce to DFAMIN and display using :pr\n\n\0"
                    as *const u8 as *const libc::c_char,
            );
        } else if strcmp(b"w\0" as *const u8 as *const libc::c_char, arg) == 0 {
            fprintf(
                fpout,
                b"\n(from GPLv3: see COPYING or http://www.gnu.org/licenses/gpl-3.0.html)\n  15. Disclaimer of Warranty.\n\n  THERE IS NO WARRANTY FOR THE PROGRAM, TO THE EXTENT PERMITTED BY\nAPPLICABLE LAW.  EXCEPT WHEN OTHERWISE STATED IN WRITING THE COPYRIGHT\nHOLDERS AND/OR OTHER PARTIES PROVIDE THE PROGRAM \"AS IS\" WITHOUT WARRANTY\nOF ANY KIND, EITHER EXPRESSED OR IMPLIED, INCLUDING, BUT NOT LIMITED TO,\nTHE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR\nPURPOSE.  THE ENTIRE RISK AS TO THE QUALITY AND PERFORMANCE OF THE PROGRAM\nIS WITH YOU.  SHOULD THE PROGRAM PROVE DEFECTIVE, YOU ASSUME THE COST OF\nALL NECESSARY SERVICING, REPAIR OR CORRECTION.\n\n\0"
                    as *const u8 as *const libc::c_char,
            );
        } else if strcmp(b"c\0" as *const u8 as *const libc::c_char, arg) == 0 {
            fprintf(
                fpout,
                b"\n(from GPLv3: see COPYING or http://www.gnu.org/licenses/gpl-3.0.html)\n  2. Basic Permissions.\n\n  All rights granted under this License are granted for the term of\ncopyright on the Program, and are irrevocable provided the stated\nconditions are met.  This License explicitly affirms your unlimited\npermission to run the unmodified Program.  The output from running a\ncovered work is covered by this License only if the output, given its\ncontent, constitutes a covered work.  This License acknowledges your\nrights of fair use or other equivalent, as provided by copyright law.\n\n  You may make, run and propagate covered works that you do not\nconvey, without conditions so long as your license otherwise remains\nin force.  You may convey covered works to others for the sole purpose\nof having them make modifications exclusively for you, or provide you\nwith facilities for running those works, provided that you comply with\nthe terms of this License in conveying all material for which you do\nnot control copyright.  Those thus making or running the covered works\nfor you must do so exclusively on your behalf, under your direction\nand control, on terms that prohibit them from making any copies of\nyour copyrighted material outside their relationship with you.\n\n  Conveying under any other circumstances is permitted solely under\nthe conditions stated below.  Sublicensing is not allowed; section 10\nmakes it unnecessary.\n\n\0"
                    as *const u8 as *const libc::c_char,
            );
        } else {
            fprintf(
                fpout,
                b"Unknown help request\0" as *const u8 as *const libc::c_char,
            );
        }
    } else {
        Warning(
            b"Unknown function\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    return A;
}
