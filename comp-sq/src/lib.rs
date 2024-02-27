pub fn comp(a: Vec<i64>, b: Vec<i64>) -> bool {
    let mut a1: Vec<i64> = a.iter().map(|n| n.pow(2)).collect();
    let mut a2: Vec<i64> = b;

    a1.sort();
    a2.sort();

    a1 == a2
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(a: Vec<i64>, b: Vec<i64>, exp: bool) -> () {
        let result: &str = &format!("a = {:?}, b = {:?}", a, b)[..];
        assert_eq!(comp(a, b), exp, "{result}")
    }
    
    #[test]
    fn tests_comp() {
    
        let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
        let a2 = vec![11*11, 121*121, 144*144, 19*19, 161*161, 19*19, 144*144, 19*19];
        testing(a1, a2, true);
        let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
        let a2 = vec![11*21, 121*121, 144*144, 19*19, 161*161, 19*19, 144*144, 19*19];
        testing(a1, a2, false);
    
    }
}
