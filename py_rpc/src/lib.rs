use pyo3::prelude::*;
//use pyo3::types::PyTuple;

pub struct Config {
    pub file: &'static str
}

impl Config {
    pub fn new() -> Config {
        let py_connect = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/python/connect.py"));
        Config { file: py_connect }
    }
}

//TODO: return an error if fail to connect
pub fn connect(config: &Config) -> PyResult<()> {
    Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let hand: Py<PyAny> = PyModule::from_code(py, config.file, "", "")?
            .getattr("connect")?
            .into();
        hand.call0(py)
    })?;

    Ok(())
}

pub fn caw(config: &Config) -> PyResult<()> {
    Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let hand: Py<PyAny> = PyModule::from_code(py, config.file, "", "")?
            .getattr("caw")?
            .into();
        hand.call0(py)
    })?;

    Ok(())
}

pub fn dopamine(config: &Config) -> PyResult<()> {
    Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let hand: Py<PyAny> = PyModule::from_code(py, config.file, "", "")?
            .getattr("dopamine")?
            .into();
        hand.call0(py)
    })?;

    Ok(())
}

pub fn clear(config: &Config) -> PyResult<()> {
    Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let hand: Py<PyAny> = PyModule::from_code(py, config.file, "", "")?
            .getattr("clear")?
            .into();
        hand.call0(py)
    })?;

    Ok(())
}

pub fn pixels(config: &Config) -> PyResult<()> {
    Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let hand: Py<PyAny> = PyModule::from_code(py, config.file, "", "")?
            .getattr("pixels")?
            .into();
        hand.call0(py)
    })?;

    Ok(())
}

pub fn shapes(config: &Config) -> PyResult<()> {
    Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let hand: Py<PyAny> = PyModule::from_code(py, config.file, "", "")?
            .getattr("shapes")?
            .into();
        hand.call0(py)
    })?;

    Ok(())
}

pub fn temp(config: &Config) -> PyResult<()> {
    Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let hand: Py<PyAny> = PyModule::from_code(py, config.file, "", "")?
            .getattr("temp")?
            .into();
        hand.call0(py)
    })?;

    Ok(())
}
