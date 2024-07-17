crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/keyboard/aloe_KeyPress.h]

/**
  | Represents a key press, including any
  | modifier keys that are needed.
  | 
  | E.g. a KeyPress might represent CTRL+C,
  | SHIFT+ALT+H, Spacebar, Escape, etc.
  | 
  | @see Component, KeyListener, KeyPressMappingSet,
  | Button::addShortcut
  | 
  | @tags{GUI}
  |
  */
#[derive(Default)]
#[leak_detector]
pub struct KeyPress {
    key_code:       i32, // default = 0
    mods:           ModifierKeys,
    text_character: wchar_t, // default = 0
}

impl PartialEq<i32> for KeyPress {
    
    /**
      | Returns true if this keypress is for
      | the given keycode without any modifiers.
      |
      */
    #[inline] fn eq(&self, other: &i32) -> bool {
        todo!();
        /*
            return keyCode == otherKeyCode && ! mods.isAnyModifierKeyDown();
        */
    }
}

impl PartialEq<KeyPress> for KeyPress {
    
    #[inline] fn eq(&self, other: &KeyPress) -> bool {
        todo!();
        /*
            return mods.getRawFlags() == other.mods.getRawFlags()
                && (textCharacter == other.textCharacter
                     || textCharacter == 0
                     || other.textCharacter == 0)
                && (keyCode == other.keyCode
                     || (keyCode < 256
                          && other.keyCode < 256
                          && CharacterFunctions::toLowerCase ((aloe_wchar) keyCode)
                               == CharacterFunctions::toLowerCase ((aloe_wchar) other.keyCode)));
        */
    }
}

impl Eq for KeyPress {}

impl KeyPress {

    #[cfg(target_os="linux")]
    pub fn is_key_currently_down(&mut self, key_code: i32) -> bool {
        
        todo!();
        /*
            return XWindowSystem::getInstance()->isKeyCurrentlyDown (keyCode);
        */

    }
}

pub mod key_press {
    use super::*;

