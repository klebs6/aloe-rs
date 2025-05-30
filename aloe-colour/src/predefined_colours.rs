crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/colour/aloe_Colours.h]
//-------------------------------------------[.cpp/Aloe/modules/aloe_graphics/colour/aloe_Colours.cpp]

/**
  | Contains a set of predefined named colours
  | (mostly standard HTML colours)
  | 
  | @see Colour
  | 
  | @tags{Graphics}
  |
  */
pub mod colours {

    use super::*;

    macro_rules! colour { ($name:ident => $argb:expr) => { pub const $name: Colour = Colour::new_from_argb($argb); } }

    colour![transparent_black    => 0];
    colour![transparent_white    => 0x00ffffff];
    colour![aliceblue            => 0xfff0f8ff];
    colour![antiquewhite         => 0xfffaebd7];
    colour![aqua                 => 0xff00ffff];
    colour![aquamarine           => 0xff7fffd4];
    colour![azure                => 0xfff0ffff];
    colour![beige                => 0xfff5f5dc];
    colour![bisque               => 0xffffe4c4];
    colour![black                => 0xff000000];
    colour![blanchedalmond       => 0xffffebcd];
    colour![blue                 => 0xff0000ff];
    colour![blueviolet           => 0xff8a2be2];
    colour![brown                => 0xffa52a2a];
    colour![burlywood            => 0xffdeb887];
    colour![cadetblue            => 0xff5f9ea0];
    colour![chartreuse           => 0xff7fff00];
    colour![chocolate            => 0xffd2691e];
    colour![coral                => 0xffff7f50];
    colour![cornflowerblue       => 0xff6495ed];
    colour![cornsilk             => 0xfffff8dc];
    colour![crimson              => 0xffdc143c];
    colour![cyan                 => 0xff00ffff];
    colour![darkblue             => 0xff00008b];
    colour![darkcyan             => 0xff008b8b];
    colour![darkgoldenrod        => 0xffb8860b];
    colour![darkgrey             => 0xff555555];
    colour![darkgreen            => 0xff006400];
    colour![darkkhaki            => 0xffbdb76b];
    colour![darkmagenta          => 0xff8b008b];
    colour![darkolivegreen       => 0xff556b2f];
    colour![darkorange           => 0xffff8c00];
    colour![darkorchid           => 0xff9932cc];
    colour![darkred              => 0xff8b0000];
    colour![darksalmon           => 0xffe9967a];
    colour![darkseagreen         => 0xff8fbc8f];
    colour![darkslateblue        => 0xff483d8b];
    colour![darkslategrey        => 0xff2f4f4f];
    colour![darkturquoise        => 0xff00ced1];
    colour![darkviolet           => 0xff9400d3];
    colour![deeppink             => 0xffff1493];
    colour![deepskyblue          => 0xff00bfff];
    colour![dimgrey              => 0xff696969];
    colour![dodgerblue           => 0xff1e90ff];
    colour![firebrick            => 0xffb22222];
    colour![floralwhite          => 0xfffffaf0];
    colour![forestgreen          => 0xff228b22];
    colour![fuchsia              => 0xffff00ff];
    colour![gainsboro            => 0xffdcdcdc];
    colour![ghostwhite           => 0xfff8f8ff];
    colour![gold                 => 0xffffd700];
    colour![goldenrod            => 0xffdaa520];
    colour![grey                 => 0xff808080];
    colour![green                => 0xff008000];
    colour![greenyellow          => 0xffadff2f];
    colour![honeydew             => 0xfff0fff0];
    colour![hotpink              => 0xffff69b4];
    colour![indianred            => 0xffcd5c5c];
    colour![indigo               => 0xff4b0082];
    colour![ivory                => 0xfffffff0];
    colour![khaki                => 0xfff0e68c];
    colour![lavender             => 0xffe6e6fa];
    colour![lavenderblush        => 0xfffff0f5];
    colour![lawngreen            => 0xff7cfc00];
    colour![lemonchiffon         => 0xfffffacd];
    colour![lightblue            => 0xffadd8e6];
    colour![lightcoral           => 0xfff08080];
    colour![lightcyan            => 0xffe0ffff];
    colour![lightgoldenrodyellow => 0xfffafad2];
    colour![lightgreen           => 0xff90ee90];
    colour![lightgrey            => 0xffd3d3d3];
    colour![lightpink            => 0xffffb6c1];
    colour![lightsalmon          => 0xffffa07a];
    colour![lightseagreen        => 0xff20b2aa];
    colour![lightskyblue         => 0xff87cefa];
    colour![lightslategrey       => 0xff778899];
    colour![lightsteelblue       => 0xffb0c4de];
    colour![lightyellow          => 0xffffffe0];
    colour![lime                 => 0xff00ff00];
    colour![limegreen            => 0xff32cd32];
    colour![linen                => 0xfffaf0e6];
    colour![magenta              => 0xffff00ff];
    colour![maroon               => 0xff800000];
    colour![mediumaquamarine     => 0xff66cdaa];
    colour![mediumblue           => 0xff0000cd];
    colour![mediumorchid         => 0xffba55d3];
    colour![mediumpurple         => 0xff9370db];
    colour![mediumseagreen       => 0xff3cb371];
    colour![mediumslateblue      => 0xff7b68ee];
    colour![mediumspringgreen    => 0xff00fa9a];
    colour![mediumturquoise      => 0xff48d1cc];
    colour![mediumvioletred      => 0xffc71585];
    colour![midnightblue         => 0xff191970];
    colour![mintcream            => 0xfff5fffa];
    colour![mistyrose            => 0xffffe4e1];
    colour![moccasin             => 0xffffe4b5];
    colour![navajowhite          => 0xffffdead];
    colour![navy                 => 0xff000080];
    colour![oldlace              => 0xfffdf5e6];
    colour![olive                => 0xff808000];
    colour![olivedrab            => 0xff6b8e23];
    colour![orange               => 0xffffa500];
    colour![orangered            => 0xffff4500];
    colour![orchid               => 0xffda70d6];
    colour![palegoldenrod        => 0xffeee8aa];
    colour![palegreen            => 0xff98fb98];
    colour![paleturquoise        => 0xffafeeee];
    colour![palevioletred        => 0xffdb7093];
    colour![papayawhip           => 0xffffefd5];
    colour![peachpuff            => 0xffffdab9];
    colour![peru                 => 0xffcd853f];
    colour![pink                 => 0xffffc0cb];
    colour![plum                 => 0xffdda0dd];
    colour![powderblue           => 0xffb0e0e6];
    colour![purple               => 0xff800080];
    colour![rebeccapurple        => 0xff663399];
    colour![red                  => 0xffff0000];
    colour![rosybrown            => 0xffbc8f8f];
    colour![royalblue            => 0xff4169e1];
    colour![saddlebrown          => 0xff8b4513];
    colour![salmon               => 0xfffa8072];
    colour![sandybrown           => 0xfff4a460];
    colour![seagreen             => 0xff2e8b57];
    colour![seashell             => 0xfffff5ee];
    colour![sienna               => 0xffa0522d];
    colour![silver               => 0xffc0c0c0];
    colour![skyblue              => 0xff87ceeb];
    colour![slateblue            => 0xff6a5acd];
    colour![slategrey            => 0xff708090];
    colour![snow                 => 0xfffffafa];
    colour![springgreen          => 0xff00ff7f];
    colour![steelblue            => 0xff4682b4];
    colour![tan                  => 0xffd2b48c];
    colour![teal                 => 0xff008080];
    colour![thistle              => 0xffd8bfd8];
    colour![tomato               => 0xffff6347];
    colour![turquoise            => 0xff40e0d0];
    colour![violet               => 0xffee82ee];
    colour![wheat                => 0xfff5deb3];
    colour![white                => 0xffffffff];
    colour![whitesmoke           => 0xfff5f5f5];
    colour![yellow               => 0xffffff00];
    colour![yellowgreen          => 0xff9acd32];

