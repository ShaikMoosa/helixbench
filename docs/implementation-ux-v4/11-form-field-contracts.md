# Form field implementation contracts

Current Rust fields are preserved below. Target validation rules add the UX v4 interaction requirements; an existing String is not evidence of a validated typed input. Figma example values belong to fixtures.

## SearchState

[crates/appcore/src/state.rs:163](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:163)

- **open** · `bool` · line 164: Explicit labeled toggle; default is specified by the owning feature. Persist only when the corresponding action is confirmed.
- **mode** · `SearchMode` · line 165: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **query** · `String` · line 166: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **recents** · `Vec<(SearchMode, String)>` · line 167: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **dna_hits** · `Vec<DnaHit>` · line 168: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **protein_hits** · `Vec<ProteinHit>` · line 169: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **name_hits** · `Vec<NameHit>` · line 170: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **current** · `usize` · line 171: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **error** · `Option<String>` · line 172: System-owned validation/error state; never an editable user field.
- **focus_request** · `bool` · line 173: Explicit labeled toggle; default is specified by the owning feature. Persist only when the corresponding action is confirmed.

## FeatureDraft

[crates/appcore/src/state.rs:199](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:199)

- **id** · `Option<u64>` · line 200: Resolve a stable typed identity from a labeled picker. Validate existence/kind at submit; do not persist a row index as a foreign key.
- **name** · `String` · line 201: Trim for validation; require a nonempty meaningful label where the owning operation creates an object. Preserve original editable draft on error.
- **kind** · `String` · line 202: Use a labeled finite choice with a visible selected value. Validate enum/mode availability; preserve unknown legacy values for review.
- **segments_text** · `String` · line 203: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **direction** · `Direction` · line 204: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **color** · `[u8; 3]` · line 205: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **note** · `String` · line 206: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **product** · `String` · line 207: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **translated** · `bool` · line 208: Explicit labeled toggle; default is specified by the owning feature. Persist only when the corresponding action is confirmed.
- **genetic_code** · `u8` · line 209: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **first_codon_met** · `bool` · line 210: Explicit labeled toggle; default is specified by the owning feature. Persist only when the corresponding action is confirmed.
- **frame_offset** · `usize` · line 211: Validate coordinate/length/frame using the owning tool; UI positions are 1-based inclusive. Reject empty/out-of-range spans and unsupported frame values.
- **error** · `Option<String>` · line 212: System-owned validation/error state; never an editable user field.

## PrimerDraft

[crates/appcore/src/state.rs:216](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:216)

- **id** · `Option<u64>` · line 217: Resolve a stable typed identity from a labeled picker. Validate existence/kind at submit; do not persist a row index as a foreign key.
- **name** · `String` · line 218: Trim for validation; require a nonempty meaningful label where the owning operation creates an object. Preserve original editable draft on error.
- **sequence** · `String` · line 219: Validate the declared molecule/grammar before commit. Preserve original text and report meaningful normalization or unsupported syntax.
- **description** · `String` · line 220: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **color** · `[u8; 3]` · line 221: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **phosphorylated** · `bool` · line 222: Explicit labeled toggle; default is specified by the owning feature. Persist only when the corresponding action is confirmed.
- **error** · `Option<String>` · line 223: System-owned validation/error state; never an editable user field.

## NewSeqState

[crates/appcore/src/state.rs:226](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:226)

- **name** · `String` · line 227: Trim for validation; require a nonempty meaningful label where the owning operation creates an object. Preserve original editable draft on error.
- **text** · `String` · line 228: Validate the declared molecule/grammar before commit. Preserve original text and report meaningful normalization or unsupported syntax.
- **topology** · `Topology` · line 229: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **error** · `Option<String>` · line 230: System-owned validation/error state; never an editable user field.

## ColorDraft

[crates/appcore/src/state.rs:312](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:312)

- **range** · `Range` · line 313: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **color** · `[u8; 3]` · line 314: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **strands** · `StrandMask` · line 315: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.

## PcrDraft

[crates/appcore/src/state.rs:327](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:327)

- **fwd** · `String` · line 328: Current primer input is a string. The target resolves selected primer IDs to their explicit 5′→3′ sequence; never pass a display label such as T7-F as sequence bases.
- **rev** · `String` · line 329: Current primer input is a string. The target resolves selected primer IDs to their explicit 5′→3′ sequence; never pass a display label such as T7-F as sequence bases.
- **copy_features** · `bool` · line 330: Explicit labeled toggle; default is specified by the owning feature. Persist only when the corresponding action is confirmed.
- **error** · `Option<String>` · line 331: System-owned validation/error state; never an editable user field.

