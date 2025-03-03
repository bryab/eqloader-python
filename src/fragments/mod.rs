use pyo3::exceptions::{PyTypeError, PyValueError};

mod actordef;
mod actorinst;
mod material;
mod mesh;
pub use actordef::*;
pub use actorinst::*;
use libeq_wld::parser::{Fragment, WldDoc};
pub use material::*;
pub use mesh::*;
use owning_ref::ArcRef;
use std::collections::HashMap;
use std::sync::Arc;

use pyo3::PyResult;
/// Create a reference to a particular fragment by pairing it with its parent WLD in an OwnedRef.
fn create_fragment_ref<T: 'static + Fragment>(
    wld: Arc<WldDoc>,
    index: u32,
) -> PyResult<ArcRef<WldDoc, T>> {
    ArcRef::new(wld).try_map(|wld| {
        let fragment_type = wld
            .at(index as usize - 1)
            .ok_or_else(|| PyValueError::new_err(format!("Not a fragment reference: {index}")))?;

        fragment_type.as_any().downcast_ref::<T>().ok_or_else(|| {
            PyTypeError::new_err(format!("Fragment is not of the requested type: {index}"))
        })
    })
}

pub trait S3DFragment {
    fn new(wld: &Arc<WldDoc>, index: u32) -> PyResult<Self>
    where
        Self: Sized;
}

#[cfg(feature = "serde")]
type SerdeResult = PyResult<HashMap<String, String>>;

#[cfg(feature = "serde")]
fn frag_to_dict<T: 'static + Fragment + serde::ser::Serialize>(
    wld: &WldDoc,
    fragment: &T,
) -> SerdeResult {
    // Fixme - unimplemented.  Unsure how to use serde to produce a PyDict.
    return Ok(HashMap::new());
}
