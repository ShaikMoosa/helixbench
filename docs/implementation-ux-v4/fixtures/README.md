# Fixture expectations

These are synthetic inputs for new tests, not existing user records or claims that tests ran.

- dna-linear.fasta: 12 DNA bases, linear. EcoRI/BamHI have no site in this input; completed/no-sites is distinct from failure.
- dna-ambiguous.fasta: retain all 15 IUPAC DNA symbols.
- protein.faa and rna.fasta: molecule review must not silently coerce them into DNA.
- invalid-symbol.fasta: position 4 contains ?, and the original 12-character input remains available for correction.
- multi-and-invalid.fasta: retain two record indexes; explicit partial import can choose the valid first record.
- same-name.fasta: two records remain independently addressable despite equal names.
- annotated.gb: preserve the CDS 1–12 annotation and linear topology when supported.
- quoted.csv: two data rows, quoted comma, escaped quote and an embedded newline; do not count physical lines as records.
- legacy-v1.hbx: read/migrate without automatically overwriting the source; durable IDs survive save/reopen.
- newer-unsupported.hbx: explain unsupported schema version and preserve the file.
- transfer-plan.json: 4 × 10 µL transfers; each source 50→40 µL, each empty destination 0→10 µL. A collision/insufficient-volume variant must leave all sources unchanged.
