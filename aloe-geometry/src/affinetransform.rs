crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/geometry/aloe_AffineTransform.h]

/**
  | Represents a 2D affine-transformation
  | matrix.
  | 
  | An affine transformation is a transformation
  | such as a rotation, scale, shear, resize
  | or translation.
  | 
  | These are used for various 2D transformation
  | tasks, e.g. with Path objects.
  | 
  | @see Path, Point, Line
  | 
  | @tags{Graphics}
  |
  */
#[derive(Clone,Copy)]
pub struct AffineTransform {
    
    /* 
       | The transform matrix is:
       |
       |    (mat00 mat01 mat02)
       |    (mat10 mat11 mat12)
       |    (  0     0     1  )
       */
    mat00: f32,
    mat01: f32,
    mat02: f32,
    mat10: f32,
    mat11: f32,
    mat12: f32,
}

impl Default for AffineTransform {
    
    /**
      | Creates an identity transform.
      |
      */
    fn default() -> Self {
        Self {
            mat00:  1.0,
            mat01:  0.0,
            mat02:  0.0,
            mat10:  0.0,
            mat11:  1.0,
            mat12:  0.0,
        }
    }
}

impl PartialEq<AffineTransform> for AffineTransform {
    
    /**
      | Compares two transforms.
      |
      */
    #[inline] fn eq(&self, other: &AffineTransform) -> bool {
        todo!();
        /*
            return mat00 == other.mat00
            && mat01 == other.mat01
            && mat02 == other.mat02
            && mat10 == other.mat10
            && mat11 == other.mat11
            && mat12 == other.mat12;
        */
    }
}

impl Eq for AffineTransform {}

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/geometry/aloe_AffineTransform.cpp]
impl AffineTransform {

    /**
      | Transforms a 2D coordinate using this
      | matrix.
      |
      */
    pub fn transform_point<ValueType>(&self, 
        x: &mut ValueType,
        y: &mut ValueType)  {
    
        todo!();
        /*
            auto oldX = x;
            x = static_cast<ValueType> (mat00 * oldX + mat01 * y + mat02);
            y = static_cast<ValueType> (mat10 * oldX + mat11 * y + mat12);
        */
    }

    /**
      | Transforms two 2D coordinates using
      | this matrix.
      | 
      | This is just a shortcut for calling transformPoint()
      | on each of these pairs of coordinates
      | in turn. (And putting all the calculations
      | into one function hopefully also gives
      | the compiler a bit more scope for pipelining
      | it).
      |
      */
    pub fn transform_two_2d_coordinates<ValueType>(&self, 
        x1: &mut ValueType,
        y1: &mut ValueType,
        x2: &mut ValueType,
        y2: &mut ValueType)  {
    
        todo!();
        /*
            auto oldX1 = x1, oldX2 = x2;
            x1 = static_cast<ValueType> (mat00 * oldX1 + mat01 * y1 + mat02);
            y1 = static_cast<ValueType> (mat10 * oldX1 + mat11 * y1 + mat12);
            x2 = static_cast<ValueType> (mat00 * oldX2 + mat01 * y2 + mat02);
            y2 = static_cast<ValueType> (mat10 * oldX2 + mat11 * y2 + mat12);
        */
    }

    /**
      | Transforms three 2D coordinates using
      | this matrix.
      | 
      | This is just a shortcut for calling transformPoint()
      | on each of these pairs of coordinates
      | in turn. (And putting all the calculations
      | into one function hopefully also gives
      | the compiler a bit more scope for pipelining
      | it).
      |
      */
    pub fn transform_three_2d_coordinates<ValueType>(&self, 
        x1: &mut ValueType,
        y1: &mut ValueType,
        x2: &mut ValueType,
        y2: &mut ValueType,
        x3: &mut ValueType,
        y3: &mut ValueType)  {
    
        todo!();
        /*
            auto oldX1 = x1, oldX2 = x2, oldX3 = x3;
            x1 = static_cast<ValueType> (mat00 * oldX1 + mat01 * y1 + mat02);
            y1 = static_cast<ValueType> (mat10 * oldX1 + mat11 * y1 + mat12);
            x2 = static_cast<ValueType> (mat00 * oldX2 + mat01 * y2 + mat02);
            y2 = static_cast<ValueType> (mat10 * oldX2 + mat11 * y2 + mat12);
            x3 = static_cast<ValueType> (mat00 * oldX3 + mat01 * y3 + mat02);
            y3 = static_cast<ValueType> (mat10 * oldX3 + mat11 * y3 + mat12);
        */
    }

    /**
      | Returns a new transform which is the
      | same as this one followed by a translation.
      |
      */
    pub fn translated_by_point<PointType>(&self, delta: PointType) -> AffineTransform {
    
        todo!();
        /*
            return translated ((float) delta.x, (float) delta.y);
        */
    }

