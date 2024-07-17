crate::ix!();

/**
  | Holds either a simple integer translation,
  | or an affine transform.
  | 
  | @tags{Graphics}
  |
  */
#[derive(Default)]
pub struct TranslationOrTransform {
    complex_transform:  AffineTransform,
    offset:             Point<i32>,
    is_only_translated: bool, // default = true
    is_rotated:         bool, // default = false
}

impl TranslationOrTransform {

    pub fn new(origin: Point<i32>) -> Self {
    
        todo!();
        /*
        : offset(origin),
        */
    }
    
    pub fn get_transform(&self) -> AffineTransform {
        
        todo!();
        /*
            return isOnlyTranslated ? AffineTransform::translation (offset)
                                    : complexTransform;
        */
    }
    
    pub fn get_transform_with(&self, user_transform: &AffineTransform) -> AffineTransform {
        
        todo!();
        /*
            return isOnlyTranslated ? userTransform.translated (offset)
                                    : userTransform.followedBy (complexTransform);
        */
    }
    
    pub fn is_identity(&self) -> bool {
        
        todo!();
        /*
            return isOnlyTranslated && offset.isOrigin();
        */
    }
    
    pub fn set_origin(&mut self, delta: Point<i32>)  {
        
        todo!();
        /*
            if (isOnlyTranslated)
                offset += delta;
            else
                complexTransform = AffineTransform::translation (delta)
                                                   .followedBy (complexTransform);
        */
    }
    
    pub fn add_transform(&mut self, t: &AffineTransform)  {
        
        todo!();
        /*
            if (isOnlyTranslated && t.isOnlyTranslation())
            {
                auto tx = (int) (t.getTranslationX() * 256.0f);
                auto ty = (int) (t.getTranslationY() * 256.0f);

                if (((tx | ty) & 0xf8) == 0)
                {
                    offset += Point<int> (tx >> 8, ty >> 8);
                    return;
                }
            }

            complexTransform = getTransformWith (t);
            isOnlyTranslated = false;
            isRotated = (complexTransform.mat01 != 0.0f || complexTransform.mat10 != 0.0f
                          || complexTransform.mat00 < 0 || complexTransform.mat11 < 0);
        */
    }
    
    pub fn get_physical_pixel_scale_factor(&self) -> f32 {
        
        todo!();
        /*
            return isOnlyTranslated ? 1.0f : std::sqrt (std::abs (complexTransform.getDeterminant()));
        */
    }
    
    pub fn move_origin_in_device_space(&mut self, delta: Point<i32>)  {
        
        todo!();
        /*
            if (isOnlyTranslated)
                offset += delta;
            else
                complexTransform = complexTransform.translated (delta);
        */
    }
    
    pub fn translated_i32(&self, r: Rectangle<i32>) -> Rectangle<i32> {
        
        todo!();
        /*
            jassert (isOnlyTranslated);
            return r + offset;
        */
    }
    
    pub fn translated(&self, r: Rectangle<f32>) -> Rectangle<f32> {
        
        todo!();
        /*
            jassert (isOnlyTranslated);
            return r + offset.toFloat();
        */
    }
    
    pub fn transformed<RectangleOrPoint>(&self, r: RectangleOrPoint) -> RectangleOrPoint {
    
        todo!();
        /*
            jassert (! isOnlyTranslated);
            return r.transformedBy (complexTransform);
        */
    }
    
    
    pub fn device_space_to_user_space<Type: Copy>(&self, r: Rectangle<Type>) -> Rectangle<Type> {
    
        todo!();
        /*
            return isOnlyTranslated ? r - offset
                                    : r.transformedBy (complexTransform.inverted());
        */
    }
}

