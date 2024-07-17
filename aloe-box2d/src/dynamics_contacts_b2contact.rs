crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Dynamics/Contacts/b2Contact.h]

/**
  | Friction mixing law. The idea is to allow
  | either fixture to drive the restitution to
  | zero.
  |
  | For example, anything slides on ice.
  */
#[inline] pub fn b2mix_friction(
        friction1: f32,
        friction2: f32) -> f32 {
    
    todo!();
    /*
        return std::sqrt(friction1 * friction2);
    */
}

/**
  | Restitution mixing law. The idea is allow for
  | anything to bounce off an inelastic surface.
  |
  | For example, a superball bounces on anything.
  */
#[inline] pub fn b2mix_restitution(
        restitution1: f32,
        restitution2: f32) -> f32 {
    
    todo!();
    /*
        return restitution1 > restitution2 ? restitution1 : restitution2;
    */
}

pub type b2ContactCreateFcn = fn(
        fixturea:  *mut b2Fixture,
        indexa:    i32,
        fixtureb:  *mut b2Fixture,
        indexb:    i32,
        allocator: *mut b2BlockAllocator) -> *mut b2Contact;


pub type b2ContactDestroyFcn = fn(
        contact:   *mut b2Contact,
        allocator: *mut b2BlockAllocator);


pub struct b2ContactRegister
{
    create_fcn:  *mut b2ContactCreateFcn,
    destroy_fcn: *mut b2ContactDestroyFcn,
    primary:     bool,
}

/**
  | A contact edge is used to connect bodies and
  | contacts together in a contact graph where
  | each body is a node and each contact is an
  | edge. A contact edge belongs to a doubly
  | linked list maintained in each attached
  | body. Each contact has two contact nodes, one
  | for each attached body.
  */
pub struct b2ContactEdge
{
    /**
      provides quick access to the other body
      attached.
      */
    other:   *mut b2Body,

    /**
      the contact
      */
    contact: *mut b2Contact,

    /**
      the previous contact edge in the body's
      contact list
      */
    prev:    *mut b2ContactEdge,

    /**
      the next contact edge in the body's contact
      list
      */
    next:    *mut b2ContactEdge,
}

/**
  | Used when crawling contact graph when
  | forming islands.
  |
  */
pub const B2_CONTACT_E_ISLAND_FLAG: usize = 0x0001;

/**
  | Set when the shapes are touching.
  |
  */
pub const B2_CONTACT_E_TOUCHING_FLAG: usize = 0x0002;

/**
  | This contact can be disabled (by user)
  |
  */
pub const B2_CONTACT_E_ENABLED_FLAG: usize = 0x0004;

/**
  | This contact needs filtering because
  | a fixture filter was changed.
  |
  */
pub const B2_CONTACT_E_FILTER_FLAG: usize = 0x0008;

/**
  | This bullet contact had a TOI event
  |
  */
pub const B2_CONTACT_E_BULLET_HIT_FLAG: usize = 0x0010;

/**
  | This contact has a valid TOI in m_toi
  |
  */
pub const B2_CONTACT_E_TOI_FLAG: usize = 0x0020;

pub trait B2ContactInterface {

    /**
      | Evaluate this contact with your own
      | manifold and transforms.
      |
      */
    fn evaluate(
        &mut self, 
        manifold: *mut b2Manifold,
        xfa:      &b2Transform,
        xfb:      &b2Transform
    );
}

/**
  | The class manages contact between two
  | shapes. A contact exists for each overlapping
  | AABB in the broad-phase (except if
  | filtered). Therefore a contact object may
  | exist that has no contact points.
  */
pub struct b2Contact {
    flags:       u32,

    /**
      | World pool and list pointers.
      |
      */
    prev:        *mut b2Contact,

    next:        *mut b2Contact,

    /**
      | Nodes for connecting bodies.
      |
      */
    nodea:       b2ContactEdge,

    nodeb:       b2ContactEdge,
    fixturea:    *mut b2Fixture,
    fixtureb:    *mut b2Fixture,
    indexa:      i32,
    indexb:      i32,
    manifold:    b2Manifold,
    toi_count:   i32,
    toi:         f32,
    friction:    f32,
    restitution: f32,
}

pub mod b2_contact {

    use super::*;

    lazy_static!{
        /*
           static b2ContactRegister s_registers[b2Shape::e_typeCount][b2Shape::e_typeCount];
           static bool s_initialized;
           b2ContactRegister b2Contact::s_registers[b2Shape::e_typeCount][b2Shape::e_typeCount];
           bool b2Contact::s_initialized = false;
           */
    }
}

impl Default for b2Contact {
    
