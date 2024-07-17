crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/pluginterfaces/vst/ivstrepresentation.h]

pub const REPRESENTATION_INFO_NAME_SIZE: usize = 64;

/**
  | RepresentationInfo is the structure
  | describing a representation
  | 
  | This structure is used in the function
  | \see IXmlRepresentationController::getXmlRepresentationStream.
  | \see IXmlRepresentationController
  |
  */
pub struct RepresentationInfo {

    /**
      | Vendor name of the associated representation
      | (remote) (eg. "Yamaha").
      |
      */
    vendor:  [u8; REPRESENTATION_INFO_NAME_SIZE],

    /**
      | Representation (remote) Name (eg.
      | "O2").
      |
      */
    name:    [u8; REPRESENTATION_INFO_NAME_SIZE],

    /**
      | Version of this "Remote" (eg. "1.0").
      |
      */
    version: [u8; REPRESENTATION_INFO_NAME_SIZE],

    /**
      | Optional: used if the representation
      | is for a given host only (eg. "Nuendo").
      |
      */
    host:    [u8; REPRESENTATION_INFO_NAME_SIZE],
}

impl Default for RepresentationInfo {
    
    fn default() -> Self {
        todo!();
        /*


            memset (vendor, 0, REPRESENTATION_INFO_NAME_SIZE);
            memset (name, 0, REPRESENTATION_INFO_NAME_SIZE);
            memset (version, 0, REPRESENTATION_INFO_NAME_SIZE);
            memset (host, 0, REPRESENTATION_INFO_NAME_SIZE);
        */
    }
}

impl RepresentationInfo {

    pub fn new(
        vendor:  *mut u8,
        name:    *mut u8,
        version: *mut u8,
        host:    *mut u8

    ) -> Self {

        todo!();
        /*


            memset (vendor, 0, REPRESENTATION_INFO_NAME_SIZE);
            if (_vendor)
                strcpy (vendor, _vendor);
            memset (name, 0, REPRESENTATION_INFO_NAME_SIZE);
            if (_name)
                strcpy (name, _name);
            memset (version, 0, REPRESENTATION_INFO_NAME_SIZE);
            if (_version)
                strcpy (version, _version);
            memset (host, 0, REPRESENTATION_INFO_NAME_SIZE);
            if (_host)
                strcpy (host, _host);
        */
    }
}

/**
  | Extended plug-in interface IEditController
  | for a component: Vst::IXmlRepresentationController
  | \ingroup vstIPlug vst350
  | 
  | - [plug imp]
  | 
  | - [extends IEditController]
  | 
  | - [released: 3.5.0]
  | 
  | - [optional]
  | 
  | A representation based on XML is a way
  | to export, structure, and group plug-ins
  | parameters for a specific remote (hardware
  | or software rack (such as quick controls)).
  | \n
  | 
  | It allows to describe each parameter
  | more precisely (what is the best matching
  | to a knob, different title lengths matching
  | limited remote display,...).\n See
  | an \ref Example. \n\n
  | 
  | - A representation is composed of pages
  | (this means that to see all exported
  | parameters, the user has to navigate
  | through the pages).
  | 
  | - A page is composed of cells (for example
  | 8 cells per page).
  | 
  | - A cell is composed of layers (for example
  | a cell could have a knob, a display, and
  | a button, which means 3 layers).
  | 
  | - A layer is associated to a plug-in parameter
  | using the ParameterID as identifier:
  | 
  | - it could be a knob with a display for
  | title and/or value, this display uses
  | the same parameterId, but it could an
  | another one.
  | 
  | - switch
  | 
  | - link which allows to jump directly
  | to a subpage (another page)
  | 
  | - more... See Vst::LayerType .
  | 
  | \n
  | 
  | This representation is implemented
  | as XML text following the Document Type
  | Definition (DTD): http://dtd.steinberg.net/Vst-Remote-1.1.dtd
  | 
  | \section Example
  | 
  | Here an example of what should be passed
  | in the stream of getXmlRepresentationStream:
  | 
  | -----------
  | @code
  | 
  | <?xml version="1.0" encoding="utf-8"?>
  | <!DOCTYPE vstXML PUBLIC "-//Steinberg//DTD Vst Remote 1.1//EN" "http://dtd.steinberg.net/Vst-Remote-1.1.dtd">
  | <vstXML version="1.0">
  |     <plugin classID="341FC5898AAA46A7A506BC0799E882AE" name="Chorus" vendor="Steinberg Media Technologies" />
  |     <originator>My name</originator>
  |     <date>2010-12-31</date>
  |     <comment>This is an example for 4 Cells per Page for the Remote named ProductRemote
  |              from company HardwareCompany.</comment>
  | 
  |     <!-- ===================================== -->
  |     <representation name="ProductRemote" vendor="HardwareCompany" version="1.0">
  |         <page name="Root">
  |             <cell>
  |                 <layer type="knob" parameterID="0">
  |                     <titleDisplay>
  |                         <name>Mix dry/wet</name>
  |                         <name>Mix</name>
  |                     </titleDisplay>
  |                 </layer>
  |             </cell>
  |             <cell>
  |                 <layer type="display"></layer>
  |             </cell>
  |             <cell>
  |                 <layer type="knob" parameterID="3">
  |                     <titleDisplay>
  |                         <name>Delay</name>
  |                         <name>Dly</name>
  |                     </titleDisplay>
  |                 </layer>
  |             </cell>
  |             <cell>
  |                 <layer type="knob" parameterID="15">
  |                     <titleDisplay>
  |                         <name>Spatial</name>
  |                         <name>Spat</name>
  |                     </titleDisplay>
  |                 </layer>
  |             </cell>
  |         </page>
  |         <page name="Page 2">
  |             <cell>
  |                 <layer type="LED" ledStyle="spread" parameterID="2">
  |                     <titleDisplay>
  |                         <name>Width +</name>
  |                         <name>Widt</name>
  |                     </titleDisplay>
  |                 </layer>
  |                 <!--this is the switch for shape A/B-->
  |                 <layer type="switch" switchStyle="pushIncLooped" parameterID="4"></layer>
  |             </cell>
  |             <cell>
  |                 <layer type="display"></layer>
  |             </cell>
  |             <cell>
  |                 <layer type="LED" ledStyle="singleDot" parameterID="17">
  |                     <titleDisplay>
  |                         <name>Sync Note +</name>
  |                         <name>Note</name>
  |                     </titleDisplay>
  |                 </layer>
  |                 <!--this is the switch for sync to tempo on /off-->
  |                 <layer type="switch" switchStyle="pushIncLooped" parameterID="16"></layer>
  |             </cell>
  |             <cell>
  |                 <layer type="knob" parameterID="1">
  |                     <titleDisplay>
  |                         <name>Rate</name>
  |                     </titleDisplay>
  |                 </layer>
  |             </cell>
  |         </page>
  |     </representation>
  | </vstXML>
  |
  */
