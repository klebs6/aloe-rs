crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Dynamics/Joints/b2Joint.h]

pub enum b2JointType
{
    e_unknownJoint,
    e_revoluteJoint,
    e_prismaticJoint,
    e_distanceJoint,
    e_pulleyJoint,
    e_mouseJoint,
    e_gearJoint,
    e_wheelJoint,
    e_weldJoint,
    e_frictionJoint,
    e_ropeJoint
}

pub enum b2LimitState
{
    e_inactiveLimit,
    e_atLowerLimit,
    e_atUpperLimit,
    e_equalLimits
}

pub struct b2Jacobian
{
    linear:   b2Vec2,
    angulara: f32,
    angularb: f32,
}

/**
  | A joint edge is used to connect bodies and
  | joints together in a joint graph where each
  | body is a node and each joint is an
  | edge. A joint edge belongs to a doubly linked
  | list maintained in each attached body. Each
  | joint has two joint nodes, one for each
  | attached body.
  */
pub struct b2JointEdge
{
    /**
      | provides quick access to the other body
      | attached.
      |
      */
    other: *mut b2Body,

    /**
      | the joint
      |
      */
    joint: *mut b2Joint,

    /**
      | the previous joint edge in the body's
      | joint list
      |
      */
    prev:  *mut b2JointEdge,

    /**
      | the next joint edge in the body's joint
      | list
      |
      */
    next:  *mut b2JointEdge,
}

/**
  | Joint definitions are used to construct
  | joints.
  |
  */
pub struct b2JointDef {

    /**
      | The joint type is set automatically
      | for concrete joint types.
      |
      */
    ty:                b2JointType,

    /**
      | Use this to attach application specific
      | data to your joints.
      |
      */
    user_data:         *mut c_void,

    /**
      | The first attached body.
      |
      */
    bodya:             *mut b2Body,

    /**
      | The second attached body.
      |
      */
    bodyb:             *mut b2Body,

    /**
      | Set this flag to true if the attached
      | bodies should collide.
      |
      */
    collide_connected: bool,
}

impl Default for b2JointDef {
    
    fn default() -> Self {
        todo!();
        /*

            type = e_unknownJoint;
            userData = NULL;
            bodyA = NULL;
            bodyB = NULL;
            collideConnected = false
        */
    }
}

pub trait B2JointInterface {

    /**
      | Get the anchor point on bodyA in world
      | coordinates.
      |
      */
    fn get_anchora(&self) -> b2Vec2;

    /**
      | Get the anchor point on bodyB in world
      | coordinates.
      |
      */
    fn get_anchorb(&self) -> b2Vec2;

    /**
      | Get the reaction force on bodyB at the
      | joint anchor in Newtons.
      |
      */
    fn get_reaction_force(&self, inv_dt: f32) -> b2Vec2;

    /**
      | Get the reaction torque on bodyB in N*m.
      |
      */
    fn get_reaction_torque(&self, inv_dt: f32) -> f32;

    /**
      | Dump this joint to the log file.
      |
      */
    fn dump(&mut self)  {
        
        todo!();
        /*
            b2Log("// Dump is not supported for this joint type.\n");
        */
    }

    fn init_velocity_constraints(&mut self, data: &b2SolverData);

    fn solve_velocity_constraints(&mut self, data: &b2SolverData);

    /**
      | This returns true if the position errors
      | are within tolerance.
      |
      */
    fn solve_position_constraints(&mut self, data: &b2SolverData) -> bool;
}

/**
  | The base joint class. Joints are used to
  | constraint two bodies together in various
  | fashions. Some joints also feature limits and
  | motors.
  */
pub struct b2Joint {
    ty:                b2JointType,
    prev:              *mut b2Joint,
    next:              *mut b2Joint,
    edgea:             b2JointEdge,
    edgeb:             b2JointEdge,
    bodya:             *mut b2Body,
    bodyb:             *mut b2Body,
    index:             i32,
    island_flag:       bool,
    collide_connected: bool,
    user_data:         *mut c_void,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Dynamics/Joints/b2Joint.cpp]
impl b2Joint {

    /**
      | Get the type of the concrete joint.
      |
      */
    #[inline] pub fn get_type(&self) -> b2JointType {
        
