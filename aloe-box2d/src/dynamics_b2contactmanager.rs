crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Dynamics/b2ContactManager.h]

/**
   Delegate of b2World.
  */
pub struct b2ContactManager {
    broad_phase:      b2BroadPhase,
    contact_list:     *mut b2Contact,
    contact_count:    i32,
    contact_filter:   *mut dyn b2ContactFilter,
    contact_listener: *mut dyn b2ContactListener,
    allocator:        *mut b2BlockAllocator,
}

lazy_static!{
    /*
    b2ContactFilter b2_defaultFilter;
    b2ContactListener b2_defaultListener;
    */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Dynamics/b2ContactManager.cpp]
impl Default for b2ContactManager {

    fn default() -> Self {
    
        todo!();
        /*


            m_contactList = NULL;
        m_contactCount = 0;
        m_contactFilter = &b2_defaultFilter;
        m_contactListener = &b2_defaultListener;
        m_allocator = NULL;
        */
    }
}

impl b2ContactManager {
    
    pub fn destroy(&mut self, c: *mut b2Contact)  {
        
        todo!();
        /*
            b2Fixture* fixtureA = c->GetFixtureA();
        b2Fixture* fixtureB = c->GetFixtureB();
        b2Body* bodyA = fixtureA->GetBody();
        b2Body* bodyB = fixtureB->GetBody();

        if (m_contactListener && c->IsTouching())
        {
            m_contactListener->EndContact(c);
        }

        // Remove from the world.
        if (c->m_prev)
        {
            c->m_prev->m_next = c->m_next;
        }

        if (c->m_next)
        {
            c->m_next->m_prev = c->m_prev;
        }

        if (c == m_contactList)
        {
            m_contactList = c->m_next;
        }

        // Remove from body 1
        if (c->m_nodeA.prev)
        {
            c->m_nodeA.prev->next = c->m_nodeA.next;
        }

        if (c->m_nodeA.next)
        {
            c->m_nodeA.next->prev = c->m_nodeA.prev;
        }

        if (&c->m_nodeA == bodyA->m_contactList)
        {
            bodyA->m_contactList = c->m_nodeA.next;
        }

        // Remove from body 2
        if (c->m_nodeB.prev)
        {
            c->m_nodeB.prev->next = c->m_nodeB.next;
        }

        if (c->m_nodeB.next)
        {
            c->m_nodeB.next->prev = c->m_nodeB.prev;
        }

        if (&c->m_nodeB == bodyB->m_contactList)
        {
            bodyB->m_contactList = c->m_nodeB.next;
        }

        // Call the factory.
        b2Contact::Destroy(c, m_allocator);
        --m_contactCount;
        */
    }

    /**
      | This is the top level collision call for
      | the time step. Here all the narrow phase
      | collision is processed for the world
      | contact list.
      */
    pub fn collide(&mut self)  {
        
        todo!();
        /*
            // Update awake contacts.
        b2Contact* c = m_contactList;
        while (c)
        {
            b2Fixture* fixtureA = c->GetFixtureA();
            b2Fixture* fixtureB = c->GetFixtureB();
            int32 indexA = c->GetChildIndexA();
            int32 indexB = c->GetChildIndexB();
            b2Body* bodyA = fixtureA->GetBody();
            b2Body* bodyB = fixtureB->GetBody();

            // Is this contact flagged for filtering?
            if (c->m_flags & b2Contact::e_filterFlag)
            {
                // Should these bodies collide?
                if (bodyB->ShouldCollide(bodyA) == false)
                {
                    b2Contact* cNuke = c;
                    c = cNuke->GetNext();
                    Destroy(cNuke);
                    continue;
                }

                // Check user filtering.
                if (m_contactFilter && m_contactFilter->ShouldCollide(fixtureA, fixtureB) == false)
                {
                    b2Contact* cNuke = c;
                    c = cNuke->GetNext();
                    Destroy(cNuke);
                    continue;
                }

                // Clear the filtering flag.
                c->m_flags &= ~b2Contact::e_filterFlag;
            }

            bool activeA = bodyA->IsAwake() && bodyA->m_type != b2_staticBody;
            bool activeB = bodyB->IsAwake() && bodyB->m_type != b2_staticBody;

            // At least one body must be awake and it must be dynamic or kinematic.
            if (activeA == false && activeB == false)
            {
                c = c->GetNext();
                continue;
            }

            int32 proxyIdA = fixtureA->m_proxies[indexA].proxyId;
            int32 proxyIdB = fixtureB->m_proxies[indexB].proxyId;
            bool overlap = m_broadPhase.TestOverlap(proxyIdA, proxyIdB);

            // Here we destroy contacts that cease to overlap in the broad-phase.
            if (overlap == false)
            {
                b2Contact* cNuke = c;
                c = cNuke->GetNext();
                Destroy(cNuke);
                continue;
            }

            // The contact persists.
            c->Update(m_contactListener);
            c = c->GetNext();
        }
        */
    }
    