## PrimerDesignDraft

[crates/appcore/src/state.rs:335](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:335)

- **start** · `String` · line 336: Validate coordinate/length/frame using the owning tool; UI positions are 1-based inclusive. Reject empty/out-of-range spans and unsupported frame values.
- **len** · `String` · line 337: Validate coordinate/length/frame using the owning tool; UI positions are 1-based inclusive. Reject empty/out-of-range spans and unsupported frame values.
- **target_tm** · `String` · line 338: Parse finite numeric input, display units/range where relevant and apply domain-specific bounds. Do not silently substitute a default after invalid input.
- **overhang** · `String` · line 339: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **fwd** · `Vec<domain::primer_design::Candidate>` · line 340: Computed preview/output state. Clear or mark stale when relevant input changes; never commit an unrelated result.
- **rev** · `Vec<domain::primer_design::Candidate>` · line 341: Computed preview/output state. Clear or mark stale when relevant input changes; never commit an unrelated result.
- **fwd_pick** · `usize` · line 342: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **rev_pick** · `usize` · line 343: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **error** · `Option<String>` · line 344: System-owned validation/error state; never an editable user field.

## AlignmentDraft

[crates/appcore/src/state.rs:348](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:348)

- **reference** · `usize` · line 350: Resolve a stable typed identity from a labeled picker. Validate existence/kind at submit; do not persist a row index as a foreign key.
- **reindex** · `bool` · line 351: Explicit labeled toggle; default is specified by the owning feature. Persist only when the corresponding action is confirmed.
- **result** · `Option<domain::align::Alignment>` · line 352: Computed preview/output state. Clear or mark stale when relevant input changes; never commit an unrelated result.
- **labels** · `Vec<String>` · line 354: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **error** · `Option<String>` · line 355: System-owned validation/error state; never an editable user field.

## AssemblyDraft

[crates/appcore/src/state.rs:359](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:359)

- **strategy** · `usize` · line 360: Use a labeled finite choice with a visible selected value. Validate enum/mode availability; preserve unknown legacy values for review.
- **cut** · `String` · line 361: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **inserts** · `Vec<bool>` · line 362: Selection state must be backed by stable IDs before mutation; positions alone are not command identity.
- **primers** · `Vec<domain::assembly::JunctionPrimers>` · line 363: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **gg_issues** · `Vec<String>` · line 364: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **error** · `Option<String>` · line 365: System-owned validation/error state; never an editable user field.

## CrisprDraft

[crates/appcore/src/state.rs:369](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:369)

- **limit** · `String` · line 370: Parse finite numeric input, display units/range where relevant and apply domain-specific bounds. Do not silently substitute a default after invalid input.
- **guides** · `Vec<domain::crispr::Guide>` · line 371: Computed preview/output state. Clear or mark stale when relevant input changes; never commit an unrelated result.
- **off** · `Vec<domain::crispr::OffTarget>` · line 372: Computed preview/output state. Clear or mark stale when relevant input changes; never commit an unrelated result.
- **error** · `Option<String>` · line 373: System-owned validation/error state; never an editable user field.

## CodonDraft

[crates/appcore/src/state.rs:377](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:377)

- **host** · `usize` · line 378: Use a labeled finite choice with a visible selected value. Validate enum/mode availability; preserve unknown legacy values for review.
- **gc_min** · `String` · line 379: Parse finite numeric input, display units/range where relevant and apply domain-specific bounds. Do not silently substitute a default after invalid input.
- **gc_max** · `String` · line 380: Parse finite numeric input, display units/range where relevant and apply domain-specific bounds. Do not silently substitute a default after invalid input.
- **avoid** · `String` · line 381: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **harmonize** · `bool` · line 382: Explicit labeled toggle; default is specified by the owning feature. Persist only when the corresponding action is confirmed.
- **result** · `Option<Vec<u8>>` · line 383: Computed preview/output state. Clear or mark stale when relevant input changes; never commit an unrelated result.
- **cai_before** · `f32` · line 384: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **cai_after** · `f32` · line 385: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **error** · `Option<String>` · line 386: System-owned validation/error state; never an editable user field.

## BlastDraft

[crates/appcore/src/state.rs:390](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:390)

- **k** · `String` · line 391: Parse finite numeric input, display units/range where relevant and apply domain-specific bounds. Do not silently substitute a default after invalid input.
- **max** · `String` · line 392: Parse finite numeric input, display units/range where relevant and apply domain-specific bounds. Do not silently substitute a default after invalid input.
- **db_count** · `usize` · line 393: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **hits** · `Vec<domain::blast::Hit>` · line 394: Computed preview/output state. Clear or mark stale when relevant input changes; never commit an unrelated result.
- **error** · `Option<String>` · line 395: System-owned validation/error state; never an editable user field.

