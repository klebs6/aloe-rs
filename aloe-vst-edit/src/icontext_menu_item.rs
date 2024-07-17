crate::ix!();

/**
  | IContextMenuItem is an entry element
  | of the context menu.
  |
  */
pub struct IContextMenuItem {

    /**
      | Name of the item
      |
      */
    name:  String128,

    /**
      | Identifier tag of the item
      |
      */
    tag:   i32,

    /**
      | IContextMenuItemFlags of the item
      |
      */
    flags: i32,
}

bitflags!{

    pub struct IContextMenuItemFlags: u32 {

        // Item is a separator
        const IsSeparator    = 1 << 0;

        // Item is disabled
        const IsDisabled     = 1 << 1;

        // Item is checked
        const IsChecked      = 1 << 2;

        // Item is a group start (like sub folder)
        const IsGroupStart   = 1 << 3 | Self::IsDisabled.bits();

        // Item is a group end
        const IsGroupEnd     = 1 << 4 | Self::IsSeparator.bits();
    }
}
