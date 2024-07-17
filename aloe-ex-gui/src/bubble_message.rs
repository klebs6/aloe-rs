crate::ix!();

pub fn show_bubble_message(
    target_component:                    &mut Component,
    text_to_show:                        &String,
    bmc:                                 &mut Box<BubbleMessageComponent>,
    is_running_component_transform_demo: bool
) {

    todo!();
    /*
        bmc.reset (new BubbleMessageComponent());

        if (isRunningComponentTransformDemo)
        {
            targetComponent.findParentComponentOfClass<WidgetsDemo>()->addChildComponent (bmc.get());
        }
        else if (Desktop::canUseSemiTransparentWindows())
        {
            bmc->setAlwaysOnTop (true);
            bmc->addToDesktop (0);
        }
        else
        {
            targetComponent.getTopLevelComponent()->addChildComponent (bmc.get());
        }

        AttributedString text (textToShow);
        text.setJustification (Justification::centred);
        text.setColour (targetComponent.findColour (TextButton::textColourOffId));

        bmc->showAt (&targetComponent, text, 2000, true, false);
    */
}
