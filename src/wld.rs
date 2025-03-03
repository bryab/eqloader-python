use crate::fragments::{S3DActorDef, S3DActorInstance, S3DFragment, S3DMaterial, S3DMesh};
use libeq_wld::parser::{Actor, ActorDef, DmSpriteDef2, Fragment, MaterialDef, WldDoc};
use pyo3::{exceptions::PyValueError, prelude::*};
use std::sync::Arc;

#[pyclass]
pub struct S3DWld {
    wld: Arc<WldDoc>,
    filename: String,
}

#[pymethods]
impl S3DWld {
    #[getter]
    pub fn filename(&self) -> &str {
        &self.filename
    }

    #[getter]
    pub fn meshes(&self) -> Vec<S3DMesh> {
        self.build_fragment_type_array::<S3DMesh, DmSpriteDef2>()
    }
    #[getter]
    pub fn materials(&self) -> Vec<S3DMaterial> {
        self.build_fragment_type_array::<S3DMaterial, MaterialDef>()
    }
    #[getter]
    pub fn actordefs(&self) -> Vec<S3DActorDef> {
        self.build_fragment_type_array::<S3DActorDef, ActorDef>()
    }
    #[getter]
    pub fn actorinstances(&self) -> Vec<S3DActorInstance> {
        self.build_fragment_type_array::<S3DActorInstance, Actor>()
    }
}

impl S3DWld {
    pub fn new(data: Vec<u8>, filename: &str) -> PyResult<Self> {
        let wld_doc = WldDoc::parse(&data[..])
            .map_err(|e| PyValueError::new_err(format!("Failed to parse '{filename}': {e:?}")))?;
        Ok(S3DWld {
            wld: Arc::new(wld_doc),
            filename: String::from(filename),
        })
    }

    fn build_fragment_type_array<T: S3DFragment, T2: 'static + Fragment>(&self) -> Vec<T> {
        self.wld
            .iter()
            .enumerate()
            .filter_map(|(index, fragment)| {
                let fragment = fragment.as_any().downcast_ref::<T2>();
                fragment.and_then(|_| T::new(&self.wld, index as u32 + 1).ok())
            })
            .collect()
    }
}
