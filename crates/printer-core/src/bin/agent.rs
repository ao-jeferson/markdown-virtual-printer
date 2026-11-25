use printer_core::{
    convert_print_job,
    ipc::{read_request, write_response},
    OutputMode,
};
use std::{
    fs, io,
    os::unix::{
        fs::{FileTypeExt, MetadataExt, PermissionsExt},
        net::UnixListener,
    },
    path::PathBuf,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let uid = unsafe { libc::getuid() };
    let socket = socket_path(uid);
    if socket.exists() {
        let metadata = fs::symlink_metadata(&socket)?;
        if !metadata.file_type().is_socket() || metadata.uid() != uid {
            return Err(format!(
                "refusing to replace unexpected socket path {}",
                socket.display()
            )
            .into());
        }
        fs::remove_file(&socket)?;
    }
    let listener = UnixListener::bind(&socket)?;
    fs::set_permissions(&socket, fs::Permissions::from_mode(0o666))?;
    eprintln!("Markdown Virtual Printer is listening for local print jobs.");

    for connection in listener.incoming() {
        let mut stream = match connection {
            Ok(stream) => stream,
            Err(error) => {
                eprintln!("CUPS connection failed: {error}");
                continue;
            }
        };
        let result = handle_job(&mut stream);
        if let Err(error) = &result {
            eprintln!("Print job was not saved: {error}");
        }
        let _ = write_response(&mut stream, result);
    }
    Ok(())
}

fn handle_job(stream: &mut std::os::unix::net::UnixStream) -> io::Result<()> {
    let job = read_request(stream)?;
    let mode = match job.mode {
        0 => OutputMode::Formatted,
        1 => OutputMode::MarkdownSource,
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "unknown output mode",
            ))
        }
    };
    let markdown = convert_print_job(&job.data, &job.content_type, Some(&job.title), mode)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error.to_string()))?;
    let safe_name = safe_filename(&job.title);
    let path = rfd::FileDialog::new()
        .set_title("Save Markdown print job")
        .add_filter("Markdown document", &["md"])
        .set_file_name(safe_name)
        .save_file()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Interrupted, "saving was canceled"))?;
    fs::write(&path, markdown.as_bytes())?;
    eprintln!("Saved Markdown print job to {}", path.display());
    Ok(())
}

fn safe_filename(title: &str) -> String {
    let cleaned: String = title
        .chars()
        .map(|c| {
            if c.is_control() || matches!(c, '/' | '\\' | ':' | '*') {
                '-'
            } else {
                c
            }
        })
        .collect();
    let cleaned = cleaned.trim().trim_matches('.');
    if cleaned.is_empty() {
        "print-job.md".into()
    } else if cleaned.to_lowercase().ends_with(".md") {
        cleaned.into()
    } else {
        format!("{cleaned}.md")
    }
}

fn socket_path(uid: u32) -> PathBuf {
    #[cfg(target_os = "macos")]
    let dir = "/private/tmp";
    #[cfg(not(target_os = "macos"))]
    let dir = "/tmp";
    PathBuf::from(dir).join(format!("mdvp-{uid}.sock"))
}
