// Last updated: 6/6/2025, 3:30:57 PM
impl Solution 
{
    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>, k: i32) -> Vec<i32> 
    {
        fn dfs(i: usize, parent: i32, children: &Vec<Vec<i32>>, k: i32) -> i32 
        {
            if k < 0 { return 0; }
            
            let mut res = 1;
            for &c in &children[i] 
            {
                if c == parent { continue; }
                res += dfs(c as usize, i as i32, children, k - 1);
            }
            res
        }

        fn build(edges: Vec<Vec<i32>>, k: i32) -> Vec<i32> 
        {
            let N = edges.len() + 1;
            let mut children = vec![vec![]; N];
            for edge in edges 
            {
                let (u, v) = (edge[0] as usize, edge[1] as usize);
                children[u].push(v as i32);
                children[v].push(u as i32);
            }
            
            let mut res = vec![0; N];
            for i in 0..N { res[i] = dfs(i, -1, &children, k); }
            res
        }

        let N = edges1.len() + 1;
        let tree1 = build(edges1, k);
        let tree2 = build(edges2, k - 1);
        let max_tree2 = *tree2.iter().max().unwrap();
        let mut res = vec![0; N];
        for i in 0..N { res[i] = tree1[i] + max_tree2; }
        res
    }
}