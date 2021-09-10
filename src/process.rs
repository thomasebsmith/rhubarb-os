type ProcessId = u64;

struct Process {
    id: ProcessId,

    // runner_id is None if this is the root process. Otherwise, runner_id the
    //  ID of the process that handles all system calls for this process. This
    //  allows one process to hook into another process (if given appropriate
    //  permissions). For example, the runner can create fake hardware events
    //  that appear to come from the user.
    runner_id: Option<ProcessId>,
}
