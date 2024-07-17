crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_opengl/geometry/aloe_Quaternion.h]

/**
  | Holds a quaternion (a 3D vector and a
  | scalar value).
  | 
  | @tags{OpenGL}
  |
  */
pub struct Quaternion<Type> {

    /**
      | The vector part of the quaternion.
      |
      */
    vector: Vector3D<Type>,

    /**
      | The scalar part of the quaternion.
      |
      */
    scalar: Type,
}

impl<Type> Default for Quaternion<Type> {
    
    fn default() -> Self {
        todo!();
        /*
        : scalar(),

        
        */
    }
}

impl<Type> MulAssign<Quaternion<Type>> for Quaternion<Type> {
    
    #[inline] fn mul_assign(&mut self, other: Quaternion<Type>) {
        todo!();
        /*
            const Type oldScalar (scalar);
            scalar = (scalar * other.scalar) - (vector * other.vector);
            vector = (other.vector * oldScalar) + (vector * other.scalar) + (vector ^ other.vector);
            return *this;
        */
    }
}

impl<Type> Quaternion<Type> {

    pub fn new(other: &Quaternion<Type>) -> Self {
    
        todo!();
        /*
        : vector(other.vector),
        : scalar(other.scalar),

        
        */
    }
    
    pub fn new_from_vector_part_and_scalar_part(
        vector_part: Vector3D<Type>,
        scalar_part: Type

    ) -> Self {
    
        todo!();
        /*
        : vector(vectorPart),
        : scalar(scalarPart),

        
        */
    }
    
    pub fn new_xyzw(
        x: Type,
        y: Type,
        z: Type,
        w: Type) -> Self {
    
        todo!();
        /*
        : vector(x, y, z),
        : scalar(w),

        
        */
    }

    /**
      | Creates a quaternion from an angle and
      | an axis.
      |
      */
    pub fn from_angle(
        angle: Type,
        axis:  Vector3D<Type>) -> Quaternion<Type> {
        
        todo!();
        /*
            return Quaternion (axis.normalised() * std::sin (angle / (Type) 2), std::cos (angle / (Type) 2));
        */
    }
    
    pub fn assign_from(&mut self, other: Quaternion<Type>) -> &mut Quaternion<Type> {
        
        todo!();
        /*
            vector = other.vector;
            scalar = other.scalar;
            return *this;
        */
    }
    
    pub fn length(&self) -> Type {
        
        todo!();
        /*
            return std::sqrt (normal());
        */
    }
    
    pub fn normal(&self) -> Type {
        
        todo!();
        /*
            return scalar * scalar + vector.lengthSquared();
        */
    }
    
    pub fn normalised(&self) -> Quaternion<Type> {
        
        todo!();
        /*
            const Type len (length());
            jassert (len > 0);
            return Quaternion (vector / len, scalar / len);
        */
    }

    /**
      | Returns the matrix that will perform
      | the rotation specified by this quaternion.
      |
      */
    pub fn get_rotation_matrix(&self) -> Matrix3D<Type> {
        
        todo!();
        /*
            const Type norm (normal());
            const Type s (norm > 0 ? ((Type) 2) / norm : 0);
            const Type xs (s * vector.x),  ys (s * vector.y),  zs (s * vector.z);
            const Type wx (xs * scalar),   wy (ys * scalar),   wz (zs * scalar);
            const Type xx (xs * vector.x), xy (ys * vector.x), xz (zs * vector.x);
            const Type yy (ys * vector.y), yz (zs * vector.y), zz (zs * vector.z);

            return Matrix3D<Type> (((Type) 1) - (yy + zz), xy - wz, xz + wy, 0,
                                   xy + wz, ((Type) 1) - (xx+ zz),  yz - wx, 0,
                                   xz - wy, yz + wx, ((Type) 1) - (xx + yy), 0,
                                   0, 0, 0, (Type) 1);
        */
    }
}
