crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Dynamics/b2Fixture.h]

/**
   This holds contact filtering data.
  */
pub struct b2Filter {

    /**
      | The collision category bits. Normally
      | you would just set one bit.
      |
      */
    category_bits: u16,

    /**
      | The collision mask bits. This states
      | the categories that this shape would
      | accept for collision.
      |
      */
    mask_bits:     u16,

    /**
      | Collision groups allow a certain group
      | of objects to never collide (negative)
      | or always collide (positive). Zero
      | means no collision group. Non-zero
      | group filtering always wins against
      | the mask bits.
      |
      */
    group_index:   i16,
}

impl Default for b2Filter {
    
    fn default() -> Self {
        todo!();
        /*


            categoryBits = 0x0001;
            maskBits = 0xFFFF;
            groupIndex = 0
        */
    }
}

/**
  | A fixture definition is used to create
  | a fixture. This class defines an abstract
  | fixture definition. You can reuse fixture
  | definitions safely.
  */
pub struct b2FixtureDef {

    /**
      | The shape, this must be set. The shape
      | will be cloned, so you can create the
      | shape on the stack.
      |
      */
    shape:       *const b2Shape,

    /**
      | Use this to store application specific
      | fixture data.
      |
      */
    user_data:   *mut c_void,

    /**
      | The friction coefficient, usually
      | in the range [0,1].
      |
      */
    friction:    f32,

    /**
      | The restitution (elasticity) usually
      | in the range [0,1].
      |
      */
    restitution: f32,

    /**
      | The density, usually in kg/m^2.
      |
      */
    density:     f32,

    /**
      | A sensor shape collects contact information
      | but never generates a collision response.
      |
      */
    is_sensor:   bool,

    /**
      | Contact filtering data.
      |
      */
    filter:      b2Filter,
}

impl Default for b2FixtureDef {

    /**
      | The constructor sets the default fixture
      | definition values.
      |
      */
    fn default() -> Self {
        todo!();
        /*


            shape = NULL;
            userData = NULL;
            friction = 0.2f;
            restitution = 0.0f;
            density = 0.0f;
            isSensor = false
        */
    }
}

/**
  | This proxy is used internally to connect
  | fixtures to the broad-phase.
  |
  */
pub struct b2FixtureProxy
{
    aabb:        b2AABB,
    fixture:     *mut b2Fixture,
    child_index: i32,
    proxy_id:    i32,
}

/**
  | A fixture is used to attach a shape to
  | a body for collision detection. A fixture
  | inherits its transform from its parent.
  | Fixtures hold additional non-geometric
  | data such as friction, collision filters,
  | etc. Fixtures are created via b2Body::CreateFixture.
  | 
  | -----------
  | @warning
  | 
  | you cannot reuse fixtures.
  |
  */
pub struct b2Fixture {
    density:     f32,
    next:        *mut b2Fixture,
    body:        *mut b2Body,
    shape:       *mut b2Shape,
    friction:    f32,
    restitution: f32,
    proxies:     *mut b2FixtureProxy,
    proxy_count: i32,
    filter:      b2Filter,
    is_sensor:   bool,
    user_data:   *mut c_void,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Dynamics/b2Fixture.cpp]
impl b2Fixture {

    /**
      | Get the type of the child shape. You can
      | use this to down cast to the concrete
      | shape.
      |
      | @return the shape type.
      */
    #[inline] pub fn get_type(&self) -> B2ShapeType {
        
