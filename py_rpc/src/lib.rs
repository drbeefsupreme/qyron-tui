use pyo3::prelude::*;
use pyo3::types::PyTuple;

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
    let a = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let hand: Py<PyAny> = PyModule::from_code(py, config.file, "", "")?
            .getattr("connect")?
            .into();
        hand.call0(py)
    });

    println!("connect: {:?}", a);

    Ok(())
}

pub fn nextGif(config: &Config) -> PyResult<()> {
    let a = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let hand: Py<PyAny> = PyModule::from_code(py, config.file, "", "")?
            .getattr("nextGif")?
            .into();
        hand.call0(py)
    });

    println!("nextGif: {:?}", a);

    Ok(())
}

pub fn noGif(config: &Config) -> PyResult<()> {
    let a = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let hand: Py<PyAny> = PyModule::from_code(py, config.file, "", "")?
            .getattr("noGif")?
            .into();
        hand.call0(py)
    });

    println!("noGif: {:?}", a);

    Ok(())
}

pub fn caw(config: &Config) -> PyResult<()> {
    let a = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let hand: Py<PyAny> = PyModule::from_code(py, config.file, "", "")?
            .getattr("caw")?
            .into();
        hand.call0(py)
    })?;
    println!("caw: {:?}", a);

    Ok(())
}

pub fn text(config: &Config, text: String) -> PyResult<()> {
    let a = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let args = PyTuple::new(py, &[text.as_bytes()]);
        let hand: Py<PyAny> = PyModule::from_code(py, config.file, "", "")?
            .getattr("text")?
            .into();
        hand.call1(py, args)
    })?;

    println!("text: {:?}", a);

    Ok(())
}

pub fn dopamine(config: &Config) -> PyResult<()> {
    let a = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let hand: Py<PyAny> = PyModule::from_code(py, config.file, "", "")?
            .getattr("dopamine")?
            .into();
        hand.call0(py)
    })?;

    println!("dopamine: {:?}", a);

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

pub fn pixelsBg(config: &Config) -> PyResult<()> {
    Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let hand: Py<PyAny> = PyModule::from_code(py, config.file, "", "")?
            .getattr("hitPixels")?
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

pub fn shapesBg(config: &Config) -> PyResult<()> {
    let a = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let hand: Py<PyAny> = PyModule::from_code(py, config.file, "", "")?
            .getattr("hitShapes")?
            .into();
        hand.call0(py)
    });

    println!("shapesBg: {:?}", a);

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
