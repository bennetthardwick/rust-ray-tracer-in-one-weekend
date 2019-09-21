use std::fmt;
use std::ops;

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

macro_rules! vec3 {
    ($v: expr) => {
        Vec3 {
            x: $v,
            y: $v,
            z: $v
        }
    };
    ($x: expr, $y: expr, $z: expr) => {
        Vec3 {
            x: $x,
            y: $y,
            z: $z
        }
    }
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        vec3!(x, y, z)
    }

    pub fn of(v: f32) -> Vec3 {
        vec3!(v)
    }

    pub fn map<F>(&mut self, f: F) -> &mut Vec3 where F: Fn(f32) -> f32 {
        self.x = f(self.x);
        self.y = f(self.y);
        self.z = f(self.z);
        self
    }

    pub fn unit_vector(&self) -> Vec3 {
        self / self.length()
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }

    pub fn r(&self) -> f32 {
        self.x
    }

    pub fn g(&self) -> f32 {
        self.y
    }

    pub fn b(&self) -> f32 {
        self.z
    }

    pub fn squared_length(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }

    pub fn make_unit_vector(&mut self) {
        self.x /= self.length();
        self.y /= self.length();
        self.z /= self.length();
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

macro_rules! vec3_mut_impl {
    (f32) => {
        impl ops::MulAssign<f32> for Vec3 {
            fn mul_assign(&mut self, other: f32) {
                self.x *= other;
                self.y *= other;
                self.z *= other;
            }
        }

        impl ops::DivAssign<f32> for Vec3 {
            fn div_assign(&mut self, other: f32) {
                self.x /= other;
                self.y /= other;
                self.z /= other;
            }
        }
    };

    ($a:ty) => {
        impl ops::AddAssign<$a> for Vec3 {
            fn add_assign(&mut self, other: $a) {
                self.x += other.x;
                self.y += other.y;
                self.z += other.z;
            }
        }

        impl ops::SubAssign<$a> for Vec3 {
            fn sub_assign(&mut self, other: $a) {
                self.x -= other.x;
                self.y -= other.y;
                self.z -= other.z;
            }
        }

        impl ops::DivAssign<$a> for Vec3 {
            fn div_assign(&mut self, other: $a) {
                self.x /= other.x;
                self.y /= other.y;
                self.z /= other.z;
            }
        }

        impl ops::MulAssign<$a> for Vec3 {
            fn mul_assign(&mut self, other: $a) {
                self.x *= other.x;
                self.y *= other.y;
                self.z *= other.z;
            }
        }
    };
}

macro_rules! vec3_impl {
    ($a:ty, $b:ty) => {
        impl ops::Add<$b> for $a {
            type Output = Vec3;
            fn add(self, rhs: $b) -> Vec3 {
                Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
            }
        }

        impl ops::Sub<$b> for $a {
            type Output = Vec3;
            fn sub(self, rhs: $b) -> Vec3 {
                Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
            }
        }

        impl ops::Mul<$b> for $a {
            type Output = Vec3;
            fn mul(self, rhs: $b) -> Vec3 {
                Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
            }
        }

        impl ops::Div<$b> for $a {
            type Output = Vec3;
            fn div(self, rhs: $b) -> Vec3 {
                Vec3::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
            }
        }
    };
    ($a:ty) => {
        impl ops::Mul<f32> for $a {
            type Output = Vec3;

            fn mul(self, rhs: f32) -> Vec3 {
                Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
            }
        }

        impl ops::Mul<$a> for f32 {
            type Output = Vec3;

            fn mul(self, rhs: $a) -> Vec3 {
                Vec3::new(rhs.x * self, rhs.y * self, rhs.z * self)
            }
        }

        impl ops::Div<f32> for $a {
            type Output = Vec3;

            fn div(self, rhs: f32) -> Vec3 {
                Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
            }
        }

        impl ops::Div<$a> for f32 {
            type Output = Vec3;

            fn div(self, rhs: $a) -> Vec3 {
                Vec3::new(rhs.x / self, rhs.y / self, rhs.z / self)
            }
        }

        impl ops::Neg for $a {
            type Output = Vec3;

            fn neg(self) -> Vec3 {
                Vec3::new(-self.x, -self.y, -self.z)
            }
        }
    };
}

vec3_impl!(&Vec3, &Vec3);
vec3_impl!(Vec3, &Vec3);
vec3_impl!(&Vec3, Vec3);
vec3_impl!(Vec3, Vec3);
vec3_impl!(Vec3);
vec3_impl!(&Vec3);

vec3_mut_impl!(&Vec3);
vec3_mut_impl!(Vec3);
vec3_mut_impl!(f32);
