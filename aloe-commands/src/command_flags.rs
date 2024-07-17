crate::ix!();

/** 
  | Flags describing the ways in which this
  | command should be used.
  |
  | A bitwise-OR of these values is stored in
  | the ApplicationCommandInfo::flags variable.
  */
pub enum ApplicationCommandInfoCommandFlags
{
    /** 
      | Indicates that the command can't
      | currently be performed.
      |
      | The
      | ApplicationCommandTarget::getCommandInfo()
      | method must set this flag if it's not
      | currently permissible to perform the
      | command. If the flag is set, then
      | components that trigger the command,
      | e.g. PopupMenu, may choose to grey-out
      | the command or show themselves as not
      | being enabled.
      |
      | @see ApplicationCommandInfo::setActive
      */
    isDisabled                  = 1 << 0,

    /** 
      | Indicates that the command should have
      | a tick next to it on a menu.
      |
      | If your command is shown on a menu and
      | this is set, it'll show a tick next to
      | it. Other components such as buttons may
      | also use this flag to indicate that it
      | is a value that can be toggled, and is
      | currently in the 'on' state.
      |
      | @see ApplicationCommandInfo::setTicked
      */
    isTicked                    = 1 << 1,

    /** 
      | If this flag is present, then when
      | a KeyPressMappingSet invokes the
      | command, it will call the command twice,
      | once on key-down and again on key-up.
      |
      | @see
      | ApplicationCommandTarget::InvocationInfo
      */
    wantsKeyUpDownCallbacks     = 1 << 2,

    /** 
      | If this flag is present, then
      | a KeyMappingEditorComponent will not
      | display the command in its list.
      */
    hiddenFromKeyEditor         = 1 << 3,

    /** 
      | If this flag is present, then
      | a KeyMappingEditorComponent will display
      | the command in its list, but won't allow
      | the assigned keypress to be changed.
      */
    readOnlyInKeyEditor         = 1 << 4,

    /** 
      | If this flag is present and the command
      | is invoked from a keypress, then any
      | buttons or menus that are also connected
      | to the command will not flash to indicate
      | that they've been triggered.
      */
    dontTriggerVisualFeedback   = 1 << 5
}
