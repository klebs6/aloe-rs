crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/GUI/GridDemo.h]

pub struct GridDemo<'a> {
    base:  Component<'a>,
    items: Vec<Box<GridItemPanel<'a>>>,
}

impl<'a> Default for GridDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            addGridItemPanel (Colours::aquamarine, "0");
            addGridItemPanel (Colours::red,        "1");
            addGridItemPanel (Colours::blue,       "2");
            addGridItemPanel (Colours::green,      "3");
            addGridItemPanel (Colours::orange,     "4");
            addGridItemPanel (Colours::white,      "5");
            addGridItemPanel (Colours::aquamarine, "6");
            addGridItemPanel (Colours::red,        "7");
            addGridItemPanel (Colours::blue,       "8");
            addGridItemPanel (Colours::green,      "9");
            addGridItemPanel (Colours::orange,     "10");
            addGridItemPanel (Colours::white,      "11");

            setSize (750, 750)
        */
    }
}

impl<'a> Paint for GridDemo<'a> {

    fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (Colours::black);
        */
    }
}

impl<'a> Resized for GridDemo<'a> {

    fn resized(&mut self)  {
        
        todo!();
        /*
            Grid grid;

            grid.rowGap    = 20_px;
            grid.columnGap = 20_px;

            using Track = Grid::TrackInfo;

            grid.templateRows = { Track (1_fr), Track (1_fr), Track (1_fr) };

            grid.templateColumns = { Track (1_fr),
                                     Track (1_fr),
                                     Track (1_fr) };

            grid.autoColumns = Track (1_fr);
            grid.autoRows    = Track (1_fr);

            grid.autoFlow = Grid::AutoFlow::column;

            grid.items.addArray ({ GridItem (items[0]).withArea (2, 2, 4, 4),
                                   GridItem (items[1]),
                                   GridItem (items[2]).withArea ({}, 3),
                                   GridItem (items[3]),
                                   GridItem (items[4]).withArea (GridItem::Span (2), {}),
                                   GridItem (items[5]),
                                   GridItem (items[6]),
                                   GridItem (items[7]),
                                   GridItem (items[8]),
                                   GridItem (items[9]),
                                   GridItem (items[10]),
                                   GridItem (items[11])
                                });

            grid.performLayout (getLocalBounds());
        */
    }
}

impl<'a> GridDemo<'a> {

    pub fn add_grid_item_panel(&mut self, 
        colour: Colour,
        text:   *const u8)  {
        
        todo!();
        /*
            addAndMakeVisible (items.add (new GridItemPanel (colour, text)));
        */
    }
}
