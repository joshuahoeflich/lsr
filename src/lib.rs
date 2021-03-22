use std::path::PathBuf;

#[derive(Debug)]
pub enum SearchError {
    ReadDir,
    FileString,
    BadDir,
}

fn read_dir(path: PathBuf) -> Result<std::fs::ReadDir, SearchError> {
    Ok(std::fs::read_dir(path).map_err(|_| SearchError::ReadDir)?)
}

pub fn log_err(err: SearchError) {
    match err {
        SearchError::ReadDir => println!("Could not read directory."),
        SearchError::FileString => println!("Could not convert file name to string."),
        SearchError::BadDir => println!("Could not get the current directory."),
    }
}

pub fn log_strings(strs: Vec<String>) {
    for a_string in strs {
        println!("{}", a_string)
    }
}

pub fn get_arg_paths() -> Result<Vec<PathBuf>, SearchError> {
    let mut out: Vec<PathBuf> = vec![];
    for arg in std::env::args().skip(1) {
        let mut path = std::path::PathBuf::new();
        path.push(std::env::current_dir().map_err(|_| SearchError::BadDir)?);
        path.push(arg);
        out.push(path);
    }
    Ok(out)
}

pub fn get_dir_files(path: PathBuf) -> Result<Vec<String>, SearchError> {
    let mut out: Vec<String> = vec![];
    if path.is_dir() {
        for el in read_dir(path)? {
            out.append(&mut get_dir_files(
                el.map_err(|_| SearchError::ReadDir)?.path(),
            )?);
        }
        return Ok(out);
    }
    if path.is_file() {
        out.push(path.to_str().ok_or(SearchError::FileString)?.to_string());
        return Ok(out);
    }
    return Ok(out);
}
