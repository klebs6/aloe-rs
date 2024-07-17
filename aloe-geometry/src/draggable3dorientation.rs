crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_opengl/geometry/aloe_Draggable3DOrientation.h]

/**
  | Stores a 3D orientation, which can be
  | rotated by dragging with the mouse.
  | 
  | @tags{OpenGL}
  |
  */
pub struct Draggable3DOrientation<QuaternionType = Quaternion<f32>> {
    area:       Rectangle<i32>,
    radius:     f32,
    quaternion: QuaternionType,
    last_mouse: Point<f32>,
}

pub mod draggable_3d_orientation {
    use super::*;
    pub type VectorType     = Vector3D<f32>;
}

impl<QuaternionType> Draggable3DOrientation<QuaternionType> {

    /**
      | Creates a Draggable3DOrientation,
      | initially set up to be aligned along
      | the X axis.
      |
      */
    pub fn new(object_radius: Option<f32>) -> Self {

        let object_radius: f32 = object_radius.unwrap_or(0.5);

        todo!();
        /*


            : radius (jmax (0.1f, objectRadius)),
              quaternion (VectorType::xAxis(), 0)
        */
    }

    /**
      | Creates a Draggable3DOrientation
      | from a user-supplied quaternion.
      |
      */
    pub fn new_with_quaternion(
        quaternion_to_use: &Quaternion<f32>,
        object_radius:     Option<f32>) -> Self {

        let object_radius: f32 = object_radius.unwrap_or(0.5);

        todo!();
        /*


            : radius (jmax (0.1f, objectRadius)),
              quaternion (quaternionToUse)
        */
    }

    /**
      | Resets the orientation, specifying
      | the axis to align it along.
      |
      */
    pub fn reset(&mut self, axis: &draggable_3d_orientation::VectorType)  {
        
        todo!();
        /*
            quaternion = QuaternionType (axis, 0);
        */
    }

    /**
      | Sets the viewport area within which
      | mouse-drag positions will occur.
      | 
      | You'll need to set this rectangle before
      | calling mouseDown. The centre of the
      | rectangle is assumed to be the centre
      | of the object that will be rotated, and
      | the size of the rectangle will be used
      | to scale the object radius - see setRadius().
      |
      */
    pub fn set_viewport(&mut self, new_area: Rectangle<i32>)  {
        
        todo!();
        /*
            area = newArea;
        */
    }

    /**
      | Sets the size of the rotated object,
      | as a proportion of the viewport's size.
      | @see setViewport
      |
      */
    pub fn set_radius(&mut self, new_radius: f32)  {
        
        todo!();
        /*
            radius = jmax (0.1f, newRadius);
        */
    }

    /**
      | Begins a mouse-drag operation.
      | 
      | You must call this before any calls to
      | mouseDrag(). The position that is supplied
      | will be treated as being relative to
      | the centre of the rectangle passed to
      | setViewport().
      |
      */
    pub fn mouse_down<Type>(&mut self, mouse_pos: Point<Type>)  {
    
        todo!();
        /*
            lastMouse = mousePosToProportion (mousePos.toFloat());
        */
    }

    /**
      | Continues a mouse-drag operation.
      | 
      | After calling mouseDown() to begin
      | a drag sequence, you can call this method
      | to continue it.
      |
      */
    pub fn mouse_drag<Type>(&mut self, mouse_pos: Point<Type>)  {
    
        todo!();
        /*
            auto oldPos = projectOnSphere (lastMouse);
            lastMouse = mousePosToProportion (mousePos.toFloat());
            auto newPos = projectOnSphere (lastMouse);

            quaternion *= rotationFromMove (oldPos, newPos);
        */
    }

    /**
      | Returns the matrix that should be used
      | to apply the current orientation. @see
      | applyToOpenGLMatrix
      |
      */
    pub fn get_rotation_matrix(&self) -> Matrix3D<f32> {
        
        todo!();
        /*
            return quaternion.getRotationMatrix();
        */
    }

    /**
      | Provides direct access to the quaternion.
      |
      */
    pub fn get_quaternion<'a>(&'a mut self) -> &'a mut QuaternionType {
        
        todo!();
        /*
            return quaternion;
        */
    }
    
    pub fn mouse_pos_to_proportion(&self, mouse_pos: Point<f32>) -> Point<f32> {
        
        todo!();
        /*
            auto scale = jmin (area.getWidth(), area.getHeight()) / 2;

            // You must call setViewport() to give this object a valid window size before
            // calling any of the mouse input methods!
            jassert (scale > 0);

            return { (mousePos.x - (float) area.getCentreX()) / (float) scale,
                     ((float) area.getCentreY() - mousePos.y) / (float) scale };
        */
    }
    
    pub fn project_on_sphere(&self, pos: Point<f32>) -> draggable_3d_orientation::VectorType {
        
        todo!();
        /*
            auto radiusSquared = radius * radius;
            auto xySquared = pos.x * pos.x + pos.y * pos.y;

            return { pos.x, pos.y,
                     xySquared < radiusSquared * 0.5f ? std::sqrt (radiusSquared - xySquared)
                                                      : (radiusSquared / (2.0f * std::sqrt (xySquared))) };
        */
    }
    
    pub fn rotation_from_move(&self, 
        from: &draggable_3d_orientation::VectorType,
        to:   &draggable_3d_orientation::VectorType) -> QuaternionType {
        
        todo!();
        /*
            auto rotationAxis = (to ^ from);

            if (rotationAxis.lengthIsBelowEpsilon())
                rotationAxis = VectorType::xAxis();

            auto d = jlimit (-1.0f, 1.0f, (from - to).length() / (2.0f * radius));

            return QuaternionType::fromAngle (2.0f * std::asin (d), rotationAxis);
        */
    }
}
