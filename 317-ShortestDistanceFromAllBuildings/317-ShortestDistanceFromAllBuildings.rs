// Last updated: 6/7/2025, 2:41:41 AM
use std::collections::VecDeque;
use std::i32::MAX;

impl Solution 
{
    pub fn shortest_distance(grid: Vec<Vec<i32>>) -> i32 
    {
        let (rows, columns) = (grid.len(), grid[0].len());
        let (mut houses, mut minDistance) = (0, MAX);

        // Criterion	                Empty-cell BFS (Original)	            House-based BFS (Optimized)
        // BFS Runs	                    O(#empty cells)	                        O(#houses)
        // BFS Target	                Reach all houses	                    Update all empty land
        // Needs totalHouses first?	    Yes	                                    No (used only at the end)
        // Early Pruning	            Yes, via marking unreachable 0	        No, relies on final cell check
        // Memory Use	                Low	                                    Requires 3D array ([r][c][2])
        // Better for...	            Sparse grids with few empty cells	    Grids with many empty cells

        // *Empty-cell BFS
        // Count total houses
        for r in 0..rows 
        {
            for c in 0..columns 
            {
                if grid[r][c] == 1 
                {
                    houses += 1;
                }
            }
        }

        for r in 0..rows 
        {
            for c in 0..columns 
            {
                if grid[r][c] == 0 
                {
                    let mut cpgrid =  grid.clone();
                    minDistance = minDistance.min(Self::bfs(&mut cpgrid, r, c, houses));
                }
            }
        }

        // *House-based BFS
        // let mut distances = vec![vec![[0; 2]; cols]; rows];

        // // Count total houses
        // for r in 0..rows 
        // {
        //     for c in 0..columns 
        //     {
        //         if grid[r][c] == 1 
        //         {
        //             total_houses += 1;
        //             bfs(grid, distances, r, c, house);
        //         }
        //     }
        // }

        // for r in 0..rows 
        // {
        //     for c in 0..columns 
        //     {
        //         if grid[r][c] == 0 
        //         {
        //             minDistance = min(minDistance, distances[row][col][0]);
        //         }
        //     }
        // }

        if (minDistance == MAX) { -1 }
        else { minDistance }
    }

    // *House-based BFS
    // fn bfs(grid: &mut Vec<Vec<i32>>, distances , row: usize, col: usize, totalHouses: i32) -> i32 
    fn bfs(grid: &mut Vec<Vec<i32>>, row: usize, col: usize, totalHouses: i32) -> i32 
    {
        let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        let (rows, columns) = (grid.len(), grid[0].len());
        let (mut distances, mut visitedHouses) = (0, 0);

        // BFS Queue 
        let mut queue = VecDeque::new();
        queue.push_back((row, col));

        let mut visited = vec![vec![false; columns]; rows];
        visited[row][col] = true;

        let mut steps = 0;
        while !queue.is_empty() && visitedHouses != totalHouses
        {
            for _ in 0..queue.len()
            {
                let (r, c) = queue.pop_front().unwrap();

                // *House-based BFS
                // if (grid[r][c] == 0) 
                // {
                //     distances[r][c][0] += steps;
                //     distances[r][c][1] += 1;
                // }

                if grid[r][c] == 1
                {
                    distances += steps;
                    visitedHouses += 1;
                    continue;
                }

                for (dr, dc) in &directions
                {
                    let nr = r as isize + dr;
                    let nc = c as isize + dc;

                    if nr >= 0 && nc >= 0 && (nr as usize) < rows && (nc as usize) < columns
                    {
                        let u_nr = nr as usize;
                        let u_nc = nc as usize;

                        if !visited[u_nr][u_nc] && grid[u_nr][u_nc] != 2
                        {
                            visited[u_nr][u_nc] = true;
                            queue.push_back((u_nr, u_nc));
                        }
                    }
                }
            }
            steps += 1;
        }

        // *Empty-cell BFS
        // Pruning Optimization:
        // If this BFS iteration did not reach all houses, then any cell visited also cannot reach all houses.
        // Set all cells visited to 2 so we do not check them again and return INT_MAX.
        if visitedHouses != totalHouses
        {
            for r in 0..rows
            {
                for c in 0..columns
                {
                    if grid[r][c] == 0 && visited[r][c] { grid[r][c] = 2; }
                }
            }
            return MAX;
        }

        distances
    }
}