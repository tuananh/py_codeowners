use std::str;
use std::path::PathBuf;
use pyo3::prelude::*;
use pyo3::types::PyType;

extern crate codeowners;

#[pyclass]
struct PyCodeowners {
    owners: codeowners::Owners,
}

#[pymethods]
impl PyCodeowners {
    #[new]
    fn new(content: String) -> Self {
        let owners = codeowners::from_reader(content.as_bytes());
        Self {owners : owners}
    }

    #[classmethod]
    fn from_path(_cls: &Bound<'_, PyType>, path_string: String) -> Self {
        let path = PathBuf::from(path_string);
        let owners = codeowners::from_path(path);
        Self {owners : owners}
    }
    
    fn of(&self, path: String) -> Vec<String> {
        match self.owners.of(&path) {
            None => Vec::new(),
            Some(owners) => {
                owners.into_iter()
                    .map(|owner| owner.to_string())
                    .collect()
            }
          }
    }
}

#[pymodule]
fn py_codeowners(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyCodeowners>()?;
    Ok(())
}