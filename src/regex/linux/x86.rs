#[repr(C)]
pub struct regex_t {
    __buffer: [u32, ..1],
    __allocated: [u32, ..1],
    __used: [u32, ..1],
    __syntax: [u32, ..1],
    __fastmap: [u32, ..1],
    __translate: [u32, ..1],
    pub re_nsub: ::size_t,
    __can_be_null: [u32, ..1],
    __regs_allocated: [u32, ..1],
    __fastmap_accurate: [u32, ..1],
    __no_sub: [u32, ..1],
    __not_bol: [u32, ..1],
    __not_eol: [u32, ..1],
    __newline_anchor: [u32, ..1],
}
new!(regex_t)
pub type regoff_t = ::int_t;
#[repr(C)]
pub struct regmatch_t {
    pub rm_so: regoff_t,
    pub rm_eo: regoff_t,
}
new!(regmatch_t)
pub static REG_EXTENDED: ::int_t = 1;
pub static REG_ICASE: ::int_t = (1 << 1);
pub static REG_NOSUB: ::int_t = (((1 << 1) << 1) << 1);
pub static REG_NEWLINE: ::int_t = ((1 << 1) << 1);
pub static REG_NOTBOL: ::int_t = 1;
pub static REG_NOTEOL: ::int_t = (1 << 1);
pub static REG_NOMATCH: ::int_t = 1;
pub static REG_BADPAT: ::int_t = 2;
pub static REG_ECOLLATE: ::int_t = 3;
pub static REG_ECTYPE: ::int_t = 4;
pub static REG_EESCAPE: ::int_t = 5;
pub static REG_ESUBREG: ::int_t = 6;
pub static REG_EBRACK: ::int_t = 7;
pub static REG_EPAREN: ::int_t = 8;
pub static REG_EBRACE: ::int_t = 9;
pub static REG_BADBR: ::int_t = 10;
pub static REG_ERANGE: ::int_t = 11;
pub static REG_ESPACE: ::int_t = 12;
pub static REG_BADRPT: ::int_t = 13;