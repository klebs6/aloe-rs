#![feature(allocator_api)]

#[macro_use] mod imports; use imports::*;

x!{cached_glyph_edge_table}
x!{clip_regions}
x!{drawfont}
x!{drop_shadow}
x!{edge_table_fillers}
x!{filltype}
x!{float_rectangle_rasterising_info}
x!{glow}
x!{glyph_cache}
x!{gradient_pixel_color}
x!{graphicscontext}
x!{graphics}
x!{imageeffectfilter}
x!{image}
x!{linux_iconhelpers}
x!{lowlevelgraphicscontext}
x!{lowlevelgraphicspostscriptrenderer}
x!{lowlevelgraphicssoftwarerenderer}

#[cfg(target_os="macos")]
x!{mac_coregraphicscontext}

#[cfg(target_os="macos")]
x!{mac_coregraphicshelpers}

#[cfg(target_os="macos")]
x!{mac_iconhelpers}

x!{rectangleplacement}
x!{saved_state_base}
x!{saved_state_stack}
x!{software_renderer_saved_state}
x!{stack_based_low_level_graphics_context}
x!{translation_or_transform}
