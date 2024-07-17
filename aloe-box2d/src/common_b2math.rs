crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Common/b2Math.h]

/**
   This function is used to ensure that
   a floating point number is not a NaN or
   infinity.
  */
#[inline] pub fn b2is_valid(x: f32) -> bool {
    
    todo!();
    /*
        if (x != x)
        {
            // NaN.
            return false;
        }

        float32 infinity = std::numeric_limits<float32>::infinity();
        return -infinity < x && x < infinity;
    */
}

/**
   This is a approximate yet fast inverse
   square-root.
  */
#[inline] pub fn b2inv_sqrt(x: f32) -> f32 {
    
    todo!();
    /*
        union
        {
            float32 x;
            int32 i;
        } convert;

        convert.x = x;
        float32 xhalf = 0.5f * x;
        convert.i = 0x5f3759df - (convert.i >> 1);
        x = convert.x;
        x = x * (1.5f - xhalf * x * x);
        return x;
    */
}

macro_rules! b2sqrt {
    ($x:ident) => {
        /*
                std::sqrt(x)
        */
    }
}

macro_rules! b2atan2 {
    ($y:ident, $x:ident) => {
        /*
                std::atan2(y, x)
        */
    }
}

/**
   A 2D column vector.
  */
pub struct b2Vec2 {
    x: f32,
    y: f32,
}

impl Default for b2Vec2 {
    
    /**
       Default constructor does nothing (for
       performance).
      */
    fn default() -> Self {
        todo!();
        /*


        
        */
    }
}

impl Neg for b2Vec2 {

    type Output = Self;

    /**
       Negate this vector.
      */
    #[inline] fn neg(self) -> Self::Output {
        todo!();
        /*
            b2Vec2 v; v.Set(-x, -y); return v;
        */
    }
}

impl AddAssign<&b2Vec2> for b2Vec2 {

    /**
       Add a vector to this vector.
      */
    #[inline] fn add_assign(&mut self, other: &b2Vec2) {
        todo!();
        /*
            x += v.x; y += v.y;
        */
    }
}

impl SubAssign<&b2Vec2> for b2Vec2 {

    /**
       Subtract a vector from this vector.
      */
    #[inline] fn sub_assign(&mut self, v: &b2Vec2) {
        todo!();
        /*
            x -= v.x; y -= v.y;
        */
    }
}

impl MulAssign<f32> for b2Vec2 {

    /**
       Multiply this vector by a scalar.
      */
    #[inline] fn mul_assign(&mut self, a: f32) {
        todo!();
        /*
            x *= a; y *= a;
        */
    }
}

impl b2Vec2 {

    /**
       Construct using coordinates.
      */
    pub fn new_from_coordinates(
        x_coord: f32,
        y_coord: f32) -> Self {
    
        todo!();
        /*
        : x(xCoord),
        : y(yCoord),

        
        */
    }

    /**
       Set this vector to all zeros.
      */
    pub fn set_zero(&mut self)  {
        
        todo!();
        /*
            x = 0.0f; y = 0.0f;
        */
    }

    /**
       Set this vector to some specified
       coordinates.
      */
    pub fn set(&mut self, x: f32, y: f32)  {
        
        todo!();
        /*
            x = x_; y = y_;
        */
    }

    /**
       Read from and indexed element.
      */
    pub fn invoke_read(&mut self, i: i32) -> f32 {
        
        todo!();
        /*
            return (&x)[i];
        */
    }

    /**
       Write to an indexed element.
      */
    pub fn invoke_write(&mut self, i: i32) -> &mut f32 {
        
        todo!();
        /*
            return (&x)[i];
        */
    }

    /**
       Get the length of this vector (the norm).
      */
    pub fn length(&self) -> f32 {
        
        todo!();
        /*
            return b2Sqrt(x * x + y * y);
        */
    }

    /**
       Get the length squared. For performance,
       use this instead of b2Vec2::Length (if
       possible).
      */
    pub fn length_squared(&self) -> f32 {
        
        todo!();
        /*
            return x * x + y * y;
        */
    }

    /**
      | Convert this vector into a unit vector.
      | Returns the length.
      |
      */
    pub fn normalize(&mut self) -> f32 {
        
        todo!();
        /*
            float32 length = Length();
            if (length < b2_epsilon)
            {
                return 0.0f;
            }
            float32 invLength = 1.0f / length;
            x *= invLength;
            y *= invLength;

            return length;
        */
    }