pub trait IXmlRepresentationController: FUnknown {

    /**
      | Retrieves a stream containing a XmlRepresentation
      | for a wanted representation info
      |
      */
    #[PLUGIN_API]
    fn get_xml_representation_stream(
        &mut self, 
        info:   &mut RepresentationInfo,
        stream: *mut dyn IBStream

    ) -> tresult;

}

lazy_static!{
    /*
    static const FUID ixml_representation_controller_iid;
    */
}

declare_class_iid!{
    IXmlRepresentationController, 
    0xA81A0471, 
    0x48C34DC4, 
    0xAC30C9E1, 
    0x3C8393D5
}

/** Defines for XML representation Tags and Attributes */

pub const ROOTXML_TAG:            &'static str = "vstXML";
pub const COMMENT_TAG:            &'static str = "comment";
pub const CELL_TAG:               &'static str = "cell";
pub const CELLGROUP_TAG:          &'static str = "cellGroup";
pub const CELLGROUPTEMPLATE_TAG:  &'static str = "cellGroupTemplate";
pub const CURVE_TAG:              &'static str = "curve";
pub const CURVETEMPLATE_TAG:      &'static str = "curveTemplate";
pub const DATE_TAG:               &'static str = "date";
pub const LAYER_TAG:              &'static str = "layer";
pub const NAME_TAG:               &'static str = "name";
pub const ORIGINATOR_TAG:         &'static str = "originator";
pub const PAGE_TAG:               &'static str = "page";
pub const PAGETEMPLATE_TAG:       &'static str = "pageTemplate";
pub const PLUGIN_TAG:             &'static str = "plugin";
pub const VALUE_TAG:              &'static str = "value";
pub const VALUEDISPLAY_TAG:       &'static str = "valueDisplay";
pub const VALUELIST_TAG:          &'static str = "valueList";
pub const REPRESENTATION_TAG:     &'static str = "representation";
pub const SEGMENT_TAG:            &'static str = "segment";
pub const SEGMENTLIST_TAG:        &'static str = "segmentList";
pub const TITLEDISPLAY_TAG:       &'static str = "titleDisplay";
pub const ATTR_CATEGORY:          &'static str = "category";
pub const ATTR_CLASSID:           &'static str = "classID";
pub const ATTR_ENDPOINT:          &'static str = "endPoint";
pub const ATTR_INDEX:             &'static str = "index";
pub const ATTR_FLAGS:             &'static str = "flags";
pub const ATTR_FUNCTION:          &'static str = "function";
pub const ATTR_HOST:              &'static str = "host";
pub const ATTR_LEDSTYLE:          &'static str = "ledStyle";
pub const ATTR_LENGTH:            &'static str = "length";
pub const ATTR_LINKEDTO:          &'static str = "linkedTo";
pub const ATTR_NAME:              &'static str = "name";
pub const ATTR_ORDER:             &'static str = "order";
pub const ATTR_PAGE:              &'static str = "page";
pub const ATTR_PARAMID:           &'static str = "parameterID";
pub const ATTR_STARTPOINT:        &'static str = "startPoint";
pub const ATTR_STYLE:             &'static str = "style";
pub const ATTR_SWITCHSTYLE:       &'static str = "switchStyle";
pub const ATTR_TEMPLATE:          &'static str = "template";
pub const ATTR_TURNSPERFULLRANGE: &'static str = "turnsPerFullRange";
pub const ATTR_TYPE:              &'static str = "type";
pub const ATTR_UNITID:            &'static str = "unitID";
pub const ATTR_VARIABLES:         &'static str = "variables";
pub const ATTR_VENDOR:            &'static str = "vendor";
pub const ATTR_VERSION:           &'static str = "version";

