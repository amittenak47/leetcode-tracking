// Last updated: 6/6/2025, 3:32:05 PM
use std::collections::{HashMap, VecDeque, HashSet};

struct Logger 
{
    // HashMap: O(1) / O(M) no deletion method
    // mmap: HashMap<String, i32>,

    // Queue + Set: O(N) / [O(1), O(N)]
    // O((k + k + 1)/(k + 1)) = O((2k + 1)/(k + 1)) = O(1 + k/(k + 1)) < O(2) = O(1) 
    mq: VecDeque<(String, i32)>,
    mset: HashSet<String>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Logger 
{
    fn new() -> Self 
    {
        // Logger { mmap: HashMap::new(), }
        Logger { mq: VecDeque::new(), mset: HashSet::new(), }
    }
    
    fn should_print_message(&mut self, timestamp: i32, message: String) -> bool 
    {
        // Store each message in the HashMap if it is unique, or the timestamp difference exceeds 10s
        // if let Some(pts) = self.mmap.get(&message)
        // {
        //     if (timestamp - *pts) >= 10 { self.mmap.insert(message, timestamp); true }
        //     else { false }
        // }
        // else { self.mmap.insert(message, timestamp); true }

        // 1. Check front of the queue for the oldest message and compare timestamp before removal from queue and set
        // 2. Add message to queue / set if it is not in set (either removed previously or a new message)
        while let Some(front) = self.mq.front()
        {
            if (timestamp - front.1) >= 10 
            {
                if let Some(del) = self.mq.pop_front() { self.mset.remove(&del.0); }
            } 
            else { break; } 
        }

        if !self.mset.contains(&message)
        {
            self.mq.push_back((message.clone(), timestamp));
            self.mset.insert(message);
            true
        }
        else { false }
    }
}

/**
 * Your Logger object will be instantiated and called as such:
 * let obj = Logger::new();
 * let ret_1: bool = obj.should_print_message(timestamp, message);
 */
