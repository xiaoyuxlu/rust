#![deny(unsafe_op_in_unsafe_fn)]

pub mod abi;

#[path = "../unsupported/alloc.rs"]
pub mod alloc;
#[path = "../unsupported/args.rs"]
pub mod args;
#[path = "../unix/cmath.rs"]
pub mod cmath;
#[path = "../unsupported/condvar.rs"]
pub mod condvar;
#[path = "../unsupported/env.rs"]
pub mod env;
#[path = "../unsupported/fs.rs"]
pub mod fs;
#[path = "../unsupported/io.rs"]
pub mod io;
#[path = "../unsupported/mutex.rs"]
pub mod mutex;
#[path = "../unsupported/net.rs"]
pub mod net;
#[path = "../unsupported/os.rs"]
pub mod os;
#[path = "../unix/os_str.rs"]
pub mod os_str;
#[path = "../unix/path.rs"]
pub mod path;
#[path = "../unsupported/pipe.rs"]
pub mod pipe;
#[path = "../unsupported/process.rs"]
pub mod process;
#[path = "../unsupported/rwlock.rs"]
pub mod rwlock;
#[path = "../unsupported/stdio.rs"]
pub mod stdio;
#[path = "../unsupported/thread.rs"]
pub mod thread;

#[cfg(target_thread_local)]
#[path = "../unsupported/thread_local_dtor.rs"]
pub mod thread_local_dtor;
pub mod thread_local_key;
pub mod time;

mod common;
pub use common::*;

pub fn abort_internal() -> ! {
    core::intrinsics::abort();
}

// This function is needed by the panic runtime. The symbol is named in
// pre-link args for the target specification, so keep that in sync.
#[cfg(not(test))]
#[no_mangle]
// NB. used by both libunwind and libpanic_abort
pub extern "C" fn __rust_abort() {
    abort_internal();
}