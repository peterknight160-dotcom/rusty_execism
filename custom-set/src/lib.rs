//use std::clone;

#[derive(Debug, PartialEq, Eq)]
pub struct CustomSet<T> {
    // We fake using T here, so the compiler does not complain that
    // "parameter `T` is never used". Delete when no longer needed.
    //phantom: std::marker::PhantomData<T>,
    set: Vec<T>
}

impl<T: Clone + std::cmp::PartialEq + std::cmp::Ord> CustomSet<T> {
    pub fn new(_input: &[T]) -> Self {
        let mut set = Vec::new();
       for element in _input {
            if !set.contains(element) {
                set.push(element.clone());
            }
        }
        set.sort();
        CustomSet { set }
    }

    pub fn contains(&self, _element: &T) -> bool {
        self.set.contains(_element)
    }

    pub fn add(&mut self, _element: T) {

        // Insert the element in the correct position to maintain sorted order
        if !self.contains(&_element) {
            self.set.push(_element);
        }   
        self.set.sort();
    }

    pub fn is_subset(&self, _other: &Self) -> bool {
        // Check if every element in self is also in other
        self.set.iter().all(|element| _other.contains(element))
    }

    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }

    pub fn is_disjoint(&self, _other: &Self) -> bool {
        self.set.iter().all(|element| !_other.contains(element))
    }

    #[must_use]
    pub fn intersection(&self, _other: &Self) -> Self {
            let intersection_set: Vec<T> = self.set.iter()
                .filter(|element| _other.contains(element))
                .cloned()
                .collect();
            CustomSet { set: intersection_set }
    }

    #[must_use]
    pub fn difference(&self, _other: &Self) -> Self {
        let difference_set: Vec<T> = self.set.iter()
            .filter(|element| !_other.contains(element))
            .cloned()
            .collect();
        CustomSet { set: difference_set }
    }

    #[must_use]
    pub fn union(&self, _other: &Self) -> Self {
        let mut union_set: Vec<T> = self.set.clone();
        for element in &_other.set {
            if !union_set.contains(element) {
                union_set.push(element.clone());
            }
        }
        union_set.sort();
        CustomSet { set: union_set }
    }
}