## ProteinDraft

[crates/appcore/src/state.rs:399](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:399)

- **stats** · `Option<domain::protein::ProteinStats>` · line 400: Computed preview/output state. Clear or mark stale when relevant input changes; never commit an unrelated result.
- **align** · `Option<(Vec<u8>, Vec<u8>, i32)>` · line 401: Computed preview/output state. Clear or mark stale when relevant input changes; never commit an unrelated result.
- **error** · `Option<String>` · line 402: System-owned validation/error state; never an editable user field.

## FeatureLibDraft

[crates/appcore/src/state.rs:406](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:406)

- **name** · `String` · line 407: Trim for validation; require a nonempty meaningful label where the owning operation creates an object. Preserve original editable draft on error.
- **kind** · `String` · line 408: Use a labeled finite choice with a visible selected value. Validate enum/mode availability; preserve unknown legacy values for review.
- **motif** · `String` · line 409: Validate the declared molecule/grammar before commit. Preserve original text and report meaningful normalization or unsupported syntax.
- **error** · `Option<String>` · line 410: System-owned validation/error state; never an editable user field.

## WorkspaceDraft

[crates/appcore/src/state.rs:414](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:414)

- **selected** · `Vec<bool>` · line 415: Selection state must be backed by stable IDs before mutation; positions alone are not command identity.
- **tag** · `String` · line 416: Trim for validation; require a nonempty meaningful label where the owning operation creates an object. Preserve original editable draft on error.
- **error** · `Option<String>` · line 417: System-owned validation/error state; never an editable user field.

## NotebookDraft

[crates/appcore/src/state.rs:421](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:421)

- **filter** · `usize` · line 422: Use a labeled finite choice with a visible selected value. Validate enum/mode availability; preserve unknown legacy values for review.
- **error** · `Option<String>` · line 423: System-owned validation/error state; never an editable user field.

## EntryEditDraft

[crates/appcore/src/state.rs:427](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:427)

- **id** · `Option<u64>` · line 428: Resolve a stable typed identity from a labeled picker. Validate existence/kind at submit; do not persist a row index as a foreign key.
- **title** · `String` · line 429: Trim for validation; require a nonempty meaningful label where the owning operation creates an object. Preserve original editable draft on error.
- **project** · `Option<u64>` · line 430: Resolve a stable typed identity from a labeled picker. Validate existence/kind at submit; do not persist a row index as a foreign key.
- **template** · `usize` · line 431: Use a labeled finite choice with a visible selected value. Validate enum/mode availability; preserve unknown legacy values for review.
- **sections** · `Vec<(String, String)>` · line 432: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **tables** · `Vec<domain::notebook::ResultTable>` · line 433: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **error** · `Option<String>` · line 434: System-owned validation/error state; never an editable user field.

## ProjectsDraft

[crates/appcore/src/state.rs:438](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:438)

- **name** · `String` · line 439: Trim for validation; require a nonempty meaningful label where the owning operation creates an object. Preserve original editable draft on error.
- **parent** · `Option<u64>` · line 440: Resolve a stable typed identity from a labeled picker. Validate existence/kind at submit; do not persist a row index as a foreign key.
- **error** · `Option<String>` · line 441: System-owned validation/error state; never an editable user field.

## AuditDraft

[crates/appcore/src/state.rs:445](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:445)

- **filter** · `String` · line 446: Use a labeled finite choice with a visible selected value. Validate enum/mode availability; preserve unknown legacy values for review.

## HomeDraft

[crates/appcore/src/state.rs:450](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:450)


## EntityMapDraft

[crates/appcore/src/state.rs:453](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:453)


## BiologicsDraft

[crates/appcore/src/state.rs:458](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:458)

- **name** · `String` · line 459: Trim for validation; require a nonempty meaningful label where the owning operation creates an object. Preserve original editable draft on error.
- **format** · `usize` · line 460: Use a labeled finite choice with a visible selected value. Validate enum/mode availability; preserve unknown legacy values for review.
- **numbering** · `usize` · line 461: Use a labeled finite choice with a visible selected value. Validate enum/mode availability; preserve unknown legacy values for review.
- **seqs** · `Vec<String>` · line 462: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **error** · `Option<String>` · line 463: System-owned validation/error state; never an editable user field.

## ChemAddDraft

[crates/appcore/src/state.rs:468](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:468)

