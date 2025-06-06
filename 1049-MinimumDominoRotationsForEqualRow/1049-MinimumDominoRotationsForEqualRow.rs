// Last updated: 6/6/2025, 3:31:36 PM
use std::cmp;

impl Solution 
{
    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 
    {
        // 1. Start with first item, iterate through list, checking if item exists in either list
        //      - If item doesn't exist, then return -1
        // 2. Track top / bottom rotations separately, as the minimum of the two is soln

        // let N = tops.len();
        // let (a, b) = (tops[0], bottoms[0]);
        // let (mut checkA, mut checkB) = (true, true);
        // let (mut abot, mut atop, mut bbot, mut btop) = (0, 0, 0, 0);
        // for i in 0..N
        // {
        //     if checkA 
        //     {
        //         if (tops[i] != a && bottoms[i] != a) { checkA = false; }
        //         // else if (bottoms[i] ^ tops[i] == 0) { continue; }
        //         else if (bottoms[i] != a) { abot += 1; }
        //         else if (tops[i] != a) { atop += 1; }
        //     } 
        //     if checkB
        //     {
        //         if (tops[i] != b && bottoms[i] != b) { checkB = false; }
        //         // else if (bottoms[i] ^ tops[i] == 0) { continue; }
        //         else if (tops[i] != b) { btop += 1; }
        //         else if (bottoms[i] != b) { bbot += 1; }
        //     }
        //     if (!checkB) && (!checkA) { return -1; }
        // }

        // if checkA { cmp::min(abot, atop) } else { cmp::min(bbot, btop) }

        tops.iter()
            .copied()
            .zip(bottoms)
            .fold([[0; 3]; 7], |mut acc, (t, b)| {
                acc[t as usize][0] += 1;
                acc[b as usize][1] += 1;
                if t == b { acc[t as usize][2] += 1; }
                acc
            })
            .into_iter()
            .find(|[t, b, s]| t + b - s == tops.len() as i32)
            .map_or(-1, |[t, b, s]| t.min(b) - s)
    }
}