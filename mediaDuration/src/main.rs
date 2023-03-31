// use mp4::{Result};
use std::fs::File;
// use std::fs;
use std::io::{BufReader};
use walkdir::{DirEntry, WalkDir};

fn main() {
    let mut durations = Vec::new();
    for entry in WalkDir::new("/Users/godlikeonline/Downloads/react")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() && e.path().extension().unwrap_or_default() == "mp4")
    {
        let duration = get_duration(&entry);
        durations.push((entry.path().to_owned(), duration));
    }

    for (path, duration) in durations {
        let mut filename = path.file_stem().unwrap().to_string_lossy().to_string();
        let pos = filename.rfind('(');
        if let Some(index) = pos {
            filename.truncate(index);
            filename = filename.trim().to_string();
        }
        let new_path = format!(
            "{}({})",
            filename,
            duration.as_secs() / 60
        );
        let new_path = path.with_file_name(new_path + ".mp4");
        std::fs::rename(path, new_path).unwrap();
    }
}

// // 去掉mp4文件名最后的括号及其内容
// fn get_filename(filename: String) -> String {
//     let mut filename = entry.file_name().to_string_lossy().to_string();
//     let pos = filename.rfind('(');
//     if let Some(index) = pos {
//         filename.truncate(index);
//         filename = filename.trim().to_string();
//     }
//     filename
// }


fn get_duration(entry: &DirEntry) -> std::time::Duration {
    let f = File::open(entry.path()).unwrap();
    // let metadata = fs::metadata("/Users/godlikeonline/Downloads/react/第06章 完成节点任务/6-8 renderRoot中对于错误的处理.mp4");
    let size = match f.metadata() {
        Ok(media) => media.len(),
        Err(err) => {
            eprintln!("Error: {}", err);
            return std::time::Duration::from_secs(0);
        }
    };
    let reader = BufReader::new(f);
    let mp4 = mp4::Mp4Reader::read_header(reader, size).unwrap();
    // let duration = mp4.duration().as_secs() / 60;

    // 返回分钟
    mp4.duration()
    // print!("{:?}", len);
    // let f = File::open("tests/samples/minimal.mp4").unwrap();
    // let metadata = f.metadata()?;
    // let metadata = fs::metadata("tests/samples/minimal.mp4");
    // let size = metadata?.len();
    // let reader = BufReader::new(f);

    // let mp4 = mp4::Mp4Reader::read_header(reader, size)?;
    // mp4.duration()
    // let file = File::open(entry.path()).unwrap();
    // let size = file.metadata().len();
    // let reader = BufReader::new(file);

    // let mp4 = mp4::Mp4Reader::read_header(reader, size);
    // let duration = mp4.duration();
    // duration
}