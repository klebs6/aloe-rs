crate::ix!();

pub struct GetTrackInfo {

}

impl GetTrackInfo {

    /**
      | Combo boxes need a lot of room
      |
      */
    pub fn invoke_attached_combo(&self, _0: &mut AttachedCombo) -> GridTrackInfo {
        
        todo!();
        /*
            return 120_px;
        */
    }

    /**
      | Toggles are a bit smaller
      |
      */
    pub fn invoke_attached_toggle(&self, _0: &mut AttachedToggle) -> GridTrackInfo {
        
        todo!();
        /*
            return 80_px;
        */
    }

    /**
      | Sliders take up as much room as they can
      |
      */
    pub fn invoke_attached_slider(&self, _0: &mut AttachedSlider) -> GridTrackInfo {
        
        todo!();
        /*
            return 1_fr;
        */
    }
}
