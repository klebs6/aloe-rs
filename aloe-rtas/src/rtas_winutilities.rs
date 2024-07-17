crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/RTAS/aloe_RTAS_WinUtilities.cpp]

#[cfg(AloePlugin_Build_RTAS)]
pub mod rtas {
    use super::*;

    /**
       (these functions are in their own file because
       of problems including windows.h at the same
       time as the Digi headers)
      */

    pub const _DO_NOT_DECLARE_INTERLOCKED_INTRINSICS_IN_MEMORY: bool = true; // (workaround for a VC build problem)

    pub const _WIN32_WINNT: usize = 0x0500;
    pub const STRICT: bool = true;

    pub fn attach_sub_window(
            host_window: *mut c_void,
            titlew:      &mut i32,
            titleh:      &mut i32,
            comp:        *mut Component)  {
        
        todo!();
        /*
            using namespace aloe;

            RECT clientRect;
            GetClientRect ((HWND) hostWindow, &clientRect);

            titleW = clientRect.right - clientRect.left;
            titleH = jmax (0, (int) (clientRect.bottom - clientRect.top) - comp->getHeight());
            comp->setTopLeftPosition (0, titleH);

            comp->addToDesktop (0);

            HWND plugWnd = (HWND) comp->getWindowHandle();
            SetParent (plugWnd, (HWND) hostWindow);

            DWORD val = GetWindowLong (plugWnd, GWL_STYLE);
            val = (val & ~WS_POPUP) | WS_CHILD;
            SetWindowLong (plugWnd, GWL_STYLE, val);

            val = GetWindowLong ((HWND) hostWindow, GWL_STYLE);
            SetWindowLong ((HWND) hostWindow, GWL_STYLE, val | WS_CLIPCHILDREN);
        */
    }

    pub fn resize_host_window(
            host_window: *mut c_void,
            titlew:      &mut i32,
            titleh:      &mut i32,
            comp:        *mut Component)  {
        
        todo!();
        /*
            using namespace aloe;

            RECT clientRect, windowRect;
            GetClientRect ((HWND) hostWindow, &clientRect);
            GetWindowRect ((HWND) hostWindow, &windowRect);
            const int borderW = (windowRect.right - windowRect.left) - (clientRect.right - clientRect.left);
            const int borderH = (windowRect.bottom - windowRect.top) - (clientRect.bottom - clientRect.top);

            SetWindowPos ((HWND) hostWindow, 0, 0, 0,
                          borderW + jmax (titleW, comp->getWidth()),
                          borderH + comp->getHeight() + titleH,
                          SWP_NOACTIVATE | SWP_NOMOVE | SWP_NOZORDER | SWP_NOOWNERZORDER);
        */
    }

    extern "C"  {
        #[WINAPI]
        pub fn dll_mainrtas(
                _0: HINSTANCE,
                _1: u64,
                _2: LPVOID) -> bool {
            
            todo!();
            /*
            
            */
        }

        #[WINAPI]
        pub fn dll_main(
                instance: HINSTANCE,
                reason:   u64,
                reserved: LPVOID) -> bool {
            
            todo!();
            /*
                if (reason == DLL_PROCESS_ATTACH)
                    Process::setCurrentModuleInstanceHandle (instance);

                if (GetModuleHandleA ("DAE.DLL") != 0)
                    return DllMainRTAS (instance, reason, reserved);

                ignoreUnused (reserved);
                return TRUE;
            */
        }
    }


    #[cfg(not(AloePlugin_EditorRequiresKeyboardFocus))]
    pub fn find_mdi_parent_of(w: HWND) -> HWND {
        
        todo!();
        /*
            const int frameThickness = GetSystemMetrics (SM_CYFIXEDFRAME);

                while (w != 0)
                {
                    HWND parent = GetParent (w);

                    if (parent == 0)
                        break;

                    TCHAR windowType [32] = { 0 };
                    GetClassName (parent, windowType, 31);

                    if (String (windowType).equalsIgnoreCase ("MDIClient"))
                    {
                        w = parent;
                        break;
                    }

                    RECT windowPos, parentPos;
                    GetWindowRect (w, &windowPos);
                    GetWindowRect (parent, &parentPos);

                    int dw = (parentPos.right - parentPos.left) - (windowPos.right - windowPos.left);
                    int dh = (parentPos.bottom - parentPos.top) - (windowPos.bottom - windowPos.top);

                    if (dw > 100 || dh > 100)
                        break;

                    w = parent;

                    if (dw == 2 * frameThickness)
                        break;
                }

                return w;
        */
    }

    #[cfg(not(AloePlugin_EditorRequiresKeyboardFocus))]
    pub fn pass_focus_to_host_window(host_window: *mut c_void)  {
        
        todo!();
        /*
            SetFocus (findMDIParentOf ((HWND) hostWindow));
        */
    }
}
