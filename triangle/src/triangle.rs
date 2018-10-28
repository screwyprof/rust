pub fn calc_c_side(a: f32, b: f32) -> f32 {
    f32::sqrt(a * a + b * b)
}

pub fn calc_perimeter(a: f32, b: f32, c: f32) -> f32 {
    a + b + c
}

pub fn calc_square(a: f32, b: f32) -> f32 {
    0.5 * a * b
}

pub fn is_right_triangle(a: f32, b: f32, c: f32) -> bool {
    (a + b) > c && (b + c) > a && (a + c) > b && a * a + b * b == c * c
}

pub fn calc_alfa_via_sinus_theorem(a: f32, c: f32) -> f32 {
    f32::to_degrees(f32::asin(a / c))
}

pub fn calc_beta_via_sinus_theorem(b: f32, c: f32) -> f32 {
    f32::to_degrees(f32::asin(b / c))
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_calc_c_side() {
        assert_eq!(5.0, calc_c_side(3.0, 4.0));
    }

    #[test]
    fn test_calc_perimeter() {
        assert_eq!(12.0, calc_perimeter(3.0, 4.0, 5.0));
    }

    #[test]
    fn test_calc_square() {
        assert_eq!(6.0, calc_square(3.0, 4.0));
    }

    #[test]
    fn test_calc_alfa_via_sinus_theorem() {
        assert_eq!(36.869896, calc_alfa_via_sinus_theorem(3.0, 5.0));
    }

    #[test]
    fn test_calc_beta_via_sinus_theorem() {
        assert_eq!(53.130104, calc_beta_via_sinus_theorem(4.0, 5.0));
    }    
}
