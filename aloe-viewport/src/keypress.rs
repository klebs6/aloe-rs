crate::ix!();

pub type ViewportDragPosition 
= AnimatedPosition<ContinuousWithMomentum>;

pub trait ViewportDragPositionListener: 
AnimatedPositionListener<ContinuousWithMomentum> {}

pub fn rescale_mouse_wheel_distance(
        distance:         f32,
        single_step_size: i32) -> i32 {
    
    todo!();
    /*
        if (distance == 0.0f)
            return 0;

        distance *= 14.0f * (float) singleStepSize;

        return roundToInt (distance < 0 ? jmin (distance, -1.0f)
                                        : jmax (distance,  1.0f));
    */
}

pub fn is_up_down_key_press(key: &KeyPress) -> bool {
    
    todo!();
    /*
        return key == KeyPress::upKey
            || key == KeyPress::downKey
            || key == KeyPress::pageUpKey
            || key == KeyPress::pageDownKey
            || key == KeyPress::homeKey
            || key == KeyPress::endKey;
    */
}

pub fn is_left_right_key_press(key: &KeyPress) -> bool {
    
    todo!();
    /*
        return key == KeyPress::leftKey
            || key == KeyPress::rightKey;
    */
}
