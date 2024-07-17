crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Dynamics/Contacts/b2CircleContact.h]

pub struct b2CircleContact {
    base: b2Contact,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Dynamics/Contacts/b2CircleContact.cpp]
impl b2CircleContact {

    pub fn create(&mut self, 
        fixturea:  *mut b2Fixture,
        _1:        i32,
        fixtureb:  *mut b2Fixture,
        _3:        i32,
        allocator: *mut b2BlockAllocator) -> *mut b2Contact {
        
        todo!();
        /*
            void* mem = allocator->Allocate(sizeof(b2CircleContact));
        return new (mem) b2CircleContact(fixtureA, fixtureB);
        */
    }
    
    pub fn destroy(&mut self, 
        contact:   *mut b2Contact,
        allocator: *mut b2BlockAllocator)  {
        
        todo!();
        /*
            ((b2CircleContact*)contact)->~b2CircleContact();
        allocator->Free(contact, sizeof(b2CircleContact));
        */
    }
    
    pub fn new(
        fixturea: *mut b2Fixture,
        fixtureb: *mut b2Fixture) -> Self {
    
        todo!();
        /*


            : b2Contact(fixtureA, 0, fixtureB, 0)

        b2Assert(m_fixtureA->GetType() == b2Shape::e_circle);
        b2Assert(m_fixtureB->GetType() == b2Shape::e_circle);
        */
    }
    
    pub fn evaluate(&mut self, 
        manifold: *mut b2Manifold,
        xfa:      &b2Transform,
        xfb:      &b2Transform)  {
        
        todo!();
        /*
            b2CollideCircles(manifold,
                        (b2CircleShape*)m_fixtureA->GetShape(), xfA,
                        (b2CircleShape*)m_fixtureB->GetShape(), xfB);
        */
    }
}