/**
  | Defines some predefined Representation
  | Remote Names
  |
  */
pub const GENERIC:               &'static str = "Generic";
pub const GENERIC_4_CELLS:       &'static str = "Generic 4 Cells";
pub const GENERIC_8_CELLS:       &'static str = "Generic 8 Cells";
pub const GENERIC_12_CELLS:      &'static str = "Generic 12 Cells";
pub const GENERIC_24_CELLS:      &'static str = "Generic 24 Cells";
pub const GENERIC_N_CELLS:       &'static str = "Generic %d Cells";
pub const QUICK_CONTROL_8_CELLS: &'static str = "Quick Controls 8 Cells";

/**
  | Layer Types used in a Vst XML Representation
  |
  */
pub const LAYER_TYPE_KNOB:              usize = 0; // a knob (encoder or not)
pub const LAYER_TYPE_PRESSED_KNOB:      usize = 1; // a knob which is used by pressing and turning
pub const LAYER_TYPE_SWITCH_KNOB:       usize = 2; // knob could be pressed to simulate a switch
pub const LAYER_TYPE_SWITCH:            usize = 3; // a "on/off" button
pub const LAYER_TYPE_LED:               usize = 4; // LED like VU-meter or display around a knob
pub const LAYER_TYPE_LINK:              usize = 5; // indicates that this layer is a folder linked to an another INode (page)
pub const LAYER_TYPE_DISPLAY:           usize = 6; // only for text display (not really a control)
pub const LAYER_TYPE_FADER:             usize = 7; // a fader
pub const LAYER_TYPE_END_OF_LAYER_TYPE: usize = 8;

/**
  | FIDString variant of the LayerType
  |
  */
pub const LAYER_TYPE_FID_STRING: &[FIDString] = &[
    "knob",
    "pressedKnob",
    "switchKnob",
    "switch",
    "LED",
    "link",
    "display",
    "fader",
];

/**
  | Curve Types used in a Vst XML Representation
  |
  */
pub const CURVE_TYPE_SEGMENT:    &'static str = "segment";
pub const CURVE_TYPE_VALUE_LIST: &'static str = "valueList";

/**
  | Attributes used to defined a Layer in
  | a Vst XML Representation
  |
  */
pub const ATTRIBUTES_STYLE:                     &'static str = ATTR_STYLE;             // string attribute : See AttributesStyle for available string value
pub const ATTRIBUTES_LED_STYLE:                 &'static str = ATTR_LEDSTYLE;          // string attribute : See AttributesStyle for available string value
pub const ATTRIBUTES_SWITCH_STYLE:              &'static str = ATTR_SWITCHSTYLE;       // string attribute : See AttributesStyle for available string value
pub const ATTRIBUTES_KNOB_TURNS_PER_FULL_RANGE: &'static str = ATTR_TURNSPERFULLRANGE; // float attribute
pub const ATTRIBUTES_FUNCTION:                  &'static str = ATTR_FUNCTION;          // string attribute : See AttributesFunction for available string value
pub const ATTRIBUTES_FLAGS:                     &'static str = ATTR_FLAGS;             // string attribute : See AttributesFlags for available string value

/* ----------------- Global Style  ----------------- */

/**
  | Attributes Function used to defined
  | the function of a Layer in a Vst XML Representation
  |
  */
