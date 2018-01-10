use std::any::Any;
use std::cmp::Ordering;
use errors;
use executor::{ExecutorImpl};
use object_pool::ObjectPool;

pub trait Object {
    fn finalize(&self, _pool: &mut ObjectPool) {}

    // before allocating on the object pool...
    fn initialize(&mut self, _pool: &mut ObjectPool) {}

    fn call(&self, _executor: &mut ExecutorImpl) -> usize {
        panic!(errors::VMError::from(errors::RuntimeError::new("Not callable")));
    }
    fn get_field(&self, _name: &str) -> Option<usize> {
        None
    }
    fn set_field(&self, _name: &str, _value_ref: usize) {
        panic!(errors::VMError::from(errors::RuntimeError::new("Cannot set field")));
    }
    fn must_get_field(&self, name: &str) -> usize {
        match self.get_field(name) {
            Some(v) => v,
            None => panic!(errors::VMError::from(errors::FieldNotFoundError::from_field_name(name)))
        }
    }
    fn compare(&self, _other: &Object) -> Option<Ordering> {
        None
    }
    fn test_eq(&self, _other: &Object) -> bool {
        false
    }
    fn typename(&self) -> &str {
        "object"
    }
    fn to_i64(&self) -> i64 {
        panic!(errors::VMError::from(errors::RuntimeError::new("Cannot cast to i64")));
    }
    fn to_f64(&self) -> f64 {
        panic!(errors::VMError::from(errors::RuntimeError::new("Cannot cast to f64")));
    }
    fn to_str(&self) -> &str {
        panic!(errors::VMError::from(errors::RuntimeError::new("Cannot cast to str")));
    }
    fn to_string(&self) -> String {
        self.to_str().to_string()
    }
    fn to_bool(&self) -> bool {
        panic!(errors::VMError::from(errors::RuntimeError::new("Cannot cast to bool")));
    }
    fn get_children(&self) -> Vec<usize>;
    fn as_any(&self) -> &Any;
    fn as_any_mut(&mut self) -> &mut Any;
}