    /**
      | Returns a new transform which is a translation.
      |
      */
    pub fn translation_by_delta<PointType>(delta: PointType) -> AffineTransform {
    
        todo!();
        /*
            return translation ((float) delta.x, (float) delta.y);
        */
    }

    /**
      | Returns the transform that will map
      | three specified points onto three target
      | points.
      |
      */
    pub fn from_target_points_with_point_types<PointType>(
        source1: PointType,
        target1: PointType,
        source2: PointType,
        target2: PointType,
        source3: PointType,
        target3: PointType) -> AffineTransform {
    
        todo!();
        /*
            return fromTargetPoints (source1.x, source1.y, target1.x, target1.y,
                                     source2.x, source2.y, target2.x, target2.y,
                                     source3.x, source3.y, target3.x, target3.y);
        */
    }

    /**
      | If this transform is only a translation,
      | this returns the X offset. @see isOnlyTranslation
      |
      */
    pub fn get_translationx(&self) -> f32 {
        
        todo!();
        /*
            return mat02;
        */
    }

    /**
      | If this transform is only a translation,
      | this returns the X offset. @see isOnlyTranslation
      |
      */
    pub fn get_translationy(&self) -> f32 {
        
        todo!();
        /*
            return mat12;
        */
    }

    /** 
      | Creates a transform from a set of raw matrix
      | values.
      |
      |   The resulting matrix is:
      |
      |       (mat00 mat01 mat02)
      |       (mat10 mat11 mat12)
      |       (  0     0     1  )
      */
    pub fn new(
        m00: f32,
        m01: f32,
        m02: f32,
        m10: f32,
        m11: f32,
        m12: f32) -> Self {
    
        todo!();
        /*
        : mat00(m00),
        : mat01(m01),
        : mat02(m02),
        : mat10(m10),
        : mat11(m11),
        : mat12(m12),

        
        */
    }
    
    /**
      | Returns true if this transform has no
      | effect on points.
      |
      */
    pub fn is_identity(&self) -> bool {
        
        todo!();
        /*
            return mat01 == 0.0f
            && mat02 == 0.0f
            && mat10 == 0.0f
            && mat12 == 0.0f
            && mat00 == 1.0f
            && mat11 == 1.0f;
        */
    }
    
    /**
      | Returns the result of concatenating
      | another transformation after this
      | one.
      |
      */
    pub fn followed_by(&self, other: &AffineTransform) -> AffineTransform {
        
        todo!();
        /*
            return { other.mat00 * mat00 + other.mat01 * mat10,
                 other.mat00 * mat01 + other.mat01 * mat11,
                 other.mat00 * mat02 + other.mat01 * mat12 + other.mat02,
                 other.mat10 * mat00 + other.mat11 * mat10,
                 other.mat10 * mat01 + other.mat11 * mat11,
                 other.mat10 * mat02 + other.mat11 * mat12 + other.mat12 };
        */
    }
    
    /**
      | Returns a new transform which is the
      | same as this one followed by a translation.
      |
      */
    pub fn translated(&self, dx: f32, dy: f32) -> AffineTransform {
        
        todo!();
        /*
            return { mat00, mat01, mat02 + dx,
                 mat10, mat11, mat12 + dy };
        */
    }
    
    /**
      | Returns a new transform which is a translation.
      |
      */
    pub fn translation(&mut self, dx: f32, dy: f32) -> AffineTransform {
        
        todo!();
        /*
            return { 1.0f, 0.0f, dx,
                 0.0f, 1.0f, dy };
        */
    }
    
    /**
      | Returns a copy of this transform with
      | the specified translation matrix values.
      |
      */
    pub fn with_absolute_translation(&self, tx: f32, ty: f32) -> AffineTransform {
        
        todo!();
        /*
            return { mat00, mat01, tx,
                 mat10, mat11, ty };
        */
    }
    
    /**
      | Returns a transform which is the same
      | as this one followed by a rotation.
      | 
      | The rotation is specified by a number
      | of radians to rotate clockwise, centred
      | around the origin (0, 0).
      |
      */
    pub fn rotated_about_origin(&self, rad: f32) -> AffineTransform {
        
        todo!();
        /*
            auto cosRad = std::cos (rad);
        auto sinRad = std::sin (rad);

        return { cosRad * mat00 - sinRad * mat10,
                 cosRad * mat01 - sinRad * mat11,
                 cosRad * mat02 - sinRad * mat12,
                 sinRad * mat00 + cosRad * mat10,
                 sinRad * mat01 + cosRad * mat11,
                 sinRad * mat02 + cosRad * mat12 };
        */
    }
    
