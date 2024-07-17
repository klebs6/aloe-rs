crate::ix!();

pub trait PreviewDisplayListener
{
    fn preview_display_ready(&mut self);
    fn preview_display_about_to_be_destroyed(&mut self);
}

#[no_copy]
pub struct PreviewDisplay<'a> {
    listeners:                             ListenerList<Box<dyn PreviewDisplayListener>>,
    texture_view_surface_texture_listener: TextureViewSurfaceTextureListener<'a>,
    texture_view:                          GlobalRef,
    width:                                 i32, // default = -1
    height:                                i32, // default = -1
    buffer_width:                          i32,
    buffer_height:                         i32,
}

impl<'a> TextureViewSurfaceTextureListenerOwner for PreviewDisplay<'a> {

    fn on_surface_texture_available(&mut self, 
        surface:       &mut LocalRef<jobject>,
        width_to_use:  i32,
        height_to_use: i32)  {
        
        todo!();
        /*
            ALOE_CAMERA_LOG ("onSurfaceTextureAvailable()");

                width = widthToUse;
                height = heightToUse;

                updateSurfaceTransform();

                listeners.call (&PreviewDisplayListener::previewDisplayReady);
        */
    }
    
    fn on_surface_texture_destroyed(&mut self, surface: &mut LocalRef<jobject>) -> bool {
        
        todo!();
        /*
            ALOE_CAMERA_LOG ("onSurfaceTextureDestroyed()");

                listeners.call (&PreviewDisplayListener::previewDisplayAboutToBeDestroyed);

                return true;
        */
    }
    
    fn on_surface_texture_size_changed(&mut self, 
        surface:       &mut LocalRef<jobject>,
        width_to_use:  i32,
        height_to_use: i32)  {
        
        todo!();
        /*
            ALOE_CAMERA_LOG ("onSurfaceTextureSizeChanged()");

                width = widthToUse;
                height = heightToUse;

                updateSurfaceTransform();
        */
    }
    
    fn on_surface_texture_updated(&mut self, surface: &mut LocalRef<jobject>)  {
        
        todo!();
        /*
            ALOE_CAMERA_LOG ("onSurfaceTextureUpdated()");
        */
    }
}

impl<'a> Drop for PreviewDisplay<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            getEnv()->CallVoidMethod (textureView, AndroidTextureView.setSurfaceTextureListener, nullptr);
        */
    }
}

impl<'a> PreviewDisplay<'a> {

    pub fn new(buffer_size: Rectangle<i32>) -> Self {
    
        todo!();
        /*


            : textureViewSurfaceTextureListener (*this),
                  textureView (LocalRef<jobject> (getEnv()->NewObject (AndroidTextureView, AndroidTextureView.constructor,
                                                                       getAppContext().get()))),
                  bufferWidth (bufferSize.getWidth()),
                  bufferHeight (bufferSize.getHeight())

                auto* env = getEnv();

                if (! isReady())
                    env->CallVoidMethod (textureView, AndroidTextureView.setSurfaceTextureListener,
                                         CreateJavaInterface (&textureViewSurfaceTextureListener,
                                                              "android/view/TextureView$SurfaceTextureListener").get());
        */
    }
    
    pub fn add_listener(&mut self, l: *mut dyn PreviewDisplayListener)  {
        
        todo!();
        /*
            if (l == nullptr)
                {
                    jassertfalse;
                    return;
                }

                listeners.add (l);

                if (isReady())
                    l->previewDisplayReady();
        */
    }
    
    pub fn remove_listener(&mut self, l: *mut dyn PreviewDisplayListener)  {
        
        todo!();
        /*
            if (l == nullptr)
                {
                    jassertfalse;
                    return;
                }

                listeners.remove (l);
        */
    }
    
    pub fn is_ready(&self) -> bool {
        
        todo!();
        /*
            return (getEnv()->CallBooleanMethod (textureView, AndroidTextureView.isAvailable) != 0)
                        && width > 0 && height > 0;
        */
    }
    
    pub fn create_surface(&mut self) -> LocalRef<jobject> {
        
        todo!();
        /*
            // Surface may get destroyed while session is being configured, if
                // the preview gets hidden in the meantime, so bailout.
                if (! isReady())
                    return LocalRef<jobject> (nullptr);

                auto* env = getEnv();

                auto surfaceTexture = LocalRef<jobject> (env->CallObjectMethod (textureView,
                                                                                AndroidTextureView.getSurfaceTexture));

                // NB: too small buffer will result in pixelated preview. A buffer with wrong aspect ratio
                //     can result in a cropped preview.
                env->CallVoidMethod (surfaceTexture, AndroidSurfaceTexture.setDefaultBufferSize, (jint) bufferWidth, (jint) bufferHeight);

                auto surface = LocalRef<jobject> (env->NewObject (AndroidSurface, AndroidSurface.constructor, surfaceTexture.get()));

                return surface;
        */
    }
    
    pub fn get_native_view(&mut self) -> &GlobalRef {
        
        todo!();
        /*
            return textureView;
        */
    }
    
    pub fn update_surface_transform(&mut self)  {
        
        todo!();
        /*
            auto* env = getEnv();

                auto windowManager = LocalRef<jobject> (env->CallObjectMethod (getAppContext(), AndroidContext.getSystemService, javaString ("window").get()));
                auto display = LocalRef<jobject> (env->CallObjectMethod (windowManager, AndroidWindowManager.getDefaultDisplay));
                auto rotation = env->CallIntMethod (display, AndroidDisplay.getRotation);

                static constexpr int rotation90 = 1;
                static constexpr int rotation270 = 3;

                auto matrix = LocalRef<jobject> (env->NewObject (AndroidMatrix, AndroidMatrix.constructor));

                if (rotation == rotation90 || rotation == rotation270)
                {
                    env->CallBooleanMethod (matrix, AndroidMatrix.postScale, jfloat ((float) height / (float) width), jfloat ((float) width / (float) height), (jfloat) 0, (jfloat) 0);
                    env->CallBooleanMethod (matrix, AndroidMatrix.postRotate, (jfloat) 90 * ((float) rotation - 2), (jfloat) 0, (jfloat) 0);
                    env->CallBooleanMethod (matrix, AndroidMatrix.postTranslate, (jfloat) (rotation == 3 ? width : 0), (jfloat) (rotation == 1 ? height : 0));
                }

                env->CallVoidMethod (textureView, AndroidTextureView.setTransform, matrix.get());
        */
    }
    
}
