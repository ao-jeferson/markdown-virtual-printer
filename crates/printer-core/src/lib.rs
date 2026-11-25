//! Shared Rust core for the Markdown Virtual Printer.
//!
//! Print adapters pass the system-spooled document here. PDF text extraction is
//! local. Formatting is necessarily approximate when the operating system has
//! already flattened the original document into a print description.

use std::fmt;

pub mod ipc;

const MAX_INPUT_BYTES: usize = 256 * 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputMode {
    /// Organize detected structure as readable Markdown.
    Formatted,
    /// Preserve extracted characters as Markdown source without reformatting.
    MarkdownSource,
}

#[derive(Debug)]
pub enum ConvertError {
    InputTooLarge,
    InvalidUtf8,
    UnsupportedFormat(String),
    Pdf(String),
    NoExtractableText,
}

impl fmt::Display for ConvertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InputTooLarge => write!(f, "print job exceeds the 256 MiB processing limit"),
            Self::InvalidUtf8 => write!(f, "text print job is not valid UTF-8"),
            Self::UnsupportedFormat(kind) => write!(f, "unsupported print format: {kind}"),
            Self::Pdf(reason) => write!(f, "could not extract text from PDF: {reason}"),
            Self::NoExtractableText => write!(
                f,
                "the print job contains no extractable text; scanned jobs need OCR"
            ),
        }
    }
}

impl std::error::Error for ConvertError {}

/// Convert a text or PDF print job to Markdown. The function performs no file
/// writes and makes no network requests; adapters own the save dialog.
pub fn convert_print_job(
    data: &[u8],
    content_type: &str,
    title: Option<&str>,
    mode: OutputMode,
) -> Result<String, ConvertError> {
    if data.len() > MAX_INPUT_BYTES {
        return Err(ConvertError::InputTooLarge);
    }

    let kind = content_type
        .split(';')
        .next()
        .unwrap_or(content_type)
        .trim();
    let extracted = if kind.eq_ignore_ascii_case("application/pdf") || data.starts_with(b"%PDF-") {
        let pages = pdf_extract::extract_text_from_mem_by_pages(data)
            .map_err(|e| ConvertError::Pdf(e.to_string()))?;
        let text = pages
            .into_iter()
            .map(|p| p.trim().to_owned())
            .collect::<Vec<_>>()
            .join("\n\n");
        if text.trim().is_empty() {
            return Err(ConvertError::NoExtractableText);
        }
        text
    } else if kind.eq_ignore_ascii_case("text/plain")
        || kind.eq_ignore_ascii_case("text/markdown")
        || kind.eq_ignore_ascii_case("application/octet-stream")
    {
        std::str::from_utf8(data)
            .map_err(|_| ConvertError::InvalidUtf8)?
            .to_owned()
    } else {
        return Err(ConvertError::UnsupportedFormat(kind.to_owned()));
    };

    Ok(match mode {
        OutputMode::MarkdownSource => extracted.trim().to_owned(),
        OutputMode::Formatted => format_as_markdown(&extracted, title),
    })
}

