crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AAX/aloe_AAX_Modifier_Injector.h]

pub trait ModifierKeyProvider {
    fn get_win_32modifiers(&self) -> i32;
}

pub trait ModifierKeyReceiver
{
    fn set_modifier_key_provider(&mut self, _0: *mut dyn ModifierKeyProvider);
    fn remove_modifier_key_provider(&mut self);
}