    /**
      | Key codes
      |
      | Note that the actual values of these are
      | platform-specific and may change without
      | warning, so don't store them anywhere as
      | constants. For persisting/retrieving
      | KeyPress objects, use getTextDescription()
      | and createFromDescription() instead.
      |
      */
    lazy_static!{
        /*
        static const int spaceKey;      /**< key-code for the space bar */
            static const int escapeKey;     /**< key-code for the escape key */
            static const int returnKey;     /**< key-code for the return key*/
            static const int tabKey;        /**< key-code for the tab key*/

            static const int deleteKey;     /**< key-code for the delete key (not backspace) */
            static const int backspaceKey;  /**< key-code for the backspace key */
            static const int insertKey;     /**< key-code for the insert key */

            static const int upKey;         /**< key-code for the cursor-up key */
            static const int downKey;       /**< key-code for the cursor-down key */
            static const int leftKey;       /**< key-code for the cursor-left key */
            static const int rightKey;      /**< key-code for the cursor-right key */
            static const int pageUpKey;     /**< key-code for the page-up key */
            static const int pageDownKey;   /**< key-code for the page-down key */
            static const int homeKey;       /**< key-code for the home key */
            static const int endKey;        /**< key-code for the end key */

            static const int F1Key;         /**< key-code for the F1 key */
            static const int F2Key;         /**< key-code for the F2 key */
            static const int F3Key;         /**< key-code for the F3 key */
            static const int F4Key;         /**< key-code for the F4 key */
            static const int F5Key;         /**< key-code for the F5 key */
            static const int F6Key;         /**< key-code for the F6 key */
            static const int F7Key;         /**< key-code for the F7 key */
            static const int F8Key;         /**< key-code for the F8 key */
            static const int F9Key;         /**< key-code for the F9 key */
            static const int F10Key;        /**< key-code for the F10 key */
            static const int F11Key;        /**< key-code for the F11 key */
            static const int F12Key;        /**< key-code for the F12 key */
            static const int F13Key;        /**< key-code for the F13 key */
            static const int F14Key;        /**< key-code for the F14 key */
            static const int F15Key;        /**< key-code for the F15 key */
            static const int F16Key;        /**< key-code for the F16 key */
            static const int F17Key;        /**< key-code for the F17 key */
            static const int F18Key;        /**< key-code for the F18 key */
            static const int F19Key;        /**< key-code for the F19 key */
            static const int F20Key;        /**< key-code for the F20 key */
            static const int F21Key;        /**< key-code for the F21 key */
            static const int F22Key;        /**< key-code for the F22 key */
            static const int F23Key;        /**< key-code for the F23 key */
            static const int F24Key;        /**< key-code for the F24 key */
            static const int F25Key;        /**< key-code for the F25 key */
            static const int F26Key;        /**< key-code for the F26 key */
            static const int F27Key;        /**< key-code for the F27 key */
            static const int F28Key;        /**< key-code for the F28 key */
            static const int F29Key;        /**< key-code for the F29 key */
            static const int F30Key;        /**< key-code for the F30 key */
            static const int F31Key;        /**< key-code for the F31 key */
            static const int F32Key;        /**< key-code for the F32 key */
            static const int F33Key;        /**< key-code for the F33 key */
            static const int F34Key;        /**< key-code for the F34 key */
            static const int F35Key;        /**< key-code for the F35 key */

            static const int numberPad0;     /**< key-code for the 0 on the numeric keypad. */
            static const int numberPad1;     /**< key-code for the 1 on the numeric keypad. */
            static const int numberPad2;     /**< key-code for the 2 on the numeric keypad. */
            static const int numberPad3;     /**< key-code for the 3 on the numeric keypad. */
            static const int numberPad4;     /**< key-code for the 4 on the numeric keypad. */
            static const int numberPad5;     /**< key-code for the 5 on the numeric keypad. */
            static const int numberPad6;     /**< key-code for the 6 on the numeric keypad. */
            static const int numberPad7;     /**< key-code for the 7 on the numeric keypad. */
            static const int numberPad8;     /**< key-code for the 8 on the numeric keypad. */
            static const int numberPad9;     /**< key-code for the 9 on the numeric keypad. */

            static const int numberPadAdd;            /**< key-code for the add sign on the numeric keypad. */
            static const int numberPadSubtract;       /**< key-code for the subtract sign on the numeric keypad. */
            static const int numberPadMultiply;       /**< key-code for the multiply sign on the numeric keypad. */
            static const int numberPadDivide;         /**< key-code for the divide sign on the numeric keypad. */
            static const int numberPadSeparator;      /**< key-code for the comma on the numeric keypad. */
            static const int numberPadDecimalPoint;   /**< key-code for the decimal point sign on the numeric keypad. */
            static const int numberPadEquals;         /**< key-code for the equals key on the numeric keypad. */
            static const int numberPadDelete;         /**< key-code for the delete key on the numeric keypad. */

            static const int playKey;        /**< key-code for a multimedia 'play' key, (not all keyboards will have one) */
            static const int stopKey;        /**< key-code for a multimedia 'stop' key, (not all keyboards will have one) */
            static const int fastForwardKey; /**< key-code for a multimedia 'fast-forward' key, (not all keyboards will have one) */
            static const int rewindKey;      /**< key-code for a multimedia 'rewind' key, (not all keyboards will have one) */
        */
    }

    pub struct KeyNameAndCode
    {
        name: *const u8,
        code: i32,
    }

    lazy_static!{
        /*
        const KeyNameAndCode translations[] =
            {
                { "spacebar",       KeyPress::spaceKey },
                { "return",         KeyPress::returnKey },
                { "escape",         KeyPress::escapeKey },
                { "backspace",      KeyPress::backspaceKey },
                { "cursor left",    KeyPress::leftKey },
                { "cursor right",   KeyPress::rightKey },
                { "cursor up",      KeyPress::upKey },
                { "cursor down",    KeyPress::downKey },
                { "page up",        KeyPress::pageUpKey },
                { "page down",      KeyPress::pageDownKey },
                { "home",           KeyPress::homeKey },
                { "end",            KeyPress::endKey },
                { "delete",         KeyPress::deleteKey },
                { "insert",         KeyPress::insertKey },
                { "tab",            KeyPress::tabKey },
                { "play",           KeyPress::playKey },
                { "stop",           KeyPress::stopKey },
                { "fast forward",   KeyPress::fastForwardKey },
                { "rewind",         KeyPress::rewindKey }
            };
        */
    }

    pub struct ModifierDescription
    {
        name: *const u8,
        flag: i32,
    }

