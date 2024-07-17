crate::ix!();

#[no_copy]
#[leak_detector]
pub struct TestListComponent<'a> {
    base:        Component<'a>,
    base2:       ListBoxModel,
    demo_holder: &'a mut DemoHolderComponent<'a>,
    list_box:    ListBox<'a>,
    demos:       Vec<Box<GraphicsDemoBase<'a>>>,
}

impl<'a> TestListComponent<'a> {

    pub fn new(
        holder:   &mut DemoHolderComponent,
        controls: &mut ControllersComponent) -> Self {
    
        todo!();
        /*
        : demo_holder(holder),

            demos.add (new PathsDemo (controls, false, true));
            demos.add (new PathsDemo (controls, true,  false));
            demos.add (new PathsDemo (controls, false, false));
            demos.add (new RectangleFillTypesDemo (controls));
            demos.add (new StrokesDemo (controls));
            demos.add (new ImagesRenderingDemo (controls, false, false));
            demos.add (new ImagesRenderingDemo (controls, false, true));
            demos.add (new ImagesRenderingDemo (controls, true,  false));
            demos.add (new ImagesRenderingDemo (controls, true,  true));
            demos.add (new GlyphsDemo (controls));
            demos.add (new SVGDemo    (controls));
            demos.add (new LinesDemo  (controls));

            addAndMakeVisible (listBox);
            listBox.setTitle ("Test List");
            listBox.setModel (this);
            listBox.selectRow (0);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            listBox.setBounds (getLocalBounds());
        */
    }
    
    pub fn get_num_rows(&mut self) -> i32 {
        
        todo!();
        /*
            return demos.size();
        */
    }
    
    pub fn paint_list_box_item(&mut self, 
        row_number:      i32,
        g:               &mut Graphics,
        width:           i32,
        height:          i32,
        row_is_selected: bool)  {
        
        todo!();
        /*
            if (demos[rowNumber] == nullptr)
                return;

            if (rowIsSelected)
                g.fillAll (Colour::contrasting (findColour (ListBox::textColourId),
                                                findColour (ListBox::backgroundColourId)));

            g.setColour (findColour (ListBox::textColourId));
            g.setFont (14.0f);
            g.drawFittedText (getNameForRow (rowNumber), 8, 0, width - 10, height, Justification::centredLeft, 2);
        */
    }
    
    pub fn get_name_for_row(&mut self, row_number: i32) -> String {
        
        todo!();
        /*
            if (auto* demo = demos[rowNumber])
                return demo->getName();

            return {};
        */
    }
    
    pub fn selected_rows_changed(&mut self, last_row_selected: i32)  {
        
        todo!();
        /*
            demoHolder.setDemo (demos [lastRowSelected]);
        */
    }
}
