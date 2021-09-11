use crate::process::ProcessId;

pub type ThreadId = u64;

pub struct Thread {
    // Thread IDs are guaranteed to be unique among all running threads *within
    //  a single process* but may be reused after thread exits and within
    //  different processes.
    id: ThreadId,

    // The ID of the process that is running this thread.
    parent_process_id: ProcessId,
}
