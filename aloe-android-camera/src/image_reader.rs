crate::ix!();

pub const IMAGE_READER_NUM_IMAGES_TO_KEEP: i32 = 2;

pub type ByteArray<'a> = &'a [u8];

pub struct ImageBuffer<'a>
{
    byte_array: LocalRef<ByteArray<'a>>,
    size:       i32,
}

#[weak_referenceable]
pub struct ImageReader<'a> {
    owner:                       &'a mut CameraDeviceImpl<'a>,
    camera_sensor_orientation:   i32,
    image_reader:                GlobalRef,
    on_image_available_listener: ImageReaderOnImageAvailableListener<'a>,
    has_notified_listeners:      Atomic<i32>, // default = { 0  }
}

impl<'a> ImageReaderOnImageAvailableListenerOwner for ImageReader<'a> {

    fn on_image_available(&mut self, image_reader: &mut LocalRef<jobject>)  {
        
        todo!();
        /*
            ALOE_CAMERA_LOG ("onImageAvailable()");

                auto* env = getEnv();

                auto jImage = LocalRef<jobject> (env->CallObjectMethod (imageReader, AndroidImageReader.acquireLatestImage));

                if (jImage.get() == nullptr)
                    return;

                auto cameraLensFrontFacing = owner.getCameraLensFacing() == 0;

                // NB: could use sensor orientation here to get real-world orientation, but then the resulting
                //     image could not match the UI orientation.
                auto image = androidImageToAloeWithFixedOrientation (jImage, owner.deviceOrientationChangeListener.getDeviceOrientation(),
                                                                     Desktop::getInstance().getCurrentOrientation(),
                                                                     cameraLensFrontFacing,
                                                                     cameraSensorOrientation);

                env->CallVoidMethod (jImage, AndroidImage.close);

                owner.callListeners (image);

                // Android may take multiple pictures before it handles a request to stop.
                if (hasNotifiedListeners.compareAndSetBool (1, 0))
                    MessageManager::callAsync ([safeThis = WeakReference<ImageReader> { this }, image]() mutable
                    {
                        if (safeThis != nullptr)
                            safeThis->owner.notifyPictureTaken (image);
                    });
        */
    }
}

impl<'a> Drop for ImageReader<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            getEnv()->CallVoidMethod (imageReader, AndroidImageReader.close);
        */
    }
}

impl<'a> ImageReader<'a> {

    pub fn new(
        owner_to_use:                     &mut CameraDeviceImpl,
        handler_to_use:                   &mut GlobalRef,
        image_width:                      i32,
        image_height:                     i32,
        camera_sensor_orientation_to_use: i32) -> Self {
    
        todo!();
        /*


            : owner (ownerToUse),
                  cameraSensorOrientation (cameraSensorOrientationToUse),
                  imageReader (LocalRef<jobject> (getEnv()->CallStaticObjectMethod (AndroidImageReader, AndroidImageReader.newInstance,
                                                                                    imageWidth, imageHeight, StreamConfigurationMap::jpegImageFormat,
                                                                                    numImagesToKeep))),
                  onImageAvailableListener (*this)
                getEnv()->CallVoidMethod (imageReader, AndroidImageReader.setOnImageAvailableListener,
                                          CreateJavaInterface (&onImageAvailableListener,
                                                               "android/media/ImageReader$OnImageAvailableListener").get(),
                                          handlerToUse.get());
        */
    }
    
    pub fn get_surface(&self) -> LocalRef<jobject> {
        
        todo!();
        /*
            return LocalRef<jobject> (getEnv()->CallObjectMethod (imageReader, AndroidImageReader.getSurface));
        */
    }
    
    pub fn reset_notification_flag(&mut self)  {
        
        todo!();
        /*
            hasNotifiedListeners.set (0);
        */
    }
    
    pub fn android_image_to_aloe_with_fixed_orientation(
        android_image:                                &LocalRef<jobject>,
        device_orientation_from_accelerometer_sensor: DesktopDisplayOrientation,
        target_orientation:                           DesktopDisplayOrientation,
        camera_lens_front_facing:                     bool,
        camera_sensor_orientation:                    i32) -> Image {
        
        todo!();
        /*
            auto* env = getEnv();

                auto planes = LocalRef<jobjectArray> ((jobjectArray) env->CallObjectMethod (androidImage, AndroidImage.getPlanes));
                jassert (env->GetArrayLength (planes) > 0);

                auto plane = LocalRef<jobject> (env->GetObjectArrayElement (planes, 0));
                auto byteBuffer = LocalRef<jobject> (env->CallObjectMethod (plane, AndroidImagePlane.getBuffer));

                ImageBuffer correctedBuffer = getImageBufferWithCorrectedOrientationFrom (byteBuffer, deviceOrientationFromAccelerometerSensor,
                                                                                          targetOrientation, cameraLensFrontFacing, cameraSensorOrientation);

                jbyte* rawBytes = env->GetByteArrayElements (correctedBuffer.byteArray, nullptr);

                Image result = ImageFileFormat::loadFrom (rawBytes, (size_t) correctedBuffer.size);

                env->ReleaseByteArrayElements (correctedBuffer.byteArray, rawBytes, 0);

                return result;
        */
    }
    
    pub fn get_image_buffer_with_corrected_orientation_from(
        image_plane_buffer:                           &LocalRef<jobject>,
        device_orientation_from_accelerometer_sensor: DesktopDisplayOrientation,
        target_orientation:                           DesktopDisplayOrientation,
        camera_lens_front_facing:                     bool,
        camera_sensor_orientation:                    i32

    ) -> ImageBuffer<'a> {

