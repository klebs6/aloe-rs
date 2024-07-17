crate::ix!();

pub fn create_text_editors<'a>() -> Vec<*mut PropertyComponent<'a>> {
    
    todo!();
    /*
        return { new TextPropertyComponent (Value (var ("This is a single-line Text Property")), "Text 1", 200, false),
                 new TextPropertyComponent (Value (var ("Another one")), "Text 2", 200, false),
                 new TextPropertyComponent (Value (var ( "Lorem ipsum dolor sit amet, cu mei labore admodum facilisi. Iriure iuvaret invenire ea vim, cum quod"
                                                         "si intellegat delicatissimi an. Cetero recteque ei eos, his an scripta fastidii placerat. Nec et anc"
                                                         "illae nominati corrumpit. Vis dictas audire accumsan ad, elit fabulas saperet mel eu.\n"
                                                         "\n"
                                                         "Dicam utroque ius ne, eum choro phaedrum eu. Ut mel omnes virtute appareat, semper quodsi labitur in"
                                                         " cum. Est aeque eripuit deleniti in, amet ferri recusabo ea nec. Cu persius maiorum corrumpit mei, i"
                                                         "n ridens perpetua mea, pri nobis tation inermis an. Vis alii autem cotidieque ut, ius harum salutatu"
                                                         "s ut. Mel eu purto veniam dissentias, malis doctus bonorum ne vel, mundi aperiam adversarium cu eum."
                                                         " Mei quando graeci te, dolore accusata mei te.")),
                                            "Multi-line text",
                                            1000, true) };
    */
}

pub fn create_sliders<'a>(how_many: i32) -> Vec<*mut PropertyComponent<'a>> {
    
    todo!();
    /*
        Vec<PropertyComponent*> comps;

        for (int i = 0; i < howMany; ++i)
            comps.add (new DemoSliderPropertyComponent ("Slider " + String (i + 1)));

        return comps;
    */
}

pub fn create_buttons<'a>(how_many: i32) -> Vec<*mut PropertyComponent<'a>> {
    
    todo!();
    /*
        Vec<PropertyComponent*> comps;

        for (int i = 0; i < howMany; ++i)
            comps.add (new DemoButtonPropertyComponent ("Button " + String (i + 1)));

        for (int i = 0; i < howMany; ++i)
            comps.add (new BooleanPropertyComponent (Value (Random::getSystemRandom().nextBool()), "Toggle " + String (i + 1), "Description of toggleable thing"));

        return comps;
    */
}

pub fn create_choices<'a>(how_many: i32) -> Vec<*mut PropertyComponent<'a>> {
    
    todo!();
    /*
        Vec<PropertyComponent*> comps;

        StringArray choices;
        Vec<var> choiceVars;

        for (int i = 0; i < 12; ++i)
        {
            choices.add ("Item " + String (i));
            choiceVars.add (i);
        }

        for (int i = 0; i < howMany; ++i)
            comps.add (new ChoicePropertyComponent (Value (Random::getSystemRandom().nextInt (12)), "Choice Property " + String (i + 1), choices, choiceVars));

        for (int i = 0; i < howMany; ++i)
            comps.add (new MultiChoicePropertyComponent (Value (Vec<var>()), "Multi-Choice Property " + String (i + 1), choices, choiceVars));

        return comps;
    */
}
