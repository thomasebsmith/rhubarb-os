use crate::process::ProcessId;

// There is a hard limit of at most 2^64 threads per process. As with process
//  IDs, this is unlikely to be exceeded.
pub struct ThreadId(u64);

pub struct Thread {
    // Thread IDs are guaranteed to be unique among all running threads *within
    //  a single process* but may be reused after thread exits and within
    //  different processes.
    id: ThreadId,

    // The ID of the process that is running this thread.
    parent_process_id: ProcessId,
}
