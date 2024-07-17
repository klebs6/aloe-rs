crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/maths/aloe_Matrix_test.cpp]

pub struct LinearAlgebraUnitTest {
    base: UnitTest,
}

pub mod linear_algebra_unit_test {
    use super::*;

    pub mod addition_test {

        use super::*;

        pub fn run<ElementType>(u: &mut LinearAlgebraUnitTest)  {
        
            todo!();
            /*
                const ElementType data1[] = { 1,  2, 3,  4,  5,  6,  7,  8 };
                    const ElementType data2[] = { 1, -1, 3, -1,  5, -1,  7, -1 };
                    const ElementType data3[] = { 2,  1, 6,  3, 10,  5, 14,  7 };

                    Matrix<ElementType> mat1 (2, 4, data1);
                    Matrix<ElementType> mat2 (2, 4, data2);
                    Matrix<ElementType> mat3 (2, 4, data3);

                    u.expect((mat1 + mat2) == mat3);
            */
        }
    }

    pub mod difference_test {
        use super::*;

        pub fn run<ElementType>(u: &mut LinearAlgebraUnitTest)  {
        
            todo!();
            /*
                const ElementType data1[] = { 1,  2, 3,  4, 5,  6, 7,  8 };
                    const ElementType data2[] = { 1, -1, 3, -1, 5, -1, 7, -1 };
                    const ElementType data3[] = { 0,  3, 0,  5, 0,  7, 0,  9 };

                    Matrix<ElementType> mat1 (2, 4, data1);
                    Matrix<ElementType> mat2 (2, 4, data2);
                    Matrix<ElementType> mat3 (2, 4, data3);

                    u.expect((mat1 - mat2) == mat3);
            */
        }
    }

    pub mod scalar_multiplication_test {
        use super::*;

        pub fn run<ElementType>(u: &mut LinearAlgebraUnitTest)  {
        
            todo!();
            /*
                const ElementType data1[] = { 1,  2, 3,  4, 5,  6, 7,  8 };
                    const ElementType scalar = 2.0;
                    const ElementType data2[] = { 2, 4, 6, 8, 10, 12, 14, 16 };

                    Matrix<ElementType> x (2, 4, data1);
                    Matrix<ElementType> expected (2, 4, data2);

                    u.expect ((x * scalar) == expected);
            */
        }
    }

    pub mod hadamard_product_test {
        use super::*;
        
        pub fn run<ElementType>(u: &mut LinearAlgebraUnitTest)  {
        
            todo!();
            /*
                const ElementType data1[] = { 1,  2, 3,  4,  5,  6,  7,  8 };
                    const ElementType data2[] = { 1, -1, 3, -1,  5, -1,  7, -1 };
                    const ElementType data3[] = { 1, -2, 9, -4, 25, -6, 49, -8 };

                    Matrix<ElementType> mat1 (2, 4, data1);
                    Matrix<ElementType> mat2 (2, 4, data2);
                    Matrix<ElementType> mat3 (2, 4, data3);

                    u.expect (Matrix<ElementType>::hadarmard (mat1, mat2) == mat3);
            */
        }
    }

    pub mod multiplication_test {
        use super::*;

        pub fn run<ElementType>(u: &mut LinearAlgebraUnitTest)  {
        
            todo!();
            /*
                const ElementType data1[] = { 1,  2, 3,  4,  5,  6,  7,  8 };
                    const ElementType data2[] = { 1, -1, 3, -1,  5, -1,  7, -1 };
                    const ElementType data3[] = { 50, -10, 114, -26 };

                    Matrix<ElementType> mat1 (2, 4, data1);
                    Matrix<ElementType> mat2 (4, 2, data2);
                    Matrix<ElementType> mat3 (2, 2, data3);

                    u.expect((mat1 * mat2) == mat3);
            */
        }
    }

    pub mod identity_test {
        use super::*;

        pub fn run<ElementType>(u: &mut LinearAlgebraUnitTest)  {
        
            todo!();
            /*
                const ElementType data1[] = { 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1};
                    u.expect (Matrix<ElementType>::identity (4) == Matrix<ElementType> (4, 4, data1));
            */
        }
    }

    pub mod solving_test {
        use super::*;

        pub fn run<ElementType>(u: &mut LinearAlgebraUnitTest)  {
        
            todo!();
            /*
                const ElementType data1[] = { 1, -1, 2, -2 };
                    const ElementType data2[] = { -1, 0, -1, -7 };
                    const ElementType data3[] = { 1, 4, 2, 1, -1, 1, 4, 3, -2, -1, 1, 1, -1, 0, 1, 4 };

                    Matrix<ElementType> X (4, 1, data1);
                    Matrix<ElementType> B (4, 1, data2);
                    Matrix<ElementType> A (4, 4, data3);

                    u.expect (A.solve (B));
                    u.expect (Matrix<ElementType>::compare (X, B, (ElementType) 1e-4));
            */
        }
    }
}

impl Default for LinearAlgebraUnitTest {
    
    fn default() -> Self {
        todo!();
        /*


            : UnitTest ("Linear Algebra UnitTests", UnitTestCategories::dsp
        */
    }
}

impl LinearAlgebraUnitTest {

    pub fn run_test_for_all_types<TheTest>(&mut self, unit_test_name: *const u8)  {
    
        todo!();
        /*
            beginTest (unitTestName);

            TheTest::template run<float> (*this);
            TheTest::template run<double> (*this);
        */
    }
    
    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            runTestForAllTypes<AdditionTest> ("AdditionTest");
            runTestForAllTypes<DifferenceTest> ("DifferenceTest");
            runTestForAllTypes<ScalarMultiplicationTest> ("ScalarMultiplication");
            runTestForAllTypes<HadamardProductTest> ("HadamardProductTest");
            runTestForAllTypes<MultiplicationTest> ("MultiplicationTest");
            runTestForAllTypes<IdentityMatrixTest> ("IdentityMatrixTest");
            runTestForAllTypes<SolvingTest> ("SolvingTest");
        */
    }
}

lazy_static!{
    /*
    static LinearAlgebraUnitTest linearAlgebraUnitTest;
    */
}
