//use notcurses::*;
use pyo3::prelude::*;
//use pyo3::types::PyTuple;

fn main() -> PyResult<()> {
    let py_connect = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/python/connect.py"));
    let from_python = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
//        let simple_rpc = PyModule::import(py, "arduino-simple-rpc").expect("bweh");
        let connect: Py<PyAny> = PyModule::from_code(py, py_connect, "", "")?
            .getattr("run")?
            .into();
        connect.call0(py)
    });

    println!("py: {}", from_python?);
    Ok(())
}
