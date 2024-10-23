mod mlfq;

fn main() {
    // Create a new MLFQ scheduler with 3 levels and specified time quanta
    let mut scheduler = mlfq::MLFQ::new(3, vec![2, 4, 8]);

    // Define and add processes to the scheduler
    let processes = vec![
        mlfq::Process { id: 1, priority: 0, remaining_time: 10, total_executed_time: 0 },
        mlfq::Process { id: 2, priority: 0, remaining_time: 3, total_executed_time: 0 },
        mlfq::Process { id: 3, priority: 1, remaining_time: 5, total_executed_time: 0 },
    ];
    
    for process in processes {
        scheduler.add_process(process);
    }

    // Execute all processes in the highest priority queue
    for queue_index in 0..scheduler.num_levels {
        while !scheduler.queues[queue_index].is_empty() {
            scheduler.execute_process(queue_index);
        }
    }

    // Simulate the passage of time and trigger a priority boost if necessary
    scheduler.update_time(100);

    // Print the state of the queues after the boost
    for (i, queue) in scheduler.queues.iter().enumerate() {
        println!("Queue {}: {:?}", i, queue);
    }
}
