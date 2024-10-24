// Import the mlfq module, which contains the MLFQ scheduler and Process structs
mod mlfq;

fn main() {
    // Initialize a new MLFQ scheduler with 3 levels and respective time slices of 2, 4, and 8 units
    let mut scheduler = mlfq::MLFQ::new(3, vec![2, 4, 8]);

    // Add processes to the scheduler with their IDs, priorities, and execution times
    let processes = vec![
        mlfq::Process { id: 1, priority: 0, remaining_time: 10, total_executed_time: 0 },
        mlfq::Process { id: 2, priority: 0, remaining_time: 3, total_executed_time: 0 },
        mlfq::Process { id: 3, priority: 1, remaining_time: 5, total_executed_time: 0 },
    ];

    for process in processes {
        scheduler.add_process(process);
    }

    // Process each queue in the scheduler
    for level in 0..scheduler.num_levels {
        while !scheduler.queues[level].is_empty() {
            scheduler.execute_process(level);
        }
    }

    // Simulate time passage and update the scheduler's state
    scheduler.update_time(100);

    // Print the current state of each queue in the scheduler
    for (level, queue) in scheduler.queues.iter().enumerate() {
        println!("Queue {}: {:?}", level, queue);
    }
}
