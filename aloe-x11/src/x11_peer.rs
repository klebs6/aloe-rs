crate::ix!();

pub fn get_peer_for<'a>(windowh: Window) -> *mut ComponentPeer<'a> {
    
    todo!();
        /*
            if (windowH == 0)
            return nullptr;

        XPointer peer = nullptr;

        if (auto* display = XWindowSystem::getInstance()->getDisplay())
        {
            XWindowSystemUtilities::ScopedXLock xLock;
            X11Symbols::getInstance()->xFindContext (display, (XID) windowH, windowHandleXContext, &peer);
        }

        return unalignedPointerCast<ComponentPeer*> (peer);
        */
}

lazy_static!{
    /*
    static std::unordered_map<LinuxComponentPeer*, X11DragState> dragAndDropStateMap;
    */
}
