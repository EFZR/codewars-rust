// Given an n x n array, return the array elements arranged from outermost elements to the middle element, traveling clockwise.
// array    =   [[1,2,3],
//              [4,5,6],
//              [7,8,9]]
// snail(array) #=> [1,2,3,6,9,8,7,4,5]

pub fn snail(matrix: &[Vec<i32>]) -> Vec<i32> {
    if matrix.len() == 1 as usize {
        return matrix[0].iter().copied().collect();
    }

    let mut matrix: Vec<Vec<i32>> = matrix.iter().map(|f| f.to_vec()).collect();
    let mut res: Vec<i32> = Vec::new();

    while matrix.len() > 0 {
        res.extend(matrix.remove(0));

        if matrix.len() == 0 {
            break;
        }

        for i in 0..matrix.len() {
            let n = matrix.len();
            res.push(matrix[i].remove(n));
        }

        res.extend(matrix.remove(matrix.len() - 1).iter().rev());

        for i in (0..matrix.len()).rev() {
            res.push(matrix[i].remove(0));
        }
    }

    res
}

// clever solution ->

fn one_layer(n: usize, layer: usize) -> impl Iterator<Item = (usize, usize)> {
    let left_right = (layer..n - layer).map(move |i| (layer, i));
    let up_down = (layer + 1..n - layer).map(move |i| (i, n - 1 - layer));
    let right_left = (layer..n - 1 - layer)
        .rev()
        .map(move |i| (n - 1 - layer, i));
    let down_up = (layer + 1..n - 1 - layer).rev().map(move |i| (i, layer));

    left_right.chain(up_down).chain(right_left).chain(down_up)
}

pub fn _snail(matrix: &[Vec<i32>]) -> Vec<i32> {
    let n = matrix.get(0).map(|xs| xs.len()).unwrap_or(0);
    (0..n)
        .flat_map(|x| one_layer(n, x))
        .map(|(x, y)| matrix[x][y])
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test1() {
        let square = &[vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = vec![1, 2, 3, 6, 9, 8, 7, 4, 5];
        assert_eq!(_snail(square), expected);
    }

    #[test]
    fn sample_test2() {
        let square = &[vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]];
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(_snail(square), expected);
    }

    #[test]
    fn sample_test3() {
        let square: &[Vec<i32>; 1] = &[Vec::new()];
        let expected = Vec::new();
        assert_eq!(_snail(square), expected, "Failed with empty input");
    }

    #[test]
    fn sample_test4() {
        let square = &[vec![1]];
        let expected = vec![1];
        assert_eq!(_snail(square), expected);
    }
}
