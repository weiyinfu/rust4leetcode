struct Solution;

impl Solution {
    pub fn angle_clock(hour: i32, minutes: i32) -> f64 {
        let mut h = hour as f64 % 12.0 / 12.0 * 360.0;
        h += minutes as f64 / 60.0 * 30.0;
        h %= 360.0;
        let mut m = minutes as f64 / 60.0 * 360.0;
        m %= 360.0;
        let mut dif = h - m;
        if dif < 0.0 {
            dif = -dif;
        }
        if dif > 180.0 {
            dif = 360.0 - dif;
        }
        dif
    }
}