    /**
      | Returns a new transform which is a rotation
      | about (0, 0).
      |
      */
    pub fn rotation_about_the_origin(&mut self, rad: f32) -> AffineTransform {
        
        todo!();
        /*
            auto cosRad = std::cos (rad);
        auto sinRad = std::sin (rad);

        return { cosRad, -sinRad, 0,
                 sinRad,  cosRad, 0 };
        */
    }
    
    /**
      | Returns a new transform which is a rotation
      | about a given point.
      |
      */
    pub fn rotation_about_point(
        &mut self, 
        rad:    f32,
        pivotx: f32,
        pivoty: f32) -> AffineTransform {
        
        todo!();
        /*
            auto cosRad = std::cos (rad);
        auto sinRad = std::sin (rad);

        return { cosRad, -sinRad, -cosRad * pivotX +  sinRad * pivotY + pivotX,
                 sinRad,  cosRad, -sinRad * pivotX + -cosRad * pivotY + pivotY };
        */
    }
    
    /**
      | Returns a transform which is the same
      | as this one followed by a rotation about
      | a given point.
      | 
      | The rotation is specified by a number
      | of radians to rotate clockwise, centred
      | around the coordinates passed in.
      |
      */
    pub fn rotated_about_pivot(
        &self, 
        angle:  f32,
        pivotx: f32,
        pivoty: f32) -> AffineTransform {
        
        todo!();
        /*
            return followedBy (rotation (angle, pivotX, pivotY));
        */
    }
    
    /**
      | Returns a transform which is the same
      | as this one followed by a re-scaling.
      | The scaling is centred around the origin
      | (0, 0).
      |
      */
    pub fn scaled_by_x_and_y_factor(
        &self, 
        factorx: f32,
        factory: f32) -> AffineTransform {
        
        todo!();
        /*
            return { factorX * mat00, factorX * mat01, factorX * mat02,
                 factorY * mat10, factorY * mat11, factorY * mat12 };
        */
    }
    
    /**
      | Returns a transform which is the same
      | as this one followed by a re-scaling.
      | The scaling is centred around the origin
      | (0, 0).
      |
      */
    pub fn scaled_by_factor(&self, factor: f32) -> AffineTransform {
        
        todo!();
        /*
            return { factor * mat00, factor * mat01, factor * mat02,
                 factor * mat10, factor * mat11, factor * mat12 };
        */
    }
    
    /**
      | Returns a new transform which is a re-scale
      | about the origin.
      |
      */
    pub fn scale_by_x_and_y_factors(&mut self, 
        factorx: f32,
        factory: f32) -> AffineTransform {
        
        todo!();
        /*
            return { factorX, 0, 0, 0, factorY, 0 };
        */
    }
    
    /**
      | Returns a new transform which is a re-scale
      | about the origin.
      |
      */
    pub fn scale_by_factor(&mut self, factor: f32) -> AffineTransform {
        
        todo!();
        /*
            return { factor, 0, 0, 0, factor, 0 };
        */
    }
    
    /**
      | Returns a transform which is the same
      | as this one followed by a re-scaling.
      | The scaling is centred around the origin
      | provided.
      |
      */
    pub fn scaled_by_x_and_y_factor_with_pivots(
        &self, 
        factorx: f32,
        factory: f32,
        pivotx:  f32,
        pivoty:  f32) -> AffineTransform {
        
        todo!();
        /*
            return { factorX * mat00, factorX * mat01, factorX * mat02 + pivotX * (1.0f - factorX),
                 factorY * mat10, factorY * mat11, factorY * mat12 + pivotY * (1.0f - factorY) };
        */
    }
    
    /**
      | Returns a new transform which is a re-scale
      | centred around the point provided.
      |
      */
    pub fn scale_by_x_and_y_factors_with_pivots(
        &mut self, 
        factorx: f32,
        factory: f32,
        pivotx:  f32,
        pivoty:  f32) -> AffineTransform {
        
        todo!();
        /*
            return { factorX, 0, pivotX * (1.0f - factorX),
                 0, factorY, pivotY * (1.0f - factorY) };
        */
    }
    
    /**
      | Returns a shear transform, centred
      | around the origin (0, 0).
      |
      */
    pub fn shear(&mut self, 
        shearx: f32,
        sheary: f32) -> AffineTransform {
        
        todo!();
        /*
            return { 1.0f,   shearX, 0,
                 shearY, 1.0f,   0 };
        */
    }
    
    /**
      | Returns a transform which is the same
      | as this one followed by a shear. The shear
      | is centred around the origin (0, 0).
      |
      */
    pub fn sheared(&self, 
        shearx: f32,
        sheary: f32) -> AffineTransform {
        
        todo!();
        /*
            return { mat00 + shearX * mat10,
                 mat01 + shearX * mat11,
                 mat02 + shearX * mat12,
                 mat10 + shearY * mat00,
                 mat11 + shearY * mat01,
                 mat12 + shearY * mat02 };
        */
    }
    
