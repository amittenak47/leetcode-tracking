// Last updated: 6/7/2025, 2:41:45 AM
impl Solution 
{
    // fn dfs(image: &mut Vec<Vec<char>>, x: i32, y: i32, top: &mut i32, bottom: &mut i32, left: &mut i32, right: &mut i32) 
    // {
    //     let (rows, cols) = (image.len() as i32, image[0].len() as i32);
    //     if x < 0 || y < 0 || x >= rows || y >= cols { return; }

    //     let (ux, uy) = (x as usize, y as usize);
    //     if image[ux][uy] == '0' { return; }

    //     image[ux][uy] = '0'; // mark visited

    //     *top = (*top).min(x);
    //     *bottom = (*bottom).max(x + 1);
    //     *left = (*left).min(y);
    //     *right = (*right).max(y + 1);

    //     Self::dfs(image, x + 1, y, top, bottom, left, right);
    //     Self::dfs(image, x - 1, y, top, bottom, left, right);
    //     Self::dfs(image, x, y - 1, top, bottom, left, right);
    //     Self::dfs(image, x, y + 1, top, bottom, left, right);
    // }

    // pub fn min_area(image: Vec<Vec<char>>, x: i32, y: i32) -> i32 
    // {
    //     // DFS:
    //     // O(edges) = O(black = O(m * n)
    //     if image.is_empty() || image[0].is_empty() { return 0; }

    //     let mut img = image.clone();
    //     let (mut top, mut bottom, mut left, mut right) = (x, x + 1, y, y + 1);

    //     Self::dfs(&mut img, x, y, &mut top, &mut bottom, &mut left, &mut right);

    //     (right - left) * (bottom - top)
    // }

    // Binary search bounds
    // O(m searches * log m + n searches * log n)
    fn min_area(image: Vec<Vec<char>>, x: i32, y: i32) -> i32
    {
        let (mut m, mut n) = (image.len() as i32, image[0].len() as i32);

        let mut left = Self::searchColumns(&image, 0, y, true);
        let mut right = Self::searchColumns(&image, y + 1, n, false);

        let mut top = Self::searchRows(&image, 0, x, true);
        let mut bottom = Self::searchRows(&image, x + 1, m, false);
        
        (right - left) * (bottom - top)
    }
    
    fn searchColumns(image: &Vec<Vec<char>>, i: i32, j: i32, black: bool) -> i32
    {
        let (mut x, mut y) = (i, j);
        while (x != y) 
        {
            let mut mid = (x + y) / 2;
            let jack = image.iter().any(|row| row[mid as usize] == '1'); // search entire "middle" column for a black

            if (jack == black) { y = mid; } // reduce bound to lower half
            else { x = mid + 1; } // increase bound to upper half
        }
        x
    }
    
    fn searchRows(image: &Vec<Vec<char>>, i: i32, j: i32, black: bool) -> i32
    {
        let (mut x, mut y) = (i, j);
        while (x != y) 
        {
            let mut mid = (x + y) / 2;
            let jack = image[mid as usize].iter().any(|&col| col == '1'); // search entire "middle" row for a black

            if (jack == black) { y = mid; } // reduce bound to lower half
            else { x = mid + 1; } // increase bound to upper half
        }
        x
    }
}
