use std::io::{Error, ErrorKind, Write};

pub enum FileType {
    File,
    Dir,
}

struct File {
    filename: String,
    data: Vec<u8>,
}

pub struct Directory {
    filename: String,
    files: Vec<File>,
    directories: Vec<Directory>,
}

impl File {
    fn new(filename: String) -> Self {
        Self {
            filename,
            data: vec![],
        }
    }
}

struct E;

fn split_filename(filename: &str) -> Result<Vec<String>, Error> {
    if filename == "" {
        return Ok(vec![String::new(); 0]);
    }

    let mut r = filename
        .split('/')
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    if filename.ends_with('/') {
        r.remove(r.len() - 1);
    }
    Ok(r)
}

impl Directory {
    fn new(filename: String) -> Self {
        Self {
            filename,
            files: vec![],
            directories: vec![],
        }
    }

    pub fn file_exists(&self, filename: &str) -> bool {
        let path_vec = match split_filename(filename) {
            Ok(v) => v,
            Err(_) => {
                return false;
            }
        };

        let mut curr = self;
        for path in path_vec {
            match curr.directories.iter().find(|&x| x.filename == path) {
                Some(dir) => {
                    curr = &dir;
                    continue;
                }
                None => {}
            };

            match curr.files.iter().find(|&x| x.filename == path) {
                Some(_) => {
                    return true;
                }
                None => {}
            }

            return false;
        }

        true
    }

    pub fn file_type(&self, filename: &str) -> Option<FileType> {
        let path_vec = match split_filename(filename) {
            Ok(v) => v,
            Err(_) => {
                return None;
            }
        };

        let mut curr = self;
        for path in path_vec {
            match curr.directories.iter().find(|&x| x.filename == path) {
                Some(dir) => {
                    curr = &dir;
                    continue;
                }
                None => {}
            };

            match curr.files.iter().find(|&x| x.filename == path) {
                Some(_) => {
                    return Some(FileType::File);
                }
                None => {}
            }

            return None;
        }

        Some(FileType::Dir)
    }

