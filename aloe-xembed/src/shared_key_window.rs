crate::ix!();

pub struct XembedComponentSharedKeyWindow<'a> {
    key_peer:  *mut ComponentPeer<'a>,
    key_proxy: Window,
}

pub type XembedComponentSharedKeyWindowPtr<'a> = Rc<RefCell<XembedComponentSharedKeyWindow<'a>>>;

impl<'a> Drop for XembedComponentSharedKeyWindow<'a> {

    fn drop(&mut self) {

        todo!();

        /*
            aloe_deleteKeyProxyWindow (keyProxy);

                auto& keyWindows = getKeyWindows();
                keyWindows.remove (keyPeer);
        */
    }
}

impl<'a> XembedComponentSharedKeyWindow<'a> {

    pub fn new(peer_to_use: *mut ComponentPeer) -> Self {
    
        todo!();
        /*
        : key_peer(peerToUse),
        : key_proxy(aloe_createKeyProxyWindow (keyPeer)),

        
        */
    }
    
    pub fn get_handle(&mut self) -> Window {
        
        todo!();
        /*
            return keyProxy;
        */
    }
    
    pub fn get_current_focus_window(peer_to_look_for: *mut ComponentPeer) -> Window {
        
        todo!();
        /*
            auto& keyWindows = getKeyWindows();

                if (peerToLookFor != nullptr)
                    if (auto* foundKeyWindow = keyWindows[peerToLookFor])
                        return foundKeyWindow->keyProxy;

                return {};
        */
    }
    
    pub fn get_key_window_for_peer(peer_to_look_for: *mut ComponentPeer) -> XembedComponentSharedKeyWindowPtr<'a> {
        
        todo!();
        /*
            jassert (peerToLookFor != nullptr);

                auto& keyWindows = getKeyWindows();
                auto foundKeyWindow = keyWindows[peerToLookFor];

                if (foundKeyWindow == nullptr)
                {
                    foundKeyWindow = new XembedComponentSharedKeyWindow (peerToLookFor);
                    keyWindows.set (peerToLookFor, foundKeyWindow);
                }

                return foundKeyWindow;
        */
    }
    
    pub fn get_key_windows() -> &'a mut HashMap<*mut ComponentPeer<'a>,*mut XembedComponentSharedKeyWindow<'a>> {
        
        todo!();
        /*
            // store a weak reference to the shared key windows
                static HashMap<ComponentPeer*, XembedComponentSharedKeyWindow*> keyWindows;
                return keyWindows;
        */
    }
}
