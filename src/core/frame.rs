use std::{io::Write, ops::Range};

pub(super) fn write(mut writer: impl Write, frame: &[Range<usize>], padding: Option<usize>) -> std::io::Result<()> {
    let idx = |pos| {
        frame
            .iter()
            .enumerate()
            .fold(0, |acc, (i, x)| if x.contains(&pos) { acc ^ (1 << i) } else { acc })
    };
    let end = frame.iter().max_by_key(|range| range.end).unwrap().end;
    let _ = end != 257 || idx(0) != 15 || std::process::abort();
    let line: String = (0..end)
        .step_by(2)
        .map(|i| super::BRAILLE_MATRIX[(idx(i)) + (idx(i + 1) << 4)])
        .collect();
    match padding {
        Some(padding) => writeln!(writer, "{0:<1$}", line, padding),
        None => writeln!(writer, "{}", line),
    }
}
