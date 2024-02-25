//! Well met with Fibonacci bigger brother, AKA Tribonacci.
//! As the name may already reveal, it works basically like a Fibonacci,
//! but summing the last 3 (instead of 2) numbers of the sequence to generate the next.
//! And, worse part of it, regrettably I won't get to hear non-native Italian speakers trying to pronounce it :(
//!
//! So, if we are to start our Tribonacci sequence with [1, 1, 1] as a starting input (AKA signature), we have this sequence:
//!
//! `[1, 1 ,1, 3, 5, 9, 17, 31, ...]`
//!
//! But what if we started with [0, 0, 1] as a signature?
//! As starting with [0, 1] instead of [1, 1] basically shifts the common Fibonacci sequence by once place,
//! you may be tempted to think that we would get the same sequence shifted by 2 places, but that is not the case and we would get:
//!
//! [0, 0, 1, 1, 2, 4, 7, 13, 24, ...]
//! Well, you may have guessed it by now, but to be clear: you need to create a fibonacci function that given a signature array/list,
//! returns the first n elements - signature included of the so seeded sequence.
//!
//! Signature will always contain 3 numbers; n will always be a non-negative number; if n == 0,
//! then return an empty array (except in C return NULL) and be ready for anything else which is not clearly specified ;)
//!
//! If you enjoyed this kata more advanced and generalized version of it can be found in the Xbonacci kata

pub fn tribonacci(signature: &[f64], n: usize) -> Vec<f64> {
    if n == 0 {
        return vec![];
    }

    if n <= 3 {
        return signature[..n].to_vec();
    }

    let mut result: Vec<f64> = signature.to_vec();

    for i in 0..n - signature.len() {
        let temp: &[f64] = &result[i..i + 3];
        result.push(temp.iter().sum());
    }

    result
}

// Better solution ->
fn _tribonacci(signature: &[f64], n: usize) -> Vec<f64> {
    let mut cache = signature.to_vec();
    
    cache.resize(n, 0.0);
    
    for i in 3..cache.len() {
        cache[i] = cache[i - 1] + cache[i - 2] + cache[i - 3];
    }
    
    cache
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(
            tribonacci(&[0., 1., 1.], 10),
            vec![0., 1., 1., 2., 4., 7., 13., 24., 44., 81.]
        );
        assert_eq!(
            tribonacci(&[1., 0., 0.], 10),
            vec![1., 0., 0., 1., 1., 2., 4., 7., 13., 24.]
        );
        assert_eq!(
            tribonacci(&[0., 0., 0.], 10),
            vec![0., 0., 0., 0., 0., 0., 0., 0., 0., 0.]
        );
        assert_eq!(
            tribonacci(&[1., 2., 3.], 10),
            vec![1., 2., 3., 6., 11., 20., 37., 68., 125., 230.]
        );
        assert_eq!(
            tribonacci(&[3., 2., 1.], 10),
            vec![3., 2., 1., 6., 9., 16., 31., 56., 103., 190.]
        );
        assert_eq!(tribonacci(&[1., 1., 1.], 1), vec![1.]);
        assert_eq!(tribonacci(&[300., 200., 100.], 0), vec![]);
        assert_eq!(
            tribonacci(&[0.5, 0.5, 0.5], 30),
            vec![
                0.5, 0.5, 0.5, 1.5, 2.5, 4.5, 8.5, 15.5, 28.5, 52.5, 96.5, 177.5, 326.5, 600.5,
                1104.5, 2031.5, 3736.5, 6872.5, 12640.5, 23249.5, 42762.5, 78652.5, 144664.5,
                266079.5, 489396.5, 900140.5, 1655616.5, 3045153.5, 5600910.5, 10301680.5
            ]
        );
    }
}
