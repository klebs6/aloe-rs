crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/native/x11/aloe_linux_XWindowSystem.cpp]

pub fn add_atom_if_exists(
    condition: bool,
    key:       *const u8,
    display:   *mut Display,
    atoms:     &mut Vec<Atom>

) {

    todo!();
        /*
            if (condition)
        {
            auto atom = XWindowSystemUtilities::XWindowSystemAtoms::getIfExists (display, key);

            if (atom != None)
                atoms.push_back (atom);
        }
        */
}
