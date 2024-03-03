// Kadanes Algorithm: https://medium.com/@rsinghal757/kadanes-algorithm-dynamic-programming-how-and-why-does-it-work-3fd8849ed73d

pub fn max_sequence(seq: &[i32]) -> i32 {
    let mut local_max: i32 = 0;
    let mut global_max: i32 = 0;

    for i in 0..seq.len() {
        local_max = std::cmp::max(seq[i], local_max + seq[i]);

        if local_max > global_max {
            global_max = local_max;
        }
    }

    global_max
}

// clever solution -> 

fn _max_sequence(seq: &[i32]) -> i32 {
    let mut m = 0;
    seq.iter().fold(0, |prev, &v|{
        let p = v.max(prev + v);
        m = m.max(p);
        p
    });
    m
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(max_sequence(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
        assert_eq!(max_sequence(&[11]), 11);
        assert_eq!(max_sequence(&[-32]), 0);
    }
}
