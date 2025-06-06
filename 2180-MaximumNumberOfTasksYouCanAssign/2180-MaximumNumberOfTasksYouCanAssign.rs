// Last updated: 6/6/2025, 3:31:20 PM
use std::
{
    collections::BTreeMap,
    collections::VecDeque,
    cmp::{max, min},
};


impl Solution 
{
    pub fn max_task_assign(tasks: Vec<i32>, workers: Vec<i32>, pills: i32, strength: i32) -> i32 
    {
        let (mut tasks, mut workers) = (tasks, workers);
        let (m, n, mut l, mut r, mut res) = (tasks.len(), workers.len(), 1, workers.len().min(tasks.len()), 0);

        tasks.sort();
        workers.sort();

        // Binary Search + BTree
        while l <= r 
        {
            let m = (l + r) / 2;
            if Self::check(&tasks, &workers, pills, strength, m, true) 
            {
                res = m as i32;
                l = m + 1;
            } 
            else { r = m - 1; }
        }
        res as i32
    }

    fn check(tasks: &[i32], workers: &[i32], pills: i32, strength: i32, mid: usize, deque: bool) -> bool 
    {
        if deque
        {
            let (mut m, mut n, mut p, mut ws, mut ptr) = (tasks.len(), workers.len(), pills, VecDeque::new(), workers.len() - 1);

            // Enumerate each task from largest to smallest
            for i in (0..mid).rev() 
            {
                // Add all *fit-for-the-task* workers [mid, workers.len()]
                // fit-for-the-task: Workers + Pill ; ie. workers[ptr] + strength
                while ptr as i32 >= (n - mid) as i32 && workers[ptr] + strength >= tasks[i] 
                {
                    ws.push_front(workers[ptr]);
                    ptr -= 1;
                }
                if ws.is_empty() { return false; }
                
                // ws.back() :: max element ; Assign strongest worker to hardest task
                if *ws.back().unwrap() >= tasks[i] { ws.pop_back(); }
                
                // Assign weakest worker with pill to hardest task
                else 
                {
                    if p == 0 { return false; }
                    p -= 1;
                    ws.pop_front();
                }
            }
            true
        }
        else
        {
            let (mut T, mut bb) = (pills, BTreeMap::new());
            
            // Insert all workers up to midpoint into BTree
            for &w in workers.iter().skip(workers.len() - mid) { *bb.entry(w).or_insert(0) += 1; }

            // Iterate in descending order for hardest task
            for &t in tasks.iter().take(mid).rev() 
            {
                // next_back() :: (max) strength worker ; 
                if let Some((&arnold, _)) = bb.iter().next_back() 
                {
                    // Assign task to strongest worker and remove 
                    if arnold >= t 
                    {
                        *bb.get_mut(&arnold).unwrap() -= 1;
                        if bb[&arnold] == 0 { bb.remove(&arnold); }
                    }

                    // No natural workers 
                    else 
                    {
                        // No Pill --> False
                        if T == 0 { return false; }

                        // Assign pill to worker (t - strength)
                        if let Some((&key, _)) = bb.range(t - strength..).next() 
                        {
                            *bb.get_mut(&key).unwrap() -= 1;
                            if bb[&key] == 0 { bb.remove(&key); }
                            T -= 1;
                        } 

                        // No workers even with pill --> False
                        else { return false; }
                    }
                }
            }
            true
        }
    }
}