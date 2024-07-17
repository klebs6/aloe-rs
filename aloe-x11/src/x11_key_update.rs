crate::ix!();

pub fn update_key_states(
    keycode: i32,
    press:   bool

) {
    
    todo!();
        /*
            auto keybyte = keycode >> 3;
        auto keybit = (1 << (keycode & 7));

        if (press)
            Keys::keyStates [keybyte] |= keybit;
        else
            Keys::keyStates [keybyte] &= ~keybit;
        */
}

pub fn update_key_modifiers(status: i32)  {
    
    todo!();
        /*
            int keyMods = 0;

        if ((status & ShiftMask)     != 0) keyMods |= ModifierKeys::shiftModifier;
        if ((status & ControlMask)   != 0) keyMods |= ModifierKeys::ctrlModifier;
        if ((status & Keys::AltMask) != 0) keyMods |= ModifierKeys::altModifier;

        ModifierKeys::currentModifiers = ModifierKeys::currentModifiers.withOnlyMouseButtons().withFlags (keyMods);

        Keys::numLock  = ((status & Keys::NumLockMask) != 0);
        Keys::capsLock = ((status & LockMask)          != 0);
        */
}

pub fn update_key_modifiers_from_sym(
    sym:   KeySym,
    press: bool

) -> bool {
    
    todo!();
        /*
            int modifier = 0;
        bool isModifier = true;

        switch (sym)
        {
            case XK_Shift_L:
            case XK_Shift_R:   modifier = ModifierKeys::shiftModifier; break;

            case XK_Control_L:
            case XK_Control_R: modifier = ModifierKeys::ctrlModifier; break;

            case XK_Alt_L:
            case XK_Alt_R:     modifier = ModifierKeys::altModifier; break;

            case XK_Num_Lock:
                if (press)
                    Keys::numLock = ! Keys::numLock;

                break;

            case XK_Caps_Lock:
                if (press)
                    Keys::capsLock = ! Keys::capsLock;

                break;

            case XK_Scroll_Lock:
                break;

            default:
                isModifier = false;
                break;
        }

        ModifierKeys::currentModifiers = press ? ModifierKeys::currentModifiers.withFlags (modifier)
                                               : ModifierKeys::currentModifiers.withoutFlags (modifier);

        return isModifier;
        */
}

pub const KeyPressEventType: usize = 2;
