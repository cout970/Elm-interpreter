use std::collections::HashMap;
use std::ffi::OsString;
use std::fs;
use std::io::Error;
use std::path::Path;

use source::SourceCode;

#[derive(Clone, Debug)]
pub struct ModuleLoader {
    loaded_modules: HashMap<String, SourceFile>
}

#[derive(Clone, Debug)]
struct SourceFile {
    name: String,
    path: String,
    source: SourceCode,
}

impl ModuleLoader {
    pub fn new() -> Self {
        ModuleLoader { loaded_modules: HashMap::new() }
    }

    pub fn include_folder(&mut self, path: &str) -> Result<(), Error> {
        let mut sources = vec![];
        get_all_source_files(&mut sources, "", path)?;

        dbg!(sources);
        Ok(())
    }

    pub fn include_file(&mut self, inner_path: &str, path: &str) -> Result<(), Error> {
        let source = get_source_file(inner_path, path)?;

        dbg!(source);
        Ok(())
    }
}

fn get_all_source_files(dst: &mut Vec<SourceFile>, inner_path: &str, path: &str) -> Result<(), Error> {
    let directory = fs::read_dir(path)?;

    for entryResult in directory {
        let entry = entryResult?;
        let file_type = entry.file_type()?;
        let file_name = entry.file_name().to_str().unwrap().to_string();
        let file_path = format!("{}/{}", path, file_name);

        if file_type.is_file() && file_name.ends_with(".elm") {
            dst.push(get_source_file(inner_path, &file_path)?);
        } else if file_type.is_dir() {
            let inner: String = if inner_path.is_empty() {
                file_name
            } else {
                format!("{}/{}", inner_path, file_name)
            };

            get_all_source_files(dst, &inner, &file_path)?
        }
    }
    Ok(())
}

fn get_source_file(inner_path: &str, abs_path: &str) -> Result<SourceFile, Error> {
    let path = Path::new(abs_path);
    let file_name = path.file_name().unwrap().to_str().unwrap();

    let module_name = if inner_path.is_empty() {
        file_name.to_string()
    } else {
        format!("{}/{}", inner_path, file_name)
    };

    let file_contents = fs::read(abs_path)?;

    let loaded_module = SourceFile {
        name: module_name.trim_end_matches(".elm").to_string(),
        path: abs_path.to_string(),
        source: SourceCode::from_bytes(file_contents),
    };

    println!("Including file: {}, module: {:?}", abs_path, loaded_module);
    Ok(loaded_module)
}

#[cfg(test)]
mod test {
    use util::test_resource;

    use super::*;

    #[test]
    fn test_include_folder() {
        let mut loader = ModuleLoader::new();
        loader.include_folder(&test_resource("sample_project")).unwrap();
    }
}