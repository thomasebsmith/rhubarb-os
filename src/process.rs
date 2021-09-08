type ProcessId = u64;

struct Process {
    id: ProcessId,
    runner_id: Option<ProcessId>,
}
