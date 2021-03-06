pub use self::arch::{blkcnt_t};
pub use self::arch::{blksize_t};
pub use self::arch::{clock_t};
pub use self::arch::{clockid_t};
pub use self::arch::{dev_t};
pub use self::arch::{fsblkcnt_t};
pub use self::arch::{fsfilcnt_t};
pub use self::arch::{gid_t};
pub use self::arch::{id_t};
pub use self::arch::{ino_t};
pub use self::arch::{key_t};
pub use self::arch::{mode_t};
pub use self::arch::{nlink_t};
pub use self::arch::{off_t};
pub use self::arch::{pid_t};
pub use self::arch::{pthread_attr_t};
pub use self::arch::{pthread_barrier_t};
pub use self::arch::{pthread_barrierattr_t};
pub use self::arch::{pthread_cond_t};
pub use self::arch::{pthread_condattr_t};
pub use self::arch::{pthread_key_t};
pub use self::arch::{pthread_mutex_t};
pub use self::arch::{pthread_mutexattr_t};
pub use self::arch::{pthread_once_t};
pub use self::arch::{pthread_rwlock_t};
pub use self::arch::{pthread_rwlockattr_t};
pub use self::arch::{pthread_spinlock_t};
pub use self::arch::{suseconds_t};
pub use self::arch::{pthread_t};
pub use self::arch::{time_t};
pub use self::arch::{timer_t};
pub use self::arch::{uid_t};

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;

#[cfg(target_arch = "x86")]
#[path = "x86.rs"]
mod arch;

