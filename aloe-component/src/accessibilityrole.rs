crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/accessibility/enums/aloe_AccessibilityRole.h]

/**
  | The list of available roles for an AccessibilityHandler
  | object.
  | 
  | When creating a custom AccessibilityHandler
  | you should select the role that best
  | describes the UI element being represented.
  | 
  | @tags{Accessibility}
  |
  */
pub enum AccessibilityRole
{
    button,
    toggleButton,
    radioButton,
    comboBox,
    image,
    slider,
    label,
    staticText,
    editableText,
    menuItem,
    menuBar,
    popupMenu,
    table,
    tableHeader,
    column,
    row,
    cell,
    hyperlink,
    list,
    listItem,
    tree,
    treeItem,
    progressBar,
    group,
    dialogWindow,
    window,
    scrollBar,
    tooltip,
    splashScreen,
    ignored,
    unspecified
}