/// Format extracted text as a readable Markdown report. It recognizes explicit
/// Markdown headings, lists, quotes, and code fences, and joins wrapped PDF
/// lines inside paragraphs. It cannot recover visual styling absent from PDL.
pub fn format_as_markdown(text: &str, title: Option<&str>) -> String {
    let normalized = text.replace("\r\n", "\n").replace('\r', "\n");
    let mut out = String::new();
    if let Some(title) = title.map(str::trim).filter(|s| !s.is_empty()) {
        out.push_str("# ");
        out.push_str(title);
        out.push_str("\n\n");
    }

    let mut paragraph = Vec::<String>::new();
    let mut list = Vec::<String>::new();
    let mut in_code = false;

    fn flush_paragraph(out: &mut String, lines: &mut Vec<String>) {
        if !lines.is_empty() {
            if !out.is_empty() && !out.ends_with("\n\n") {
                out.push_str("\n\n");
            }
            out.push_str(&lines.join(" "));
            lines.clear();
        }
    }
    fn flush_list(out: &mut String, items: &mut Vec<String>) {
        if !items.is_empty() {
            if !out.is_empty() && !out.ends_with("\n\n") {
                out.push_str("\n\n");
            }
            out.push_str(&items.join("\n"));
            items.clear();
        }
    }

    for raw in normalized.lines() {
        let line = raw.trim();
        if line.starts_with("```") || line.starts_with("~~~") {
            flush_paragraph(&mut out, &mut paragraph);
            flush_list(&mut out, &mut list);
            if !out.is_empty() && !out.ends_with("\n\n") {
                out.push_str("\n\n");
            }
            out.push_str(line);
            out.push('\n');
            in_code = !in_code;
            continue;
        }
        if in_code {
            out.push_str(raw.trim_end());
            out.push('\n');
            continue;
        }
        if line.is_empty() {
            flush_paragraph(&mut out, &mut paragraph);
            flush_list(&mut out, &mut list);
            continue;
        }
        if is_list_item(line) {
            flush_paragraph(&mut out, &mut paragraph);
            list.push(line.to_owned());
            continue;
        }
        flush_list(&mut out, &mut list);
        if line.starts_with('#') && line.chars().take_while(|c| *c == '#').count() <= 6 {
            flush_paragraph(&mut out, &mut paragraph);
            if !out.is_empty() && !out.ends_with("\n\n") {
                out.push_str("\n\n");
            }
            out.push_str(line);
            continue;
        }
        if line.starts_with("> ") || line == ">" || line == "---" || line == "***" {
            flush_paragraph(&mut out, &mut paragraph);
            if !out.is_empty() && !out.ends_with("\n\n") {
                out.push_str("\n\n");
            }
            out.push_str(line);
            continue;
        }
        if looks_like_heading(line) {
            flush_paragraph(&mut out, &mut paragraph);
            if !out.is_empty() && !out.ends_with("\n\n") {
                out.push_str("\n\n");
            }
            out.push_str("## ");
            out.push_str(line.trim_end_matches(':'));
            continue;
        }
        paragraph.push(line.to_owned());
    }
    flush_paragraph(&mut out, &mut paragraph);
    flush_list(&mut out, &mut list);
    out.trim().to_owned()
}

fn is_list_item(line: &str) -> bool {
    let bytes = line.as_bytes();
    (line.starts_with("- ") || line.starts_with("* ") || line.starts_with("+ "))
        || bytes.iter().take_while(|b| b.is_ascii_digit()).count() > 0
            && bytes.iter().filter(|b| **b == b'.').count() > 0
            && line.find(". ").is_some()
}

fn looks_like_heading(line: &str) -> bool {
    line.len() <= 72
        && line.chars().any(char::is_alphabetic)
        && line
            .chars()
            .filter(|c| c.is_alphabetic())
            .all(char::is_uppercase)
}

/// C ABI used by native front ends. Call once with a null output buffer to get
/// the required byte count, then again with an output buffer of that size.
/// Returns a negative value on invalid pointers, UTF-8, or mode.
///
/// # Safety
/// Non-null pointers must reference readable/writable allocations of their
/// declared lengths for the duration of the call. `output` must not overlap
/// the input allocation.
#[no_mangle]
pub unsafe extern "C" fn mdvp_format_text(
    input: *const u8,
    input_len: usize,
    mode: u32,
    output: *mut u8,
    output_capacity: usize,
) -> isize {
    if input_len > MAX_INPUT_BYTES || (input_len > 0 && input.is_null()) {
        return -1;
    }
    let bytes = if input_len == 0 {
        &[]
    } else {
        std::slice::from_raw_parts(input, input_len)
    };
    let Ok(text) = std::str::from_utf8(bytes) else {
        return -2;
    };
    let formatted = match mode {
        0 => format_as_markdown(text, None),
        1 => text.trim().to_owned(),
        _ => return -3,
    };
    if output.is_null() || output_capacity == 0 {
        return formatted.len() as isize;
    }
    if output_capacity < formatted.len() {
        return -(formatted.len() as isize);
    }
    std::ptr::copy_nonoverlapping(formatted.as_ptr(), output, formatted.len());
    formatted.len() as isize
}

