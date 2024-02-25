/*
Your task is to construct a building which will be a pile of n cubes. The cube at the bottom will have a volume of n pow 3,
the cube above will have (n - 1) pow 3 volume of and so on until the top which will have a volume of 1 pow 3.

You are given the total volume m of the building. Being given m can you find the number n of cubes you will have to build?

The parameter of the function find_nb (find_nb, find-nb, findNb, ...) will be an integer m and you have to return the integer n
if such a n exists or -1 if there is no such n.
*/

pub fn find_nb(m: u64) -> i32 {
    let mut cache: u64 = 0;
    let mut counter: u64 = 0;

    while cache < m {
        counter += 1;
        cache += counter.pow(3);
    }

    if cache == m {
        counter as i32
    } else {
        -1
    }
}

// Better solution ->
fn _find_nb(n: u64) -> i32 {
    let mut sum = 0_u64;
    let l = (0_u64..)
        .take_while(|&x| {
            sum += x.pow(3);
            sum < n
        })
        .count() as i32;
    if sum == n {
        l
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::find_nb;

    fn do_test(n: u64, exp: i32) {
        assert_eq!(
            find_nb(n),
            exp,
            "\nYour result (left) did not match expected output (right) for n={n}"
        );
    }

    #[test]
    fn basics_find_nb() {
        let cases = [
            (4, -1),
            (16, -1),
            (4183059834009, 2022),
            (24723578342962, -1),
            (135440716410000, 4824),
            (40539911473216, 3568),
            (26825883955641, 3218),
        ];
        for (n, expected) in cases {
            do_test(n, expected);
        }
    }
}
