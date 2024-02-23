/*
A square of squares
You like building blocks. You especially like building blocks that are squares. And what you even like more, is to arrange them into a square of square building blocks!

However, sometimes, you can't arrange them into a square. Instead, you end up with an ordinary rectangle! Those blasted things! If you just had a way to know, whether you're currently working in vain… Wait! That's it! You just have to check if your number of building blocks is a perfect square.

Task
Given an integral number, determine if it's a square number:

In mathematics, a square number or perfect square is an integer that is the square of an integer; in other words, it is the product of some integer with itself.

The tests will always use some integral number, so don't worry about that in dynamic typed languages.

Examples
-1  =>  false
0  =>  true
3  =>  false
4  =>  true
25  =>  true
26  =>  false
*/

pub fn is_square(n: i64) -> bool {
    let root: i64 = (n as f64).sqrt().round() as i64;
    root * root == n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_tests() {
        assert_eq!(is_square(-1), false, "\nYour answer (left) is not the expected answer (right).");
        assert_eq!(is_square(0), true, "\nYour answer (left) is not the expected answer (right).");
        assert_eq!(is_square(3), false, "\nYour answer (left) is not the expected answer (right).");
        assert_eq!(is_square(4), true, "\nYour answer (left) is not the expected answer (right).");
        assert_eq!(is_square(25), true, "\nYour answer (left) is not the expected answer (right).");
        assert_eq!(is_square(26), false, "\nYour answer (left) is not the expected answer (right).");
    }
}
