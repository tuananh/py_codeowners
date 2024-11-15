use std::str;
use std::path::PathBuf;
use pyo3::prelude::*;

extern crate codeowners;

#[pyclass]
struct PyCodeowners {
    owners: codeowners::Owners,
}

#[pymethods]
impl PyCodeowners {
    #[new]
    fn from_path(path_string: String) -> Self {
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