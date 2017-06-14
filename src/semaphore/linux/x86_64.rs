#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem_t {
    _data: [u64; 4usize],
}

new!(sem_t);

pub const SEM_FAILED: *mut sem_t = 0 as *mut _;