    pub fn ls(&self, filename: &str) -> Result<Vec<String>, Error> {
        let path_vec = split_filename(filename)?;

        let mut curr = self;
        for path in path_vec {
            match curr.directories.iter().find(|&x| x.filename == path) {
                Some(dir) => {
                    curr = &dir;
                    continue;
                }
                None => {}
            };

            match curr.files.iter().find(|&x| x.filename == path) {
                Some(_) => {
                    return Ok(vec![filename.to_string()]);
                }
                None => {}
            }

            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("\"{}\" does not correspond to a filename.", filename),
            ));
        }

        let mut results: Vec<String> = vec![];
        for dir in &curr.directories {
            results.push(filename.to_string() + &dir.filename);
        }
        for file in &curr.files {
            results.push(filename.to_string() + &file.filename)
        }
        Ok(results)
    }

    pub fn mkdir(&mut self, filename: &str) -> Result<(), Error> {
        let mut path_vec = split_filename(filename)?;
        let new_dir = path_vec.pop().unwrap();

        let mut curr = self;
        for path in path_vec {
            match curr.directories.iter_mut().find(|x| x.filename == path) {
                Some(dir) => {
                    curr = dir;
                    continue;
                }
                None => {}
            };
            return Err(Error::new(ErrorKind::InvalidInput, "Not a directory"));
        }

        curr.directories.push(Directory::new(new_dir));
        Ok(())
    }

    pub fn touch(&mut self, filename: &str) -> Result<(), Error> {
        let mut path_vec = split_filename(filename)?;
        let new_file = path_vec.pop().unwrap();

        let mut curr = self;
        for path in path_vec {
            match curr.directories.iter_mut().find(|x| x.filename == path) {
                Some(dir) => {
                    curr = dir;
                    continue;
                }
                None => {}
            };
            return Err(Error::new(ErrorKind::InvalidInput, "Not a directory"));
        }

        curr.files.push(File::new(new_file));
        Ok(())
    }

    pub fn write(&mut self, filename: &str, contents: &[u8]) -> Result<(), Error> {
        let mut path_vec = split_filename(filename)?;
        let new_file = path_vec.pop().unwrap();

        let mut curr = self;
        for path in path_vec {
            match curr.directories.iter_mut().find(|x| x.filename == path) {
                Some(dir) => {
                    curr = dir;
                    continue;
                }
                None => {}
            };
            return Err(Error::new(ErrorKind::InvalidInput, "Not a directory"));
        }

        // check if directory exists with filename
        match curr.directories.iter().find(|x| x.filename == new_file) {
            Some(_) => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Filename refers to directory",
                ));
            }
            None => {}
        };

        // check if file exists with filename
        match curr.files.iter_mut().find(|x| x.filename == new_file) {
            Some(file) => {
                file.data.clear();
                file.data.write_all(contents)?;
            }
            // create file first
            None => curr.files.push(File {
                filename: filename.to_string(),
                data: contents.to_vec(),
            }),
        };

        Ok(())
    }

    pub fn read(&self, filename: &str) -> Result<Vec<u8>, Error> {
        let mut path_vec = split_filename(filename)?;
        let new_file = path_vec.pop().unwrap();

        let mut curr = self;
        for path in path_vec {
            match curr.directories.iter().find(|x| x.filename == path) {
                Some(dir) => {
                    curr = dir;
                    continue;
                }
                None => {}
            };
            return Err(Error::new(ErrorKind::InvalidInput, "Not a directory"));
        }

        // check if file exists with filename
        match curr.files.iter().find(|x| x.filename == new_file) {
            Some(file) => Ok(file.data.clone()),
            None => Err(Error::new(
                ErrorKind::InvalidInput,
                "Filename refers to directory",
            )),
        }
    }

    pub fn cat(&mut self, filename: &str, contents: &[u8]) -> Result<(), Error> {
        let mut path_vec = split_filename(filename)?;
        let new_file = path_vec.pop().unwrap();

        let mut curr = self;
        for path in path_vec {
            match curr.directories.iter_mut().find(|x| x.filename == path) {
                Some(dir) => {
                    curr = dir;
                    continue;
                }
                None => {}
            };
            return Err(Error::new(ErrorKind::InvalidInput, "Not a directory"));
        }

        // check if directory exists with filename
        match curr.directories.iter().find(|x| x.filename == new_file) {
            Some(_) => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Filename refers to directory",
                ));
            }
            None => {}
        };

        // check if file exists with filename
        match curr.files.iter_mut().find(|x| x.filename == new_file) {
            Some(file) => {
                file.data.write_all(contents)?;
            }
            // create file first
            None => curr.files.push(File {
                filename: filename.to_string(),
                data: contents.to_vec(),
            }),
        };

        Ok(())
    }

    pub fn mv(&mut self, source: &str, dest: &str) -> Result<(), Error> {
        unimplemented!()
    }

    pub fn cp(&mut self, source: &str, dest: &str) -> Result<(), Error> {
        unimplemented!()
    }
}

const ROOT_FILENAME: &str = "";

pub fn create_root() -> Directory {
    Directory {
        filename: String::from(ROOT_FILENAME),
        files: vec![],
        directories: vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_filename_empty() {
        let r = split_filename("").unwrap();
        assert_eq!(r.len(), 0);
    }

    #[test]
    fn test_split_filename_single1() {
        let r = split_filename("acd").unwrap();
        assert_eq!(r, vec!["acd".to_string()]);
    }

    #[test]
    fn test_split_filename_single2() {
        let r = split_filename("a  cd").unwrap();
        assert_eq!(r, vec!["a  cd".to_string()]);
    }

    #[test]
    fn test_split_filename_single3() {
        let r = split_filename("a-c").unwrap();
        assert_eq!(r, vec!["a-c".to_string()]);
    }

    #[test]
    fn test_split_filename_single4() {
        let r = split_filename("abcd/").unwrap();
        assert_eq!(r, vec!["abcd".to_string()]);
    }
}
