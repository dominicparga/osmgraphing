//------------------------------------------------------------------------------------------------//
// other modules

use std::time::SystemTime;
use std::{fs, io, path};

//------------------------------------------------------------------------------------------------//
// own modules

use super::model::EdgeInfo;

//------------------------------------------------------------------------------------------------//

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
            Err(_) => return Err(format!("Could not open file {}", path.display())),
        }
    };

    Ok(())
}

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
                "Problem with path {}: {}",
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

    // get reader
    let reader = {
        let file = match fs::File::open(file_path) {
            Ok(file) => file,
            Err(_) => return Err(format!("No such file {}", file_path.display())),
        };
        let reader = io::BufReader::new(file);
        csv::Reader::from_reader(reader)
    };

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

pub fn export_statistics<P: AsRef<path::Path> + ?Sized>(
    mut data: Vec<Option<EdgeInfo>>,
    out_file_path: &P,
) -> Result<(), String> {
    let out_file_path = out_file_path.as_ref();

    // file should have been created
    let mut writer = {
        let out_file = match fs::File::create(out_file_path) {
            Ok(file) => file,
            Err(_) => return Err(format!("Could not open file {}", out_file_path.display())),
        };
        let writer = io::BufWriter::new(out_file);
        csv::Writer::from_writer(writer)
    };

    // remove None's from data
    data.retain(|ei| ei.is_some());

    // write head-line to csv-file
    {
        let result = writer.write_record(EdgeInfo::head_line());
        if let Err(e) = result {
            return Err(format!("Could not write record to csv-file due to {}", e));
        }
    }
    // write data to csv-file
    {
        for edge_info in data {
            let result = writer.serialize(edge_info);
            if let Err(e) = result {
                return Err(format!("Could not write data to csv-file due to {}", e));
            }
        }
    }

    // csv-writer needs explicit flush
    // https://rust-lang-nursery.github.io/rust-cookbook/encoding/csv.html#serialize-records-to-csv
    if writer.flush().is_err() {
        Err(format!(
            "Could not flush csv-writer of file {}",
            out_file_path.display()
        ))
    } else {
        Ok(())
    }
}