    fn default() -> Self {
        todo!();
        /*
        : fixturea(NULL),
        : fixtureb(NULL),

        
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Dynamics/Contacts/b2Contact.cpp]
impl b2Contact {

    /**
      | Get the contact manifold. Do not modify
      | the manifold unless you understand
      | the internals of Box2D.
      |
      */
    #[inline] pub fn get_manifold_mut(&mut self) -> *mut b2Manifold {
        
        todo!();
        /*
            return &m_manifold;
        */
    }
    
    #[inline] pub fn get_manifold(&self) -> *const b2Manifold {
        
        todo!();
        /*
            return &m_manifold;
        */
    }
    
    /**
      | Get the world manifold.
      |
      */
    #[inline] pub fn get_world_manifold(&self, world_manifold: *mut b2WorldManifold)  {
        
        todo!();
        /*
            const b2Body* bodyA = m_fixtureA->GetBody();
        const b2Body* bodyB = m_fixtureB->GetBody();
        const b2Shape* shapeA = m_fixtureA->GetShape();
        const b2Shape* shapeB = m_fixtureB->GetShape();

        worldManifold->Initialize(&m_manifold, bodyA->GetTransform(), shapeA->m_radius, bodyB->GetTransform(), shapeB->m_radius);
        */
    }
    
    /**
      | Enable/disable this contact. This can be
      | used inside the pre-solve contact
      | listener. The contact is only disabled for
      | the current time step (or sub-step in
      | continuous collisions).
      */
    #[inline] pub fn set_enabled(&mut self, flag: bool)  {
        
        todo!();
        /*
            if (flag)
        {
            m_flags |= B2_CONTACT_E_ENABLED_FLAG;
        }
        else
        {
            m_flags &= ~((unsigned int) B2_CONTACT_E_ENABLED_FLAG);
        }
        */
    }
    
    /**
      | Has this contact been disabled?
      |
      */
    #[inline] pub fn is_enabled(&self) -> bool {
        
        todo!();
        /*
            return (m_flags & B2_CONTACT_E_ENABLED_FLAG) == B2_CONTACT_E_ENABLED_FLAG;
        */
    }
    
    /**
      | Is this contact touching?
      |
      */
    #[inline] pub fn is_touching(&self) -> bool {
        
        todo!();
        /*
            return (m_flags & B2_CONTACT_E_TOUCHING_FLAG) == B2_CONTACT_E_TOUCHING_FLAG;
        */
    }
    
    /**
      | Get the next contact in the world's contact
      | list.
      |
      */
    #[inline] pub fn get_next_mut(&mut self) -> *mut b2Contact {
        
        todo!();
        /*
            return m_next;
        */
    }
    
    #[inline] pub fn get_next(&self) -> *const b2Contact {
        
        todo!();
        /*
            return m_next;
        */
    }
    
    /**
      | Get fixture A in this contact.
      |
      */
    #[inline] pub fn get_fixturea_mut(&mut self) -> *mut b2Fixture {
        
        todo!();
        /*
            return m_fixtureA;
        */
    }
    
    #[inline] pub fn get_fixturea(&self) -> *const b2Fixture {
        
        todo!();
        /*
            return m_fixtureA;
        */
    }
    
    /**
      | Get fixture B in this contact.
      |
      */
    #[inline] pub fn get_fixtureb_mut(&mut self) -> *mut b2Fixture {
        
        todo!();
        /*
            return m_fixtureB;
        */
    }
    
    /**
      | Get the child primitive index for fixture
      | A.
      |
      */
    #[inline] pub fn get_child_indexa(&self) -> i32 {
        
        todo!();
        /*
            return m_indexA;
        */
    }
    
    #[inline] pub fn get_fixtureb(&self) -> *const b2Fixture {
        
        todo!();
        /*
            return m_fixtureB;
        */
    }
    
    /**
      | Get the child primitive index for fixture
      | B.
      |
      */
    #[inline] pub fn get_child_indexb(&self) -> i32 {
        
        todo!();
        /*
            return m_indexB;
        */
    }
    
    /**
      | Flag this contact for filtering. Filtering
      | will occur the next time step.
      |
      */
    #[inline] pub fn flag_for_filtering(&mut self)  {
        
        todo!();
        /*
            m_flags |= B2_CONTACT_E_FILTER_FLAG;
        */
    }
    
    /**
      | Override the default friction mixture. You
      | can call this in
      | b2ContactListener::PreSolve.
      |
      | This value persists until set or reset.
      */
    #[inline] pub fn set_friction(&mut self, friction: f32)  {
        
        todo!();
        /*
            m_friction = friction;
        */
    }
    
    /**
      | Get the friction.
      |
      */
    #[inline] pub fn get_friction(&self) -> f32 {
        
        todo!();
        /*
            return m_friction;
        */
    }
    
    /**
      | Reset the friction mixture to the default
      | value.
      |
      */
    #[inline] pub fn reset_friction(&mut self)  {
        
        todo!();
        /*
            m_friction = b2MixFriction(m_fixtureA->m_friction, m_fixtureB->m_friction);
        */
    }
    
    /**
      | Override the default restitution
      | mixture. You can call this in
      | b2ContactListener::PreSolve.
      |
      | The value persists until you set or reset.
      */
    #[inline] pub fn set_restitution(&mut self, restitution: f32)  {
        
        todo!();
        /*
            m_restitution = restitution;
        */
    }
    
    /**
      | Get the restitution.
      |
      */
    #[inline] pub fn get_restitution(&self) -> f32 {
        
        todo!();
        /*
            return m_restitution;
        */
    }
    
    /**
      | Reset the restitution to the default
      | value.
      |
      */
    #[inline] pub fn reset_restitution(&mut self)  {
        
        todo!();
        /*
            m_restitution = b2MixRestitution(m_fixtureA->m_restitution, m_fixtureB->m_restitution);
        */
    }
    
    pub fn initialize_registers(&mut self)  {
        
        todo!();
        /*
            AddType(b2CircleContact::Create, b2CircleContact::Destroy, b2Shape::e_circle, b2Shape::e_circle);
        AddType(b2PolygonAndCircleContact::Create, b2PolygonAndCircleContact::Destroy, b2Shape::e_polygon, b2Shape::e_circle);
        AddType(b2PolygonContact::Create, b2PolygonContact::Destroy, b2Shape::e_polygon, b2Shape::e_polygon);
        AddType(b2EdgeAndCircleContact::Create, b2EdgeAndCircleContact::Destroy, b2Shape::e_edge, b2Shape::e_circle);
        AddType(b2EdgeAndPolygonContact::Create, b2EdgeAndPolygonContact::Destroy, b2Shape::e_edge, b2Shape::e_polygon);
        AddType(b2ChainAndCircleContact::Create, b2ChainAndCircleContact::Destroy, b2Shape::e_chain, b2Shape::e_circle);
        AddType(b2ChainAndPolygonContact::Create, b2ChainAndPolygonContact::Destroy, b2Shape::e_chain, b2Shape::e_polygon);
        */
    }
    
    pub fn add_type(
        &mut self, 
        create_fcn:  *mut b2ContactCreateFcn,
        destory_fcn: *mut b2ContactDestroyFcn,
        type1:       B2ShapeType,
        type2:       B2ShapeType

    ) {
        
        todo!();
        /*
            b2Assert(0 <= type1 && type1 < b2Shape::e_typeCount);
        b2Assert(0 <= type2 && type2 < b2Shape::e_typeCount);

        s_registers[type1][type2].createFcn = createFcn;
        s_registers[type1][type2].destroyFcn = destoryFcn;
        s_registers[type1][type2].primary = true;

        if (type1 != type2)
        {
            s_registers[type2][type1].createFcn = createFcn;
            s_registers[type2][type1].destroyFcn = destoryFcn;
            s_registers[type2][type1].primary = false;
        }
        */
    }
    
    pub fn create(&mut self, 
        fixturea:  *mut b2Fixture,
        indexa:    i32,
        fixtureb:  *mut b2Fixture,
        indexb:    i32,
        allocator: *mut b2BlockAllocator) -> *mut b2Contact {
        
        todo!();
        /*
            if (s_initialized == false)
        {
            InitializeRegisters();
            s_initialized = true;
        }

        b2Shape::Type type1 = fixtureA->GetType();
        b2Shape::Type type2 = fixtureB->GetType();

        b2Assert(0 <= type1 && type1 < b2Shape::e_typeCount);
        b2Assert(0 <= type2 && type2 < b2Shape::e_typeCount);

        b2ContactCreateFcn* createFcn = s_registers[type1][type2].createFcn;
        if (createFcn)
        {
            if (s_registers[type1][type2].primary)
            {
                return createFcn(fixtureA, indexA, fixtureB, indexB, allocator);
            }
            else
            {
                return createFcn(fixtureB, indexB, fixtureA, indexA, allocator);
            }
        }
        else
        {
            return NULL;
        }
        */
    }
    
    pub fn destroy(&mut self, 
        contact:   *mut b2Contact,
        allocator: *mut b2BlockAllocator)  {
        
        todo!();
        /*
            b2Assert(s_initialized == true);

        if (contact->m_manifold.pointCount > 0)
        {
            contact->GetFixtureA()->GetBody()->SetAwake(true);
            contact->GetFixtureB()->GetBody()->SetAwake(true);
        }

        b2Shape::Type typeA = contact->GetFixtureA()->GetType();
        b2Shape::Type typeB = contact->GetFixtureB()->GetType();

        b2Assert(0 <= typeA && typeB < b2Shape::e_typeCount);
        b2Assert(0 <= typeA && typeB < b2Shape::e_typeCount);

        b2ContactDestroyFcn* destroyFcn = s_registers[typeA][typeB].destroyFcn;
        destroyFcn(contact, allocator);
        */
    }
    
    pub fn new(
        fa:     *mut b2Fixture,
        indexa: i32,
        fb:     *mut b2Fixture,
        indexb: i32) -> Self {
    
        todo!();
        /*


            m_flags = B2_CONTACT_E_ENABLED_FLAG;

        m_fixtureA = fA;
        m_fixtureB = fB;

        m_indexA = indexA;
        m_indexB = indexB;

        m_manifold.pointCount = 0;

        m_prev = NULL;
        m_next = NULL;

        m_nodeA.contact = NULL;
        m_nodeA.prev = NULL;
        m_nodeA.next = NULL;
        m_nodeA.other = NULL;

        m_nodeB.contact = NULL;
        m_nodeB.prev = NULL;
        m_nodeB.next = NULL;
        m_nodeB.other = NULL;

        m_toiCount = 0;

        m_friction = b2MixFriction(m_fixtureA->m_friction, m_fixtureB->m_friction);
        m_restitution = b2MixRestitution(m_fixtureA->m_restitution, m_fixtureB->m_restitution);
        */
    }
    
    /**
      | Update the contact manifold and touching
      | status.
      |
      | Note: do not assume the fixture AABBs are
      | overlapping or are valid.
      */
    pub fn update(&mut self, listener: *mut dyn b2ContactListener)  {
        
        todo!();
        /*
            b2Manifold oldManifold = m_manifold;

        // Re-enable this contact.
        m_flags |= B2_CONTACT_E_ENABLED_FLAG;

        bool touching = false;
        bool wasTouching = (m_flags & B2_CONTACT_E_TOUCHING_FLAG) == B2_CONTACT_E_TOUCHING_FLAG;

        bool sensorA = m_fixtureA->IsSensor();
        bool sensorB = m_fixtureB->IsSensor();
        bool sensor = sensorA || sensorB;

        b2Body* bodyA = m_fixtureA->GetBody();
        b2Body* bodyB = m_fixtureB->GetBody();
        const b2Transform& xfA = bodyA->GetTransform();
        const b2Transform& xfB = bodyB->GetTransform();

        // Is this contact a sensor?
        if (sensor)
        {
            const b2Shape* shapeA = m_fixtureA->GetShape();
            const b2Shape* shapeB = m_fixtureB->GetShape();
            touching = b2TestOverlap(shapeA, m_indexA, shapeB, m_indexB, xfA, xfB);

            // Sensors don't generate manifolds.
            m_manifold.pointCount = 0;
        }
        else
        {
            Evaluate(&m_manifold, xfA, xfB);
            touching = m_manifold.pointCount > 0;

            // Match old contact ids to new contact ids and copy the
            // stored impulses to warm start the solver.
            for (int32 i = 0; i < m_manifold.pointCount; ++i)
            {
                b2ManifoldPoint* mp2 = m_manifold.points + i;
                mp2->normalImpulse = 0.0f;
                mp2->tangentImpulse = 0.0f;
                b2ContactID id2 = mp2->id;

                for (int32 j = 0; j < oldManifold.pointCount; ++j)
                {
                    b2ManifoldPoint* mp1 = oldManifold.points + j;

                    if (mp1->id.key == id2.key)
                    {
                        mp2->normalImpulse = mp1->normalImpulse;
                        mp2->tangentImpulse = mp1->tangentImpulse;
                        break;
                    }
                }
            }

            if (touching != wasTouching)
            {
                bodyA->SetAwake(true);
                bodyB->SetAwake(true);
            }
        }

        if (touching)
        {
            m_flags |= B2_CONTACT_E_TOUCHING_FLAG;
        }
        else
        {
            m_flags &= ~B2_CONTACT_E_TOUCHING_FLAG;
        }

        if (wasTouching == false && touching == true && listener)
        {
            listener->BeginContact(this);
        }

        if (wasTouching == true && touching == false && listener)
        {
            listener->EndContact(this);
        }

        if (sensor == false && touching && listener)
        {
            listener->PreSolve(this, &oldManifold);
        }
        */
    }
}
