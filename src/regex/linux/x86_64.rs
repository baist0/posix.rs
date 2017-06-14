#[repr(C)]
#[derive(Copy, Clone)]
pub struct regex_t {
    __buffer: *mut ::uchar_t,
    __allocated: ::ulong_t,
    __used: ::ulong_t,
    __syntax: ::ulong_t,
    __fastmap: *mut ::char_t,
    __translate: *mut ::uchar_t,
    pub re_nsub: ::size_t,
    __can_be_null: ::uint_t,
    __regs_allocated: ::uint_t,
    __fastmap_accurate: ::uint_t,
    __no_sub: ::uint_t,
    __not_bol: ::uint_t,
    __not_eol: ::uint_t,
    __newline_anchor: ::uint_t,
}

new!(regex_t);

pub type regoff_t = ::int_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct regmatch_t {
    pub rm_so: regoff_t,
    pub rm_eo: regoff_t,
}

new!(regmatch_t);

pub const REG_EXTENDED: ::int_t = 1;
pub const REG_ICASE:    ::int_t = 2;
pub const REG_NOSUB:    ::int_t = 8;
pub const REG_NEWLINE:  ::int_t = 4;
pub const REG_NOTBOL:   ::int_t = 1;
pub const REG_NOTEOL:   ::int_t = 2;
pub const REG_NOMATCH:  ::int_t = 1;
pub const REG_BADPAT:   ::int_t = 2;
pub const REG_ECOLLATE: ::int_t = 3;
pub const REG_ECTYPE:   ::int_t = 4;
pub const REG_EESCAPE:  ::int_t = 5;
pub const REG_ESUBREG:  ::int_t = 6;
pub const REG_EBRACK:   ::int_t = 7;
pub const REG_EPAREN:   ::int_t = 8;
pub const REG_EBRACE:   ::int_t = 9;
pub const REG_BADBR:    ::int_t = 10;
pub const REG_ERANGE:   ::int_t = 11;
pub const REG_ESPACE:   ::int_t = 12;
pub const REG_BADRPT:   ::int_t = 13;
