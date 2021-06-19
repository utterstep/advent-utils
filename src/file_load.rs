use std::{
    error::Error,
    fs::File,
    io::{self, BufReader, Read},
    path::Path,
    str::FromStr,
};

pub fn read_file<P: AsRef<Path>>(p: P) -> io::Result<String> {
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
    <T as FromStr>::Err: Error + 'static,
{
    Ok(data
        .as_ref()
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?)
}

pub fn parse_file<P, T>(p: P) -> Result<Vec<T>, Box<dyn Error>>
where
    P: AsRef<Path>,
    T: FromStr<Err = Box<dyn Error>>,
    <T as FromStr>::Err: Error + 'static,
{
    let contents = read_file(p)?;

    parse_raw_data(contents)
}
