crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/Plugins/SurroundPluginDemo.h]

pub trait ChannelClickListener {
    fn channel_button_clicked(&mut self, channel_index: i32);
    fn is_channel_active(&mut self, channel_index: i32) -> bool;
}
