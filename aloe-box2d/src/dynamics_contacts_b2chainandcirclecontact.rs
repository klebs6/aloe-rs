crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Dynamics/Contacts/b2ChainAndCircleContact.h]

pub struct b2ChainAndCircleContact {
    base: b2Contact,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Dynamics/Contacts/b2ChainAndCircleContact.cpp]
impl b2ChainAndCircleContact {

    pub fn create(&mut self, 
        fixturea:  *mut b2Fixture,
        indexa:    i32,
        fixtureb:  *mut b2Fixture,
        indexb:    i32,
        allocator: *mut b2BlockAllocator) -> *mut b2Contact {
        
        todo!();
        /*
            void* mem = allocator->Allocate(sizeof(b2ChainAndCircleContact));
        return new (mem) b2ChainAndCircleContact(fixtureA, indexA, fixtureB, indexB);
        */
    }
    
    pub fn destroy(&mut self, 
        contact:   *mut b2Contact,
        allocator: *mut b2BlockAllocator)  {
        
        todo!();
        /*
            ((b2ChainAndCircleContact*)contact)->~b2ChainAndCircleContact();
        allocator->Free(contact, sizeof(b2ChainAndCircleContact));
        */
    }
    
    pub fn new(
        fixturea: *mut b2Fixture,
        indexa:   i32,
        fixtureb: *mut b2Fixture,
        indexb:   i32) -> Self {
    
        todo!();
        /*


            : b2Contact(fixtureA, indexA, fixtureB, indexB)

        b2Assert(m_fixtureA->GetType() == b2Shape::e_chain);
        b2Assert(m_fixtureB->GetType() == b2Shape::e_circle);
        */
    }
    
    pub fn evaluate(&mut self, 
        manifold: *mut b2Manifold,
        xfa:      &b2Transform,
        xfb:      &b2Transform)  {
        
        todo!();
        /*
            b2ChainShape* chain = (b2ChainShape*)m_fixtureA->GetShape();
        b2EdgeShape edge;
        chain->GetChildEdge(&edge, m_indexA);
        b2CollideEdgeAndCircle( manifold, &edge, xfA,
                                (b2CircleShape*)m_fixtureB->GetShape(), xfB);
        */
    }
}
