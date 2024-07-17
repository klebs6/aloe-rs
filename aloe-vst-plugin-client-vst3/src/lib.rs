//#[cfg(all(AloePlugin_Build_Vst3,any(any(target_os="macos",target_os="windows"),any(target_os="linux",target_os="bsd"))))]
#[macro_use] mod imports; use imports::*;

x!{in_parameter_changed_callback_setter}
x!{aloe_audio_processor}
x!{aloe_plugin_factory}
x!{aloe_plugin_factory_class_entry}
x!{aloe_vst3_component}
x!{aloe_vst3_component_locked_vst_com_smartptr}
x!{aloe_vst3_component_scoped_in_setup_processing_setter}
x!{aloe_vst3_edit_controller}
x!{aloe_vst3_edit_controller_editor_context_menu}
x!{aloe_vst3_edit_controller_editor_host_context}
x!{aloe_vst3_edit_controller_midi_controller}
x!{aloe_vst3_edit_controller_owned_parameter_listener}
x!{aloe_vst3_edit_controller_param}
x!{aloe_vst3_edit_controller_program_change_parameter}
x!{aloe_vst3_editor}
x!{aloe_vst3_editor_content_wrapper_component}
x!{aloe_vst3_editor_cubase_workaround}
x!{aloe_vst3_editor_message_manager_locked_deleter}
x!{linux_or_bsd_vst3_event_handler}
x!{linux_or_bsd_vst3_event_handler_host_runloop_interfaces}
x!{mac_vst3}
x!{startup_config_entry}
