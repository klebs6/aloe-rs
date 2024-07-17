crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/maths/aloe_Matrix.h]

/**
  | General matrix and vectors class, meant for
  | classic math manipulation such as additions,
  | multiplications, and linear systems of
  | equations solving.
  |
  | @see LinearAlgebra
  |
  | @tags{DSP}
  */
#[leak_detector]
pub struct Matrix<ElementType> {
    data:              Vec<ElementType>,
    data_acceleration: Vec<usize>,
    rows:              usize,
    columns:           usize,
}

impl<ElementType> AddAssign<&Matrix<ElementType>> for Matrix<ElementType> {
    
    /**
      | Addition of two matrices
      |
      */
    fn add_assign(&mut self, other: &Matrix<ElementType>) {
        todo!();
        /*
            return apply (other, [] (ElementType a, ElementType b) { return a + b; } );
        */
    }
}

impl<ElementType> SubAssign<&Matrix<ElementType>> for Matrix<ElementType> {

    /**
      | Subtraction of two matrices
      |
      */
    fn sub_assign(&mut self, other: &Matrix<ElementType>) {
        todo!();
        /*
            return apply (other, [] (ElementType a, ElementType b) { return a - b; } );
        */
    }
}

impl<ElementType> MulAssign<ElementType> for Matrix<ElementType> {
    
    /**
      | Scalar multiplication
      |
      */
    fn mul_assign(&mut self, scalar: ElementType) {
        todo!();
        /*
            std::for_each (begin(), end(), [scalar] (ElementType& x) { x *= scalar; });
            return *this;
        */
    }
}

impl<ElementType> Add<&Matrix<ElementType>> for Matrix<ElementType> {

    type Output = Self;
    
    /**
      | Addition of two matrices
      |
      */
    fn add(self, other: &Matrix<ElementType>) -> Self::Output {
        todo!();
        /*
            Matrix result (*this); result += other;  return result;
        */
    }
}

impl<ElementType> Sub<&Matrix<ElementType>> for Matrix<ElementType> {

    type Output = Matrix<ElementType>;
    
    /**
      | Addition of two matrices
      |
      */
    fn sub(self, other: &Matrix<ElementType>) -> Self::Output {
        todo!();
        /*
            Matrix result (*this); result -= other;  return result;
        */
    }
}

impl<ElementType> Mul<&ElementType> for Matrix<ElementType> {
    type Output = Matrix<ElementType>;

    /**
      | Scalar multiplication
      |
      */
    fn mul(self, other: &ElementType) -> Self::Output {
        todo!();
        /*
            Matrix result (*this); result *= scalar; return result;
        */
    }
}

impl<ElementType> PartialEq<Matrix<ElementType>> for Matrix<ElementType> {
    
    /**
      | Comparison operator
      |
      */
    fn eq(&self, other: &Matrix<ElementType>) -> bool {
        todo!();
        /*
            return compare (*this, other);
        */
    }
}

impl<ElementType> Eq for Matrix<ElementType> {}

impl<ElementType> Mul<&Matrix<ElementType>> for Matrix<ElementType> {

    type Output = Matrix<ElementType>;

    /**
      | Matrix multiplication
      |
      */
    #[inline] fn mul(self, other: &Matrix<ElementType>) -> Self::Output {
        todo!();
        /*
            auto n = getNumRows(), m = other.getNumColumns(), p = getNumColumns();
        Matrix result (n, m);

        jassert (p == other.getNumRows());

        size_t offsetMat = 0, offsetlhs = 0;

        auto* dst = result.getRawDataPointer();
        auto* a = getRawDataPointer();
        auto* b = other.getRawDataPointer();

        for (size_t i = 0; i < n; ++i)
        {
            size_t offsetrhs = 0;

            for (size_t k = 0; k < p; ++k)
            {
                auto ak = a[offsetlhs++];

                for (size_t j = 0; j < m; ++j)
                    dst[offsetMat + j] += ak * b[offsetrhs + j];

                offsetrhs += m;
            }

            offsetMat += m;
        }

        return result;
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/maths/aloe_Matrix.cpp]
impl<ElementType> Matrix<ElementType> {

    /**
      | Creates a new matrix with a given number
      | of rows and columns.
      |
      */
    pub fn new(
        num_rows:    usize,
        num_columns: usize) -> Self {
    
        todo!();
        /*
        : rows(numRows),
        : columns(numColumns),

            resize();
            clear();
        */
    }

    /**
      | Creates a new matrix with a given number
      | of rows and columns, with initial data
      | coming from an array, stored in row-major
      | order.
      |
      */
    pub fn new_with_initial_data(
        num_rows:     usize,
        num_columns:  usize,
        data_pointer: *const ElementType) -> Self {
    
        todo!();
        /*
        : rows(numRows),
        : columns(numColumns),

            resize();
            memcpy (data.getRawDataPointer(), dataPointer, rows * columns * sizeof (ElementType));
        */
    }

