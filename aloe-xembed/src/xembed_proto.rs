crate::ix!();

pub const maxXEmbedVersionToSupport:     usize     = 0;
pub const XEMBED_MAPPED:                 usize     = (1<<0);

pub const XEMBED_EMBEDDED_NOTIFY:        usize     = 0;
pub const XEMBED_WINDOW_ACTIVATE:        usize     = 1;
pub const XEMBED_WINDOW_DEACTIVATE:      usize     = 2;
pub const XEMBED_REQUEST_FOCUS:          usize     = 3;
pub const XEMBED_FOCUS_IN:               usize     = 4;
pub const XEMBED_FOCUS_OUT:              usize     = 5;
pub const XEMBED_FOCUS_NEXT:             usize     = 6;
pub const XEMBED_FOCUS_PREV:             usize     = 7;
pub const XEMBED_MODALITY_ON:            usize     = 10;
pub const XEMBED_MODALITY_OFF:           usize     = 11;
pub const XEMBED_REGISTER_ACCELERATOR:   usize     = 12;
pub const XEMBED_UNREGISTER_ACCELERATOR: usize     = 13;
pub const XEMBED_ACTIVATE_ACCELERATOR:   usize     = 14;

pub const XEMBED_FOCUS_CURRENT:          usize     = 0;
pub const XEMBED_FOCUS_FIRST:            usize     = 1;
pub const XEMBED_FOCUS_LAST:             usize     = 2;

