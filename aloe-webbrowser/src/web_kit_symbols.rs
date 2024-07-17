crate::ix!();

pub struct WebkitSymbolBinding<'a, FuncPtr> {
    func: &'a mut FuncPtr,
    name: *const u8,
}

#[no_copy]
#[leak_detector]
pub struct WebKitSymbols {
    base:                 DeletedAtShutdown,
    gtk_lib:              DynamicLibrary, // default = { "libgtk-3.so"  }
    webkit_lib:           DynamicLibrary, // default = { "libwebkit2gtk-4.0.so"  }
    web_kit_is_available: bool, //= loadWebkitSymbols() && loadGtkSymbols();
}

aloe_implement_singleton!{
    WebKitSymbols
}

pub mod webkit_symbols {

    use super::*;

    aloe_generate_function_with_default!{
        webkit_settings_new, 
        aloe_webkit_settings_new,
        (), 
        *mut WebKitSettings
    }

    aloe_generate_function_with_default!{
        webkit_settings_set_hardware_acceleration_policy, 
        aloe_webkit_settings_set_hardware_acceleration_policy,
        (*mut WebKitSettings, i32), 
        ()
    }

    aloe_generate_function_with_default!{
        webkit_web_view_new_with_settings, 
        aloe_webkit_web_view_new_with_settings,
        (*mut WebKitSettings), 
        *mut GtkWidget
    }

    aloe_generate_function_with_default!{
        webkit_web_view_load_uri, 
        aloe_webkit_web_view_load_uri,
        (*mut WebKitWebView, *const gchar), 
        ()
    }

    aloe_generate_function_with_default!{
        webkit_policy_decision_use, 
        aloe_webkit_policy_decision_use,
        (*mut WebKitPolicyDecision), 
        ()
    }

    aloe_generate_function_with_default!{
        webkit_policy_decision_ignore, 
        aloe_webkit_policy_decision_ignore,
        (*mut WebKitPolicyDecision), 
        ()
    }

    aloe_generate_function_with_default!{
        webkit_web_view_go_back, 
        aloe_webkit_web_view_go_back,
        (*mut WebKitWebView), 
        ()
    }

    aloe_generate_function_with_default!{
        webkit_web_view_go_forward, 
        aloe_webkit_web_view_go_forward,
        (*mut WebKitWebView), 
        ()
    }

    aloe_generate_function_with_default!{
        webkit_web_view_reload, 
        aloe_webkit_web_view_reload,
        (*mut WebKitWebView), 
        ()
    }

    aloe_generate_function_with_default!{
        webkit_web_view_stop_loading, 
        aloe_webkit_web_view_stop_loading,
        (*mut WebKitWebView), 
        ()
    }

    aloe_generate_function_with_default!{
        webkit_uri_request_get_uri, 
        aloe_webkit_uri_request_get_uri,
        (*mut WebKitURIRequest), 
        *const gchar
    }

    aloe_generate_function_with_default!{
        webkit_navigation_action_get_request, 
        aloe_webkit_navigation_action_get_request,
        (*mut WebKitNavigationAction), 
        *mut WebKitURIRequest
    }

    aloe_generate_function_with_default!{
        webkit_navigation_policy_decision_get_frame_name, 
        aloe_webkit_navigation_policy_decision_get_frame_name,
        (*mut WebKitNavigationPolicyDecision), 
        *const gchar
    }

    aloe_generate_function_with_default!{
        webkit_navigation_policy_decision_get_navigation_action, 
        aloe_webkit_navigation_policy_decision_get_navigation_action,
        (*mut WebKitNavigationPolicyDecision), 
        *mut WebKitNavigationAction
    }

    aloe_generate_function_with_default!{
        webkit_web_view_get_uri, 
        aloe_webkit_web_view_get_uri,
        (*mut WebKitWebView), 
        *const gchar
    }

    aloe_generate_function_with_default!{
        gtk_init, 
        aloe_gtk_init,
        (*mut i32, *mut *mut *mut u8), 
        ()
    }

    aloe_generate_function_with_default!{
        gtk_plug_new, 
        aloe_gtk_plug_new,
        (Window), 
        *mut GtkWidget
    }

    aloe_generate_function_with_default!{
        gtk_scrolled_window_new, 
        aloe_gtk_scrolled_window_new,
        (*mut GtkAdjustment, *mut GtkAdjustment), 
        *mut GtkWidget
    }

    aloe_generate_function_with_default!{
        gtk_container_add, 
        aloe_gtk_container_add,
        (*mut GtkContainer, *mut GtkWidget), 
        void
    }

    aloe_generate_function_with_default!{
        gtk_widget_show_all, 
        aloe_gtk_widget_show_all,
        (*mut GtkWidget), 
        void
    }

    aloe_generate_function_with_default!{
        gtk_plug_get_id, 
        aloe_gtk_plug_get_id,
        (*mut GtkPlug), 
        ::Window
    }

    aloe_generate_function_with_default!{
        gtk_main, 
        aloe_gtk_main,
        (), 
        void
    }

    aloe_generate_function_with_default!{
        gtk_main_quit, 
        aloe_gtk_main_quit,
        (), 
        void
    }

    aloe_generate_function_with_default!{
        g_unix_fd_add, 
        aloe_g_unix_fd_add,
        (gint, GIOCondition, GUnixFDSourceFunc, gpointer), 
        guint
    }