    lazy_static!{
        /*
        static const ModifierDescription modifierNames[] =
            {
                { "ctrl",      ModifierKeys::ctrlModifier },
                { "control",   ModifierKeys::ctrlModifier },
                { "ctl",       ModifierKeys::ctrlModifier },
                { "shift",     ModifierKeys::shiftModifier },
                { "shft",      ModifierKeys::shiftModifier },
                { "alt",       ModifierKeys::altModifier },
                { "option",    ModifierKeys::altModifier },
                { "command",   ModifierKeys::commandModifier },
                { "cmd",       ModifierKeys::commandModifier }
            };
        */
    }

    pub fn number_pad_prefix() -> *const u8 {
        
        todo!();
        /*
            return "numpad ";
        */
    }

    pub fn get_numpad_key_code(desc: &String) -> i32 {
        
        todo!();
        /*
            if (desc.containsIgnoreCase (numberPadPrefix()))
                {
                    auto lastChar = desc.trimEnd().getLastCharacter();

                    switch (lastChar)
                    {
                        case '0': case '1': case '2': case '3': case '4':
                        case '5': case '6': case '7': case '8': case '9':
                            return (int) (KeyPress::numberPad0 + (int) lastChar - '0');

                        case '+':   return KeyPress::numberPadAdd;
                        case '-':   return KeyPress::numberPadSubtract;
                        case '*':   return KeyPress::numberPadMultiply;
                        case '/':   return KeyPress::numberPadDivide;
                        case '.':   return KeyPress::numberPadDecimalPoint;
                        case '=':   return KeyPress::numberPadEquals;

                        default:    break;
                    }

                    if (desc.endsWith ("separator"))  return KeyPress::numberPadSeparator;
                    if (desc.endsWith ("delete"))     return KeyPress::numberPadDelete;
                }

                return 0;
        */
    }

    #[cfg(any(target_os="macos",target_os="ios"))]
    pub struct OSXSymbolReplacement
    {
        text:   *const u8,
        symbol: wchar_t,
    }

    #[cfg(any(target_os="macos",target_os="ios"))]
    lazy_static!{
        /*
        const OSXSymbolReplacement osxSymbols[] =
            {
                { "shift + ",     0x21e7 },
                { "command + ",   0x2318 },
                { "option + ",    0x2325 },
                { "ctrl + ",      0x2303 },
                { "return",       0x21b5 },
                { "cursor left",  0x2190 },
                { "cursor right", 0x2192 },
                { "cursor up",    0x2191 },
                { "cursor down",  0x2193 },
                { "backspace",    0x232b },
                { "delete",       0x2326 },
                { "spacebar",     0x2423 }
            };
        */
    }
}


//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/keyboard/aloe_KeyPress.cpp]
impl KeyPress {

    /**
      | Returns true if this is a valid KeyPress.
      | 
      | A null keypress can be created by the
      | default constructor, in case it's needed.
      |
      */
    pub fn is_valid(&self) -> bool {
        
        todo!();
        /*
            return keyCode != 0;
        */
    }

    /**
      | Returns the key code itself.
      | 
      | This will either be one of the special
      | constants defined in this class, or
      | an 8-bit character code.
      |
      */
    pub fn get_key_code(&self) -> i32 {
        
        todo!();
        /*
            return keyCode;
        */
    }

    /**
      | Returns the key modifiers.
      | 
      | @see ModifierKeys
      |
      */
    pub fn get_modifiers(&self) -> ModifierKeys {
        
        todo!();
        /*
            return mods;
        */
    }

    /**
      | Returns the character that is associated
      | with this keypress.
      | 
      | This is the character that you'd expect
      | to see printed if you press this keypress
      | in a text editor or similar component.
      |
      */
    pub fn get_text_character(&self) -> wchar_t {
        
        todo!();
        /*
            return textCharacter;
        */
    }

    /**
      | Checks whether the KeyPress's key is
      | the same as the one provided, without
      | checking the modifiers.
      | 
      | The values for key codes can either be
      | one of the special constants defined
      | in this class, or an 8-bit character
      | code.
      | 
      | @see getKeyCode
      |
      */
    pub fn is_key_code(&self, key_code_to_compare: i32) -> bool {
        
        todo!();
        /*
            return keyCode == keyCodeToCompare;
        */
    }
    
