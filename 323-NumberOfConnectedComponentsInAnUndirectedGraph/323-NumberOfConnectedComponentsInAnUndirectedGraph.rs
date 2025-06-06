// Last updated: 6/6/2025, 3:32:12 PM
impl Solution {

    // DFS Method: 
    // O(edges + nodes): O(edges) adjacency list loop + O(nodes) visit loop
    // O(edges + nodes): O(edges) adjacency list + O(nodes) visited + O(nodes) DFS runtime stack 

    // pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 
    // {
    //     // If there are 0 nodes, there are 0 components
    //     if (n == 0) { return 0; }

    //     let n = n as usize;
    //     let (mut components, mut adjacency_list, mut visited) = (0, vec![vec![]; n], vec![false; n]);

    //     // Build adjacency list for all adjacent edge connections
    //     for i in 0..edges.len()
    //     {
    //         let (u, v) = (edges[i][0] as usize, edges[i][1] as usize);
    //         adjacency_list[u].push(v);
    //         adjacency_list[v].push(u);
    //     }

    //     // Visit each new node to count number of components
    //     for i in 0..n 
    //     {
    //         if !visited[i] 
    //         {
    //             components += 1;
    //             Self::dfs(i, &adjacency_list, &mut visited);
    //         }
    //     }

    //     components
    // }

    // fn dfs(node: usize, graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>) 
    // {
    //     visited[node] = true;

    //     for &neighbor in &graph[node]
    //     {
    //         if !visited[neighbor] { Self::dfs(neighbor, graph, visited) }
    //     }
    // }

    // Disjoint Set Union (DSU) Method: 
    // O(edges + nodes): O(edges) adjacency list loop + O(nodes) visit loop
    // O(edges + nodes): O(edges) adjacency list + O(nodes) visited + O(nodes) DFS runtime stack 

    fn find(rep: &mut Vec<usize>, v: usize) -> usize
    {
        if (v == rep[v]) { return v; }
        
        rep[v] = Self::find(rep, rep[v]);
        rep[v]
    }
    
    fn combine(rep: &mut Vec<usize>, size: &mut Vec<i32>, mut v1: usize, mut v2: usize) -> i32
    {
        // println!("\nroots = {}, {}", v1, v2);
        v1 = Self::find(rep, v1);
        v2 = Self::find(rep, v2);
        // println!("\ncombine({}, {})", v1, v2);

        if (v1 == v2) { return 0; } 

        if (size[v1] > size[v2]) 
        {
            size[v1] += size[v2];
            rep[v2] = v1;
        } 
        else 
        {
            size[v2] += size[v1];
            rep[v1] = v2;
        }

        // println!("  => New rep: {:?}", rep);
        // println!("  => New sizes: {:?}", size);

        1
    }

    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 
    {
        // If there are 0 nodes, there are 0 components
        if (n <= 0) { return 0; }

        let n = n as usize;
        let (mut components, mut rep, mut size) = (n as i32, (0..n).collect::<Vec<usize>>(), vec![1; n]);

        for i in 0..n{ rep[i] = i; }
        
        for edge in edges.iter()
        {
            let (u, v) = (edge[0] as usize, edge[1] as usize);
            components -= Self::combine(&mut rep, &mut size, u, v);
        }

        components
    }
}