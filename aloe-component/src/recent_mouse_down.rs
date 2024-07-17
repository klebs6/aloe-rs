crate::ix!();

#[derive(Default)]
pub struct RecentMouseDown {
    position: Point<f32>,
    time:     Time,
    buttons:  ModifierKeys,
    peerid:   u32, // default = 0
    is_touch: bool, // default = false
}

impl RecentMouseDown {

    pub fn can_be_part_of_multiple_click_with(&self, 
        other:               &RecentMouseDown,
        max_time_between_ms: i32) -> bool {
        
        todo!();
        /*
            return time - other.time < RelativeTime::milliseconds (maxTimeBetweenMs)
                        && std::abs (position.x - other.position.x) < (float) getPositionToleranceForInputType()
                        && std::abs (position.y - other.position.y) < (float) getPositionToleranceForInputType()
                        && buttons == other.buttons
                        && peerID == other.peerID;
        */
    }
    
    pub fn get_position_tolerance_for_input_type(&self) -> i32 {
        
        todo!();
        /*
            return isTouch ? 25 : 8;
        */
    }
}
