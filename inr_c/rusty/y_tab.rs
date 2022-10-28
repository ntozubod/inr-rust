use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    static mut fpin: *mut FILE;
    static mut fpout: *mut FILE;
    fn Srealloc(_: *mut libc::c_char, _: libc::c_long) -> *mut libc::c_char;
    fn Ssize(_: *mut libc::c_char) -> libc::c_long;
    fn Warning(_: *mut libc::c_char);
    fn pr_time_diff();
    fn Tn_member(_: Tn_OBJECT, _: *mut libc::c_char, _: libc::c_int) -> libc::c_int;
    fn Tn_insert(_: Tn_OBJECT, _: *mut libc::c_char, _: libc::c_int) -> libc::c_int;
    fn T2_member(_: T2_OBJECT, _: *mut libc::c_char, _: libc::c_int) -> libc::c_int;
    fn T2_insert(_: T2_OBJECT, _: *mut libc::c_char, _: libc::c_int) -> libc::c_int;
    fn P_destroy(_: P_OBJECT);
    fn P_length(_: P_OBJECT) -> libc::c_int;
    fn P_cstr(_: P_OBJECT) -> *mut libc::c_char;
    fn Q_fromP(_: P_OBJECT) -> Q_OBJECT;
    fn Q_destroy(_: Q_OBJECT);
    fn Q_tapeno(_: Q_OBJECT) -> libc::c_int;
    fn Q_length(_: Q_OBJECT) -> libc::c_int;
    fn Q_cstr(_: Q_OBJECT) -> *mut libc::c_char;
    static mut A_report: libc::c_int;
    fn A_create() -> A_OBJECT;
    fn A_destroy(_: A_OBJECT);
    fn A_rept(_: A_OBJECT) -> A_OBJECT;
    fn A_copy(_: A_OBJECT) -> A_OBJECT;
    fn A_load(_: *mut libc::c_char, _: T2_OBJECT) -> A_OBJECT;
    fn A_pr(_: A_OBJECT, _: *mut libc::c_char, _: T2_OBJECT) -> A_OBJECT;
    fn A_min(_: A_OBJECT) -> A_OBJECT;
    fn A_phi() -> A_OBJECT;
    fn A_lambda() -> A_OBJECT;
    fn A_letter(_: libc::c_int, _: libc::c_int) -> A_OBJECT;
    fn A_opt(_: A_OBJECT) -> A_OBJECT;
    fn A_plus(_: A_OBJECT) -> A_OBJECT;
    fn A_star(_: A_OBJECT) -> A_OBJECT;
    fn A_union(_: A_OBJECT, _: A_OBJECT) -> A_OBJECT;
    fn A_percent(_: A_OBJECT, _: A_OBJECT) -> A_OBJECT;
    fn A_concat(_: A_OBJECT, _: A_OBJECT) -> A_OBJECT;
    fn A_intersect(_: A_OBJECT, _: A_OBJECT) -> A_OBJECT;
    fn A_differ(_: A_OBJECT, _: A_OBJECT) -> A_OBJECT;
    fn A_xor(_: A_OBJECT, _: A_OBJECT) -> A_OBJECT;
    fn A_alph(_: A_OBJECT) -> A_OBJECT;
    fn A_shuffle(_: A_OBJECT, _: A_OBJECT) -> A_OBJECT;
    fn A_enum(_: A_OBJECT, _: T2_OBJECT, _: libc::c_int) -> A_OBJECT;
    fn A_compose(_: A_OBJECT, _: A_OBJECT) -> A_OBJECT;
    fn A_join(_: A_OBJECT, _: A_OBJECT) -> A_OBJECT;
    fn A_retape(_: A_OBJECT, _: A_OBJECT, _: T2_OBJECT) -> A_OBJECT;
    fn A_comma(_: A_OBJECT, _: A_OBJECT) -> A_OBJECT;
    fn A_cmpow(_: A_OBJECT, _: libc::c_int) -> A_OBJECT;
    static mut disp_flag: libc::c_int;
    fn do_n_i(_: *mut libc::c_char) -> libc::c_int;
    fn do_an_a(_: A_OBJECT, _: *mut libc::c_char) -> A_OBJECT;
    fn do_ann_a(_: A_OBJECT, _: *mut libc::c_char, _: *mut libc::c_char) -> A_OBJECT;
    fn do_nn_a(_: *mut libc::c_char, _: *mut libc::c_char) -> A_OBJECT;
    static mut A: A_OBJECT;
    static mut Atemp: A_OBJECT;
    static mut Alist: *mut A_OBJECT;
    static mut TAlist: Tn_OBJECT;
    static mut TT2: T2_OBJECT;
    fn pad20(_: *mut libc::c_char) -> *mut libc::c_char;
    fn yylex() -> libc::c_int;
    fn yyerror(_: *mut libc::c_char);
    fn tonum(_: *mut libc::c_char) -> libc::c_int;
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
pub struct Q_desc {
    pub Type: libc::c_int,
    pub Q_tapeno: libc::c_int,
    pub Q_length: libc::c_int,
    pub Q_cstr: *mut libc::c_char,
}
pub type Q_OBJECT = *mut Q_desc;
pub type yy_state_t = yytype_int8;
pub type yytype_int8 = libc::c_schar;
#[derive(Copy, Clone)]
#[repr(C)]
pub union YYSTYPE {
    pub up: P_OBJECT,
    pub uA: A_OBJECT,
}
pub type yysymbol_kind_t = libc::c_int;
pub const YYSYMBOL_name: yysymbol_kind_t = 42;
pub const YYSYMBOL_brace_reg: yysymbol_kind_t = 41;
pub const YYSYMBOL_paren_reg: yysymbol_kind_t = 40;
pub const YYSYMBOL_reg_8: yysymbol_kind_t = 39;
pub const YYSYMBOL_reg_7: yysymbol_kind_t = 38;
pub const YYSYMBOL_reg_6: yysymbol_kind_t = 37;
pub const YYSYMBOL_reg_5: yysymbol_kind_t = 36;
pub const YYSYMBOL_reg_4: yysymbol_kind_t = 35;
pub const YYSYMBOL_reg_3: yysymbol_kind_t = 34;
pub const YYSYMBOL_reg_2: yysymbol_kind_t = 33;
pub const YYSYMBOL_reg_1: yysymbol_kind_t = 32;
pub const YYSYMBOL_reg_0: yysymbol_kind_t = 31;
pub const YYSYMBOL_statement: yysymbol_kind_t = 30;
pub const YYSYMBOL_statseq: yysymbol_kind_t = 29;
pub const YYSYMBOL_YYACCEPT: yysymbol_kind_t = 28;
pub const YYSYMBOL_PNAME: yysymbol_kind_t = 27;
pub const YYSYMBOL_LNAME: yysymbol_kind_t = 26;
pub const YYSYMBOL_RBRACE: yysymbol_kind_t = 25;
pub const YYSYMBOL_VBAR: yysymbol_kind_t = 24;
pub const YYSYMBOL_LBRACE: yysymbol_kind_t = 23;
pub const YYSYMBOL_CIRCUMFLEX: yysymbol_kind_t = 22;
pub const YYSYMBOL_RBRACK: yysymbol_kind_t = 21;
pub const YYSYMBOL_BSLASH: yysymbol_kind_t = 20;
pub const YYSYMBOL_LBRACK: yysymbol_kind_t = 19;
pub const YYSYMBOL_AT: yysymbol_kind_t = 18;
pub const YYSYMBOL_QUESTION: yysymbol_kind_t = 17;
pub const YYSYMBOL_EQUAL: yysymbol_kind_t = 16;
pub const YYSYMBOL_SEMI: yysymbol_kind_t = 15;
pub const YYSYMBOL_COLON: yysymbol_kind_t = 14;
pub const YYSYMBOL_SLASH: yysymbol_kind_t = 13;
pub const YYSYMBOL_MINUS: yysymbol_kind_t = 12;
pub const YYSYMBOL_COMMA: yysymbol_kind_t = 11;
pub const YYSYMBOL_PLUS: yysymbol_kind_t = 10;
pub const YYSYMBOL_STAR: yysymbol_kind_t = 9;
pub const YYSYMBOL_RPAREN: yysymbol_kind_t = 8;
pub const YYSYMBOL_LPAREN: yysymbol_kind_t = 7;
pub const YYSYMBOL_AMPERSAND: yysymbol_kind_t = 6;
pub const YYSYMBOL_PERCENT: yysymbol_kind_t = 5;
pub const YYSYMBOL_DOLLAR: yysymbol_kind_t = 4;
pub const YYSYMBOL_EXCLAM: yysymbol_kind_t = 3;
pub const YYSYMBOL_YYUNDEF: yysymbol_kind_t = 2;
pub const YYSYMBOL_YYerror: yysymbol_kind_t = 1;
pub const YYSYMBOL_YYEOF: yysymbol_kind_t = 0;
pub const YYSYMBOL_YYEMPTY: yysymbol_kind_t = -2;
pub type yy_state_fast_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub union yyalloc {
    pub yyss_alloc: yy_state_t,
    pub yyvs_alloc: YYSTYPE,
}
#[no_mangle]
pub static mut i: libc::c_int = 0;
#[no_mangle]
pub static mut num: libc::c_int = 0;
#[no_mangle]
pub static mut P: P_OBJECT = 0 as *const P_desc as *mut P_desc;
#[no_mangle]
pub static mut Q: Q_OBJECT = 0 as *const Q_desc as *mut Q_desc;
#[no_mangle]
pub static mut t: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut l: libc::c_int = 0;
#[no_mangle]
pub static mut i_var: libc::c_int = 0;
#[no_mangle]
pub static mut tapeno: libc::c_int = 0;
#[no_mangle]
pub static mut i_tok: libc::c_int = 0;
#[no_mangle]
pub static mut i_aut: libc::c_int = 0;
static mut yytranslate: [yytype_int8; 283] = [
    0 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
];
static mut yypact: [yytype_int8; 91] = [
    -(41 as libc::c_int) as yytype_int8,
    15 as libc::c_int as yytype_int8,
    -(41 as libc::c_int) as yytype_int8,
    -(6 as libc::c_int) as yytype_int8,
    59 as libc::c_int as yytype_int8,
    84 as libc::c_int as yytype_int8,
    82 as libc::c_int as yytype_int8,
    -(41 as libc::c_int) as yytype_int8,
    65 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    -(41 as libc::c_int) as yytype_int8,
    113 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    126 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    35 as libc::c_int as yytype_int8,
    116 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    -(41 as libc::c_int) as yytype_int8,
    -(41 as libc::c_int) as yytype_int8,
    82 as libc::c_int as yytype_int8,
    84 as libc::c_int as yytype_int8,
    -(41 as libc::c_int) as yytype_int8,
    -(41 as libc::c_int) as yytype_int8,
    44 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    -(41 as libc::c_int) as yytype_int8,
    -(41 as libc::c_int) as yytype_int8,
    48 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    -(41 as libc::c_int) as yytype_int8,
    44 as libc::c_int as yytype_int8,
    -(4 as libc::c_int) as yytype_int8,
    42 as libc::c_int as yytype_int8,
    -(41 as libc::c_int) as yytype_int8,
    76 as libc::c_int as yytype_int8,
    116 as libc::c_int as yytype_int8,
    116 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    93 as libc::c_int as yytype_int8,
    116 as libc::c_int as yytype_int8,
    116 as libc::c_int as yytype_int8,
    116 as libc::c_int as yytype_int8,
    116 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    -(41 as libc::c_int) as yytype_int8,
    -(41 as libc::c_int) as yytype_int8,
    -(41 as libc::c_int) as yytype_int8,
    99 as libc::c_int as yytype_int8,
    44 as libc::c_int as yytype_int8,
    84 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    -(41 as libc::c_int) as yytype_int8,
    82 as libc::c_int as yytype_int8,
    -(41 as libc::c_int) as yytype_int8,
    -(41 as libc::c_int) as yytype_int8,
    -(41 as libc::c_int) as yytype_int8,
    82 as libc::c_int as yytype_int8,
    -(41 as libc::c_int) as yytype_int8,
    84 as libc::c_int as yytype_int8,
    -(41 as libc::c_int) as yytype_int8,
    84 as libc::c_int as yytype_int8,
    116 as libc::c_int as yytype_int8,
    126 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    116 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    116 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    35 as libc::c_int as yytype_int8,
    116 as libc::c_int as yytype_int8,
    84 as libc::c_int as yytype_int8,
    118 as libc::c_int as yytype_int8,
    44 as libc::c_int as yytype_int8,
    44 as libc::c_int as yytype_int8,
    52 as libc::c_int as yytype_int8,
    -(41 as libc::c_int) as yytype_int8,
    126 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    114 as libc::c_int as yytype_int8,
    -(41 as libc::c_int) as yytype_int8,
    -(41 as libc::c_int) as yytype_int8,
    -(41 as libc::c_int) as yytype_int8,
];
static mut yydefact: [yytype_int8; 91] = [
    4 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    40 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    32 as libc::c_int as yytype_int8,
    34 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    49 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    40 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    53 as libc::c_int as yytype_int8,
    54 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    45 as libc::c_int as yytype_int8,
    52 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    35 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    47 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    42 as libc::c_int as yytype_int8,
    48 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    41 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    44 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    46 as libc::c_int as yytype_int8,
    51 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
];
static mut yypgoto: [yytype_int8; 15] = [
    -(41 as libc::c_int) as yytype_int8,
    -(41 as libc::c_int) as yytype_int8,
    -(41 as libc::c_int) as yytype_int8,
    4 as libc::c_int as yytype_int8,
    -(41 as libc::c_int) as yytype_int8,
    -(33 as libc::c_int) as yytype_int8,
    96 as libc::c_int as yytype_int8,
    -(40 as libc::c_int) as yytype_int8,
    100 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    -(18 as libc::c_int) as yytype_int8,
    -(41 as libc::c_int) as yytype_int8,
    -(41 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
];
static mut yydefgoto: [yytype_int8; 15] = [
    0 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    59 as libc::c_int as yytype_int8,
];
static mut yytable: [yytype_int8; 146] = [
    21 as libc::c_int as yytype_int8,
    48 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    73 as libc::c_int as yytype_int8,
    32 as libc::c_int as yytype_int8,
    42 as libc::c_int as yytype_int8,
    67 as libc::c_int as yytype_int8,
    61 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    56 as libc::c_int as yytype_int8,
    35 as libc::c_int as yytype_int8,
    44 as libc::c_int as yytype_int8,
    57 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    49 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    45 as libc::c_int as yytype_int8,
    63 as libc::c_int as yytype_int8,
    62 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    54 as libc::c_int as yytype_int8,
    51 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    70 as libc::c_int as yytype_int8,
    53 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    85 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    86 as libc::c_int as yytype_int8,
    84 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    -(53 as libc::c_int) as yytype_int8,
    65 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    -(54 as libc::c_int) as yytype_int8,
    55 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    47 as libc::c_int as yytype_int8,
    63 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    60 as libc::c_int as yytype_int8,
    46 as libc::c_int as yytype_int8,
    65 as libc::c_int as yytype_int8,
    52 as libc::c_int as yytype_int8,
    79 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    55 as libc::c_int as yytype_int8,
    48 as libc::c_int as yytype_int8,
    89 as libc::c_int as yytype_int8,
    80 as libc::c_int as yytype_int8,
    82 as libc::c_int as yytype_int8,
    58 as libc::c_int as yytype_int8,
    83 as libc::c_int as yytype_int8,
    81 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    76 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    77 as libc::c_int as yytype_int8,
    87 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    34 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    66 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    78 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    72 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    90 as libc::c_int as yytype_int8,
    40 as libc::c_int as yytype_int8,
    41 as libc::c_int as yytype_int8,
    55 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    68 as libc::c_int as yytype_int8,
    69 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    74 as libc::c_int as yytype_int8,
    75 as libc::c_int as yytype_int8,
];
static mut yycheck: [yytype_int8; 146] = [
    1 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    42 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    70 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    72 as libc::c_int as yytype_int8,
    66 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    55 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    52 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    77 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    57 as libc::c_int as yytype_int8,
    63 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    65 as libc::c_int as yytype_int8,
    61 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    46 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    47 as libc::c_int as yytype_int8,
    78 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    22 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    18 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    22 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    26 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    22 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    26 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    19 as libc::c_int as yytype_int8,
    40 as libc::c_int as yytype_int8,
    41 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    44 as libc::c_int as yytype_int8,
    45 as libc::c_int as yytype_int8,
];
static mut yystos: [yytype_int8; 91] = [
    0 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    32 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    34 as libc::c_int as yytype_int8,
    35 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    42 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    40 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    42 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    41 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    42 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    42 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    42 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    34 as libc::c_int as yytype_int8,
    34 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    35 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    35 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    42 as libc::c_int as yytype_int8,
    42 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    35 as libc::c_int as yytype_int8,
    35 as libc::c_int as yytype_int8,
    42 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
];
static mut yyr1: [yytype_int8; 55] = [
    0 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    32 as libc::c_int as yytype_int8,
    32 as libc::c_int as yytype_int8,
    32 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    34 as libc::c_int as yytype_int8,
    34 as libc::c_int as yytype_int8,
    34 as libc::c_int as yytype_int8,
    34 as libc::c_int as yytype_int8,
    34 as libc::c_int as yytype_int8,
    35 as libc::c_int as yytype_int8,
    35 as libc::c_int as yytype_int8,
    35 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    40 as libc::c_int as yytype_int8,
    40 as libc::c_int as yytype_int8,
    40 as libc::c_int as yytype_int8,
    40 as libc::c_int as yytype_int8,
    40 as libc::c_int as yytype_int8,
    41 as libc::c_int as yytype_int8,
    41 as libc::c_int as yytype_int8,
    42 as libc::c_int as yytype_int8,
    42 as libc::c_int as yytype_int8,
];
static mut yyr2: [yytype_int8; 55] = [
    0 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
];
unsafe extern "C" fn yydestruct(
    mut yymsg: *const libc::c_char,
    mut yykind: yysymbol_kind_t,
    mut yyvaluep: *mut YYSTYPE,
) {
    if yymsg.is_null() {
        yymsg = b"Deleting\0" as *const u8 as *const libc::c_char;
    }
}
#[no_mangle]
pub static mut yychar: libc::c_int = 0;
#[no_mangle]
pub static mut yylval: YYSTYPE = YYSTYPE {
    up: 0 as *const P_desc as *mut P_desc,
};
#[no_mangle]
pub static mut yynerrs: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn yyparse() -> libc::c_int {
    let mut current_block: u64;
    let mut yystate: yy_state_fast_t = 0 as libc::c_int;
    let mut yyerrstatus: libc::c_int = 0 as libc::c_int;
    let mut yystacksize: libc::c_long = 200 as libc::c_int as libc::c_long;
    let mut yyssa: [yy_state_t; 200] = [0; 200];
    let mut yyss: *mut yy_state_t = yyssa.as_mut_ptr();
    let mut yyssp: *mut yy_state_t = yyss;
    let mut yyvsa: [YYSTYPE; 200] = [YYSTYPE {
        up: 0 as *const P_desc as *mut P_desc,
    }; 200];
    let mut yyvs: *mut YYSTYPE = yyvsa.as_mut_ptr();
    let mut yyvsp: *mut YYSTYPE = yyvs;
    let mut yyn: libc::c_int = 0;
    let mut yyresult: libc::c_int = 0;
    let mut yytoken: yysymbol_kind_t = YYSYMBOL_YYEMPTY;
    let mut yyval: YYSTYPE = YYSTYPE {
        up: 0 as *const P_desc as *mut P_desc,
    };
    let mut yylen: libc::c_int = 0 as libc::c_int;
    yychar = -(2 as libc::c_int);
    's_64: loop {
        (0 as libc::c_int != 0
            && (0 as libc::c_int <= yystate && yystate < 91 as libc::c_int))
            as libc::c_int;
        *yyssp = yystate as yy_state_t;
        if yyss.offset(yystacksize as isize).offset(-(1 as libc::c_int as isize))
            <= yyssp
        {
            let mut yysize: libc::c_long = yyssp.offset_from(yyss) as libc::c_long
                + 1 as libc::c_int as libc::c_long;
            if 10000 as libc::c_int as libc::c_long <= yystacksize {
                current_block = 5824645410755033635;
                break;
            }
            yystacksize *= 2 as libc::c_int as libc::c_long;
            if (10000 as libc::c_int as libc::c_long) < yystacksize {
                yystacksize = 10000 as libc::c_int as libc::c_long;
            }
            let mut yyss1: *mut yy_state_t = yyss;
            let mut yyptr: *mut yyalloc = malloc(
                (yystacksize
                    * (::std::mem::size_of::<yy_state_t>() as libc::c_ulong
                        as libc::c_long
                        + ::std::mem::size_of::<YYSTYPE>() as libc::c_ulong
                            as libc::c_long)
                    + (::std::mem::size_of::<yyalloc>() as libc::c_ulong as libc::c_long
                        - 1 as libc::c_int as libc::c_long)) as libc::c_ulong,
            ) as *mut yyalloc;
            if yyptr.is_null() {
                current_block = 5824645410755033635;
                break;
            }
            let mut yynewbytes: libc::c_long = 0;
            libc::memcpy(
                &mut (*yyptr).yyss_alloc as *mut yy_state_t as *mut libc::c_void,
                yyss as *const libc::c_void,
                (yysize as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<yy_state_t>() as libc::c_ulong)
                    as libc::size_t,
            );
            yyss = &mut (*yyptr).yyss_alloc;
            yynewbytes = yystacksize
                * ::std::mem::size_of::<yy_state_t>() as libc::c_ulong as libc::c_long
                + (::std::mem::size_of::<yyalloc>() as libc::c_ulong as libc::c_long
                    - 1 as libc::c_int as libc::c_long);
            yyptr = yyptr
                .offset(
                    (yynewbytes
                        / ::std::mem::size_of::<yyalloc>() as libc::c_ulong
                            as libc::c_long) as isize,
                );
            let mut yynewbytes_0: libc::c_long = 0;
            libc::memcpy(
                &mut (*yyptr).yyvs_alloc as *mut YYSTYPE as *mut libc::c_void,
                yyvs as *const libc::c_void,
                (yysize as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<YYSTYPE>() as libc::c_ulong)
                    as libc::size_t,
            );
            yyvs = &mut (*yyptr).yyvs_alloc;
            yynewbytes_0 = yystacksize
                * ::std::mem::size_of::<YYSTYPE>() as libc::c_ulong as libc::c_long
                + (::std::mem::size_of::<yyalloc>() as libc::c_ulong as libc::c_long
                    - 1 as libc::c_int as libc::c_long);
            yyptr = yyptr
                .offset(
                    (yynewbytes_0
                        / ::std::mem::size_of::<yyalloc>() as libc::c_ulong
                            as libc::c_long) as isize,
                );
            if yyss1 != yyssa.as_mut_ptr() {
                free(yyss1 as *mut libc::c_void);
            }
            yyssp = yyss.offset(yysize as isize).offset(-(1 as libc::c_int as isize));
            yyvsp = yyvs.offset(yysize as isize).offset(-(1 as libc::c_int as isize));
            if yyss.offset(yystacksize as isize).offset(-(1 as libc::c_int as isize))
                <= yyssp
            {
                current_block = 16957704171120644238;
                break;
            }
        }
        if yystate == 2 as libc::c_int {
            yyresult = 0 as libc::c_int;
            current_block = 15939128995536588556;
            break;
        } else {
            yyn = yypact[yystate as usize] as libc::c_int;
            if yyn == -(41 as libc::c_int) {
                current_block = 8315090260299038268;
            } else {
                if yychar == -(2 as libc::c_int) {
                    yychar = yylex();
                }
                if yychar <= 0 as libc::c_int {
                    yychar = 0 as libc::c_int;
                    yytoken = YYSYMBOL_YYEOF;
                    current_block = 17728966195399430138;
                } else if yychar == 256 as libc::c_int {
                    yychar = 257 as libc::c_int;
                    yytoken = YYSYMBOL_YYerror;
                    current_block = 12649877089218048548;
                } else {
                    yytoken = (if 0 as libc::c_int <= yychar
                        && yychar <= 282 as libc::c_int
                    {
                        yytranslate[yychar as usize] as yysymbol_kind_t as libc::c_int
                    } else {
                        YYSYMBOL_YYUNDEF as libc::c_int
                    }) as yysymbol_kind_t;
                    current_block = 17728966195399430138;
                }
                match current_block {
                    12649877089218048548 => {}
                    _ => {
                        yyn += yytoken as libc::c_int;
                        if yyn < 0 as libc::c_int || (145 as libc::c_int) < yyn
                            || yycheck[yyn as usize] as libc::c_int
                                != yytoken as libc::c_int
                        {
                            current_block = 8315090260299038268;
                        } else {
                            yyn = yytable[yyn as usize] as libc::c_int;
                            if yyn <= 0 as libc::c_int {
                                yyn = -yyn;
                                current_block = 4512938705799644664;
                            } else {
                                if yyerrstatus != 0 {
                                    yyerrstatus -= 1;
                                }
                                yystate = yyn;
                                yyvsp = yyvsp.offset(1);
                                *yyvsp = yylval;
                                yychar = -(2 as libc::c_int);
                                current_block = 2854092039226949611;
                            }
                        }
                    }
                }
            }
            match current_block {
                8315090260299038268 => {
                    yyn = yydefact[yystate as usize] as libc::c_int;
                    if yyn == 0 as libc::c_int {
                        yytoken = (if yychar == -(2 as libc::c_int) {
                            YYSYMBOL_YYEMPTY as libc::c_int
                        } else if 0 as libc::c_int <= yychar
                            && yychar <= 282 as libc::c_int
                        {
                            yytranslate[yychar as usize] as yysymbol_kind_t
                                as libc::c_int
                        } else {
                            YYSYMBOL_YYUNDEF as libc::c_int
                        }) as yysymbol_kind_t;
                        if yyerrstatus == 0 {
                            yynerrs += 1;
                            yyerror(
                                b"syntax error\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                        }
                        if yyerrstatus == 3 as libc::c_int {
                            if yychar <= 0 as libc::c_int {
                                if yychar == 0 as libc::c_int {
                                    current_block = 16957704171120644238;
                                    break;
                                }
                            } else {
                                yydestruct(
                                    b"Error: discarding\0" as *const u8 as *const libc::c_char,
                                    yytoken,
                                    &mut yylval,
                                );
                                yychar = -(2 as libc::c_int);
                            }
                        }
                        current_block = 12649877089218048548;
                    } else {
                        current_block = 4512938705799644664;
                    }
                }
                _ => {}
            }
            match current_block {
                12649877089218048548 => {
                    yyerrstatus = 3 as libc::c_int;
                    loop {
                        yyn = yypact[yystate as usize] as libc::c_int;
                        if !(yyn == -(41 as libc::c_int)) {
                            yyn += YYSYMBOL_YYerror as libc::c_int;
                            if 0 as libc::c_int <= yyn && yyn <= 145 as libc::c_int
                                && yycheck[yyn as usize] as libc::c_int
                                    == YYSYMBOL_YYerror as libc::c_int
                            {
                                yyn = yytable[yyn as usize] as libc::c_int;
                                if (0 as libc::c_int) < yyn {
                                    break;
                                }
                            }
                        }
                        if yyssp == yyss {
                            current_block = 16957704171120644238;
                            break 's_64;
                        }
                        yydestruct(
                            b"Error: popping\0" as *const u8 as *const libc::c_char,
                            yystos[yystate as usize] as yysymbol_kind_t,
                            yyvsp,
                        );
                        yyvsp = yyvsp.offset(-(1 as libc::c_int as isize));
                        yyssp = yyssp.offset(-(1 as libc::c_int as isize));
                        yystate = *yyssp as yy_state_fast_t;
                    }
                    yyvsp = yyvsp.offset(1);
                    *yyvsp = yylval;
                    yystate = yyn;
                }
                4512938705799644664 => {
                    yylen = yyr2[yyn as usize] as libc::c_int;
                    yyval = *yyvsp.offset((1 as libc::c_int - yylen) as isize);
                    match yyn {
                        3 => {
                            yyerrstatus = 0 as libc::c_int;
                            if A_report != 0 {
                                pr_time_diff();
                            }
                            if isatty(fileno(fpin)) != 0 && isatty(fileno(fpout)) != 0 {
                                printf(b"--* \0" as *const u8 as *const libc::c_char);
                            }
                        }
                        5 => {
                            A = (*yyvsp.offset(-(1 as libc::c_int) as isize)).uA;
                            if disp_flag == 2 as libc::c_int {
                                A = A_min(A);
                            }
                            if disp_flag != 0 {
                                A_rept(A);
                                fprintf(fpout, b"\n\0" as *const u8 as *const libc::c_char);
                                A_pr(A, 0 as *mut libc::c_void as *mut libc::c_char, TT2);
                                fprintf(fpout, b"\n\0" as *const u8 as *const libc::c_char);
                            }
                            A_destroy(*Alist.offset(0 as libc::c_int as isize));
                            let ref mut fresh0 = *Alist
                                .offset(0 as libc::c_int as isize);
                            *fresh0 = A;
                            if A_report != 0 {
                                pr_time_diff();
                            }
                            if isatty(fileno(fpin)) != 0 && isatty(fileno(fpout)) != 0 {
                                printf(b"--* \0" as *const u8 as *const libc::c_char);
                            }
                        }
                        6 => {
                            A = A_enum(
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).uA,
                                TT2,
                                1000 as libc::c_int,
                            );
                            A_destroy(*Alist.offset(0 as libc::c_int as isize));
                            let ref mut fresh1 = *Alist
                                .offset(0 as libc::c_int as isize);
                            *fresh1 = A;
                            if A_report != 0 {
                                pr_time_diff();
                            }
                            if isatty(fileno(fpin)) != 0 && isatty(fileno(fpout)) != 0 {
                                printf(b"--* \0" as *const u8 as *const libc::c_char);
                            }
                        }
                        7 => {
                            A = (*yyvsp.offset(-(1 as libc::c_int) as isize)).uA;
                            if disp_flag == 2 as libc::c_int {
                                A = A_min(A);
                            }
                            if disp_flag != 0 {
                                fprintf(
                                    fpout,
                                    b"%s  \0" as *const u8 as *const libc::c_char,
                                    pad20(
                                        P_cstr((*yyvsp.offset(-(3 as libc::c_int) as isize)).up),
                                    ),
                                );
                                A_rept(A);
                            }
                            i = Tn_member(
                                TAlist,
                                P_cstr((*yyvsp.offset(-(3 as libc::c_int) as isize)).up),
                                P_length((*yyvsp.offset(-(3 as libc::c_int) as isize)).up),
                            );
                            if i >= 0 as libc::c_int {
                                A_destroy(*Alist.offset(i as isize));
                            }
                            i_aut = Tn_insert(
                                TAlist,
                                P_cstr((*yyvsp.offset(-(3 as libc::c_int) as isize)).up),
                                P_length((*yyvsp.offset(-(3 as libc::c_int) as isize)).up),
                            );
                            if i_aut as libc::c_ulong
                                >= (Ssize(Alist as *mut libc::c_char) as libc::c_ulong)
                                    .wrapping_div(
                                        ::std::mem::size_of::<A_OBJECT>() as libc::c_ulong,
                                    )
                            {
                                Alist = Srealloc(
                                    Alist as *mut libc::c_char,
                                    ((2 as libc::c_int * i_aut) as libc::c_ulong)
                                        .wrapping_mul(
                                            ::std::mem::size_of::<A_OBJECT>() as libc::c_ulong,
                                        ) as libc::c_long,
                                ) as *mut A_OBJECT;
                            }
                            let ref mut fresh2 = *Alist.offset(i_aut as isize);
                            *fresh2 = A;
                            if T2_member(
                                TT2,
                                P_cstr((*yyvsp.offset(-(3 as libc::c_int) as isize)).up),
                                P_length((*yyvsp.offset(-(3 as libc::c_int) as isize)).up),
                            ) >= 0 as libc::c_int
                            {
                                fprintf(
                                    fpout,
                                    b"Warning: %s is also a token\n\0" as *const u8
                                        as *const libc::c_char,
                                    P_cstr((*yyvsp.offset(-(3 as libc::c_int) as isize)).up),
                                );
                            }
                            P_destroy((*yyvsp.offset(-(3 as libc::c_int) as isize)).up);
                            if A_report != 0 {
                                pr_time_diff();
                            }
                            if isatty(fileno(fpin)) != 0 && isatty(fileno(fpout)) != 0 {
                                printf(b"--* \0" as *const u8 as *const libc::c_char);
                            }
                        }
                        8 => {
                            if strcmp(
                                b"read\0" as *const u8 as *const libc::c_char,
                                P_cstr((*yyvsp.offset(-(1 as libc::c_int) as isize)).up),
                            ) == 0
                                || strcmp(
                                    b"load\0" as *const u8 as *const libc::c_char,
                                    P_cstr((*yyvsp.offset(-(1 as libc::c_int) as isize)).up),
                                ) == 0
                            {
                                A = A_load(
                                    P_cstr((*yyvsp.offset(-(4 as libc::c_int) as isize)).up),
                                    TT2,
                                );
                            } else {
                                Warning(
                                    b"Unknown function\0" as *const u8 as *const libc::c_char
                                        as *mut libc::c_char,
                                );
                                A = A_create();
                            }
                            fprintf(
                                fpout,
                                b"%s  \0" as *const u8 as *const libc::c_char,
                                pad20(
                                    P_cstr((*yyvsp.offset(-(4 as libc::c_int) as isize)).up),
                                ),
                            );
                            A_rept(A);
                            i = Tn_member(
                                TAlist,
                                P_cstr((*yyvsp.offset(-(4 as libc::c_int) as isize)).up),
                                P_length((*yyvsp.offset(-(4 as libc::c_int) as isize)).up),
                            );
                            if i >= 0 as libc::c_int {
                                A_destroy(*Alist.offset(i as isize));
                            }
                            i_aut = Tn_insert(
                                TAlist,
                                P_cstr((*yyvsp.offset(-(4 as libc::c_int) as isize)).up),
                                P_length((*yyvsp.offset(-(4 as libc::c_int) as isize)).up),
                            );
                            if i_aut as libc::c_ulong
                                >= (Ssize(Alist as *mut libc::c_char) as libc::c_ulong)
                                    .wrapping_div(
                                        ::std::mem::size_of::<A_OBJECT>() as libc::c_ulong,
                                    )
                            {
                                Alist = Srealloc(
                                    Alist as *mut libc::c_char,
                                    ((2 as libc::c_int * i_aut) as libc::c_ulong)
                                        .wrapping_mul(
                                            ::std::mem::size_of::<A_OBJECT>() as libc::c_ulong,
                                        ) as libc::c_long,
                                ) as *mut A_OBJECT;
                            }
                            let ref mut fresh3 = *Alist.offset(i_aut as isize);
                            *fresh3 = A;
                            if T2_member(
                                TT2,
                                P_cstr((*yyvsp.offset(-(4 as libc::c_int) as isize)).up),
                                P_length((*yyvsp.offset(-(4 as libc::c_int) as isize)).up),
                            ) >= 0 as libc::c_int
                            {
                                fprintf(
                                    fpout,
                                    b"Warning: %s is also a token\n\0" as *const u8
                                        as *const libc::c_char,
                                    P_cstr((*yyvsp.offset(-(4 as libc::c_int) as isize)).up),
                                );
                            }
                            P_destroy((*yyvsp.offset(-(4 as libc::c_int) as isize)).up);
                            P_destroy((*yyvsp.offset(-(1 as libc::c_int) as isize)).up);
                            if A_report != 0 {
                                pr_time_diff();
                            }
                            if isatty(fileno(fpin)) != 0 && isatty(fileno(fpout)) != 0 {
                                printf(b"--* \0" as *const u8 as *const libc::c_char);
                            }
                        }
                        9 => {
                            i = do_n_i(
                                P_cstr((*yyvsp.offset(-(1 as libc::c_int) as isize)).up),
                            );
                            P_destroy((*yyvsp.offset(-(1 as libc::c_int) as isize)).up);
                            if i != 0 {
                                return 0 as libc::c_int;
                            }
                            if A_report != 0 {
                                pr_time_diff();
                            }
                            if isatty(fileno(fpin)) != 0 && isatty(fileno(fpout)) != 0 {
                                printf(b"--* \0" as *const u8 as *const libc::c_char);
                            }
                        }
                        10 => {
                            yyval
                                .uA = do_an_a(
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).uA,
                                P_cstr((*yyvsp.offset(0 as libc::c_int as isize)).up),
                            );
                            P_destroy((*yyvsp.offset(0 as libc::c_int as isize)).up);
                        }
                        11 => {
                            disp_flag = 0 as libc::c_int;
                            A = (*yyvsp.offset(-(4 as libc::c_int) as isize)).uA;
                            num = tonum(
                                P_cstr((*yyvsp.offset(-(1 as libc::c_int) as isize)).up),
                            );
                            if num >= 0 as libc::c_int {
                                if A_report != 0 {
                                    fprintf(
                                        fpout,
                                        b"(@pow)\n\0" as *const u8 as *const libc::c_char,
                                    );
                                }
                                A = A_cmpow(
                                    (*yyvsp.offset(-(4 as libc::c_int) as isize)).uA,
                                    num,
                                );
                                disp_flag = 2 as libc::c_int;
                            } else {
                                Warning(
                                    b"Unknown function\0" as *const u8 as *const libc::c_char
                                        as *mut libc::c_char,
                                );
                            }
                            P_destroy((*yyvsp.offset(-(1 as libc::c_int) as isize)).up);
                            yyval.uA = A;
                        }
                        12 => {
                            A = do_ann_a(
                                (*yyvsp.offset(-(3 as libc::c_int) as isize)).uA,
                                P_cstr((*yyvsp.offset(-(1 as libc::c_int) as isize)).up),
                                P_cstr((*yyvsp.offset(0 as libc::c_int as isize)).up),
                            );
                            P_destroy((*yyvsp.offset(-(1 as libc::c_int) as isize)).up);
                            P_destroy((*yyvsp.offset(0 as libc::c_int as isize)).up);
                            yyval.uA = A;
                        }
                        13 => {
                            A = do_nn_a(
                                P_cstr((*yyvsp.offset(-(1 as libc::c_int) as isize)).up),
                                P_cstr((*yyvsp.offset(0 as libc::c_int as isize)).up),
                            );
                            P_destroy((*yyvsp.offset(-(1 as libc::c_int) as isize)).up);
                            P_destroy((*yyvsp.offset(0 as libc::c_int as isize)).up);
                            if A.is_null() {
                                A = A_create();
                            }
                            yyval.uA = A;
                        }
                        14 => {
                            disp_flag = 2 as libc::c_int;
                        }
                        15 => {
                            if A_report != 0 {
                                fprintf(
                                    fpout,
                                    b"(@)\n\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            yyval
                                .uA = A_compose(
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).uA,
                                (*yyvsp.offset(0 as libc::c_int as isize)).uA,
                            );
                        }
                        16 => {
                            if A_report != 0 {
                                fprintf(
                                    fpout,
                                    b"(@@)\n\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            yyval
                                .uA = A_join(
                                (*yyvsp.offset(-(3 as libc::c_int) as isize)).uA,
                                (*yyvsp.offset(0 as libc::c_int as isize)).uA,
                            );
                        }
                        18 => {
                            if A_report != 0 {
                                fprintf(
                                    fpout,
                                    b"($)\n\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            yyval
                                .uA = A_retape(
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).uA,
                                (*yyvsp.offset(0 as libc::c_int as isize)).uA,
                                TT2,
                            );
                        }
                        19 => {
                            if A_report != 0 {
                                fprintf(
                                    fpout,
                                    b"(%%)\n\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            yyval
                                .uA = A_percent(
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).uA,
                                (*yyvsp.offset(0 as libc::c_int as isize)).uA,
                            );
                        }
                        21 => {
                            if A_report != 0 {
                                fprintf(
                                    fpout,
                                    b"(|)\n\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            yyval
                                .uA = A_union(
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).uA,
                                (*yyvsp.offset(0 as libc::c_int as isize)).uA,
                            );
                        }
                        22 => {
                            if A_report != 0 {
                                fprintf(
                                    fpout,
                                    b"(!)\n\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            yyval
                                .uA = A_xor(
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).uA,
                                (*yyvsp.offset(0 as libc::c_int as isize)).uA,
                            );
                        }
                        23 => {
                            if A_report != 0 {
                                fprintf(
                                    fpout,
                                    b"(||)\n\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            if (*(*yyvsp.offset(-(3 as libc::c_int) as isize)).uA).A_nT
                                > 1 as libc::c_int
                                || (*(*yyvsp.offset(0 as libc::c_int as isize)).uA).A_nT
                                    > 1 as libc::c_int
                            {
                                Atemp = A_differ(
                                    A_retape(
                                        A_copy((*yyvsp.offset(0 as libc::c_int as isize)).uA),
                                        A_letter(
                                            0 as libc::c_int,
                                            T2_insert(
                                                TT2,
                                                b"0\0" as *const u8 as *const libc::c_char
                                                    as *mut libc::c_char,
                                                1 as libc::c_int,
                                            ),
                                        ),
                                        TT2,
                                    ),
                                    A_retape(
                                        A_copy((*yyvsp.offset(-(3 as libc::c_int) as isize)).uA),
                                        A_letter(
                                            0 as libc::c_int,
                                            T2_insert(
                                                TT2,
                                                b"0\0" as *const u8 as *const libc::c_char
                                                    as *mut libc::c_char,
                                                1 as libc::c_int,
                                            ),
                                        ),
                                        TT2,
                                    ),
                                );
                                yyval
                                    .uA = A_union(
                                    (*yyvsp.offset(-(3 as libc::c_int) as isize)).uA,
                                    A_join(Atemp, (*yyvsp.offset(0 as libc::c_int as isize)).uA),
                                );
                            } else {
                                yyval
                                    .uA = A_union(
                                    (*yyvsp.offset(-(3 as libc::c_int) as isize)).uA,
                                    (*yyvsp.offset(0 as libc::c_int as isize)).uA,
                                );
                            }
                        }
                        24 => {
                            if A_report != 0 {
                                fprintf(
                                    fpout,
                                    b"(!!)\n\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            yyval
                                .uA = A_shuffle(
                                (*yyvsp.offset(-(3 as libc::c_int) as isize)).uA,
                                (*yyvsp.offset(0 as libc::c_int as isize)).uA,
                            );
                        }
                        26 => {
                            if A_report != 0 {
                                fprintf(
                                    fpout,
                                    b"(&)\n\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            yyval
                                .uA = A_intersect(
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).uA,
                                (*yyvsp.offset(0 as libc::c_int as isize)).uA,
                            );
                        }
                        27 => {
                            if A_report != 0 {
                                fprintf(
                                    fpout,
                                    b"(-)\n\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            yyval
                                .uA = A_differ(
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).uA,
                                (*yyvsp.offset(0 as libc::c_int as isize)).uA,
                            );
                        }
                        29 => {
                            if A_report != 0 {
                                fprintf(
                                    fpout,
                                    b"(\\)\n\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            if (*(*yyvsp.offset(-(2 as libc::c_int) as isize)).uA).A_nT
                                > 1 as libc::c_int
                                || (*(*yyvsp.offset(0 as libc::c_int as isize)).uA).A_nT
                                    > 1 as libc::c_int
                            {
                                fprintf(
                                    fpout,
                                    b"Error: \\ applied to multitape automaton\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                                A_destroy((*yyvsp.offset(-(2 as libc::c_int) as isize)).uA);
                                A_destroy((*yyvsp.offset(0 as libc::c_int as isize)).uA);
                                yyval.uA = A_phi();
                            } else {
                                Atemp = A_retape(
                                    A_star(
                                        A_alph(
                                            A_copy((*yyvsp.offset(0 as libc::c_int as isize)).uA),
                                        ),
                                    ),
                                    A_comma(
                                        A_letter(
                                            0 as libc::c_int,
                                            T2_insert(
                                                TT2,
                                                b"0\0" as *const u8 as *const libc::c_char
                                                    as *mut libc::c_char,
                                                1 as libc::c_int,
                                            ),
                                        ),
                                        A_letter(
                                            0 as libc::c_int,
                                            T2_insert(
                                                TT2,
                                                b"0\0" as *const u8 as *const libc::c_char
                                                    as *mut libc::c_char,
                                                1 as libc::c_int,
                                            ),
                                        ),
                                    ),
                                    TT2,
                                );
                                yyval
                                    .uA = A_compose(
                                    (*yyvsp.offset(0 as libc::c_int as isize)).uA,
                                    A_concat(
                                        (*yyvsp.offset(-(2 as libc::c_int) as isize)).uA,
                                        Atemp,
                                    ),
                                );
                            }
                        }
                        31 => {
                            if A_report != 0 {
                                fprintf(
                                    fpout,
                                    b"(/)\n\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            if (*(*yyvsp.offset(-(2 as libc::c_int) as isize)).uA).A_nT
                                > 1 as libc::c_int
                                || (*(*yyvsp.offset(0 as libc::c_int as isize)).uA).A_nT
                                    > 1 as libc::c_int
                            {
                                fprintf(
                                    fpout,
                                    b"Error: / applied to multitape automaton\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                                A_destroy((*yyvsp.offset(-(2 as libc::c_int) as isize)).uA);
                                A_destroy((*yyvsp.offset(0 as libc::c_int as isize)).uA);
                                yyval.uA = A_phi();
                            } else {
                                Atemp = A_retape(
                                    A_star(
                                        A_alph(
                                            A_copy((*yyvsp.offset(-(2 as libc::c_int) as isize)).uA),
                                        ),
                                    ),
                                    A_comma(
                                        A_letter(
                                            0 as libc::c_int,
                                            T2_insert(
                                                TT2,
                                                b"0\0" as *const u8 as *const libc::c_char
                                                    as *mut libc::c_char,
                                                1 as libc::c_int,
                                            ),
                                        ),
                                        A_letter(
                                            0 as libc::c_int,
                                            T2_insert(
                                                TT2,
                                                b"0\0" as *const u8 as *const libc::c_char
                                                    as *mut libc::c_char,
                                                1 as libc::c_int,
                                            ),
                                        ),
                                    ),
                                    TT2,
                                );
                                yyval
                                    .uA = A_compose(
                                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).uA,
                                    A_concat(
                                        Atemp,
                                        (*yyvsp.offset(0 as libc::c_int as isize)).uA,
                                    ),
                                );
                            }
                        }
                        33 => {
                            if A_report != 0 {
                                fprintf(
                                    fpout,
                                    b"( )\n\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            yyval
                                .uA = A_concat(
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).uA,
                                (*yyvsp.offset(0 as libc::c_int as isize)).uA,
                            );
                        }
                        35 => {
                            if A_report != 0 {
                                fprintf(
                                    fpout,
                                    b"(+)\n\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            yyval
                                .uA = A_plus(
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).uA,
                            );
                        }
                        36 => {
                            if A_report != 0 {
                                fprintf(
                                    fpout,
                                    b"(*)\n\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            yyval
                                .uA = A_star(
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).uA,
                            );
                        }
                        37 => {
                            if A_report != 0 {
                                fprintf(
                                    fpout,
                                    b"(?)\n\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            yyval
                                .uA = A_opt(
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).uA,
                            );
                        }
                        38 => {
                            yyval.uA = A_lambda();
                        }
                        39 => {
                            t = P_cstr((*yyvsp.offset(0 as libc::c_int as isize)).up);
                            i = Tn_member(
                                TAlist,
                                t,
                                P_length((*yyvsp.offset(0 as libc::c_int as isize)).up),
                            );
                            if i >= 0 as libc::c_int
                                && T2_member(
                                    TT2,
                                    t,
                                    P_length((*yyvsp.offset(0 as libc::c_int as isize)).up),
                                ) < 0 as libc::c_int
                            {
                                yyval.uA = A_copy(*Alist.offset(i as isize));
                            } else {
                                yyval
                                    .uA = A_letter(
                                    0 as libc::c_int,
                                    T2_insert(
                                        TT2,
                                        t,
                                        P_length((*yyvsp.offset(0 as libc::c_int as isize)).up),
                                    ),
                                );
                                if i >= 0 as libc::c_int {
                                    fprintf(
                                        fpout,
                                        b"Warning: %s is a variable and a token\n\0" as *const u8
                                            as *const libc::c_char,
                                        t,
                                    );
                                }
                            }
                            P_destroy((*yyvsp.offset(0 as libc::c_int as isize)).up);
                        }
                        40 => {
                            P = (*yyvsp.offset(0 as libc::c_int as isize)).up;
                            t = P_cstr(P);
                            l = P_length(P);
                            i_var = Tn_member(TAlist, t, l);
                            Q = Q_fromP(P);
                            t = Q_cstr(Q);
                            l = Q_length(Q);
                            tapeno = Q_tapeno(Q);
                            i_tok = T2_member(TT2, t, l);
                            if i_var >= 0 as libc::c_int && i_tok >= 0 as libc::c_int {
                                fprintf(
                                    fpout,
                                    b"Warning: %s is a variable and a token\n\0" as *const u8
                                        as *const libc::c_char,
                                    t,
                                );
                            }
                            if i_var < 0 as libc::c_int {
                                if tapeno < 0 as libc::c_int {
                                    tapeno = 0 as libc::c_int;
                                }
                                i_tok = T2_insert(TT2, t, l);
                                yyval.uA = A_letter(tapeno, i_tok);
                            } else {
                                yyval.uA = A_copy(*Alist.offset(i_var as isize));
                            }
                            Q_destroy(Q);
                        }
                        41 => {
                            if A_report != 0 {
                                fprintf(
                                    fpout,
                                    b"([])\n\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            yyval
                                .uA = A_comma(
                                A_lambda(),
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).uA,
                            );
                        }
                        42 => {
                            yyval.uA = (*yyvsp.offset(-(1 as libc::c_int) as isize)).uA;
                        }
                        43 => {
                            yyval.uA = A_lambda();
                        }
                        44 => {
                            yyval.uA = (*yyvsp.offset(-(1 as libc::c_int) as isize)).uA;
                        }
                        45 => {
                            yyval.uA = A_phi();
                        }
                        46 => {
                            if A_report != 0 {
                                fprintf(
                                    fpout,
                                    b"(,)\n\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            yyval
                                .uA = A_comma(
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).uA,
                                (*yyvsp.offset(0 as libc::c_int as isize)).uA,
                            );
                        }
                        47 => {
                            if A_report != 0 {
                                fprintf(
                                    fpout,
                                    b"(,)\n\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            yyval
                                .uA = A_comma(
                                A_lambda(),
                                (*yyvsp.offset(0 as libc::c_int as isize)).uA,
                            );
                        }
                        48 => {
                            if A_report != 0 {
                                fprintf(
                                    fpout,
                                    b"(,)\n\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            yyval
                                .uA = A_comma(
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).uA,
                                A_lambda(),
                            );
                        }
                        49 => {
                            if A_report != 0 {
                                fprintf(
                                    fpout,
                                    b"(,)\n\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            yyval.uA = A_comma(A_lambda(), A_lambda());
                        }
                        51 => {
                            if A_report != 0 {
                                fprintf(
                                    fpout,
                                    b"(,|)\n\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            yyval
                                .uA = A_union(
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).uA,
                                (*yyvsp.offset(0 as libc::c_int as isize)).uA,
                            );
                        }
                        _ => {}
                    }
                    yyvsp = yyvsp.offset(-(yylen as isize));
                    yyssp = yyssp.offset(-(yylen as isize));
                    yylen = 0 as libc::c_int;
                    yyvsp = yyvsp.offset(1);
                    *yyvsp = yyval;
                    let yylhs: libc::c_int = yyr1[yyn as usize] as libc::c_int
                        - 28 as libc::c_int;
                    let yyi: libc::c_int = yypgoto[yylhs as usize] as libc::c_int
                        + *yyssp as libc::c_int;
                    yystate = if 0 as libc::c_int <= yyi && yyi <= 145 as libc::c_int
                        && yycheck[yyi as usize] as libc::c_int == *yyssp as libc::c_int
                    {
                        yytable[yyi as usize] as libc::c_int
                    } else {
                        yydefgoto[yylhs as usize] as libc::c_int
                    };
                }
                _ => {}
            }
            yyssp = yyssp.offset(1);
        }
    }
    match current_block {
        5824645410755033635 => {
            yyerror(
                b"memory exhausted\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            yyresult = 2 as libc::c_int;
        }
        16957704171120644238 => {
            yyresult = 1 as libc::c_int;
        }
        _ => {}
    }
    if yychar != -(2 as libc::c_int) {
        yytoken = (if 0 as libc::c_int <= yychar && yychar <= 282 as libc::c_int {
            yytranslate[yychar as usize] as yysymbol_kind_t as libc::c_int
        } else {
            YYSYMBOL_YYUNDEF as libc::c_int
        }) as yysymbol_kind_t;
        yydestruct(
            b"Cleanup: discarding lookahead\0" as *const u8 as *const libc::c_char,
            yytoken,
            &mut yylval,
        );
    }
    yyvsp = yyvsp.offset(-(yylen as isize));
    yyssp = yyssp.offset(-(yylen as isize));
    while yyssp != yyss {
        yydestruct(
            b"Cleanup: popping\0" as *const u8 as *const libc::c_char,
            yystos[*yyssp as libc::c_int as usize] as yysymbol_kind_t,
            yyvsp,
        );
        yyvsp = yyvsp.offset(-(1 as libc::c_int as isize));
        yyssp = yyssp.offset(-(1 as libc::c_int as isize));
    }
    if yyss != yyssa.as_mut_ptr() {
        free(yyss as *mut libc::c_void);
    }
    return yyresult;
}