    /**
       Does this vector contain finite
       coordinates?
      */
    pub fn is_valid(&self) -> bool {
        
        todo!();
        /*
            return b2IsValid(x) && b2IsValid(y);
        */
    }

    /**
       Get the skew vector such that dot(skew_vec,
       other) == cross(vec, other)
      */
    pub fn skew(&self) -> b2Vec2 {
        
        todo!();
        /*
            return b2Vec2(-y, x);
        */
    }
}

/**
   A 2D column vector with 3 elements.
  */
pub struct b2Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Default for b2Vec3 {
    
    /**
       Default constructor does nothing (for
       performance).
      */
    fn default() -> Self {
        todo!();
        /*


        
        */
    }
}

impl Neg for b2Vec3 {

    type Output = Self;

    /**
       Negate this vector.
      */
    #[inline] fn neg(self) -> Self::Output {
        todo!();
        /*
            b2Vec3 v; v.Set(-x, -y, -z); return v;
        */
    }
}

impl AddAssign<&b2Vec3> for b2Vec3 {

    /**
       Add a vector to this vector.
      */
    #[inline] fn add_assign(&mut self, other: &b2Vec3) {
        todo!();
        /*
            x += v.x; y += v.y; z += v.z;
        */
    }
}

impl SubAssign<&b2Vec3> for b2Vec3 {

    /**
       Subtract a vector from this vector.
      */
    #[inline] fn sub_assign(&mut self, v: &b2Vec3) {
        todo!();
        /*
            x -= v.x; y -= v.y; z -= v.z;
        */
    }
}

impl MulAssign<f32> for b2Vec3 {

    /**
       Multiply this vector by a scalar.
      */
    #[inline] fn mul_assign(&mut self, s: f32) {
        todo!();
        /*
            x *= s; y *= s; z *= s;
        */
    }
}

impl b2Vec3 {

    /**
       Construct using coordinates.
      */
    pub fn new_from_coordinates(
        x_coord: f32,
        y_coord: f32,
        z_coord: f32) -> Self {
    
        todo!();
        /*
        : x(xCoord),
        : y(yCoord),
        : z(zCoord),

        
        */
    }

    /**
       Set this vector to all zeros.
      */
    pub fn set_zero(&mut self)  {
        
        todo!();
        /*
            x = 0.0f; y = 0.0f; z = 0.0f;
        */
    }

    /**
       Set this vector to some specified
       coordinates.
      */
    pub fn set(&mut self, 
        x: f32,
        y: f32,
        z: f32)  {
        
        todo!();
        /*
            x = x_; y = y_; z = z_;
        */
    }
}

/**
   A 2-by-2 matrix. Stored in column-major order.
  */
pub struct b2Mat22 {
    ex: b2Vec2,
    ey: b2Vec2,
}

impl Default for b2Mat22 {
    
    /**
       The default constructor does nothing (for
       performance).
      */
    fn default() -> Self {
        todo!();
        /*


        
        */
    }
}

impl b2Mat22 {

    /**
       Construct this matrix using columns.
      */
    pub fn new_from_columns(
        c1: &b2Vec2,
        c2: &b2Vec2) -> Self {
    
        todo!();
        /*


            ex = c1;
            ey = c2;
        */
    }

    /**
       Construct this matrix using scalars.
      */
    pub fn new_from_scalars(
        a11: f32,
        a12: f32,
        a21: f32,
        a22: f32) -> Self {
    
        todo!();
        /*


            ex.x = a11; ex.y = a21;
            ey.x = a12; ey.y = a22;
        */
    }

    /**
       Initialize this matrix using columns.
      */
    pub fn set(&mut self, 
        c1: &b2Vec2,
        c2: &b2Vec2)  {
        
        todo!();
        /*
            ex = c1;
            ey = c2;
        */
    }

    /**
       Set this to the identity matrix.
      */
    pub fn set_identity(&mut self)  {
        
        todo!();
        /*
            ex.x = 1.0f; ey.x = 0.0f;
            ex.y = 0.0f; ey.y = 1.0f;
        */
    }

    /**
       Set this matrix to all zeros.
      */
    pub fn set_zero(&mut self)  {
        
        todo!();
        /*
            ex.x = 0.0f; ey.x = 0.0f;
            ex.y = 0.0f; ey.y = 0.0f;
        */
    }
    
