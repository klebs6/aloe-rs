crate::ix!();

/**
  | Returns the actual container window, unlike
  | GetParent, which can also return a separate
  | owner window.
  */
#[cfg(target_os="windows")]
pub fn get_window_parent(w: HWND) -> HWND {
    
    todo!();
        /*
            return GetAncestor (w, GA_PARENT);
        */
}

#[cfg(target_os="windows")]
pub fn find_mdi_parent_of(w: HWND) -> HWND {
    
    todo!();
        /*
            const int frameThickness = GetSystemMetrics (SM_CYFIXEDFRAME);

        while (w != nullptr)
        {
            auto parent = getWindowParent (w);

            if (parent == nullptr)
                break;

            TCHAR windowType[32] = { 0 };
            GetClassName (parent, windowType, 31);

            if (String (windowType).equalsIgnoreCase ("MDIClient"))
                return parent;

            RECT windowPos, parentPos;
            GetWindowRect (w, &windowPos);
            GetWindowRect (parent, &parentPos);

            auto dw = (parentPos.right - parentPos.left) - (windowPos.right - windowPos.left);
            auto dh = (parentPos.bottom - parentPos.top) - (windowPos.bottom - windowPos.top);

            if (dw > 100 || dh > 100)
                break;

            w = parent;

            if (dw == 2 * frameThickness)
                break;
        }

        return w;
        */
}

#[cfg(target_os="windows")]
lazy_static!{
    /*
    static int numActivePlugins = 0;
    static bool messageThreadIsDefinitelyCorrect = false;
    */
}
