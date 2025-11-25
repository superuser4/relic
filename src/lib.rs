use std::{fs, io::{self, Read}, process::exit};

pub fn count_dir(_path: String, ext: &Option<String>) -> Result<u64, io::Error> {
    let mut count: u64 = 0;

    let dir = std::path::Path::new(&_path);
    if dir.is_dir() { 
       let paths = fs::read_dir(_path)?
           .map(|res| res.map(|e| e.path()))
           .collect::<Result<Vec<_>, io::Error>>()?;
       for path in paths {
            let path_as_str: String = String::from(path.to_str().unwrap());

           if std::path::Path::new(&path).is_dir() {
               count += count_dir(path_as_str, &ext)?
           } else {
               count += match count_lines(path_as_str, ext.clone()) {
                   Ok(c) => {c},
                   Err(e) => {
                       eprintln!("Error while trying to count lines: {e}");
                       0
                   },
               }
           }
       }
       Ok(count)
    } else {
        count += count_lines(String::from(dir.to_str().unwrap()), ext.clone())?;
        Ok(count)
    }
}

fn ext_compar(file: &String, ext: String) -> bool {
    let spl: Vec<&str> = file.split('.').collect();
    let last: String = match spl.last() {
        Some(l) => { 
            l.to_string()
        },
        None => {
            return false
        },
    };

    if last == ext {
        return true;
    }
    return false;
}

fn count_lines(_path: String, ext: Option<String>) -> Result<u64, io::Error> {
    match ext {
        Some(ex) => {
            if !ext_compar(&_path, ex) {
                return Ok(0);
            }
        },
        None => {},
    }

    let mut fp: fs::File = match fs::File::open(_path) {
        Ok(f) => {f},
        Err(e) => {return Err(e)},
    };
    let mut contents: String = String::new();
    fp.read_to_string(&mut contents)?;

    let mut count: u64 = 0;
    let mut split_lines: Vec<String> = vec![];
    for line in contents.lines() {
        split_lines.push(String::from(line));
        count += 1;
    }
    Ok(count)
}