    /**
      | Checks whether a particular key is held
      | down, irrespective of modifiers.
      | 
      | The values for key codes can either be
      | one of the special constants defined
      | in this class, or an 8-bit character
      | code.
      |
      */
    pub fn is_key_currently_down(key_code: i32) -> bool {
        
        todo!();
        /*
        
        */
    }
    
    /**
      | Creates a KeyPress for a key and some
      | modifiers.
      | 
      | e.g.
      | 
      | CTRL+C would be: KeyPress ('c', ModifierKeys::ctrlModifier,
      | 0)
      | 
      | SHIFT+Escape would be: KeyPress (KeyPress::escapeKey,
      | ModifierKeys::shiftModifier, 0)
      | 
      | -----------
      | @param keyCode
      | 
      | a code that represents the key - this
      | value must be one of special constants
      | listed in this class, or an 8-bit character
      | code such as a letter (case is ignored),
      | digit or a simple key like "," or ".".
      | Note that this isn't the same as the textCharacter
      | parameter, so for example a keyCode
      | of 'a' and a shift-key modifier should
      | have a textCharacter value of 'A'.
      | ----------
      | @param modifiers
      | 
      | the modifiers to associate with the
      | keystroke
      | ----------
      | @param textCharacter
      | 
      | the character that would be printed
      | if someone typed this keypress into
      | a text editor. This value may be null
      | if the keypress is a non-printing character
      | @see getKeyCode, isKeyCode, getModifiers
      |
      */
    pub fn new(
        code:      i32,
        m:         ModifierKeys,
        text_char: wchar_t) -> Self {
    
        todo!();
        /*
        : key_code(code),
        : mods(m),
        : text_character(textChar),

        
        */
    }
    
    /**
      | Creates a keypress with a keyCode but
      | no modifiers or text character.
      |
      */
    pub fn new_with_code(code: i32) -> Self {
    
        todo!();
        /*
        : key_code(code),

        
        */
    }
    
    /**
      | Checks whether the user is currently
      | holding down the keys that make up this
      | 
      | KeyPress.
      | 
      | -----------
      | @note
      | 
      | this will return false if any extra modifier
      | keys are down - e.g. if the keypress is
      | CTRL+X and the user is actually holding
      | CTRL+ALT+x then it will be false.
      |
      */
    pub fn is_currently_down(&self) -> bool {
        
        todo!();
        /*
            return isKeyCurrentlyDown (keyCode)
                && (ModifierKeys::currentModifiers.getRawFlags() & ModifierKeys::allKeyboardModifiers)
                      == (mods.getRawFlags() & ModifierKeys::allKeyboardModifiers);
        */
    }
    
    /**
      | Converts a textual key description
      | to a KeyPress.
      | 
      | This attempts to decode a textual version
      | of a keypress, e.g. "ctrl + c" or "spacebar".
      | 
      | This isn't designed to cope with any
      | kind of input, but should be given the
      | strings that are created by the getTextDescription()
      | method.
      | 
      | If the string can't be parsed, the object
      | returned will be invalid.
      | 
      | @see getTextDescription
      |
      */
    pub fn create_from_description(&mut self, desc: &String) -> KeyPress {
        
        todo!();
        /*
            int modifiers = 0;

        for (int i = 0; i < numElementsInArray (KeyPressHelpers::modifierNames); ++i)
            if (desc.containsWholeWordIgnoreCase (KeyPressHelpers::modifierNames[i].name))
                modifiers |= KeyPressHelpers::modifierNames[i].flag;

        int key = 0;

        for (int i = 0; i < numElementsInArray (KeyPressHelpers::translations); ++i)
        {
            if (desc.containsWholeWordIgnoreCase (String (KeyPressHelpers::translations[i].name)))
            {
                key = KeyPressHelpers::translations[i].code;
                break;
            }
        }

        if (key == 0)
            key = KeyPressHelpers::getNumpadKeyCode (desc);

        if (key == 0)
        {
            // see if it's a function key..
            if (! desc.containsChar ('#')) // avoid mistaking hex-codes like "#f1"
            {
                for (int i = 1; i <= 35; ++i)
                {
                    if (desc.containsWholeWordIgnoreCase ("f" + String (i)))
                    {
                        if (i <= 16)        key = F1Key + i - 1;
                        else if (i <= 24)   key = F17Key + i - 17;
                        else if (i <= 35)   key = F25Key + i - 25;
                    }
                }
            }

            if (key == 0)
            {
                // give up and use the hex code..
                auto hexCode = desc.fromFirstOccurrenceOf ("#", false, false)
                                   .retainCharacters ("0123456789abcdefABCDEF")
                                   .getHexValue32();

                if (hexCode > 0)
                    key = hexCode;
                else
                    key = (int) CharacterFunctions::toUpperCase (desc.getLastCharacter());
            }
        }

        return KeyPress (key, ModifierKeys (modifiers), 0);
        */
    }
    
