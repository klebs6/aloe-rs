fn main() {

    //---------------------------------
    println!("cargo:rustc-cfg=ALOE_PLUGIN_BUILD_AAX");
    println!("cargo:rustc-cfg=ALOE_PLUGIN_AAX_IDENTIFIER");
    println!("cargo:rustc-cfg=ALOE_PLUGIN_BUILD_AU");
    println!("cargo:rustc-cfg=ALOE_PLUGIN_BUILD_AUV3");
    println!("cargo:rustc-cfg=ALOE_PLUGIN_BUILD_LV2");
    println!("cargo:rustc-cfg=ALOE_PLUGIN_LV2URI");
    println!("cargo:rustc-cfg=ALOE_PLUGIN_BUILD_RTAS");
    println!("cargo:rustc-cfg=ALOE_PLUGIN_BUILD_STANDALONE");
    println!("cargo:rustc-cfg=ALOE_PLUGIN_BUILD_UNITY");
    println!("cargo:rustc-cfg=ALOE_PLUGIN_BUILD_Vst");
    println!("cargo:rustc-cfg=ALOE_PLUGIN_BUILD_Vst3");
    println!("cargo:rustc-cfg=ALOE_PLUGIN_IS_SYNTH");
    println!("cargo:rustc-cfg=ALOE_PLUGIN_MANUFACTURER_CODE");
    println!("cargo:rustc-cfg=ALOE_PLUGIN_PLUGIN_CODE");
    println!("cargo:rustc-cfg=ALOE_PLUGIN_PRODUCES_MIDI_OUTPUT");
    println!("cargo:rustc-cfg=ALOE_PLUGIN_WANTS_MIDI_INPUT");
    println!("cargo:rustc-cfg=ALOE_PLUGIN_LATENCY");
    println!("cargo:rustc-cfg=ALOE_PLUGIN_EDITOR_REQUIRES_KEYBOARD_FOCUS");
}