        todo!();
        /*
            auto* env = getEnv();

                auto bufferSize = env->CallIntMethod (imagePlaneBuffer, JavaByteBuffer.remaining);
                auto byteArray = LocalRef<jbyteArray> (env->NewByteArray (bufferSize));
                env->CallObjectMethod (imagePlaneBuffer, JavaByteBuffer.get, byteArray.get());

                auto rotationAngle = getRotationAngle (deviceOrientationFromAccelerometerSensor, targetOrientation,
                                                      cameraLensFrontFacing, cameraSensorOrientation);

                if (rotationAngle == 0)
                {
                    // Nothing to do, just get the bytes
                    return { byteArray, bufferSize };
                }

                auto origBitmap = LocalRef<jobject> (env->CallStaticObjectMethod (AndroidBitmapFactory,
                                                                                  AndroidBitmapFactory.decodeByteArray,
                                                                                  byteArray.get(), (jint) 0, (jint) bufferSize));

                if (origBitmap == nullptr)
                {
                    // Nothing to do, just get the bytes
                    return { byteArray, bufferSize };
                }

                auto correctedBitmap = getBitmapWithCorrectOrientationFrom (origBitmap, rotationAngle);

                auto byteArrayOutputStream = LocalRef<jobject> (env->NewObject (ByteArrayOutputStream,
                                                                                ByteArrayOutputStream.constructor));

                auto jCompressFormatString = javaString ("JPEG");
                auto compressFormat = LocalRef<jobject> (env->CallStaticObjectMethod (AndroidBitmapCompressFormat,
                                                                                      AndroidBitmapCompressFormat.valueOf,
                                                                                      jCompressFormatString.get()));

                if (env->CallBooleanMethod (correctedBitmap, AndroidBitmap.compress, compressFormat.get(),
                                            (jint) 100, byteArrayOutputStream.get()) != 0)
                {
                    auto correctedByteArray = LocalRef<jbyteArray> ((jbyteArray) env->CallObjectMethod (byteArrayOutputStream,
                                                                                                        ByteArrayOutputStream.toByteArray));

                    int correctedByteArraySize = env->CallIntMethod (byteArrayOutputStream, ByteArrayOutputStream.size);

                    return { correctedByteArray, correctedByteArraySize };
                }

                jassertfalse;
                // fallback, return original bitmap
                return { byteArray, bufferSize };
        */
    }
    
    pub fn get_rotation_angle(
        device_orientation_from_accelerometer_sensor: DesktopDisplayOrientation,
        target_orientation:                           DesktopDisplayOrientation,
        camera_lens_front_facing:                     bool,
        camera_sensor_orientation:                    i32) -> i32 {
        
        todo!();
        /*
            auto isSensorOrientationHorizontal = deviceOrientationFromAccelerometerSensor == Desktop::rotatedAntiClockwise
                                                  || deviceOrientationFromAccelerometerSensor == Desktop::rotatedClockwise;

                if (cameraLensFrontFacing && isSensorOrientationHorizontal)
                {
                    // flip angles for front camera
                    return getRotationAngle (deviceOrientationFromAccelerometerSensor, targetOrientation, false, (cameraSensorOrientation + 180) % 360);
                }

                switch (targetOrientation)
                {
                    case Desktop::rotatedAntiClockwise:
                        return cameraSensorOrientation == 90 ? 0 : 180;
                    case Desktop::rotatedClockwise:
                        return cameraSensorOrientation == 90 ? 180 : 0;
                    case Desktop::upright:
                    case Desktop::upsideDown:
                        if ((targetOrientation == Desktop::upright && ! cameraLensFrontFacing)
                            || (targetOrientation == Desktop::upsideDown && cameraLensFrontFacing))
                        {
                            return cameraSensorOrientation;
                        }
                        else
                        {
                            if (deviceOrientationFromAccelerometerSensor == Desktop::upright || deviceOrientationFromAccelerometerSensor == Desktop::upsideDown)
                                return cameraSensorOrientation;
                            else
                                return (cameraSensorOrientation + 180) % 360;
                        }
                        break;
                    case Desktop::allOrientations:
                    default:
                        return 0;
                }
        */
    }
    
    pub fn get_bitmap_with_correct_orientation_from(
        orig_bitmap:    &mut LocalRef<jobject>,
        rotation_angle: i32) -> LocalRef<jobject> {
        
        todo!();
        /*
            auto* env = getEnv();

                auto origBitmapWidth  = env->CallIntMethod (origBitmap, AndroidBitmap.getWidth);
                auto origBitmapHeight = env->CallIntMethod (origBitmap, AndroidBitmap.getHeight);

                auto matrix = LocalRef<jobject> (env->NewObject (AndroidMatrix, AndroidMatrix.constructor));
                env->CallBooleanMethod (matrix, AndroidMatrix.postRotate, (jfloat) rotationAngle, (jfloat) 0, (jfloat) 0);

                auto rotatedBitmap = LocalRef<jobject> (env->CallStaticObjectMethod (AndroidBitmap, AndroidBitmap.createBitmapFrom,
                                                                                     origBitmap.get(), (jint) 0, (jint) 0,
                                                                                     (jint) origBitmapWidth, (jint) origBitmapHeight,
                                                                                     matrix.get(), true));

                env->CallVoidMethod (origBitmap, AndroidBitmap.recycle);

                return rotatedBitmap;
        */
    }
}
