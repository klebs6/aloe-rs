crate::ix!();

pub trait UpdatePeer {

    fn update_peer(&mut self) {

        todo!();
        /*
            if (isOnDesktop())
            if (auto* peer = getPeer())
                peer->setConstrainer (constrainer);
        */
    }
}

pub trait CreateNewPeer {

    fn create_new_peer(&mut self, 
        style_flags:   i32,
        native_window: *mut c_void) -> *mut ComponentPeer {

        todo!();
        /*
            if (aloe_createUnityPeerFn != nullptr)
        {
            ignoreUnused (styleFlags, nativeWindow);
            return aloe_createUnityPeerFn (*this);
        }

        return Component::createNewPeer (styleFlags, nativeWindow);
        */
    }
}

//#[cfg(target_os="android")]
impl<'a> CreateNewPeer for Component<'a> {
    
    fn create_new_peer(&mut self, 
        style_flags:   i32,
        native_window: *mut c_void) -> *mut ComponentPeer {
        
        todo!();
        /*
            return new AndroidComponentPeer (*this, styleFlags, nativeWindow);
        */
    }
}
