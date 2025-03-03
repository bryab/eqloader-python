mod bmp;
mod fragments;
mod util;
mod wld;
use crate::wld::S3DWld;
use bmp::texture_transparent_color;
use fragments::{S3DActorDef, S3DActorInstance, S3DFace, S3DMaterial, S3DMesh};
use libeq_archive::EqArchive;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::fs::File;
use std::path::Path;
use util::WORLD_SCALE;

#[pyclass]
struct S3DArchive {
    archive: EqArchive,
    /// The file stem of the archive, e.g. "rivervale".  This is used to get the main WLD out of the archive without specifying its name.
    name: String,
}

#[pymethods]
impl S3DArchive {
    #[new]
    fn new(filename: &str) -> PyResult<Self> {
        let file = File::open(&filename)?;
        let archive = EqArchive::read(file).map_err(|e| {
            PyValueError::new_err(format!("Failed to read archive '{filename}': {e:?}"))
        })?;
        let name = String::from(
            Path::new(&filename)
                .file_stem()
                .ok_or(PyValueError::new_err("Invalid path"))?
                .to_str()
                .ok_or(PyValueError::new_err("Invalid string"))?,
        );
        Ok(S3DArchive { archive, name })
    }

    #[getter]
    pub fn filenames(&self) -> PyResult<Vec<String>> {
        Ok(self.archive.iter().map(|(s, _)| String::from(s)).collect())
    }

    pub fn get_bmp_mask_color(&self, filename: &str) -> PyResult<(f32, f32, f32)> {
        return texture_transparent_color(self._get(&filename)?);
    }

    pub fn get_bytes(&self, filename: &str) -> PyResult<Vec<u8>> {
        self._get(&filename)
    }

    pub fn get_wld(&self, filename: &str) -> PyResult<S3DWld> {
        self._get_wld(&filename)
    }

    /// Returns the main WLD inside the S3D file.
    /// For Zone S3Ds, this is the WLD containing the zone data.
    /// For ActorDef and Character S3Ds, this is the only WLD in the archive.
    #[getter]
    pub fn main_wld(&self) -> PyResult<S3DWld> {
        self._get_wld(&format!("{0}.wld", &self.name))
    }
    #[getter]
    /// In Zone S3Ds, this will return the lights.wld within the archive.
    pub fn lights_wld(&self) -> PyResult<S3DWld> {
        self._get_wld("lights.wld")
    }
    #[getter]
    /// In Zone S3Ds, this will return the objects.wld within the archive.
    pub fn actorinst_wld(&self) -> PyResult<S3DWld> {
        self._get_wld("objects.wld")
    }
}

impl S3DArchive {
    /// Attempt to get the given data from the archive.
    /// An error is printed in Godot if the file does not exist.
    fn _get(&self, filename: &str) -> PyResult<Vec<u8>> {
        self.archive
            .iter()
            .find(|(name, _)| name == &filename)
            .and_then(|(_, data)| Some(data.clone()))
            .ok_or_else(|| PyValueError::new_err(format!("'{filename}' not found in the archive.")))
    }

    /// Returns an EQWld object representing a WLD file
    fn _get_wld(&self, filename: &str) -> PyResult<S3DWld> {
        S3DWld::new(self._get(filename)?, filename)
    }
}

#[pyfunction]
fn world_scale() -> f32 {
    WORLD_SCALE
}

/// A Python module implemented in Rust.
#[pymodule]
fn eqloader(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<S3DArchive>()?;
    m.add_class::<S3DWld>()?;
    m.add_class::<S3DMesh>()?;
    m.add_class::<S3DFace>()?;
    m.add_class::<S3DMaterial>()?;
    m.add_class::<S3DActorDef>()?;
    m.add_class::<S3DActorInstance>()?;
    m.add_class::<S3DMaterial>()?;
    m.add_function(wrap_pyfunction!(world_scale, m)?)?;
    Ok(())
}
