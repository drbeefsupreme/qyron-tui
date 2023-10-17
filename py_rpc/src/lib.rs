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

    Ok(())
}

pub fn nextGif(config: &Config) -> PyResult<()> {
    let a = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let hand: Py<PyAny> = PyModule::from_code(py, config.file, "", "")?
            .getattr("nextGif")?
            .into();
        hand.call0(py)
    });

    Ok(())
}

pub fn noGif(config: &Config) -> PyResult<()> {
    let a = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let hand: Py<PyAny> = PyModule::from_code(py, config.file, "", "")?
            .getattr("noGif")?
            .into();
        hand.call0(py)
    });

    Ok(())
}

pub fn enableGifsLoop(config: &Config) -> PyResult<()> {
    let a = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let hand: Py<PyAny> = PyModule::from_code(py, config.file, "", "")?
            .getattr("enableGifsLoop")?
            .into();
        hand.call0(py)
    });

    Ok(())
}

pub fn disableGifsLoop(config: &Config) -> PyResult<()> {
    let a = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let hand: Py<PyAny> = PyModule::from_code(py, config.file, "", "")?
            .getattr("disableGifsLoop")?
            .into();
        hand.call0(py)
    });

    Ok(())
}

pub fn caw(config: &Config) -> PyResult<()> {
    let a = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let hand: Py<PyAny> = PyModule::from_code(py, config.file, "", "")?
            .getattr("caw")?
            .into();
        hand.call0(py)
    })?;

    Ok(())
}

pub fn text1(config: &Config, text: String) -> PyResult<()> {
    let a = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let args = PyTuple::new(py, &[text.as_bytes()]);
        let hand: Py<PyAny> = PyModule::from_code(py, config.file, "", "")?
            .getattr("text1")?
            .into();
        hand.call1(py, args)
    })?;

    Ok(())
}
pub fn speed1(config: &Config, speed: u32) -> PyResult<()> {
    let a = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let args = PyTuple::new(py, &[speed]);
        let hand: Py<PyAny> = PyModule::from_code(py, config.file, "", "")?
            .getattr("speed1")?
            .into();
        hand.call1(py, args)
    })?;

    Ok(())
}
pub fn text2(config: &Config, text: String) -> PyResult<()> {
    let a = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let args = PyTuple::new(py, &[text.as_bytes()]);
        let hand: Py<PyAny> = PyModule::from_code(py, config.file, "", "")?
            .getattr("text2")?
            .into();
        hand.call1(py, args)
    })?;

    Ok(())
}
pub fn speed2(config: &Config, speed: u32) -> PyResult<()> {
    let a = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let args = PyTuple::new(py, &[speed]);
        let hand: Py<PyAny> = PyModule::from_code(py, config.file, "", "")?
            .getattr("speed2")?
            .into();
        hand.call1(py, args)
    })?;

    Ok(())
}
pub fn text3(config: &Config, text: String) -> PyResult<()> {
    let a = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let args = PyTuple::new(py, &[text.as_bytes()]);
        let hand: Py<PyAny> = PyModule::from_code(py, config.file, "", "")?
            .getattr("text3")?
            .into();
        hand.call1(py, args)
    })?;

    Ok(())
}
pub fn speed3(config: &Config, speed: u32) -> PyResult<()> {
    let a = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let args = PyTuple::new(py, &[speed]);
        let hand: Py<PyAny> = PyModule::from_code(py, config.file, "", "")?
            .getattr("speed3")?
            .into();
        hand.call1(py, args)
    })?;

    Ok(())
}
pub fn text4(config: &Config, text: String) -> PyResult<()> {
    let a = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let args = PyTuple::new(py, &[text.as_bytes()]);
        let hand: Py<PyAny> = PyModule::from_code(py, config.file, "", "")?
            .getattr("text4")?
            .into();
        hand.call1(py, args)
    })?;

    Ok(())
}
pub fn speed4(config: &Config, speed: u32) -> PyResult<()> {
    let a = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let args = PyTuple::new(py, &[speed]);
        let hand: Py<PyAny> = PyModule::from_code(py, config.file, "", "")?
            .getattr("speed4")?
            .into();
        hand.call1(py, args)
    })?;

    Ok(())
}
pub fn text5(config: &Config, text: String) -> PyResult<()> {
    let a = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let args = PyTuple::new(py, &[text.as_bytes()]);
        let hand: Py<PyAny> = PyModule::from_code(py, config.file, "", "")?
            .getattr("text5")?
            .into();
        hand.call1(py, args)
    })?;

    Ok(())
}
pub fn speed5(config: &Config, speed: u32) -> PyResult<()> {
    let a = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let args = PyTuple::new(py, &[speed]);
        let hand: Py<PyAny> = PyModule::from_code(py, config.file, "", "")?
            .getattr("speed5")?
            .into();
        hand.call1(py, args)
    })?;

    Ok(())
}