    pub fn find_new_contacts(&mut self)  {
        
        todo!();
        /*
            m_broadPhase.UpdatePairs(this);
        */
    }
    
    /**
      | Broad-phase callback.
      |
      */
    pub fn add_pair(&mut self, 
        proxy_user_dataa: *mut c_void,
        proxy_user_datab: *mut c_void)  {
        
        todo!();
        /*
            b2FixtureProxy* proxyA = (b2FixtureProxy*)proxyUserDataA;
        b2FixtureProxy* proxyB = (b2FixtureProxy*)proxyUserDataB;

        b2Fixture* fixtureA = proxyA->fixture;
        b2Fixture* fixtureB = proxyB->fixture;

        int32 indexA = proxyA->childIndex;
        int32 indexB = proxyB->childIndex;

        b2Body* bodyA = fixtureA->GetBody();
        b2Body* bodyB = fixtureB->GetBody();

        // Are the fixtures on the same body?
        if (bodyA == bodyB)
        {
            return;
        }

        // TODO_ERIN use a hash table to remove a potential bottleneck when both
        // bodies have a lot of contacts.
        // Does a contact already exist?
        b2ContactEdge* edge = bodyB->GetContactList();
        while (edge)
        {
            if (edge->other == bodyA)
            {
                b2Fixture* fA = edge->contact->GetFixtureA();
                b2Fixture* fB = edge->contact->GetFixtureB();
                int32 iA = edge->contact->GetChildIndexA();
                int32 iB = edge->contact->GetChildIndexB();

                if (fA == fixtureA && fB == fixtureB && iA == indexA && iB == indexB)
                {
                    // A contact already exists.
                    return;
                }

                if (fA == fixtureB && fB == fixtureA && iA == indexB && iB == indexA)
                {
                    // A contact already exists.
                    return;
                }
            }

            edge = edge->next;
        }

        // Does a joint override collision? Is at least one body dynamic?
        if (bodyB->ShouldCollide(bodyA) == false)
        {
            return;
        }

        // Check user filtering.
        if (m_contactFilter && m_contactFilter->ShouldCollide(fixtureA, fixtureB) == false)
        {
            return;
        }

        // Call the factory.
        b2Contact* c = b2Contact::Create(fixtureA, indexA, fixtureB, indexB, m_allocator);
        if (c == NULL)
        {
            return;
        }

        // Contact creation may swap fixtures.
        fixtureA = c->GetFixtureA();
        fixtureB = c->GetFixtureB();
    //  indexA = c->GetChildIndexA();
    //  indexB = c->GetChildIndexB();
        bodyA = fixtureA->GetBody();
        bodyB = fixtureB->GetBody();

        // Insert into the world.
        c->m_prev = NULL;
        c->m_next = m_contactList;
        if (m_contactList != NULL)
        {
            m_contactList->m_prev = c;
        }
        m_contactList = c;

        // Connect to island graph.

        // Connect to body A
        c->m_nodeA.contact = c;
        c->m_nodeA.other = bodyB;

        c->m_nodeA.prev = NULL;
        c->m_nodeA.next = bodyA->m_contactList;
        if (bodyA->m_contactList != NULL)
        {
            bodyA->m_contactList->prev = &c->m_nodeA;
        }
        bodyA->m_contactList = &c->m_nodeA;

        // Connect to body B
        c->m_nodeB.contact = c;
        c->m_nodeB.other = bodyA;

        c->m_nodeB.prev = NULL;
        c->m_nodeB.next = bodyB->m_contactList;
        if (bodyB->m_contactList != NULL)
        {
            bodyB->m_contactList->prev = &c->m_nodeB;
        }
        bodyB->m_contactList = &c->m_nodeB;

        // Wake up the bodies
        bodyA->SetAwake(true);
        bodyB->SetAwake(true);

        ++m_contactCount;
        */
    }
}
