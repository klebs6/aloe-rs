crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_data_structures/values/aloe_ValueTreeSynchroniser.h]

pub trait ValueTreeSynchronizerInterface {

    /**
      | This callback happens when the ValueTree
      | changes and the given state-change
      | message needs to be applied to any other
      | trees that need to stay in sync with it.
      | 
      | The data is an opaque blob of binary that
      | you should transmit to wherever your
      | target tree lives, and use applyChange()
      | to apply this to the target tree.
      |
      */
    fn state_changed(&mut self, 
        encoded_change:      *const c_void,
        encoded_change_size: usize);

}

pub enum ValueTreeSynchronizerChangeType
{
    propertyChanged  = 1,
    fullSync         = 2,
    childAdded       = 3,
    childRemoved     = 4,
    childMoved       = 5,
    propertyRemoved  = 6
}

/**
  | This class can be used to watch for all
  | changes to the state of a ValueTree,
  | and to convert them to a transmittable
  | binary encoding.
  | 
  | The purpose of this class is to allow
  | two or more ValueTrees to be remotely
  | synchronised by transmitting encoded
  | changes over some kind of transport
  | mechanism.
  | 
  | To use it, you'll need to implement a
  | subclass of ValueTreeSynchroniser
  | and implement the stateChanged() method
  | to transmit the encoded change (maybe
  | via a network or other means) to a remote
  | destination, where it can be applied
  | to a target tree.
  | 
  | @tags{DataStructures}
  |
  */
#[no_copy]
#[leak_detector]
pub struct ValueTreeSynchroniser {
    value_tree: ValueTree,
}

impl ValueTreeListener for ValueTreeSynchroniser {

    fn value_tree_property_changed(
        &mut self, 
        vt:       &mut ValueTree,
        property: &Identifier

    )  {
        
        todo!();
        /*
            MemoryOutputStream m;

        if (auto* value = vt.getPropertyPointer (property))
        {
            ValueTreeSynchroniserHelpers::writeHeader (*this, m, ValueTreeSynchroniserHelpers::propertyChanged, vt);
            m.writeString (property.toString());
            value->writeToStream (m);
        }
        else
        {
            ValueTreeSynchroniserHelpers::writeHeader (*this, m, ValueTreeSynchroniserHelpers::propertyRemoved, vt);
            m.writeString (property.toString());
        }

        stateChanged (m.getData(), m.getDataSize());
        */
    }

    fn value_tree_child_added(
        &mut self, 
        parent_tree: &mut ValueTree,
        child_tree:  &mut ValueTree

    ) {
        
        todo!();
        /*
            const int index = parentTree.indexOf (childTree);
        jassert (index >= 0);

        MemoryOutputStream m;
        ValueTreeSynchroniserHelpers::writeHeader (*this, m, ValueTreeSynchroniserHelpers::childAdded, parentTree);
        m.writeCompressedInt (index);
        childTree.writeToStream (m);
        stateChanged (m.getData(), m.getDataSize());
        */
    }

    fn value_tree_child_removed(
        &mut self, 
        parent_tree: &mut ValueTree,
        _1:          &mut ValueTree,
        old_index:   i32

    ) {
        
        todo!();
        /*
            MemoryOutputStream m;
        ValueTreeSynchroniserHelpers::writeHeader (*this, m, ValueTreeSynchroniserHelpers::childRemoved, parentTree);
        m.writeCompressedInt (oldIndex);
        stateChanged (m.getData(), m.getDataSize());
        */
    }

    fn value_tree_child_order_changed(
        &mut self, 
        parent:    &mut ValueTree,
        old_index: i32,
        new_index: i32

    ) {
        
        todo!();
        /*
            MemoryOutputStream m;
        ValueTreeSynchroniserHelpers::writeHeader (*this, m, ValueTreeSynchroniserHelpers::childMoved, parent);
        m.writeCompressedInt (oldIndex);
        m.writeCompressedInt (newIndex);
        stateChanged (m.getData(), m.getDataSize());
        */
    }
}

impl Drop for ValueTreeSynchroniser {

    fn drop(&mut self) {
        todo!();
        /* 
        valueTree.removeListener (this);
 */
    }
}

pub mod value_tree_synchroniser {

    use super::*;

    pub fn get_value_tree_path(
        v:              ValueTree,
        top_level_tree: &ValueTree,
        path:           &mut Vec<i32>

    ) {
        
        todo!();
        /*
            while (v != topLevelTree)
                {
                    ValueTree parent (v.getParent());

                    if (! parent.isValid())
                        break;

                    path.add (parent.indexOf (v));
                    v = parent;
                }
        */
    }

    pub fn write_header(
        stream: &mut MemoryOutputStream,
        ty:     ValueTreeSynchronizerChangeType

    ) {
        
        todo!();
        /*
            stream.writeByte ((char) type);
        */
    }