- **name** · `String` · line 469: Trim for validation; require a nonempty meaningful label where the owning operation creates an object. Preserve original editable draft on error.
- **smiles** · `String` · line 470: Validate the declared molecule/grammar before commit. Preserve original text and report meaningful normalization or unsupported syntax.
- **error** · `Option<String>` · line 471: System-owned validation/error state; never an editable user field.

## ChemReactionDraft

[crates/appcore/src/state.rs:476](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:476)

- **name** · `String` · line 477: Trim for validation; require a nonempty meaningful label where the owning operation creates an object. Preserve original editable draft on error.
- **reactant** · `usize` · line 478: Resolve a stable typed identity from a labeled picker. Validate existence/kind at submit; do not persist a row index as a foreign key.
- **product** · `usize` · line 479: Resolve a stable typed identity from a labeled picker. Validate existence/kind at submit; do not persist a row index as a foreign key.
- **req** · `String` · line 480: Parse finite numeric input, display units/range where relevant and apply domain-specific bounds. Do not silently substitute a default after invalid input.
- **peq** · `String` · line 481: Parse finite numeric input, display units/range where relevant and apply domain-specific bounds. Do not silently substitute a default after invalid input.
- **error** · `Option<String>` · line 482: System-owned validation/error state; never an editable user field.

## ChemLotDraft

[crates/appcore/src/state.rs:487](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:487)

- **form** · `usize` · line 488: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **quantity** · `String` · line 489: Parse finite numeric input, display units/range where relevant and apply domain-specific bounds. Do not silently substitute a default after invalid input.
- **unit** · `String` · line 490: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **error** · `Option<String>` · line 491: System-owned validation/error state; never an editable user field.

## RnaDraft

[crates/appcore/src/state.rs:497](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:497)

- **name** · `String` · line 498: Trim for validation; require a nonempty meaningful label where the owning operation creates an object. Preserve original editable draft on error.
- **seq** · `String` · line 499: Validate the declared molecule/grammar before commit. Preserve original text and report meaningful normalization or unsupported syntax.
- **pattern** · `usize` · line 500: Use a labeled finite choice with a visible selected value. Validate enum/mode availability; preserve unknown legacy values for review.
- **mod_code** · `usize` · line 501: Use a labeled finite choice with a visible selected value. Validate enum/mode availability; preserve unknown legacy values for review.
- **helm** · `String` · line 502: Validate the declared molecule/grammar before commit. Preserve original text and report meaningful normalization or unsupported syntax.
- **error** · `Option<String>` · line 503: System-owned validation/error state; never an editable user field.

## IvStudyDraft

[crates/appcore/src/state.rs:508](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:508)

- **name** · `String` · line 509: Trim for validation; require a nonempty meaningful label where the owning operation creates an object. Preserve original editable draft on error.
- **species** · `String` · line 510: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **strain** · `String` · line 511: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **error** · `Option<String>` · line 512: System-owned validation/error state; never an editable user field.

## IvAnimalDraft

[crates/appcore/src/state.rs:517](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:517)

- **tag** · `String` · line 518: Trim for validation; require a nonempty meaningful label where the owning operation creates an object. Preserve original editable draft on error.
- **sex** · `usize` · line 519: Use a labeled finite choice with a visible selected value. Validate enum/mode availability; preserve unknown legacy values for review.
- **weight** · `String` · line 520: Parse finite numeric input, display units/range where relevant and apply domain-specific bounds. Do not silently substitute a default after invalid input.
- **error** · `Option<String>` · line 521: System-owned validation/error state; never an editable user field.

## IvMeasureDraft

[crates/appcore/src/state.rs:526](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:526)

- **animal** · `usize` · line 527: Resolve a stable typed identity from a labeled picker. Validate existence/kind at submit; do not persist a row index as a foreign key.
- **day** · `String` · line 528: Parse finite numeric input, display units/range where relevant and apply domain-specific bounds. Do not silently substitute a default after invalid input.
- **weight** · `String` · line 529: Parse finite numeric input, display units/range where relevant and apply domain-specific bounds. Do not silently substitute a default after invalid input.
- **tumor** · `String` · line 530: Parse finite numeric input, display units/range where relevant and apply domain-specific bounds. Do not silently substitute a default after invalid input.
- **error** · `Option<String>` · line 531: System-owned validation/error state; never an editable user field.

## BpRecipeDraft

[crates/appcore/src/state.rs:536](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:536)

- **name** · `String` · line 537: Trim for validation; require a nonempty meaningful label where the owning operation creates an object. Preserve original editable draft on error.
- **product** · `String` · line 538: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **error** · `Option<String>` · line 539: System-owned validation/error state; never an editable user field.

## BpReplicateDraft

[crates/appcore/src/state.rs:544](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:544)

