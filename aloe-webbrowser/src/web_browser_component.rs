crate::ix!();

pub fn aloe_gtk_webkit_main(
        argc: i32,
        argv: &[*const u8]) -> i32 {
    
    todo!();
    /*
        if (argc != 4)
        return -1;

    GtkChildProcess child (String (argv[2]).getIntValue(),
                           String (argv[3]).getIntValue());

    return child.entry();
    */
}

macro_rules! aloe_g_signal_connect {
    ($instance:ident, 
     $detailed_signal:ident, 
     $c_handler:ident, 
     $data:ident) => {
        /*
        
            WebKitSymbols::getInstance()->aloe_g_signal_connect_data (instance, detailed_signal, c_handler, data, nullptr, (GConnectFlags) 0)
        */
    }
}