    /**
      | Returns a transform that will flip coordinates
      | vertically within a window of the given
      | height. This is handy for converting
      | between upside-down coordinate systems
      | such as OpenGL or CoreGraphics.
      |
      */
    pub fn vertical_flip(&mut self, height: f32) -> AffineTransform {
        
        todo!();
        /*
            return { 1.0f,  0.0f, 0.0f,
                 0.0f, -1.0f, height };
        */
    }
    
    /**
      | Returns a matrix which is the inverse
      | operation of this one.
      | 
      | Some matrices don't have an inverse
      | - in this case, the method will just return
      | an identity transform.
      |
      */
    pub fn inverted(&self) -> AffineTransform {
        
        todo!();
        /*
            double determinant = getDeterminant();

        if (! approximatelyEqual (determinant, 0.0))
        {
            determinant = 1.0 / determinant;

            auto dst00 = (float) ( mat11 * determinant);
            auto dst10 = (float) (-mat10 * determinant);
            auto dst01 = (float) (-mat01 * determinant);
            auto dst11 = (float) ( mat00 * determinant);

            return { dst00, dst01, -mat02 * dst00 - mat12 * dst01,
                     dst10, dst11, -mat02 * dst10 - mat12 * dst11 };
        }

        // singularity..
        return *this;
        */
    }
    
    /**
      | Returns true if this transform maps
      | to a singularity - i.e. if it has no inverse.
      |
      */
    pub fn is_singularity(&self) -> bool {
        
        todo!();
        /*
            return (mat00 * mat11 - mat10 * mat01) == 0.0f;
        */
    }
    
    /**
      | Returns the transform that will map
      | three known points onto three coordinates
      | that are supplied.
      | 
      | This returns the transform that will
      | transform (0, 0) into (x00, y00), (1,
      | 0) to (x10, y10), and (0, 1) to (x01, y01).
      |
      */
    pub fn from_target_points(
        &mut self, 
        x00: f32,
        y00: f32,
        x10: f32,
        y10: f32,
        x01: f32,
        y01: f32) -> AffineTransform {
        
        todo!();
        /*
            return { x10 - x00, x01 - x00, x00,
                 y10 - y00, y01 - y00, y00 };
        */
    }
    
    /**
      | Returns the transform that will map
      | three specified points onto three target
      | points.
      |
      */
    pub fn from_target_points_mapped(
        &mut self, 
        sx1: f32,
        sy1: f32,
        tx1: f32,
        ty1: f32,
        sx2: f32,
        sy2: f32,
        tx2: f32,
        ty2: f32,
        sx3: f32,
        sy3: f32,
        tx3: f32,
        ty3: f32) -> AffineTransform {
        
        todo!();
        /*
            return fromTargetPoints (sx1, sy1, sx2, sy2, sx3, sy3)
                .inverted()
                .followedBy (fromTargetPoints (tx1, ty1, tx2, ty2, tx3, ty3));
        */
    }
    
    /**
      | Returns true if the transform only translates,
      | and doesn't scale or rotate the points.
      |
      */
    pub fn is_only_translation(&self) -> bool {
        
        todo!();
        /*
            return mat01 == 0.0f
            && mat10 == 0.0f
            && mat00 == 1.0f
            && mat11 == 1.0f;
        */
    }
    
    /**
      | Returns the determinant of the transform.
      |
      */
    pub fn get_determinant(&self) -> f32 {
        
        todo!();
        /*
            return (mat00 * mat11) - (mat01 * mat10);
        */
    }
    
    pub fn get_scale_factor(&self) -> f32 {
        
        todo!();
        /*
            return (std::abs (mat00) + std::abs (mat11)) / 2.0f;
        */
    }
}

///----------------------
#[cfg(ALOE_UNIT_TESTS)]
pub struct AffineTransformTests {
    base: UnitTest,
}

#[cfg(ALOE_UNIT_TESTS)]
impl Default for AffineTransformTests {
    
    fn default() -> Self {
        todo!();
        /*


            : UnitTest ("AffineTransform", UnitTestCategories::maths
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
impl AffineTransformTests {

    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            beginTest ("Determinant");
            {
                constexpr float scale1 = 1.5f, scale2 = 1.3f;

                auto transform = AffineTransform::scale (scale1)
                                                 .followedBy (AffineTransform::rotation (degreesToRadians (72.0f)))
                                                 .followedBy (AffineTransform::translation (100.0f, 20.0f))
                                                 .followedBy (AffineTransform::scale (scale2));

                expect (approximatelyEqual (std::sqrt (std::abs (transform.getDeterminant())), scale1 * scale2));
            }
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
lazy_static!{
    /*
    static AffineTransformTests timeTests;
    */
}