    pub fn get_inverse(&self) -> b2Mat22 {
        
        todo!();
        /*
            float32 a = ex.x, b = ey.x, c = ex.y, d = ey.y;
            b2Mat22 B;
            float32 det = a * d - b * c;
            if (det != 0.0f)
            {
                det = 1.0f / det;
            }
            B.ex.x =  det * d;  B.ey.x = -det * b;
            B.ex.y = -det * c;  B.ey.y =  det * a;
            return B;
        */
    }

    /**
       Solve A * x = b, where b is a column
       vector. This is more efficient than
       computing the inverse in one-shot cases.
      */
    pub fn solve(&self, b: &b2Vec2) -> b2Vec2 {
        
        todo!();
        /*
            float32 a11 = ex.x, a12 = ey.x, a21 = ex.y, a22 = ey.y;
            float32 det = a11 * a22 - a12 * a21;
            if (det != 0.0f)
            {
                det = 1.0f / det;
            }
            b2Vec2 x;
            x.x = det * (a22 * b.x - a12 * b.y);
            x.y = det * (a11 * b.y - a21 * b.x);
            return x;
        */
    }
}

/**
   A 3-by-3 matrix. Stored in column-major order.
  */
pub struct b2Mat33 {
    ex: b2Vec3,
    ey: b2Vec3,
    ez: b2Vec3,
}

impl Default for b2Mat33 {
    
    /**
       The default constructor does nothing (for
       performance).
      */
    fn default() -> Self {
        todo!();
        /*


        
        */
    }
}

impl b2Mat33 {

    /**
      | Construct this matrix using columns.
      |
      */
    pub fn new_from_columns(
        c1: &b2Vec3,
        c2: &b2Vec3,
        c3: &b2Vec3) -> Self {
    
        todo!();
        /*


            ex = c1;
            ey = c2;
            ez = c3;
        */
    }

    /**
      | Set this matrix to all zeros.
      |
      */
    pub fn set_zero(&mut self)  {
        
        todo!();
        /*
            ex.SetZero();
            ey.SetZero();
            ez.SetZero();
        */
    }
}

/**
  | Rotation
  |
  */
pub struct b2Rot {

    /**
      Sine and cosine
      */
    s: f32,
    c: f32,
}

impl b2Rot {

    /**
       Initialize from an angle in radians
      */
    pub fn new_from_angle(angle: f32) -> Self {
    
        todo!();
        /*


            /// TODO_ERIN optimize
            s = sinf(angle);
            c = cosf(angle);
        */
    }

    /**
       Set using an angle in radians.
      */
    pub fn set(&mut self, angle: f32)  {
        
        todo!();
        /*
            /// TODO_ERIN optimize
            s = sinf(angle);
            c = cosf(angle);
        */
    }

    /**
       Set to the identity rotation
      */
    pub fn set_identity(&mut self)  {
        
        todo!();
        /*
            s = 0.0f;
            c = 1.0f;
        */
    }

    /**
       Get the angle in radians
      */
    pub fn get_angle(&self) -> f32 {
        
        todo!();
        /*
            return b2Atan2(s, c);
        */
    }

    /**
       Get the x-axis
      */
    pub fn getx_axis(&self) -> b2Vec2 {
        
        todo!();
        /*
            return b2Vec2(c, s);
        */
    }

    /**
       Get the u-axis
      */
    pub fn gety_axis(&self) -> b2Vec2 {
        
        todo!();
        /*
            return b2Vec2(-s, c);
        */
    }
}

/**
   A transform contains translation and
   rotation. It is used to represent the position
   and orientation of rigid frames.
  */
pub struct b2Transform {
    p: b2Vec2,
    q: b2Rot,
}

impl Default for b2Transform {
    
    /**
       The default constructor does nothing.
      */
    fn default() -> Self {
        todo!();
        /*


        
        */
    }
}

impl b2Transform {

    /**
       Initialize using a position vector and
       a rotation.
      */
    pub fn new_from_position_and_rotation(
        position: &b2Vec2,
        rotation: &b2Rot) -> Self {
    
        todo!();
        /*
        : p(position),
        : q(rotation),

        
        */
    }

    /**
       Set this to the identity transform.
      */
    pub fn set_identity(&mut self)  {
        
        todo!();
        /*
            p.SetZero();
            q.SetIdentity();
        */
    }

    /**
       Set this based on the position and angle.
      */
    pub fn set(&mut self, 
        position: &b2Vec2,
        angle:    f32)  {
        
        todo!();
        /*
            p = position;
            q.Set(angle);
        */
    }
}

