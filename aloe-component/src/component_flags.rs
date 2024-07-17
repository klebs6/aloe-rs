crate::ix!();

///-----------------------------
#[bitfield]
#[derive(Copy,Clone)]
pub struct ComponentFlags
{
    has_heavyweight_peer_flag:        B1,
    visible_flag:                     B1,
    opaque_flag:                      B1,
    ignores_mouse_clicks_flag:        B1,
    allow_child_mouse_clicks_flag:    B1,
    wants_keyboard_focus_flag:        B1,
    is_focus_container_flag:          B1,
    is_keyboard_focus_container_flag: B1,
    child_keyboard_focused_flag:      B1,
    dont_focus_on_mouse_click_flag:   B1,
    always_on_top_flag:               B1,
    buffer_to_image_flag:             B1,
    bring_to_front_on_click_flag:     B1,
    repaint_on_mouse_activity_flag:   B1,
    is_disabled_flag:                 B1,
    dont_clip_graphics_flag:          B1,
    mouse_down_was_blocked:           B1,
    is_move_callback_pending:         B1,
    is_resize_callback_pending:       B1,
    viewport_ignore_drag_flag:        B1,
    accessibility_ignored_flag:       B1,
    cached_mouse_inside_component:    B1,

    #[cfg(ALOE_DEBUG)]
    is_inside_paint_call:             B1,

    pad0:                             B1,
}

pub mod component_flags_access {
    use super::*;

    pub union U
    {
        component_flags: u32,
        flags:           ComponentFlags,
    }
}
