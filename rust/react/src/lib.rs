use std::collections::{HashMap, HashSet, VecDeque};

/// `InputCellID` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InputCellID(usize);

/// `ComputeCellID` is a unique identifier for a compute cell.
/// Values of type `InputCellID` and `ComputeCellID` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellID = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellID = r.create_compute(&[react::CellID::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComputeCellID(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CallbackID(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CellID {
    Input(InputCellID),
    Compute(ComputeCellID),
}

#[derive(Debug, PartialEq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

struct Compute<'a, T> {
    callbacks: HashSet<CallbackID>,
    dependencies: Vec<CellID>,
    compute_func: Box<dyn Fn(&[T]) -> T + 'a>,
    cached: T,
}

struct Callback<'a, T> {
    compute: ComputeCellID,
    callback: Box<dyn FnMut(T) -> () + 'a>,
}

#[derive(Debug)]
struct WithDependents<T> {
    dependents: HashSet<ComputeCellID>,
    t: T,
}

pub struct Reactor<'a, T> {
    inputs: HashMap<InputCellID, WithDependents<T>>,
    computes: HashMap<ComputeCellID, WithDependents<Compute<'a, T>>>,
    callbacks: HashMap<CallbackID, Callback<'a, T>>,
    next_id: usize,

}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq + std::fmt::Debug> Reactor<'a, T> {
    pub fn new() -> Self {
        Self {
            inputs: HashMap::new(),
            computes: HashMap::new(),
            callbacks: HashMap::new(),
            next_id: 0,
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellID {
        let id = InputCellID(self.next_id);
        let new_dependency = WithDependents {
            t: initial,
            dependents: HashSet::new(),
        };
        self.next_id += 1;
        self.inputs.insert(id, new_dependency);
        id
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: Fn(&[T]) -> T + 'a>(
        &mut self,
        dependencies: &[CellID],
        compute_func: F,
    ) -> Result<ComputeCellID, CellID> {
        let mut inputs = Vec::with_capacity(dependencies.len());

        for dep in dependencies {
            match self.value(*dep) {
                None => return Err(*dep),
                Some(t) => inputs.push(t.clone()),
            };
        }

        let id = ComputeCellID(self.next_id);
        self.next_id += 1;

        dependencies.iter()
            .for_each(|&dep| {
                match dep {
                    CellID::Input(input_id) => {
                        self.inputs.get_mut(&input_id)
                            .unwrap()
                            .dependents
                            .insert(id);
                    }
                    CellID::Compute(compute_id) => {
                        self.computes.get_mut(&compute_id)
                            .unwrap()
                            .dependents
                            .insert(id);
                    }
                }
            });

        let cached = compute_func(&inputs);

        let initial = Compute {
            callbacks: HashSet::new(),
            dependencies: dependencies.to_vec(),
            compute_func: Box::new(compute_func),
            cached,
        };
        let new_dependent = WithDependents {
            t: initial,
            dependents: HashSet::new(),
        };
        self.computes.insert(id, new_dependent);
        Ok(id)
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellID) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellID) -> Option<T> {
        match id {
            CellID::Input(id) => self.inputs.get(&id)
                .map(|i| i.t.clone()),
            CellID::Compute(id) => self.computes.get(&id)
                .map(|c| c.t.cached),
        }
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellID) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: InputCellID, new_value: T) -> bool {
        let mut todo: VecDeque<ComputeCellID> = VecDeque::new();

        match self.inputs.get_mut(&id) {
            None => return false,
            Some(x) => {
                if x.t != new_value {
                    x.t = new_value;
                    todo = x.dependents.iter()
                        .copied()
                        .collect();
                }
            }
        }

        let mut callbacks: HashMap<CallbackID, T> = HashMap::new();

        // React
        while let Some(compute_id) = todo.pop_front() {
            let compute = self.computes.get(&compute_id)
                .unwrap();
            todo.append(&mut compute.dependents.iter()
                .copied()
                .collect());

            let inputs: Vec<T> = compute.t.dependencies.iter()
                .map(|&i| self.value(i)
                    .unwrap())
                .collect();

            let new_value = (compute.t.compute_func)(&inputs[..]);

            // Annoying I have to look it up again with mutability
            let mut compute = self.computes.get_mut(&compute_id)
                .unwrap();

            if compute.t.cached != new_value {
                compute.t.callbacks.iter()
                    .for_each(|&c| {
                        callbacks.insert(c, compute.t.cached);
                    });
                compute.t.cached = new_value;
            }
        }

        callbacks.iter()
            .for_each(|(callback_id, &previous)| {
                let c = self.callbacks.get(&callback_id)
                    .unwrap();
                let x = self.value(CellID::Compute(c.compute))
                    .unwrap();

                if x != previous {
                    dbg!(callback_id, x, previous);
                    (self.callbacks.get_mut(&callback_id)
                        .unwrap().callback)(x);
                }
            });
        true
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: FnMut(T) -> () + 'a>(
        &mut self,
        id: ComputeCellID,
        callback: F,
    ) -> Option<CallbackID> {
        match self.computes.get_mut(&id) {
            None => None,
            Some(compute) => {
                let callback_id = CallbackID(self.next_id);
                self.next_id += 1;
                self.callbacks.insert(
                    callback_id,
                    Callback {
                        compute: id,
                        callback: Box::new(callback),
                    },
                );
                compute.t.callbacks.insert(callback_id);
                Some(callback_id)
            }
        }
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellID,
        callback: CallbackID,
    ) -> Result<(), RemoveCallbackError> {
        match (self.callbacks.remove(&callback), self.computes.get_mut(&cell)) {
            (None, _) => Err(RemoveCallbackError::NonexistentCallback),
            (_, None) => Err(RemoveCallbackError::NonexistentCell),
            (_, Some(c)) => {
                c.t.callbacks.remove(&callback);
                Ok(())
            }
        }
    }
}
