type ProcessId = u64;

struct Process {
    id: ProcessId,

    // runner_id is None if this is the root process (so this process is not
    //  controlled by any other process).
    runner_id: Option<ProcessId>,
}
