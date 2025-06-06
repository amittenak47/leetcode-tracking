// Last updated: 6/6/2025, 3:31:33 PM
use itertools::Itertools;

impl Solution 
{
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32
    {
        // let mut num = vec![0; 100];
        // let mut ret = 0;
        // for i in dominoes
        // {
        //     let mut val = if (i[0] < i[1]) { (i[0] * 10 + i[1]) as usize } else { (i[1] * 10 + i[0]) as usize };                // concatenate each binary pair into a two-digit positive integer: (x,y) = 10x + y
        //     ret += num[val];
        //     num[val] += 1;                                                                                                      // update ret with cached value (number of equivalent pairs)
        // }
        // ret

        dominoes
            .into_iter()
            .map(|d| [d[0].min(d[1]), d[0].max(d[1])])
            // .fold(std::collections::HashMap::new(), |mut map, d| 
            // {
            //     *map.entry(d).or_default() += 1;
            //     map
            // })
            .counts()
            .into_values()
            .map(|c: usize| { let c = c as i32; (c * (c - 1)) >> 1 })
            .sum()
    }
}