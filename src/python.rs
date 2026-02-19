use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use crate::AztecCode;
use crate::writer::{AztecCodeBuilder, build_rune};

#[pyclass(name = "AztecCode")]
struct PyAztecCode {
    inner: AztecCode,
}

#[pymethods]
impl PyAztecCode {
    fn size(&self) -> usize {
        self.inner.size()
    }

    fn is_compact(&self) -> bool {
        self.inner.is_compact()
    }

    fn invert(&mut self) {
        self.inner.invert();
    }

    fn to_mono8(&self, module_size: usize) -> Vec<u8> {
        self.inner.to_mono8(module_size)
    }

    fn to_rgb8(&self, module_size: usize) -> Vec<u32> {
        self.inner.to_rgb8(module_size)
    }

    fn __str__(&self) -> String {
        format!("{}", self.inner)
    }

    fn __repr__(&self) -> String {
        format!(
            "AztecCode(size={}, compact={})",
            self.inner.size(),
            self.inner.is_compact()
        )
    }
}

#[pyclass(name = "AztecCodeBuilder")]
struct PyAztecCodeBuilder {
    inner: AztecCodeBuilder,
}

#[pymethods]
impl PyAztecCodeBuilder {
    #[new]
    fn new() -> Self {
        PyAztecCodeBuilder {
            inner: AztecCodeBuilder::new(),
        }
    }

    fn error_correction(&mut self, rate: usize) {
        self.inner.error_correction(rate);
    }

    fn append(&mut self, text: &str) {
        self.inner.append(text);
    }

    fn append_bytes(&mut self, data: &[u8]) {
        self.inner.append_bytes(data);
    }

    fn append_eci(&mut self, code: u16) {
        self.inner.append_eci(code);
    }

    fn build(&self) -> PyResult<PyAztecCode> {
        self.inner
            .build()
            .map(|code| PyAztecCode { inner: code })
            .map_err(|e| PyValueError::new_err(e))
    }
}

#[pyfunction]
fn rune(data: u8) -> PyAztecCode {
    PyAztecCode {
        inner: build_rune(data),
    }
}

#[pymodule]
fn raztec(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyAztecCode>()?;
    m.add_class::<PyAztecCodeBuilder>()?;
    m.add_function(wrap_pyfunction!(rune, m)?)?;
    Ok(())
}
