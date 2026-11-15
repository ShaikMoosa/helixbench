# Scientific and input-validation contracts

## Alphabet handling

Validate source molecule before constructing a typed sequence. Retain original bytes/text, detected type, user-confirmed type and transformation report. DNA IUPAC symbols remain supported. U→T, gap removal, unsupported-residue deletion and format annotation loss require explicit review; unsupported molecule/type combinations remain recoverably blocked.

## Feature coordinates

For UI start s and length n, internal start=s−1 and len=n. An inclusive UI end e gives len=e−s+1. Validate linear end ≤ sequence length. Circular wrap uses a normalized start and a length no greater than the allowed whole-molecule span. Feature-segment order and strand are preserved.

## Primer/PCR

Keep existing hybridization, Tm, candidate ranking and secondary-structure calculations. A 5′ overhang is not a template-binding body. Report primer IDs, orientation, binding sites and parameter assumptions with each product; copy annotations only when requested and coordinate mapping is defined.

## Digest accounting

Use domain digest outputs. For an uncut linear 12-base input ATGCGTACGTAA with EcoRI/BamHI, the expected diagnostic is no sites and one uncut 12-base molecule. Circular and linear cut cases use their respective fragment accounting; do not substitute the 18/3003 illustrative Figma fixture for computed data.

## Alignment/assembly

Preserve progressive MSA, pair alignment, circular reindex mapping, strategy-specific assembly, junction primer design, Golden Gate overhang checks and combinatorial caps. Show unsupported/invalid combinations as parameter errors or no-result diagnostics with original inputs retained.

## CRISPR/codon/BLAST

Keep PAM/strand scan, per-guide/off-target lists, available scores, codon host/GC/avoidance/harmonization parameters and local database selection. Display the implemented method name/version and scope. A seed-and-extend result is not claimed to be an NCBI BLAST search with equivalent statistical guarantees.

## Protein/chemistry/RNA

Typed input and chain/monomer/compound references are mandatory. Report unavailable properties for unsupported chemistry rather than guessing. Show units for molecular mass, temperature, charge assumptions, titer and concentration. Preserve supported serialization round trips and source type.

## Numeric validation

Parse into validated finite values; report field-local errors. Quantity/duration/volume rules are operation-specific: transfer volume must be positive and not exceed available source; empty wells may have zero quantity; division requires nonzero concentration. Avoid comparing floats for strict equality except discrete/count fields.

## CSV and text export

Handle quoted commas, escaped quotes, embedded line breaks, BOM/Unicode and missing/ragged columns. Export the selected scope and actual units. Offer a clearly described spreadsheet-safe mode if formula neutralization changes text; raw scientific text must not be silently altered. The app never evaluates imported formulas.

## Resource budget

Keep the UI responsive using background work, cancellation and virtualized rendering. Do not introduce an unmeasured hard sequence-size limit that removes existing capability. Proposed default: warn before eager processing of files over 64 MiB, allow a reviewed continue/cancel choice, stream records when possible and handle allocation failure recoverably. Benchmark supported sizes before declaring performance guarantees.

