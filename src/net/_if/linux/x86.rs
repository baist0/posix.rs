#[repr(C)]
pub struct if_nameindex {
    pub if_index: ::uint_t,
    pub if_name: *mut ::char_t,
}

pub static IF_NAMESIZE: ::uint_t = 16;
