use fs::*;
use proc::*;

mod fs;
mod proc;

const SYS_CALL_GET_CWD: usize = 17;
const SYS_CALL_MKDIR: usize = 34;
const SYS_CALL_CHDIR: usize = 49;
const SYS_CALL_OPEN: usize = 56;
const SYS_CALL_CLOSE: usize = 57;
const SYS_CALL_READ: usize = 63;
const SYS_CALL_WRITE: usize = 64;
const SYS_CALL_EXIT: usize = 93;
const SYS_CALL_YIELD: usize = 124;
const SYS_CALL_GET_TIME: usize = 169;
const SYSCALL_GETPID: usize = 172;
const SYSCALL_FORK: usize = 220;
// const SYSCALL_EXEC: usize = 221;

pub fn sys_call(which: usize, args: [usize; 3]) -> isize {
    match which {
        SYS_CALL_GET_CWD => sys_get_cwd(args[0] as *const u8, args[1] as usize),
        SYS_CALL_MKDIR => sys_mkdir(args[0] as *const u8),
        SYS_CALL_CHDIR => sys_chdir(args[0] as *const u8),
        SYS_CALL_OPEN => sys_open(args[0] as *const u8, args[1] as u32),
        SYS_CALL_CLOSE => sys_close(args[0]),
        SYS_CALL_READ => sys_read(args[0], args[1] as *mut u8, args[2]),
        SYS_CALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYS_CALL_EXIT => sys_exit(args[0] as i32),
        SYS_CALL_YIELD => sys_yield(),
        SYS_CALL_GET_TIME => sys_get_time(),
        SYSCALL_GETPID => sys_getpid(),
        SYSCALL_FORK => sys_fork(),
        // SYSCALL_EXEC => sys_exec(args[0] as *const u8, args[1] as *const usize),
        _ => panic!("sys_call with unknown id: {}", which),
    }
}
