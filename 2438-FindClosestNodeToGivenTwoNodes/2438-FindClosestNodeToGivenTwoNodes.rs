// Last updated: 6/6/2025, 3:31:12 PM
impl Solution 
{
    pub fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 
    // {
    //     // BFS: O(N) / O(N)
    //     // 1. "Graph", not tree, with <= 1 connection
    //     // 2. Create queue, add node1/node2, and visit each neighbor connection using 'visited' vec; Note 'distance' array is initialized with i32::MAX and starting node set to 0, and is built iteratively while processing q
    //     // 3. For each node visited, update the distance[neighbor] = 1 + distance[start_node]
    //     // 4. Iterate over N nodes, minimizing the distance (minD) against max distance (maxD; initialized to i32::MAX)
    //     fn bfs(start_node: i32, edges: &Vec<i32>, distance: &mut Vec<i32>) 
    //     {
    //         let N = edges.len();
    //         let mut q = std::collections::VecDeque::new();
    //         q.push_back(start_node);

    //         let mut visited = vec![false; N];
    //         distance[start_node as usize] = 0;                                               // note zeroing the start position

    //         while let Some(node) = q.pop_front() 
    //         {
    //             if visited[node as usize] { continue; }

    //             visited[node as usize] = true;
    //             let neighbor = edges[node as usize];
    //             if neighbor != -1 && !visited[neighbor as usize] 
    //             {
    //                 distance[neighbor as usize] = 1 + distance[node as usize];
    //                 q.push_back(neighbor);                                                   // note queue for BFS
    //             }
    //         }
    //     }

    //     let N = edges.len();
    //     let (mut ND1, mut ND2) = (vec![i32::MAX; N], vec![i32::MAX; N]);                     // Key: Iteratively minimize calculated BFS/DFS distance against the i32::MAX (node without edges)

    //     bfs(node1, &edges, &mut ND1);
    //     bfs(node2, &edges, &mut ND2);

    //     let (mut minD, mut minD_node) = (i32::MAX, -1);                                      // Key: Iteratively minimize calculated BFS/DFS distance against the i32::MAX (node without edges)
    //     for i in 0..N 
    //     {
    //         let maxD = std::cmp::max(ND1[i], ND2[i]);
    //         if maxD < minD 
    //         {
    //             minD = maxD;
    //             minD_node = i as i32;
    //         }
    //     }
    //     minD_node
    // }

    {
        // DFS: O(N) / O(N)
        // 1. "Graph", not tree, with <= 1 connection
        // 2. Create dfs function with 'dist', 'visited' as arguments, and recursively visit each neighbor connection using 'visited' vec; Note 'distance' array is initialized with i32::MAX and starting node set to 0, and is built iteratively while processing q
        // 3. For each node visited, update the distance[neighbor] = 1 + distance[start_node]
        // 4. Iterate over N nodes, minimizing the distance (minD) against max distance (maxD; initialized to i32::MAX)
        fn dfs(node: i32, edges: &Vec<i32>, distance: &mut Vec<i32>, visited: &mut Vec<bool>) 
        {
            visited[node as usize] = true;
            let neighbor = edges[node as usize];
            if neighbor != -1 && !visited[neighbor as usize] 
            {
                distance[neighbor as usize] = 1 + distance[node as usize];
                dfs(neighbor, edges, distance, visited);                                    // note recursion for DFS
            }
        }

        let N = edges.len();
        let (mut ND1, mut ND2) = (vec![i32::MAX; N], vec![i32::MAX; N]);                    // Key: Initialize vec with i32::MAX to represent nodes without edges
        let (mut v1, mut v2) = (vec![false; N], vec![false; N]);

        ND1[node1 as usize] = 0;
        ND2[node2 as usize] = 0;

        dfs(node1, &edges, &mut ND1, &mut v1);
        dfs(node2, &edges, &mut ND2, &mut v2);

        println!("{:?}, {:?}", ND1, ND2);

        let (mut minD, mut minD_node) = (i32::MAX, -1);                                     // Key: Iteratively minimize calculated BFS/DFS distance against the i32::MAX (node without edges)
        for i in 0..N 
        {
            let maxD = std::cmp::max(ND1[i], ND2[i]);
            if maxD < minD 
            {
                minD = maxD;
                minD_node = i as i32;
            }
        }
        minD_node
    }
}