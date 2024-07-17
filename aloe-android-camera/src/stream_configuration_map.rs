crate::ix!();

pub type InternalOpenCameraResultCallback = fn(camera_id: &String, error: &String) -> ();

pub const ERROR_CAMERA_IN_USE:      usize = 1;
pub const ERROR_MAX_CAMERAS_IN_USE: usize = 2;
pub const ERROR_CAMERA_DISABLED:    usize = 3;
pub const ERROR_CAMERA_DEVICE:      usize = 4;
pub const ERROR_CAMERA_SERVICE:     usize = 5;

pub const STREAM_CONFIGURATION_MAP_JPEG_IMAGE_FORMAT: i32 = 256;

///-------------------------
pub struct StreamConfigurationMap {
    scaler_stream_configuration_map:        GlobalRef,
    supported_preview_output_sizes:         Vec<Rectangle<i32>>,
    supported_still_image_output_sizes:     Vec<Rectangle<i32>>,
    supported_video_recording_output_sizes: Vec<Rectangle<i32>>,
    default_preview_size:                   Rectangle<i32>,
    preview_buffer_size:                    Rectangle<i32>,
}

impl StreamConfigurationMap {

    pub fn new(camera_characteristics_to_use: &GlobalRef) -> Self {
    
        todo!();
        /*


            : scalerStreamConfigurationMap (getStreamConfigurationMap (cameraCharacteristicsToUse)),
                  supportedPreviewOutputSizes (retrieveOutputSizes (scalerStreamConfigurationMap,
                                                                    getClassForName ("android.graphics.SurfaceTexture"),
                                                                    -1)),
                  supportedStillImageOutputSizes (retrieveOutputSizes (scalerStreamConfigurationMap,
                                                                       LocalRef<jobject>(),
                                                                       jpegImageFormat)),
                  supportedVideoRecordingOutputSizes (retrieveOutputSizes (scalerStreamConfigurationMap,
                                                                           getClassForName ("android.media.MediaRecorder"),
                                                                           -1)),
                  defaultPreviewSize (getSmallestSize (supportedPreviewOutputSizes)),
                  previewBufferSize (getLargestSize (supportedPreviewOutputSizes))
                printSizesLog (supportedPreviewOutputSizes, "SurfaceTexture");
                printSizesLog (supportedStillImageOutputSizes, "JPEG");
                printSizesLog (supportedVideoRecordingOutputSizes, "MediaRecorder");
        */
    }
    
    pub fn get_supported_preview_output_sizes(&self) -> Vec<Rectangle<i32>> {
        
        todo!();
        /*
            return supportedPreviewOutputSizes;
        */
    }
    
    pub fn get_supported_still_image_output_sizes(&self) -> Vec<Rectangle<i32>> {
        
        todo!();
        /*
            return supportedStillImageOutputSizes;
        */
    }
    
    pub fn get_supported_video_recording_output_sizes(&self) -> Vec<Rectangle<i32>> {
        
        todo!();
        /*
            return supportedVideoRecordingOutputSizes;
        */
    }
    
    pub fn get_default_preview_size(&self) -> Rectangle<i32> {
        
        todo!();
        /*
            return defaultPreviewSize;
        */
    }
    
    pub fn get_preview_buffer_size(&self) -> Rectangle<i32> {
        
        todo!();
        /*
            return previewBufferSize;
        */
    }
    
    pub fn is_output_supported_for_surface(&self, surface: &LocalRef<jobject>) -> bool {
        
        todo!();
        /*
            return getEnv()->CallBooleanMethod (scalerStreamConfigurationMap, AndroidStreamConfigurationMap.isOutputSupportedForSurface, surface.get()) != 0;
        */
    }
    
