mod mlfq;

fn main() {
    let mut scheduler = mlfq::MLFQ::new(3, vec![2, 4, 8]);


    scheduler.add_process(mlfq::Process { id: 1, priority: 0, remaining_time: 10, total_executed_time: 0 });
    scheduler.add_process(mlfq::Process { id: 2, priority: 0, remaining_time: 3, total_executed_time: 0 });
    scheduler.add_process(mlfq::Process { id: 3, priority: 1, remaining_time: 5, total_executed_time: 0 });

    for queue_index in 0..scheduler.num_levels {
        while !scheduler.queues[queue_index].is_empty() {
            scheduler.execute_process(queue_index);
        }
    }

    scheduler.update_time(100); 

    for (i, queue) in scheduler.queues.iter().enumerate() {
        println!("Queue {}: {:?}", i, queue);
    }
}