    aloe_generate_function_with_default!{
        g_object_ref, 
        aloe_g_object_ref,
        (gpointer), 
        gpointer
    }

    aloe_generate_function_with_default!{
        g_object_unref, 
        aloe_g_object_unref,
        (gpointer), 
        void
    }

    aloe_generate_function_with_default!{
        g_signal_connect_data, 
        aloe_g_signal_connect_data,
        (gpointer, *const gchar, GCallback, gpointer, GClosureNotify, GConnectFlags), 
        gulong
    }

    aloe_declare_singleton_singlethreaded_minimal!{
        WebKitSymbols
    }
}

impl Drop for WebKitSymbols {

    fn drop(&mut self) {
        todo!();
        /*
            clearSingletonInstance();
        */
    }
}

impl WebKitSymbols {

    pub fn is_web_kit_available(&self) -> bool {
        
        todo!();
        /*
            return webKitIsAvailable;
        */
    }
    
    pub fn make_symbol_binding<FuncPtr>(&mut self, 
        func: &mut FuncPtr,
        name: *const u8) -> WebkitSymbolBinding<FuncPtr> {
    
        todo!();
        /*
            return { func, name };
        */
    }
    
    pub fn load_symbols<FuncPtr>(&mut self, 
        lib:     &mut DynamicLibrary,
        binding: WebkitSymbolBinding<FuncPtr>) -> bool {
    
        todo!();
        /*
            if (auto* func = lib.getFunction (binding.name))
            {
                binding.func = reinterpret_cast<FuncPtr> (func);
                return true;
            }

            return false;
        */
    }
    
    pub fn load_symbols_with_args<FuncPtr, Args>(
        &mut self, 
        lib:     &mut DynamicLibrary,
        binding: WebkitSymbolBinding<FuncPtr>,
        args:    Args
    ) -> bool {
    
        todo!();
        /*
            return loadSymbols (lib, binding) && loadSymbols (lib, args...);
        */
    }
    
    pub fn load_webkit_symbols(&mut self) -> bool {
        
        todo!();
        /*
            return loadSymbols (webkitLib,
                                makeSymbolBinding (aloe_webkit_settings_new,                                     "webkit_settings_new"),
                                makeSymbolBinding (aloe_webkit_settings_set_hardware_acceleration_policy,        "webkit_settings_set_hardware_acceleration_policy"),
                                makeSymbolBinding (aloe_webkit_web_view_new_with_settings,                       "webkit_web_view_new_with_settings"),
                                makeSymbolBinding (aloe_webkit_policy_decision_use,                              "webkit_policy_decision_use"),
                                makeSymbolBinding (aloe_webkit_policy_decision_ignore,                           "webkit_policy_decision_ignore"),
                                makeSymbolBinding (aloe_webkit_web_view_go_back,                                 "webkit_web_view_go_back"),
                                makeSymbolBinding (aloe_webkit_web_view_go_forward,                              "webkit_web_view_go_forward"),
                                makeSymbolBinding (aloe_webkit_web_view_reload,                                  "webkit_web_view_reload"),
                                makeSymbolBinding (aloe_webkit_web_view_stop_loading,                            "webkit_web_view_stop_loading"),
                                makeSymbolBinding (aloe_webkit_uri_request_get_uri,                              "webkit_uri_request_get_uri"),
                                makeSymbolBinding (aloe_webkit_web_view_load_uri,                                "webkit_web_view_load_uri"),
                                makeSymbolBinding (aloe_webkit_navigation_action_get_request,                    "webkit_navigation_action_get_request"),
                                makeSymbolBinding (aloe_webkit_navigation_policy_decision_get_frame_name,        "webkit_navigation_policy_decision_get_frame_name"),
                                makeSymbolBinding (aloe_webkit_navigation_policy_decision_get_navigation_action, "webkit_navigation_policy_decision_get_navigation_action"),
                                makeSymbolBinding (aloe_webkit_web_view_get_uri,                                 "webkit_web_view_get_uri"));
        */
    }
    
    pub fn load_gtk_symbols(&mut self) -> bool {
        
        todo!();
        /*
            return loadSymbols (gtkLib,
                                makeSymbolBinding (aloe_gtk_init,                "gtk_init"),
                                makeSymbolBinding (aloe_gtk_plug_new,            "gtk_plug_new"),
                                makeSymbolBinding (aloe_gtk_scrolled_window_new, "gtk_scrolled_window_new"),
                                makeSymbolBinding (aloe_gtk_container_add,       "gtk_container_add"),
                                makeSymbolBinding (aloe_gtk_widget_show_all,     "gtk_widget_show_all"),
                                makeSymbolBinding (aloe_gtk_plug_get_id,         "gtk_plug_get_id"),
                                makeSymbolBinding (aloe_gtk_main,                "gtk_main"),
                                makeSymbolBinding (aloe_gtk_main_quit,           "gtk_main_quit"),
                                makeSymbolBinding (aloe_g_unix_fd_add,           "g_unix_fd_add"),
                                makeSymbolBinding (aloe_g_object_ref,            "g_object_ref"),
                                makeSymbolBinding (aloe_g_object_unref,          "g_object_unref"),
                                makeSymbolBinding (aloe_g_signal_connect_data,   "g_signal_connect_data"));
        */
    }
}

lazy_static!{
    /*
    extern int aloe_gtkWebkitMain (int argc, const char* argv[]);
    */
}
