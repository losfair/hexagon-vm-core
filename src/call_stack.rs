use std::any::Any;
use std::cell::RefCell;
use smallvec::SmallVec;
use object::Object;
use errors;

pub struct CallStack {
    frames: Vec<Frame>
}

pub struct Frame {
    pub(crate) arguments: RefCell<SmallVec<[usize; 4]>>,
    pub(crate) locals: RefCell<SmallVec<[usize; 16]>>,
    pub(crate) exec_stack: RefCell<SmallVec<[usize; 16]>>
}

impl CallStack {
    pub fn new() -> CallStack {
        CallStack {
            frames: Vec::new()
        }
    }

    pub fn push(&mut self, frame: Frame) {
        self.frames.push(frame);
    }

    pub fn pop(&mut self) -> Frame {
        self.frames.pop().unwrap()
    }

    pub fn top(&self) -> &Frame {
        &self.frames[self.frames.len() - 1]
    }
}

impl Frame {
    pub fn with_arguments(args: Vec<usize>) -> Frame {
        Frame {
            arguments: RefCell::new(args.into()),
            locals: RefCell::new(SmallVec::new()),
            exec_stack: RefCell::new(SmallVec::new())
        }
    }
    pub fn push_exec(&self, id: usize) {
        self.exec_stack.borrow_mut().push(id);
    }

    pub fn pop_exec(&self) -> usize {
        match self.exec_stack.borrow_mut().pop() {
            Some(v) => v,
            None => panic!(errors::VMError::from(errors::RuntimeError::new("Execution stack corrupted")))
        }
    }

    pub fn reset_locals(&self, n_slots: usize) {
        let mut locals = self.locals.borrow_mut();
        locals.clear();
        for _ in 0..n_slots {
            locals.push(0);
        }
    }

    pub fn get_local(&self, ind: usize) -> usize {
        let locals = self.locals.borrow();
        if ind >= locals.len() {
            panic!(errors::VMError::from(errors::RuntimeError::new("Index out of bound")));
        }

        (*locals)[ind]
    }

    pub fn set_local(&self, ind: usize, obj_id: usize) {
        let mut locals = self.locals.borrow_mut();
        if ind >= locals.len() {
            panic!(errors::VMError::from(errors::RuntimeError::new("Index out of bound")));
        }

        (*locals)[ind] = obj_id;
    }
}
