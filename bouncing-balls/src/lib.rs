/*
A child is playing with a ball on the nth floor of a tall building. The height of this floor above ground level, h, is known.

He drops the ball out of the window. The ball bounces (for example), to two-thirds of its height (a bounce of 0.66).

His mother looks out of a window 1.5 meters from the ground.

How many times will the mother see the ball pass in front of her window (including when it's falling and bouncing)?
*/

pub fn bouncing_ball(h: f64, bounce: f64, window: f64) -> i32 {
    if !(h > 0. && 0. < bounce && bounce < 1. && window < h) {
        return -1;
    }

    let mut count: i32 = 1;
    let mut bounce_height: f64 = h;

    loop {
        bounce_height *= bounce;

        if bounce_height > window {
            count += 2;
        } else {
            break;
        }
    }

    count
}

pub fn _bouncing_ball(h: f64, bounce: f64, window: f64) -> i32 {
    if !(h > 0. && 0. < bounce && bounce < 1. && window < h) {
        -1
    } else {
        (window / h).log(bounce).ceil() as i32 * 2 - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testequal(h: f64, bounce: f64, window: f64, exp: i32) -> () {
        assert_eq!(_bouncing_ball(h, bounce, window), exp)
    }

    #[test]
    fn tests_bouncing_ball() {
        testequal(3.0, 0.66, 1.5, 3);
        testequal(30.0, 0.66, 1.5, 15);
        testequal(40.0, 0.4, 10.0, 3);
        testequal(10.0, 0.6, 10.0, -1);
    }
}
