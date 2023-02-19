pub enum FileType {
    File,
    Dir
}

struct File {
    filename: String,
    data: Vec<u8>
}

pub struct Directory {
    filename: String,
    files: Vec<File>,
    directories: Vec<Directory>
}

impl File {
    fn new(filename: String) -> Self {
        Self {
            filename,
            data: vec![]
        }
    }
}

impl Directory {
    fn new(filename: String) -> Self {
        Self {
            filename,
            files: vec![],
            directories: vec![]
        }
    }

    pub fn file_exists(&self, filename: &str) -> bool {
        unimplemented!()
    }

    pub fn file_type(&self, filename: &str) -> FileType {
        unimplemented!()
    }

    pub fn ls(&self, filename: &str) -> Result<Vec<String>, ()> {
        unimplemented!()
    }

    pub fn mkdir(&mut self, filename: &str) -> Result<(), ()> {
        unimplemented!()
    }

    pub fn touch(&mut self, filename: &str) {
        unimplemented!()
    }

    pub fn write(&mut self, filename: &str, contents: &Vec<u8>) -> Result<(), ()> {
        unimplemented!()
    }

    pub fn read(&self, filename: &str) -> Result<Vec<u8>, ()> {
        unimplemented!()
    }

    pub fn cat(&mut self, filename: &str, contents: &Vec<u8>) -> Result<(), ()> {
        unimplemented!()
    }

    pub fn mv(&mut self, source: &str, dest: &str) -> Result<(), ()> {
        unimplemented!()
    }

    pub fn cp(&mut self, source: &str, dest: &str) -> Result<(), ()> {
        unimplemented!()
    }
}

const ROOT_FILENAME: &str= "";

pub fn create_root() -> Directory {
    Directory {
        filename: String::from(ROOT_FILENAME),
        files: vec![],
        directories: vec![]
    }
}