crate::ix!();

pub struct LoopPointsOverlay<'a> {
    base:                      Component<'a>,
    data_model:                DataModel<'a>,
    visible_range:             VisibleRangeDataModel<'a>,
    loop_points_on_mouse_down: Range<f64>,
    begin_marker:              LoopPointMarker<'a>,
    end_marker:                LoopPointMarker<'a>,
    undo_manager:              *mut UndoManager<'a>,
}

impl<'a> VisibleRangeDataModelListener for LoopPointsOverlay<'a> {

}

impl<'a> DataModelListener for LoopPointsOverlay<'a> {

}

impl<'a> LoopPointsOverlay<'a> {

    pub fn new(
        d_model:         &DataModel,
        model:           &VisibleRangeDataModel,
        undo_manager_in: &mut UndoManager) -> Self {
    
        todo!();
        /*


            : dataModel (dModel),
              visibleRange (vModel),
              beginMarker ("B",
                           [this] (LoopPointMarker& m, const MouseEvent& e) { this->loopPointMouseDown (m, e); },
                           [this] (LoopPointMarker& m, const MouseEvent& e) { this->loopPointDragged   (m, e); },
                           [this] (LoopPointMarker& m, const MouseEvent& e) { this->loopPointMouseUp   (m, e); }),
              endMarker   ("E",
                           [this] (LoopPointMarker& m, const MouseEvent& e) { this->loopPointMouseDown (m, e); },
                           [this] (LoopPointMarker& m, const MouseEvent& e) { this->loopPointDragged   (m, e); },
                           [this] (LoopPointMarker& m, const MouseEvent& e) { this->loopPointMouseUp   (m, e); }),
              undoManager (&undoManagerIn)
            dataModel   .addListener (*this);
            visibleRange.addListener (*this);

            for (auto ptr : { &beginMarker, &endMarker })
                addAndMakeVisible (ptr);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            positionLoopPointMarkers();
        */
    }
    
    pub fn loop_point_mouse_down(&mut self, 
        _0: &mut LoopPointMarker,
        _1: &MouseEvent)  {
        
        todo!();
        /*
            loopPointsOnMouseDown = dataModel.getLoopPointsSeconds();
            undoManager->beginNewTransaction();
        */
    }
    
    pub fn loop_point_dragged(&mut self, 
        marker: &mut LoopPointMarker,
        e:      &MouseEvent)  {
        
        todo!();
        /*
            auto x = xPositionToTime (e.getEventRelativeTo (this).position.x);
            const Range<double> newLoopRange (&marker == &beginMarker ? x : loopPointsOnMouseDown.getStart(),
                                              &marker == &endMarker   ? x : loopPointsOnMouseDown.getEnd());

            dataModel.setLoopPointsSeconds (newLoopRange, undoManager);
        */
    }
    
    pub fn loop_point_mouse_up(&mut self, 
        marker: &mut LoopPointMarker,
        e:      &MouseEvent)  {
        
        todo!();
        /*
            auto x = xPositionToTime (e.getEventRelativeTo (this).position.x);
            const Range<double> newLoopRange (&marker == &beginMarker ? x : loopPointsOnMouseDown.getStart(),
                                              &marker == &endMarker   ? x : loopPointsOnMouseDown.getEnd());

            dataModel.setLoopPointsSeconds (newLoopRange, undoManager);
        */
    }
    
    pub fn loop_points_seconds_changed(&mut self, _0: Range<f64>)  {
        
        todo!();
        /*
            positionLoopPointMarkers();
        */
    }
    
    pub fn visible_range_changed(&mut self, _0: Range<f64>)  {
        
        todo!();
        /*
            positionLoopPointMarkers();
        */
    }
    
    pub fn time_to_xposition(&self, time: f64) -> f64 {
        
        todo!();
        /*
            return (time - visibleRange.getVisibleRange().getStart()) * getWidth()
                         / visibleRange.getVisibleRange().getLength();
        */
    }
    
    pub fn x_position_to_time(&self, x_position: f64) -> f64 {
        
        todo!();
        /*
            return ((xPosition * visibleRange.getVisibleRange().getLength()) / getWidth())
                               + visibleRange.getVisibleRange().getStart();
        */
    }
    
    pub fn position_loop_point_markers(&mut self)  {
        
        todo!();
        /*
            auto halfMarkerWidth = 7;

            for (auto tup : { std::make_tuple (&beginMarker, dataModel.getLoopPointsSeconds().getStart()),
                              std::make_tuple (&endMarker,   dataModel.getLoopPointsSeconds().getEnd()) })
            {
                auto ptr  = std::get<0> (tup);
                auto time = std::get<1> (tup);
                ptr->setSize (halfMarkerWidth * 2, getHeight());
                ptr->setTopLeftPosition (roundToInt (timeToXPosition (time) - halfMarkerWidth), 0);
            }
        */
    }
}
