crate::ix!();

lazy_static!{
    /*
    static void* aloe_libjackHandle = nullptr;
    */
}

pub fn aloe_load_jack_function(name: *const u8)  {
    
    todo!();
    /*
        if (aloe_libjackHandle == nullptr)
            return nullptr;

        return dlsym (aloe_libjackHandle, name);
    */
}

macro_rules! aloe_decl_jack_function {
    (
        $return_type:ty, 
        $fn_name:ident, 
        ($($argument_name:ident : $argument_type:ty),*)
    ) => {
        /*
        
          return_type fn_name argument_types                                              
          {                                                                               
              using ReturnType = return_type;                                             
              typedef return_type (*fn_type) argument_types;                              
              static fn_type fn = (fn_type) aloe_loadJackFunction (#fn_name);             
              jassert (fn != nullptr);                                                    
              return (fn != nullptr) ? ((*fn) arguments) : ReturnType();                  
          }
        */
    }
}

macro_rules! aloe_decl_void_jack_function {
    (
        $fn_name:ident, 
        ($($argument_name:ident : $argument_type:ty),*)
    ) => {
        /*
        
          void fn_name argument_types                                                     
          {                                                                               
              typedef void (*fn_type) argument_types;                                     
              static fn_type fn = (fn_type) aloe_loadJackFunction (#fn_name);             
              jassert (fn != nullptr);                                                    
              if (fn != nullptr) (*fn) arguments;                                         
          }
        */
    }
}

aloe_decl_jack_function!{
    *mut jack_client_t, 
    jack_client_open, 

    (client_name: *const char, 
     options:     jack_options_t, 
     status:      *mut jack_status_t,
     args:        &[&str])
}

aloe_decl_jack_function!{
    i32, 
    jack_client_close, 
    (client: *mut jack_client_t)
}

aloe_decl_jack_function!{
    i32, 
    jack_activate, 
    (client: *mut jack_client_t)
}

aloe_decl_jack_function!{
    i32, 
    jack_deactivate, 
    (client: *mut jack_client_t)
}

aloe_decl_jack_function!{
    jack_nframes_t, 
    jack_get_buffer_size, 
    (client: *mut jack_client_t)
}

aloe_decl_jack_function!{
    jack_nframes_t, 
    jack_get_sample_rate, 
    (client: *mut jack_client_t)
}

aloe_decl_void_jack_function!{
    jack_on_shutdown, 

    (client:   *mut jack_client_t, 
     function: fn(arg: *mut c_void) -> (),
     arg:      *mut c_void)
}

aloe_decl_void_jack_function!{
    jack_on_info_shutdown, 

    (client: *mut jack_client_t, 
     function: JackInfoShutdownCallback, 
     arg: *mut ())
}

aloe_decl_jack_function!{
    *mut c_void, 
    jack_port_get_buffer, 

    (port: *mut jack_port_t, 
     nframes: jack_nframes_t) 
}

aloe_decl_jack_function!{
    jack_nframes_t, 
    jack_port_get_total_latency, 

    (client: *mut jack_client_t, 
     port: *mut jack_port_t) 
}

aloe_decl_jack_function!{
    *mut jack_port_t, 
    jack_port_register, 

    (client:      *mut jack_client_t, 
     port_name:   *const char, 
     port_type:   *const char, 
     flags:       u64, 
     buffer_size: u64)
}

aloe_decl_void_jack_function!{
    jack_set_error_function, 
    (
        func: fn(*const char) -> ()
    )
}

aloe_decl_jack_function!{
    i32, 
    jack_set_process_callback, 

    (client:           *mut jack_client_t, 
     process_callback: JackProcessCallback, 
     arg:              *mut c_void)
}

aloe_decl_jack_function!{
    *const *const u8, 
    jack_get_ports, 

    (client:            *mut jack_client_t, 
     port_name_pattern: *const u8, 
     type_name_pattern: *const u8, 
     flags:             u64)
}

aloe_decl_jack_function!{
    i32, 
    jack_connect, 

    (client:           *mut jack_client_t, 
     source_port:      *const char, 
     destination_port: *const char)
}

aloe_decl_jack_function!{
    *const char, 
    jack_port_name, 
    (port: *const jack_port_t) 
}

aloe_decl_jack_function!{
    *mut c_void, 
    jack_set_port_connect_callback, 

    (client: *mut jack_client_t, 
     connect_callback: JackPortConnectCallback, 
     arg: *mut c_void) 
}

aloe_decl_jack_function!{
    *mut jack_port_t, 
    jack_port_by_id, 

    (client: *mut jack_client_t, 
     port_id: jack_port_id_t)
}

aloe_decl_jack_function!{
    i32, 
    jack_port_connected, 
    (port: *const jack_port_t) 
}

aloe_decl_jack_function!{
    i32, 
    jack_port_connected_to, 

    (port: *const jack_port_t, 
     port_name: *const char) 
}

aloe_decl_jack_function!{
    i32, 
    jack_set_xrun_callback, 

    (client: *mut jack_client_t, 
     xrun_callback: JackXRunCallback, 
     arg: *mut c_void) 
}

aloe_decl_jack_function!{
    i32, 
    jack_port_flags, 
    (port: *const jack_port_t) 
}

aloe_decl_jack_function!{
    *mut jack_port_t, 
    jack_port_by_name, 

    (client: *mut jack_client_t, 
     name: *const char)
}

aloe_decl_void_jack_function!{
    jack_free, 
    (ptr: *mut c_void)
}


