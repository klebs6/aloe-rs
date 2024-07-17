crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/GUI/FlexBoxDemo.h]

#[no_copy]
#[leak_detector]
pub struct DemoFlexPanel<'a> {
    base: Component<'a>,
    flex_item:          &'a mut FlexItem<'a>,
    flex_order_editor:  TextEditor<'a>,
    flex_grow_editor:   TextEditor<'a>,
    flex_shrink_editor: TextEditor<'a>,
    flex_basis_editor:  TextEditor<'a>,
    align_self_combo:   ComboBox<'a>,
    colour:             Colour,
    labels:             Vec<Box<Label<'a>>>,
}

impl<'a> DemoFlexPanel<'a> {

    pub fn new(
        col:  Colour,
        item: &mut FlexItem) -> Self {
    
        todo!();
        /*
        : flex_item(item),
        : colour(col),

            int x = 70;
            int y = 3;

            setupTextEditor (flexOrderEditor, { x, y, 20, 18 }, "0", [this] { flexItem.order = (int) flexOrderEditor.getText().getFloatValue(); });
            addLabel ("order", flexOrderEditor);
            y += 20;

            setupTextEditor (flexGrowEditor, { x, y, 20, 18 }, "0", [this] { flexItem.flexGrow = flexGrowEditor.getText().getFloatValue(); });
            addLabel ("flex-grow", flexGrowEditor);
            y += 20;

            setupTextEditor (flexShrinkEditor, { x, y, 20, 18 }, "1", [this] { flexItem.flexShrink = flexShrinkEditor.getText().getFloatValue(); });
            addLabel ("flex-shrink", flexShrinkEditor);
            y += 20;

            setupTextEditor (flexBasisEditor, { x, y, 33, 18 }, "100", [this] { flexItem.flexBasis = flexBasisEditor.getText().getFloatValue(); });
            addLabel ("flex-basis", flexBasisEditor);
            y += 20;

            alignSelfCombo.addItem ("auto",       1);
            alignSelfCombo.addItem ("flex-start", 2);
            alignSelfCombo.addItem ("flex-end",   3);
            alignSelfCombo.addItem ("center",     4);
            alignSelfCombo.addItem ("stretch",    5);

            alignSelfCombo.setBounds (x, y, 90, 18);
            alignSelfCombo.onChange = [this] { updateAssignSelf(); };
            alignSelfCombo.setSelectedId (5);
            alignSelfCombo.setColour (ComboBox::outlineColourId, Colours::transparentBlack);
            addAndMakeVisible (alignSelfCombo);
            addLabel ("align-self", alignSelfCombo);
        */
    }
    
    pub fn setup_text_editor(&mut self, 
        te:           &mut TextEditor,
        b:            Rectangle<i32>,
        initial_text: &str,
        update_fn:    fn() -> ())  {
        
        todo!();
        /*
            te.setBounds (b);
            te.setText (initialText);

            te.onTextChange = [this, updateFn]
            {
                updateFn();
                refreshLayout();
            };

            addAndMakeVisible (te);
        */
    }
    
    pub fn add_label(&mut self, 
        name:   &String,
        target: &mut Component)  {
        
        todo!();
        /*
            auto label = new Label (name, name);
            label->attachToComponent (&target, true);
            labels.add (label);
            addAndMakeVisible (label);
        */
    }
    
    pub fn update_assign_self(&mut self)  {
        
        todo!();
        /*
            switch (alignSelfCombo.getSelectedId())
            {
                case 1:  flexItem.alignSelf = FlexItem::AlignSelf::autoAlign; break;
                case 2:  flexItem.alignSelf = FlexItem::AlignSelf::flexStart; break;
                case 3:  flexItem.alignSelf = FlexItem::AlignSelf::flexEnd;   break;
                case 4:  flexItem.alignSelf = FlexItem::AlignSelf::center;    break;
                case 5:  flexItem.alignSelf = FlexItem::AlignSelf::stretch;   break;
                default: break;
            }

            refreshLayout();
        */
    }
    
    pub fn refresh_layout(&mut self)  {
        
        todo!();
        /*
            if (auto parent = getParentComponent())
                parent->resized();
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            auto r = getLocalBounds();

            g.setColour (colour);
            g.fillRect (r);

            g.setColour (Colours::black);
            g.drawFittedText ("w: " + String (r.getWidth()) + newLine + "h: " + String (r.getHeight()),
                              r.reduced (4), Justification::bottomRight, 2);
        */
    }
    
    pub fn look_and_feel_changed(&mut self)  {
        
        todo!();
        /*
            flexOrderEditor .applyFontToAllText (flexOrderEditor .getFont());
            flexGrowEditor  .applyFontToAllText (flexGrowEditor  .getFont());
            flexShrinkEditor.applyFontToAllText (flexShrinkEditor.getFont());
            flexBasisEditor .applyFontToAllText (flexBasisEditor .getFont());
        */
    }
}
