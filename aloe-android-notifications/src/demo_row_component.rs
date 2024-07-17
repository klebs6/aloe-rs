crate::ix!();

#[cfg(target_os="android")]
pub struct PushNotificationsDemoRowComponent<'a> {
    base:      Component<'a>,
    label:     &'a mut Label<'a>,
    editor:    &'a mut Component<'a>,
    row_units: i32,
}

#[cfg(target_os="android")]
impl<'a> PushNotificationsDemoRowComponent<'a> {

    pub fn new(
        l: &mut Label,
        c: &mut Component<'a>,
        u: i32) -> Self {

        let u: i32 = u.unwrap_or(1);

        todo!();
        /*
        : label(l),
        : editor(c),
        : row_units(u),

            addAndMakeVisible (label);
                addAndMakeVisible (editor);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto bounds = getLocalBounds();
                label .setBounds (bounds.removeFromLeft (getWidth() / 3));
                editor.setBounds (bounds);
        */
    }
}

