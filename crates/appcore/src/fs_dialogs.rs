//! Native file-dialog adapter: the only OS-facing module in the core.

use std::path::PathBuf;

pub fn pick_open(last_dir: Option<&PathBuf>) -> Option<PathBuf> {
    let mut d = rfd::FileDialog::new()
        .set_title("Open Sequence")
        .add_filter(
            "All supported",
            &[
                "hbx", "gb", "gbk", "genbank", "gp", "fa", "fasta", "fna", "faa", "txt", "seq",
            ],
        )
        .add_filter("HelixBench", &["hbx"])
        .add_filter("GenBank", &["gb", "gbk", "genbank", "gp"])
        .add_filter("FASTA", &["fa", "fasta", "fna", "faa"]);
    if let Some(dir) = last_dir {
        d = d.set_directory(dir);
    }
    d.pick_file()
}

pub fn pick_save(
    title: &str,
    label: &str,
    exts: &[&str],
    default_name: &str,
    last_dir: Option<&PathBuf>,
) -> Option<PathBuf> {
    let mut d = rfd::FileDialog::new()
        .set_title(title)
        .add_filter(label, exts)
        .set_file_name(default_name);
    if let Some(dir) = last_dir {
        d = d.set_directory(dir);
    }
    d.save_file()
}

/// Pick a FASTA file to use as a BLAST database.
pub fn pick_fasta(last_dir: Option<&PathBuf>) -> Option<PathBuf> {
    let mut d = rfd::FileDialog::new()
        .set_title("Load BLAST Database (FASTA)")
        .add_filter("FASTA", &["fa", "fasta", "fna", "faa", "txt"]);
    if let Some(dir) = last_dir {
        d = d.set_directory(dir);
    }
    d.pick_file()
}

/// Pick a CSV file to use as automation pipeline input.
pub fn pick_csv(last_dir: Option<&PathBuf>) -> Option<PathBuf> {
    let mut d = rfd::FileDialog::new()
        .set_title("Load CSV (automation input)")
        .add_filter("CSV", &["csv", "txt"]);
    if let Some(dir) = last_dir {
        d = d.set_directory(dir);
    }
    d.pick_file()
}

/// Pick a directory (for bulk export).
pub fn pick_folder() -> Option<PathBuf> {
    rfd::FileDialog::new()
        .set_title("Export to folder")
        .pick_folder()
}
