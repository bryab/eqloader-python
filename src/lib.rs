use libeq_archive::EqArchive;
use pyo3::exceptions::PyOSError;
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
}

/// A Python module implemented in Rust.
#[pymodule]
fn blender_eqloader(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<S3DArchive>()?;
    Ok(())
}
