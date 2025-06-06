// Last updated: 6/6/2025, 3:31:26 PM
impl Solution 
{
    pub fn find_lucky(mut arr: Vec<i32>) -> i32 
    {
        // constraints: 0 <= x <= 500
        arr.into_iter()
            .fold([0; 500], |mut f, num| 
            {
                f[num as usize - 1] += 1;
                f
            })
            .into_iter()
            .enumerate()
            .rfind(|(i, v)| *v - *i as i32 == 1)
            .map_or(-1, | (_, v) | v)
    }
}