    /**
      | Returns the number of rows in the matrix.
      |
      */
    pub fn get_num_rows(&self) -> usize {
        
        todo!();
        /*
            return rows;
        */
    }

    /**
      | Returns the number of columns in the
      | matrix.
      |
      */
    pub fn get_num_columns(&self) -> usize {
        
        todo!();
        /*
            return columns;
        */
    }

    /**
      | Returns an Array of 2 integers with the
      | number of rows and columns in the matrix.
      |
      */
    pub fn get_size(&self) -> Vec<usize> {
        
        todo!();
        /*
            return { rows, columns };
        */
    }

    /**
      | Fills the contents of the matrix with
      | zeroes.
      |
      */
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            zeromem (data.begin(), (size_t) data.size() * sizeof (ElementType));
        */
    }

    /**
      | Returns the value of the matrix at a given
      | row and column (for reading).
      |
      */
    #[inline] pub fn invoke(
        &self,
        row:    usize,
        column: usize) -> ElementType {
        
        todo!();
        /*
            jassert (row < rows && column < columns);
            return data.getReference (static_cast<int> (dataAcceleration.getReference (static_cast<int> (row))) + static_cast<int> (column));
        */
    }

    /**
      | Returns the value of the matrix at a given
      | row and column (for modifying).
      |
      */
    #[inline] pub fn invoke_mut(
        &mut self, 
        row:    usize,
        column: usize) -> &mut ElementType {
        
        todo!();
        /*
            jassert (row < rows && column < columns);
            return data.getReference (static_cast<int> (dataAcceleration.getReference (static_cast<int> (row))) + static_cast<int> (column));
        */
    }

    /**
      | Returns a pointer to the raw data of the
      | matrix object, ordered in row-major
      | order (for modifying).
      |
      */
    #[inline] pub fn get_raw_data_pointer_mut(&mut self) -> *mut ElementType {
        
        todo!();
        /*
            return data.getRawDataPointer();
        */
    }

    /**
      | Returns a pointer to the raw data of the
      | matrix object, ordered in row-major
      | order (for reading).
      |
      */
    #[inline] pub fn get_raw_data_pointer(&self) -> *const ElementType {
        
        todo!();
        /*
            return data.begin();
        */
    }

    /**
      | Does a hadarmard product with the receiver
      | and other and stores the result in the
      | receiver
      |
      */
    #[inline] pub fn hadarmard_with_receiver(&mut self, other: &Matrix<ElementType>) -> &mut Matrix<ElementType> {
        
        todo!();
        /*
            return apply (other, [] (ElementType a, ElementType b) { return a * b; } );
        */
    }

    /**
      | Does a hadarmard product with a and b
      | returns the result.
      |
      */
    pub fn hadarmard(
        a: &Matrix<ElementType>,
        b: &Matrix<ElementType>) -> Matrix<ElementType> {
        
        todo!();
        /*
            Matrix result (a); result.hadarmard (b); return result;
        */
    }
    
    /**
      | Tells if the matrix is a square matrix
      |
      */
    pub fn is_square(&self) -> bool {
        
        todo!();
        /*
            return rows == columns;
        */
    }

    /**
      | Tells if the matrix is a vector
      |
      */
    pub fn is_vector(&self) -> bool {
        
        todo!();
        /*
            return isOneColumnVector() || isOneRowVector();
        */
    }

    /**
      | Tells if the matrix is a one column vector
      |
      */
    pub fn is_one_column_vector(&self) -> bool {
        
        todo!();
        /*
            return columns == 1;
        */
    }

    /**
      | Tells if the matrix is a one row vector
      |
      */
    pub fn is_one_row_vector(&self) -> bool {
        
        todo!();
        /*
            return rows == 1;
        */
    }

    /**
      | Tells if the matrix is a null matrix
      |
      */
    pub fn is_null_matrix(&self) -> bool {
        
        todo!();
        /*
            return rows == 0 || columns == 0;
        */
    }

    pub fn begin_mut(&mut self) -> *mut ElementType {
        
        todo!();
        /*
            return data.begin();
        */
    }


    pub fn end_mut(&mut self) -> *mut ElementType {
        
        todo!();
        /*
            return data.end();
        */
    }


    pub fn begin(&self) -> *const ElementType {
        
        todo!();
        /*
            return &data.getReference (0);
        */
    }


    pub fn end(&self) -> *const ElementType {
        
        todo!();
        /*
            return begin() + data.size();
        */
    }
    
    /**
      | Resizes the matrix.
      |
      */
    pub fn resize(&mut self)  {
        
        todo!();
        /*
            data.resize (static_cast<int> (columns * rows));
            dataAcceleration.resize (static_cast<int> (rows));

            for (size_t i = 0; i < rows; ++i)
                dataAcceleration.setUnchecked (static_cast<int> (i), i * columns);
        */
    }
    
    pub fn apply<BinaryOperation>(&mut self, 
        other:     &Matrix<ElementType>,
        binary_op: BinaryOperation) -> &mut Matrix<ElementType> {
    
        todo!();
        /*
            jassert (rows == other.rows && columns == other.columns);

            auto* dst = getRawDataPointer();

            for (auto src : other)
            {
                *dst = binaryOp (*dst, src);
                ++dst;
            }

            return *this;
        */
    }
    
    /**
      | Creates the identity matrix
      |
      */
    pub fn identity(&mut self, size: usize) -> Matrix<ElementType> {
        
        todo!();
        /*
            Matrix result (size, size);

        for (size_t i = 0; i < size; ++i)
            result(i, i) = 1;

        return result;
        */
    }
    
    /**
      | Creates a Toeplitz Matrix from a vector
      | with a given squared size
      |
      */
    pub fn toeplitz(&mut self, 
        vector: &Matrix<ElementType>,
        size:   usize) -> Matrix<ElementType> {
        
        todo!();
        /*
            jassert (vector.isOneColumnVector());
        jassert (size <= vector.rows);

        Matrix result (size, size);

        for (size_t i = 0; i < size; ++i)
            result (i, i) = vector (0, 0);

        for (size_t i = 1; i < size; ++i)
            for (size_t j = i; j < size; ++j)
                result (j, j - i) = result (j - i, j) = vector (i, 0);

        return result;
        */
    }
    
    /**
      | Creates a squared size x size Hankel
      | Matrix from a vector with an optional
      | offset.
      | 
      | -----------
      | @param vector
      | 
      | The vector from which the Hankel matrix
      | should be generated. Its number of rows
      | should be at least 2 * (size - 1) + 1
      | ----------
      | @param size
      | 
      | The size of resulting square matrix.
      | ----------
      | @param offset
      | 
      | An optional offset into the given vector.
      |
      */
    pub fn hankel(
        &mut self, 
        vector: &Matrix<ElementType>,
        size:   usize,
        offset: Option<usize>) -> Matrix<ElementType> {

        let offset: usize = offset.unwrap_or(0);
        
        todo!();
        /*
            jassert(vector.isOneColumnVector());
        jassert(vector.rows >= (2 * (size - 1) + 1));

        Matrix result (size, size);

        for (size_t i = 0; i < size; ++i)
            result (i, i) = vector ((2 * i) + offset, 0);

        for (size_t i = 1; i < size; ++i)
            for (size_t j = i; j < size; ++j)
                result (j, j - i) = result (j - i, j) = vector (i + 2 * (j - i) + offset, 0);

        return result;
        */
    }
    
    /**
      | Swaps the contents of two columns in
      | the matrix and returns a reference to
      | itself.
      |
      */
    pub fn swap_columns(&mut self, 
        column_one: usize,
        column_two: usize) -> &mut Matrix<ElementType> {
        
        todo!();
        /*
            jassert (columnOne < columns && columnTwo < columns);

        auto* p = data.getRawDataPointer();

        for (size_t i = 0; i < rows; ++i)
        {
            auto offset = dataAcceleration.getUnchecked (static_cast<int> (i));
            std::swap (p[offset + columnOne], p[offset + columnTwo]);
        }

        return *this;
        */
    }
    
    /**
      | Swaps the contents of two rows in the
      | matrix and returns a reference to itself.
      |
      */
    pub fn swap_rows(&mut self, 
        row_one: usize,
        row_two: usize) -> &mut Matrix<ElementType> {
        
        todo!();
        /*
            jassert (rowOne < rows && rowTwo < rows);

        auto offset1 = rowOne * columns;
        auto offset2 = rowTwo * columns;

        auto* p = data.getRawDataPointer();

        for (size_t i = 0; i < columns; ++i)
            std::swap (p[offset1 + i], p[offset2 + i]);

        return *this;
        */
    }
    
    
    /**
      | Solves a linear system of equations
      | represented by this object and the argument
      | b, using various algorithms depending
      | on the size of the arguments.
      | 
      | The matrix must be a square matrix N times
      | N, and b must be a vector N times 1, with
      | the coefficients of b. After the execution
      | of the algorithm, the vector b will contain
      | the solution.
      | 
      | Returns true if the linear system of
      | equations was successfully solved.
      |
      */
    pub fn solve(&self, b: &mut Matrix<ElementType>) -> bool {
        
        todo!();
        /*
            auto n = columns;
        jassert (n == n && n == b.rows && b.isOneColumnVector());

        auto* x = b.getRawDataPointer();
        const auto& A = *this;

        switch (n)
        {
            case 1:
            {
                auto denominator = A (0,0);

                if (denominator == 0)
                    return false;

                b (0, 0) /= denominator;
            }
            break;

            case 2:
            {
                auto denominator = A (0, 0) * A (1, 1) - A (0, 1) * A (1, 0);

                if (denominator == 0)
                    return false;

                auto factor = (1 / denominator);
                auto b0 = x[0], b1 = x[1];

                x[0] = factor * (A (1, 1) * b0 - A (0, 1) * b1);
                x[1] = factor * (A (0, 0) * b1 - A (1, 0) * b0);
            }
            break;

            case 3:
            {
                auto denominator = A (0, 0) * (A (1, 1) * A (2, 2) - A (1, 2) * A (2, 1))
                                 + A (0, 1) * (A (1, 2) * A (2, 0) - A (1, 0) * A (2, 2))
                                 + A (0, 2) * (A (1, 0) * A (2, 1) - A (1, 1) * A (2, 0));

                if (denominator == 0)
                    return false;

                auto factor = 1 / denominator;
                auto b0 = x[0], b1 = x[1], b2 = x[2];

                x[0] =  ( ( A (0, 1) * A (1, 2) - A (0, 2) * A (1, 1)) * b2
                        + (-A (0, 1) * A (2, 2) + A (0, 2) * A (2, 1)) * b1
                        + ( A (1, 1) * A (2, 2) - A (1, 2) * A (2, 1)) * b0) * factor;

                x[1] = -( ( A (0, 0) * A (1, 2) - A (0, 2) * A (1, 0)) * b2
                        + (-A (0, 0) * A (2, 2) + A (0, 2) * A (2, 0)) * b1
                        + ( A (1, 0) * A (2, 2) - A (1, 2) * A (2, 0)) * b0) * factor;

                x[2] =  ( ( A (0, 0) * A (1, 1) - A (0, 1) * A (1, 0)) * b2
                        + (-A (0, 0) * A (2, 1) + A (0, 1) * A (2, 0)) * b1
                        + ( A (1, 0) * A (2, 1) - A (1, 1) * A (2, 0)) * b0) * factor;
            }
            break;

            default:
            {
                Matrix<ElementType> M (A);

                for (size_t j = 0; j < n; ++j)
                {
                    if (M (j, j) == 0)
                    {
                        auto i = j;
                        while (i < n && M (i, j) == 0)
                            ++i;

                        if (i == n)
                            return false;

                        for (size_t k = 0; k < n; ++k)
                            M (j, k) += M (i, k);

                        x[j] += x[i];
                    }

                    auto t = 1 / M (j, j);

                    for (size_t k = 0; k < n; ++k)
                        M (j, k) *= t;

                    x[j] *= t;

                    for (size_t k = j + 1; k < n; ++k)
                    {
                        auto u = -M (k, j);

                        for (size_t l = 0; l < n; ++l)
                            M (k, l) += u * M (j, l);

                        x[k] += u * x[j];
                    }
                }

                for (int k = static_cast<int> (n) - 2; k >= 0; --k)
                    for (size_t i = static_cast<size_t> (k) + 1; i < n; ++i)
                        x[k] -= M (static_cast<size_t> (k), i) * x[i];
            }
        }

        return true;
        */
    }
    
    /**
      | Returns a String displaying in a convenient
      | way the matrix contents.
      |
      */
    pub fn to_string(&self) -> String {
        
        todo!();
        /*
            StringArray entries;
        int sizeMax = 0;

        auto* p = data.begin();

        for (size_t i = 0; i < rows; ++i)
        {
            for (size_t j = 0; j < columns; ++j)
            {
                String entry (*p++, 4);
                sizeMax = jmax (sizeMax, entry.length());

                entries.add (entry);
            }
        }

        sizeMax = ((sizeMax + 1) / 4 + 1) * 4;

        MemoryOutputStream result;

        auto n = static_cast<size_t> (entries.size());

        for (size_t i = 0; i < n; ++i)
        {
            result << entries[(int) i].paddedRight (' ', sizeMax);

            if (i % columns == (columns - 1))
                result << newLine;
        }

        return result.toString();
        */
    }
}

impl<ElementType: Zero> Matrix<ElementType> {

    /**
      | Compare to matrices with a given tolerance
      |
      */
    pub fn compare(
        &mut self, 
        a:         &Matrix<ElementType>,
        b:         &Matrix<ElementType>,
        tolerance: Option<ElementType>) -> bool {

        let tolerance: ElementType = tolerance.unwrap_or_else(ElementType::zero);
        
        todo!();
        /*
            if (a.rows != b.rows || a.columns != b.columns)
            return false;

        tolerance = std::abs (tolerance);

        auto* bPtr = b.begin();
        for (auto aValue : a)
            if (std::abs (aValue - *bPtr++) > tolerance)
                return false;

        return true;
        */
    }
}
