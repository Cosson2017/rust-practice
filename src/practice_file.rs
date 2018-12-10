use std::path::{Path};
use std::fs;
use std::iter::Iterator;

use failure::Error;

fn test_file() {
    let path = &".";
    let mut count = 0;
    get_sub_dir(path).unwrap().iter().for_each(|p| {
        get_sub_file(p).unwrap().iter().for_each(|f|{
            read_file(f, |content| -> Result<(), Error> {
                count += 1;
                Ok(())
            }).unwrap();
        })
    });
    println!("file count {}", count);
}

// 是否是目录
fn is_dir(p: impl AsRef<Path>) -> Result<bool, Error> {
    let md = fs::metadata(p)?;
    Ok(md.is_dir())
}

// 读取文件
fn read_file(p: impl AsRef<Path>, mut f: impl FnMut(String) -> Result<(), Error>) -> Result<(), Error> {
    let cont = fs::read_to_string(p)?;
    f(cont)
}

// 获取子文件
fn get_sub_file(p: impl AsRef<Path>) -> Result<Vec<impl AsRef<Path>>, Error> {
    let dir = fs::read_dir(p)?;
    let mut vec = Vec::new();
    for entry in dir {
        let path = entry?.path();
        if is_dir(&path)? {
            continue;
        }

        vec.push(path);
    }
    Ok(vec)
}

// 获取子目录
fn get_sub_dir(p: impl AsRef<Path>) -> Result<Vec<impl AsRef<Path>>, Error> {
    let dir = fs::read_dir(p)?;
    let mut vec = Vec::new();
    for entry in dir {
        let path = entry?.path();
        if is_dir(&path)? {
            vec.push(path);
        }
    }
    Ok(vec)
}
