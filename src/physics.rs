//inherited mutability: player is mutable if let is mutable

//todo: Test in ticks
//      Implement player acceleration based on ticks between inputs.
//      Potentially ticks holding a button and ticks not
//      Rotation accel/vel
use game;

use std::cmp::{max, min};

pub use self::collision::collide;

#[deriving(Eq)]
pub enum Direction {
    Forward,
    Backward,
    Still
}

pub fn accel(p: &mut game::Player){ //mut player:&mut player would allow to play w/ pointer
    if p.velocity >= -0.15 && p.velocity <= 0.15 {
        p.velocity += p.accel * 0.0001;
    }

    if p.velocity < -0.05 {
        p.velocity = -0.05;
    }

    if p.velocity >= 0.05 {
        p.velocity = 0.05
    }

    p.position += p.velocity;

    let (acc, amod) = accel_compute(p.dir, p.accel, p.accel_mod);

    p.accel = acc;
    p.accel_mod = amod;
}


pub fn accel_compute (dir: Direction, mut accel: f32, mut accel_mod: int) -> (f32, int) {
    //this will use accel/accel_mod to compute the rate of increase of acceleration.
    if dir == Forward {
        let bounds = [
            (-85, -75, 25),
            (-75, -60, 22),
            (-60, -41, 19),
            (-40, -15, 17),
            (0, 15, 12),
            (14, 40, 10),
            (40, 60, 8),
            (60, 75, 5),
            (75, 85, 2)
        ];

        accel_mod = max(accel_mod, -84);

        if accel_mod == 0 {
            accel_mod = 15;
        } else if accel_mod >= -15 && accel_mod < 0 {
            accel_mod = 0;
        } else {
            for &(lower, upper, increment) in bounds.iter() {
                if accel_mod >= lower && accel_mod <= upper {
                    accel_mod += increment;
                    break
                }
            }
        }

    } else if dir == Backward {
        let bounds = [
            (-85, -75, -2),
            (-75, -60, -5),
            (-60, -41, -8),
            (-40, -15, -10),
            (-15, 0, -12),
            (15, 40, 17),
            (40, 60, 19),
            (60, 75, -22),
            (75, 85, -25)
        ];

        accel_mod = min(accel_mod, 84);

        if accel_mod == 0 {
            accel_mod = -15;
        } else if accel_mod <= 15 && accel_mod > 0 {
            accel_mod = 0;
        } else {
            for &(lower, upper, increment) in bounds.iter() {
                if accel_mod >= lower && accel_mod <= upper {
                    accel_mod += increment;
                    break
                }
            }
        }
    }

    if accel <= 0.05 && accel >= -0.05 {
        accel = accel + (0.00003 * (accel_mod as f32));
    } else {
        accel = accel.signum() * 0.05;
    }

    (accel, accel_mod) //returns accel and accel mod
}

mod collision {
    use cgmath::angle::Rad;
    use cgmath::vector::{Vector, Vector2};
    use cgmath::matrix::{Matrix, Matrix2};
    use gl::types::GLfloat;
    use render::Sprite;
    use std::f32::consts::PI;

    type V = Vector2<GLfloat>;

    fn min(a: GLfloat, b: GLfloat) -> GLfloat {
        a.min(b)
    }

    fn max(a: GLfloat, b: GLfloat) -> GLfloat {
        a.max(b)
    }

    fn slope(a: V, b: V) -> GLfloat {
        (a.y - b.y) / (a.x - b.x)
    }

    fn line_intercept((a1, a2): (V, V), (b1, b2): (V, V)) -> f32 {
        let (x1, x2, y1, y2, m1, m2) = (a1.x, b1.x, a1.y, b1.y, slope(a1, a2), slope(b1, b2));

        ((m1*x1) - (m2*x2) - y1 + y2) / (m1 - m2)
    }

