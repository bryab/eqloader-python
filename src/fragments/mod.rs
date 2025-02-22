mod mesh;
use libeq_wld::parser::{Fragment, WldDoc};
pub use mesh::*;
use owning_ref::ArcRef;
use std::sync::Arc;

/// Create a reference to a particular fragment by pairing it with its parent WLD in an OwnedRef.
fn create_fragment_ref<T: 'static + Fragment>(wld: Arc<WldDoc>, index: u32) -> ArcRef<WldDoc, T> {
    ArcRef::new(wld).map(|wld| {
        wld.at((index - 1) as usize)
            .expect(format!("Fragment index {index} is out of bounds!").as_str())
            .as_any()
            .downcast_ref()
            .expect(format!("Fragment at index {index} is not of the requested type!").as_str())
    })
}