/**
  | This describes the motion of a body/shape for
  | TOI computation.  Shapes are defined with
  | respect to the body origin, which may no
  | coincide with the center of mass. However, to
  | support dynamics we must interpolate the
  | center of mass position.
  */
pub struct b2Sweep {

    /**
      local center of mass position
      */
    local_center: b2Vec2,

    /**
      center world positions
      */
    c0:           b2Vec2,

    /**
      center world positions
      */
    c:            b2Vec2,

    /**
      world angles
      */
    a0:           f32,

    /**
      world angles
      */
    a:            f32,

    /**
      | Fraction of the current time step in
      | the range [0,1] c0 and a0 are the positions
      | at alpha0.
      |
      */
    alpha0:       f32,
}

/**
  | Useful constant
  |
  */
lazy_static!{
    /*
    extern const b2Vec2 b2Vec2_zero;
    */
}

impl Dot for b2Vec2 {

    type Output = f32;

    /**
      | Perform the dot product on two vectors.
      |
      */
    #[inline] fn dot(
        a: &b2Vec2,
        b: &b2Vec2

    ) -> Self::Output {
        
        todo!();
        /*
            return a.x * b.x + a.y * b.y;
        */
    }
}

pub trait Cross<T> {

    type Output;

    fn cross(&self, a: T) -> Self::Output;
}

impl Cross<&b2Vec2> for b2Vec2 {

    type Output = f32;

    /**
      | Perform the cross product on two vectors.
      | In 2D this produces a scalar.
      |
      */
    #[inline] fn cross(&self, b: &b2Vec2) -> Self::Output {
        
        todo!();
        /*
            return a.x * b.y - a.y * b.x;
        */
    }
}

impl Cross<f32> for b2Vec2 {

    type Output = b2Vec2;

    /**
      | Perform the cross product on a vector
      | and a scalar. In 2D this produces a vector.
      |
      */
    #[inline] fn cross(&self, s: f32) -> Self::Output {
        
        todo!();
        /*
            return b2Vec2(s * a.y, -s * a.x);
        */
    }
}

impl Cross<&b2Vec2> for f32 {

    type Output = b2Vec2;

    /**
      | Perform the cross product on a scalar
      | and a vector. In 2D this produces a vector.
      |
      */
    #[inline] fn cross(&self, a: &b2Vec2) -> Self::Output {
        
        todo!();
        /*
            return b2Vec2(-s * a.y, s * a.x);
        */
    }
}

impl Mul<&b2Vec2> for b2Mat22 {

    type Output = b2Vec2;

    /**
      | Multiply a matrix times a vector. If
      | a rotation matrix is provided, then
      | this transforms the vector from one
      | frame to another.
      |
      */
    #[inline] fn mul(self, v: &b2Vec2) -> Self::Output {
        
        todo!();
        /*
            return b2Vec2(A.ex.x * v.x + A.ey.x * v.y, A.ex.y * v.x + A.ey.y * v.y);
        */
    }
}

pub trait MulTranspose<T> {

    type Output;

    #[inline] fn mult(&self, v: T) -> Self::Output;
}

impl MulTranspose<&b2Vec2> for b2Mat22 {

    type Output = b2Vec2;

    /**
      | Multiply a matrix transpose times a
      | vector. If a rotation matrix is provided,
      | then this transforms the vector from
      | one frame to another (inverse transform).
      |
      */
    #[inline] fn mult(&self, v: &b2Vec2) -> b2Vec2 {
        
        todo!();
        /*
            return b2Vec2(b2Dot(v, A.ex), b2Dot(v, A.ey));
        */
    }
}

impl Add<&b2Vec2> for b2Vec2 {

    type Output = Self;

    /**
      | Add two vectors component-wise.
      |
      */
    fn add(self, other: &b2Vec2) -> Self::Output {
        todo!();
        /*
            return b2Vec2(a.x + b.x, a.y + b.y);
        */
    }
}

impl Sub<&b2Vec2> for &b2Vec2 {

    type Output = b2Vec2;

    /**
      | Subtract two vectors component-wise.
      |
      */
    fn sub(self, other: &b2Vec2) -> Self::Output {
        todo!();
        /*
            return b2Vec2(a.x - b.x, a.y - b.y);
        */
    }
}

impl Mul<&b2Vec2> for f32 {
    type Output = b2Vec2;