pub fn dopamine(config: &Config) -> PyResult<()> {
    let a = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
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
pub fn clear_text(config: &Config) -> PyResult<()> {
    Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let hand: Py<PyAny> = PyModule::from_code(py, config.file, "", "")?
            .getattr("clear_text")?
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

pub fn gifK(config: &Config) -> PyResult<()> {
    let a = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let hand: Py<PyAny> = PyModule::from_code(py, config.file, "", "")?
            .getattr("gifK")?
            .into();
        hand.call0(py)
    });

    Ok(())
}
pub fn gifO(config: &Config) -> PyResult<()> {
    let a = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let hand: Py<PyAny> = PyModule::from_code(py, config.file, "", "")?
            .getattr("gifO")?
            .into();
        hand.call0(py)
    });

    Ok(())
}
pub fn gifF(config: &Config) -> PyResult<()> {
    let a = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let hand: Py<PyAny> = PyModule::from_code(py, config.file, "", "")?
            .getattr("gifF")?
            .into();
        hand.call0(py)
    });

    Ok(())
}
pub fn gifD(config: &Config) -> PyResult<()> {
    let a = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let hand: Py<PyAny> = PyModule::from_code(py, config.file, "", "")?
            .getattr("gifD")?
            .into();
        hand.call0(py)
    });

    Ok(())
}
pub fn gifJ(config: &Config) -> PyResult<()> {
    let a = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let hand: Py<PyAny> = PyModule::from_code(py, config.file, "", "")?
            .getattr("gifJ")?
            .into();
        hand.call0(py)
    });

    Ok(())
}
pub fn gifS(config: &Config) -> PyResult<()> {
    let a = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let hand: Py<PyAny> = PyModule::from_code(py, config.file, "", "")?
            .getattr("gifS")?
            .into();
        hand.call0(py)
    });

    Ok(())
}
pub fn gifB(config: &Config) -> PyResult<()> {
    let a = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let hand: Py<PyAny> = PyModule::from_code(py, config.file, "", "")?
            .getattr("gifB")?
            .into();
        hand.call0(py)
    });

    Ok(())
}
pub fn gifT(config: &Config) -> PyResult<()> {
    let a = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let hand: Py<PyAny> = PyModule::from_code(py, config.file, "", "")?
            .getattr("gifT")?
            .into();
        hand.call0(py)
    });

    Ok(())
}
pub fn gifA(config: &Config) -> PyResult<()> {
    let a = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let hand: Py<PyAny> = PyModule::from_code(py, config.file, "", "")?
            .getattr("gifA")?
            .into();
        hand.call0(py)
    });

    Ok(())
}

pub fn gifP(config: &Config) -> PyResult<()> {
    let a = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let hand: Py<PyAny> = PyModule::from_code(py, config.file, "", "")?
            .getattr("gifP")?
            .into();
        hand.call0(py)
    });

    Ok(())
}
pub fn gifH(config: &Config) -> PyResult<()> {
    let a = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let hand: Py<PyAny> = PyModule::from_code(py, config.file, "", "")?
            .getattr("gifH")?
            .into();
        hand.call0(py)
    });

    Ok(())
}
