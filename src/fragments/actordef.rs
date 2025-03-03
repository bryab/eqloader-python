use libeq_wld::parser::{ActorDef, DmSprite, FragmentRef, WldDoc};
use pyo3::{pyclass, pymethods};
use std::sync::Arc;
extern crate owning_ref;
use super::{create_fragment_ref, S3DFragment, S3DMesh};
#[cfg(feature = "serde")]
use super::{frag_to_dict, SerdeResult};
use owning_ref::ArcRef;
use pyo3::prelude::*;
#[pyclass]
pub struct S3DActorDef {
    fragment: ArcRef<WldDoc, ActorDef>,
}

impl S3DFragment for S3DActorDef {
    fn new(wld: &Arc<WldDoc>, index: u32) -> PyResult<Self> {
        Ok(S3DActorDef {
            fragment: create_fragment_ref(wld.clone(), index)?,
        })
    }
}

/// The S3DMaterial object simplifies the Materials and Textures system in S3D files, flattening it into something that is easy to use in Godot.
#[pymethods]
impl S3DActorDef {
    #[getter]
    pub fn name(&self) -> String {
        String::from(
            self.get_wld()
                .get_string(self.get_frag().name_reference)
                .expect("Failed to get string from WLD!"),
        )
    }

    #[getter]
    fn callback_name(&self) -> String {
        String::from(
            self.get_wld()
                .get_string(self.get_frag().callback_name_reference)
                .expect("Failed to get string from WLD!"),
        )
    }

    // #[getter]
    // fn references(&self) -> Array<Variant> {
    //     let wld = self.get_wld();
    //     self.get_frag()
    //         .fragment_references
    //         .iter()
    //         .filter_map(|fragment_ref| Some(gd_from_frag(wld, *fragment_ref)))
    //         .collect()
    // }

    #[getter]
    fn meshes(&self) -> Vec<S3DMesh> {
        let wld = self.get_wld();
        self.get_frag()
            .fragment_references
            .iter()
            .filter_map(|fragment_ref| {
                let mesh_reference_ref = FragmentRef::<DmSprite>::new(*fragment_ref as i32);
                let mesh_reference = wld.get(&mesh_reference_ref)?;
                S3DMesh::from_reference(wld, mesh_reference).ok()
            })
            .collect()
    }

    #[cfg(feature = "serde")]
    #[getter]
    pub fn as_dict(&self) -> SerdeResult {
        let frag = self.get_frag();
        let wld = self.get_wld();
        frag_to_dict(wld, frag)
    }
}

impl S3DActorDef {
    fn get_wld(&self) -> &Arc<WldDoc> {
        self.fragment.as_owner()
    }

    fn get_frag(&self) -> &ActorDef {
        self.fragment.as_ref()
    }
}