    fn mul(self, other: &b2Vec2) -> Self::Output {
        todo!();
        /*
            return b2Vec2(s * a.x, s * a.y);
        */
    }
}

impl PartialEq<b2Vec2> for b2Vec2 {
    
    #[inline] fn eq(&self, other: &b2Vec2) -> bool {
        todo!();
        /*
            return a.x == b.x && a.y == b.y;
        */
    }
}

impl Eq for b2Vec2 {}

#[inline] pub fn b2distance(
        a: &b2Vec2,
        b: &b2Vec2) -> f32 {
    
    todo!();
    /*
        b2Vec2 c = a - b;
        return c.Length();
    */
}

#[inline] pub fn b2distance_squared(
        a: &b2Vec2,
        b: &b2Vec2) -> f32 {
    
    todo!();
    /*
        b2Vec2 c = a - b;
        return b2Dot(c, c);
    */
}

impl Mul<&b2Vec3> for f32 {
    type Output = b2Vec3;
    
    fn mul(self, other: &b2Vec3) -> Self::Output {
        todo!();
        /*
            return b2Vec3(s * a.x, s * a.y, s * a.z);
        */
    }
}

impl Add<&b2Vec3> for b2Vec3 {

    type Output = Self;
    
    /**
      | Add two vectors component-wise.
      |
      */
    fn add(self, other: &b2Vec3) -> Self::Output {
        todo!();
        /*
            return b2Vec3(a.x + b.x, a.y + b.y, a.z + b.z);
        */
    }
}

impl Sub<&b2Vec3> for &b2Vec3 {

    type Output = b2Vec3;

    /**
      | Subtract two vectors component-wise.
      |
      */
    fn sub(self, other: &b2Vec3) -> Self::Output {
        todo!();
        /*
            return b2Vec3(a.x - b.x, a.y - b.y, a.z - b.z);
        */
    }
}

pub trait Dot {

    type Output;

    #[inline] fn dot(
            a: &Self,
            b: &Self) -> Self::Output;
}

/**
   Perform the dot product on two vectors.
  */
impl Dot for b2Vec3 {

    type Output = f32;

    #[inline] fn dot(
            a: &b2Vec3,
            b: &b2Vec3) -> Self::Output {
        
        todo!();
        /*
            return a.x * b.x + a.y * b.y + a.z * b.z;
        */
    }
}

/**
   Perform the cross product on two vectors.
  */
#[inline] pub fn b2cross(
        a: &b2Vec3,
        b: &b2Vec3) -> b2Vec3 {
    
    todo!();
    /*
        return b2Vec3(a.y * b.z - a.z * b.y, a.z * b.x - a.x * b.z, a.x * b.y - a.y * b.x);
    */
}

impl Add<&b2Mat22> for b2Mat22 {

    type Output = Self;
    
    fn add(self, other: &b2Mat22) -> Self::Output {
        todo!();
        /*
            return b2Mat22(A.ex + B.ex, A.ey + B.ey);
        */
    }
}

/**
   A * B
  */
impl Mul<&b2Mat22> for b2Mat22 {

    type Output = b2Mat22;

    #[inline] fn mul(self, b: &b2Mat22) -> Self::Output {
        
        todo!();
        /*
            return b2Mat22(b2Mul(A, B.ex), b2Mul(A, B.ey));
        */
    }
}

impl MulTranspose<&b2Mat22> for b2Mat22 {

    type Output = b2Mat22;

    /**
      | A^T * B
      |
      */
    #[inline] fn mult(&self, b: &b2Mat22) -> Self::Output {
        
        todo!();
        /*
            b2Vec2 c1(b2Dot(A.ex, B.ex), b2Dot(A.ey, B.ex));
            b2Vec2 c2(b2Dot(A.ex, B.ey), b2Dot(A.ey, B.ey));
            return b2Mat22(c1, c2);
        */
    }
}

/**
   Multiply a matrix times a vector.
  */
impl Mul<&b2Vec3> for b2Mat33 {

    type Output = b2Vec3;

    #[inline] fn mul(self, v: &b2Vec3) -> Self::Output {
        
        todo!();
        /*
            return v.x * A.ex + v.y * A.ey + v.z * A.ez;
        */
    }
}

/**
   Multiply a matrix times a vector.
  */
#[inline] pub fn b2mul22(
        a: &b2Mat33,
        v: &b2Vec2) -> b2Vec2 {
    
    todo!();
    /*
        return b2Vec2(A.ex.x * v.x + A.ey.x * v.y, A.ex.y * v.x + A.ey.y * v.y);
    */
}

