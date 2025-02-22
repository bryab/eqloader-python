use libeq_archive::EqArchive;
use libeq_wld::parser::{
    Actor, ActorDef, Fragment, FragmentType, HierarchicalSpriteDef, MaterialDef, WldDoc,
};
use pyo3::exceptions::PyOSError;
use pyo3::prelude::*;
use std::ffi::OsStr;
use std::fs::File;
use std::path::Path;
use std::sync::Arc;

#[pyclass]
pub struct S3DWld {
    wld: Arc<WldDoc>,
}

#[pymethods]
impl S3DWld {
    pub fn meshes(&self) -> PyResult<Vec<String>> {
        Ok(self
            .wld
            .iter()
            .enumerate()
            .filter_map(|(index, fragment)| match fragment.as_ref() {
                FragmentType::DmSpriteDef2(f) => {
                    Some(String::from(self.wld.get_string(f.name_reference)?))
                }
                _ => None,
            })
            .collect())
    }
}

impl S3DWld {
    pub fn new(data: Vec<u8>) -> PyResult<Self> {
        let wld = match WldDoc::parse(&data[..]) {
            Ok(wld_doc) => Some(Arc::new(wld_doc)),
            Err(err) => panic!("Failed to parse Wld: {:?}", err),
        }
        .unwrap();
        Ok(S3DWld { wld })
    }
}
