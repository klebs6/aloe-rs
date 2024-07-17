crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/native/aloe_android_GraphicsContext.cpp]

pub mod graphics_helpers {

    use super::*;

#[cfg(target_os="android")]
    pub fn create_paint(quality: GraphicsResamplingQuality) -> LocalRef<jobject> {
        
        todo!();
            /*
                jint constructorFlags = 1 /*ANTI_ALIAS_FLAG*/
                                        | 4 /*DITHER_FLAG*/
                                        | 128 /*SUBPIXEL_TEXT_FLAG*/;

                if (quality > typename Graphics::lowResamplingQuality)
                    constructorFlags |= 2; /*FILTER_BITMAP_FLAG*/

                return LocalRef<jobject>(getEnv()->NewObject (AndroidPaint, AndroidPaint.constructor, constructorFlags));
            */
    }

#[cfg(target_os="android")]
    pub fn create_matrix(
            env: *mut JNIEnv,
            t:   &AffineTransform) -> LocalRef<jobject> {
        
        todo!();
            /*
                auto m = LocalRef<jobject>(env->NewObject (AndroidMatrix, AndroidMatrix.constructor));

                jfloat values[9] = { t.mat00, t.mat01, t.mat02,
                                     t.mat10, t.mat11, t.mat12,
                                     0.0f, 0.0f, 1.0f };

                jfloatArray javaArray = env->NewFloatArray (9);
                env->SetFloatArrayRegion (javaArray, 0, 9, values);

                env->CallVoidMethod (m, AndroidMatrix.setValues, javaArray);
                env->DeleteLocalRef (javaArray);

                return m;
            */
    }
}

#[cfg(target_os="android")]
impl NativeImageType {

    pub fn create(&self, 
        format:      ImagePixelFormat,
        width:       i32,
        height:      i32,
        clear_image: bool) -> ImagePixelDataPtr {
        
        todo!();
        /*
            return SoftwareImageType().create (format, width, height, clearImage);
        */
    }
}
