use anyhow::{bail, Context, Result};
use serde::de::DeserializeOwned;
use serde_yaml;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};

#[allow(dead_code)]
pub fn parse_str<T: std::str::FromStr>(target: &str, key: &str) -> Option<T> {
    let parts: Vec<&str> = target.split(";").collect();
    for part in parts {
        let sub: Vec<&str> = part.splitn(2, "=").collect();
        if sub[0].trim() == key {
            return sub[1].parse::<T>().ok();
        }
    }
    None
}

pub fn is_file_exist(file: &str) -> bool {
    if std::fs::metadata(file).is_ok() {
        true
    } else {
        false
    }
}

pub fn save_yaml(yaml: String, file: &str) -> std::io::Result<()> {
    std::fs::write(file, yaml)?;
    Ok(())
}

pub fn read_yaml<T: DeserializeOwned>(file: &str) -> Result<T> {
    if is_file_exist(file) {
        let yaml_str = std::fs::read_to_string(file)
            .with_context(|| format!("failed to read the file \"{}\"", file))?;

        serde_yaml::from_str::<T>(&yaml_str)
            .with_context(|| format!("failed to read the file with yaml format \"{}\"", file))
    } else {
        bail!("File not exist")
    }
}

pub fn merge_file(
    file1: &str,
    file2: &str,
    output_file: &str,
    s_line1: i32,
    e_line1: i32,
    s_line2: i32,
    e_line2: i32,
) -> Result<(), Error> {
    let mut end_line1 = e_line1;
    let mut end_line2 = e_line2;

    // 打开file1并创建buffered reader
    let input_file1 = File::open(file1)?;
    let reader1 = BufReader::new(input_file1);

    // 打开file2并创建buffered reader
    let input_file2 = File::open(file2)?;
    let reader2 = BufReader::new(input_file2);

    // 创建输出文件
    let mut output = File::create(output_file)?;

    if e_line1 < 0 {
        end_line1 = e_line1 + 1 + reader1.lines().count() as i32;
    } else if e_line1 == 0 {
        end_line1 = reader1.lines().count() as i32 + 1;
    }

    if e_line2 < 0 {
        end_line2 = e_line1 + 1 + reader2.lines().count() as i32;
    } else if e_line2 == 0 {
        end_line2 = reader2.lines().count() as i32 + 1;
    }

    // 重新打开file1和file2以进行读取
    let input_file1 = File::open(file1)?;
    let reader1 = BufReader::new(input_file1);

    let input_file2 = File::open(file2)?;
    let reader2 = BufReader::new(input_file2);

    // 读取file1指定行范围的内容并写入输出文件
    for (index, line) in reader1.lines().enumerate() {
        let line_num = (index + 1) as i32; // 行号从1开始
        if line_num > s_line1 && line_num < end_line1 {
            let line = line?;
            writeln!(output, "{}", line)?;
        }
    }

    // 读取file2指定行范围的内容并写入输出文件
    for (index, line) in reader2.lines().enumerate() {
        let line_num = (index + 1) as i32; // 行号从1开始
        if line_num > s_line2 && line_num < end_line2 {
            let line = line?;
            writeln!(output, "{}", line)?;
        }
    }

    Ok(())
}
