// process
// ---
// Provides code for scheduling processes and providing security among them.

use core::fmt::{Display, Formatter, Error};

// There is a hard limit of at most 2^64 processes at once. It currently seems
//  unlikely that computers will reach that limit since computers do not even
//  have 2^64 bytes of memory (as of the writing of this comment).
pub struct ProcessId(pub u64);

impl Display for ProcessId {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "ProcessId({})", self.0)
    }
}

pub struct Process {
    // Process IDs are guaranteed to be unique among all currently running
    //  processes. However, process IDs of exited processes may be reused.
    pub id: ProcessId,

    // runner_id is None if this is the root process. Otherwise, runner_id is
    //  the ID of the process that handles all system calls for this process.
    //  This allows one process to hook into another process (if given
    //  appropriate permissions). For example, the runner can create fake
    //  hardware events that appear to come from the user.
    pub runner_id: Option<ProcessId>,
}

impl Process {
    pub fn get_current() -> Self {
        // TODO
        Self { id: ProcessId(0), runner_id: None }
    }
}