/// C ABI for platform print adapters. Pass MIME/title as UTF-8 slices and call
/// once with a null output to query the required size, then with a buffer.
///
/// # Safety
/// Non-null pointers must reference readable/writable allocations of their
/// declared lengths for the duration of the call. `output` must not overlap
/// any input allocation.
#[no_mangle]
pub unsafe extern "C" fn mdvp_convert_job(
    input: *const u8,
    input_len: usize,
    content_type: *const u8,
    content_type_len: usize,
    title: *const u8,
    title_len: usize,
    mode: u32,
    output: *mut u8,
    output_capacity: usize,
) -> isize {
    if input_len > MAX_INPUT_BYTES
        || content_type_len > 1024
        || title_len > 64 * 1024
        || (input_len > 0 && input.is_null())
        || (content_type_len > 0 && content_type.is_null())
        || (title_len > 0 && title.is_null())
    {
        return -1;
    }
    let bytes = if input_len == 0 {
        &[]
    } else {
        std::slice::from_raw_parts(input, input_len)
    };
    let mime = if content_type_len == 0 {
        &[]
    } else {
        std::slice::from_raw_parts(content_type, content_type_len)
    };
    let title_bytes = if title_len == 0 {
        &[]
    } else {
        std::slice::from_raw_parts(title, title_len)
    };
    let (Ok(mime), Ok(title)) = (std::str::from_utf8(mime), std::str::from_utf8(title_bytes))
    else {
        return -2;
    };
    let mode = match mode {
        0 => OutputMode::Formatted,
        1 => OutputMode::MarkdownSource,
        _ => return -3,
    };
    let Ok(converted) = convert_print_job(bytes, mime, Some(title), mode) else {
        return -4;
    };
    if output.is_null() || output_capacity == 0 {
        return converted.len() as isize;
    }
    if output_capacity < converted.len() {
        return -(converted.len() as isize);
    }
    std::ptr::copy_nonoverlapping(converted.as_ptr(), output, converted.len());
    converted.len() as isize
}

#[cfg(test)]
mod tests {
    use super::*;

    fn minimal_pdf() -> Vec<u8> {
        let stream = b"BT /F1 12 Tf 72 720 Td (Hello PDF) Tj ET";
        let mut pdf = b"%PDF-1.4\n".to_vec();
        let objects: [&[u8]; 4] = [
            b"<< /Type /Catalog /Pages 2 0 R >>",
            b"<< /Type /Pages /Kids [3 0 R] /Count 1 >>",
            b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Resources << /Font << /F1 4 0 R >> >> /Contents 5 0 R >>",
            b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>",
        ];
        let mut offsets = Vec::new();
        for (index, body) in objects.iter().enumerate() {
            offsets.push(pdf.len());
            pdf.extend_from_slice(format!("{} 0 obj\n", index + 1).as_bytes());
            pdf.extend_from_slice(body);
            pdf.extend_from_slice(b"\nendobj\n");
        }
        offsets.push(pdf.len());
        pdf.extend_from_slice(b"5 0 obj\n");
        pdf.extend_from_slice(format!("<< /Length {} >>\nstream\n", stream.len()).as_bytes());
        pdf.extend_from_slice(stream);
        pdf.extend_from_slice(b"\nendstream\nendobj\n");
        let xref = pdf.len();
        pdf.extend_from_slice(b"xref\n0 6\n0000000000 65535 f \n");
        for offset in offsets {
            pdf.extend_from_slice(format!("{offset:010} 00000 n \n").as_bytes());
        }
        pdf.extend_from_slice(
            format!("trailer\n<< /Size 6 /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n").as_bytes(),
        );
        pdf
    }

    #[test]
    fn formatted_mode_recognizes_lists_and_headings() {
        let result = format_as_markdown(
            "SUMMARY\n\nWrapped line\ncontinues here.\n\n- First\n- Second",
            Some("Print report"),
        );
        assert_eq!(
            result,
            "# Print report\n\n## SUMMARY\n\nWrapped line continues here.\n\n- First\n- Second"
        );
    }

    #[test]
    fn markdown_source_mode_keeps_source_text() {
        let result = convert_print_job(
            b"# Heading\n\n**bold**",
            "text/markdown",
            None,
            OutputMode::MarkdownSource,
        )
        .unwrap();
        assert_eq!(result, "# Heading\n\n**bold**");
    }

    #[test]
    fn text_mode_rejects_invalid_utf8() {
        assert!(matches!(
            convert_print_job(&[0xff], "text/plain", None, OutputMode::Formatted),
            Err(ConvertError::InvalidUtf8)
        ));
    }

    #[test]
    fn text_mode_rejects_unknown_formats() {
        assert!(matches!(
            convert_print_job(b"data", "image/png", None, OutputMode::Formatted),
            Err(ConvertError::UnsupportedFormat(_))
        ));
    }

    #[test]
    fn extracts_text_from_pdf_print_job() {
        let pdf = minimal_pdf();
        let result = convert_print_job(
            &pdf,
            "application/pdf",
            Some("Print"),
            OutputMode::Formatted,
        )
        .unwrap();
        assert!(result.contains("# Print"));
        assert!(result.contains("Hello PDF"));
    }
}
