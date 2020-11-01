#![feature(fn_traits)]

use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub struct InputCellID(usize);

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub struct ComputeCellID(usize);

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub struct CallbackID(usize);

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub enum CellID {
    Input(InputCellID),
    Compute(ComputeCellID),
}

#[derive(Debug, PartialEq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

struct Input<T> {
    value: T,
}

impl<T> Input<T> {
    pub fn new(value: T) -> Self {
        Input { value }
    }
}

struct Compute<'a, T> {
    func: Box<dyn Fn(&[T]) -> T + 'a>,
    dependencies: Vec<CellID>,
    current_value: T,
}

impl<'a, T> Compute<'a, T> {
    pub fn new<F: Fn(&[T]) -> T + 'a>(func: F, dependencies: &[CellID], initial_value: T) -> Self {
        Compute {
            func: Box::new(func),
            dependencies: dependencies.to_vec(),
            current_value: initial_value,
        }
    }
}

struct Callback<'a, T> {
    func: Box<dyn Fn(T) + 'a>,
}

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

pub struct Reactor<'a, T> {
    inputs: HashMap<InputCellID, Input<T>>,
    computes: HashMap<ComputeCellID, Compute<'a, T>>,

    change_propagation: HashMap<CellID, Vec<ComputeCellID>>,
    compute_callbacks: HashMap<ComputeCellID, HashSet<CallbackID>>,

    callbacks: HashMap<CallbackID, Callback<'a, T>>,
}

impl<'a, T: Copy + PartialEq> Default for Reactor<'a, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T: Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Reactor {
            inputs: HashMap::new(),
            computes: HashMap::new(),
            change_propagation: HashMap::new(),
            compute_callbacks: HashMap::new(),
            callbacks: HashMap::new(),
        }
    }

    pub fn generate_id(&self) -> usize {
        NEXT_ID.fetch_add(1, Ordering::Relaxed)
    }

    pub fn create_input(&mut self, _initial: T) -> InputCellID {
        let id = InputCellID(self.generate_id());
        self.inputs.insert(id, Input::new(_initial));
        id
    }

    pub fn create_compute<F: Fn(&[T]) -> T + 'a>(
        &mut self,
        _dependencies: &[CellID],
        _compute_func: F,
    ) -> Result<ComputeCellID, CellID> {
        let id = ComputeCellID(self.generate_id());
        let mut dependency_values = Vec::with_capacity(_dependencies.len());

        for dependency in _dependencies {
            let exists = match dependency {
                CellID::Input(cell_id) => self.inputs.contains_key(cell_id),
                CellID::Compute(cell_id) => self.computes.contains_key(cell_id),
            };

            if !exists {
                return Err(*dependency);
            }

            dependency_values.push(self.value(*dependency).unwrap());

            let change_list = self
                .change_propagation
                .entry(*dependency)
                .or_insert_with(Vec::new);
            change_list.push(id);
        }

        let initial_value = _compute_func.call((dependency_values.as_slice(),));
        self.computes.insert(
            id,
            Compute::new(_compute_func, _dependencies, initial_value),
        );

        Ok(id)
    }

    pub fn value(&self, id: CellID) -> Option<T> {
        match id {
            CellID::Input(cell_id) => self.inputs.get(&cell_id).map(|input| input.value),
            CellID::Compute(cell_id) => self
                .computes
                .get(&cell_id)
                .map(|compute| compute.current_value),
        }
    }

    fn update_compute(&mut self, id: &ComputeCellID) -> Option<()> {
        let compute_cell = self.computes.get(id)?;
        let dependency_values = compute_cell
            .dependencies
            .iter()
            .map(|dependency| self.value(*dependency))
            .collect::<Option<Vec<_>>>()?;

        let new_value = compute_cell.func.call((dependency_values.as_slice(),));

        if compute_cell.current_value == new_value {
            return None;
        }

        let compute_cell_mut = self.computes.get_mut(id).unwrap();
        compute_cell_mut.current_value = new_value;

        Some(())
    }

    fn propagate_changes(&mut self, ids: Vec<ComputeCellID>) {
        //update all of the affected compute cells
        let mut current_ids = ids;
        let mut affected_ids = HashSet::new();

        while !current_ids.is_empty() {
            let mut next_ids = vec![];

            for id in current_ids {
                if self.update_compute(&id).is_some() {
                    affected_ids.insert(id);

                    if let Some(related_computes) =
                        self.change_propagation.get(&CellID::Compute(id))
                    {
                        next_ids.append(&mut related_computes.clone());
                    }
                }
            }

            current_ids = next_ids;
        }

        //then go through the compute cells and call all of the callbacks
        for affected_id in affected_ids {
            if let Some(compute_callbacks) = self.compute_callbacks.get(&affected_id) {
                let compute = self.computes.get(&affected_id).unwrap();

                for compute_callback in compute_callbacks {
                    let callback = self.callbacks.get(compute_callback).unwrap();
                    callback.func.call((compute.current_value,));
                }
            }
        }
    }

    pub fn set_value(&mut self, _id: InputCellID, _new_value: T) -> bool {
        let cell_option = self.inputs.get_mut(&_id);

        if cell_option.is_none() {
            return false;
        }

        let mut cell = cell_option.unwrap();
        cell.value = _new_value;

        if let Some(cell_ids_to_update) = self.change_propagation.get(&CellID::Input(_id)) {
            let cell_ids_vec = cell_ids_to_update.to_vec();
            self.propagate_changes(cell_ids_vec);
        }

        true
    }

    pub fn add_callback<F: Fn(T) + 'a>(
        &mut self,
        _id: ComputeCellID,
        _callback: F,
    ) -> Option<CallbackID> {
        if !self.computes.contains_key(&_id) {
            return None;
        }

        let callback_id = CallbackID(self.generate_id());

        self.callbacks.insert(
            callback_id,
            Callback {
                func: Box::new(_callback),
            },
        );

        let compute_callbacks = self
            .compute_callbacks
            .entry(_id)
            .or_insert_with(HashSet::new);
        compute_callbacks.insert(callback_id);

        Some(callback_id)
    }

    pub fn remove_callback(
        &mut self,
        cell: ComputeCellID,
        callback: CallbackID,
    ) -> Result<(), RemoveCallbackError> {
        if self.callbacks.remove(&callback).is_none() {
            return Err(RemoveCallbackError::NonexistentCallback);
        }

        if self
            .compute_callbacks
            .get_mut(&cell)
            .map(|callbacks| callbacks.remove(&callback))
            .is_none()
        {
            return Err(RemoveCallbackError::NonexistentCell);
        }

        Ok(())
    }
}
