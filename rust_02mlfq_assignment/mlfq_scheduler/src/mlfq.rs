#[derive(Debug, Clone)] // Enable debugging for the Process struct
pub struct Process {
    pub id: u32,
    pub priority: usize,  // Current queue index
    pub remaining_time: u32,
    pub total_executed_time: u32,
}

pub struct MLFQ {
    pub queues: Vec<Vec<Process>>,  // Public access to queues
    pub num_levels: usize,           // Public access to number of levels
    time_quanta: Vec<u32>,
    current_time: u32,
}

impl MLFQ {
    // Initialize a new MLFQ scheduler
    pub fn new(num_levels: usize, time_quanta: Vec<u32>) -> Self {
        MLFQ {
            queues: vec![Vec::new(); num_levels],
            num_levels,
            time_quanta,
            current_time: 0,
        }
    }

    // Add a new process to the appropriate queue
    pub fn add_process(&mut self, process: Process) {
        let priority = process.priority;
        if priority < self.num_levels {
            self.queues[priority].push(process);
        } else {
            self.queues[self.num_levels - 1].push(process); // Add to lowest priority queue
        }
    }

    // Execute the next process in the specified queue
    pub fn execute_process(&mut self, queue_index: usize) {
        if let Some(mut process) = self.queues[queue_index].pop() {
            let time_quantum = self.time_quanta[queue_index];
            let executed_time = if process.remaining_time > time_quantum {
                time_quantum
            } else {
                process.remaining_time
            };

            // Update process times
            process.remaining_time -= executed_time;
            process.total_executed_time += executed_time;
            self.current_time += executed_time;

            println!("Executed Process ID: {}, Time Executed: {}, Remaining Time: {}", 
                     process.id, executed_time, process.remaining_time);

            // If the process is not finished, move it to a lower priority queue
            if process.remaining_time > 0 && queue_index + 1 < self.num_levels {
                process.priority += 1;
                self.queues[queue_index + 1].push(process);
            }
            // If completed, the process won't be added back
        }
    }

    // Boost the priority of processes in lower queues
    pub fn priority_boost(&mut self) {
        for queue in 1..self.num_levels {
            while let Some(mut process) = self.queues[queue].pop() {
                process.priority = 0; // Reset priority to the highest
                self.queues[0].push(process);
            }
        }
    }

    // Simulate time passing and check for priority boosts
    pub fn update_time(&mut self, elapsed_time: u32) {
        self.current_time += elapsed_time;
        if self.current_time % 100 == 0 {
            self.priority_boost(); // Trigger a boost at specific intervals
        }
    }
}

// Automated Test Cases
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_process() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);

        let process1 = Process { id: 1, priority: 0, remaining_time: 10, total_executed_time: 0 };
        let process2 = Process { id: 2, priority: 1, remaining_time: 5, total_executed_time: 0 };
        let process3 = Process { id: 3, priority: 5, remaining_time: 8, total_executed_time: 0 };

        mlfq.add_process(process1);
        mlfq.add_process(process2);
        mlfq.add_process(process3);

        assert_eq!(mlfq.queues[0].len(), 1);
        assert_eq!(mlfq.queues[1].len(), 1);
        assert_eq!(mlfq.queues[2].len(), 1);
    }

    #[test]
    fn test_execute_process() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        mlfq.queues[0].push(Process { id: 1, priority: 0, remaining_time: 5, total_executed_time: 0 });

        mlfq.execute_process(0);

        assert_eq!(mlfq.queues[0].len(), 0);
        assert_eq!(mlfq.queues[1].len(), 1);
        assert_eq!(mlfq.queues[1][0].remaining_time, 3);
        assert_eq!(mlfq.queues[1][0].total_executed_time, 2);
    }

    #[test]
    fn test_priority_boost() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        mlfq.queues[1].push(Process { id: 1, priority: 1, remaining_time: 5, total_executed_time: 3 });
        mlfq.queues[2].push(Process { id: 2, priority: 2, remaining_time: 3, total_executed_time: 7 });

        mlfq.update_time(100); // Should trigger priority boost

        assert_eq!(mlfq.queues[0].len(), 2);
        assert_eq!(mlfq.queues[1].len(), 0);
        assert_eq!(mlfq.queues[2].len(), 0);
    }

    #[test]
    fn test_boost_does_not_occur_prematurely() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        mlfq.queues[1].push(Process { id: 1, priority: 1, remaining_time: 5, total_executed_time: 3 });
        
        mlfq.update_time(50); // No boost should happen

        assert_eq!(mlfq.queues[1].len(), 1);
        assert_eq!(mlfq.queues[0].len(), 0);
    }
}
