crate::ix!();

#[cfg(ALOE_USE_CAMERA)]
#[cfg(any(target_os="android",target_os="ios"))]
pub struct CameraFactoryPendingCameraOpen
{
    request_id:      i32,
    device:          Box<CameraDevice>,
    result_callback: CameraDeviceOpenCameraResultCallback,
}

#[cfg(ALOE_USE_CAMERA)]
#[cfg(any(target_os="android",target_os="ios"))]
pub struct CameraFactory {
    cameras_to_open: Vec<CameraFactoryPendingCameraOpen>,
}

#[cfg(ALOE_USE_CAMERA)]
#[cfg(any(target_os="android",target_os="ios"))]
pub mod camera_factory {

    use super::*;

    lazy_static!{
        /*
        static int nextRequestId;
        int CameraDevice::CameraFactory::nextRequestId = 0;
        */
    }
}

#[cfg(ALOE_USE_CAMERA)]
#[cfg(any(target_os="android",target_os="ios"))]
impl CameraFactory {

    pub fn get_instance() -> &mut CameraFactory {
        
        todo!();
        /*
            static CameraFactory factory;
                return factory;
        */
    }
    
    pub fn open_camera(&mut self, 
        index:            i32,
        result_callback:  CameraDeviceOpenCameraResultCallback,
        min_width:        i32,
        min_height:       i32,
        max_width:        i32,
        max_height:       i32,
        use_high_quality: bool)  {
        
        todo!();
        /*
            auto cameraId = getAvailableDevices()[index];

                if (getCameraIndex (cameraId) != -1)
                {
                    // You are trying to open the same camera twice.
                    jassertfalse;
                    return;
                }

                std::unique_ptr<CameraDevice> device (new CameraDevice (cameraId, index,
                                                                        minWidth, minHeight, maxWidth,
                                                                        maxHeight, useHighQuality));

                camerasToOpen.add ({ nextRequestId++,
                                     std::unique_ptr<CameraDevice> (device.release()),
                                     resultCallback });

                auto& pendingOpen = camerasToOpen.getReference (camerasToOpen.size() - 1);

                pendingOpen.device->impl->open ([this] (const String& deviceId, const String& error)
                                                 {
                                                     int cIndex = getCameraIndex (deviceId);

                                                     if (cIndex == -1)
                                                         return;

                                                     auto& cameraPendingOpen = camerasToOpen.getReference (cIndex);

                                                     if (error.isEmpty())
                                                         cameraPendingOpen.resultCallback (cameraPendingOpen.device.release(), error);
                                                     else
                                                         cameraPendingOpen.resultCallback (nullptr, error);

                                                     int id = cameraPendingOpen.requestId;

                                                     MessageManager::callAsync ([this, id]() { removeRequestWithId (id); });
                                                 });
        */
    }
    
    pub fn get_camera_index(&self, camera_id: &String) -> i32 {
        
        todo!();
        /*
            for (int i = 0; i < camerasToOpen.size(); ++i)
                {
                    auto& pendingOpen = camerasToOpen.getReference (i);

                    if (pendingOpen.device->impl->getCameraId() == cameraId)
                        return i;
                }

                return -1;
        */
    }
    
    pub fn remove_request_with_id(&mut self, id: i32)  {
        
        todo!();
        /*
            for (int i = camerasToOpen.size(); --i >= 0;)
                {
                    if (camerasToOpen.getReference (i).requestId == id)
                    {
                        camerasToOpen.remove (i);
                        return;
                    }
                }
        */
    }
}
