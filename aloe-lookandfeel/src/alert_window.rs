crate::ix!();

/**
  | This abstract base class is implemented
  | by LookAndFeel classes to provide alert-window
  | drawing functionality.
  |
  */
pub trait AlertWindowLookAndFeelMethods {

    fn create_alert_window<'a>(
        &mut self, 
        title:                &String,
        message:              &String,
        button1:              &String,
        button2:              &String,
        button3:              &String,
        icon_type:            MessageBoxIconType,
        num_buttons:          i32,
        associated_component: *mut Component<'a>
    ) -> *mut AlertWindow;

    fn draw_alert_box(
        &mut self, 
        _0:        &mut Graphics,
        _1:        &mut AlertWindow,
        text_area: &Rectangle<i32>,
        _3:        &mut TextLayout
    );

    fn get_alert_box_window_flags(&mut self) -> i32;

    fn get_widths_for_text_buttons(
        &mut self, 
        _0: &mut AlertWindow,
        _1: &[*mut TextButton]
    ) -> Vec<i32>;

    fn get_alert_window_button_height(&mut self) -> i32;

    fn get_alert_window_title_font(&mut self) -> Font;

    fn get_alert_window_message_font(&mut self) -> Font;

    fn get_alert_window_font(&mut self) -> Font;
}

impl dyn LookAndFeel {

    #[cfg(target_os="linux")]
    pub fn play_alert_sound(&mut self)  {
        
        todo!();
        /*
            std::cout << "\a" << std::flush;
        */

    }
}
