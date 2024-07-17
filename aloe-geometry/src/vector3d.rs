crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_opengl/geometry/aloe_Vector3D.h]

/**
  | A three-coordinate vector.
  | 
  | @tags{OpenGL}
  |
  */
pub struct Vector3D<Type> {
    x: Type,
    y: Type,
    z: Type,
}

impl<Type> Default for Vector3D<Type> {
    
    fn default() -> Self {
        todo!();
        /*
        : x(),
        : y(),
        : z(),

        
        */
    }
}

impl<Type> AddAssign<Vector3D<Type>> for Vector3D<Type> {
    
    #[inline]fn add_assign(&mut self, other: Vector3D<Type>) {
        todo!();
        /*
            x += other.x;  y += other.y;  z += other.z;  return *this;
        */
    }
}

impl<Type> SubAssign<Vector3D<Type>> for Vector3D<Type> {
    
    #[inline]fn sub_assign(&mut self, other: Vector3D<Type>) {
        todo!();
        /*
            x -= other.x;  y -= other.y;  z -= other.z;  return *this;
        */
    }
}

impl<Type> MulAssign<Type> for Vector3D<Type> {
    
    #[inline] fn mul_assign(&mut self, scale_factor: Type) {
        todo!();
        /*
            x *= scaleFactor;  y *= scaleFactor;  z *= scaleFactor;  return *this;
        */
    }
}

impl<Type> DivAssign<Type> for Vector3D<Type> {
    
    #[inline] fn div_assign(&mut self, scale_factor: Type) {
        todo!();
        /*
            x /= scaleFactor;  y /= scaleFactor;  z /= scaleFactor;  return *this;
        */
    }
}

impl<Type> Add<Vector3D<Type>> for Vector3D<Type> {

    type Output = Vector3D<Type>;
    
    #[inline]fn add(self, other: Vector3D<Type>) -> Self::Output {
        todo!();
        /*
            return { x + other.x, y + other.y, z + other.z };
        */
    }
}

impl<Type> Sub<Vector3D<Type>> for Vector3D<Type> {

    type Output = Vector3D<Type>;
    
    #[inline]fn sub(self, other: Vector3D<Type>) -> Self::Output {
        todo!();
        /*
            return { x - other.x, y - other.y, z - other.z };
        */
    }
}

impl<Type> Mul<&Type> for Vector3D<Type> {

    type Output = Vector3D<Type>;
    
    #[inline] fn mul(self, other: &Type) -> Self::Output {
        todo!();
        /*
            return { x * scaleFactor, y * scaleFactor, z * scaleFactor };
        */
    }
}

impl<Type> Div<Type> for Vector3D<Type> {

    type Output = Vector3D<Type>;
    
    #[inline] fn div(self, other: Type) -> Self::Output {
        todo!();
        /*
            return { x / scaleFactor, y / scaleFactor, z / scaleFactor };
        */
    }
}

impl<Type> Neg for Vector3D<Type> {

    type Output = Self;
    
    #[inline] fn neg(self) -> Self::Output {
        todo!();
        /*
            return { -x, -y, -z };
        */
    }
}

impl<Type> Mul<&Vector3D<Type>> for Vector3D<Type> {

    type Output = Type;

    /**
      | Returns the dot-product of these two
      | vectors.
      |
      */
    #[inline] fn mul(self, other: &Vector3D<Type>) -> Self::Output {
        todo!();
        /*
            return x * other.x + y * other.y + z * other.z;
        */
    }
}

impl<Type> BitXor<Vector3D<Type>> for Vector3D<Type> {

    type Output = Vector3D<Type>;
    
    /**
      | Returns the cross-product of these
      | two vectors.
      |
      */
    #[inline] fn bitxor(self, other: Vector3D<Type>) -> Self::Output {
        todo!();
        /*
            return { y * other.z - z * other.y, z * other.x - x * other.z, x * other.y - y * other.x };
        */
    }
}

impl<Type> Vector3D<Type> {

    pub fn new_from_xyz(
        x_value: Type,
        y_value: Type,
        z_value: Type) -> Self {
    
        todo!();
        /*
        : x(xValue),
        : y(yValue),
        : z(zValue),

        
        */
    }
    
    pub fn new_from_vec3d_ref(other: &Vector3D<Type>) -> Self {
    
        todo!();
        /*
        : x(other.x),
        : y(other.y),
        : z(other.z),

        
        */
    }
    
    pub fn assign_from(&mut self, other: Vector3D<Type>) -> &mut Vector3D<Type> {
        
        todo!();
        /*
            x = other.x;  y = other.y;  z = other.z; return *this;
        */
    }

    /**
      | Returns a vector that lies along the
      | X axis.
      |
      */
    pub fn x_axis() -> Vector3D<Type> {
        
        todo!();
        /*
            return { (Type) 1, 0, 0 };
        */
    }

    /**
      | Returns a vector that lies along the
      | Y axis.
      |
      */
    pub fn y_axis() -> Vector3D<Type> {
        
        todo!();
        /*
            return { 0, (Type) 1, 0 };
        */
    }

    /**
      | Returns a vector that lies along the
      | Z axis.
      |
      */
    pub fn z_axis() -> Vector3D<Type> {
        
        todo!();
        /*
            return { 0, 0, (Type) 1 };
        */
    }
    
    pub fn length(&self) -> Type {
        
        todo!();
        /*
            return std::sqrt (lengthSquared());
        */
    }
    
    pub fn length_squared(&self) -> Type {
        
        todo!();
        /*
            return x * x + y * y + z * z;
        */
    }
    
    pub fn normalised(&self) -> Vector3D<Type> {
        
        todo!();
        /*
            return *this / length();
        */
    }

    /**
      | Returns true if the vector is practically
      | equal to the origin.
      |
      */
    pub fn length_is_below_epsilon(&self) -> bool {
        
        todo!();
        /*
            auto epsilon = std::numeric_limits<Type>::epsilon();
            return ! (x < -epsilon || x > epsilon || y < -epsilon || y > epsilon || z < -epsilon || z > epsilon);
        */
    }
}
