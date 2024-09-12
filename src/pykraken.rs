use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyAny};
use serde_json::Value;
mod kraken;


fn json_value_to_pyobject(py: Python, value: &serde_json::Value) -> PyObject {
    match value {
        Value::Null => py.None(),
        Value::Bool(b) => b.into_py(py),
        Value::Number(num) => {
            if let Some(i) = num.as_i64() {
                i.into_py(py)
            } else if let Some(u) = num.as_u64() {
                u.into_py(py)
            } else if let Some(f) = num.as_f64() {
                f.into_py(py)
            } else {
                py.None() // Handle unexpected cases gracefully
            }
        }
        Value::String(s) => s.into_py(py),
        Value::Array(arr) => {
            let py_list = PyList::new(py, arr.into_iter().map(|x| json_value_to_pyobject(py, x)));
            py_list.into_py(py)
        }
        Value::Object(obj) => {
            let py_dict = PyDict::new(py);
            for (k, v) in obj {
                py_dict.set_item(k, json_value_to_pyobject(py, v)).unwrap();
            }
            py_dict.into_py(py)
        }
    }
}

fn convert_json_to_dict(py: Python, value: Value) -> PyResult<Py<PyDict>>
{
    if let Value::Object(map) = value {
        let dict = PyDict::new(py);
        for (key, val) in map.iter() {
            dict.set_item(key, json_value_to_pyobject(py, val)).unwrap();
        }
        Ok(dict.into())
    } else {
        let args = "Expected a JSON object";
        Err(pyo3::exceptions::PyTypeError::new_err(args))
    }
}

#[pyfunction]
fn get_info(py: Python, key: String, value: String) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py_with_locals(
        py,
        pyo3_asyncio::tokio::get_current_locals(py)?,
        async move {
            match kraken::get_info(&key, &value).await {
                Ok(response) => match response.json::<Value>().await {
                    Ok(json_data) => {
                        Python::with_gil(|py| {
                            convert_json_to_dict(py, json_data).map(|dict| dict.into_py(py))
                        })
                    },
                    Err(e) => Err(pyo3::exceptions::PyException::new_err(format!("Failed to parse JSON: {}", e))),
                },
                Err(e) => Err(pyo3::exceptions::PyException::new_err(format!("Request failed: {}", e))),
            }
        }
    )
}

/// A Python module implemented in Rust.
#[pymodule]
fn pykraken(_py: Python, m: &PyModule) -> PyResult<()>
{
    m.add_function(wrap_pyfunction!(get_info, m)?)?;

    Ok(())
}
