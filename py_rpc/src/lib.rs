use pyo3::prelude::*;
//use pyo3::types::PyTuple;

fn main() -> PyResult<()> {
    let py_connect = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/python/connect.py"));
    Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let connect: Py<PyAny> = PyModule::from_code(py, py_connect, "", "")?
            .getattr("connect")?
            .into();
        let connect: Py<PyAny> = connect();
        connect.call0(py)
    });

    Ok(())
}

fn connect() -> PyResult<()> {
    let py_connect = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/python/connect.py"));
    Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        PyModule::from_code(py, py_connect, "", "")?.getattr("connect")?.into()
    });

    Ok(())
}
