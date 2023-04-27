pub fn assert_error_desc_eq<T>(res: anyhow::Result<T>, expected: &str) {
    let error = format!("{:#}", res.err().unwrap());
    assert_eq!(error, expected.trim_end());
}

pub fn err_str<T>(s: &str) -> Result<T, String> {
    Err(s.to_string())
}

pub fn stringify_error<T>(res: anyhow::Result<T>) -> Result<T, String> {
    res.map_err(|e| format!("{:#}", e))
}
