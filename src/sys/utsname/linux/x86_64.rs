#[repr(C)]
#[derive(Copy, Clone)]
pub struct utsname {
    pub sysname: [::char_t; 65],
    pub nodename: [::char_t; 65],
    pub release: [::char_t; 65],
    pub version: [::char_t; 65],
    pub machine: [::char_t; 65],
    __domainname: [::char_t; 65],
}

new!(utsname);
