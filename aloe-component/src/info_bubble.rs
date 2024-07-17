crate::ix!();

pub trait ShowInfoBubble {

    /**
      | Shows a floating text bubble pointing
      | to the icon (if the current OS supports
      | this).
      |
      */
    fn show_info_bubble(
        &mut self, 
        title:   &String,
        content: &String
    );
}

pub trait HidInfoBubble {

    /**
      | Hides the icon's floating text bubble
      | (if the current OS supports this).
      |
      */
    fn hide_info_bubble(&mut self);
}
