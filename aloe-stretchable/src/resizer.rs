crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/layout/aloe_StretchableObjectResizer.h]
pub struct StretchableObjectResizerItem
{
    size:     f64,
    min_size: f64,
    max_size: f64,
    order:    i32,
}

/**
  | A utility class for fitting a set of objects
  | whose sizes can vary between a minimum
  | and maximum size, into a space.
  | 
  | This is a trickier algorithm than it
  | would first seem, so I've put it in this
  | class to allow it to be shared by various
  | bits of code.
  | 
  | To use it, create one of these objects,
  | call addItem() to add the list of items
  | you need, then call resizeToFit(),
  | which will change all their sizes. You
  | can then retrieve the new sizes with
  | getItemSize() and getNumItems().
  | 
  | It's currently used by the TableHeaderComponent
  | for stretching out the table headings
  | to fill the table's width.
  | 
  | @tags{GUI}
  |
  */
#[no_copy]
#[leak_detector]
pub struct StretchableObjectResizer {
    items: Vec<StretchableObjectResizerItem>,
}

impl Default for StretchableObjectResizer {
    
    /**
      | Creates an empty object resizer.
      |
      */
    fn default() -> Self {

        todo!();

        /*
        
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/layout/aloe_StretchableObjectResizer.cpp]
impl StretchableObjectResizer {
    
    /**
      | Returns the number of items that have
      | been added.
      |
      */
    pub fn get_num_items(&self) -> i32 {
        
        todo!();
        /*
            return items.size();
        */
    }

    /**
      | Adds an item to the list.
      | 
      | The order parameter lets you specify
      | groups of items that are resized first
      | when some space needs to be found. Those
      | items with an order of 0 will be the first
      | ones to be resized, and if that doesn't
      | provide enough space to meet the requirements,
      | the algorithm will then try resizing
      | the items with an order of 1, then 2, and
      | so on.
      |
      */
    pub fn add_item(
        &mut self, 
        size:     f64,
        min_size: f64,
        max_size: f64,
        order:    Option<i32>

    ) {

        let order: i32 = order.unwrap_or(0);
        
        todo!();
        /*
            // the order must be >= 0 but less than the maximum integer value.
        jassert (order >= 0 && order < std::numeric_limits<int>::max());
        jassert (maxSize >= minSize);

        StretchableObjectResizerItem item;
        item.size = size;
        item.minSize = minSize;
        item.maxSize = maxSize;
        item.order = order;
        items.add (item);
        */
    }
    
    /**
      | Returns the size of one of the items.
      |
      */
    pub fn get_item_size(&self, index: i32) -> f64 {
        
        todo!();
        /*
            return isPositiveAndBelow (index, items.size()) ? items.getReference (index).size
                                                        : 0.0;
        */
    }
    
    /**
      | Resizes all the items to fit this amount
      | of space.
      | 
      | This will attempt to fit them in without
      | exceeding each item's minimum and maximum
      | sizes. In cases where none of the items
      | can be expanded or enlarged any further,
      | the final size may be greater or less
      | than the size passed in.
      | 
      | After calling this method, you can retrieve
      | the new sizes with the getItemSize()
      | method.
      |
      */
    pub fn resize_to_fit(&mut self, target_size: f64)  {
        
        todo!();
        /*
            int order = 0;

        for (;;)
        {
            double currentSize = 0;
            double minSize = 0;
            double maxSize = 0;

            int nextHighestOrder = std::numeric_limits<int>::max();

            for (int i = 0; i < items.size(); ++i)
            {
                const StretchableObjectResizerItem& it = items.getReference(i);
                currentSize += it.size;

                if (it.order <= order)
                {
                    minSize += it.minSize;
                    maxSize += it.maxSize;
                }
                else
                {
                    minSize += it.size;
                    maxSize += it.size;
                    nextHighestOrder = jmin (nextHighestOrder, it.order);
                }
            }

            const double thisIterationTarget = jlimit (minSize, maxSize, targetSize);

            if (thisIterationTarget >= currentSize)
            {
                const double availableExtraSpace = maxSize - currentSize;
                const double targetAmountOfExtraSpace = thisIterationTarget - currentSize;
                const double scale = availableExtraSpace > 0 ? targetAmountOfExtraSpace / availableExtraSpace : 1.0;

                for (int i = 0; i < items.size(); ++i)
                {
                    StretchableObjectResizerItem& it = items.getReference(i);

                    if (it.order <= order)
                        it.size = jlimit (it.minSize, it.maxSize, it.size + (it.maxSize - it.size) * scale);
                }
            }
            else
            {
                const double amountOfSlack = currentSize - minSize;
                const double targetAmountOfSlack = thisIterationTarget - minSize;
                const double scale = targetAmountOfSlack / amountOfSlack;

                for (int i = 0; i < items.size(); ++i)
                {
                    StretchableObjectResizerItem& it = items.getReference(i);

                    if (it.order <= order)
                        it.size = jmax (it.minSize, it.minSize + (it.size - it.minSize) * scale);
                }
            }

            if (nextHighestOrder < std::numeric_limits<int>::max())
                order = nextHighestOrder;
            else
                break;
        }
        */
    }
}
