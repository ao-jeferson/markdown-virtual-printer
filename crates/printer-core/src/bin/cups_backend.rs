use printer_core::ipc::{read_response, write_request, JobRequest, MAX_JOB_BYTES};
use std::{
    fs::File,
    io::{self, Read, Write},
    os::unix::net::UnixStream,
    path::PathBuf,
    process::Command,
    time::Duration,
};

fn main() {
    if let Err(error) = run() {
        eprintln!("ERROR: Markdown Virtual Printer: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        println!("direct mdvp:/ \"Markdown Virtual Printer\" \"Save a print job as Markdown\"");
        return Ok(());
    }
    if args.len() < 6 {
        return Err("CUPS supplied an incomplete backend job argument list".into());
    }
    let username = &args[2];
    let title = &args[3];
    let options = &args[5];
    let uid = user_id(username)?;
    let mode = if options
        .split_whitespace()
        .any(|o| o.eq_ignore_ascii_case("KeepFormatting=False"))
    {
        1
    } else {
        0
    };

    let mut data = Vec::new();
    if let Some(filename) = args.get(6).filter(|f| !f.is_empty() && f.as_str() != "-") {
        File::open(filename)?
            .take((MAX_JOB_BYTES + 1) as u64)
            .read_to_end(&mut data)?;
    } else {
        io::stdin()
            .take((MAX_JOB_BYTES + 1) as u64)
            .read_to_end(&mut data)?;
    }
    if data.len() > MAX_JOB_BYTES {
        return Err("print job exceeds the 256 MiB limit".into());
    }
    if data.is_empty() {
        return Err("print job is empty".into());
    }

    let socket = socket_path(uid);
    let mut stream = UnixStream::connect(&socket).map_err(|error| {
        format!("cannot reach the user-session save dialog ({error}); start Markdown Virtual Printer and retry")
    })?;
    stream.set_read_timeout(Some(Duration::from_secs(15 * 60)))?;
    stream.set_write_timeout(Some(Duration::from_secs(2 * 60)))?;
    write_request(
        &mut stream,
        &JobRequest {
            content_type: "application/pdf".into(),
            title: title.clone(),
            mode,
            data,
        },
    )?;
    stream.flush()?;
    read_response(&mut stream)?;
    eprintln!("INFO: Markdown Virtual Printer saved the print job.");
    Ok(())
}

fn user_id(username: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let output = Command::new("id").args(["-u", username]).output()?;
    if !output.status.success() {
        return Err(format!("could not resolve printing user {username:?}").into());
    }
    Ok(String::from_utf8(output.stdout)?.trim().parse()?)
}

fn socket_path(uid: u32) -> PathBuf {
    #[cfg(target_os = "macos")]
    let dir = "/private/tmp";
    #[cfg(not(target_os = "macos"))]
    let dir = "/tmp";
    PathBuf::from(dir).join(format!("mdvp-{uid}.sock"))
}
