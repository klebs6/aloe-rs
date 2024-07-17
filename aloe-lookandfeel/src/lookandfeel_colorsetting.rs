crate::ix!();

pub struct LookAndFeelColourSetting
{
    colourid: i32,
    colour:   Colour,
}

impl Ord for LookAndFeelColourSetting {
    
    #[inline] fn cmp(&self, other: &LookAndFeelColourSetting) 
    -> std::cmp::Ordering 
    {
        todo!();
        /*
            return colourID <  other.colourID;
        */
    }
}

impl PartialOrd<LookAndFeelColourSetting> for LookAndFeelColourSetting {

    #[inline] fn partial_cmp(&self, other: &LookAndFeelColourSetting) 
    -> Option<std::cmp::Ordering> 
    {
        Some(self.cmp(other))
    }
}

impl PartialEq<LookAndFeelColourSetting> for LookAndFeelColourSetting {
    
    #[inline] fn eq(&self, other: &LookAndFeelColourSetting) -> bool {
        todo!();
        /*
            return colourID == other.colourID;
        */
    }
}

impl Eq for LookAndFeelColourSetting {}