        todo!();
        /*
            return m_shape->GetType();
        */
    }
    
    /**
      | Get the child shape. You can modify the
      | child shape, however you should not
      | change the number of vertices because
      | this will crash some collision caching
      | mechanisms. Manipulating the shape
      | may lead to non-physical behavior.
      |
      */
    #[inline] pub fn get_shape_mut(&mut self) -> *mut b2Shape {
        
        todo!();
        /*
            return m_shape;
        */
    }
    
    #[inline] pub fn get_shape(&self) -> *const b2Shape {
        
        todo!();
        /*
            return m_shape;
        */
    }
    
    /**
      | Is this fixture a sensor (non-solid)?
      | 
      | -----------
      | @return
      | 
      | the true if the shape is a sensor.
      |
      */
    #[inline] pub fn is_sensor(&self) -> bool {
        
        todo!();
        /*
            return m_isSensor;
        */
    }
    
    /**
      | Get the contact filtering data.
      |
      */
    #[inline] pub fn get_filter_data(&self) -> &b2Filter {
        
        todo!();
        /*
            return m_filter;
        */
    }
    
    /**
      | Get the user data that was assigned in
      | the fixture definition. Use this to
      | store your application specific data.
      |
      */
    #[inline] pub fn get_user_data(&self)  {
        
        todo!();
        /*
            return m_userData;
        */
    }
    
    /**
      | Set the user data. Use this to store your
      | application specific data.
      |
      */
    #[inline] pub fn set_user_data(&mut self, data: *mut c_void)  {
        
        todo!();
        /*
            m_userData = data;
        */
    }
    
    /**
      | Get the parent body of this fixture. This
      | is NULL if the fixture is not attached.
      |
      | @return the parent body.
      */
    #[inline] pub fn get_body_mut(&mut self) -> *mut b2Body {
        
        todo!();
        /*
            return m_body;
        */
    }
    
    #[inline] pub fn get_body(&self) -> *const b2Body {
        
        todo!();
        /*
            return m_body;
        */
    }
    
    /**
      | Get the next fixture in the parent body's
      | fixture list.
      |
      | @return the next shape.
      */
    #[inline] pub fn get_next_mut(&mut self) -> *mut b2Fixture {
        
        todo!();
        /*
            return m_next;
        */
    }
    
    #[inline] pub fn get_next(&self) -> *const b2Fixture {
        
        todo!();
        /*
            return m_next;
        */
    }
    
    /**
      | Set the density of this fixture. This will
      | _not_ automatically adjust the mass of the
      | body. You must call b2Body::ResetMassData
      | to update the body's mass.
      */
    #[inline] pub fn set_density(&mut self, density: f32)  {
        
        todo!();
        /*
            b2Assert(b2IsValid(density) && density >= 0.0f);
        m_density = density;
        */
    }
    
    /**
      | Get the density of this fixture.
      |
      */
    #[inline] pub fn get_density(&self) -> f32 {
        
        todo!();
        /*
            return m_density;
        */
    }
    
    /**
      | Get the coefficient of friction.
      |
      */
    #[inline] pub fn get_friction(&self) -> f32 {
        
        todo!();
        /*
            return m_friction;
        */
    }
    
    /**
      | Set the coefficient of friction. This
      | will _not_ change the friction of existing
      | contacts.
      |
      */
    #[inline] pub fn set_friction(&mut self, friction: f32)  {
        
        todo!();
        /*
            m_friction = friction;
        */
    }
    
    /**
      | Get the coefficient of restitution.
      |
      */
    #[inline] pub fn get_restitution(&self) -> f32 {
        
        todo!();
        /*
            return m_restitution;
        */
    }
    
    /**
      | Set the coefficient of restitution.
      | This will _not_ change the restitution
      | of existing contacts.
      |
      */
    #[inline] pub fn set_restitution(&mut self, restitution: f32)  {
        
        todo!();
        /*
            m_restitution = restitution;
        */
    }
    
    /**
      | Test a point for containment in this
      | fixture.
      | 
      | -----------
      | @param p
      | 
      | a point in world coordinates.
      |
      */
    #[inline] pub fn test_point(&self, p: &b2Vec2) -> bool {
        
        todo!();
        /*
            return m_shape->TestPoint(m_body->GetTransform(), p);
        */
    }
    
    /**
      | Cast a ray against this shape.
      | 
      | -----------
      | @param output
      | 
      | the ray-cast results.
      | ----------
      | @param input
      | 
      | the ray-cast input parameters.
      |
      */
    #[inline] pub fn ray_cast(&self, 
        output:      *mut b2RayCastOutput,
        input:       &b2RayCastInput,
        child_index: i32) -> bool {
        
        todo!();
        /*
            return m_shape->RayCast(output, input, m_body->GetTransform(), childIndex);
        */
    }
    
    /**
      | Get the mass data for this fixture. The
      | mass data is based on the density and the
      | shape. The rotational inertia is about the
      | shape's origin. This operation may be
      | expensive.
      */
    #[inline] pub fn get_mass_data(&self, mass_data: *mut b2MassData)  {
        
        todo!();
        /*
            m_shape->ComputeMass(massData, m_density);
        */
    }
    
    /**
      | Get the fixture's AABB. This AABB may be
      | enlarge and/or stale.
      |
      | If you need a more accurate AABB, compute
      | it using the shape and the body transform.
      */
    #[inline] pub fn getaabb(&self, child_index: i32) -> &b2AABB {
        
        todo!();
        /*
            b2Assert(0 <= childIndex && childIndex < m_proxyCount);
        return m_proxies[childIndex].aabb;
        */
    }
    
    pub fn new() -> Self {
    
        todo!();
        /*


            m_userData = NULL;
        m_body = NULL;
        m_next = NULL;
        m_proxies = NULL;
        m_proxyCount = 0;
        m_shape = NULL;
        m_density = 0.0f;
        */
    }
    
    /**
      | We need separation create/destroy functions
      | from the constructor/destructor because the
      | destructor cannot access the allocator (no
      | destructor arguments allowed by C++).
      */
    pub fn create(&mut self, 
        allocator: *mut b2BlockAllocator,
        body:      *mut b2Body,
        def:       *const b2FixtureDef)  {
        
        todo!();
        /*
            m_userData = def->userData;
        m_friction = def->friction;
        m_restitution = def->restitution;

        m_body = body;
        m_next = NULL;

        m_filter = def->filter;

        m_isSensor = def->isSensor;

        m_shape = def->shape->Clone(allocator);

        // Reserve proxy space
        int32 childCount = m_shape->GetChildCount();
        m_proxies = (b2FixtureProxy*)allocator->Allocate(childCount * sizeof(b2FixtureProxy));
        for (int32 i = 0; i < childCount; ++i)
        {
            m_proxies[i].fixture = NULL;
            m_proxies[i].proxyId = b2BroadPhase::e_nullProxy;
        }
        m_proxyCount = 0;

        m_density = def->density;
        */
    }
    
    pub fn destroy(&mut self, allocator: *mut b2BlockAllocator)  {
        
        todo!();
        /*
            // The proxies must be destroyed before calling this.
        b2Assert(m_proxyCount == 0);

        // Free the proxy array.
        int32 childCount = m_shape->GetChildCount();
        allocator->Free(m_proxies, childCount * sizeof(b2FixtureProxy));
        m_proxies = NULL;

        // Free the child shape.
        switch (m_shape->m_type)
        {
        case b2Shape::e_circle:
            {
                b2CircleShape* s = (b2CircleShape*)m_shape;
                s->~b2CircleShape();
                allocator->Free(s, sizeof(b2CircleShape));
            }
            break;

        case b2Shape::e_edge:
            {
                b2EdgeShape* s = (b2EdgeShape*)m_shape;
                s->~b2EdgeShape();
                allocator->Free(s, sizeof(b2EdgeShape));
            }
            break;

        case b2Shape::e_polygon:
            {
                b2PolygonShape* s = (b2PolygonShape*)m_shape;
                s->~b2PolygonShape();
                allocator->Free(s, sizeof(b2PolygonShape));
            }
            break;

        case b2Shape::e_chain:
            {
                b2ChainShape* s = (b2ChainShape*)m_shape;
                s->~b2ChainShape();
                allocator->Free(s, sizeof(b2ChainShape));
            }
            break;

        default:
            b2Assert(false);
            break;
        }

        m_shape = NULL;
        */
    }
    
    /**
      | These support body activation/deactivation.
      |
      */
    pub fn create_proxies(&mut self, 
        broad_phase: *mut b2BroadPhase,
        xf:          &b2Transform)  {
        
        todo!();
        /*
            b2Assert(m_proxyCount == 0);

        // Create proxies in the broad-phase.
        m_proxyCount = m_shape->GetChildCount();

        for (int32 i = 0; i < m_proxyCount; ++i)
        {
            b2FixtureProxy* proxy = m_proxies + i;
            m_shape->ComputeAABB(&proxy->aabb, xf, i);
            proxy->proxyId = broadPhase->CreateProxy(proxy->aabb, proxy);
            proxy->fixture = this;
            proxy->childIndex = i;
        }
        */
    }
    
    pub fn destroy_proxies(&mut self, broad_phase: *mut b2BroadPhase)  {
        
        todo!();
        /*
            // Destroy proxies in the broad-phase.
        for (int32 i = 0; i < m_proxyCount; ++i)
        {
            b2FixtureProxy* proxy = m_proxies + i;
            broadPhase->DestroyProxy(proxy->proxyId);
            proxy->proxyId = b2BroadPhase::e_nullProxy;
        }

        m_proxyCount = 0;
        */
    }
    
    pub fn synchronize(&mut self, 
        broad_phase: *mut b2BroadPhase,
        transform1:  &b2Transform,
        transform2:  &b2Transform)  {
        
        todo!();
        /*
            if (m_proxyCount == 0)
        {
            return;
        }

        for (int32 i = 0; i < m_proxyCount; ++i)
        {
            b2FixtureProxy* proxy = m_proxies + i;

            // Compute an AABB that covers the swept shape (may miss some rotation effect).
            b2AABB aabb1, aabb2;
            m_shape->ComputeAABB(&aabb1, transform1, proxy->childIndex);
            m_shape->ComputeAABB(&aabb2, transform2, proxy->childIndex);

            proxy->aabb.Combine(aabb1, aabb2);

            b2Vec2 displacement = transform2.p - transform1.p;

            broadPhase->MoveProxy(proxy->proxyId, proxy->aabb, displacement);
        }
        */
    }
    
    /**
      | Set the contact filtering data. This will
      | not update contacts until the next time
      | step when either parent body is active and
      | awake.
      |
      | This automatically calls Refilter.
      */
    pub fn set_filter_data(&mut self, filter: &b2Filter)  {
        
        todo!();
        /*
            m_filter = filter;

        Refilter();
        */
    }
    
    /**
      | Call this if you want to establish collision
      | that was previously disabled by b2ContactFilter::ShouldCollide.
      |
      */
    pub fn refilter(&mut self)  {
        
        todo!();
        /*
            if (m_body == NULL)
        {
            return;
        }

        // Flag associated contacts for filtering.
        b2ContactEdge* edge = m_body->GetContactList();
        while (edge)
        {
            b2Contact* contact = edge->contact;
            b2Fixture* fixtureA = contact->GetFixtureA();
            b2Fixture* fixtureB = contact->GetFixtureB();
            if (fixtureA == this || fixtureB == this)
            {
                contact->FlagForFiltering();
            }

            edge = edge->next;
        }

        b2World* world = m_body->GetWorld();

        if (world == NULL)
        {
            return;
        }

        // Touch each proxy so that new pairs may be created
        b2BroadPhase* broadPhase = &world->m_contactManager.m_broadPhase;
        for (int32 i = 0; i < m_proxyCount; ++i)
        {
            broadPhase->TouchProxy(m_proxies[i].proxyId);
        }
        */
    }
    
    /**
      | Set if this fixture is a sensor.
      |
      */
    pub fn set_sensor(&mut self, sensor: bool)  {
        
        todo!();
        /*
            if (sensor != m_isSensor)
        {
            m_body->SetAwake(true);
            m_isSensor = sensor;
        }
        */
    }
    
    /**
      | Dump this fixture to the log file.
      |
      */
    pub fn dump(&mut self, body_index: i32)  {
        
        todo!();
        /*
            b2Log("    b2FixtureDef fd;\n");
        b2Log("    fd.friction = %.15lef;\n", m_friction);
        b2Log("    fd.restitution = %.15lef;\n", m_restitution);
        b2Log("    fd.density = %.15lef;\n", m_density);
        b2Log("    fd.isSensor = bool(%d);\n", m_isSensor);
        b2Log("    fd.filter.categoryBits = uint16(%d);\n", m_filter.categoryBits);
        b2Log("    fd.filter.maskBits = uint16(%d);\n", m_filter.maskBits);
        b2Log("    fd.filter.groupIndex = int16(%d);\n", m_filter.groupIndex);

        switch (m_shape->m_type)
        {
        case b2Shape::e_circle:
            {
                b2CircleShape* s = (b2CircleShape*)m_shape;
                b2Log("    b2CircleShape shape;\n");
                b2Log("    shape.m_radius = %.15lef;\n", s->m_radius);
                b2Log("    shape.m_p.Set(%.15lef, %.15lef);\n", s->m_p.x, s->m_p.y);
            }
            break;

        case b2Shape::e_edge:
            {
                b2EdgeShape* s = (b2EdgeShape*)m_shape;
                b2Log("    b2EdgeShape shape;\n");
                b2Log("    shape.m_radius = %.15lef;\n", s->m_radius);
                b2Log("    shape.m_vertex0.Set(%.15lef, %.15lef);\n", s->m_vertex0.x, s->m_vertex0.y);
                b2Log("    shape.m_vertex1.Set(%.15lef, %.15lef);\n", s->m_vertex1.x, s->m_vertex1.y);
                b2Log("    shape.m_vertex2.Set(%.15lef, %.15lef);\n", s->m_vertex2.x, s->m_vertex2.y);
                b2Log("    shape.m_vertex3.Set(%.15lef, %.15lef);\n", s->m_vertex3.x, s->m_vertex3.y);
                b2Log("    shape.m_hasVertex0 = bool(%d);\n", s->m_hasVertex0);
                b2Log("    shape.m_hasVertex3 = bool(%d);\n", s->m_hasVertex3);
            }
            break;

        case b2Shape::e_polygon:
            {
                b2PolygonShape* s = (b2PolygonShape*)m_shape;
                b2Log("    b2PolygonShape shape;\n");
                b2Log("    b2Vec2 vs[%d];\n", b2_maxPolygonVertices);
                for (int32 i = 0; i < s->m_vertexCount; ++i)
                {
                    b2Log("    vs[%d].Set(%.15lef, %.15lef);\n", i, s->m_vertices[i].x, s->m_vertices[i].y);
                }
                b2Log("    shape.Set(vs, %d);\n", s->m_vertexCount);
            }
            break;

        case b2Shape::e_chain:
            {
                b2ChainShape* s = (b2ChainShape*)m_shape;
                b2Log("    b2ChainShape shape;\n");
                b2Log("    b2Vec2 vs[%d];\n", s->m_count);
                for (int32 i = 0; i < s->m_count; ++i)
                {
                    b2Log("    vs[%d].Set(%.15lef, %.15lef);\n", i, s->m_vertices[i].x, s->m_vertices[i].y);
                }
                b2Log("    shape.CreateChain(vs, %d);\n", s->m_count);
                b2Log("    shape.m_prevVertex.Set(%.15lef, %.15lef);\n", s->m_prevVertex.x, s->m_prevVertex.y);
                b2Log("    shape.m_nextVertex.Set(%.15lef, %.15lef);\n", s->m_nextVertex.x, s->m_nextVertex.y);
                b2Log("    shape.m_hasPrevVertex = bool(%d);\n", s->m_hasPrevVertex);
                b2Log("    shape.m_hasNextVertex = bool(%d);\n", s->m_hasNextVertex);
            }
            break;

        default:
            return;
        }

        b2Log("\n");
        b2Log("    fd.shape = &shape;\n");
        b2Log("\n");
        b2Log("    bodies[%d]->CreateFixture(&fd);\n", bodyIndex);
        */
    }
}