    pub fn get_stream_configuration_map(&mut self, camera_characteristics_to_use: &GlobalRef) -> GlobalRef {
        
        todo!();
        /*
            auto* env = getEnv();

                auto scalerStreamConfigurationMapKey = LocalRef<jobject> (env->GetStaticObjectField (CameraCharacteristics,
                                                                                                     CameraCharacteristics.SCALER_STREAM_CONFIGURATION_MAP));

                return GlobalRef (LocalRef<jobject> (env->CallObjectMethod (cameraCharacteristicsToUse,
                                                                            CameraCharacteristics.get,
                                                                            scalerStreamConfigurationMapKey.get())));
        */
    }
    
    pub fn retrieve_output_sizes(
        scaler_stream_configuration_map: &mut GlobalRef,
        output_class:                    &LocalRef<jobject>,
        format:                          i32) -> Vec<Rectangle<i32>> {
        
        todo!();
        /*
            Vec<Rectangle<int>> result;

                auto* env = getEnv();

                auto outputSizes = outputClass.get() != nullptr
                                 ? LocalRef<jobjectArray> ((jobjectArray) env->CallObjectMethod (scalerStreamConfigurationMap,
                                                                                                 AndroidStreamConfigurationMap.getOutputSizesForClass,
                                                                                                 outputClass.get()))
                                 : LocalRef<jobjectArray> ((jobjectArray) env->CallObjectMethod (scalerStreamConfigurationMap,
                                                                                                 AndroidStreamConfigurationMap.getOutputSizesForFormat,
                                                                                                 (jint) format));

                if (format != -1)
                {
                    auto supported = (env->CallBooleanMethod (scalerStreamConfigurationMap, AndroidStreamConfigurationMap.isOutputSupportedFor, (jint) format) != 0);

                    if (! supported)
                    {
                        // The output format is not supported by this device, still image capture will not work!
                        jassertfalse;
                        return {};
                    }
                }

                int numSizes = env->GetArrayLength (outputSizes);

                jassert (numSizes > 0);

                for (int i = 0; i < numSizes; ++i)
                {
                    auto size = LocalRef<jobject> (env->GetObjectArrayElement (outputSizes, i));

                    auto width  = env->CallIntMethod (size, AndroidSize.getWidth);
                    auto height = env->CallIntMethod (size, AndroidSize.getHeight);

                    result.add (Rectangle<int> (0, 0, width, height));
                }

                return result;
        */
    }
    
    pub fn get_class_for_name(name: &String) -> LocalRef<jobject> {
        
        todo!();
        /*
            return LocalRef<jobject> (getEnv()->CallStaticObjectMethod (JavaClass, JavaClass.forName,
                                                                            javaString (name).get()));
        */
    }
    
    pub fn print_sizes_log(
        sizes:      &[Rectangle<i32>],
        class_name: &String)  {
        
        todo!();
        /*
            ignoreUnused (sizes, className);

                ALOE_CAMERA_LOG ("Sizes for class " + className);

              #if ALOE_CAMERA_LOG_ENABLED
                for (auto& s : sizes)
                    ALOE_CAMERA_LOG (s.toString() + "\n");
              #endif
        */
    }
    
    pub fn get_smallest_size(&self, sizes: &[Rectangle<i32>]) -> Rectangle<i32> {
        
        todo!();
        /*
            if (sizes.size() == 0)
                    return {};

                auto smallestSize = sizes[0];

                for (auto& size : sizes)
                {
                    if (size.getWidth() < smallestSize.getWidth() && size.getHeight() < smallestSize.getHeight())
                        smallestSize = size;
                }

                return smallestSize;
        */
    }
    
    pub fn get_largest_size(&self, sizes: &[Rectangle<i32>]) -> Rectangle<i32> {
        
        todo!();
        /*
            if (sizes.size() == 0)
                    return {};

                auto largestSize = sizes[0];

                for (auto& size : sizes)
                {
                    if (size.getWidth() > largestSize.getWidth() && size.getHeight() > largestSize.getHeight())
                        largestSize = size;
                }

                return largestSize;
        */
    }
}
