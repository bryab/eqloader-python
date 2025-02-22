mod fragments;
mod util;
mod wld;
use crate::wld::S3DWld;
use libeq_archive::EqArchive;
use pyo3::exceptions::PyOSError;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::ffi::OsStr;
use std::fs::File;
use std::path::Path;

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
        let archive = EqArchive::read(file)
            //.map_err(|e| Err(PyOSError::new_err("Fail")))
            .unwrap();
        let name = String::from(Path::new(&filename).file_stem().unwrap().to_str().unwrap());
        Ok(S3DArchive { archive, name })
    }

    pub fn get_filenames(&mut self) -> PyResult<Vec<String>> {
        Ok(self.archive.iter().map(|(s, _)| String::from(s)).collect())
    }

    pub fn get_wld(&self, filename: &str) -> PyResult<S3DWld> {
        self._get_wld(filename.to_string().as_str())
    }

    /// Returns the main WLD inside the S3D file.
    /// For Zone S3Ds, this is the WLD containing the zone data.
    /// For ActorDef and Character S3Ds, this is the only WLD in the archive.
    pub fn get_main_wld(&self) -> PyResult<S3DWld> {
        self._get_wld(&format!("{0}.wld", &self.name))
    }

    /// In Zone S3Ds, this will return the lights.wld within the archive.
    pub fn get_lights_wld(&self) -> PyResult<S3DWld> {
        self._get_wld("lights.wld")
    }

    /// In Zone S3Ds, this will return the objects.wld within the archive.
    pub fn get_actorinst_wld(&self) -> PyResult<S3DWld> {
        self._get_wld("objects.wld")
    }
}

impl S3DArchive {
    /// Attempt to get the given data from the archive.
    /// An error is printed in Godot if the file does not exist.
    fn _get(&self, filename: &str) -> PyResult<Vec<u8>> {
        Ok(self
            .archive
            .iter()
            .find(|(name, _)| name == &filename)
            .and_then(|(_, data)| Some(data.clone()))
            .unwrap())
    }

    /// Returns an EQWld object representing a WLD file
    fn _get_wld(&self, filename: &str) -> PyResult<S3DWld> {
        let data = self._get(filename)?;
        S3DWld::new(data)
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn blender_eqloader(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<S3DArchive>()?;
    m.add_class::<S3DWld>()?;
    //m.add_class::<S3DFace>()?;
    Ok(())
}