pub const ATTRIBUTES_FUNCTION_PAN_POS_CENTER_XFUNC:      &'static str = "PanPosCenterX";     // Gravity point X-axis (L-R) (for stereo: middle between left and right)
pub const ATTRIBUTES_FUNCTION_PAN_POS_CENTER_YFUNC:      &'static str = "PanPosCenterY";     // Gravity point Y-axis (Front-Rear)
pub const ATTRIBUTES_FUNCTION_PAN_POS_FRONT_LEFT_XFUNC:  &'static str = "PanPosFrontLeftX";  // Left channel Position in X-axis
pub const ATTRIBUTES_FUNCTION_PAN_POS_FRONT_LEFT_YFUNC:  &'static str = "PanPosFrontLeftY";  // Left channel Position in Y-axis
pub const ATTRIBUTES_FUNCTION_PAN_POS_FRONT_RIGHT_XFUNC: &'static str = "PanPosFrontRightX"; // Right channel Position in X-axis
pub const ATTRIBUTES_FUNCTION_PAN_POS_FRONT_RIGHT_YFUNC: &'static str = "PanPosFrontRightY"; // Right channel Position in Y-axis
pub const ATTRIBUTES_FUNCTION_PAN_ROTATION_FUNC:         &'static str = "PanRotation";       // Rotation around the Center (gravity point)
pub const ATTRIBUTES_FUNCTION_PAN_LAW_FUNC:              &'static str = "PanLaw";            // Panning Law
pub const ATTRIBUTES_FUNCTION_PAN_MIRROR_MODE_FUNC:      &'static str = "PanMirrorMode";     // Panning Mirror Mode
pub const ATTRIBUTES_FUNCTION_PAN_LFE_GAIN_FUNC:         &'static str = "PanLfeGain";        // Panning LFE Gain
pub const ATTRIBUTES_FUNCTION_GAIN_REDUCTION_FUNC:       &'static str = "GainReduction";     // Gain Reduction for compressor
pub const ATTRIBUTES_FUNCTION_SOLO_FUNC:                 &'static str = "Solo";              // Solo
pub const ATTRIBUTES_FUNCTION_MUTE_FUNC:                 &'static str = "Mute";              // Mute
pub const ATTRIBUTES_FUNCTION_VOLUME_FUNC:               &'static str = "Volume";            // Volume

/* ----------------- Global Style  ----------------- 
  | Attributes Style associated a specific
  | Layer Type in a Vst XML Representation
  |
  */

/**
   the associated layer should use the inverse
   value of parameter (1 - x).
  */
pub const ATTRIBUTE_STYLE_INVERSE_STYLE: &'static str = "inverse"; 

/* ------------------- LED Style  ------------------- */

pub const ATTRIBUTE_STYLE_LED_WRAP_LEFT_STYLE:  &'static str = "wrapLeft";  // |======>----- (the default one if not specified)
pub const ATTRIBUTE_STYLE_LED_WRAP_RIGHT_STYLE: &'static str = "wrapRight"; // -------<====|
pub const ATTRIBUTE_STYLE_LED_SPREAD_STYLE:     &'static str = "spread";    // ---<==|==>---
pub const ATTRIBUTE_STYLE_LED_BOOST_CUT_STYLE:  &'static str = "boostCut";  // ------|===>--
pub const ATTRIBUTE_STYLE_LED_SINGLE_DOT_STYLE: &'static str = "singleDot"; // --------|----

/* ----------------- Switch Style  ----------------- */

/**
  | Apply only when pressed, unpressed will
  | reset the value to min.
  */
pub const ATTRIBUTE_STYLE_SWITCH_PUSH_STYLE:            &'static str = "push";          

/**
  | Push will increment the value. When the max
  | is reached it will restart with min.  The
  | default one if not specified (with 2 states
  | values it is a OnOff switch).
  */
pub const ATTRIBUTE_STYLE_SWITCH_PUSH_INC_LOOPED_STYLE: &'static str = "pushIncLooped"; 

/**
  | Push will decrement the value. When the min
  | is reached it will restart with max.
  */
pub const ATTRIBUTE_STYLE_SWITCH_PUSH_DEC_LOOPED_STYLE: &'static str = "pushDecLooped"; 

/**
  | Increment after each press (delta depends
  | of the curve).
  */
pub const ATTRIBUTE_STYLE_SWITCH_PUSH_INC_STYLE:        &'static str = "pushInc";       

/**
  | Decrement after each press (delta depends
  | of the curve).
  */
pub const ATTRIBUTE_STYLE_SWITCH_PUSH_DEC_STYLE:        &'static str = "pushDec";       

/**
  | Each push-release will change the value
  | between min and max.  A timeout between
  | push and release could be used to simulate
  | a push style (if timeout is reached).
  */
pub const ATTRIBUTE_STYLE_SWITCH_LATCH_STYLE:           &'static str = "latch";         

/**
  | Attributes Flags defining a Layer in
  | a Vst XML Representation
  |
  | the associated layer marked as hideable
  | allows a remote to hide or make it not
  | usable a parameter when the associated
  | value is inactive
  */
pub const ATTRIBUTES_FLAGS_HIDEABLE_FLAG: &'static str = "hideable"; 
