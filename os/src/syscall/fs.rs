//! File and filesystem-related syscalls

const FD_STDOUT: usize = 1;
use crate::syscall::SYSCALL_WRITE;
use crate::task::TASK_MANAGER;
/// write buf of length `len`  to a file with `fd`
pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    trace!("kernel: sys_write");
    match fd {
        FD_STDOUT => {
            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            let str = core::str::from_utf8(slice).unwrap();
            print!("{}", str);
            TASK_MANAGER.read_inner_systimes_updata(SYSCALL_WRITE);
            //println!("sys_call-write------------------------|");
            len as isize
        }
        _ => {
            panic!("Unsupported fd in sys_write!");
        }
    }
}