    pub fn write_header_with_value_tree(
        target: &mut ValueTreeSynchroniser,
        stream: &mut MemoryOutputStream,
        ty:     ValueTreeSynchronizerChangeType,
        v:      ValueTree

    ) {

        todo!();
        /*
            writeHeader (stream, type);

                Vec<int> path;
                getValueTreePath (v, target.getRoot(), path);

                stream.writeCompressedInt (path.size());

                for (int i = path.size(); --i >= 0;)
                    stream.writeCompressedInt (path.getUnchecked(i));
        */
    }

    pub fn read_sub_tree_location(
        input: &mut MemoryInputStream,
        v:     ValueTree

    ) -> ValueTree {
        
        todo!();
        /*
            const int numLevels = input.readCompressedInt();

                if (! isPositiveAndBelow (numLevels, 65536)) // sanity-check
                    return {};

                for (int i = numLevels; --i >= 0;)
                {
                    const int index = input.readCompressedInt();

                    if (! isPositiveAndBelow (index, v.getNumChildren()))
                        return {};

                    v = v.getChild (index);
                }

                return v;
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_data_structures/values/aloe_ValueTreeSynchroniser.cpp]
impl ValueTreeSynchroniser {

    /**
      | Returns the root ValueTree that is being
      | observed.
      |
      */
    pub fn get_root(&mut self) -> &ValueTree {
        
        todo!();
        /*
            return valueTree;
        */
    }
    
    /**
      | Creates a ValueTreeSynchroniser that
      | watches the given tree.
      | 
      | After creating an instance of this class
      | and somehow attaching it to a target
      | tree, you probably want to call sendFullSyncCallback()
      | to get them into a common starting state.
      |
      */
    pub fn new(tree: &ValueTree) -> Self {
    
        todo!();
        /*
        : value_tree(tree),

            valueTree.addListener (this);
        */
    }

    /**
      | Forces the sending of a full state message,
      | which may be large, as it encodes the
      | entire ValueTree.
      | 
      | This will internally invoke stateChanged()
      | with the encoded version of the state.
      |
      */
    pub fn send_full_sync_callback(&mut self)  {
        
        todo!();
        /*
            MemoryOutputStream m;
        writeHeader (m, ValueTreeSynchroniserHelpers::fullSync);
        valueTree.writeToStream (m);
        stateChanged (m.getData(), m.getDataSize());
        */
    }

    /**
      | Applies an encoded change to the given
      | destination tree.
      | 
      | When you implement a receiver for changes
      | that were sent by the stateChanged()
      | message, this is the function that you'll
      | need to call to apply them to the target
      | tree that you want to be synced.
      |
      */
    pub fn apply_change(&mut self, 
        root:         &mut ValueTree,
        data:         *const c_void,
        data_size:    usize,
        undo_manager: *mut UndoManager) -> bool {
        
        todo!();
        /*
            MemoryInputStream input (data, dataSize, false);

        const ValueTreeSynchroniserHelpers::ValueTreeSynchronizerChangeType type = (ValueTreeSynchroniserHelpers::ValueTreeSynchronizerChangeType) input.readByte();

        if (type == ValueTreeSynchroniserHelpers::fullSync)
        {
            root = ValueTree::readFromStream (input);
            return true;
        }

        ValueTree v (ValueTreeSynchroniserHelpers::readSubTreeLocation (input, root));

        if (! v.isValid())
            return false;

        switch (type)
        {
            case ValueTreeSynchroniserHelpers::propertyChanged:
            {
                Identifier property (input.readString());
                v.setProperty (property, var::readFromStream (input), undoManager);
                return true;
            }

            case ValueTreeSynchroniserHelpers::propertyRemoved:
            {
                Identifier property (input.readString());
                v.removeProperty (property, undoManager);
                return true;
            }

            case ValueTreeSynchroniserHelpers::childAdded:
            {
                const int index = input.readCompressedInt();
                v.addChild (ValueTree::readFromStream (input), index, undoManager);
                return true;
            }

            case ValueTreeSynchroniserHelpers::childRemoved:
            {
                const int index = input.readCompressedInt();

                if (isPositiveAndBelow (index, v.getNumChildren()))
                {
                    v.removeChild (index, undoManager);
                    return true;
                }

                jassertfalse; // Either received some corrupt data, or the trees have drifted out of sync
                break;
            }

            case ValueTreeSynchroniserHelpers::childMoved:
            {
                const int oldIndex = input.readCompressedInt();
                const int newIndex = input.readCompressedInt();

                if (isPositiveAndBelow (oldIndex, v.getNumChildren())
                     && isPositiveAndBelow (newIndex, v.getNumChildren()))
                {
                    v.moveChild (oldIndex, newIndex, undoManager);
                    return true;
                }

                jassertfalse; // Either received some corrupt data, or the trees have drifted out of sync
                break;
            }

            case ValueTreeSynchroniserHelpers::fullSync:
                break;

            default:
                jassertfalse; // Seem to have received some corrupt data?
                break;
        }

        return false;
        */
    }
}
