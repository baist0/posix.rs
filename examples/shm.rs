extern crate posix;

use std::ptr::null;
use posix::sys::shm::{shmget,shmctl,shmdt,shmat,shmid_ds};
use posix::sys::ipc;
use posix::{int_t,void_t,size_t};

pub struct ShMem
{
	shm_id: int_t,
	shm_addr: *const void_t
}

impl ShMem
{
	pub fn new(size: usize) -> Result<ShMem, u8>
	{
		let shm_id: int_t = 
			shmget(ipc::IPC_PRIVATE, size as size_t, ipc::IPC_CREAT | 0o644);
		println!("shmid = {}", shm_id);
		if shm_id < 0 {
			return Err(1);
		}
		let shm_addr: *mut void_t = shmat(shm_id, null(), 0);
		println!("shmaddr = {:p}", shm_addr);
		if shm_addr == ((usize::max_value()) as *mut void_t) {
			return Err(2);
		}
		let mut shm_id_ds = shmid_ds::new();
		shmctl(shm_id, ipc::IPC_RMID, &mut shm_id_ds);
		Ok(ShMem {shm_id, shm_addr})
	}
}

impl Drop for ShMem
{
	fn drop(&mut self)
	{
		println!("drop");
		shmdt(self.shm_addr);
	}
}

fn main()
{
	let a = ShMem::new(1024).unwrap();
}