        todo!();
        /*
            return m_type;
        */
    }

    /**
      | Get the first body attached to this joint.
      |
      */
    #[inline] pub fn get_bodya(&mut self) -> *mut b2Body {
        
        todo!();
        /*
            return m_bodyA;
        */
    }

    /**
      | Get the second body attached to this
      | joint.
      |
      */
    #[inline] pub fn get_bodyb(&mut self) -> *mut b2Body {
        
        todo!();
        /*
            return m_bodyB;
        */
    }

    /**
      | Get the next joint the world joint list.
      |
      */
    #[inline] pub fn get_next_mut(&mut self) -> *mut b2Joint {
        
        todo!();
        /*
            return m_next;
        */
    }

    #[inline] pub fn get_next(&self) -> *const b2Joint {
        
        todo!();
        /*
            return m_next;
        */
    }

    /**
      | Get the user data pointer.
      |
      */
    #[inline] pub fn get_user_data(&self)  {
        
        todo!();
        /*
            return m_userData;
        */
    }

    /**
      | Set the user data pointer.
      |
      */
    #[inline] pub fn set_user_data(&mut self, data: *mut c_void)  {
        
        todo!();
        /*
            m_userData = data;
        */
    }

    /**
      | Get collide connected.
      | 
      | -----------
      | @note
      | 
      | modifying the collide connect flag
      | won't work correctly because the flag
      | is only checked when fixture AABBs begin
      | to overlap.
      |
      */
    #[inline] pub fn get_collide_connected(&self) -> bool {
        
        todo!();
        /*
            return m_collideConnected;
        */
    }

    pub fn create(&mut self, 
        def:       *const b2JointDef,
        allocator: *mut b2BlockAllocator) -> *mut b2Joint {
        
        todo!();
        /*
            b2Joint* joint = NULL;

        switch (def->type)
        {
        case e_distanceJoint:
            {
                void* mem = allocator->Allocate(sizeof(b2DistanceJoint));
                joint = new (mem) b2DistanceJoint((b2DistanceJointDef*)def);
            }
            break;

        case e_mouseJoint:
            {
                void* mem = allocator->Allocate(sizeof(b2MouseJoint));
                joint = new (mem) b2MouseJoint((b2MouseJointDef*)def);
            }
            break;

        case e_prismaticJoint:
            {
                void* mem = allocator->Allocate(sizeof(b2PrismaticJoint));
                joint = new (mem) b2PrismaticJoint((b2PrismaticJointDef*)def);
            }
            break;

        case e_revoluteJoint:
            {
                void* mem = allocator->Allocate(sizeof(b2RevoluteJoint));
                joint = new (mem) b2RevoluteJoint((b2RevoluteJointDef*)def);
            }
            break;

        case e_pulleyJoint:
            {
                void* mem = allocator->Allocate(sizeof(b2PulleyJoint));
                joint = new (mem) b2PulleyJoint((b2PulleyJointDef*)def);
            }
            break;

        case e_gearJoint:
            {
                void* mem = allocator->Allocate(sizeof(b2GearJoint));
                joint = new (mem) b2GearJoint((b2GearJointDef*)def);
            }
            break;

        case e_wheelJoint:
            {
                void* mem = allocator->Allocate(sizeof(b2WheelJoint));
                joint = new (mem) b2WheelJoint((b2WheelJointDef*)def);
            }
            break;

        case e_weldJoint:
            {
                void* mem = allocator->Allocate(sizeof(b2WeldJoint));
                joint = new (mem) b2WeldJoint((b2WeldJointDef*)def);
            }
            break;

        case e_frictionJoint:
            {
                void* mem = allocator->Allocate(sizeof(b2FrictionJoint));
                joint = new (mem) b2FrictionJoint((b2FrictionJointDef*)def);
            }
            break;

        case e_ropeJoint:
            {
                void* mem = allocator->Allocate(sizeof(b2RopeJoint));
                joint = new (mem) b2RopeJoint((b2RopeJointDef*)def);
            }
            break;

        default:
            b2Assert(false);
            break;
        }

        return joint;
        */
    }

    pub fn destroy(&mut self, 
        joint:     *mut b2Joint,
        allocator: *mut b2BlockAllocator)  {
        
        todo!();
        /*
            joint->~b2Joint();
        switch (joint->m_type)
        {
        case e_distanceJoint:
            allocator->Free(joint, sizeof(b2DistanceJoint));
            break;

        case e_mouseJoint:
            allocator->Free(joint, sizeof(b2MouseJoint));
            break;

        case e_prismaticJoint:
            allocator->Free(joint, sizeof(b2PrismaticJoint));
            break;

        case e_revoluteJoint:
            allocator->Free(joint, sizeof(b2RevoluteJoint));
            break;

        case e_pulleyJoint:
            allocator->Free(joint, sizeof(b2PulleyJoint));
            break;

        case e_gearJoint:
            allocator->Free(joint, sizeof(b2GearJoint));
            break;

        case e_wheelJoint:
            allocator->Free(joint, sizeof(b2WheelJoint));
            break;

        case e_weldJoint:
            allocator->Free(joint, sizeof(b2WeldJoint));
            break;

        case e_frictionJoint:
            allocator->Free(joint, sizeof(b2FrictionJoint));
            break;

        case e_ropeJoint:
            allocator->Free(joint, sizeof(b2RopeJoint));
            break;

        default:
            b2Assert(false);
            break;
        }
        */
    }

    pub fn new(def: *const b2JointDef) -> Self {
    
        todo!();
        /*
        b2Assert(def->bodyA != def->bodyB);

        m_type = def->type;
        m_prev = NULL;
        m_next = NULL;
        m_bodyA = def->bodyA;
        m_bodyB = def->bodyB;
        m_index = 0;
        m_collideConnected = def->collideConnected;
        m_islandFlag = false;
        m_userData = def->userData;

        m_edgeA.joint = NULL;
        m_edgeA.other = NULL;
        m_edgeA.prev = NULL;
        m_edgeA.next = NULL;

        m_edgeB.joint = NULL;
        m_edgeB.other = NULL;
        m_edgeB.prev = NULL;
        m_edgeB.next = NULL;
        */
    }

    /**
      | Short-cut function to determine if
      | either body is inactive.
      |
      */
    pub fn is_active(&self) -> bool {
        
        todo!();
        /*
            return m_bodyA->IsActive() && m_bodyB->IsActive();
        */
    }
}