/**
   Multiply two rotations: q * r
  */
impl Mul<&b2Rot> for b2Rot {

    type Output = b2Rot;

    #[inline] fn mul(self, r: &b2Rot) -> Self::Output {
        
        todo!();
        /*
            // [qc -qs] * [rc -rs] = [qc*rc-qs*rs -qc*rs-qs*rc]
            // [qs  qc]   [rs  rc]   [qs*rc+qc*rs -qs*rs+qc*rc]
            // s = qs * rc + qc * rs
            // c = qc * rc - qs * rs
            b2Rot qr;
            qr.s = q.s * r.c + q.c * r.s;
            qr.c = q.c * r.c - q.s * r.s;
            return qr;
        */
    }
}

impl MulTranspose<&b2Rot> for b2Rot {

    type Output = b2Rot;

    /**
      | Transpose multiply two rotations:
      | qT * r
      |
      */
    #[inline] fn mult(&self, r: &b2Rot) -> Self::Output {
        
        todo!();
        /*
            // [ qc qs] * [rc -rs] = [qc*rc+qs*rs -qc*rs+qs*rc]
            // [-qs qc]   [rs  rc]   [-qs*rc+qc*rs qs*rs+qc*rc]
            // s = qc * rs - qs * rc
            // c = qc * rc + qs * rs
            b2Rot qr;
            qr.s = q.c * r.s - q.s * r.c;
            qr.c = q.c * r.c + q.s * r.s;
            return qr;
        */
    }
}

/**
   Rotate a vector
  */
impl Mul<&b2Vec2> for b2Rot {

    type Output = b2Vec2;

    //self is "q"
    #[inline] fn mul(self, v: &b2Vec2) -> Self::Output {
        
        todo!();
        /*
            return b2Vec2(q.c * v.x - q.s * v.y, q.s * v.x + q.c * v.y);
        */
    }
}

impl MulTranspose<&b2Vec2> for b2Rot {

    type Output = b2Vec2;

    /**
      | Inverse rotate a vector
      |
      */
    #[inline] fn mult(&self, v: &b2Vec2) -> Self::Output {
        
        todo!();
        /*
            return b2Vec2(q.c * v.x + q.s * v.y, -q.s * v.x + q.c * v.y);
        */
    }
}

impl Mul<&b2Vec2> for b2Transform {

    type Output = b2Vec2;

    #[inline] fn mul(self, v: &b2Vec2) -> b2Vec2 {
        
        todo!();
        /*
            float32 x = (T.q.c * v.x - T.q.s * v.y) + T.p.x;
            float32 y = (T.q.s * v.x + T.q.c * v.y) + T.p.y;

            return b2Vec2(x, y);
        */
    }
}

impl MulTranspose<&b2Vec2> for b2Transform {

    type Output = b2Vec2;

    #[inline] fn mult(&self, v: &b2Vec2) -> Self::Output {
        
        todo!();
        /*
            float32 px = v.x - T.p.x;
            float32 py = v.y - T.p.y;
            float32 x = (T.q.c * px + T.q.s * py);
            float32 y = (-T.q.s * px + T.q.c * py);

            return b2Vec2(x, y);
        */
    }
}

/**
   v2 = A.q.Rot(B.q.Rot(v1) + B.p) + A.p
      = (A.q * B.q).Rot(v1) + A.q.Rot(B.p) + A.p
  */
impl Mul<&b2Transform> for b2Transform {

    type Output = b2Transform;

    #[inline] fn mul(self, b: &b2Transform) -> b2Transform {
        
        todo!();
        /*
            b2Transform C;
            C.q = b2Mul(A.q, B.q);
            C.p = b2Mul(A.q, B.p) + A.p;
            return C;
        */
    }
}

/**
   v2 = A.q' * (B.q * v1 + B.p - A.p)
      = A.q' * B.q * v1 + A.q' * (B.p - A.p)
  */
impl MulTranspose<&b2Transform> for b2Transform {

    type Output = b2Transform;

    #[inline] fn mult(&self, b: &b2Transform) -> Self::Output {
        
        todo!();
        /*
            b2Transform C;
            C.q = b2MulT(A.q, B.q);
            C.p = b2MulT(A.q, B.p - A.p);
            return C;
        */
    }
}

pub trait Abs {
    fn abs(&self) -> Self;
}

