#[derive(Debug)]
pub struct CustomSet<T> {
    items: Vec<T>,
}

impl<T: Clone + PartialEq> CustomSet<T> {
    pub fn new(_input: &[T]) -> Self {
        CustomSet {
            items: _input.to_vec(),
        }
    }

    pub fn contains(&self, _element: &T) -> bool {
        self.items.contains(_element)
    }

    pub fn add(&mut self, _element: T) {
        if !self.items.contains(&_element) {
            self.items.push(_element);
        }
    }

    pub fn is_subset(&self, _other: &Self) -> bool {
        self.items.iter().all(|item| _other.items.contains(item))
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn is_disjoint(&self, _other: &Self) -> bool {
        self.items.iter().all(|item| !_other.items.contains(item))
    }

    pub fn intersection(&self, _other: &Self) -> Self {
        CustomSet {
            items: _other
                .items
                .iter()
                .filter(|item| self.items.contains(item))
                .cloned()
                .collect::<Vec<_>>(),
        }
    }

    pub fn difference(&self, _other: &Self) -> Self {
        CustomSet {
            items: self
                .items
                .iter()
                .filter(|item| !_other.items.contains(item))
                .cloned()
                .collect::<Vec<_>>(),
        }
    }

    pub fn union(&self, _other: &Self) -> Self {
        let mut new_set = self.difference(_other);
        new_set.items.extend(_other.items.clone());
        new_set
    }
}

impl<T: Clone + PartialEq> PartialEq for CustomSet<T> {
    fn eq(&self, other: &Self) -> bool {
        self.items.len() == other.items.len()
            && self.items.iter().all(|item| other.items.contains(item))
    }
}
