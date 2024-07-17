crate::ix!();

bitflags! {

    /**
      | A combination of these flags are passed
      | into the addColumn() method to specify
      | the properties of a column.
      |
      */
    pub struct TableHeaderComponentColumnPropertyFlags: u32 {

        /**
          | If this is set, the column will be shown;
          | if not, it will be hidden until the user
          | enables it with the pop-up menu.
          |
          */
        const VISIBLE                   = 1;

        /**
          | If this is set, the column can be resized
          | by dragging it.
          |
          */
        const RESIZABLE                 = 2;

        /**
          | If this is set, the column can be dragged
          | around to change its order in the table.
          |
          */
        const DRAGGABLE                 = 4;

        /**
          | If this is set, the column will be shown
          | on the pop-up menu allowing it to be hidden/shown.
          |
          */
        const APPEARS_ON_COLUMN_MENU    = 8;

        /**
          | If this is set, then clicking on the column
          | header will set it to be the sort column,
          | and clicking again will reverse the
          | order.
          |
          */
        const SORTABLE                  = 16;

        /**
          | If this is set, the column is currently
          | the one by which the table is sorted (forwards).
          |
          */
        const SORTED_FORWARDS           = 32;

        /**
          | If this is set, the column is currently
          | the one by which the table is sorted (backwards).
          |
          */
        const SORTED_BACKWARDS          = 64;

        /**
          | This set of default flags is used as the
          | default parameter value in addColumn().
          |
          */
        const DEFAULT_FLAGS
            = Self::VISIBLE.bits() 
            | Self::RESIZABLE.bits() 
            | Self::DRAGGABLE.bits() 
            | Self::APPEARS_ON_COLUMN_MENU.bits() 
            | Self::SORTABLE.bits();

        /**
          | A quick way of combining flags for a column
          | that's not resizable.
          |
          */
        const NOT_RESIZABLE             
            = Self::VISIBLE.bits() 
            | Self::DRAGGABLE.bits() 
            | Self::APPEARS_ON_COLUMN_MENU.bits() 
            | Self::SORTABLE.bits();

        /**
          | A quick way of combining flags for a column
          | that's not resizable or sortable.
          |
          */
        const NOT_RESIZABLE_OR_SORTABLE 
            = Self::VISIBLE.bits() 
            | Self::DRAGGABLE.bits() 
            | Self::APPEARS_ON_COLUMN_MENU.bits();

        /**
          | A quick way of combining flags for a column
          | that's not sortable.
          |
          */
        const NOT_SORTABLE              
            = Self::VISIBLE.bits() 
            | Self::RESIZABLE.bits() 
            | Self::DRAGGABLE.bits() 
            | Self::APPEARS_ON_COLUMN_MENU.bits();
    }
}
