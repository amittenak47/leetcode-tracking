// Last updated: 6/6/2025, 3:30:53 PM
impl Solution 
{
    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> Vec<i32> 
    {
        // 1. Build adjacency list for both trees
        // 2. DFS on the adjacency list to get count; It should return 0 if depth is exceeded

        fn dfs(i: usize, parent: i32, children: &Vec<Vec<i32>>, depth: usize, color: &mut Vec<usize>) -> i32 
        {
            color[i] = depth % 2;
            let mut res = 1 - (depth % 2) as i32;
            
            for &c in &children[i] 
            {
                if c == parent { continue; }
                res += dfs(c as usize, i as i32, children, depth+1, color);
            }
            
            res
        }

        fn build(edges: &Vec<Vec<i32>>, color: &mut Vec<usize>) -> Vec<i32> 
        {
            let MN = edges.len() + 1;
            let mut children = vec![vec![]; MN];
            for edge in edges 
            {
                let (u, v) = (edge[0] as usize, edge[1] as usize);
                children[u].push(v as i32);
                children[v].push(u as i32);
            }
            
            let mut res = dfs(0, -1, &children, 0, color);
            vec![res, (MN  as i32) - res]
        }

        let (M, N) = (edges1.len() + 1, edges2.len() + 1);
        let (mut c1, mut c2) = (vec![0; M], vec![0; N]);
        let (tree1, tree2) = (build(&edges1, &mut c1), build(&edges2, &mut c2));

        let max_tree2 = tree2[0].max(tree2[1]);
        let mut res = vec![0; M];
        for i in 0..M { res[i] = tree1[c1[i]] + max_tree2; }
        res

    }
}