// Last updated: 6/6/2025, 3:31:19 PM
impl Solution 
{
    pub fn max_distance(colors: Vec<i32>) -> i32 
    {
        // let mut seen = [false; 101];
        // let mut max_diff = 0;
        // for i in 0..colors.len() {
        //     match seen.get_mut(colors[i] as usize).filter(|c| !(**c)) {
        //         Some(c) => *c = true,
        //         None => continue,
        //     }
        //     if let Some(nc) = colors[i + 1..].iter().rposition(|&nc| nc != colors[i]) {
        //         max_diff = max_diff.max(nc + 1);
        //     }
        // }
        // max_diff as _

        let (mut N, mut i, mut j) = (colors.len(), 0, colors.len() - 1);
        while (colors[0] == colors[j]) { j -= 1; } 
        while (colors[N-1] == colors[i]) { i += 1; }
        return std::cmp::max(((N as i32) - 1) - (i as i32), j as i32);
    }
}