    /**
      | Creates a textual description of the
      | key combination.
      | 
      | e.g. "ctrl + c" or "delete".
      | 
      | To store a keypress in a file, use this
      | method, along with createFromDescription()
      | to retrieve it later.
      |
      */
    pub fn get_text_description(&self) -> String {
        
        todo!();
        /*
            String desc;

        if (keyCode > 0)
        {
            // some keyboard layouts use a shift-key to get the slash, but in those cases, we
            // want to store it as being a slash, not shift+whatever.
            if (textCharacter == '/' && keyCode != numberPadDivide)
                return "/";

            if (mods.isCtrlDown())      desc << "ctrl + ";
            if (mods.isShiftDown())     desc << "shift + ";

           #if ALOE_MAC || ALOE_IOS
            if (mods.isAltDown())       desc << "option + ";
            if (mods.isCommandDown())   desc << "command + ";
           #else
            if (mods.isAltDown())       desc << "alt + ";
           #endif

            for (int i = 0; i < numElementsInArray (KeyPressHelpers::translations); ++i)
                if (keyCode == KeyPressHelpers::translations[i].code)
                    return desc + KeyPressHelpers::translations[i].name;

            // not all F keys have consecutive key codes on all platforms
            if      (keyCode >= F1Key  && keyCode <= F16Key)                  desc << 'F' << (1 + keyCode - F1Key);
            else if (keyCode >= F17Key && keyCode <= F24Key)                  desc << 'F' << (17 + keyCode - F17Key);
            else if (keyCode >= F25Key && keyCode <= F35Key)                  desc << 'F' << (25 + keyCode - F25Key);
            else if (keyCode >= numberPad0 && keyCode <= numberPad9)    desc << KeyPressHelpers::numberPadPrefix() << (keyCode - numberPad0);
            else if (keyCode >= 33 && keyCode < 176)        desc += CharacterFunctions::toUpperCase ((aloe_wchar) keyCode);
            else if (keyCode == numberPadAdd)               desc << KeyPressHelpers::numberPadPrefix() << '+';
            else if (keyCode == numberPadSubtract)          desc << KeyPressHelpers::numberPadPrefix() << '-';
            else if (keyCode == numberPadMultiply)          desc << KeyPressHelpers::numberPadPrefix() << '*';
            else if (keyCode == numberPadDivide)            desc << KeyPressHelpers::numberPadPrefix() << '/';
            else if (keyCode == numberPadSeparator)         desc << KeyPressHelpers::numberPadPrefix() << "separator";
            else if (keyCode == numberPadDecimalPoint)      desc << KeyPressHelpers::numberPadPrefix() << '.';
            else if (keyCode == numberPadEquals)            desc << KeyPressHelpers::numberPadPrefix() << '=';
            else if (keyCode == numberPadDelete)            desc << KeyPressHelpers::numberPadPrefix() << "delete";
            else                                            desc << '#' << String::toHexString (keyCode);
        }

        return desc;
        */
    }
    
    /**
      | Creates a textual description of the
      | key combination, using unicode icon
      | symbols if possible.
      | 
      | On OSX, this uses the Apple symbols for
      | command, option, shift, etc, instead
      | of the textual modifier key descriptions
      | that are returned by getTextDescription()
      |
      */
    pub fn get_text_description_with_icons(&self) -> String {
        
        todo!();
        /*
            #if ALOE_MAC || ALOE_IOS
        auto s = getTextDescription();

        for (int i = 0; i < numElementsInArray (KeyPressHelpers::osxSymbols); ++i)
            s = s.replace (KeyPressHelpers::osxSymbols[i].text,
                           String::charToString (KeyPressHelpers::osxSymbols[i].symbol));

        return s;
       #else
        return getTextDescription();
       #endif
        */
    }
}