    /**
      | Attempts to look up a string in the list
      | of known colour names, and return the
      | appropriate colour.
      | 
      | A non-case-sensitive search is made
      | of the list of predefined colours, and
      | if a match is found, that colour is returned.
      | If no match is found, the colour passed
      | in as the defaultColour parameter is
      | returned.
      |
      */
    pub fn find_colour_for_name(
        colour_name:    &String,
        default_colour: Colour) -> Colour {
        
        todo!();
        /*
            struct StringHashAndColour { uint32 stringHash, colour; };

        static const StringHashAndColour presets[]
        {
            { 0x05978fff, 0xff000000 }, /* black */
            { 0x06bdcc29, 0xffffffff }, /* white */
            { 0x002e305a, 0xff0000ff }, /* blue */
            { 0x00308adf, 0xff808080 }, /* grey */
            { 0x05e0cf03, 0xff008000 }, /* green */
            { 0x0001b891, 0xffff0000 }, /* red */
            { 0xd43c6474, 0xffffff00 }, /* yellow */
            { 0x620886da, 0xfff0f8ff }, /* aliceblue */
            { 0x20a2676a, 0xfffaebd7 }, /* antiquewhite */
            { 0x002dcebc, 0xff00ffff }, /* aqua */
            { 0x46bb5f7e, 0xff7fffd4 }, /* aquamarine */
            { 0x0590228f, 0xfff0ffff }, /* azure */
            { 0x05947fe4, 0xfff5f5dc }, /* beige */
            { 0xad388e35, 0xffffe4c4 }, /* bisque */
            { 0x00674f7e, 0xffffebcd }, /* blanchedalmond */
            { 0x39129959, 0xff8a2be2 }, /* blueviolet */
            { 0x059a8136, 0xffa52a2a }, /* brown */
            { 0x89cea8f9, 0xffdeb887 }, /* burlywood */
            { 0x0fa260cf, 0xff5f9ea0 }, /* cadetblue */
            { 0x6b748956, 0xff7fff00 }, /* chartreuse */
            { 0x2903623c, 0xffd2691e }, /* chocolate */
            { 0x05a74431, 0xffff7f50 }, /* coral */
            { 0x618d42dd, 0xff6495ed }, /* cornflowerblue */
            { 0xe4b479fd, 0xfffff8dc }, /* cornsilk */
            { 0x3d8c4edf, 0xffdc143c }, /* crimson */
            { 0x002ed323, 0xff00ffff }, /* cyan */
            { 0x67cc74d0, 0xff00008b }, /* darkblue */
            { 0x67cd1799, 0xff008b8b }, /* darkcyan */
            { 0x31bbd168, 0xffb8860b }, /* darkgoldenrod */
            { 0x67cecf55, 0xff555555 }, /* darkgrey */
            { 0x920b194d, 0xff006400 }, /* darkgreen */
            { 0x923edd4c, 0xffbdb76b }, /* darkkhaki */
            { 0x5c293873, 0xff8b008b }, /* darkmagenta */
            { 0x6b6671fe, 0xff556b2f }, /* darkolivegreen */
            { 0xbcfd2524, 0xffff8c00 }, /* darkorange */
            { 0xbcfdf799, 0xff9932cc }, /* darkorchid */
            { 0x55ee0d5b, 0xff8b0000 }, /* darkred */
            { 0xc2e5f564, 0xffe9967a }, /* darksalmon */
            { 0x61be858a, 0xff8fbc8f }, /* darkseagreen */
            { 0xc2b0f2bd, 0xff483d8b }, /* darkslateblue */
            { 0xc2b34d42, 0xff2f4f4f }, /* darkslategrey */
            { 0x7cf2b06b, 0xff00ced1 }, /* darkturquoise */
            { 0xc8769375, 0xff9400d3 }, /* darkviolet */
            { 0x25832862, 0xffff1493 }, /* deeppink */
            { 0xfcad568f, 0xff00bfff }, /* deepskyblue */
            { 0x634c8b67, 0xff696969 }, /* dimgrey */
            { 0x45c1ce55, 0xff1e90ff }, /* dodgerblue */
            { 0xef19e3cb, 0xffb22222 }, /* firebrick */
            { 0xb852b195, 0xfffffaf0 }, /* floralwhite */
            { 0xd086fd06, 0xff228b22 }, /* forestgreen */
            { 0xe106b6d7, 0xffff00ff }, /* fuchsia */
            { 0x7880d61e, 0xffdcdcdc }, /* gainsboro */
            { 0x2018a2fa, 0xfff8f8ff }, /* ghostwhite */
            { 0x00308060, 0xffffd700 }, /* gold */
            { 0xb3b3bc1e, 0xffdaa520 }, /* goldenrod */
            { 0xbab8a537, 0xffadff2f }, /* greenyellow */
            { 0xe4cacafb, 0xfff0fff0 }, /* honeydew */
            { 0x41892743, 0xffff69b4 }, /* hotpink */
            { 0xd5796f1a, 0xffcd5c5c }, /* indianred */
            { 0xb969fed2, 0xff4b0082 }, /* indigo */
            { 0x05fef6a9, 0xfffffff0 }, /* ivory */
            { 0x06149302, 0xfff0e68c }, /* khaki */
            { 0xad5a05c7, 0xffe6e6fa }, /* lavender */
            { 0x7c4d5b99, 0xfffff0f5 }, /* lavenderblush */
            { 0x41cc4377, 0xff7cfc00 }, /* lawngreen */
            { 0x195756f0, 0xfffffacd }, /* lemonchiffon */
            { 0x28e4ea70, 0xffadd8e6 }, /* lightblue */
            { 0xf3c7ccdb, 0xfff08080 }, /* lightcoral */
            { 0x28e58d39, 0xffe0ffff }, /* lightcyan */
            { 0x21234e3c, 0xfffafad2 }, /* lightgoldenrodyellow */
            { 0xf40157ad, 0xff90ee90 }, /* lightgreen */
            { 0x28e744f5, 0xffd3d3d3 }, /* lightgrey */
            { 0x28eb3b8c, 0xffffb6c1 }, /* lightpink */
            { 0x9fb78304, 0xffffa07a }, /* lightsalmon */
            { 0x50632b2a, 0xff20b2aa }, /* lightseagreen */
            { 0x68fb7b25, 0xff87cefa }, /* lightskyblue */
            { 0xa8a35ba2, 0xff778899 }, /* lightslategrey */
            { 0xa20d484f, 0xffb0c4de }, /* lightsteelblue */
            { 0xaa2cf10a, 0xffffffe0 }, /* lightyellow */
            { 0x0032afd5, 0xff00ff00 }, /* lime */
            { 0x607bbc4e, 0xff32cd32 }, /* limegreen */
            { 0x06234efa, 0xfffaf0e6 }, /* linen */
            { 0x316858a9, 0xffff00ff }, /* magenta */
            { 0xbf8ca470, 0xff800000 }, /* maroon */
            { 0xbd58e0b3, 0xff66cdaa }, /* mediumaquamarine */
            { 0x967dfd4f, 0xff0000cd }, /* mediumblue */
            { 0x056f5c58, 0xffba55d3 }, /* mediumorchid */
            { 0x07556b71, 0xff9370db }, /* mediumpurple */
            { 0x5369b689, 0xff3cb371 }, /* mediumseagreen */
            { 0x066be19e, 0xff7b68ee }, /* mediumslateblue */
            { 0x3256b281, 0xff00fa9a }, /* mediumspringgreen */
            { 0xc0ad9f4c, 0xff48d1cc }, /* mediumturquoise */
            { 0x628e63dd, 0xffc71585 }, /* mediumvioletred */
            { 0x168eb32a, 0xff191970 }, /* midnightblue */
            { 0x4306b960, 0xfff5fffa }, /* mintcream */
            { 0x4cbc0e6b, 0xffffe4e1 }, /* mistyrose */
            { 0xd9447d59, 0xffffe4b5 }, /* moccasin */
            { 0xe97218a6, 0xffffdead }, /* navajowhite */
            { 0x00337bb6, 0xff000080 }, /* navy */
            { 0xadd2d33e, 0xfffdf5e6 }, /* oldlace */
            { 0x064ee1db, 0xff808000 }, /* olive */
            { 0x9e33a98a, 0xff6b8e23 }, /* olivedrab */
            { 0xc3de262e, 0xffffa500 }, /* orange */
            { 0x58bebba3, 0xffff4500 }, /* orangered */
            { 0xc3def8a3, 0xffda70d6 }, /* orchid */
            { 0x28cb4834, 0xffeee8aa }, /* palegoldenrod */
            { 0x3d9dd619, 0xff98fb98 }, /* palegreen */
            { 0x74022737, 0xffafeeee }, /* paleturquoise */
            { 0x15e2ebc8, 0xffdb7093 }, /* palevioletred */
            { 0x5fd898e2, 0xffffefd5 }, /* papayawhip */
            { 0x93e1b776, 0xffffdab9 }, /* peachpuff */
            { 0x003472f8, 0xffcd853f }, /* peru */
            { 0x00348176, 0xffffc0cb }, /* pink */
            { 0x00348d94, 0xffdda0dd }, /* plum */
            { 0xd036be93, 0xffb0e0e6 }, /* powderblue */
            { 0xc5c507bc, 0xff800080 }, /* purple */
            { 0xf381f607, 0xff663399 }, /* rebeccapurple */
            { 0xa89d65b3, 0xffbc8f8f }, /* rosybrown */
            { 0xbd9413e1, 0xff4169e1 }, /* royalblue */
            { 0xf456044f, 0xff8b4513 }, /* saddlebrown */
            { 0xc9c6f66e, 0xfffa8072 }, /* salmon */
            { 0x0bb131e1, 0xfff4a460 }, /* sandybrown */
            { 0x34636c14, 0xff2e8b57 }, /* seagreen */
            { 0x3507fb41, 0xfffff5ee }, /* seashell */
            { 0xca348772, 0xffa0522d }, /* sienna */
            { 0xca37d30d, 0xffc0c0c0 }, /* silver */
            { 0x80da74fb, 0xff87ceeb }, /* skyblue */
            { 0x44a8dd73, 0xff6a5acd }, /* slateblue */
            { 0x44ab37f8, 0xff708090 }, /* slategrey */
            { 0x0035f183, 0xfffffafa }, /* snow */
            { 0xd5440d16, 0xff00ff7f }, /* springgreen */
            { 0x3e1524a5, 0xff4682b4 }, /* steelblue */
            { 0x0001bfa1, 0xffd2b48c }, /* tan */
            { 0x0036425c, 0xff008080 }, /* teal */
            { 0xafc8858f, 0xffd8bfd8 }, /* thistle */
            { 0xcc41600a, 0xffff6347 }, /* tomato */
            { 0xfeea9b21, 0xff40e0d0 }, /* turquoise */
            { 0xcf57947f, 0xffee82ee }, /* violet */
            { 0x06bdbae7, 0xfff5deb3 }, /* wheat */
            { 0x10802ee6, 0xfff5f5f5 }, /* whitesmoke */
            { 0xe1b5130f, 0xff9acd32 }, /* yellowgreen */
        };

        const auto hash = (uint32) colourName.trim().toLowerCase().hashCode();

        for (auto entry : presets)
            if (entry.stringHash == hash)
                return Colour (entry.colour);

        return defaultColour;
        */
    }
}