- **label** · `String` · line 545: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **titer** · `String` · line 546: Parse finite numeric input, display units/range where relevant and apply domain-specific bounds. Do not silently substitute a default after invalid input.
- **temp** · `String` · line 547: Parse finite numeric input, display units/range where relevant and apply domain-specific bounds. Do not silently substitute a default after invalid input.
- **ph** · `String` · line 548: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **error** · `Option<String>` · line 549: System-owned validation/error state; never an editable user field.

## RegistryImportDraft

[crates/appcore/src/state.rs:553](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:553)

- **csv** · `String` · line 554: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **kind** · `usize` · line 555: Use a labeled finite choice with a visible selected value. Validate enum/mode availability; preserve unknown legacy values for review.
- **name_col** · `Option<usize>` · line 557: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **row_errors** · `Vec<String>` · line 558: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **error** · `Option<String>` · line 559: System-owned validation/error state; never an editable user field.
- **imported** · `usize` · line 560: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.

## InventoryDraft

[crates/appcore/src/state.rs:564](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:564)

- **name** · `String` · line 565: Trim for validation; require a nonempty meaningful label where the owning operation creates an object. Preserve original editable draft on error.
- **kind** · `usize` · line 566: Use a labeled finite choice with a visible selected value. Validate enum/mode availability; preserve unknown legacy values for review.
- **parent** · `Option<u64>` · line 567: Resolve a stable typed identity from a labeled picker. Validate existence/kind at submit; do not persist a row index as a foreign key.
- **selected_plate** · `Option<u64>` · line 568: Resolve a stable typed identity from a labeled picker. Validate existence/kind at submit; do not persist a row index as a foreign key.
- **lookup** · `String` · line 569: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **error** · `Option<String>` · line 570: System-owned validation/error state; never an editable user field.

## WorkflowDraft

[crates/appcore/src/state.rs:574](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:574)

- **title** · `String` · line 575: Trim for validation; require a nonempty meaningful label where the owning operation creates an object. Preserve original editable draft on error.
- **schema** · `String` · line 576: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **request_name** · `String` · line 577: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **output_key** · `String` · line 578: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **output_val** · `String` · line 579: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **selected_task** · `Option<u64>` · line 580: Resolve a stable typed identity from a labeled picker. Validate existence/kind at submit; do not persist a row index as a foreign key.
- **error** · `Option<String>` · line 581: System-owned validation/error state; never an editable user field.

## CombDraft

[crates/appcore/src/state.rs:585](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:585)

- **strategy** · `usize` · line 586: Use a labeled finite choice with a visible selected value. Validate enum/mode availability; preserve unknown legacy values for review.
- **cut** · `String` · line 587: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **cap** · `String` · line 588: Parse finite numeric input, display units/range where relevant and apply domain-specific bounds. Do not silently substitute a default after invalid input.
- **results** · `Vec<domain::assembly::Assembly>` · line 589: Computed preview/output state. Clear or mark stale when relevant input changes; never commit an unrelated result.
- **labels** · `Vec<String>` · line 590: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **error** · `Option<String>` · line 591: System-owned validation/error state; never an editable user field.

## RegistryDraft

[crates/appcore/src/state.rs:595](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:595)

- **filter** · `usize` · line 597: Use a labeled finite choice with a visible selected value. Validate enum/mode availability; preserve unknown legacy values for review.
- **error** · `Option<String>` · line 598: System-owned validation/error state; never an editable user field.

## RegistryEditDraft

[crates/appcore/src/state.rs:602](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:602)

- **id** · `Option<u64>` · line 603: Resolve a stable typed identity from a labeled picker. Validate existence/kind at submit; do not persist a row index as a foreign key.
- **kind** · `usize` · line 604: Use a labeled finite choice with a visible selected value. Validate enum/mode availability; preserve unknown legacy values for review.
- **values** · `Vec<String>` · line 605: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **error** · `Option<String>` · line 606: System-owned validation/error state; never an editable user field.

## EnzymeFilterState

[crates/appcore/src/state.rs:610](C:/Users/Admin/Documents/Qoder/2026-11-13/87e5a6b2/helixbench/crates/appcore/src/state.rs:610)

- **name** · `String` · line 611: Trim for validation; require a nonempty meaningful label where the owning operation creates an object. Preserve original editable draft on error.
- **cut_count** · `usize` · line 612: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **overhang** · `usize` · line 613: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **site_len** · `usize` · line 614: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **region** · `usize` · line 615: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **region_start** · `String` · line 616: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.
- **region_end** · `String` · line 617: Use the owning feature/module requirements and the current typed source definition. Optional values remain distinguishable from empty/default values; validate before commit.