    fn intersect((a1, a2): (V, V), (b1, b2): (V, V)) -> Option<V> {
        debug!("Checking the intersection of the two lines, {} to {} and {} to {}", a1, a2, b1, b2);

        let x = line_intercept((a1, a2), (b1, b2));
        debug!("The lines, were they infinite, intersect at x = {}", x);

        let y = slope(a1, a2)*(x - a1.x) + a1.y;
        debug!("The corresponding y is {}", y);

        if x >= min(a1.x, a2.x) && x <= max(a1.x, a2.x) {
            debug!("It's within the first line's x values");
            if x >= min(b1.x, b2.x) && x <= max(b1.x, b2.x) {
                debug!("It's within the second line's x values");
                if y >= min(a1.y, a2.y) && y <= max(a1.y, a2.y) {
                    debug!("It's within the first line's y values");
                    if y >= min(b1.y, b2.y) && y <= max(b1.y, b2.y) {
                        debug!("It's within the second line's y values");
                        return Some(Vector2::new(x, y));
                    }
                }
            }
        }

        None
    }

    #[test]
    fn test_intersect() {
        let a = Vector2::new(1f32, 1f32);
        let b = Vector2::new(-1f32, -1f32);

        let c = Vector2::new(-1f32, 1f32);
        let d = Vector2::new(1f32, -1f32);

        assert_eq!(intersect((a, b), (c, d)), Some(Vector2::new(0f32, 0f32)));
    }

    /// Determines if two Sprites collide, assuming that they collide iff their
    /// (possibly rotated) bounding boxes collide, not using the texture in any
    /// way.
    pub fn collide(a: &Sprite, b: &Sprite) -> bool {

        // make lists of points making up each sprite, transformed with the
        // rotation

        let mut arot = a.rot;
        let mut brot = b.rot;

        // make sure their slopes aren't NaN :(
        if arot % (PI / 4.0) == 0.0 {
            arot += 0.01;
        }

        if brot % (PI / 4.0) == 0.0 {
            brot += 0.01;
        }

        let amat = Matrix2::from_angle(Rad { s: arot });
        let bmat = Matrix2::from_angle(Rad { s: brot });

        let pairs = [(0, 1), (1, 2), (2, 3), (3, 0)]; // which indices form lines we care about?

        let &Sprite {x, y, width, height, .. } = a;
        let apoints = [
            amat.mul_v(&Vector2::new(x, y)),
            amat.mul_v(&Vector2::new(x + width, y)),
            amat.mul_v(&Vector2::new(x + width, y + height)),
            amat.mul_v(&Vector2::new(x, y + height))
        ];

        let &Sprite {x, y, width, height, .. } = b;
        let bpoints = [
            bmat.mul_v(&Vector2::new(x, y)),
            bmat.mul_v(&Vector2::new(x + width, y)),
            bmat.mul_v(&Vector2::new(x + width, y + height)),
            bmat.mul_v(&Vector2::new(x, y + height))
        ];

        for &(a1, a2) in pairs.iter() {
            for &(b1, b2) in pairs.iter() {
                if intersect((apoints[a1], apoints[a2]), (bpoints[b1], bpoints[b2])) != None {
                    return true;
                }
            }
        }

        false
    }

    #[test]
    fn test_collide() {
        use std;

        let s1 = Sprite::new(0.0, 0.0, 1.0, 1.0, 0.0, unsafe { std::mem::uninit() });
        let s2 = Sprite::new(0.5, 0.5, 1.0, 1.0, 0.0, unsafe { std::mem::uninit() });

        assert_eq!(collide(&s1, &s2), true);

        unsafe {
            // since we're stubbing out the texture
            std::cast::forget(s1);
            std::cast::forget(s2);
        }

        let s1 = Sprite::new(0.0, 0.0, 1.0, 1.0, 0.0, unsafe { std::mem::uninit() });
        let s2 = Sprite::new(1.0, 1.0, 1.0, 1.0, 0.0, unsafe { std::mem::uninit() });

        assert_eq!(collide(&s1, &s2), true);

        unsafe {
            // since we're stubbing out the texture
            std::cast::forget(s1);
            std::cast::forget(s2);
        }

        let s1 = Sprite::new(0.0, 0.0, 1.0, 1.0, 0.0, unsafe { std::mem::uninit() });
        let s2 = Sprite::new(0.0, 0.0, 1.0, 1.0, 0.5, unsafe { std::mem::uninit() });

        assert_eq!(collide(&s1, &s2), true);

        unsafe {
            // since we're stubbing out the texture
            std::cast::forget(s1);
            std::cast::forget(s2);
        }
    }
}

