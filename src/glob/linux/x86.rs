#[repr(C)]
pub struct glob_t {
    pub gl_pathc: ::size_t,
    pub gl_pathv: *mut *mut ::schar_t,
    pub gl_offs: ::size_t,
    pub gl_flags: ::int_t,
    pub gl_closedir: fn(*mut ::void_t, ),
    pub gl_readdir: fn(*mut ::void_t, ) -> *mut ::void_t,
    pub gl_opendir: fn(*const ::schar_t, ) -> *mut ::void_t,
    pub gl_lstat: fn(*const ::schar_t, *mut ::void_t, ) -> ::int_t,
    pub gl_stat: fn(*const ::schar_t, *mut ::void_t, ) -> ::int_t,
}
new!(glob_t)
pub static GLOB_APPEND: ::int_t = (1 << 5);
pub static GLOB_DOOFFS: ::int_t = (1 << 3);
pub static GLOB_ERR: ::int_t = (1 << 0);
pub static GLOB_MARK: ::int_t = (1 << 1);
pub static GLOB_NOCHECK: ::int_t = (1 << 4);
pub static GLOB_NOESCAPE: ::int_t = (1 << 6);
pub static GLOB_NOSORT: ::int_t = (1 << 2);
pub static GLOB_ABORTED: ::int_t = 2;
pub static GLOB_NOMATCH: ::int_t = 3;
pub static GLOB_NOSPACE: ::int_t = 1;
