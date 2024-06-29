use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
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
            let py_list = PyList::new_bound(py, arr.into_iter().map(|x| json_value_to_pyobject(py, x)));
            py_list.into_py(py)
        }
        Value::Object(obj) => {
            let py_dict = PyDict::new_bound(py);
            for (k, v) in obj {
                py_dict.set_item(k, json_value_to_pyobject(py, v)).unwrap();
            }
            py_dict.into_py(py)
        }
    }
}

fn convert_json_to_dict<'py>(py: Python<'py>, value: Value) -> PyResult<Bound<'py, PyDict>>
{
    if let Value::Object(map) = value {
        let dict = PyDict::new_bound(py);
        for (key, val) in map.iter() {
            dict.set_item(key, json_value_to_pyobject(py, val)).unwrap();
        }
        Ok(dict.into())
    } else {
        Err(pyo3::exceptions::PyTypeError::new_err("Expected a JSON object"))
    }
}

#[pyfunction]
fn get_info<'py>(py: Python<'py>, key: &String, value: &String) -> PyResult<Bound<'py, PyDict>>
{
    pyo3_asyncio::async_std::future_into_py(py, async {

    let data = kraken::get_info(key, value).await.map_err(|e| {
        pyo3::exceptions::PyException::new_err(format!("Request failed: {}", e))
    })?;

    if data.status().is_success() {

        let json_data: Value = data.json().await.map_err(|e| {
            pyo3::exceptions::PyException::new_err(format!("Failed to parse JSON: {}", e))
        })?;

        convert_json_to_dict(py, json_data)
    } else  {
        Err(pyo3::exceptions::PyException::new_err(format!("Request failed with status code: {}", data.status())))
    }
    })
}


/// A Python module implemented in Rust.
#[pymodule]
fn pykraken(m: &Bound<'_, PyModule>) -> PyResult<()>
{
    m.add_function(wrap_pyfunction!(get_info, m)?)?;

    Ok(())
}