/*
impl<T> Abs for T {

    #[inline] fn abs(&self) -> Self {

        todo!();
        /*
            return a > T(0) ? a : -a;
        */
    }
}
*/

impl Abs for b2Vec2 {

    #[inline] fn abs(&self) -> Self {
        
        todo!();
        /*
            return b2Vec2(b2Abs(a.x), b2Abs(a.y));
        */
    }
}

impl Abs for b2Mat22 {

    #[inline] fn abs(&self) -> Self {
        
        todo!();
        /*
            return b2Mat22(b2Abs(A.ex), b2Abs(A.ey));
        */
    }
}


pub trait Min {
    fn min(&self, b: &Self) -> Self;
}

#[inline] pub fn min<T: Min>(a: T, b: T) -> T {
    a.min(&b)
}

pub trait Max {
    fn max(&self, b: &Self) -> Self;
}

#[inline] pub fn max<T: Max>(a: T, b: T) -> T {
    a.max(&b)
}

/*
impl<T> Min for T {

    #[inline] fn min(a: T, b: T) -> T {

        todo!();
        /*
            return a < b ? a : b;
        */
    }
}
*/

impl Min for b2Vec2 {

    #[inline] fn min(&self, b: &b2Vec2) -> Self {
        
        todo!();
        /*
            return b2Vec2(b2Min(a.x, b.x), b2Min(a.y, b.y));
        */
    }
}

/*
impl<T> Max for T {
    #[inline] fn max(&self, b: T) -> T {

        todo!();
        /*
            return a > b ? a : b;
        */
    }
}
*/

impl Max for b2Vec2 {
    #[inline] fn max(&self, b: &b2Vec2) -> Self {
        
        todo!();
        /*
            return b2Vec2(b2Max(a.x, b.x), b2Max(a.y, b.y));
        */
    }
}

pub trait Clamp: Sized {

    fn clamp(&self, low: &Self, high: &Self) -> Self {

        todo!();
        /*
            return b2Max(low, b2Min(a, high));
        */
    }
}

#[inline] pub fn clamp<T: Clamp>(a: T, low: T, high: T) -> T {
    a.clamp(&low,&high)
}


/*
impl<T> Clamp for T {

    #[inline] fn clamp(
            a:    T,
            low:  T,
            high: T) -> T {

        todo!();
        /*
            return b2Max(low, b2Min(a, high));
        */
    }
}
*/

impl Clamp for b2Vec2 {

    #[inline] fn clamp(
        &self,
        low:  &b2Vec2,
        high: &b2Vec2

    ) -> b2Vec2 {
        
        todo!();
        /*
            return b2Max(low, b2Min(a, high));
        */
    }
}

#[inline] pub fn b2swap<T>(
    a: &mut T,
    b: &mut T

) {

    todo!();
    /*
        T tmp = a;
        a = b;
        b = tmp;
    */
}

/**
  | "Next Largest Power of 2
  |
  | Given a binary integer value x, the next
  | largest power of 2 can be computed by a SWAR
  | algorithm that recursively "folds" the upper
  | bits into the lower bits. This process yields
  | a bit vector with the same most significant
  | 1 as x, but all 1's below it. Adding 1 to that
  | value yields the next largest power of 2. For
  | a 32-bit value:"
  */
#[inline] pub fn b2next_power_of_two(x: u32) -> u32 {
    
    todo!();
    /*
        x |= (x >> 1);
        x |= (x >> 2);
        x |= (x >> 4);
        x |= (x >> 8);
        x |= (x >> 16);
        return x + 1;
    */
}

#[inline] pub fn b2is_power_of_two(x: u32) -> bool {
    
    todo!();
    /*
        bool result = x > 0 && (x & (x - 1)) == 0;
        return result;
    */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Common/b2Math.cpp]
impl b2Sweep {
    
