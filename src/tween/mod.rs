/*
    Implementation of tweener.

    A tween function takes an input value `x` ranging from 0 to 1 and maps it according to a specific procedure.
    It ensures that the function satisfies the following conditions:
    - f(0) = 0
    - f(1) = 1

    Tween functions are commonly used in animations to interpolate values smoothly over time.
*/

use std::f32::consts::*;

#[allow(unused)]
pub struct Tweener<'a> {
    //pub value: f32,
    pub tween_function: &'a dyn Fn(f32) -> f32,
}

impl<'a> Tweener<'a> {
    pub fn new(tween_function: &'a dyn Fn(f32) -> f32) -> Self {
        Self {
            //value: 0.0,
            tween_function,
        }
    }

    pub fn get(&self, value : f32) -> f32{
        (self.tween_function)(value)
    }
}

#[allow(unused)]
impl Tweener<'_> {
    pub fn ease_linear(x: f32) -> f32 {
        x
    }

    pub fn ease_in_sin(x: f32) -> f32 {
        1.0 - f32::cos(x * PI * 0.5)
    }

    pub fn ease_out_sin(x: f32) -> f32 {
        f32::sin(x * PI * 0.5)
    }

    pub fn ease_in_out_sin(x: f32) -> f32 {
        (1.0 - f32::cos(x * PI)) * 0.5
    }

    pub fn ease_in_quad(x: f32) -> f32 {
        x * x
    }

    pub fn ease_out_quad(x: f32) -> f32 {
        x * (2.0 - x)
    }

    pub fn ease_in_out_quad(x: f32) -> f32 {
        if x < 0.5 {
            2.0 * x * x
        } else {
            -1.0 + (4.0 - 2.0 * x) * x
        }
    }

    pub fn ease_in_cubic(x: f32) -> f32 {
        x * x * x
    }

    pub fn ease_out_cubic(x: f32) -> f32 {
        let t = x - 1.0;
        t * t * t + 1.0
    }

    pub fn ease_in_out_cubic(x: f32) -> f32 {
        if x < 0.5 {
            4.0 * x * x * x
        } else {
            let t = 2.0 * x - 2.0;
            0.5 * t * t * t + 1.0
        }
    }

    pub fn ease_in_expo(x: f32) -> f32 {
        if x == 0.0 {
            0.0
        } else {
            2.0_f32.powf(10.0 * (x - 1.0))
        }
    }

    pub fn ease_out_expo(x: f32) -> f32 {
        if x == 1.0 {
            1.0
        } else {
            1.0 - 2.0_f32.powf(-10.0 * x)
        }
    }

    pub fn ease_in_out_expo(x: f32) -> f32 {
        if x == 0.0 {
            0.0
        } else if x == 1.0 {
            1.0
        } else if x < 0.5 {
            2.0_f32.powf(20.0 * x - 10.0) * 0.5
        } else {
            (2.0 - 2.0_f32.powf(-20.0 * x + 10.0)) * 0.5
        }
    }

    pub fn ease_in_circ(x: f32) -> f32 {
        1.0 - (1.0 - x * x).sqrt()
    }

    pub fn ease_out_circ(x: f32) -> f32 {
        (1.0 - (x - 1.0) * (x - 1.0)).sqrt()
    }

    pub fn ease_in_out_circ(x: f32) -> f32 {
        if x < 0.5 {
            (1.0 - (1.0 - 4.0 * x * x).sqrt()) * 0.5
        } else {
            ((1.0 - (-(2.0 * x - 2.0)).powi(2)).sqrt() + 1.0) * 0.5
        }
    }

    pub fn ease_in_bounce(x: f32) -> f32 {
        1.0 - Tweener::ease_out_bounce(1.0 - x)
    }

    pub fn ease_out_bounce(x: f32) -> f32 {
        if x < 1.0 / 2.75 {
            7.5625 * x * x
        } else if x < 2.0 / 2.75 {
            let t = x - 1.5 / 2.75;
            7.5625 * t * t + 0.75
        } else if x < 2.5 / 2.75 {
            let t = x - 2.25 / 2.75;
            7.5625 * t * t + 0.9375
        } else {
            let t = x - 2.625 / 2.75;
            7.5625 * t * t + 0.984375
        }
    }

    pub fn ease_in_out_bounce(x: f32) -> f32 {
        if x < 0.5 {
            Tweener::ease_in_bounce(x * 2.0) * 0.5
        } else {
            Tweener::ease_out_bounce(x * 2.0 - 1.0) * 0.5 + 0.5
        }
    }

    pub fn ease_in_elastic(x: f32) -> f32 {
        if x == 0.0 || x == 1.0 {
            x
        } else {
            -2.0_f32.powf(10.0 * (x - 1.0)) * ((x - 1.1) * 5.0 * PI).sin()
        }
    }

    pub fn ease_out_elastic(x: f32) -> f32 {
        if x == 0.0 || x == 1.0 {
            x
        } else {
            2.0_f32.powf(-10.0 * x) * ((x - 0.1) * 5.0 * PI).sin() + 1.0
        }
    }

    pub fn ease_in_out_elastic(x: f32) -> f32 {
        if x == 0.0 || x == 1.0 {
            x
        } else if x < 0.5 {
            -0.5 * 2.0_f32.powf(10.0 * (2.0 * x - 1.0)) * ((2.0 * x - 1.1) * 5.0 * PI).sin()
        } else {
            0.5 * 2.0_f32.powf(-10.0 * (2.0 * x - 1.0)) * ((2.0 * x - 1.1) * 5.0 * PI).sin() + 1.0
        }
    }
}
