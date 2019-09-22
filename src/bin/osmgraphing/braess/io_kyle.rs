//------------------------------------------------------------------------------------------------//
// other modules

use std::time::SystemTime;
use std::{fs, io, path};

use osmgraphing::network::Graph;

//------------------------------------------------------------------------------------------------//
// own modules

use super::model::{EdgeInfo, SmallEdgeInfo};

//------------------------------------------------------------------------------------------------//

/// Returns output-path, which is "{dir_path}/{%Y-%m-%d}/{%H:%M:%S}"
pub fn create_datetime_dir<P: AsRef<path::Path> + ?Sized>(
    dir_path: &P,
) -> Result<path::PathBuf, String> {
    // check if necessary directories do already exist
    let out_dir_path = {
        // get and format current time
        let now = SystemTime::now();
        let now: chrono::DateTime<chrono::Utc> = now.into();
        let now_ymd = format!("{}", now.format("%Y-%m-%d"));
        let now_hms = format!("{}", now.format("%T")); // %T == %H:%M:%S

        let dir_path = dir_path.as_ref();
        if !dir_path.exists() {
            return Err(format!("Path {} does not exist.", dir_path.display()));
        }
        dir_path.join(now_ymd).join(now_hms)
    };

    match fs::create_dir_all(&out_dir_path) {
        Ok(_) => (),
        Err(e) => {
            return Err(format!(
                "Problem with path {} due to {}",
                out_dir_path.display(),
                e
            ))
        }
    };

    Ok(out_dir_path)
}

pub fn read_proto_routes<P: AsRef<path::Path> + ?Sized>(
    file_path: &P,
) -> Result<Vec<(i64, i64)>, String> {
    let file_path = file_path.as_ref();
    let reader = open_csv_reader(file_path)?;

    // deserialize, cast and let collect() check for errors
    let proto_routes = match reader.into_deserialize().collect() {
        Ok(v) => v,
        Err(_) => {
            return Err(format!(
                "Could not deserialize file {}",
                file_path.display()
            ))
        }
    };
    Ok(proto_routes)
}

pub fn write_proto_routes<P: AsRef<path::Path> + ?Sized>(
    proto_routes: &Vec<(i64, i64)>,
    file_path: &P,
    appending: bool,
) -> Result<(), String> {
    let file_path = file_path.as_ref();
    let mut writer = open_csv_writer(file_path, appending)?;

    // write data to csv-file
    {
        if !appending {
            // write head-line to csv-file
            let result = writer.write_record(vec!["src-id", "dst-id"]);
            if let Err(e) = result {
                return Err(format!("Could not write record to csv-file due to {}", e));
            }
        }
        for proto_route in proto_routes {
            let result = writer.serialize(proto_route);
            if let Err(e) = result {
                return Err(format!(
                    "Could not write proto-route to csv-file due to {}",
                    e
                ));
            }
        }
    }

    flush_csv_writer(writer)
}

pub fn write_edge_stats<P: AsRef<path::Path> + ?Sized>(
    stats: &Vec<Option<SmallEdgeInfo>>,
    file_path: &P,
    appending: bool,
    graph: &Graph,
) -> Result<(), String> {
    // remove all None-values
    // and parse data into output-format
    let data: Vec<EdgeInfo> = stats
        .into_iter()
        .filter_map(|s| match s {
            Some(small_edge_info) => Some(EdgeInfo::from(&small_edge_info, &graph)),
            None => None,
        })
        .collect();

    let file_path = file_path.as_ref();
    let mut writer = open_csv_writer(file_path, appending)?;

    // write data to csv-file
    {
        for edge_info in data {
            let result = writer.serialize(edge_info);
            if let Err(e) = result {
                return Err(format!("Could not write data to csv-file due to {}", e));
            }
        }
    }

    flush_csv_writer(writer)
}

//------------------------------------------------------------------------------------------------//
// basic io

/// Returns error if the file exists
pub fn create_dir<P: AsRef<path::Path> + ?Sized>(path: &P) -> Result<path::PathBuf, String> {
    let path = path.as_ref();
    if path.exists() {
        return Err(format!(
            "Dir {} does already exist. Please (re)move it.",
            path.display()
        ));
    } else {
        match fs::create_dir(path) {
            Ok(dir) => dir,
            Err(e) => {
                return Err(format!(
                    "Could not open dir {} due to {}",
                    path.display(),
                    e
                ))
            }
        }
    };

    Ok(path.to_path_buf())
}

/// Returns error if the file exists
pub fn create_file<P: AsRef<path::Path> + ?Sized>(path: &P) -> Result<(), String> {
    let path = path.as_ref();
    if path.exists() {
        return Err(format!(
            "File {} does already exist. Please (re)move it.",
            path.display()
        ));
    } else {
        match fs::File::create(path) {
            Ok(file) => file,
            Err(e) => {
                return Err(format!(
                    "Could not open file {} due to {}",
                    path.display(),
                    e
                ))
            }
        }
    };

    Ok(())
}

pub fn open_csv_reader<P: AsRef<path::Path> + ?Sized>(
    file_path: &P,
) -> Result<csv::Reader<io::BufReader<fs::File>>, String> {
    let file_path = file_path.as_ref();

    match fs::File::open(file_path) {
        Ok(file) => {
            let writer = io::BufReader::new(file);
            Ok(csv::Reader::from_reader(writer))
        }
        Err(e) => Err(format!(
            "Could not open file {} due to {}",
            file_path.display(),
            e
        )),
    }
}

pub fn open_csv_writer<P: AsRef<path::Path> + ?Sized>(
    file_path: &P,
    appending: bool,
) -> Result<csv::Writer<io::BufWriter<fs::File>>, String> {
    let file_path = file_path.as_ref();

    match fs::OpenOptions::new()
        .write(true)
        .append(appending)
        .create(false)
        .open(file_path)
    {
        Ok(file) => {
            let writer = io::BufWriter::new(file);
            Ok(csv::Writer::from_writer(writer))
        }
        Err(e) => Err(format!(
            "Could not open file {} due to {}",
            file_path.display(),
            e
        )),
    }
}

pub fn flush_csv_writer(mut writer: csv::Writer<io::BufWriter<fs::File>>) -> Result<(), String> {
    // csv-writer needs explicit flush
    // https://rust-lang-nursery.github.io/rust-cookbook/encoding/csv.html#serialize-records-to-csv
    if let Err(e) = writer.flush() {
        Err(format!("Could not flush csv-writer due to {}", e))
    } else {
        Ok(())
    }
}
