use std::{
    error::Error,
    fmt::Debug,
    fs::File,
    io::{BufReader, Read},
    path::Path,
    str::FromStr,
};

pub fn read_file<P: AsRef<Path>>(p: P) -> Result<String, Box<dyn Error>> {
    let file = File::open(p)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn parse_raw_data<S, T>(data: S) -> Result<Vec<T>, Box<dyn Error>>
where
    S: AsRef<str>,
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    Ok(data
        .as_ref()
        .lines()
        .map(str::parse)
        .map(|r| r.unwrap())
        .collect())
}

pub fn parse_file<P, T>(p: P) -> Result<Vec<T>, Box<dyn Error>>
where
    P: AsRef<Path>,
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let contents = read_file(p)?;

    parse_raw_data(contents)
}
