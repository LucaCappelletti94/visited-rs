use std::{intrinsics::unlikely, ops::AddAssign};

use num_traits::{bounds::UpperBounded, AsPrimitive, One, Zero};

#[derive(Clone, Debug)]
pub struct Visited<T> {
    visited: Vec<T>,
    visited_flag: T,
}

impl<T> Visited<T>
where
    T: Zero + One + Clone + PartialOrd + UpperBounded + AddAssign,
{
    #[inline(always)]
    /// Creates new zeroed visited struct with given capacity.
    pub fn zero(capacity: usize) -> Self {
        Self {
            visited: vec![T::zero(); capacity],
            visited_flag: T::one(),
        }
    }

    #[inline(always)]
    /// Returns whether the value at given index was already visited.
    pub fn is_visited<U>(&self, index: U) -> bool
    where
        U: AsPrimitive<usize>,
    {
        self.visited[index.as_()] == self.visited_flag
    }

    #[inline(always)]
    /// Sets the value at provided index as visited.
    pub fn set_visited<U>(&mut self, index: U)
    where
        U: AsPrimitive<usize>,
    {
        self.visited[index.as_()] = self.visited_flag.clone();
    }

    #[inline(always)]
    /// Sets the value at provided index as visited and returns the previous value.
    pub fn set_and_get_visited<U>(&mut self, index: U) -> bool
    where
        U: AsPrimitive<usize>,
    {
        let value = &mut self.visited[index.as_()];
        let original = value.clone();
        *value = self.visited_flag.clone();
        original == self.visited_flag
    }

    #[inline(always)]
    /// Sets data-racingly the value at provided index as visited.
    ///
    /// # Safety
    /// This method allows mutable access to the data-structure
    /// that may be allowed in parallel by multiple threads.
    /// Use this method carefully, being aware that there might
    /// be collisions from the different threads emploing this
    /// method.
    pub unsafe fn set_visited_racing<U>(&self, index: U)
    where
        U: AsPrimitive<usize>,
    {
        (&mut *(self.visited.as_ptr() as *mut Vec<T>))[index.as_()] = self.visited_flag.clone();
    }
    
    #[inline(always)]
    /// Sets data-racingly the value at provided index as visited  and returns the previous value.
    ///
    /// # Safety
    /// This method allows mutable access to the data-structure
    /// that may be allowed in parallel by multiple threads.
    /// Use this method carefully, being aware that there might
    /// be collisions from the different threads emploing this
    /// method.
    pub unsafe fn set_and_get_visited_racing<U>(&self, index: U) -> bool
    where
        U: AsPrimitive<usize>,
    {
        let value = &mut (&mut *(self.visited.as_ptr() as *mut Vec<T>))[index.as_()];
        let original = value.clone();
        *value = self.visited_flag.clone();
        original == self.visited_flag
    }

    #[inline(always)]
    /// Clears all visited values.
    ///
    /// # Implementative details
    /// Up until the visited flag does not rich the maximal value
    /// that can be represented with the current type, we can surely
    /// get away with simply bumping the flag by one. The problems
    /// begin when the flag reaches the maximal value, and we need
    /// to actually clear the vector. This is necessary as in some
    /// cases we may have not visited some elments in none of the
    /// iterations and we may erroneously set these elements as
    /// `visited` while they where never visited.
    pub fn clear(&mut self) {
        if unlikely(self.visited_flag == T::max_value()) {
            self.visited_flag = T::one();
            self.visited.iter_mut().for_each(|v| {
                *v = T::zero();
            });
        } else {
            self.visited_flag += T::one();
        }
    }
}