    /**
      | Get the interpolated transform at a
      | specific time.
      | 
      | -----------
      | @param beta
      | 
      | is a factor in [0,1], where 0 indicates
      | alpha0.
      |
      */
    #[inline] pub fn get_transform(&self, 
        xf:   *mut b2Transform,
        beta: f32)  {
        
        todo!();
        /*
            xf->p = (1.0f - beta) * c0 + beta * c;
        float32 angle = (1.0f - beta) * a0 + beta * a;
        xf->q.Set(angle);

        // Shift to origin
        xf->p -= b2Mul(xf->q, localCenter);
        */
    }
    
    /**
      | Advance the sweep forward, yielding
      | a new initial state.
      | 
      | -----------
      | @param alpha
      | 
      | the new initial time.
      |
      */
    #[inline] pub fn advance(&mut self, alpha: f32)  {
        
        todo!();
        /*
            b2Assert(alpha0 < 1.0f);
        float32 beta = (alpha - alpha0) / (1.0f - alpha0);
        c0 = (1.0f - beta) * c0 + beta * c;
        a0 = (1.0f - beta) * a0 + beta * a;
        alpha0 = alpha;
        */
    }

    /**
      | Normalize an angle in radians to be between
      | -pi and pi
      |
      */
    #[inline] pub fn normalize(&mut self)  {
        
        todo!();
        /*
            float32 twoPi = 2.0f * b2_pi;
        float32 d =  twoPi * floorf(a0 / twoPi);
        a0 -= d;
        a -= d;
        */
    }
}

lazy_static!{
    /*
    const b2Vec2 b2Vec2_zero(0.0f, 0.0f);
    */
}

impl b2Mat33 {
    
    /**
      | Solve A * x = b, where b is a column vector.
      | This is more efficient than computing
      | the inverse in one-shot cases.
      |
      */
    pub fn solve33(&self, b: &b2Vec3) -> b2Vec3 {
        
        todo!();
        /*
            float32 det = b2Dot(ex, b2Cross(ey, ez));
        if (det != 0.0f)
        {
            det = 1.0f / det;
        }
        b2Vec3 x;
        x.x = det * b2Dot(b, b2Cross(ey, ez));
        x.y = det * b2Dot(ex, b2Cross(b, ez));
        x.z = det * b2Dot(ex, b2Cross(ey, b));
        return x;
        */
    }
}

impl b2Mat33 {
    
    /**
      | Solve A * x = b, where b is a column
      | vector. This is more efficient than
      | computing the inverse in one-shot
      | cases. Solve only the upper 2-by-2 matrix
      | equation.
      */
    pub fn solve22(&self, b: &b2Vec2) -> b2Vec2 {
        
        todo!();
        /*
            float32 a11 = ex.x, a12 = ey.x, a21 = ex.y, a22 = ey.y;
        float32 det = a11 * a22 - a12 * a21;
        if (det != 0.0f)
        {
            det = 1.0f / det;
        }
        b2Vec2 x;
        x.x = det * (a22 * b.x - a12 * b.y);
        x.y = det * (a11 * b.y - a21 * b.x);
        return x;
        */
    }
    
    /**
      | Get the inverse of this matrix as a 2-by-2.
      | Returns the zero matrix if singular.
      |
      */
    pub fn get_inverse22(&self, m: *mut b2Mat33)  {
        
        todo!();
        /*
            float32 a = ex.x, b = ey.x, c = ex.y, d = ey.y;
        float32 det = a * d - b * c;
        if (det != 0.0f)
        {
            det = 1.0f / det;
        }

        M->ex.x =  det * d; M->ey.x = -det * b; M->ex.z = 0.0f;
        M->ex.y = -det * c; M->ey.y =  det * a; M->ey.z = 0.0f;
        M->ez.x = 0.0f; M->ez.y = 0.0f; M->ez.z = 0.0f;
        */
    }

    /**
      | Get the symmetric inverse of this matrix
      | as a 3-by-3.
      | 
      | Returns the zero matrix if singular.
      |
      */
    pub fn get_sym_inverse33(&self, m: *mut b2Mat33)  {
        
        todo!();
        /*
            float32 det = b2Dot(ex, b2Cross(ey, ez));
        if (det != 0.0f)
        {
            det = 1.0f / det;
        }

        float32 a11 = ex.x, a12 = ey.x, a13 = ez.x;
        float32 a22 = ey.y, a23 = ez.y;
        float32 a33 = ez.z;

        M->ex.x = det * (a22 * a33 - a23 * a23);
        M->ex.y = det * (a13 * a23 - a12 * a33);
        M->ex.z = det * (a12 * a23 - a13 * a22);

        M->ey.x = M->ex.y;
        M->ey.y = det * (a11 * a33 - a13 * a13);
        M->ey.z = det * (a13 * a12 - a11 * a23);

        M->ez.x = M->ex.z;
        M->ez.y = M->ey.z;
        M->ez.z = det * (a11 * a22 - a12 * a12);
        */
    }
}
