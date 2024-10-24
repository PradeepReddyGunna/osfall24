// Define the structure representing a process in the MLFQ scheduler
#[derive(Debug)]  // Automatically implement the Debug trait
pub struct Process {
    pub id: u32,                        // Unique identifier for the process
    pub priority: usize,                // Current queue index for the process
    pub remaining_time: u32,            // Time left for the process to complete execution
    pub total_executed_time: u32,       // Total time the process has been executed
}

// Define the MLFQ scheduler structure
pub struct MLFQ {
    pub queues: Vec<Vec<Process>>,      // Queues for each priority level (vector of vectors of processes)
    pub num_levels: usize,              // Number of priority levels in the scheduler
    time_quanta: Vec<u32>,              // Time slices for each queue level
    current_time: u32,                  // Tracks the current time in the scheduler
}

impl MLFQ {
    // Create a new MLFQ scheduler with a specified number of levels and their corresponding time quanta
    pub fn new(num_levels: usize, time_quanta: Vec<u32>) -> Self {
        // Initialize an empty queue for each level
        let queues = (0..num_levels).map(|_| Vec::new()).collect();
        
        MLFQ {
            queues,                      // Use the initialized queues
            num_levels,                   // Set the number of priority levels
            time_quanta,                  // Time slices for each level
            current_time: 0,              // Initialize the current time to zero
        }
    }

    // Add a process to the appropriate priority queue
    pub fn add_process(&mut self, process: Process) {
        let priority = process.priority;

        // Place the process in the corresponding queue if the priority is valid
        if priority < self.num_levels {
            self.queues[priority].push(process);
        } else {
            // If the priority exceeds available levels, place it in the lowest priority queue
            self.queues[self.num_levels - 1].push(process);
        }
    }

    // Execute the next process from the given queue (identified by queue_index)
    pub fn execute_process(&mut self, queue_index: usize) {
        // Attempt to fetch the next process from the specified queue
        if let Some(mut process) = self.queues[queue_index].pop() {
            let time_quantum = self.time_quanta[queue_index]; // Get the time quantum for this queue level
            
            // Determine how much time to allocate for execution
            let executed_time = if process.remaining_time > time_quantum {
                time_quantum // Use the time quantum if remaining time is greater
            } else {
                process.remaining_time // Otherwise, execute for the remaining time
            };

            // Update the process's execution details
            process.remaining_time -= executed_time;
            process.total_executed_time += executed_time;
            self.current_time += executed_time; // Update the scheduler's current time

            // Log the execution details for debugging
            println!(
                "Executed Process ID: {}, Time Executed: {}, Time Remaining: {}",
                process.id, executed_time, process.remaining_time
            );

            // If the process is not yet finished, demote it to a lower priority queue
            if process.remaining_time > 0 {
                if queue_index + 1 < self.num_levels {
                    process.priority += 1;  // Lower its priority (move it down the queue hierarchy)
                    self.queues[queue_index + 1].push(process); // Add the process to the next lower queue
                }
            }
            // If the process is completed, it is not re-added to any queue
        }
    }

    // Perform a priority boost, moving all processes from lower queues back to the highest priority queue
    pub fn priority_boost(&mut self) {
        // Iterate over all queues except the highest priority one
        for queue_index in 1..self.num_levels {
            // Move each process from the current queue back to the highest priority queue
            while let Some(mut process) = self.queues[queue_index].pop() {
                process.priority = 0; // Reset the process priority to the highest level
                self.queues[0].push(process); // Add it to the highest priority queue
            }
        }
    }

    // Update the scheduler's time and trigger a priority boost if necessary
    pub fn update_time(&mut self, elapsed_time: u32) {
        self.current_time += elapsed_time; // Increment the current time
        let boost_interval = 100;          // Define how often the priority boost occurs
        
        // If the current time has reached the boost interval, trigger a priority boost
        if self.current_time % boost_interval == 0 {
            self.priority_boost();
        }
    }
}

// Unit tests for the MLFQ scheduling system
#[cfg(test)]
mod tests {
    use super::*;

    // Test that adding processes to the MLFQ correctly assigns them to the appropriate queues
    #[test]
    fn test_add_process() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);

        // Create sample processes
        let process1 = Process { id: 1, priority: 0, remaining_time: 10, total_executed_time: 0 };
        let process2 = Process { id: 2, priority: 1, remaining_time: 5, total_executed_time: 0 };
        let process3 = Process { id: 3, priority: 5, remaining_time: 8, total_executed_time: 0 };

        // Add processes to the MLFQ
        mlfq.add_process(process1);
        mlfq.add_process(process2);
        mlfq.add_process(process3);

        // Check that processes were added to the correct queues
        assert_eq!(mlfq.queues[0].len(), 1);  // Process 1 should be in queue 0
        assert_eq!(mlfq.queues[1].len(), 1);  // Process 2 should be in queue 1
        assert_eq!(mlfq.queues[2].len(), 1);  // Process 3 should have been placed in the last queue (queue 2)
    }

    // Test that executing a process properly updates its state and moves it to the correct queue
    #[test]
    fn test_execute_process() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);

        // Add a process to the highest priority queue
        mlfq.queues[0].push(Process { id: 1, priority: 0, remaining_time: 5, total_executed_time: 0 });

        // Execute the process
        mlfq.execute_process(0);

        // Verify that the process has moved from queue 0 to queue 1
        assert_eq!(mlfq.queues[0].len(), 0);   // Queue 0 should now be empty
        assert_eq!(mlfq.queues[1].len(), 1);   // The process should have been moved to queue 1
        assert_eq!(mlfq.queues[1][0].remaining_time, 3);  // The process should have 3 units of remaining time
        assert_eq!(mlfq.queues[1][0].total_executed_time, 2);  // 2 units of time should have been executed
    }

    // Test that priority boost correctly moves processes to the highest priority queue
    #[test]
    fn test_priority_boost() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);

        // Add processes to lower priority queues
        mlfq.queues[1].push(Process { id: 1, priority: 1, remaining_time: 5, total_executed_time: 3 });
        mlfq.queues[2].push(Process { id: 2, priority: 2, remaining_time: 3, total_executed_time: 7 });

        // Simulate enough time to trigger a priority boost
        mlfq.update_time(100);

        // Verify that all processes have been boosted to the highest priority queue
        assert_eq!(mlfq.queues[0].len(), 2);  // Both processes should now be in queue 0
        assert_eq!(mlfq.queues[1].len(), 0);  // Queue 1 should be empty
        assert_eq!(mlfq.queues[2].len(), 0);  // Queue 2 should be empty
    }

    // Test that priority boost does not occur prematurely
    #[test]
    fn test_boost_does_not_occur_prematurely() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);

        // Add a process to a lower priority queue
        mlfq.queues[1].push(Process { id: 1, priority: 1, remaining_time: 5, total_executed_time: 3 });

        // Simulate time passing without hitting the boost interval
        mlfq.update_time(50);

        // Verify that no priority boost has occurred
        assert_eq!(mlfq.queues[1].len(), 1);  // The process should still be in queue 1
        assert_eq!(mlfq.queues[0].len(), 0);  // Queue 0 should still be empty
    }
}

