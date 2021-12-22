// thread
// ---
// Contains code for creating and scheduling threads.
// The OS's core scheduler deals with threads, not processes,
//  although threads' priorities are influenced by their parent
//  processes' priorities.

use core::fmt::{Display, Formatter, Error};
use crate::process::ProcessId;

// There is a hard limit of at most 2^64 threads per process. As with process
//  IDs, this is unlikely to be exceeded.
pub struct ThreadId(pub u64);

impl Display for ThreadId {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.0.fmt(f)
    }
}

pub struct Thread {
    // Thread IDs are guaranteed to be unique among all running threads *within
    //  a single process* but may be reused after thread exits and within
    //  different processes.
    pub id: ThreadId,

    // The ID of the process that is running this thread.
    pub parent_process_id: ProcessId,
}

impl Thread {
    pub fn get_current() -> Self {
        // TODO
        Self { id: ThreadId(0), parent_process_id: ProcessId(0) }
    }
}
