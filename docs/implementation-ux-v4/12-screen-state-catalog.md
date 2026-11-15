# Figma screen and state catalog

All 686 captured root frames are indexed. A frame can be a scenario, responsive variant or reference; it is not automatically a production route. Use shared state machines and typed identities. Back/Close and native variable effects are preserved as authored evidence.

## ST-001 · Form / New Sequence

**Role:** Reference or alternate fixture. **Frame:** 107:25096 · 640 × 700. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-25096).

**Displayed content:**

- New Sequence
- Create a DNA or protein document.
- Name
- Untitled sequence
- Molecule
- DNA
- Topology
- Circular
- Sequence
- Paste sequence here…
- ATGC…
- Supported letters are cleaned and validated before creation.
- Cancel
- Create

**Authored actions:**

- `107:25101` Icon / x → `[{"type":"CLOSE"}]`
- `107:25132` Button / Cancel → `[{"type":"CLOSE"}]`
- `107:25133` Button / Create → `[{"type":"NODE","target":"107:29677","navigation":"NAVIGATE"}]`

## ST-002 · Form / Add Feature

**Role:** Linked functional state. **Frame:** 107:25134 · 640 × 700. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-25134).

**Displayed content:**

- Add Feature
- Name
- T7 promoter
- Kind
- promoter
- Segments
- 97..116
- Direction
- Forward
- Product / Note
- T7 RNA polymerase promoter
- Translate feature
- Genetic code
- Standard
- Cancel
- Add feature

**Authored actions:**

- `107:25139` Icon / x → `[{"type":"CLOSE"}]`
- `107:25180` Button / Cancel → `[{"type":"CLOSE"}]`
- `107:25181` Button / Add feature → `[{"type":"NODE","target":"107:34881","navigation":"SWAP"}]`

## ST-003 · Form / Add Primer

**Role:** Linked functional state. **Frame:** 107:25182 · 640 × 700. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-25182).

**Displayed content:**

- Add Primer
- Name
- M13-F (-20)
- Sequence 5′→3′
- GTAAAACGACGGCCAGT
- Description
- Universal M13 forward sequencing primer
- 5′ phosphorylated
- Length 17 nt · GC 52.9%
- Secondary structure
- Hairpin: low
- Self-dimer: low
- Structure values are calculated by the application.
- Cancel
- Add primer

**Authored actions:**

- `107:25187` Icon / x → `[{"type":"CLOSE"}]`
- `107:25215` Button / Cancel → `[{"type":"CLOSE"}]`
- `107:25216` Button / Add primer → `[{"type":"NODE","target":"128:6704","navigation":"NAVIGATE"}]`

## ST-004 · Form / Map & Sequence Options

**Role:** Linked functional state. **Frame:** 107:25217 · 640 × 700. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-25217).

**Displayed content:**

- Map & Sequence Options
- Show features
- Show enzymes
- Show ORFs
- Complement strand
- Show translations
- Zoom controls
- Genetic code
- Standard
- Minimum ORF length (aa)
- 50
- Cancel
- Apply

**Authored actions:**

- `107:25222` Icon / x → `[{"type":"CLOSE"}]`
- `107:25264` Button / Cancel → `[{"type":"CLOSE"}]`
- `107:25265` Button / Apply → `[{"type":"CLOSE"}]`

## ST-005 · Form / PCR Simulation

**Role:** Linked functional state. **Frame:** 107:25266 · 640 × 700. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-25266).

**Displayed content:**

- PCR Simulation
- Select a forward and reverse primer for the active template.
- Forward primer
- T7-F
- Reverse primer
- Demo-R
- Copy template features
- Simulate
- Product preview
- 255 bp · linear · illustrative result
- Open product as a new document after simulation.
- Cancel
- Open product

**Authored actions:**

- `107:25271` Icon / x → `[{"type":"CLOSE"}]`
- `107:25295` Button / Simulate → `[{"type":"NODE","target":"107:33352","navigation":"SWAP"}]`
- `107:25301` Button / Cancel → `[{"type":"CLOSE"}]`
- `107:25302` Button / Open product → `[{"type":"NODE","target":"107:34861","navigation":"SWAP"}]`

## ST-006 · Form / Primer Design

**Role:** Linked functional state. **Frame:** 107:25303 · 640 × 700. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-25303).

**Displayed content:**

- Primer Design
- DNA-001 v7 · selected anchor 97–116. Review the proposed target 97–351 before designing a pair.
- Start
- 97
- Length
- 255
- Target Tm (°C)
- 60
- 5′ overhang
- Enter 5′ overhang…
- Design primers
- Candidate
- GC%
- Tm
- Structure
- Forward 1
- 24
- 54.2
- 60.1
- Low risk
- Reverse 1
- 50.0
- 59.8
- Illustrative candidates · validate in the application.
- Cancel
- Add selected pair

**Authored actions:**

- `107:25308` Icon / x → `[{"type":"CLOSE"}]`
- `107:25330` Button / Design primers → `[{"type":"NODE","target":"107:36891","navigation":"SWAP"}]`
- `107:25367` Button / Cancel → `[{"type":"CLOSE"}]`
- `107:25368` Button / Add selected pair → `[{"type":"NODE","target":"128:6734","navigation":"NAVIGATE"}]`

## ST-007 · Form / Virtual Digest & Gel

**Role:** Reference or alternate fixture. **Frame:** 107:25369 · 640 × 700. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-25369).

**Displayed content:**

- Virtual Digest & Gel
- Fragment sizes for the selected enzymes.
- Enzymes
- EcoRI + BamHI
- Fragment
- Size
- Topology
- 1
- 3003 bp
- Linear
- 2
- 18 bp
- Ladder
- Digest
- Gel is a simulated preview, not a measured experiment.
- Cancel
- Close
- Review digest result

**Authored actions:**

- `107:25374` Icon / x → `[{"type":"CLOSE"}]`
- `107:25422` Button / Cancel → `[{"type":"CLOSE"}]`
- `107:25423` Button / Close → `[{"type":"CLOSE"}]`
- `107:25424` Action / Review digest result → `[{"type":"NODE","target":"107:33375","navigation":"SWAP"}]`

## ST-008 · Form / Sequence Alignment

**Role:** Linked functional state. **Frame:** 107:25425 · 640 × 700. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-25425).

**Displayed content:**

- Sequence Alignment
- Reference
- pHelixBench-Demo
- Compare with
- Expression insert
- Auto-reindex circular sequences
- Align
- Reference   ATGCGTAAAGGCTTAGCTAG
-             |||||||||||  |||||||
- Insert      ATGCGTAAAGG--TAGCTAG
- Consensus   ATGCGTAAAGGCTTAGCTAG
- Illustrative alignment preview
- Cancel
- Save alignment

**Authored actions:**

- `107:25430` Icon / x → `[{"type":"CLOSE"}]`
- `107:25453` Button / Align → `[{"type":"NODE","target":"107:34130","navigation":"SWAP"}]`
- `107:25461` Button / Cancel → `[{"type":"CLOSE"}]`
- `107:25462` Button / Save alignment → `[{"type":"NODE","target":"107:34861","navigation":"SWAP"}]`

## ST-009 · Form / Assembly Wizard

**Role:** Linked functional state. **Frame:** 107:25463 · 640 × 700. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-25463).

**Displayed content:**

- Assembly Wizard
- Method
- Gibson assembly
- Vector
- pHelixBench-Demo
- Insert
- Expression insert
- Construct name
- Assembled construct
- Preview assembly
- Junction validation
- Review overlap sequences before creating the construct.
- Digest & Ligate · Gibson · Golden Gate
- Cancel
- Create construct

**Authored actions:**

- `107:25468` Icon / x → `[{"type":"CLOSE"}]`
- `107:25497` Button / Preview assembly → `[{"type":"NODE","target":"107:36918","navigation":"SWAP"}]`
- `107:25503` Button / Cancel → `[{"type":"CLOSE"}]`
- `107:25504` Button / Create construct → `[{"type":"NODE","target":"107:34861","navigation":"SWAP"}]`

## ST-010 · Form / CRISPR Guides

**Role:** Linked functional state. **Frame:** 107:25505 · 640 × 700. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-25505).

**Displayed content:**

- CRISPR Guides
- NGG PAM scan on both strands of the active sequence.
- Target start
- 169
- Target length
- 255
- Scan guides
- Guide 5′→3′
- PAM
- Strand
- On-target
- GCTAGCTAGGATCCGAATTC
- NGG
- +
- —
- GGTACCTAGCTAGCATGCGT
- −
- Doench Rule-Set-2 on-target score · CFD off-target score
- Scores require calculation in the application.
- Cancel
- Add selected guide

**Authored actions:**

- `107:25510` Icon / x → `[{"type":"CLOSE"}]`
- `107:25523` Button / Scan guides → `[{"type":"NODE","target":"107:36945","navigation":"SWAP"}]`
- `107:25555` Button / Cancel → `[{"type":"CLOSE"}]`
- `107:25556` Button / Add selected guide → `[{"type":"NODE","target":"107:34861","navigation":"SWAP"}]`

## ST-011 · Form / Codon Optimization

**Role:** Linked functional state. **Frame:** 107:25557 · 640 × 700. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-25557).

**Displayed content:**

- Codon Optimization
- Protein-preserving re-encoding for the selected host.
- Host
- E. coli
- Smooth GC windows
- Avoid restriction sites
- Codon harmonization
- Optimize
- Compare original and optimized sequence
- CAI · GC% · avoided sites · amino-acid preservation
- Run optimization to populate the result.
- Cancel
- Open optimized copy

**Authored actions:**

- `107:25562` Icon / x → `[{"type":"CLOSE"}]`
- `107:25587` Button / Optimize → `[{"type":"NODE","target":"107:35074","navigation":"SWAP"}]`
- `107:25593` Button / Cancel → `[{"type":"CLOSE"}]`
- `107:25594` Button / Open optimized copy → `[{"type":"NODE","target":"107:34861","navigation":"SWAP"}]`

## ST-012 · Form / Local BLAST

**Role:** Linked functional state. **Frame:** 107:25595 · 640 × 700. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-25595).

**Displayed content:**

- Local BLAST
- Seed-and-extend search across loaded FASTA or open documents.
- Database
- Open documents
- Load FASTA database…
- Query sequence
- ATGCGTAAAGGCTTAGCTAG
- Search
- Subject
- Identity
- Length
- Score
- pHelixBench-Demo
- —
- Expression insert
- Run search to calculate hit scores and alignment detail.
- Cancel
- Close

**Authored actions:**

- `107:25600` Icon / x → `[{"type":"CLOSE"}]`
- `107:25612` Button / Load FASTA database… → `[{"type":"NODE","target":"110:19645","navigation":"NAVIGATE"}]`
- `107:25617` Button / Search → `[{"type":"NODE","target":"107:35044","navigation":"SWAP"}]`
- `107:25648` Button / Cancel → `[{"type":"CLOSE"}]`
- `107:25649` Button / Close → `[{"type":"CLOSE"}]`

## ST-013 · Form / Inventory

**Role:** Linked functional state. **Frame:** 107:25650 · 640 × 700. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-25650).

**Displayed content:**

- Inventory
- Physical storage and plate well manager.
- Selected plate
- Assembly plate 01
- 1
- 2
- 3
- 4
- 5
- 6
- 7
- 8
- 9
- 10
- 11
- 12
- A
- B
- C
- D
- E
- F
- G
- H
- Fill wells
- Snapshot into entry
- Cancel
- Done

**Authored actions:**

- `107:25655` Icon / x → `[{"type":"CLOSE"}]`
- `107:25819` Button / Fill wells → `[{"type":"NODE","target":"107:26858","navigation":"SWAP"}]`
- `107:25820` Button / Snapshot into entry → `[{"type":"NODE","target":"107:33579","navigation":"SWAP"}]`
- `107:25822` Button / Cancel → `[{"type":"CLOSE"}]`
- `107:25823` Button / Done → `[{"type":"NODE","target":"107:31089","navigation":"NAVIGATE"}]`

## ST-014 · Form / Entry

**Role:** Reference or alternate fixture. **Frame:** 107:25824 · 640 × 700. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-25824).

**Displayed content:**

- Entry
- Title
- Expression cassette validation
- Project
- Expression construct
- Draft
- Notes
- Reviewed the expression cassette and annotated
T7 promoter, RBS, CDS and terminator.
- Sample
- Concentration
- Result
- Demo plasmid
- 84 ng/µL
- Pass
- PCR product
- 32 ng/µL
- Add section
- Cancel
- Save entry

**Authored actions:**

- `107:25829` Icon / x → `[{"type":"CLOSE"}]`
- `107:25871` Button / Add section → `[{"type":"NODE","target":"107:36972","navigation":"SWAP"}]`
- `107:25873` Button / Cancel → `[{"type":"CLOSE"}]`
- `107:25874` Button / Save entry → `[{"type":"NODE","target":"107:30702","navigation":"NAVIGATE"}]`

## ST-015 · Form / Registry Entry

**Role:** Linked functional state. **Frame:** 107:25875 · 640 × 700. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-25875).

**Displayed content:**

- Registry Entry
- Kind
- Plasmid
- Name
- pHelixBench-Demo
- Backbone
- Synthetic demo backbone
- Insert
- demo_cds
- Resistance
- Demo marker
- Origin
- ori (demo)
- Promoter
- T7 promoter
- Cancel
- Save entry

**Authored actions:**

- `107:25880` Icon / x → `[{"type":"CLOSE"}]`
- `107:25916` Button / Cancel → `[{"type":"CLOSE"}]`
- `107:25917` Button / Save entry → `[{"type":"NODE","target":"128:6764","navigation":"NAVIGATE"}]`

## ST-016 · Form / Unsaved Changes

**Role:** Reference or alternate fixture. **Frame:** 107:25918 · 640 × 700. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-25918).

**Displayed content:**

- Unsaved Changes
- Save changes to pHelixBench-Demo before closing?
- Your edits will be lost if you discard.
- Cancel
- Discard
- Save

**Authored actions:**

- `107:25923` Icon / x → `[{"type":"CLOSE"}]`
- `107:25935` Button / Cancel → `[{"type":"CLOSE"}]`
- `107:25936` Button / Discard → `[{"type":"NODE","target":"107:29237","navigation":"NAVIGATE"}]`
- `107:25937` Button / Save → `[{"type":"NODE","target":"128:6794","navigation":"NAVIGATE"}]`

## ST-017 · Form / Genetic code

**Role:** Linked functional state. **Frame:** 107:25938 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-25938).

**Displayed content:**

- Genetic code
- Close
- Current code
- Standard (1)
- Use document default
- Standard
- Available codes
- Standard · Vertebrate mitochondrial · Bacterial / plastid
- Cancel
- Use selected code

**Authored actions:**

- `107:25942` Action / Close → `[{"type":"CLOSE"}]`
- `107:25958` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:25959` Action / Use selected code → `[{"type":"NODE","target":"128:6824","navigation":"NAVIGATE"}]`

## ST-018 · Form / Nucleotide letter codes

**Role:** Linked functional state. **Frame:** 107:25960 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-25960).

**Displayed content:**

- Nucleotide letter codes
- Close
- Bases
- A · T / U · C · G
- Ambiguous bases
- R A/G · Y C/T · S C/G · W A/T
- Additional codes
- K G/T · M A/C · B C/G/T · D A/G/T
- Any base
- N
- Cancel
- Done

**Authored actions:**

- `107:25964` Action / Close → `[{"type":"CLOSE"}]`
- `107:25984` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:25985` Action / Done → `[{"type":"BACK"}]`

## ST-019 · Form / Find in sequence

**Role:** Reference or alternate fixture. **Frame:** 107:25986 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-25986).

**Displayed content:**

- Find in sequence
- Close
- Search mode
- DNA · Protein · Annotation name
- Query
- GAATTC
- Scope
- Active document · both strands
- Results
- 1 of 2 · Previous / Next
- Cancel
- Find

**Authored actions:**

- `107:25990` Action / Close → `[{"type":"CLOSE"}]`
- `107:26010` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:26011` Action / Find → `[{"type":"NODE","target":"128:6854","navigation":"NAVIGATE"}]`

## ST-020 · Form / Help & about

**Role:** Reference or alternate fixture. **Frame:** 107:26012 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-26012).

**Displayed content:**

- Help & about
- Close
- Application
- HelixBench · version 1.17.0
- Sequence workflows
- Map · annotations · primers · digestion · assembly
- Keyboard shortcuts
- Find Ctrl+F · Save Ctrl+S · Undo Ctrl+Z
- Workspace
- Local records and documents
- Cancel
- Done

**Authored actions:**

- `107:26016` Action / Close → `[{"type":"CLOSE"}]`
- `107:26036` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:26037` Action / Done → `[{"type":"BACK"}]`

## ST-021 · Form / Rotate sequence origin

**Role:** Linked functional state. **Frame:** 107:26038 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-26038).

**Displayed content:**

- Rotate sequence origin
- Close
- Document
- pHelixBench-Demo · circular
- New origin position
- 1
- Affected coordinates
- Features and primers will be re-indexed
- Cancel
- Review rotation

**Authored actions:**

- `107:26042` Action / Close → `[{"type":"CLOSE"}]`
- `107:26058` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:26059` Action / Review rotation → `[{"type":"NODE","target":"128:6884","navigation":"NAVIGATE"}]`

## ST-022 · Form / Detect common features

**Role:** Linked functional state. **Frame:** 107:26060 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-26060).

**Displayed content:**

- Detect common features
- Close
- Document
- pHelixBench-Demo
- New matches
- T7 promoter · lac operator · RBS
- Match selection
- Select all new / Select none
- Existing annotations
- Duplicates remain identified
- Cancel
- Add selected features

**Authored actions:**

- `107:26064` Action / Close → `[{"type":"CLOSE"}]`
- `107:26084` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:26085` Action / Add selected features → `[{"type":"NODE","target":"107:33313","navigation":"SWAP"}]`

## ST-023 · Form / Color sequence selection

**Role:** Linked functional state. **Frame:** 107:26086 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-26086).

**Displayed content:**

- Color sequence selection
- Close
- Selected range
- 169..423
- Strands
- Forward · Reverse · Both
- Color
- Mint
- Preview
- 255 bp selected
- Cancel
- Apply color

**Authored actions:**

- `107:26090` Action / Close → `[{"type":"CLOSE"}]`
- `107:26110` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:26111` Action / Apply color → `[{"type":"NODE","target":"128:6914","navigation":"NAVIGATE"}]`

## ST-024 · Form / Combinatorial assembly

**Role:** Linked functional state. **Frame:** 107:26112 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-26112).

**Displayed content:**

- Combinatorial assembly
- Close
- Vector
- pHelixBench-Demo
- Insert documents
- Expression insert · PCR product
- Strategy
- Gibson assembly
- Variant limit
- Capped by the application
- Constructs
- Generate to review candidate orderings
- Cancel
- Generate constructs

**Authored actions:**

- `107:26116` Action / Close → `[{"type":"CLOSE"}]`
- `107:26140` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:26141` Action / Generate constructs → `[{"type":"NODE","target":"107:34861","navigation":"SWAP"}]`

## ST-025 · Form / Registry lookup

**Role:** Linked functional state. **Frame:** 107:26142 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-26142).

**Displayed content:**

- Registry lookup
- Close
- Entity kind
- Plasmids · Strains · Primers
- Selected entity
- pHelixBench-Demo
- Document
- pHelixBench-Demo.hbx
- Record actions
- Edit · Link active document · Open linked document
- Cancel
- Open selected entity

**Authored actions:**

- `107:26146` Action / Close → `[{"type":"CLOSE"}]`
- `107:26166` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:26167` Action / Open selected entity → `[{"type":"NODE","target":"128:6944","navigation":"NAVIGATE"}]`

## ST-026 · Form / Protein analysis

**Role:** Linked functional state. **Frame:** 107:26168 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-26168).

**Displayed content:**

- Analyze selected chain
- Close
- Selected protein chain
- HC-014 · heavy chain · 450 aa · source record
- Comparison protein
- Optional sequence for BLOSUM62 alignment
- Properties
- MW · pI · GRAVY · charge · extinction · composition
- Results
- Run analysis to calculate
- Cancel
- Analyze protein

**Authored actions:**

- `107:26172` Action / Close → `[{"type":"CLOSE"}]`
- `107:26192` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:26193` Action / Analyze protein → `[{"type":"NODE","target":"107:35021","navigation":"SWAP"}]`

## ST-027 · Form / Feature library

**Role:** Linked functional state. **Frame:** 107:26194 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-26194).

**Displayed content:**

- Feature library
- Close
- Motif name
- T7 promoter
- DNA motif
- TAATACGACTCACTATAGGG
- Feature kind
- promoter
- Target documents
- Selected open documents
- Actions
- Add motif · Delete motif · Bulk auto-annotate
- Cancel
- Review annotations

**Authored actions:**

- `107:26198` Action / Close → `[{"type":"CLOSE"}]`
- `107:26222` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:26223` Action / Review annotations → `[{"type":"NODE","target":"128:6974","navigation":"NAVIGATE"}]`

## ST-028 · Form / Bulk document actions

**Role:** Reference or alternate fixture. **Frame:** 107:26224 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-26224).

**Displayed content:**

- Bulk document actions
- Close
- Selected documents
- pHelixBench-Demo · PCR product
- Tag
- Expression project
- Export format
- FASTA
- Close selected
- Review unsaved documents first
- Cancel
- Apply selected action

**Authored actions:**

- `107:26228` Action / Close → `[{"type":"CLOSE"}]`
- `107:26248` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:26249` Action / Apply selected action → `[{"type":"NODE","target":"128:7004","navigation":"NAVIGATE"}]`

## ST-029 · Form / Choose notebook entry

**Role:** Linked functional state. **Frame:** 107:26250 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-26250).

**Displayed content:**

- Choose notebook entry
- Close
- Entry status
- Draft · Submitted · Approved
- Entry
- Expression cassette validation
- Project
- Expression construct
- Actions
- New entry · Open · Submit · Approve
- Cancel
- Open entry

**Authored actions:**

- `107:26254` Action / Close → `[{"type":"CLOSE"}]`
- `107:26274` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:26275` Action / Open entry → `[{"type":"NODE","target":"128:7034","navigation":"NAVIGATE"}]`

## ST-030 · Form / Projects & folders

**Role:** Linked functional state. **Frame:** 107:26276 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-26276).

**Displayed content:**

- Projects & folders
- Close
- Project or folder name
- Expression construct
- Parent folder
- Workspace
- Current projects
- Expression construct · Primer screening
- Actions
- Add folder · Move item · Delete folder
- Cancel
- Create folder

**Authored actions:**

- `107:26280` Action / Close → `[{"type":"CLOSE"}]`
- `107:26300` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:26301` Action / Create folder → `[{"type":"NODE","target":"128:7064","navigation":"NAVIGATE"}]`

## ST-031 · Form / Audit log

**Role:** Linked functional state. **Frame:** 107:26302 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-26302).

**Displayed content:**

- Audit log
- Close
- Scope
- Workspace events
- Latest event
- Entry saved · Expression cassette validation
- Previous event
- Registry record updated · pHelixBench-Demo
- Export
- CSV · current audit events
- Cancel
- Export CSV

**Authored actions:**

- `107:26306` Action / Close → `[{"type":"CLOSE"}]`
- `107:26326` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:26327` Action / Export CSV → `[{"type":"NODE","target":"128:7094","navigation":"NAVIGATE"}]`

## ST-032 · Form / Resume workspace item

**Role:** Reference or alternate fixture. **Frame:** 107:26328 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-26328).

**Displayed content:**

- Resume workspace item
- Close
- My work
- pHelixBench-Demo · PCR product · Expression insert
- Recent entry
- Expression cassette validation
- Open task
- Validate expression cassette
- Return location
- Keep the selected document or record
- Cancel
- Resume selected item

**Authored actions:**

- `107:26332` Action / Close → `[{"type":"CLOSE"}]`
- `107:26352` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:26353` Action / Resume selected item → `[{"type":"NODE","target":"128:7124","navigation":"NAVIGATE"}]`

## ST-033 · Form / Entity relationships

**Role:** Linked functional state. **Frame:** 107:26354 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-26354).

**Displayed content:**

- Entity relationships
- Close
- Source entity
- pHelixBench-Demo
- Related entity
- Demo-Fwd
- Relationship
- Linked entity
- Current relationships
- Review existing links before adding
- Cancel
- Link entities

**Authored actions:**

- `107:26358` Action / Close → `[{"type":"CLOSE"}]`
- `107:26378` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:26379` Action / Link entities → `[{"type":"NODE","target":"128:7154","navigation":"NAVIGATE"}]`

## ST-034 · Form / Import registry CSV

**Role:** Linked functional state. **Frame:** 107:26380 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-26380).

**Displayed content:**

- Import registry CSV
- Close
- Entity kind
- Plasmid
- CSV file
- Choose a CSV file
- Field mapping
- Name → Name · Backbone → Backbone
- Validation
- Preview missing and invalid values
- Rows to import
- Review the mapped records
- Cancel
- Review import

**Authored actions:**

- `107:26384` Action / Close → `[{"type":"CLOSE"}]`
- `107:26408` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:26409` Action / Review import → `[{"type":"NODE","target":"107:34461","navigation":"SWAP"}]`

## ST-035 · Form / Task details & output

**Role:** Linked functional state. **Frame:** 107:26410 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-26410).

**Displayed content:**

- Task details & output
- Close
- Details
- Output
- Requests
- Task
- Validate expression cassette
- Input schema
- Sequence review
- Status
- In progress
- Output key
- result
- Output value
- Validated
- Next step
- Record result in notebook
- Cancel
- Complete with output

**Authored actions:**

- `107:26414` Action / Close → `[{"type":"CLOSE"}]`
- `107:26446` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:26447` Action / Complete with output → `[{"type":"NODE","target":"128:7184","navigation":"NAVIGATE"}]`

## ST-036 · Form / Register biologic

**Role:** Linked functional state. **Frame:** 107:26448 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-26448).

**Displayed content:**

- Register biologic
- Close
- Name
- Demo IgG1
- Format
- IgG · Bispecific · Nanobody · TCR · Fab
- Numbering scheme
- IMGT
- Heavy chain sequence
- EVQLVESGGGLVQPGGSLRLSCAASGFTFSSYAMS
- Light chain sequence
- DIQMTQSPSSLSASVGDRVTITCRASQDVNTAVA
- Cancel

**Authored actions:**

- `107:26452` Action / Close → `[{"type":"CLOSE"}]`
- `107:26476` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:26477` Action / Register biologic → `[{"type":"NODE","target":"128:7214","navigation":"NAVIGATE"}]`

## ST-037 · Form / Add compound

**Role:** Linked functional state. **Frame:** 107:26478 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-26478).

**Displayed content:**

- Add compound
- Close
- Compound name
- Ethanol
- SMILES
- CCO
- Structure preview
- Calculated after valid SMILES input
- Properties
- Molecular formula and molecular weight
- Cancel

**Authored actions:**

- `107:26482` Action / Close → `[{"type":"CLOSE"}]`
- `107:26502` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:26503` Action / Add compound → `[{"type":"NODE","target":"128:7244","navigation":"NAVIGATE"}]`

## ST-038 · Form / Add reaction

**Role:** Linked functional state. **Frame:** 107:26504 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-26504).

**Displayed content:**

- Add reaction
- Close
- Reaction name
- Esterification
- Reactant
- Ethanol
- Reactant equivalents
- 1.0
- Product
- Ethyl acetate
- Product equivalents
- Cancel

**Authored actions:**

- `107:26508` Action / Close → `[{"type":"CLOSE"}]`
- `107:26532` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:26533` Action / Add reaction → `[{"type":"NODE","target":"128:7274","navigation":"NAVIGATE"}]`

## ST-039 · Form / Add material lot

**Role:** Linked functional state. **Frame:** 107:26534 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-26534).

**Displayed content:**

- Add material lot
- Close
- Compound
- Ethanol
- Quantity
- 100
- Unit
- mL
- Salt or solvate form
- Free form
- Cancel
- Add lot

**Authored actions:**

- `107:26538` Action / Close → `[{"type":"CLOSE"}]`
- `107:26558` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:26559` Action / Add lot → `[{"type":"NODE","target":"128:7304","navigation":"NAVIGATE"}]`

## ST-040 · Form / Add RNA / oligo

**Role:** Linked functional state. **Frame:** 107:26560 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-26560).

**Displayed content:**

- Add RNA / oligo
- Close
- Sequence
- Modifications
- HELM
- Oligo name
- Demo siRNA sense
- RNA sequence / supported HELM
- GCAUGACUAGCUACGUAGCAA
- Modification pattern
- None · 2′-OMe · 2′-F · LNA
- Custom modifications
- Position and monomer code
- Cancel
- Add oligo

**Authored actions:**

- `107:26564` Action / Close → `[{"type":"CLOSE"}]`
- `107:26588` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:26589` Action / Add oligo → `[{"type":"NODE","target":"128:7334","navigation":"NAVIGATE"}]`

## ST-041 · Form / New study

**Role:** Linked functional state. **Frame:** 107:26590 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-26590).

**Displayed content:**

- New study
- Close
- Study name
- Demo study 01
- Species
- Mouse
- Strain
- C57BL/6
- Cancel
- Create study

**Authored actions:**

- `107:26594` Action / Close → `[{"type":"CLOSE"}]`
- `107:26610` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:26611` Action / Create study → `[{"type":"NODE","target":"128:7364","navigation":"NAVIGATE"}]`

## ST-042 · Form / Add animal

**Role:** Linked functional state. **Frame:** 107:26612 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-26612).

**Displayed content:**

- Add animal
- Close
- Study
- Demo study 01
- Animal tag
- M001
- Sex
- Female
- Baseline weight (g)
- 22.4
- Cancel

**Authored actions:**

- `107:26616` Action / Close → `[{"type":"CLOSE"}]`
- `107:26636` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:26637` Action / Add animal → `[{"type":"NODE","target":"128:7394","navigation":"NAVIGATE"}]`

## ST-043 · Form / Record animal measurement

**Role:** Linked functional state. **Frame:** 107:26638 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-26638).

**Displayed content:**

- Record animal measurement
- Close
- Study
- Demo study 01
- Animal
- M001
- Day
- 3
- Weight (g)
- 22.4
- Cancel
- Record measurement

**Authored actions:**

- `107:26642` Action / Close → `[{"type":"CLOSE"}]`
- `107:26662` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:26663` Action / Record measurement → `[{"type":"NODE","target":"128:7424","navigation":"NAVIGATE"}]`

## ST-044 · Form / New bioprocess recipe

**Role:** Linked functional state. **Frame:** 107:26664 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-26664).

**Displayed content:**

- New bioprocess recipe
- Close
- Recipe name
- Demo upstream → downstream
- Product
- Recombinant protein
- Operations
- Seed · Culture · Harvest · Purification · QC
- Cancel
- Create recipe

**Authored actions:**

- `107:26668` Action / Close → `[{"type":"CLOSE"}]`
- `107:26684` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:26685` Action / Create recipe → `[{"type":"NODE","target":"128:7454","navigation":"NAVIGATE"}]`

## ST-045 · Form / Add replicate

**Role:** Linked functional state. **Frame:** 107:26686 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-26686).

**Displayed content:**

- Add replicate
- Close
- Run
- Run-01
- Replicate name
- Rep-03
- Titer (mg/L)
- 124
- Temperature (°C)
- 37
- pH
- 7.0
- Cancel

**Authored actions:**

- `107:26690` Action / Close → `[{"type":"CLOSE"}]`
- `107:26714` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:26715` Action / Add replicate → `[{"type":"NODE","target":"128:7484","navigation":"NAVIGATE"}]`

## ST-046 · Form / Configure pipeline

**Role:** Linked functional state. **Frame:** 107:26716 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-26716).

**Displayed content:**

- Configure pipeline · example fixture
- Close
- Pipeline name
- Assay QC import
- Input CSV
- assay-results.csv
- Transform
- Filter valid samples → Count
- Output destination
- Notebook / Assay QC results
- Lifecycle
- Draft · review before publish
- Cancel
- Save pipeline

**Authored actions:**

- `107:26720` Action / Close → `[{"type":"CLOSE"}]`
- `107:26744` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:26745` Action / Save pipeline → `[{"type":"NODE","target":"128:7514","navigation":"NAVIGATE"}]`

## ST-047 · Form / Run selected pipeline

**Role:** Linked functional state. **Frame:** 107:26746 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-26746).

**Displayed content:**

- Run selected pipeline
- Close
- Pipeline
- Assay QC import
- Input CSV
- assay-results.csv · 96 rows
- Writeback destination
- Notebook / Assay QC results
- Validation
- Required columns present
- Preview
- Review records to be written
- Cancel

**Authored actions:**

- `107:26750` Action / Close → `[{"type":"CLOSE"}]`
- `107:26774` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:26775` Action / Run selected pipeline → `[{"type":"NODE","target":"107:33978","navigation":"SWAP"}]`

## ST-048 · Form / Plan a recipe run

**Role:** Linked functional state. **Frame:** 107:26776 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-26776).

**Displayed content:**

- Plan a recipe run
- Close
- Run name
- Run-02
- Selected recipe
- Demo upstream → downstream
- Product
- Recombinant protein
- Operation preview
- 5 operations · 72 h
- Cancel
- Plan run

**Authored actions:**

- `107:26780` Action / Close → `[{"type":"CLOSE"}]`
- `107:26800` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:26801` Action / Plan run → `[{"type":"NODE","target":"107:36763","navigation":"SWAP"}]`

## ST-049 · Form / Randomize cohorts

**Role:** Linked functional state. **Frame:** 107:26802 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-26802).

**Displayed content:**

- Randomize cohorts
- Close
- Study
- Demo study 01
- Number of groups
- 2
- Random seed
- 42
- Existing assignment
- Review before replacing cohorts
- Cancel
- Review assignment

**Authored actions:**

- `107:26806` Action / Close → `[{"type":"CLOSE"}]`
- `107:26826` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:26827` Action / Review assignment → `[{"type":"NODE","target":"128:7544","navigation":"NAVIGATE"}]`

## ST-050 · Form / Apply modification pattern

**Role:** Linked functional state. **Frame:** 107:26828 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-26828).

**Displayed content:**

- Apply modification pattern
- Close
- Selected oligo
- Demo siRNA sense
- Pattern
- 2′-OMe
- Range
- 1..21
- Repeat / spacing parameter
- 4
- Replacement preview
- Existing modifications at these positions will change
- Cancel
- Review pattern

**Authored actions:**

- `107:26832` Action / Close → `[{"type":"CLOSE"}]`
- `107:26856` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:26857` Action / Review pattern → `[{"type":"NODE","target":"107:34184","navigation":"SWAP"}]`

## ST-051 · Form / Assign plate wells

**Role:** Linked functional state. **Frame:** 107:26858 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-26858).

**Displayed content:**

- Assign plate wells
- Close
- Selected plate
- P-014
- Well range
- A1:A4
- Material / registry entity
- pHelixBench-Demo
- Well role
- Sample
- Quantity and concentration
- 50 µL · 100 ng/µL
- Occupied wells
- Review replacements before applying
- Cancel
- Review well assignment

**Authored actions:**

- `107:26862` Action / Close → `[{"type":"CLOSE"}]`
- `107:26890` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:26891` Action / Review well assignment → `[{"type":"NODE","target":"128:7574","navigation":"NAVIGATE"}]`

## ST-052 · Form / Choose entry template

**Role:** Linked functional state. **Frame:** 107:26892 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-26892).

**Displayed content:**

- Choose entry template
- Close
- Template
- Experiment · PCR · Assembly review
- Project
- Expression construct
- Entry title
- Expression cassette validation
- Included sections
- Objective · Method · Results · Notes
- Cancel
- Create draft from template

**Authored actions:**

- `107:26896` Action / Close → `[{"type":"CLOSE"}]`
- `107:26916` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:26917` Action / Create draft from template → `[{"type":"NODE","target":"128:7604","navigation":"NAVIGATE"}]`

## ST-053 · Form / Configure insight

**Role:** Linked functional state. **Frame:** 107:26918 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-26918).

**Displayed content:**

- Configure insight
- Close
- Insight title
- Registry by kind
- Dataset
- Registry
- Grouping
- Kind
- Aggregation
- Count
- Export scope
- Source dataset CSV
- Cancel
- Save insight

**Authored actions:**

- `107:26922` Action / Close → `[{"type":"CLOSE"}]`
- `107:26946` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:26947` Action / Save insight → `[{"type":"NODE","target":"128:7634","navigation":"NAVIGATE"}]`

## ST-054 · Form / Columns & saved views

**Role:** Linked functional state. **Frame:** 107:26948 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-26948).

**Displayed content:**

- Columns & saved views
- Close
- Visible columns
- Name · Kind · Direction · Length · Location · Product
- Current sort
- Name · ascending
- Filter
- All features
- Saved view name
- Default annotations
- Saved views
- Load · Save current · Delete view
- Cancel
- Apply view

**Authored actions:**

- `107:26952` Action / Close → `[{"type":"CLOSE"}]`
- `107:26976` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:26977` Action / Apply view → `[{"type":"NODE","target":"128:7664","navigation":"NAVIGATE"}]`

## ST-055 · Form / Enzyme filters

**Role:** Linked functional state. **Frame:** 107:26978 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-26978).

**Displayed content:**

- Enzyme filters
- Close
- Name contains
- Enter name contains…
- Cut count
- Any · 0 · 1 · Multiple
- Overhang type
- Any · Blunt · 5′ · 3′
- Recognition length
- Any
- Coordinate region
- Whole sequence · Selection · Custom
- Cancel
- Apply filters

**Authored actions:**

- `107:26982` Action / Close → `[{"type":"CLOSE"}]`
- `107:27006` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:27007` Action / Apply filters → `[{"type":"NODE","target":"128:7694","navigation":"NAVIGATE"}]`

## ST-056 · Form / Add storage container

**Role:** Linked functional state. **Frame:** 107:27008 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-27008).

**Displayed content:**

- Add storage container · example fixture
- Close
- Container kind
- Location · Box · Plate · Tube
- Name
- Assembly plate 02
- Parent location
- −20°C Freezer
- Dimensions / position
- 8 × 12 wells
- Cancel
- Add container

**Authored actions:**

- `107:27012` Action / Close → `[{"type":"CLOSE"}]`
- `107:27032` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:27033` Action / Add container → `[{"type":"NODE","target":"128:7724","navigation":"NAVIGATE"}]`

## ST-057 · Form / New task

**Role:** Linked functional state. **Frame:** 107:27034 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-27034).

**Displayed content:**

- New task
- Close
- Task title
- Validate expression cassette
- Input schema
- Sequence review
- Linked record
- pHelixBench-Demo
- Required output
- result
- Cancel
- Create task

**Authored actions:**

- `107:27038` Action / Close → `[{"type":"CLOSE"}]`
- `107:27058` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:27059` Action / Create task → `[{"type":"NODE","target":"128:7754","navigation":"NAVIGATE"}]`

## ST-058 · Form / New request

**Role:** Linked functional state. **Frame:** 107:27060 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-27060).

**Displayed content:**

- New request
- Close
- Request name
- Sequence review request
- Input schema
- Sequence review
- Linked record
- pHelixBench-Demo
- State
- Draft
- Cancel
- Save request draft

**Authored actions:**

- `107:27064` Action / Close → `[{"type":"CLOSE"}]`
- `107:27084` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:27085` Action / Save request draft → `[{"type":"NODE","target":"128:7784","navigation":"NAVIGATE"}]`

## ST-059 · Form / Review record change

**Role:** Linked functional state. **Frame:** 107:27086 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-27086).

**Displayed content:**

- Review record change
- Close
- Selected record
- The record you selected
- Action
- Review the requested change
- Impact
- Linked records and unsaved work remain identified
- Recovery
- Cancel keeps the current record unchanged
- Cancel
- Confirm change

**Authored actions:**

- `107:27090` Action / Close → `[{"type":"CLOSE"}]`
- `107:27110` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:27111` Action / Confirm change → `[{"type":"NODE","target":"128:7814","navigation":"NAVIGATE"}]`

## ST-060 · Form / Dosing calculator

**Role:** Linked functional state. **Frame:** 107:27112 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-27112).

**Displayed content:**

- Dosing calculator
- Close
- Study
- Demo study 01
- Animal
- M001 · latest weight 22.4 g
- Dose (mg/kg)
- Enter dose (mg/kg)…
- Concentration (mg/mL)
- Enter concentration (mg/ml)…
- Calculated volume (mL)
- Enter valid dose and concentration
- Cancel
- Done

**Authored actions:**

- `107:27116` Action / Close → `[{"type":"CLOSE"}]`
- `107:27140` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:27141` Action / Done → `[{"type":"BACK"}]`

## ST-061 · Form / Export document

**Role:** Linked functional state. **Frame:** 107:27142 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-27142).

**Displayed content:**

- Export document
- Close
- Document
- pHelixBench-Demo
- Format
- FASTA · GenBank · Plain sequence
- Output
- Choose destination
- Scope
- Active document
- Cancel
- Export

**Authored actions:**

- `107:27146` Action / Close → `[{"type":"CLOSE"}]`
- `107:27166` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:27167` Action / Export → `[{"type":"NODE","target":"128:7844","navigation":"NAVIGATE"}]`

## ST-062 · Form / Search workspace records

**Role:** Reference or alternate fixture. **Frame:** 107:27168 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-27168).

**Displayed content:**

- Search workspace records
- Close
- Search records
- Enter search records…
- Record kind
- Documents · Entries · Registry · Tasks
- Results
- Search record names or identifiers
- Cancel
- Open selected record

**Authored actions:**

- `107:27172` Action / Close → `[{"type":"CLOSE"}]`
- `107:27188` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:27189` Action / Open selected record → `[{"type":"NODE","target":"128:7874","navigation":"NAVIGATE"}]`

## ST-063 · Inspector / Home

**Role:** Reference or alternate fixture. **Frame:** 107:27190 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-27190).

**Displayed content:**

- Home · selected record
- Close
- Selected record context stays visible while you inspect or act.
- Selected document
- pHelixBench-Demo
- Molecule
- DNA · 3021 bp · circular
- Location
- Local workspace
- Resume state
- Keep current view and selected region
- Edit
- Related records
- Review change
- Cancel
- Done

**Authored actions:**

- `107:27194` Action / Close → `[{"type":"CLOSE"}]`
- `107:27214` Action / Edit → `[{"type":"NODE","target":"107:25217","navigation":"SWAP"}]`
- `107:27215` Action / Related records → `[{"type":"NODE","target":"107:26142","navigation":"SWAP"}]`
- `107:27216` Action / Review change → `[{"type":"NODE","target":"107:27086","navigation":"SWAP"}]`
- `107:27219` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:27220` Action / Done → `[{"type":"CLOSE"}]`

## ST-064 · Inspector / Map

**Role:** Reference or alternate fixture. **Frame:** 107:27221 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-27221).

**Displayed content:**

- Map · selected record
- Close
- Selected record context stays visible while you inspect or act.
- Document
- pHelixBench-Demo
- Selected feature
- T7 promoter
- Coordinates
- 97..116 · forward strand
- View
- Plasmid · Linear · Metadata
- Zoom
- 100% · Fit · Center selection
- Edit
- Related records
- Review change
- Cancel
- Done

**Authored actions:**

- `107:27225` Action / Close → `[{"type":"CLOSE"}]`
- `107:27249` Action / Edit → `[{"type":"NODE","target":"107:25217","navigation":"SWAP"}]`
- `107:27250` Action / Related records → `[{"type":"NODE","target":"107:26142","navigation":"SWAP"}]`
- `107:27251` Action / Review change → `[{"type":"NODE","target":"107:27086","navigation":"SWAP"}]`
- `107:27254` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:27255` Action / Done → `[{"type":"CLOSE"}]`

## ST-065 · Inspector / Sequence

**Role:** Reference or alternate fixture. **Frame:** 107:27256 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-27256).

**Displayed content:**

- Sequence · selected record
- Close
- Selected record context stays visible while you inspect or act.
- Document
- pHelixBench-Demo
- Selection
- 169..423
- Bases
- ATGCGTAAAGGCTTAGCTAG
- Strand
- Forward · Complement
- Edit controls
- Insert · Replace · Copy · Cut · Paste · Delete
- Edit
- Related records
- Review change
- Cancel
- Done

**Authored actions:**

- `107:27260` Action / Close → `[{"type":"CLOSE"}]`
- `107:27284` Action / Edit → `[{"type":"NODE","target":"107:25217","navigation":"SWAP"}]`
- `107:27285` Action / Related records → `[{"type":"NODE","target":"107:26142","navigation":"SWAP"}]`
- `107:27286` Action / Review change → `[{"type":"NODE","target":"107:27086","navigation":"SWAP"}]`
- `107:27289` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:27290` Action / Done → `[{"type":"CLOSE"}]`

## ST-066 · Inspector / Features

**Role:** Linked functional state. **Frame:** 107:27291 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-27291).

**Displayed content:**

- Features · selected record
- Close
- Selected record context stays visible while you inspect or act.
- Feature
- T7 promoter
- Kind and direction
- promoter · forward
- Coordinates
- 97..116
- Annotation state
- Visible on map and sequence
- Product / note
- T7 RNA polymerase promoter
- Edit
- Related records
- Review change
- Cancel
- Done

**Authored actions:**

- `107:27295` Action / Close → `[{"type":"CLOSE"}]`
- `107:27319` Action / Edit → `[{"type":"NODE","target":"107:25134","navigation":"SWAP"}]`
- `107:27320` Action / Related records → `[{"type":"NODE","target":"107:26142","navigation":"SWAP"}]`
- `107:27321` Action / Review change → `[{"type":"NODE","target":"107:27086","navigation":"SWAP"}]`
- `107:27324` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:27325` Action / Done → `[{"type":"CLOSE"}]`

## ST-067 · Inspector / Primers

**Role:** Linked functional state. **Frame:** 107:27326 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-27326).

**Displayed content:**

- Primers · selected record
- Close
- Selected record context stays visible while you inspect or act.
- Primer
- Demo-Fwd
- Sequence
- AGGAGGACGATGAGCGGTAGCCAT
- Properties
- 24 nt · GC 58.3%
- Secondary structure
- Hairpin · Self-dimer
- Binding sites
- 1 site · forward
- Edit
- Related records
- Review change
- Cancel
- Done

**Authored actions:**

- `107:27330` Action / Close → `[{"type":"CLOSE"}]`
- `107:27354` Action / Edit → `[{"type":"NODE","target":"107:25182","navigation":"SWAP"}]`
- `107:27355` Action / Related records → `[{"type":"NODE","target":"107:26142","navigation":"SWAP"}]`
- `107:27356` Action / Review change → `[{"type":"NODE","target":"107:27086","navigation":"SWAP"}]`
- `107:27359` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:27360` Action / Done → `[{"type":"CLOSE"}]`

## ST-068 · Inspector / Enzymes

**Role:** Linked functional state. **Frame:** 107:27361 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-27361).

**Displayed content:**

- Enzymes · selected record
- Close
- Selected record context stays visible while you inspect or act.
- Enzyme set
- Common cloning
- Selected enzyme
- EcoRI
- Recognition site
- G↓AATTC
- Cut positions
- 630
- Controls
- Choose set · Check shown · Uncheck shown · Clear set
- Edit
- Related records
- Review change
- Cancel
- Done

**Authored actions:**

- `107:27365` Action / Close → `[{"type":"CLOSE"}]`
- `107:27389` Action / Edit → `[{"type":"NODE","target":"107:25217","navigation":"SWAP"}]`
- `107:27390` Action / Related records → `[{"type":"NODE","target":"107:26142","navigation":"SWAP"}]`
- `107:27391` Action / Review change → `[{"type":"NODE","target":"107:27086","navigation":"SWAP"}]`
- `107:27394` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:27395` Action / Done → `[{"type":"CLOSE"}]`

## ST-069 · Inspector / History

**Role:** Linked functional state. **Frame:** 107:27396 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-27396).

**Displayed content:**

- History · selected record
- Close
- Selected record context stays visible while you inspect or act.
- Selected operation
- Add feature T7 promoter
- Affected region
- 97..116
- Saved version
- Initial demo plasmid
- Actions
- Undo · Redo · Preview · Revert · Save version
- Edit
- Related records
- Review change
- Cancel
- Done

**Authored actions:**

- `107:27400` Action / Close → `[{"type":"CLOSE"}]`
- `107:27420` Action / Edit → `[{"type":"NODE","target":"107:25217","navigation":"SWAP"}]`
- `107:27421` Action / Related records → `[{"type":"NODE","target":"107:26142","navigation":"SWAP"}]`
- `107:27422` Action / Review change → `[{"type":"NODE","target":"107:27086","navigation":"SWAP"}]`
- `107:27425` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:27426` Action / Done → `[{"type":"CLOSE"}]`

## ST-070 · Inspector / Notebook

**Role:** Reference or alternate fixture. **Frame:** 107:27427 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-27427).

**Displayed content:**

- Notebook · selected record
- Close
- Selected record context stays visible while you inspect or act.
- Entry
- Expression cassette validation
- Status
- Draft
- Project
- Expression construct
- Result table
- 2 rows · not submitted
- Review actions
- Save draft · Submit · Request changes · Approve
- Edit
- Related records
- Review change
- Cancel
- Done

**Authored actions:**

- `107:27431` Action / Close → `[{"type":"CLOSE"}]`
- `107:27455` Action / Edit → `[{"type":"NODE","target":"110:20232","navigation":"NAVIGATE"}]`
- `107:27456` Action / Related records → `[{"type":"NODE","target":"107:26142","navigation":"SWAP"}]`
- `107:27457` Action / Review change → `[{"type":"NODE","target":"107:27086","navigation":"SWAP"}]`
- `107:27460` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:27461` Action / Done → `[{"type":"CLOSE"}]`

## ST-071 · Inspector / Registry

**Role:** Reference or alternate fixture. **Frame:** 107:27462 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-27462).

**Displayed content:**

- Registry · selected record
- Close
- Selected record context stays visible while you inspect or act.
- Entity
- pHelixBench-Demo
- Kind
- Plasmid
- Document
- pHelixBench-Demo.hbx
- Relationships
- Demo-Fwd · Demo-Rev
- Physical storage
- Plasmid box A · A1
- Edit
- Related records
- Review change
- Cancel
- Done

**Authored actions:**

- `107:27466` Action / Close → `[{"type":"CLOSE"}]`
- `107:27490` Action / Edit → `[{"type":"NODE","target":"107:25875","navigation":"SWAP"}]`
- `107:27491` Action / Related records → `[{"type":"NODE","target":"107:26142","navigation":"SWAP"}]`
- `107:27492` Action / Review change → `[{"type":"NODE","target":"107:27086","navigation":"SWAP"}]`
- `107:27495` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:27496` Action / Done → `[{"type":"CLOSE"}]`

## ST-072 · Inspector / Inventory

**Role:** Reference or alternate fixture. **Frame:** 107:27497 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-27497).

**Displayed content:**

- Inventory · selected record
- Close
- Selected record context stays visible while you inspect or act.
- Plate / well
- Assembly plate 01 · A1
- Entity
- pHelixBench-Demo
- Role
- Sample
- Quantity
- 50 µL
- Concentration
- 100 ng/µL
- Edit
- Related records
- Review change
- Cancel
- Done

**Authored actions:**

- `107:27501` Action / Close → `[{"type":"CLOSE"}]`
- `107:27525` Action / Edit → `[{"type":"NODE","target":"107:26858","navigation":"SWAP"}]`
- `107:27526` Action / Related records → `[{"type":"NODE","target":"107:26142","navigation":"SWAP"}]`
- `107:27527` Action / Review change → `[{"type":"NODE","target":"107:27086","navigation":"SWAP"}]`
- `107:27530` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:27531` Action / Done → `[{"type":"CLOSE"}]`

## ST-073 · Inspector / Workflows

**Role:** Reference or alternate fixture. **Frame:** 107:27532 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-27532).

**Displayed content:**

- Workflows · selected record
- Close
- Selected record context stays visible while you inspect or act.
- Task
- Validate expression cassette
- Status
- In progress
- Schema
- Sequence review
- Output
- result → Validated
- Edit
- Related records
- Review change
- Cancel
- Done

**Authored actions:**

- `107:27536` Action / Close → `[{"type":"CLOSE"}]`
- `107:27556` Action / Edit → `[{"type":"NODE","target":"107:26410","navigation":"SWAP"}]`
- `107:27557` Action / Related records → `[{"type":"NODE","target":"107:26142","navigation":"SWAP"}]`
- `107:27558` Action / Review change → `[{"type":"NODE","target":"107:27086","navigation":"SWAP"}]`
- `107:27561` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:27562` Action / Done → `[{"type":"CLOSE"}]`

## ST-074 · Inspector / Insights

**Role:** Reference or alternate fixture. **Frame:** 107:27563 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-27563).

**Displayed content:**

- Insights · selected record
- Close
- Selected record context stays visible while you inspect or act.
- Insight
- Registry by kind
- Source
- Registry
- Grouping
- Kind
- Aggregation
- Count
- Edit
- Related records
- Review change
- Cancel
- Done

**Authored actions:**

- `107:27567` Action / Close → `[{"type":"CLOSE"}]`
- `107:27587` Action / Edit → `[{"type":"NODE","target":"107:25217","navigation":"SWAP"}]`
- `107:27588` Action / Related records → `[{"type":"NODE","target":"107:26142","navigation":"SWAP"}]`
- `107:27589` Action / Review change → `[{"type":"NODE","target":"107:27086","navigation":"SWAP"}]`
- `107:27592` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:27593` Action / Done → `[{"type":"CLOSE"}]`

## ST-075 · Inspector / Automation

**Role:** Reference or alternate fixture. **Frame:** 107:27594 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-27594).

**Displayed content:**

- Automation · selected record
- Close
- Selected record context stays visible while you inspect or act.
- Selected pipeline
- Assay QC import
- CSV
- assay-results.csv · 96 rows
- Destination
- Notebook / Assay QC results
- Run target
- Use the selected pipeline ID
- Edit
- Related records
- Review change
- Cancel
- Done

**Authored actions:**

- `107:27598` Action / Close → `[{"type":"CLOSE"}]`
- `107:27618` Action / Edit → `[{"type":"NODE","target":"107:26716","navigation":"SWAP"}]`
- `107:27619` Action / Related records → `[{"type":"NODE","target":"107:26142","navigation":"SWAP"}]`
- `107:27620` Action / Review change → `[{"type":"NODE","target":"107:27086","navigation":"SWAP"}]`
- `107:27623` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:27624` Action / Done → `[{"type":"CLOSE"}]`

## ST-076 · Inspector / Split

**Role:** Linked functional state. **Frame:** 107:27625 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-27625).

**Displayed content:**

- Split · selected record
- Close
- Selected record context stays visible while you inspect or act.
- Left document
- pHelixBench-Demo
- Right document
- PCR product
- View
- Map · Sequence
- Open in editor
- Left pane · Right pane
- Edit
- Related records
- Review change
- Cancel
- Done

**Authored actions:**

- `107:27629` Action / Close → `[{"type":"CLOSE"}]`
- `107:27649` Action / Edit → `[{"type":"NODE","target":"107:25217","navigation":"SWAP"}]`
- `107:27650` Action / Related records → `[{"type":"NODE","target":"107:26142","navigation":"SWAP"}]`
- `107:27651` Action / Review change → `[{"type":"NODE","target":"107:27086","navigation":"SWAP"}]`
- `107:27654` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:27655` Action / Done → `[{"type":"CLOSE"}]`

## ST-077 · Inspector / Biologics

**Role:** Reference or alternate fixture. **Frame:** 107:27656 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-27656).

**Displayed content:**

- Biologics · selected record
- Close
- Selected record context stays visible while you inspect or act.
- Entity
- Demo IgG1
- Format
- IgG · 4 chains
- Chain
- Heavy chain 1 · 446 aa
- Domains
- VH · CH1 · CH2 · CH3
- Numbering
- IMGT
- Edit
- Related records
- Review change
- Cancel
- Done

**Authored actions:**

- `107:27660` Action / Close → `[{"type":"CLOSE"}]`
- `107:27684` Action / Edit → `[{"type":"NODE","target":"107:26448","navigation":"SWAP"}]`
- `107:27685` Action / Related records → `[{"type":"NODE","target":"107:26142","navigation":"SWAP"}]`
- `107:27686` Action / Review change → `[{"type":"NODE","target":"107:27086","navigation":"SWAP"}]`
- `107:27689` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:27690` Action / Done → `[{"type":"CLOSE"}]`

## ST-078 · Inspector / Chemistry

**Role:** Reference or alternate fixture. **Frame:** 107:27691 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-27691).

**Displayed content:**

- Chemistry · selected record
- Close
- Selected record context stays visible while you inspect or act.
- Compound
- Ethanol
- Formula / MW
- C₂H₆O · 46.07 g/mol
- SMILES
- CCO
- Lots
- 100 mL · free form
- Edit
- Related records
- Review change
- Cancel
- Done

**Authored actions:**

- `107:27695` Action / Close → `[{"type":"CLOSE"}]`
- `107:27715` Action / Edit → `[{"type":"NODE","target":"107:26478","navigation":"SWAP"}]`
- `107:27716` Action / Related records → `[{"type":"NODE","target":"107:26142","navigation":"SWAP"}]`
- `107:27717` Action / Review change → `[{"type":"NODE","target":"107:27086","navigation":"SWAP"}]`
- `107:27720` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:27721` Action / Done → `[{"type":"CLOSE"}]`

## ST-079 · Inspector / RNA / Oligos

**Role:** Reference or alternate fixture. **Frame:** 107:27722 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-27722).

**Displayed content:**

- RNA / Oligos · selected record
- Close
- Selected record context stays visible while you inspect or act.
- Oligo
- Demo siRNA sense
- Sequence
- GCAUGACUAGCUACGUAGCAA
- Properties
- 21 nt · MW / GC
- HELM
- View generated notation
- Modifications
- Review position, code and monomer
- Edit
- Related records
- Review change
- Cancel
- Done

**Authored actions:**

- `107:27726` Action / Close → `[{"type":"CLOSE"}]`
- `107:27750` Action / Edit → `[{"type":"NODE","target":"107:26560","navigation":"SWAP"}]`
- `107:27751` Action / Related records → `[{"type":"NODE","target":"107:26142","navigation":"SWAP"}]`
- `107:27752` Action / Review change → `[{"type":"NODE","target":"107:27086","navigation":"SWAP"}]`
- `107:27755` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:27756` Action / Done → `[{"type":"CLOSE"}]`

## ST-080 · Inspector / In Vivo

**Role:** Reference or alternate fixture. **Frame:** 107:27757 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-27757).

**Displayed content:**

- In Vivo · selected record
- Close
- Selected record context stays visible while you inspect or act.
- Study
- Demo study 01
- Animal
- M001 · female
- Current weight
- 22.4 g
- Cohort
- Group 1
- Edit
- Related records
- Review change
- Cancel
- Done

**Authored actions:**

- `107:27761` Action / Close → `[{"type":"CLOSE"}]`
- `107:27781` Action / Edit → `[{"type":"NODE","target":"107:26638","navigation":"SWAP"}]`
- `107:27782` Action / Related records → `[{"type":"NODE","target":"107:26142","navigation":"SWAP"}]`
- `107:27783` Action / Review change → `[{"type":"NODE","target":"107:27086","navigation":"SWAP"}]`
- `107:27786` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:27787` Action / Done → `[{"type":"CLOSE"}]`

## ST-081 · Inspector / Bioprocess

**Role:** Reference or alternate fixture. **Frame:** 107:27788 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-27788).

**Displayed content:**

- Bioprocess · selected record
- Close
- Selected record context stays visible while you inspect or act.
- Run
- Run-01
- Recipe
- Demo upstream → downstream
- Status
- Running · 40%
- Replicates
- Rep-01 · Rep-02
- Worksheet
- Seed and production complete
- Edit
- Related records
- Review change
- Cancel
- Done

**Authored actions:**

- `107:27792` Action / Close → `[{"type":"CLOSE"}]`
- `107:27816` Action / Edit → `[{"type":"NODE","target":"107:26776","navigation":"SWAP"}]`
- `107:27817` Action / Related records → `[{"type":"NODE","target":"107:26142","navigation":"SWAP"}]`
- `107:27818` Action / Review change → `[{"type":"NODE","target":"107:27086","navigation":"SWAP"}]`
- `107:27821` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:27822` Action / Done → `[{"type":"CLOSE"}]`

## ST-082 · Page tools / Home

**Role:** Reference or alternate fixture. **Frame:** 107:27823 · 980 × 708. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-27823).

**Displayed content:**

- Home tools
- Close
- Choose an action. Current document or record context is retained.
- Resume open document
- Open recent file
- Create sequence
- Open/import file
- Create notebook entry
- Open recent entry
- Resume selected task
- Calendar date activity
- Change calendar month / Today
- Workspace totals
- Projects and folders
- Bulk document operations
- Audit log
- Open seeded demo
- Unsaved Changes
- Help & about
- Search workspace records

**Authored actions:**

- `107:27827` Action / Close → `[{"type":"CLOSE"}]`
- `107:27830` Feature / Resume open document → `[{"type":"NODE","target":"107:26328","navigation":"SWAP"}]`
- `107:27834` Feature / Open recent file → `[{"type":"NODE","target":"107:26328","navigation":"SWAP"}]`
- `107:27839` Feature / Create sequence → `[{"type":"NODE","target":"110:20202","navigation":"NAVIGATE"}]`
- `107:27843` Feature / Open/import file → `[{"type":"NODE","target":"110:19645","navigation":"NAVIGATE"}]`
- `107:27848` Feature / Create notebook entry → `[{"type":"NODE","target":"110:20232","navigation":"NAVIGATE"}]`
- `107:27852` Feature / Open recent entry → `[{"type":"NODE","target":"110:20232","navigation":"NAVIGATE"}]`
- `107:27857` Feature / Resume selected task → `[{"type":"NODE","target":"107:26328","navigation":"SWAP"}]`
- `107:27861` Feature / Calendar date activity → `[{"type":"NODE","target":"107:26328","navigation":"SWAP"}]`
- `107:27866` Feature / Change calendar month / Today → `[{"type":"NODE","target":"107:26328","navigation":"SWAP"}]`
- `107:27870` Feature / Workspace totals → `[{"type":"NODE","target":"107:26328","navigation":"SWAP"}]`
- `107:27875` Feature / Projects and folders → `[{"type":"NODE","target":"107:26276","navigation":"SWAP"}]`
- `107:27879` Feature / Bulk document operations → `[{"type":"NODE","target":"107:26224","navigation":"SWAP"}]`
- `107:27884` Feature / Audit log → `[{"type":"NODE","target":"107:26302","navigation":"SWAP"}]`
- `107:27888` Feature / Open seeded demo → `[{"type":"NODE","target":"107:26328","navigation":"SWAP"}]`
- `107:27893` Feature / Unsaved Changes → `[{"type":"NODE","target":"107:25918","navigation":"SWAP"}]`
- `107:27897` Feature / Help & about → `[{"type":"NODE","target":"110:21252","navigation":"NAVIGATE"}]`
- `107:27902` Feature / Search workspace records → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`

## ST-083 · Page tools / Map

**Role:** Reference or alternate fixture. **Frame:** 107:27906 · 980 × 894. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-27906).

**Displayed content:**

- Map tools
- Close
- Choose an action. Current document or record context is retained.
- Circular / linear representation
- Document metadata
- Inspect feature / enzyme / primer
- Create feature from range
- Edit selected feature
- Add primer from selection
- Design primer pair
- Simulate PCR
- Virtual digest and gel
- Alignment
- Combinatorial assembly
- CRISPR guides
- Codon optimization
- Detect common features
- Feature library / bulk auto-annotation
- Show features / primers / enzymes / ORFs / translation
- Zoom / fit / center / minimap
- Rotate origin / reverse complement / topology
- Color selection / clear colors
- Genetic code / nucleotide letter help
- Assembly Wizard
- Protein analysis
- BLAST search

**Authored actions:**

- `107:27910` Action / Close → `[{"type":"CLOSE"}]`
- `107:27913` Feature / Circular / linear representation → `[{"type":"NODE","target":"107:27221","navigation":"SWAP"}]`
- `107:27917` Feature / Document metadata → `[{"type":"NODE","target":"107:27221","navigation":"SWAP"}]`
- `107:27922` Feature / Inspect feature / enzyme / primer → `[{"type":"NODE","target":"107:27221","navigation":"SWAP"}]`
- `107:27926` Feature / Create feature from range → `[{"type":"NODE","target":"107:25134","navigation":"SWAP"}]`
- `107:27931` Feature / Edit selected feature → `[{"type":"NODE","target":"107:25134","navigation":"SWAP"}]`
- `107:27935` Feature / Add primer from selection → `[{"type":"NODE","target":"107:25182","navigation":"SWAP"}]`
- `107:27940` Feature / Design primer pair → `[{"type":"NODE","target":"107:25303","navigation":"SWAP"}]`
- `107:27944` Feature / Simulate PCR → `[{"type":"NODE","target":"107:25266","navigation":"SWAP"}]`
- `107:27949` Feature / Virtual digest and gel → `[{"type":"NODE","target":"110:20288","navigation":"NAVIGATE"}]`
- `107:27953` Feature / Alignment → `[{"type":"NODE","target":"107:25425","navigation":"SWAP"}]`
- `107:27958` Feature / Combinatorial assembly → `[{"type":"NODE","target":"107:26112","navigation":"SWAP"}]`
- `107:27962` Feature / CRISPR guides → `[{"type":"NODE","target":"107:25505","navigation":"SWAP"}]`
- `107:27967` Feature / Codon optimization → `[{"type":"NODE","target":"107:25557","navigation":"SWAP"}]`
- `107:27971` Feature / Detect common features → `[{"type":"NODE","target":"107:26060","navigation":"SWAP"}]`
- `107:27976` Feature / Feature library / bulk auto-annotation → `[{"type":"NODE","target":"107:26194","navigation":"SWAP"}]`
- `107:27980` Feature / Show features / primers / enzymes / ORFs / translation → `[{"type":"NODE","target":"107:25217","navigation":"SWAP"}]`
- `107:27985` Feature / Zoom / fit / center / minimap → `[{"type":"NODE","target":"107:27221","navigation":"SWAP"}]`
- `107:27989` Feature / Rotate origin / reverse complement / topology → `[{"type":"NODE","target":"107:26038","navigation":"SWAP"}]`
- `107:27994` Feature / Color selection / clear colors → `[{"type":"NODE","target":"107:26086","navigation":"SWAP"}]`
- `107:27998` Feature / Genetic code / nucleotide letter help → `[{"type":"NODE","target":"107:25938","navigation":"SWAP"}]`
- `107:28003` Feature / Assembly Wizard → `[{"type":"NODE","target":"107:25463","navigation":"SWAP"}]`
- `107:28008` Feature / Protein analysis → `[{"type":"NODE","target":"107:26168","navigation":"SWAP"}]`
- `107:28009` Feature / BLAST search → `[{"type":"NODE","target":"107:25595","navigation":"SWAP"}]`

## ST-084 · Page tools / Sequence

**Role:** Reference or alternate fixture. **Frame:** 107:28010 · 980 × 708. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-28010).

**Displayed content:**

- Sequence tools
- Close
- Choose an action. Current document or record context is retained.
- Select / navigate a range
- Edit / insert sequence
- Copy / cut / paste / delete
- Find DNA / protein / name
- Next / previous search result
- Create feature
- Create primer from range
- Design primer pair / PCR
- Alignment / local BLAST
- Protein analysis / codon optimization
- ORFs / translation / complement / base colors
- Translation code / minimum ORF length
- Zoom / fit / center / minimap
- Custom coloring / strand scope
- Reverse complement / rotate / topology
- Save / save as / export
- Nucleotide letter codes
- Export document

**Authored actions:**

- `107:28014` Action / Close → `[{"type":"CLOSE"}]`
- `107:28017` Feature / Select / navigate a range → `[{"type":"NODE","target":"107:27256","navigation":"SWAP"}]`
- `107:28021` Feature / Edit / insert sequence → `[{"type":"NODE","target":"107:27256","navigation":"SWAP"}]`
- `107:28026` Feature / Copy / cut / paste / delete → `[{"type":"NODE","target":"107:27086","navigation":"SWAP"}]`
- `107:28030` Feature / Find DNA / protein / name → `[{"type":"NODE","target":"107:25986","navigation":"SWAP"}]`
- `107:28035` Feature / Next / previous search result → `[{"type":"NODE","target":"107:25986","navigation":"SWAP"}]`
- `107:28039` Feature / Create feature → `[{"type":"NODE","target":"107:25134","navigation":"SWAP"}]`
- `107:28044` Feature / Create primer from range → `[{"type":"NODE","target":"107:25182","navigation":"SWAP"}]`
- `107:28048` Feature / Design primer pair / PCR → `[{"type":"NODE","target":"107:25303","navigation":"SWAP"}]`
- `107:28053` Feature / Alignment / local BLAST → `[{"type":"NODE","target":"107:25595","navigation":"SWAP"}]`
- `107:28057` Feature / Protein analysis / codon optimization → `[{"type":"NODE","target":"107:25557","navigation":"SWAP"}]`
- `107:28062` Feature / ORFs / translation / complement / base colors → `[{"type":"NODE","target":"107:25217","navigation":"SWAP"}]`
- `107:28066` Feature / Translation code / minimum ORF length → `[{"type":"NODE","target":"107:25938","navigation":"SWAP"}]`
- `107:28071` Feature / Zoom / fit / center / minimap → `[{"type":"NODE","target":"107:27256","navigation":"SWAP"}]`
- `107:28075` Feature / Custom coloring / strand scope → `[{"type":"NODE","target":"107:26086","navigation":"SWAP"}]`
- `107:28080` Feature / Reverse complement / rotate / topology → `[{"type":"NODE","target":"107:26038","navigation":"SWAP"}]`
- `107:28084` Feature / Save / save as / export → `[{"type":"NODE","target":"107:27256","navigation":"SWAP"}]`
- `107:28089` Feature / Nucleotide letter codes → `[{"type":"NODE","target":"107:25960","navigation":"SWAP"}]`
- `107:28093` Feature / Export document → `[{"type":"NODE","target":"107:27142","navigation":"SWAP"}]`

## ST-085 · Page tools / Features

**Role:** Linked functional state. **Frame:** 107:28097 · 980 × 584. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-28097).

**Displayed content:**

- Features tools
- Close
- Choose an action. Current document or record context is retained.
- Add feature
- Edit feature
- Delete feature
- Detect common features
- Show / hide feature
- Filter annotations
- Sort annotations
- Configure columns
- Save current table view
- Load / delete saved view
- Translate feature / choose code / frame
- Color / product / notes / split segments
- Reveal in Map or Sequence
- Motif library / bulk auto-annotate

**Authored actions:**

- `107:28101` Action / Close → `[{"type":"CLOSE"}]`
- `107:28104` Feature / Add feature → `[{"type":"NODE","target":"107:25134","navigation":"SWAP"}]`
- `107:28108` Feature / Edit feature → `[{"type":"NODE","target":"107:25134","navigation":"SWAP"}]`
- `107:28113` Feature / Delete feature → `[{"type":"NODE","target":"107:27086","navigation":"SWAP"}]`
- `107:28117` Feature / Detect common features → `[{"type":"NODE","target":"107:26060","navigation":"SWAP"}]`
- `107:28122` Feature / Show / hide feature → `[{"type":"NODE","target":"107:27291","navigation":"SWAP"}]`
- `107:28126` Feature / Filter annotations → `[{"type":"NODE","target":"107:26948","navigation":"SWAP"}]`
- `107:28131` Feature / Sort annotations → `[{"type":"NODE","target":"107:26948","navigation":"SWAP"}]`
- `107:28135` Feature / Configure columns → `[{"type":"NODE","target":"107:26948","navigation":"SWAP"}]`
- `107:28140` Feature / Save current table view → `[{"type":"NODE","target":"107:27291","navigation":"SWAP"}]`
- `107:28144` Feature / Load / delete saved view → `[{"type":"NODE","target":"107:27086","navigation":"SWAP"}]`
- `107:28149` Feature / Translate feature / choose code / frame → `[{"type":"NODE","target":"107:25938","navigation":"SWAP"}]`
- `107:28153` Feature / Color / product / notes / split segments → `[{"type":"NODE","target":"107:25134","navigation":"SWAP"}]`
- `107:28158` Feature / Reveal in Map or Sequence → `[{"type":"NODE","target":"107:27291","navigation":"SWAP"}]`
- `107:28162` Feature / Motif library / bulk auto-annotate → `[{"type":"NODE","target":"107:26194","navigation":"SWAP"}]`

## ST-086 · Page tools / Primers

**Role:** Linked functional state. **Frame:** 107:28166 · 980 × 584. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-28166).

**Displayed content:**

- Primers tools
- Close
- Choose an action. Current document or record context is retained.
- Add primer manually
- Primer from sequence selection
- Edit primer
- Delete primer
- Design forward/reverse pair
- Review hairpin / self-dimer
- Inspect Tm / GC / binding sites
- Use selected pair in PCR
- Filter / sort primers
- Show / hide primer
- Columns / saved views
- Show binding site in sequence
- Overhang / phosphorylation / description

**Authored actions:**

- `107:28170` Action / Close → `[{"type":"CLOSE"}]`
- `107:28173` Feature / Add primer manually → `[{"type":"NODE","target":"107:25182","navigation":"SWAP"}]`
- `107:28177` Feature / Primer from sequence selection → `[{"type":"NODE","target":"107:25182","navigation":"SWAP"}]`
- `107:28182` Feature / Edit primer → `[{"type":"NODE","target":"107:25182","navigation":"SWAP"}]`
- `107:28186` Feature / Delete primer → `[{"type":"NODE","target":"107:27086","navigation":"SWAP"}]`
- `107:28191` Feature / Design forward/reverse pair → `[{"type":"NODE","target":"107:25303","navigation":"SWAP"}]`
- `107:28195` Feature / Review hairpin / self-dimer → `[{"type":"NODE","target":"107:27326","navigation":"SWAP"}]`
- `107:28200` Feature / Inspect Tm / GC / binding sites → `[{"type":"NODE","target":"107:27326","navigation":"SWAP"}]`
- `107:28204` Feature / Use selected pair in PCR → `[{"type":"NODE","target":"107:25266","navigation":"SWAP"}]`
- `107:28209` Feature / Filter / sort primers → `[{"type":"NODE","target":"107:26948","navigation":"SWAP"}]`
- `107:28213` Feature / Show / hide primer → `[{"type":"NODE","target":"107:27326","navigation":"SWAP"}]`
- `107:28218` Feature / Columns / saved views → `[{"type":"NODE","target":"107:26948","navigation":"SWAP"}]`
- `107:28222` Feature / Show binding site in sequence → `[{"type":"NODE","target":"107:27326","navigation":"SWAP"}]`
- `107:28227` Feature / Overhang / phosphorylation / description → `[{"type":"NODE","target":"107:25182","navigation":"SWAP"}]`

## ST-087 · Page tools / Enzymes

**Role:** Linked functional state. **Frame:** 107:28231 · 980 × 584. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-28231).

**Displayed content:**

- Enzymes tools
- Close
- Choose an action. Current document or record context is retained.
- Choose / apply enzyme set
- Toggle enzyme membership
- Filter by enzyme name
- Filter cut count
- Filter overhang type
- Filter recognition length
- Filter a coordinate region
- Check / uncheck shown enzymes
- Clear chosen set
- Inspect cut position on map
- Virtual digest and gel
- Columns / saved views
- Sort enzymes

**Authored actions:**

- `107:28235` Action / Close → `[{"type":"CLOSE"}]`
- `107:28238` Feature / Choose / apply enzyme set → `[{"type":"NODE","target":"107:27361","navigation":"SWAP"}]`
- `107:28242` Feature / Toggle enzyme membership → `[{"type":"NODE","target":"107:27361","navigation":"SWAP"}]`
- `107:28247` Feature / Filter by enzyme name → `[{"type":"NODE","target":"107:26978","navigation":"SWAP"}]`
- `107:28251` Feature / Filter cut count → `[{"type":"NODE","target":"107:26978","navigation":"SWAP"}]`
- `107:28256` Feature / Filter overhang type → `[{"type":"NODE","target":"107:26978","navigation":"SWAP"}]`
- `107:28260` Feature / Filter recognition length → `[{"type":"NODE","target":"107:26978","navigation":"SWAP"}]`
- `107:28265` Feature / Filter a coordinate region → `[{"type":"NODE","target":"107:26978","navigation":"SWAP"}]`
- `107:28269` Feature / Check / uncheck shown enzymes → `[{"type":"NODE","target":"107:27361","navigation":"SWAP"}]`
- `107:28274` Feature / Clear chosen set → `[{"type":"NODE","target":"107:27361","navigation":"SWAP"}]`
- `107:28278` Feature / Inspect cut position on map → `[{"type":"NODE","target":"107:27361","navigation":"SWAP"}]`
- `107:28283` Feature / Virtual digest and gel → `[{"type":"NODE","target":"110:20288","navigation":"NAVIGATE"}]`
- `107:28287` Feature / Columns / saved views → `[{"type":"NODE","target":"107:26948","navigation":"SWAP"}]`
- `107:28292` Feature / Sort enzymes → `[{"type":"NODE","target":"107:26948","navigation":"SWAP"}]`

## ST-088 · Page tools / History

**Role:** Linked functional state. **Frame:** 107:28296 · 980 × 398. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-28296).

**Displayed content:**

- History tools
- Close
- Choose an action. Current document or record context is retained.
- Undo latest edit
- Redo edit
- Inspect affected regions
- Revert to before operation
- Save a version snapshot
- Restore saved version
- Open current state

**Authored actions:**

- `107:28300` Action / Close → `[{"type":"CLOSE"}]`
- `107:28303` Feature / Undo latest edit → `[{"type":"NODE","target":"107:27396","navigation":"SWAP"}]`
- `107:28307` Feature / Redo edit → `[{"type":"NODE","target":"107:27396","navigation":"SWAP"}]`
- `107:28312` Feature / Inspect affected regions → `[{"type":"NODE","target":"107:27396","navigation":"SWAP"}]`
- `107:28316` Feature / Revert to before operation → `[{"type":"NODE","target":"107:27396","navigation":"SWAP"}]`
- `107:28321` Feature / Save a version snapshot → `[{"type":"NODE","target":"107:29025","navigation":"SWAP"}]`
- `107:28325` Feature / Restore saved version → `[{"type":"NODE","target":"107:27396","navigation":"SWAP"}]`
- `107:28330` Feature / Open current state → `[{"type":"NODE","target":"107:27396","navigation":"SWAP"}]`

## ST-089 · Page tools / Notebook

**Role:** Linked functional state. **Frame:** 107:28334 · 980 × 584. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-28334).

**Displayed content:**

- Notebook tools
- Close
- Choose an action. Current document or record context is retained.
- Create entry
- Choose an entry template
- Open / edit existing entry
- Save draft
- Submit entry for review
- Request changes / approve
- Resubmit changed entry
- Structured result tables
- Write table back to registry
- Open linked plate snapshot
- Project association
- Sections / result authoring controls
- Choose notebook entry

**Authored actions:**

- `107:28338` Action / Close → `[{"type":"CLOSE"}]`
- `107:28341` Feature / Create entry → `[{"type":"NODE","target":"110:20232","navigation":"NAVIGATE"}]`
- `107:28345` Feature / Choose an entry template → `[{"type":"NODE","target":"107:26892","navigation":"SWAP"}]`
- `107:28350` Feature / Open / edit existing entry → `[{"type":"NODE","target":"110:20232","navigation":"NAVIGATE"}]`
- `107:28354` Feature / Save draft → `[{"type":"NODE","target":"110:20232","navigation":"NAVIGATE"}]`
- `107:28359` Feature / Submit entry for review → `[{"type":"NODE","target":"110:20232","navigation":"NAVIGATE"}]`
- `107:28363` Feature / Request changes / approve → `[{"type":"NODE","target":"110:20232","navigation":"NAVIGATE"}]`
- `107:28368` Feature / Resubmit changed entry → `[{"type":"NODE","target":"110:20232","navigation":"NAVIGATE"}]`
- `107:28372` Feature / Structured result tables → `[{"type":"NODE","target":"110:20232","navigation":"NAVIGATE"}]`
- `107:28377` Feature / Write table back to registry → `[{"type":"NODE","target":"110:20232","navigation":"NAVIGATE"}]`
- `107:28381` Feature / Open linked plate snapshot → `[{"type":"NODE","target":"110:20232","navigation":"NAVIGATE"}]`
- `107:28386` Feature / Project association → `[{"type":"NODE","target":"107:26276","navigation":"SWAP"}]`
- `107:28390` Feature / Sections / result authoring controls → `[{"type":"NODE","target":"110:20232","navigation":"NAVIGATE"}]`
- `107:28395` Feature / Choose notebook entry → `[{"type":"NODE","target":"107:26250","navigation":"SWAP"}]`

## ST-090 · Page tools / Registry

**Role:** Linked functional state. **Frame:** 107:28399 · 980 × 522. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-28399).

**Displayed content:**

- Registry tools
- Close
- Choose an action. Current document or record context is retained.
- Add plasmid / strain / primer
- Edit entity fields
- Filter kind / entity text
- Archive / unarchive
- Delete entity
- Link active document
- Open linked document
- Import CSV
- Entity relationships
- Workspace entity map
- Find physical containers
- Schema-specific metadata

**Authored actions:**

- `107:28403` Action / Close → `[{"type":"CLOSE"}]`
- `107:28406` Feature / Add plasmid / strain / primer → `[{"type":"NODE","target":"107:25875","navigation":"SWAP"}]`
- `107:28410` Feature / Edit entity fields → `[{"type":"NODE","target":"107:25875","navigation":"SWAP"}]`
- `107:28415` Feature / Filter kind / entity text → `[{"type":"NODE","target":"107:26948","navigation":"SWAP"}]`
- `107:28419` Feature / Archive / unarchive → `[{"type":"NODE","target":"107:27086","navigation":"SWAP"}]`
- `107:28424` Feature / Delete entity → `[{"type":"NODE","target":"107:27086","navigation":"SWAP"}]`
- `107:28428` Feature / Link active document → `[{"type":"NODE","target":"107:26142","navigation":"SWAP"}]`
- `107:28433` Feature / Open linked document → `[{"type":"NODE","target":"107:26142","navigation":"SWAP"}]`
- `107:28437` Feature / Import CSV → `[{"type":"NODE","target":"107:26380","navigation":"SWAP"}]`
- `107:28442` Feature / Entity relationships → `[{"type":"NODE","target":"107:26354","navigation":"SWAP"}]`
- `107:28446` Feature / Workspace entity map → `[{"type":"NODE","target":"107:26354","navigation":"SWAP"}]`
- `107:28451` Feature / Find physical containers → `[{"type":"NODE","target":"107:25650","navigation":"SWAP"}]`
- `107:28455` Feature / Schema-specific metadata → `[{"type":"NODE","target":"107:25875","navigation":"SWAP"}]`

## ST-091 · Page tools / Inventory

**Role:** Linked functional state. **Frame:** 107:28459 · 980 × 522. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-28459).

**Displayed content:**

- Inventory tools
- Close
- Choose an action. Current document or record context is retained.
- Add location / box / plate / tube
- Browse hierarchy
- Inspect selected container
- Find containers for an entity
- View selected plate wells
- Assign / fill wells
- Inspect well contents
- Snapshot plate into notebook
- Review physical quantity and concentration
- Move container / change placement
- Open linked entity

**Authored actions:**

- `107:28463` Action / Close → `[{"type":"CLOSE"}]`
- `107:28466` Feature / Add location / box / plate / tube → `[{"type":"NODE","target":"107:27008","navigation":"SWAP"}]`
- `107:28470` Feature / Browse hierarchy → `[{"type":"NODE","target":"107:25650","navigation":"SWAP"}]`
- `107:28475` Feature / Inspect selected container → `[{"type":"NODE","target":"107:25650","navigation":"SWAP"}]`
- `107:28479` Feature / Find containers for an entity → `[{"type":"NODE","target":"107:25650","navigation":"SWAP"}]`
- `107:28484` Feature / View selected plate wells → `[{"type":"NODE","target":"107:25650","navigation":"SWAP"}]`
- `107:28488` Feature / Assign / fill wells → `[{"type":"NODE","target":"107:26858","navigation":"SWAP"}]`
- `107:28493` Feature / Inspect well contents → `[{"type":"NODE","target":"107:25650","navigation":"SWAP"}]`
- `107:28497` Feature / Snapshot plate into notebook → `[{"type":"NODE","target":"107:25650","navigation":"SWAP"}]`
- `107:28502` Feature / Review physical quantity and concentration → `[{"type":"NODE","target":"107:26858","navigation":"SWAP"}]`
- `107:28506` Feature / Move container / change placement → `[{"type":"NODE","target":"107:26858","navigation":"SWAP"}]`
- `107:28511` Feature / Open linked entity → `[{"type":"NODE","target":"107:25650","navigation":"SWAP"}]`

## ST-092 · Page tools / Workflows

**Role:** Linked functional state. **Frame:** 107:28515 · 980 × 522. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-28515).

**Displayed content:**

- Workflows tools
- Close
- Choose an action. Current document or record context is retained.
- Create task
- Select / inspect task
- Start task
- Complete with output
- Mark failed task
- Draft request
- Send request
- Inspect request state
- View execution flow
- Execute next step
- Review output / retry context

**Authored actions:**

- `107:28519` Action / Close → `[{"type":"CLOSE"}]`
- `107:28522` Feature / Create task → `[{"type":"NODE","target":"107:27034","navigation":"SWAP"}]`
- `107:28526` Feature / Select / inspect task → `[{"type":"NODE","target":"107:26410","navigation":"SWAP"}]`
- `107:28531` Feature / Start task → `[{"type":"NODE","target":"107:26410","navigation":"SWAP"}]`
- `107:28535` Feature / Complete with output → `[{"type":"NODE","target":"107:26410","navigation":"SWAP"}]`
- `107:28540` Feature / Mark failed task → `[{"type":"NODE","target":"107:26410","navigation":"SWAP"}]`
- `107:28544` Feature / Draft request → `[{"type":"NODE","target":"107:27060","navigation":"SWAP"}]`
- `107:28549` Feature / Send request → `[{"type":"NODE","target":"107:26410","navigation":"SWAP"}]`
- `107:28553` Feature / Inspect request state → `[{"type":"NODE","target":"107:26410","navigation":"SWAP"}]`
- `107:28558` Feature / View execution flow → `[{"type":"NODE","target":"107:26410","navigation":"SWAP"}]`
- `107:28562` Feature / Execute next step → `[{"type":"NODE","target":"107:26410","navigation":"SWAP"}]`
- `107:28567` Feature / Review output / retry context → `[{"type":"NODE","target":"107:26410","navigation":"SWAP"}]`

## ST-093 · Page tools / Insights

**Role:** Linked functional state. **Frame:** 107:28571 · 980 × 336. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-28571).

**Displayed content:**

- Insights tools
- Close
- Choose an action. Current document or record context is retained.
- Add a registry / inventory / tasks / entries insight
- Inspect chart source and grouping
- View grouped counts
- Export source CSV
- Change source/grouping
- Remove a chart

**Authored actions:**

- `107:28575` Action / Close → `[{"type":"CLOSE"}]`
- `107:28578` Feature / Add a registry / inventory / tasks / entries insight → `[{"type":"NODE","target":"107:26918","navigation":"SWAP"}]`
- `107:28582` Feature / Inspect chart source and grouping → `[{"type":"NODE","target":"107:26918","navigation":"SWAP"}]`
- `107:28587` Feature / View grouped counts → `[{"type":"NODE","target":"107:26918","navigation":"SWAP"}]`
- `107:28591` Feature / Export source CSV → `[{"type":"NODE","target":"107:29047","navigation":"SWAP"}]`
- `107:28596` Feature / Change source/grouping → `[{"type":"NODE","target":"107:26918","navigation":"SWAP"}]`
- `107:28600` Feature / Remove a chart → `[{"type":"NODE","target":"107:26918","navigation":"SWAP"}]`

## ST-094 · Page tools / Automation

**Role:** Linked functional state. **Frame:** 107:28604 · 980 × 460. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-28604).

**Displayed content:**

- Automation tools
- Close
- Choose an action. Current document or record context is retained.
- Create pipeline
- Select pipeline
- Load CSV
- Configure transforms and destination
- Publish draft
- Run selected pipeline
- Inspect recent run
- Inspect error / correct input
- Audit pipeline operations
- Unpublish / edit published pipeline

**Authored actions:**

- `107:28608` Action / Close → `[{"type":"CLOSE"}]`
- `107:28611` Feature / Create pipeline → `[{"type":"NODE","target":"107:26716","navigation":"SWAP"}]`
- `107:28615` Feature / Select pipeline → `[{"type":"NODE","target":"107:26716","navigation":"SWAP"}]`
- `107:28620` Feature / Load CSV → `[{"type":"NODE","target":"107:26716","navigation":"SWAP"}]`
- `107:28624` Feature / Configure transforms and destination → `[{"type":"NODE","target":"107:26716","navigation":"SWAP"}]`
- `107:28629` Feature / Publish draft → `[{"type":"NODE","target":"107:26716","navigation":"SWAP"}]`
- `107:28633` Feature / Run selected pipeline → `[{"type":"NODE","target":"107:33978","navigation":"SWAP"}]`
- `107:28638` Feature / Inspect recent run → `[{"type":"NODE","target":"107:26746","navigation":"SWAP"}]`
- `107:28642` Feature / Inspect error / correct input → `[{"type":"NODE","target":"107:26746","navigation":"SWAP"}]`
- `107:28647` Feature / Audit pipeline operations → `[{"type":"NODE","target":"107:26302","navigation":"SWAP"}]`
- `107:28651` Feature / Unpublish / edit published pipeline → `[{"type":"NODE","target":"107:26716","navigation":"SWAP"}]`

## ST-095 · Page tools / Split

**Role:** Linked functional state. **Frame:** 107:28655 · 980 × 336. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-28655).

**Displayed content:**

- Split tools
- Close
- Choose an action. Current document or record context is retained.
- Choose comparison document
- Open second document
- Inspect left or right map
- Open chosen pane in editor
- Align compared documents
- Switch map / sequence representation

**Authored actions:**

- `107:28659` Action / Close → `[{"type":"CLOSE"}]`
- `107:28662` Feature / Choose comparison document → `[{"type":"NODE","target":"107:27625","navigation":"SWAP"}]`
- `107:28666` Feature / Open second document → `[{"type":"NODE","target":"107:27625","navigation":"SWAP"}]`
- `107:28671` Feature / Inspect left or right map → `[{"type":"NODE","target":"107:27625","navigation":"SWAP"}]`
- `107:28675` Feature / Open chosen pane in editor → `[{"type":"NODE","target":"107:27625","navigation":"SWAP"}]`
- `107:28680` Feature / Align compared documents → `[{"type":"NODE","target":"107:25425","navigation":"SWAP"}]`
- `107:28684` Feature / Switch map / sequence representation → `[{"type":"NODE","target":"107:27625","navigation":"SWAP"}]`

## ST-096 · Page tools / Biologics

**Role:** Linked functional state. **Frame:** 107:28688 · 980 × 398. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-28688).

**Displayed content:**

- Biologics tools
- Close
- Choose an action. Current document or record context is retained.
- Register IgG / bispecific / nanobody / TCR / Fab
- Inspect entity overview
- View chains and sequence
- Inspect domains
- Inspect numbering
- Delete biologic
- Analyze selected chain

**Authored actions:**

- `107:28692` Action / Close → `[{"type":"CLOSE"}]`
- `107:28695` Feature / Register IgG / bispecific / nanobody / TCR / Fab → `[{"type":"NODE","target":"107:26448","navigation":"SWAP"}]`
- `107:28699` Feature / Inspect entity overview → `[{"type":"NODE","target":"107:26168","navigation":"SWAP"}]`
- `107:28704` Feature / View chains and sequence → `[{"type":"NODE","target":"107:26168","navigation":"SWAP"}]`
- `107:28708` Feature / Inspect domains → `[{"type":"NODE","target":"107:26168","navigation":"SWAP"}]`
- `107:28713` Feature / Inspect numbering → `[{"type":"NODE","target":"107:26168","navigation":"SWAP"}]`
- `107:28717` Feature / Delete biologic → `[{"type":"NODE","target":"107:27086","navigation":"SWAP"}]`
- `107:28722` Feature / Analyze selected chain → `[{"type":"NODE","target":"107:26168","navigation":"SWAP"}]`

## ST-097 · Page tools / Chemistry

**Role:** Linked functional state. **Frame:** 107:28726 · 980 × 460. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-28726).

**Displayed content:**

- Chemistry tools
- Close
- Choose an action. Current document or record context is retained.
- Add compound from SMILES
- Substructure search
- Select compound / inspect structure
- Inspect MW / donors / acceptors / atoms
- Add material lot
- Inspect compound lots
- Add reaction
- Inspect reaction participants
- Delete compound / reaction
- Browse salt / solvate library

**Authored actions:**

- `107:28730` Action / Close → `[{"type":"CLOSE"}]`
- `107:28733` Feature / Add compound from SMILES → `[{"type":"NODE","target":"107:26478","navigation":"SWAP"}]`
- `107:28737` Feature / Substructure search → `[{"type":"NODE","target":"107:26478","navigation":"SWAP"}]`
- `107:28742` Feature / Select compound / inspect structure → `[{"type":"NODE","target":"107:26478","navigation":"SWAP"}]`
- `107:28746` Feature / Inspect MW / donors / acceptors / atoms → `[{"type":"NODE","target":"107:26478","navigation":"SWAP"}]`
- `107:28751` Feature / Add material lot → `[{"type":"NODE","target":"107:26534","navigation":"SWAP"}]`
- `107:28755` Feature / Inspect compound lots → `[{"type":"NODE","target":"107:26534","navigation":"SWAP"}]`
- `107:28760` Feature / Add reaction → `[{"type":"NODE","target":"107:26504","navigation":"SWAP"}]`
- `107:28764` Feature / Inspect reaction participants → `[{"type":"NODE","target":"107:26504","navigation":"SWAP"}]`
- `107:28769` Feature / Delete compound / reaction → `[{"type":"NODE","target":"107:27086","navigation":"SWAP"}]`
- `107:28773` Feature / Browse salt / solvate library → `[{"type":"NODE","target":"107:29073","navigation":"SWAP"}]`

## ST-098 · Page tools / RNA / Oligos

**Role:** Linked functional state. **Frame:** 107:28777 · 980 × 398. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-28777).

**Displayed content:**

- RNA / Oligos tools
- Close
- Choose an action. Current document or record context is retained.
- Add oligo / supported HELM input
- Select oligo / inspect sequence
- View MW and GC
- Inspect HELM representation
- Inspect individual modifications
- Apply bulk modification pattern
- Browse monomer / modification library
- Delete oligo

**Authored actions:**

- `107:28781` Action / Close → `[{"type":"CLOSE"}]`
- `107:28784` Feature / Add oligo / supported HELM input → `[{"type":"NODE","target":"107:26560","navigation":"SWAP"}]`
- `107:28788` Feature / Select oligo / inspect sequence → `[{"type":"NODE","target":"107:26560","navigation":"SWAP"}]`
- `107:28793` Feature / View MW and GC → `[{"type":"NODE","target":"107:26560","navigation":"SWAP"}]`
- `107:28797` Feature / Inspect HELM representation → `[{"type":"NODE","target":"107:26560","navigation":"SWAP"}]`
- `107:28802` Feature / Inspect individual modifications → `[{"type":"NODE","target":"107:26560","navigation":"SWAP"}]`
- `107:28806` Feature / Apply bulk modification pattern → `[{"type":"NODE","target":"107:26828","navigation":"SWAP"}]`
- `107:28811` Feature / Browse monomer / modification library → `[{"type":"NODE","target":"107:29099","navigation":"SWAP"}]`
- `107:28815` Feature / Delete oligo → `[{"type":"NODE","target":"107:27086","navigation":"SWAP"}]`

## ST-099 · Page tools / In Vivo

**Role:** Linked functional state. **Frame:** 107:28819 · 980 × 460. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-28819).

**Displayed content:**

- In Vivo tools
- Close
- Choose an action. Current document or record context is retained.
- Create study
- Select study
- Add animal
- Record measurement
- Inspect animals and current weights
- Randomize cohorts
- Inspect mean weight by day
- Inspect welfare alerts
- Open dosing calculator
- Delete study

**Authored actions:**

- `107:28823` Action / Close → `[{"type":"CLOSE"}]`
- `107:28826` Feature / Create study → `[{"type":"NODE","target":"107:26590","navigation":"SWAP"}]`
- `107:28830` Feature / Select study → `[{"type":"NODE","target":"107:26590","navigation":"SWAP"}]`
- `107:28835` Feature / Add animal → `[{"type":"NODE","target":"107:26612","navigation":"SWAP"}]`
- `107:28839` Feature / Record measurement → `[{"type":"NODE","target":"107:26638","navigation":"SWAP"}]`
- `107:28844` Feature / Inspect animals and current weights → `[{"type":"NODE","target":"107:26638","navigation":"SWAP"}]`
- `107:28848` Feature / Randomize cohorts → `[{"type":"NODE","target":"107:26802","navigation":"SWAP"}]`
- `107:28853` Feature / Inspect mean weight by day → `[{"type":"NODE","target":"107:26638","navigation":"SWAP"}]`
- `107:28857` Feature / Inspect welfare alerts → `[{"type":"NODE","target":"107:26638","navigation":"SWAP"}]`
- `107:28862` Feature / Open dosing calculator → `[{"type":"NODE","target":"107:27112","navigation":"SWAP"}]`
- `107:28866` Feature / Delete study → `[{"type":"NODE","target":"107:27086","navigation":"SWAP"}]`

## ST-100 · Page tools / Bioprocess

**Role:** Linked functional state. **Frame:** 107:28870 · 980 × 522. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-28870).

**Displayed content:**

- Bioprocess tools
- Close
- Choose an action. Current document or record context is retained.
- Create recipe
- Inspect recipe operations/duration
- Plan run from chosen recipe
- Select run
- Start planned run
- View / complete worksheet operation
- Add replicate
- Inspect replicate conditions/results
- Inspect mean titer
- Cross-run titer comparison
- Complete / abort run
- Delete run

**Authored actions:**

- `107:28874` Action / Close → `[{"type":"CLOSE"}]`
- `107:28877` Feature / Create recipe → `[{"type":"NODE","target":"107:26664","navigation":"SWAP"}]`
- `107:28881` Feature / Inspect recipe operations/duration → `[{"type":"NODE","target":"107:26664","navigation":"SWAP"}]`
- `107:28886` Feature / Plan run from chosen recipe → `[{"type":"NODE","target":"107:26776","navigation":"SWAP"}]`
- `107:28890` Feature / Select run → `[{"type":"NODE","target":"107:26776","navigation":"SWAP"}]`
- `107:28895` Feature / Start planned run → `[{"type":"NODE","target":"107:26776","navigation":"SWAP"}]`
- `107:28899` Feature / View / complete worksheet operation → `[{"type":"NODE","target":"107:26776","navigation":"SWAP"}]`
- `107:28904` Feature / Add replicate → `[{"type":"NODE","target":"107:26686","navigation":"SWAP"}]`
- `107:28908` Feature / Inspect replicate conditions/results → `[{"type":"NODE","target":"107:26686","navigation":"SWAP"}]`
- `107:28913` Feature / Inspect mean titer → `[{"type":"NODE","target":"107:26686","navigation":"SWAP"}]`
- `107:28917` Feature / Cross-run titer comparison → `[{"type":"NODE","target":"107:26686","navigation":"SWAP"}]`
- `107:28922` Feature / Complete / abort run → `[{"type":"NODE","target":"107:26776","navigation":"SWAP"}]`
- `107:28926` Feature / Delete run → `[{"type":"NODE","target":"107:27086","navigation":"SWAP"}]`

## ST-101 · Action panel / Analyze

**Role:** Reference or alternate fixture. **Frame:** 107:28930 · 720 × 650. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-28930).

**Displayed content:**

- Analyze
- Close
- PCR simulation
- Primer design
- Virtual digest & gel
- Alignment
- Assembly wizard
- Combinatorial assembly
- CRISPR guides
- Codon optimization
- Protein analysis
- Local BLAST

**Authored actions:**

- `107:28934` Action / Close → `[{"type":"CLOSE"}]`
- `107:28935` Action / PCR simulation → `[{"type":"NODE","target":"107:25266","navigation":"SWAP"}]`
- `107:28936` Action / Primer design → `[{"type":"NODE","target":"107:25303","navigation":"SWAP"}]`
- `107:28937` Action / Virtual digest & gel → `[{"type":"NODE","target":"110:20288","navigation":"NAVIGATE"}]`
- `107:28938` Action / Alignment → `[{"type":"NODE","target":"107:25425","navigation":"SWAP"}]`
- `107:28939` Action / Assembly wizard → `[{"type":"NODE","target":"107:25463","navigation":"SWAP"}]`
- `107:28940` Action / Combinatorial assembly → `[{"type":"NODE","target":"107:26112","navigation":"SWAP"}]`
- `107:28941` Action / CRISPR guides → `[{"type":"NODE","target":"107:25505","navigation":"SWAP"}]`
- `107:28942` Action / Codon optimization → `[{"type":"NODE","target":"107:25557","navigation":"SWAP"}]`
- `107:28943` Action / Protein analysis → `[{"type":"NODE","target":"107:26168","navigation":"SWAP"}]`
- `107:28944` Action / Local BLAST → `[{"type":"NODE","target":"107:25595","navigation":"SWAP"}]`

## ST-102 · Action panel / Annotations

**Role:** Reference or alternate fixture. **Frame:** 107:28945 · 720 × 300. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-28945).

**Displayed content:**

- Annotations
- Close
- Add feature
- Detect common features
- Feature library

**Authored actions:**

- `107:28949` Action / Close → `[{"type":"CLOSE"}]`
- `107:28950` Action / Add feature → `[{"type":"NODE","target":"107:25134","navigation":"SWAP"}]`
- `107:28951` Action / Detect common features → `[{"type":"NODE","target":"107:26060","navigation":"SWAP"}]`
- `107:28952` Action / Feature library → `[{"type":"NODE","target":"107:26194","navigation":"SWAP"}]`

## ST-103 · Action panel / Selection actions

**Role:** Reference or alternate fixture. **Frame:** 107:28953 · 720 × 350. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-28953).

**Displayed content:**

- Selection actions
- Close
- Create feature
- Add primer
- Color selection
- Clear custom colors

**Authored actions:**

- `107:28957` Action / Close → `[{"type":"CLOSE"}]`
- `107:28958` Action / Create feature → `[{"type":"NODE","target":"107:25134","navigation":"SWAP"}]`
- `107:28959` Action / Add primer → `[{"type":"NODE","target":"107:25182","navigation":"SWAP"}]`
- `107:28960` Action / Color selection → `[{"type":"NODE","target":"107:26086","navigation":"SWAP"}]`
- `107:28961` Action / Clear custom colors → `[{"type":"NODE","target":"107:25217","navigation":"SWAP"}]`

## ST-104 · Action panel / Sequence actions

**Role:** Reference or alternate fixture. **Frame:** 107:28962 · 720 × 300. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-28962).

**Displayed content:**

- Sequence actions
- Close
- Rotate origin
- Reverse complement
- Change topology

**Authored actions:**

- `107:28966` Action / Close → `[{"type":"CLOSE"}]`
- `107:28967` Action / Rotate origin → `[{"type":"NODE","target":"107:26038","navigation":"SWAP"}]`
- `107:28968` Action / Reverse complement → `[{"type":"NODE","target":"107:27086","navigation":"SWAP"}]`
- `107:28969` Action / Change topology → `[{"type":"NODE","target":"107:25217","navigation":"SWAP"}]`

## ST-105 · Action panel / Workspace actions

**Role:** Reference or alternate fixture. **Frame:** 107:28970 · 720 × 350. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-28970).

**Displayed content:**

- Workspace actions
- Close
- Projects & folders
- Bulk document actions
- Audit log
- Resume recent work

**Authored actions:**

- `107:28974` Action / Close → `[{"type":"CLOSE"}]`
- `107:28975` Action / Projects & folders → `[{"type":"NODE","target":"107:26276","navigation":"SWAP"}]`
- `107:28976` Action / Bulk document actions → `[{"type":"NODE","target":"107:26224","navigation":"SWAP"}]`
- `107:28977` Action / Audit log → `[{"type":"NODE","target":"107:26302","navigation":"SWAP"}]`
- `107:28978` Action / Resume recent work → `[{"type":"NODE","target":"107:26328","navigation":"SWAP"}]`

## ST-106 · Action panel / New

**Role:** Linked functional state. **Frame:** 107:28979 · 720 × 400. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-28979).

**Displayed content:**

- New
- Close
- DNA / protein sequence
- Notebook entry
- Registry entity
- Workflow task
- Storage container

**Authored actions:**

- `107:28983` Action / Close → `[{"type":"CLOSE"}]`
- `107:28984` Action / DNA / protein sequence → `[{"type":"NODE","target":"110:20202","navigation":"NAVIGATE"}]`
- `107:28985` Action / Notebook entry → `[{"type":"NODE","target":"110:20232","navigation":"NAVIGATE"}]`
- `107:28986` Action / Registry entity → `[{"type":"NODE","target":"107:25875","navigation":"SWAP"}]`
- `107:28987` Action / Workflow task → `[{"type":"NODE","target":"107:27034","navigation":"SWAP"}]`
- `107:28988` Action / Storage container → `[{"type":"NODE","target":"107:27008","navigation":"SWAP"}]`

## ST-107 · Action panel / Document actions

**Role:** Reference or alternate fixture. **Frame:** 107:28989 · 720 × 400. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-28989).

**Displayed content:**

- Document actions
- Close
- Save as
- Export document
- Rotate origin
- Genetic code
- Letter codes

**Authored actions:**

- `107:28993` Action / Close → `[{"type":"CLOSE"}]`
- `107:28994` Action / Save as → `[{"type":"NODE","target":"107:27142","navigation":"SWAP"}]`
- `107:28995` Action / Export document → `[{"type":"NODE","target":"107:27142","navigation":"SWAP"}]`
- `107:28996` Action / Rotate origin → `[{"type":"NODE","target":"107:26038","navigation":"SWAP"}]`
- `107:28997` Action / Genetic code → `[{"type":"NODE","target":"107:25938","navigation":"SWAP"}]`
- `107:28998` Action / Letter codes → `[{"type":"NODE","target":"107:25960","navigation":"SWAP"}]`

## ST-108 · Form / Open / import sequence

**Role:** Reference or alternate fixture. **Frame:** 107:28999 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-28999).

**Displayed content:**

- Open / import sequence
- Close
- File
- Choose a local sequence file
- Supported formats
- .hbx · GenBank · FASTA · raw sequence
- Destination
- Open as a new document
- Existing work
- Unsaved documents stay protected
- Cancel
- Open file

**Authored actions:**

- `107:29003` Action / Close → `[{"type":"CLOSE"}]`
- `107:29023` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:29024` Action / Open file → `[{"type":"NODE","target":"128:7904","navigation":"NAVIGATE"}]`

## ST-109 · Form / Save document version

**Role:** Linked functional state. **Frame:** 107:29025 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-29025).

**Displayed content:**

- Save document version
- Close
- Document
- pHelixBench-Demo
- Version name
- Annotated expression cassette
- Version note
- Describe this checkpoint
- Cancel
- Save version

**Authored actions:**

- `107:29029` Action / Close → `[{"type":"CLOSE"}]`
- `107:29045` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:29046` Action / Save version → `[{"type":"NODE","target":"128:7934","navigation":"NAVIGATE"}]`

## ST-110 · Form / Export workspace data

**Role:** Linked functional state. **Frame:** 107:29047 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-29047).

**Displayed content:**

- Export workspace data
- Close
- Dataset
- Registry / Inventory / Tasks / Entries
- Export format
- CSV
- Scope
- Source dataset
- Output file
- Choose destination
- Cancel
- Export CSV

**Authored actions:**

- `107:29051` Action / Close → `[{"type":"CLOSE"}]`
- `107:29071` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:29072` Action / Export CSV → `[{"type":"NODE","target":"128:7964","navigation":"NAVIGATE"}]`

## ST-111 · Form / Salt & solvate library

**Role:** Linked functional state. **Frame:** 107:29073 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-29073).

**Displayed content:**

- Salt & solvate library
- Close
- Hydrochloride
- HCl · 36.46 g/mol
- Sodium salt
- Na · 22.99 g/mol
- Hydrate
- H₂O · 18.02 g/mol
- Use in lot
- Choose the compound lot before assigning a form
- Cancel
- Done

**Authored actions:**

- `107:29077` Action / Close → `[{"type":"CLOSE"}]`
- `107:29097` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:29098` Action / Done → `[{"type":"BACK"}]`

## ST-112 · Form / Monomer & modification library

**Role:** Linked functional state. **Frame:** 107:29099 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-29099).

**Displayed content:**

- Monomer & modification library
- Close
- mA
- 2′-O-methyl adenosine
- fU
- 2′-fluoro uridine
- sP
- Phosphorothioate linkage
- Properties
- Sugar · phosphate · molecular weight
- Cancel
- Done

**Authored actions:**

- `107:29103` Action / Close → `[{"type":"CLOSE"}]`
- `107:29123` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:29124` Action / Done → `[{"type":"BACK"}]`

## ST-113 · Form / Edit sequence

**Role:** Linked functional state. **Frame:** 107:29125 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-29125).

**Displayed content:**

- Edit sequence
- Close
- Document
- pHelixBench-Demo
- Range
- 169..423
- Operation
- Insert · Replace · Delete
- Sequence bases
- ATGCGTAAAGGCTTAGCTAG
- Preview
- Review changes to bases and annotation coordinates
- Cancel
- Apply edit

**Authored actions:**

- `107:29153` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:29154` Action / Apply edit → `[{"type":"NODE","target":"115:3326","navigation":"NAVIGATE"}]`

## ST-114 · Form / Load pipeline CSV

**Role:** Linked functional state. **Frame:** 107:29155 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-29155).

**Displayed content:**

- Load pipeline CSV
- Close
- Selected pipeline
- Assay QC import
- CSV file
- Choose file
- Required headers
- Sample · Result
- Preview
- Validate headers and review input rows
- Cancel
- Use CSV

**Authored actions:**

- `107:29179` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:29180` Action / Use CSV → `[{"type":"NODE","target":"128:7994","navigation":"NAVIGATE"}]`

## ST-115 · Form / Publish pipeline

**Role:** Reference or alternate fixture. **Frame:** 107:29181 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-29181).

**Displayed content:**

- Publish pipeline
- Close
- Pipeline
- Assay QC import
- Input
- CSV · required columns defined
- Transform
- Filter valid samples → Count
- Destination
- Notebook / Assay QC results
- Validation
- Review configuration before publishing
- Cancel
- Publish

**Authored actions:**

- `107:29209` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:29210` Action / Publish → `[{"type":"NODE","target":"107:34010","navigation":"SWAP"}]`

## ST-116 · Form / Save document

**Role:** Linked functional state. **Frame:** 107:29211 · 654 × 780. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-29211).

**Displayed content:**

- Save document
- Close
- Document
- pHelixBench-Demo
- Filename
- pHelixBench-Demo.hbx
- Location
- Choose a local folder
- Format
- HelixBench native document (.hbx)
- Cancel

**Authored actions:**

- `107:29235` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:29236` Action / Save document → `[{"type":"NODE","target":"128:8024","navigation":"NAVIGATE"}]`

## ST-117 · UX v4 / Home

**Role:** Primary module. **Frame:** 107:29237 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-29237).

**Displayed content:**

- Home
- Continue your work and resolve what needs attention.
- Search
- New
- Start with your sequence
- Open your file, understand the input, and run your first analysis.
- Import a sequence
- Open GenBank, FASTA or a HelixBench document. Review the recognized sequence before opening it.
- Import sequence
- Try a sample
- No sequence ready? A sample is labeled and kept separate from your own files.
- A short path to your first result
- Review input → inspect sequence → run analysis → save and find it again.
- Create a blank sequence
- New notebook entry
- Continue working
- Open the exact item with its context preserved.
- Work item
- Type
- Next action
- pHelixBench-Demo · DNA-001
- Sequence · v7
- Open sequence
- Restriction analysis · RES-001
- Saved result · DNA-001 v7
- View result
- Expression validation · EXP-014
- Notebook draft
- Continue entry
- Your first task
- Import your own sequence, or
explore a labeled sample. You can
skip guidance and return to it
later.
- Task help

**Authored actions:**

- `107:29344` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:29345` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `110:19573` Action / Import sequence → `[{"type":"NODE","target":"110:19645","navigation":"NAVIGATE"}]`
- `110:19575` Action / Try a sample → `[{"type":"NODE","target":"110:19881","navigation":"NAVIGATE"}]`
- `110:19582` Action / Create a blank sequence → `[{"type":"NODE","target":"110:20202","navigation":"NAVIGATE"}]`
- `110:19584` Action / New notebook entry → `[{"type":"NODE","target":"110:20232","navigation":"NAVIGATE"}]`
- `110:19594` Record / pHelixBench-Demo · DNA-001 → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`
- `110:19598` Record / Restriction analysis · RES-001 → `[{"type":"NODE","target":"110:20596","navigation":"NAVIGATE"}]`
- `110:19602` Record / Expression validation · EXP-014 → `[{"type":"NODE","target":"107:30702","navigation":"NAVIGATE"}]`
- `110:19609` Action / Import sequence → `[{"type":"NODE","target":"110:19645","navigation":"NAVIGATE"}]`
- `110:19611` Action / Try a sample → `[{"type":"NODE","target":"110:19881","navigation":"NAVIGATE"}]`
- `110:19613` Action / Task help → `[{"type":"NODE","target":"110:21252","navigation":"NAVIGATE"}]`

## ST-118 · UX v4 / Map

**Role:** Primary module. **Frame:** 107:29429 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-29429).

**Displayed content:**

- Map
- Inspect a construct and act on the selected region.
- Search
- New
- pHelixBench-Demo
- v7 · saved 10:42
- Undo
- Save version
- Export
- Sequence
- Features
- Primers
- Enzymes
- History
- Add feature
- Analyze
- Display layers
- Circular map
- Selection 97–116 · forward
- 3021 bp · circular
- GC 54.5%
- drag to select
- T7 promoter
- lac operator
- RBS
- demo_cds
- Demo-Rev
- rrnB T1 terminator
- M13 fwd site
- pUC19 MCS
- M13 rev site
- ori (demo)
- demo_marker
- loxP
- FRT
- Kozak
- EcoRI
- BamHI
- ScaI
- AhdI
- SexAI
- PshAI
- DraI
- NcoI
- T7 promoter selected. Create and analysis tools retain this document and range.
- Readable annotations
- Restriction analysis
- Task help
- Details & tools
- All map tools
- Current selection
- T7 promoter · 97–116
- How to use Map
- Select a feature on the map or use
Features for a readable list. Tools
use the range shown at right.
- Record
- DNA-001 · version 7
- Selected range
- 97–116 · forward strand
- Type
- Promoter · 20 bp
- Actions
- Edit feature
- Add primer from range
- Design primer pair
- Related records
- DNA-001 · Registry
- Plate P-014 · Inventory

**Authored actions:**

- `107:29536` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:29537` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `107:29543` Action / Save version → `[{"type":"NODE","target":"107:29211","navigation":"OVERLAY"}]`
- `107:29544` Action / Export → `[{"type":"NODE","target":"107:27142","navigation":"OVERLAY"}]`
- `107:29547` Action / Sequence → `[{"type":"NODE","target":"107:29677","navigation":"NAVIGATE"}]`
- `107:29548` Action / Features → `[{"type":"NODE","target":"107:29886","navigation":"NAVIGATE"}]`
- `107:29549` Action / Primers → `[{"type":"NODE","target":"107:30093","navigation":"NAVIGATE"}]`
- `107:29550` Action / Enzymes → `[{"type":"NODE","target":"107:30295","navigation":"NAVIGATE"}]`
- `107:29551` Action / History → `[{"type":"NODE","target":"107:30500","navigation":"NAVIGATE"}]`
- `107:29554` Action / Add feature → `[{"type":"NODE","target":"107:25134","navigation":"OVERLAY"}]`
- `107:29555` Action / Analyze → `[{"type":"NODE","target":"110:20288","navigation":"NAVIGATE"}]`
- `107:29556` Action / Display layers → `[{"type":"NODE","target":"107:25217","navigation":"OVERLAY"}]`
- `110:25121` Action / Readable annotations → `[{"type":"NODE","target":"107:29886","navigation":"NAVIGATE"}]`
- `110:25123` Action / Restriction analysis → `[{"type":"NODE","target":"110:20288","navigation":"NAVIGATE"}]`
- `110:25125` Action / Task help → `[{"type":"NODE","target":"110:21252","navigation":"NAVIGATE"}]`
- `107:29648` Action / All map tools → `[{"type":"NODE","target":"110:22016","navigation":"NAVIGATE"}]`
- `107:29662` Action / Edit feature → `[{"type":"NODE","target":"107:25134","navigation":"OVERLAY"}]`
- `107:29663` Action / Add primer from range → `[{"type":"NODE","target":"107:25182","navigation":"OVERLAY"}]`
- `107:29664` Action / Design primer pair → `[{"type":"NODE","target":"107:25303","navigation":"OVERLAY"}]`
- `107:29667` Related / DNA-001 · Registry → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`
- `107:29672` Related / Plate P-014 · Inventory → `[{"type":"NODE","target":"107:31089","navigation":"NAVIGATE"}]`

## ST-119 · UX v4 / Sequence

**Role:** Primary module. **Frame:** 107:29677 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-29677).

**Displayed content:**

- Sequence
- Inspect bases, annotations and the active selection.
- Search
- New
- pHelixBench-Demo
- v7 · saved 10:42
- Undo
- Save version
- Export
- Map
- Features
- Primers
- Enzymes
- History
- Find in sequence
- Edit selection
- Analyze
- DNA sequence · forward strand
- 1                                                 60
- ACAGTGGTTCACAGACAGATACAGACTTTCGGTTCAAGACTCTGCTGACTGC
- 61                                                 120
- GCTTATAGTTCACCTACGATCGGTGACATCGGCTAGCTAGCTAGGCTAACGT
- 121                                                 180
- TACGGTACCGGAATTCGGATCCATGTCTAAAGGTGAAGAATTATTCACTGGT
- 181                                                 240
- GTTGTCCCAATTCTTGTTGAATTAGATGGTGATGTTAATGGTCACAAATTT
- Selected 97–116 · 20 nt. Changes are validated before the saved sequence is updated.
- Annotation
- Coordinates
- Strand
- T7 promoter
- 97–116
- Forward
- lac operator
- 119–143
- demo_cds
- 180–897
- Details & tools
- All sequence tools
- Current selection
- Bases 97–116
- How to use Sequence
- Coordinates are 1-based and inclusive.
Switch views without losing your
selected region.
- Document
- DNA-001 · 3021 bp
- Selection
- 20 nt · forward strand
- Translation
- Frame +1 · standard code
- Actions
- Add feature
- BLAST selection
- Related records
- Primer pair P-01

**Authored actions:**

- `107:29784` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:29785` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `107:29791` Action / Save version → `[{"type":"NODE","target":"107:29211","navigation":"OVERLAY"}]`
- `107:29792` Action / Export → `[{"type":"NODE","target":"107:27142","navigation":"OVERLAY"}]`
- `107:29794` Action / Map → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`
- `107:29796` Action / Features → `[{"type":"NODE","target":"107:29886","navigation":"NAVIGATE"}]`
- `107:29797` Action / Primers → `[{"type":"NODE","target":"107:30093","navigation":"NAVIGATE"}]`
- `107:29798` Action / Enzymes → `[{"type":"NODE","target":"107:30295","navigation":"NAVIGATE"}]`
- `107:29799` Action / History → `[{"type":"NODE","target":"107:30500","navigation":"NAVIGATE"}]`
- `107:29802` Action / Find in sequence → `[{"type":"NODE","target":"107:33290","navigation":"OVERLAY"}]`
- `107:29803` Action / Edit selection → `[{"type":"NODE","target":"107:29125","navigation":"OVERLAY"}]`
- `107:29804` Action / Analyze → `[{"type":"NODE","target":"110:20288","navigation":"NAVIGATE"}]`
- `107:29857` Action / All sequence tools → `[{"type":"NODE","target":"110:22016","navigation":"NAVIGATE"}]`
- `107:29871` Action / Edit selection → `[{"type":"NODE","target":"107:29125","navigation":"OVERLAY"}]`
- `107:29872` Action / Add feature → `[{"type":"NODE","target":"107:25134","navigation":"OVERLAY"}]`
- `107:29873` Action / BLAST selection → `[{"type":"NODE","target":"107:25595","navigation":"OVERLAY"}]`
- `107:29876` Related / T7 promoter → `[{"type":"NODE","target":"107:29886","navigation":"NAVIGATE"}]`
- `107:29881` Related / Primer pair P-01 → `[{"type":"NODE","target":"107:30093","navigation":"NAVIGATE"}]`

## ST-120 · UX v4 / Features

**Role:** Primary module. **Frame:** 107:29886 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-29886).

**Displayed content:**

- Features
- Review annotations before applying changes.
- Search
- New
- pHelixBench-Demo
- v7 · saved 10:42
- Undo
- Save version
- Export
- Map
- Sequence
- Primers
- Enzymes
- History
- Add feature
- Detect common
- Review auto-annotation
- Name
- Type
- Range
- Strand
- T7 promoter
- Promoter
- 97–116
- Forward
- lac operator
- Operator
- 119–143
- demo_cds
- CDS
- 180–897
- rrnB T1 terminator
- Terminator
- 940–1025
- AmpR
- 1700–2558
- Reverse
- Review proposed additions and conflicts before changing the annotation set.
- Details & tools
- All features tools
- Current selection
- How to use Features
- Features describe annotated regions.
Add your own annotation or review
suggested annotations before applying.
- 97–116 · forward
- Source
- Manual annotation
- Actions
- Edit selected
- Auto-annotation preview
- Related records
- Expression validation

**Authored actions:**

- `107:29993` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:29994` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `107:30000` Action / Save version → `[{"type":"NODE","target":"107:29211","navigation":"OVERLAY"}]`
- `107:30001` Action / Export → `[{"type":"NODE","target":"107:27142","navigation":"OVERLAY"}]`
- `107:30003` Action / Map → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`
- `107:30004` Action / Sequence → `[{"type":"NODE","target":"107:29677","navigation":"NAVIGATE"}]`
- `107:30006` Action / Primers → `[{"type":"NODE","target":"107:30093","navigation":"NAVIGATE"}]`
- `107:30007` Action / Enzymes → `[{"type":"NODE","target":"107:30295","navigation":"NAVIGATE"}]`
- `107:30008` Action / History → `[{"type":"NODE","target":"107:30500","navigation":"NAVIGATE"}]`
- `107:30011` Action / Add feature → `[{"type":"NODE","target":"107:25134","navigation":"OVERLAY"}]`
- `107:30012` Action / Detect common → `[{"type":"NODE","target":"107:26060","navigation":"OVERLAY"}]`
- `107:30013` Action / Review auto-annotation → `[{"type":"NODE","target":"107:33313","navigation":"OVERLAY"}]`
- `107:30020` Record / T7 promoter → `[{"type":"NODE","target":"107:25134","navigation":"OVERLAY"}]`
- `107:30025` Record / lac operator → `[{"type":"NODE","target":"107:25134","navigation":"OVERLAY"}]`
- `107:30030` Record / demo_cds → `[{"type":"NODE","target":"107:25134","navigation":"OVERLAY"}]`
- `107:30035` Record / rrnB T1 terminator → `[{"type":"NODE","target":"107:25134","navigation":"OVERLAY"}]`
- `107:30040` Record / AmpR → `[{"type":"NODE","target":"107:25134","navigation":"OVERLAY"}]`
- `107:30064` Action / All features tools → `[{"type":"NODE","target":"107:28097","navigation":"OVERLAY"}]`
- `107:30078` Action / Edit selected → `[{"type":"NODE","target":"107:25134","navigation":"OVERLAY"}]`
- `107:30079` Action / Detect common → `[{"type":"NODE","target":"107:26060","navigation":"OVERLAY"}]`
- `107:30080` Action / Auto-annotation preview → `[{"type":"NODE","target":"107:33313","navigation":"OVERLAY"}]`
- `107:30083` Related / pHelixBench-Demo → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`
- `107:30088` Related / Expression validation → `[{"type":"NODE","target":"107:30702","navigation":"NAVIGATE"}]`

## ST-121 · UX v4 / Primers

**Role:** Primary module. **Frame:** 107:30093 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-30093).

**Displayed content:**

- Primers
- Review candidate pairs and verify the predicted product.
- Search
- New
- pHelixBench-Demo
- v7 · saved 10:42
- Undo
- Save version
- Export
- Map
- Sequence
- Features
- Enzymes
- History
- Design pair
- Add primer
- Simulate PCR
- Primer / pair
- Position
- Tm
- State
- P-01 · T7-F / Demo-R
- 97–351
- 61.2 / 62.0 °C
- Linked pair
- T7-F
- 97–116
- 61.2 °C
- Forward
- Demo-R
- 330–351
- 62.0 °C
- Reverse
- P-02 · screening
- 180–598
- 60.8 / 61.4 °C
- Review needed
- P-01 is linked to DNA-001. Predicted product: 255 bp; inspect simulation before saving.
- Details & tools
- All primers tools
- Current selection
- Pair P-01
- How to use Primers
- A primer is linked to an input
sequence. Verify range, strand and
pair before simulating PCR.
- T7-F · 20 nt
- Demo-R · 22 nt
- Predicted product
- 255 bp · ready to simulate
- Actions
- Edit primer
- Related records
- DNA-001 template
- PCR product history

**Authored actions:**

- `107:30200` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:30201` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `107:30207` Action / Save version → `[{"type":"NODE","target":"107:29211","navigation":"OVERLAY"}]`
- `107:30208` Action / Export → `[{"type":"NODE","target":"107:27142","navigation":"OVERLAY"}]`
- `107:30210` Action / Map → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`
- `107:30211` Action / Sequence → `[{"type":"NODE","target":"107:29677","navigation":"NAVIGATE"}]`
- `107:30212` Action / Features → `[{"type":"NODE","target":"107:29886","navigation":"NAVIGATE"}]`
- `107:30214` Action / Enzymes → `[{"type":"NODE","target":"107:30295","navigation":"NAVIGATE"}]`
- `107:30215` Action / History → `[{"type":"NODE","target":"107:30500","navigation":"NAVIGATE"}]`
- `107:30218` Action / Design pair → `[{"type":"NODE","target":"107:25303","navigation":"OVERLAY"}]`
- `107:30219` Action / Add primer → `[{"type":"NODE","target":"107:25182","navigation":"OVERLAY"}]`
- `107:30220` Action / Simulate PCR → `[{"type":"NODE","target":"107:25266","navigation":"OVERLAY"}]`
- `107:30227` Record / P-01 · T7-F / Demo-R → `[{"type":"NODE","target":"107:25182","navigation":"OVERLAY"}]`
- `107:30232` Record / T7-F → `[{"type":"NODE","target":"107:25182","navigation":"OVERLAY"}]`
- `107:30237` Record / Demo-R → `[{"type":"NODE","target":"107:25182","navigation":"OVERLAY"}]`
- `107:30242` Record / P-02 · screening → `[{"type":"NODE","target":"107:25182","navigation":"OVERLAY"}]`
- `107:30266` Action / All primers tools → `[{"type":"NODE","target":"107:28166","navigation":"OVERLAY"}]`
- `107:30280` Action / Design pair → `[{"type":"NODE","target":"107:25303","navigation":"OVERLAY"}]`
- `107:30281` Action / Edit primer → `[{"type":"NODE","target":"107:25182","navigation":"OVERLAY"}]`
- `107:30282` Action / Simulate PCR → `[{"type":"NODE","target":"107:25266","navigation":"OVERLAY"}]`
- `107:30285` Related / DNA-001 template → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`
- `107:30290` Related / PCR product history → `[{"type":"NODE","target":"107:33352","navigation":"OVERLAY"}]`

## ST-122 · UX v4 / Enzymes

**Role:** Primary module. **Frame:** 107:30295 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-30295).

**Displayed content:**

- Enzymes
- Choose cutters and inspect predicted fragments.
- Search
- New
- pHelixBench-Demo
- v7 · saved 10:42
- Undo
- Save version
- Export
- Map
- Sequence
- Features
- Primers
- History
- Enzyme set
- Filter cutters
- Digest & gel
- Enzyme
- Sites
- Cut position
- Selection
- EcoRI
- 1
- 216
- Selected
- BamHI
- 234
- HindIII
- 260
- Available
- NotI
- 0
- No sites
- Filtered out
- Digest setup
- Two selected enzymes · circular DNA · input version 7
- Expected fragments and gel preview are generated after parameter validation.
- Details & tools
- All enzymes tools
- Current selection
- EcoRI + BamHI
- How to use Enzymes
- Restriction analysis predicts cut
sites and fragments. Confirm your
input and enzyme set before running.
- Active set
- Common cloning enzymes
- Cut sites
- 2 selected · 4 total
- Document
- DNA-001 · circular
- Actions
- Choose enzyme set
- Related records
- Input DNA-001
- Digest result

**Authored actions:**

- `107:30402` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:30403` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `107:30409` Action / Save version → `[{"type":"NODE","target":"107:29211","navigation":"OVERLAY"}]`
- `107:30410` Action / Export → `[{"type":"NODE","target":"107:27142","navigation":"OVERLAY"}]`
- `107:30412` Action / Map → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`
- `107:30413` Action / Sequence → `[{"type":"NODE","target":"107:29677","navigation":"NAVIGATE"}]`
- `107:30414` Action / Features → `[{"type":"NODE","target":"107:29886","navigation":"NAVIGATE"}]`
- `107:30415` Action / Primers → `[{"type":"NODE","target":"107:30093","navigation":"NAVIGATE"}]`
- `107:30417` Action / History → `[{"type":"NODE","target":"107:30500","navigation":"NAVIGATE"}]`
- `107:30420` Action / Enzyme set → `[{"type":"NODE","target":"107:25217","navigation":"OVERLAY"}]`
- `107:30421` Action / Filter cutters → `[{"type":"NODE","target":"107:26978","navigation":"OVERLAY"}]`
- `107:30422` Action / Digest & gel → `[{"type":"NODE","target":"110:20288","navigation":"NAVIGATE"}]`
- `107:30429` Record / EcoRI → `[{"type":"NODE","target":"110:20288","navigation":"NAVIGATE"}]`
- `107:30434` Record / BamHI → `[{"type":"NODE","target":"110:20288","navigation":"NAVIGATE"}]`
- `107:30439` Record / HindIII → `[{"type":"NODE","target":"110:20288","navigation":"NAVIGATE"}]`
- `107:30444` Record / NotI → `[{"type":"NODE","target":"110:20288","navigation":"NAVIGATE"}]`
- `107:30471` Action / All enzymes tools → `[{"type":"NODE","target":"107:28231","navigation":"OVERLAY"}]`
- `107:30485` Action / Choose enzyme set → `[{"type":"NODE","target":"107:25217","navigation":"OVERLAY"}]`
- `107:30486` Action / Digest & gel → `[{"type":"NODE","target":"110:20288","navigation":"NAVIGATE"}]`
- `107:30487` Action / Filter cutters → `[{"type":"NODE","target":"107:26978","navigation":"OVERLAY"}]`
- `107:30490` Related / Input DNA-001 → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`
- `107:30495` Related / Digest result → `[{"type":"NODE","target":"107:33375","navigation":"OVERLAY"}]`

## ST-123 · UX v4 / History

**Role:** Primary module. **Frame:** 107:30500 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-30500).

**Displayed content:**

- History
- Compare changes before restoring a saved version.
- Search
- New
- pHelixBench-Demo
- v7 · saved 10:42
- Undo
- Save version
- Export
- Map
- Sequence
- Features
- Primers
- Enzymes
- Compare versions
- Duplicate version
- Restore preview
- Version
- Changed by
- Summary
- When
- v7 · current
- You
- T7 promoter annotation
- Today 10:42
- v6
- Reverse primer added
- Today 10:12
- v5
- M. Chen
- CDS coordinates updated
- Yesterday
- v4
- Imported GenBank file
- 12 Nov
- Previewing a version does not replace the current record. Restore includes an impact review.
- Details & tools
- All history tools
- Current selection
- Version 7 · current
- How to use History
- Versions are saved checkpoints. Undo
reverses an unsaved edit; restoring a
version requires an impact review.
- Saved by
- You · today at 10:42
- Changes
- Feature annotation updated
- Record
- DNA-001
- Actions
- Related records
- Current sequence
- Record activity

**Authored actions:**

- `107:30607` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:30608` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `107:30614` Action / Save version → `[{"type":"NODE","target":"107:29211","navigation":"OVERLAY"}]`
- `107:30615` Action / Export → `[{"type":"NODE","target":"107:27142","navigation":"OVERLAY"}]`
- `107:30617` Action / Map → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`
- `107:30618` Action / Sequence → `[{"type":"NODE","target":"107:29677","navigation":"NAVIGATE"}]`
- `107:30619` Action / Features → `[{"type":"NODE","target":"107:29886","navigation":"NAVIGATE"}]`
- `107:30620` Action / Primers → `[{"type":"NODE","target":"107:30093","navigation":"NAVIGATE"}]`
- `107:30621` Action / Enzymes → `[{"type":"NODE","target":"107:30295","navigation":"NAVIGATE"}]`
- `107:30625` Action / Compare versions → `[{"type":"NODE","target":"107:33405","navigation":"OVERLAY"}]`
- `107:30626` Action / Duplicate version → `[{"type":"NODE","target":"107:29025","navigation":"OVERLAY"}]`
- `107:30627` Action / Restore preview → `[{"type":"NODE","target":"107:33433","navigation":"OVERLAY"}]`
- `107:30634` Record / v7 · current → `[{"type":"NODE","target":"107:33405","navigation":"OVERLAY"}]`
- `107:30639` Record / v6 → `[{"type":"NODE","target":"107:33405","navigation":"OVERLAY"}]`
- `107:30644` Record / v5 → `[{"type":"NODE","target":"107:33405","navigation":"OVERLAY"}]`
- `107:30649` Record / v4 → `[{"type":"NODE","target":"107:33405","navigation":"OVERLAY"}]`
- `107:30673` Action / All history tools → `[{"type":"NODE","target":"107:28296","navigation":"OVERLAY"}]`
- `107:30687` Action / Compare versions → `[{"type":"NODE","target":"107:33405","navigation":"OVERLAY"}]`
- `107:30688` Action / Duplicate version → `[{"type":"NODE","target":"107:29025","navigation":"OVERLAY"}]`
- `107:30689` Action / Restore preview → `[{"type":"NODE","target":"107:33433","navigation":"OVERLAY"}]`
- `107:30692` Related / Current sequence → `[{"type":"NODE","target":"107:29677","navigation":"NAVIGATE"}]`
- `107:30697` Related / Record activity → `[{"type":"NODE","target":"107:33461","navigation":"OVERLAY"}]`

## ST-124 · UX v4 / Notebook

**Role:** Primary module. **Frame:** 107:30702 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-30702).

**Displayed content:**

- Notebook
- Capture structured data and submit reusable results.
- Search
- New
- Insert result table
- Submit 2 rows
- Send for review
- Expression cassette validation
- Draft
- EXP-014 · Expression study · text saved at 10:36
- Notes
- Results
- Review
- Activity
- Purpose
- Verify the expression construct and record assay results.
- DNA-001 · construct
- P-014 · snapshot
- Assay results
- Sample
- Signal (a.u.)
- Result state
- S-001
- 1280
- Unsubmitted
- S-002
- 1315
- Submitting this table creates reusable results linked to this entry.
- Text saved. Two result rows are still unsubmitted; entry review has not started.
- Details & tools
- All notebook tools
- Current selection
- Expression validation
- How to use Notebook
- Write notes, link your input, then
submit result rows when ready. Saving
text does not submit results.
- Entry
- EXP-014 · draft
- Save state
- Text saved · 2 unsubmitted rows
- Not submitted for review
- Actions
- Related records
- DNA-001 · Entity
- P-014 · Plate snapshot

**Authored actions:**

- `107:30809` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:30810` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `107:30813` Action / Insert result table → `[{"type":"NODE","target":"107:33489","navigation":"OVERLAY"}]`
- `107:30814` Action / Submit 2 rows → `[{"type":"NODE","target":"107:33516","navigation":"OVERLAY"}]`
- `107:30815` Action / Send for review → `[{"type":"NODE","target":"107:33554","navigation":"OVERLAY"}]`
- `107:30825` Action / Results → `[{"type":"NODE","target":"107:33516","navigation":"OVERLAY"}]`
- `107:30826` Action / Review → `[{"type":"NODE","target":"107:33554","navigation":"OVERLAY"}]`
- `107:30827` Action / Activity → `[{"type":"NODE","target":"107:33461","navigation":"OVERLAY"}]`
- `107:30832` Action / DNA-001 · construct → `[{"type":"NODE","target":"107:33242","navigation":"OVERLAY"}]`
- `107:30833` Action / P-014 · snapshot → `[{"type":"NODE","target":"107:33579","navigation":"OVERLAY"}]`
- `107:30840` Record / S-001 → `[{"type":"NODE","target":"107:33516","navigation":"OVERLAY"}]`
- `107:30844` Record / S-002 → `[{"type":"NODE","target":"107:33516","navigation":"OVERLAY"}]`
- `107:30868` Action / All notebook tools → `[{"type":"NODE","target":"107:28334","navigation":"OVERLAY"}]`
- `107:30882` Action / Insert result table → `[{"type":"NODE","target":"107:33489","navigation":"OVERLAY"}]`
- `107:30883` Action / Submit 2 rows → `[{"type":"NODE","target":"107:33516","navigation":"OVERLAY"}]`
- `107:30884` Action / Send for review → `[{"type":"NODE","target":"107:33554","navigation":"OVERLAY"}]`
- `107:30887` Related / DNA-001 · Entity → `[{"type":"NODE","target":"107:33242","navigation":"OVERLAY"}]`
- `107:30892` Related / P-014 · Plate snapshot → `[{"type":"NODE","target":"107:33579","navigation":"OVERLAY"}]`

## ST-125 · UX v4 / Registry

**Role:** Primary module. **Frame:** 107:30897 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-30897).

**Displayed content:**

- Registry
- Identify entities and follow their connected records.
- Search
- New
- Register entity
- Import spreadsheet
- Find samples
- Entities
- Saved views
- Relationships
- Name / ID
- Schema
- Samples
- Updated
- pHelixBench-Demo · DNA-001
- DNA construct
- 3 containers
- Today
- T7-F · PR-001
- Primer
- 1 container
- AB-014
- Protein
- 2 containers
- Yesterday
- OL-014
- RNA oligo
- Registry describes scientific identity. Quantities and storage positions belong to Inventory.
- Archive DNA-001
- Delete DNA-001
- Archived records
- Details & tools
- All registry tools
- Current selection
- DNA-001 · pHelixBench-Demo
- How to use Registry
- Registry describes scientific
identity. Inventory tracks the
physical material, quantity and
location.
- Scientific identity
- 3021 bp · circular
- Physical samples
- 3 accessible containers
- Actions
- Edit entity
- Import entities
- Related records
- EXP-014 · Notebook
- P-014 · Inventory

**Authored actions:**

- `107:31004` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:31005` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `107:31008` Action / Register entity → `[{"type":"NODE","target":"107:25875","navigation":"OVERLAY"}]`
- `107:31009` Action / Import spreadsheet → `[{"type":"NODE","target":"107:33652","navigation":"OVERLAY"}]`
- `107:31010` Action / Find samples → `[{"type":"NODE","target":"107:33618","navigation":"OVERLAY"}]`
- `107:31013` Action / Saved views → `[{"type":"NODE","target":"107:26948","navigation":"OVERLAY"}]`
- `107:31014` Action / Relationships → `[{"type":"NODE","target":"107:33679","navigation":"OVERLAY"}]`
- `107:31021` Record / pHelixBench-Demo · DNA-001 → `[{"type":"NODE","target":"110:21504","navigation":"NAVIGATE"}]`
- `107:31026` Record / T7-F · PR-001 → `[{"type":"NODE","target":"110:21536","navigation":"NAVIGATE"}]`
- `107:31031` Record / AB-014 → `[{"type":"NODE","target":"110:21568","navigation":"NAVIGATE"}]`
- `107:31036` Record / OL-014 → `[{"type":"NODE","target":"110:21600","navigation":"NAVIGATE"}]`
- `110:21633` Action / Archive DNA-001 → `[{"type":"NODE","target":"125:5170","navigation":"NAVIGATE"}]`
- `110:21635` Action / Delete DNA-001 → `[{"type":"NODE","target":"110:21685","navigation":"NAVIGATE"}]`
- `125:5638` Action / Archived records → `[{"type":"NODE","target":"125:5450","navigation":"NAVIGATE"}]`
- `107:31060` Action / All registry tools → `[{"type":"NODE","target":"107:28399","navigation":"OVERLAY"}]`
- `107:31074` Action / Edit entity → `[{"type":"NODE","target":"107:25875","navigation":"OVERLAY"}]`
- `107:31075` Action / Find samples → `[{"type":"NODE","target":"107:33618","navigation":"OVERLAY"}]`
- `107:31076` Action / Import entities → `[{"type":"NODE","target":"107:33652","navigation":"OVERLAY"}]`
- `107:31079` Related / EXP-014 · Notebook → `[{"type":"NODE","target":"107:30702","navigation":"NAVIGATE"}]`
- `107:31084` Related / P-014 · Inventory → `[{"type":"NODE","target":"107:31089","navigation":"NAVIGATE"}]`

## ST-126 · UX v4 / Inventory

**Role:** Primary module. **Frame:** 107:31089 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-31089).

**Displayed content:**

- Inventory
- Locate physical samples and review changes before transfer.
- Search
- New
- Add container
- Transfer selected
- Scan / paste barcode
- Storage
- Freezer A
-   Shelf 2
-     Rack R-01
- P-014 · live plate
- 96 wells · 4 occupied
- Find entity samples
- Plate P-014
- Live inventory
- A1–A4 selected · 4 wells · Freezer A / Shelf 2
- 
- 1
- 2
- 3
- 4
- 5
- 6
- 7
- 8
- 9
- 10
- 11
- 12
- A
- S1
- S2
- S3
- S4
- —
- B
- C
- D
- E
- F
- G
- H
- Assign contents
- Set role / concentration
- Live changes do not modify the plate snapshot already attached to EXP-014.
- Details & tools
- All inventory tools
- Current selection
- P-014 · 4 wells selected
- How to use Inventory
- Select a container, then its wells.
Assignment and transfer retain the
selected sample IDs and locations.
- Record type
- Live plate · 96 wells
- Location
- Freezer A / Shelf 2
- Selection
- A1–A4 · 4 wells
- Actions
- Open experiment snapshot
- Related records
- DNA-001 · Entity
- EXP-014 · Notebook

**Authored actions:**

- `107:31196` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:31197` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `107:31200` Action / Add container → `[{"type":"NODE","target":"107:27008","navigation":"OVERLAY"}]`
- `107:31201` Action / Transfer selected → `[{"type":"NODE","target":"128:5866","navigation":"NAVIGATE"}]`
- `107:31202` Action / Scan / paste barcode → `[{"type":"NODE","target":"107:33742","navigation":"OVERLAY"}]`
- `107:31213` Action / Find entity samples → `[{"type":"NODE","target":"107:33618","navigation":"OVERLAY"}]`
- `107:31238` Well / A1 → `[{"type":"SET_VARIABLE","variable":"VariableID:128:5590","value":"P-014 / A1"},{"type":"SET_VARIABLE","variable":"VariableID:128:5591","value":"S-001 · DNA-001"},{"type":"SET_VARIABLE","variable":"VariableID:128:5592","value":"50 µL · 100 ng/µL"},{"type":"SET_VARIABLE","variable":"VariableID:128:5593","value":"Occupied"},{"type":"NODE","target":"128:5594","navigation":"NAVIGATE"}]`
- `107:31442` Well / H12 → `[{"type":"SET_VARIABLE","variable":"VariableID:128:5590","value":"P-014 / H12"},{"type":"SET_VARIABLE","variable":"VariableID:128:5591","value":"Empty well"},{"type":"SET_VARIABLE","variable":"VariableID:128:5592","value":"No quantity assigned"},{"type":"SET_VARIABLE","variable":"VariableID:128:5593","value":"Empty"},{"type":"NODE","target":"128:5594","navigation":"NAVIGATE"}]`
- `107:31445` Action / Assign contents → `[{"type":"NODE","target":"128:5816","navigation":"NAVIGATE"}]`
- `107:31446` Action / Set role / concentration → `[{"type":"NODE","target":"128:5816","navigation":"NAVIGATE"}]`
- `107:31466` Action / All inventory tools → `[{"type":"NODE","target":"107:28459","navigation":"OVERLAY"}]`
- `107:31480` Action / Assign contents → `[{"type":"NODE","target":"128:5816","navigation":"NAVIGATE"}]`
- `107:31481` Action / Transfer selected → `[{"type":"NODE","target":"128:5866","navigation":"NAVIGATE"}]`
- `107:31482` Action / Open experiment snapshot → `[{"type":"NODE","target":"107:33579","navigation":"OVERLAY"}]`
- `107:31485` Related / DNA-001 · Entity → `[{"type":"NODE","target":"107:33242","navigation":"OVERLAY"}]`
- `107:31490` Related / EXP-014 · Notebook → `[{"type":"NODE","target":"107:30702","navigation":"NAVIGATE"}]`

Well controls: 96. A1/H12 are representative authored action examples; implementation must generate identity-bound controls for every actual well.

## ST-127 · UX v4 / Workflows

**Role:** Primary module. **Frame:** 107:31495 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-31495).

**Displayed content:**

- Workflows
- Execute assigned work and repair blocked handoffs.
- Search
- New
- New task
- New request
- Fix failed mapping
- My tasks
- Requests
- Templates
- 1 task is blocked by a missing downstream sample mapping. Open the affected field to repair it.
- Task
- Assignee
- Status
- Due
- WF-042 · Sequence validation
- You
- Failed mapping
- Today
- WF-043 · PCR setup
- Ready
- WF-044 · Assay review
- M. Chen
- In progress
- Tomorrow
- Details & tools
- All workflows tools
- Current selection
- WF-042 · sequence validation
- How to use Workflows
- Tasks coordinate work. Open a task to
see its required inputs, blocker and
next action.
- You · due today
- Blocked · failed mapping
- Blocker
- Required sample link is missing
- Actions
- Open request
- Execute next step
- Related records
- DNA-001 · Input
- EXP-014 · Execution entry

**Authored actions:**

- `107:31602` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:31603` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `107:31606` Action / New task → `[{"type":"NODE","target":"107:27034","navigation":"OVERLAY"}]`
- `107:31607` Action / New request → `[{"type":"NODE","target":"107:33859","navigation":"OVERLAY"}]`
- `107:31608` Action / Fix failed mapping → `[{"type":"NODE","target":"107:33765","navigation":"OVERLAY"}]`
- `107:31611` Action / Requests → `[{"type":"NODE","target":"107:33797","navigation":"OVERLAY"}]`
- `107:31612` Action / Templates → `[{"type":"NODE","target":"107:26410","navigation":"OVERLAY"}]`
- `107:31624` Record / WF-042 · Sequence validation → `[{"type":"NODE","target":"107:33832","navigation":"OVERLAY"}]`
- `107:31629` Record / WF-043 · PCR setup → `[{"type":"NODE","target":"107:33832","navigation":"OVERLAY"}]`
- `107:31634` Record / WF-044 · Assay review → `[{"type":"NODE","target":"107:33832","navigation":"OVERLAY"}]`
- `107:31653` Action / All workflows tools → `[{"type":"NODE","target":"107:28515","navigation":"OVERLAY"}]`
- `107:31667` Action / Fix failed mapping → `[{"type":"NODE","target":"107:33765","navigation":"OVERLAY"}]`
- `107:31668` Action / Open request → `[{"type":"NODE","target":"107:33797","navigation":"OVERLAY"}]`
- `107:31669` Action / Execute next step → `[{"type":"NODE","target":"107:33832","navigation":"OVERLAY"}]`
- `107:31672` Related / DNA-001 · Input → `[{"type":"NODE","target":"107:33242","navigation":"OVERLAY"}]`
- `107:31677` Related / EXP-014 · Execution entry → `[{"type":"NODE","target":"107:30702","navigation":"NAVIGATE"}]`

## ST-128 · UX v4 / Insights

**Role:** Primary module. **Frame:** 107:31682 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-31682).

**Displayed content:**

- Insights
- Inspect results with their data scope and filter context.
- Search
- New
- Configure dashboard
- View source records
- Open investigation
- Expression QC
- Parameters
- Query details
- Project: Expression study · data scoped to your access · refreshed today at 10:38.
- Expression QC over time
- Mean signal (a.u.) by elapsed time (min)
- 100
- 50
- 0
- Time (min)     0                        30                        60                        90                       120
- Sample
- Mean signal (a.u.)
- Record
- S-001
- 1280
- DNA-001
- S-002
- 1315
- Details & tools
- All insights tools
- Current selection
- Expression QC dashboard
- How to use Insights
- Choose a data scope before
interpreting results. Query parameters
and source records stay visible.
- Data scope
- My accessible project records
- Project: Expression study
- Last refreshed
- Today · 10:38
- Actions
- Edit query
- Related records
- Expression validation
- Run R-018

**Authored actions:**

- `107:31789` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:31790` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `107:31793` Action / Configure dashboard → `[{"type":"NODE","target":"107:26918","navigation":"OVERLAY"}]`
- `107:31794` Action / View source records → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`
- `107:31795` Action / Open investigation → `[{"type":"NODE","target":"107:33890","navigation":"OVERLAY"}]`
- `107:31798` Action / Parameters → `[{"type":"NODE","target":"107:33922","navigation":"OVERLAY"}]`
- `107:31799` Action / Query details → `[{"type":"NODE","target":"107:26918","navigation":"OVERLAY"}]`
- `107:31825` Record / S-001 → `[{"type":"NODE","target":"107:33242","navigation":"OVERLAY"}]`
- `107:31829` Record / S-002 → `[{"type":"NODE","target":"107:33242","navigation":"OVERLAY"}]`
- `107:31847` Action / All insights tools → `[{"type":"NODE","target":"107:28571","navigation":"OVERLAY"}]`
- `107:31861` Action / Edit query → `[{"type":"NODE","target":"107:26918","navigation":"OVERLAY"}]`
- `107:31862` Action / View source records → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`
- `107:31863` Action / Open investigation → `[{"type":"NODE","target":"107:33890","navigation":"OVERLAY"}]`
- `107:31866` Related / Expression validation → `[{"type":"NODE","target":"107:30702","navigation":"NAVIGATE"}]`
- `107:31871` Related / Run R-018 → `[{"type":"NODE","target":"107:31876","navigation":"NAVIGATE"}]`

## ST-129 · UX v4 / Automation

**Role:** Primary module. **Frame:** 107:31876 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-31876).

**Displayed content:**

- Automation
- Validate inputs and inspect outputs before writeback.
- Search
- New
- Load input
- Test pipeline
- Publish
- Design
- Run history
- Connections
- Expression QC
- Input file → validated dataset → reviewed output
- CSV input
- assay_results.csv
- Inspect step
- Validate columns
- Sample ID · signal
- Configure selected
- Create results
- Expression results
- Draft is editable. Test validates inputs and previews outputs before publication or writeback.
- Recent run
- State
- Template reference
- Output
- R-018
- Validation failed
- TPL-014 / draft
- No records written
- R-017
- Completed
- TPL-012 / published
- 96 results
- Details & tools
- All automation tools
- Current selection
- Expression QC · draft
- How to use Automation
- Review input mapping, test the
pipeline, then review the run before
writeback. Publishing is a separate
action.
- Input
- assay_results.csv · 96 rows
- Destination
- Template state
- Draft · not published
- Actions
- Run published template
- Publish template
- Related records
- Run history R-018
- EXP-014 · Notebook

**Authored actions:**

- `107:31983` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:31984` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `107:31987` Action / Load input → `[{"type":"NODE","target":"107:29155","navigation":"OVERLAY"}]`
- `107:31988` Action / Test pipeline → `[{"type":"NODE","target":"107:33950","navigation":"OVERLAY"}]`
- `107:31989` Action / Publish → `[{"type":"NODE","target":"107:34010","navigation":"OVERLAY"}]`
- `107:31992` Action / Run history → `[{"type":"NODE","target":"107:34042","navigation":"OVERLAY"}]`
- `107:31993` Action / Connections → `[{"type":"NODE","target":"107:34076","navigation":"OVERLAY"}]`
- `107:32002` Action / Inspect step → `[{"type":"NODE","target":"107:33950","navigation":"OVERLAY"}]`
- `107:32006` Action / Configure selected → `[{"type":"NODE","target":"107:34103","navigation":"OVERLAY"}]`
- `107:32010` Action / Inspect step → `[{"type":"NODE","target":"107:33950","navigation":"OVERLAY"}]`
- `107:32022` Record / R-018 → `[{"type":"NODE","target":"107:34042","navigation":"OVERLAY"}]`
- `107:32027` Record / R-017 → `[{"type":"NODE","target":"107:34042","navigation":"OVERLAY"}]`
- `107:32046` Action / All automation tools → `[{"type":"NODE","target":"107:28604","navigation":"OVERLAY"}]`
- `107:32060` Action / Test pipeline → `[{"type":"NODE","target":"107:33950","navigation":"OVERLAY"}]`
- `107:32061` Action / Run input review → `[{"type":"NODE","target":"107:33978","navigation":"OVERLAY"}]`
- `107:32062` Action / Publish template → `[{"type":"NODE","target":"107:34010","navigation":"OVERLAY"}]`
- `107:32065` Related / Run history R-018 → `[{"type":"NODE","target":"107:34042","navigation":"OVERLAY"}]`
- `107:32070` Related / EXP-014 · Notebook → `[{"type":"NODE","target":"107:30702","navigation":"NAVIGATE"}]`

## ST-130 · UX v4 / Split

**Role:** Primary module. **Frame:** 107:32075 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-32075).

**Displayed content:**

- Compare sequences
- Compare two named input records before alignment.
- Search
- New
- Choose right input
- Align pair
- View result
- DNA-001 · v7
- DNA sequence · forward strand
- 1                                                 60
- ACAGTGGTTCACAGACAGATACAGACTTTCGGTTCAAGACTCTGCTGACTGC
- 61                                                 120
- GCTTATAGTTCACCTACGATCGGTGACATCGGCTAGCTAGCTAGGCTAACGT
- 121                                                 180
- TACGGTACCGGAATTCGGATCCATGTCTAAAGGTGAAGAATTATTCACTGGT
- 181                                                 240
- GTTGTCCCAATTCTTGTTGAATTAGATGGTGATGTTAATGGTCACAAATTT
- DNA-002 · v3
- Left pane active · independent scrolling. Alignment runs only after you review the selected pair.
- Details & tools
- All split tools
- Current selection
- Left pane · DNA-001
- How to use Split
- Tools apply to the active pane.
Alignment requires both named input
records.
- Left input
- DNA-001 · version 7
- Right input
- DNA-002 · version 3
- Navigation
- Independent scroll
- Actions
- Align selected pair
- Review alignment
- Related records
- DNA-001 · Registry
- DNA-002 · Registry

**Authored actions:**

- `107:32182` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:32183` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `107:32186` Action / Choose right input → `[{"type":"NODE","target":"110:19645","navigation":"NAVIGATE"}]`
- `107:32187` Action / Align pair → `[{"type":"NODE","target":"107:25425","navigation":"OVERLAY"}]`
- `107:32188` Action / View result → `[{"type":"NODE","target":"107:34130","navigation":"OVERLAY"}]`
- `107:32241` Action / All split tools → `[{"type":"NODE","target":"107:28655","navigation":"OVERLAY"}]`
- `107:32255` Action / Align selected pair → `[{"type":"NODE","target":"107:25425","navigation":"OVERLAY"}]`
- `107:32256` Action / Choose right input → `[{"type":"NODE","target":"110:19645","navigation":"NAVIGATE"}]`
- `107:32257` Action / Review alignment → `[{"type":"NODE","target":"107:34130","navigation":"OVERLAY"}]`
- `107:32260` Related / DNA-001 · Registry → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`
- `107:32265` Related / DNA-002 · Registry → `[{"type":"NODE","target":"107:33242","navigation":"OVERLAY"}]`

## ST-131 · UX v4 / Biologics

**Role:** Primary module. **Frame:** 107:32270 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-32270).

**Displayed content:**

- Biologics
- Inspect protein format, chains and component identity.
- Search
- New
- Register protein
- Import chains
- Analyze chain
- AB-014 · IgG
- Format-aware protein with linked chain identities.
- Heavy chain
- HC-014 · 450 aa
- Light chain
- LC-014 · 214 aa
- Protein format
- IgG · 2 chain types
- Component
- Role
- Sequence
- Identity
- HC-014
- 450 aa
- Linked entity
- LC-014
- 214 aa
- Details & tools
- All biologics tools
- Current selection
- How to use Biologics
- A biologic can contain multiple
chains. Choose the chain you want to
analyze before opening its tools.
- Format
- Actions
- Register biologic
- Import chain data
- Related records
- HC-014 · Entity
- Characterization entry

**Authored actions:**

- `107:32378` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:32379` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `107:32382` Action / Register protein → `[{"type":"NODE","target":"107:26448","navigation":"OVERLAY"}]`
- `107:32383` Action / Import chains → `[{"type":"NODE","target":"107:33652","navigation":"OVERLAY"}]`
- `107:32384` Action / Analyze chain → `[{"type":"NODE","target":"107:26168","navigation":"OVERLAY"}]`
- `107:32389` Component / Heavy chain → `[{"type":"NODE","target":"107:33242","navigation":"OVERLAY"}]`
- `107:32392` Component / Light chain → `[{"type":"NODE","target":"107:33242","navigation":"OVERLAY"}]`
- `107:32395` Component / Protein format → `[{"type":"NODE","target":"107:33242","navigation":"OVERLAY"}]`
- `107:32404` Record / HC-014 → `[{"type":"NODE","target":"107:33242","navigation":"OVERLAY"}]`
- `107:32409` Record / LC-014 → `[{"type":"NODE","target":"107:33242","navigation":"OVERLAY"}]`
- `107:32428` Action / All biologics tools → `[{"type":"NODE","target":"107:28688","navigation":"OVERLAY"}]`
- `107:32442` Action / Register biologic → `[{"type":"NODE","target":"107:26448","navigation":"OVERLAY"}]`
- `107:32443` Action / Analyze chain → `[{"type":"NODE","target":"107:26168","navigation":"OVERLAY"}]`
- `107:32444` Action / Import chain data → `[{"type":"NODE","target":"107:33652","navigation":"OVERLAY"}]`
- `107:32447` Related / HC-014 · Entity → `[{"type":"NODE","target":"107:33242","navigation":"OVERLAY"}]`
- `107:32452` Related / Characterization entry → `[{"type":"NODE","target":"107:30702","navigation":"NAVIGATE"}]`

## ST-132 · UX v4 / Chemistry

**Role:** Primary module. **Frame:** 107:32457 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-32457).

**Displayed content:**

- Chemistry
- Keep chemical identity separate from forms and physical lots.
- Search
- New
- Add compound
- Add reaction
- Add lot
- Compound, forms and lots
- Chemical identity and physical stock remain separate linked records.
- Record
- Type
- Parent
- Stock
- CMP-014
- Compound
- —
- See lots
- FRM-014A
- Molecule form
- 2 lots
- FRM-014B
- 1 lot
- LOT-014A
- Physical lot
- 12.5 mg
- Structure and salt/solvate identity are reviewed before registering a new form.
- Details & tools
- All chemistry tools
- Current selection
- CMP-014 · compound
- How to use Chemistry
- Create a compound or reaction with its
required inputs. Physical lots are
tracked separately in Inventory.
- Identity
- Compound · structure defined
- Forms
- 2 linked molecule forms
- Lots
- 3 physical lots
- Actions
- Salt & solvate library
- Related records
- Form F-014
- Lot L-014 · Inventory

**Authored actions:**

- `107:32565` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:32566` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `107:32569` Action / Add compound → `[{"type":"NODE","target":"107:26478","navigation":"OVERLAY"}]`
- `107:32570` Action / Add reaction → `[{"type":"NODE","target":"107:26504","navigation":"OVERLAY"}]`
- `107:32571` Action / Add lot → `[{"type":"NODE","target":"107:26534","navigation":"OVERLAY"}]`
- `107:32581` Record / CMP-014 → `[{"type":"NODE","target":"107:33242","navigation":"OVERLAY"}]`
- `107:32586` Record / FRM-014A → `[{"type":"NODE","target":"107:33242","navigation":"OVERLAY"}]`
- `107:32591` Record / FRM-014B → `[{"type":"NODE","target":"107:33242","navigation":"OVERLAY"}]`
- `107:32596` Record / LOT-014A → `[{"type":"NODE","target":"107:33242","navigation":"OVERLAY"}]`
- `107:32620` Action / All chemistry tools → `[{"type":"NODE","target":"107:28726","navigation":"OVERLAY"}]`
- `107:32634` Action / Add compound → `[{"type":"NODE","target":"107:26478","navigation":"OVERLAY"}]`
- `107:32635` Action / Add lot → `[{"type":"NODE","target":"107:26534","navigation":"OVERLAY"}]`
- `107:32636` Action / Salt & solvate library → `[{"type":"NODE","target":"107:29073","navigation":"OVERLAY"}]`
- `107:32639` Related / Form F-014 → `[{"type":"NODE","target":"107:33242","navigation":"OVERLAY"}]`
- `107:32644` Related / Lot L-014 · Inventory → `[{"type":"NODE","target":"107:31089","navigation":"NAVIGATE"}]`

## ST-133 · UX v4 / RNA / Oligos

**Role:** Primary module. **Frame:** 107:32649 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-32649).

**Displayed content:**

- RNA / Oligos
- Preview notation and modification changes before commit.
- Search
- New
- Add oligo
- Edit modifications
- Apply pattern
- OL-014 · RNA oligo
- Notation: HELM · 21 nucleotides · preview changes before commit.
- Position
- 1   2   3   4   5   6   7   8   9   10  11  12
- Base
- A   U   G   C   U   A   G   U   C   A   G   U
- Sugar
- r   m   m   r   r   m   r   m   r   r   m   r
- Phosphate
- p   p   s   p   p   s   p   p   s   p   p   p
- Bulk pattern applies to eligible oligos. General RNA sequence records use nucleotide-level editing.
- Record
- Type
- Pattern eligibility
- OL-014
- RNA oligo
- Eligible
- RNA-002
- RNA sequence
- Edit nucleotides individually
- Details & tools
- All rna / oligos tools
- Current selection
- How to use RNA / Oligos
- Confirm nucleic acid type, direction
and modification pattern before
inspecting a candidate.
- Notation
- HELM · 21 nt
- Record type
- RNA oligo · bulk-pattern eligible
- Modifications
- Sugar, base and phosphate
- Actions
- Choose monomer
- Related records
- OL-014 · Entity
- RNA experiment

**Authored actions:**

- `107:32757` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:32758` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `107:32761` Action / Add oligo → `[{"type":"NODE","target":"107:26560","navigation":"OVERLAY"}]`
- `107:32762` Action / Edit modifications → `[{"type":"NODE","target":"107:34153","navigation":"OVERLAY"}]`
- `107:32763` Action / Apply pattern → `[{"type":"NODE","target":"107:34184","navigation":"OVERLAY"}]`
- `107:32812` Action / All rna / oligos tools → `[{"type":"NODE","target":"107:28777","navigation":"OVERLAY"}]`
- `107:32826` Action / Edit modifications → `[{"type":"NODE","target":"107:34153","navigation":"OVERLAY"}]`
- `107:32827` Action / Apply pattern → `[{"type":"NODE","target":"107:34184","navigation":"OVERLAY"}]`
- `107:32828` Action / Choose monomer → `[{"type":"NODE","target":"107:34223","navigation":"OVERLAY"}]`
- `107:32831` Related / OL-014 · Entity → `[{"type":"NODE","target":"107:33242","navigation":"OVERLAY"}]`
- `107:32836` Related / RNA experiment → `[{"type":"NODE","target":"107:30702","navigation":"NAVIGATE"}]`

## ST-134 · UX v4 / In Vivo

**Role:** Primary module. **Frame:** 107:32841 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-32841).

**Displayed content:**

- In Vivo
- Keep the animal, task and measurement units in view.
- Search
- New
- Record measurement
- Today’s schedule
- New study
- Study S-014
- Today’s work · 16 animals · cohort C-01
- Animal / ear tag
- Task
- State
- Last value
- A-003
- Body weight
- Due today
- 23.4 g
- A-004
- 24.1 g
- A-005
- Recorded
- 23.8 g
- A-006
- Health check
- Ready
- —
- Confirm animal identity and units before saving; repeated capture keeps the scheduled task in view.
- Details & tools
- All in vivo tools
- Current selection
- Study S-014 · animal A-003
- How to use In Vivo
- Start with a study, add animals, then
record measurements. Required fields
appear in the relevant task.
- Body weight · due today
- Identifier
- Ear tag A-003
- After save
- Confirm, then next animal
- Actions
- Review schedule
- Export study
- Related records
- Cohort C-01
- Recent measurements

**Authored actions:**

- `107:32949` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:32950` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `107:32953` Action / Record measurement → `[{"type":"NODE","target":"107:34250","navigation":"OVERLAY"}]`
- `107:32954` Action / Today’s schedule → `[{"type":"NODE","target":"107:34281","navigation":"OVERLAY"}]`
- `107:32955` Action / New study → `[{"type":"NODE","target":"107:26590","navigation":"OVERLAY"}]`
- `107:32965` Record / A-003 → `[{"type":"NODE","target":"107:34250","navigation":"OVERLAY"}]`
- `107:32970` Record / A-004 → `[{"type":"NODE","target":"107:34250","navigation":"OVERLAY"}]`
- `107:32975` Record / A-005 → `[{"type":"NODE","target":"107:34250","navigation":"OVERLAY"}]`
- `107:32980` Record / A-006 → `[{"type":"NODE","target":"107:34250","navigation":"OVERLAY"}]`
- `107:33004` Action / All in vivo tools → `[{"type":"NODE","target":"107:28819","navigation":"OVERLAY"}]`
- `107:33018` Action / Record measurement → `[{"type":"NODE","target":"107:34250","navigation":"OVERLAY"}]`
- `107:33019` Action / Review schedule → `[{"type":"NODE","target":"107:34281","navigation":"OVERLAY"}]`
- `107:33020` Action / Export study → `[{"type":"NODE","target":"107:29047","navigation":"OVERLAY"}]`
- `107:33023` Related / Cohort C-01 → `[{"type":"NODE","target":"107:33242","navigation":"OVERLAY"}]`
- `107:33028` Related / Recent measurements → `[{"type":"NODE","target":"107:34309","navigation":"OVERLAY"}]`

## ST-135 · UX v4 / Bioprocess

**Role:** Primary module. **Frame:** 107:33033 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-33033).

**Displayed content:**

- Bioprocess
- Plan from a fixed recipe and compare named conditions.
- Search
- New
- Plan run
- Create draft
- Compare runs
- Recipe
- Planned runs
- Results
- Recipe RCP-014 / revision 2 is effective and read-only. Create a draft to change future runs.
- Unit operation
- Condition A
- Condition B
- Status
- Seed train
- 37 °C
- Complete
- Culture
- pH 7.0
- pH 7.2
- In progress
- Harvest
- Day 5
- Planned
- Capture
- Standard resin
- Run
- Recipe reference
- Condition
- State
- BP-014-A1
- RCP-014 / rev 2
- A · replicate 1
- BP-014-B1
- B · replicate 1
- Details & tools
- All bioprocess tools
- Current selection
- Recipe RCP-014 · effective
- How to use Bioprocess
- A recipe describes the process. A run
records an execution with its own
input lots and results.
- RCP-014 / revision 2
- Study
- BP-014 · 2 conditions
- Runs
- 3 planned · 1 in progress
- Actions
- Create recipe draft
- Related records
- Batch BP-014
- Execution records

**Authored actions:**

- `107:33141` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:33142` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `107:33145` Action / Plan run → `[{"type":"NODE","target":"107:26776","navigation":"OVERLAY"}]`
- `107:33146` Action / Create draft → `[{"type":"NODE","target":"107:34332","navigation":"OVERLAY"}]`
- `107:33147` Action / Compare runs → `[{"type":"NODE","target":"107:34359","navigation":"OVERLAY"}]`
- `107:33150` Action / Planned runs → `[{"type":"NODE","target":"107:33978","navigation":"OVERLAY"}]`
- `107:33151` Action / Results → `[{"type":"NODE","target":"107:34359","navigation":"OVERLAY"}]`
- `107:33163` Record / Seed train → `[{"type":"NODE","target":"107:34332","navigation":"OVERLAY"}]`
- `107:33168` Record / Culture → `[{"type":"NODE","target":"107:34332","navigation":"OVERLAY"}]`
- `107:33173` Record / Harvest → `[{"type":"NODE","target":"107:34332","navigation":"OVERLAY"}]`
- `107:33178` Record / Capture → `[{"type":"NODE","target":"107:34332","navigation":"OVERLAY"}]`
- `107:33213` Action / All bioprocess tools → `[{"type":"NODE","target":"107:28870","navigation":"OVERLAY"}]`
- `107:33227` Action / Plan run → `[{"type":"NODE","target":"107:26776","navigation":"OVERLAY"}]`
- `107:33228` Action / Create recipe draft → `[{"type":"NODE","target":"107:34332","navigation":"OVERLAY"}]`
- `107:33229` Action / Compare runs → `[{"type":"NODE","target":"107:34359","navigation":"OVERLAY"}]`
- `107:33232` Related / Batch BP-014 → `[{"type":"NODE","target":"107:33242","navigation":"OVERLAY"}]`
- `107:33237` Related / Execution records → `[{"type":"NODE","target":"107:30702","navigation":"NAVIGATE"}]`

## ST-136 · State / Entity

**Role:** Linked functional state. **Frame:** 107:33242 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-33242).

**Displayed content:**

- Scientific record preview
- Close
- DNA-001 · pHelixBench-Demo · scientific identity
- Type
- DNA construct · 3021 bp · circular
- Stable ID
- DNA-001
- Version
- 7 · saved today at 10:42
- Open full record
- Find physical samples

**Authored actions:**

- `107:33246` Action / Close → `[{"type":"CLOSE"}]`
- `107:33263` Action / Open full record → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`
- `107:33264` Action / Find physical samples → `[{"type":"NODE","target":"107:33618","navigation":"SWAP"}]`

## ST-137 · State / Attention

**Role:** Reference or alternate fixture. **Frame:** 107:33265 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-33265).

**Displayed content:**

- Work that needs attention
- Close
- Each action retains the record and project context.
- Record
- Needs action
- EXP-014
- 2 unsubmitted result rows
- WF-042
- Repair downstream sample mapping
- R-018
- Review failed validation
- Open notebook
- Repair mapping
- Inspect failed run

**Authored actions:**

- `107:33269` Action / Close → `[{"type":"CLOSE"}]`
- `107:33287` Action / Open notebook → `[{"type":"NODE","target":"107:30702","navigation":"NAVIGATE"}]`
- `107:33288` Action / Repair mapping → `[{"type":"NODE","target":"107:33765","navigation":"SWAP"}]`
- `107:33289` Action / Inspect failed run → `[{"type":"NODE","target":"107:34042","navigation":"SWAP"}]`

## ST-138 · State / SequenceFind

**Role:** Linked functional state. **Frame:** 107:33290 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-33290).

**Displayed content:**

- Find in sequence
- Close
- Search within DNA-001 · forward and reverse strand.
- Query
- T7 promoter
- Search mode
- Annotation name
- Matches
- 1 match · 97–116
- Select match
- Cancel

**Authored actions:**

- `107:33294` Action / Close → `[{"type":"CLOSE"}]`
- `107:33311` Action / Select match → `[{"type":"NODE","target":"107:29677","navigation":"NAVIGATE"}]`
- `107:33312` Action / Cancel → `[{"type":"CLOSE"}]`

## ST-139 · State / AnnotationPreview

**Role:** Linked functional state. **Frame:** 107:33313 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-33313).

**Displayed content:**

- Review proposed annotations
- Close
- DNA-001 · input version 7 · no changes applied yet.
- 2 proposed additions; 1 overlapping annotation needs review.
- Annotation
- Range
- Decision
- T7 promoter
- 97–116
- Keep existing
- RBS
- 150–165
- Add
- Terminator
- 940–1025
- Review overlap
- Commit scope
- Only reviewed additions are applied
- Input reference
- DNA-001 / v7
- Cancel
- Apply selected

**Authored actions:**

- `107:33317` Action / Close → `[{"type":"CLOSE"}]`
- `107:33350` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:33351` Action / Apply selected → `[{"type":"NODE","target":"107:34881","navigation":"SWAP"}]`

## ST-140 · State / PcrResult

**Role:** Linked functional state. **Frame:** 107:33352 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-33352).

**Displayed content:**

- PCR simulation result
- Close
- Input: DNA-001 / v7 · linked pair P-01
- Predicted product
- 255 bp · linear
- Primers
- T7-F / Demo-R · linked pair P-01
- Template coordinates
- 97–351
- Output action
- Save a separate PCR product record
- Save product
- Review parameters

**Authored actions:**

- `107:33356` Action / Close → `[{"type":"CLOSE"}]`
- `107:33373` Action / Save product → `[{"type":"NODE","target":"107:34861","navigation":"SWAP"}]`
- `107:33374` Action / Review parameters → `[{"type":"NODE","target":"107:25266","navigation":"SWAP"}]`

## ST-141 · State / DigestResult

**Role:** Linked functional state. **Frame:** 107:33375 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-33375).

**Displayed content:**

- Virtual digest result
- Close
- DNA-001 / v7 · EcoRI + BamHI
- Fragment
- Size
- Input
- Fragment 1
- 18 bp
- DNA-001 / v7
- Fragment 2
- 3003 bp
- Digest parameters
- EcoRI + BamHI · circular input
- Total fragment length
- 3021 bp
- Save result
- Change enzymes

**Authored actions:**

- `107:33379` Action / Close → `[{"type":"CLOSE"}]`
- `107:33403` Action / Save result → `[{"type":"NODE","target":"107:34861","navigation":"SWAP"}]`
- `107:33404` Action / Change enzymes → `[{"type":"NODE","target":"110:20288","navigation":"NAVIGATE"}]`

## ST-142 · State / VersionCompare

**Role:** Linked functional state. **Frame:** 107:33405 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-33405).

**Displayed content:**

- Compare saved versions
- Close
- DNA-001 · version 6 compared with current version 7
- Field
- Version 6
- Version 7
- T7 promoter
- Manual label
- Reviewed label
- Primer pair
- P-01 linked
- Sequence length
- 3021 bp
- Keep current
- Restore preview

**Authored actions:**

- `107:33409` Action / Close → `[{"type":"CLOSE"}]`
- `107:33431` Action / Keep current → `[{"type":"CLOSE"}]`
- `107:33432` Action / Restore preview → `[{"type":"NODE","target":"107:33433","navigation":"SWAP"}]`

## ST-143 · State / RestorePreview

**Role:** Linked functional state. **Frame:** 107:33433 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-33433).

**Displayed content:**

- Review version restoration
- Close
- Restoring creates an explicit record change; preview alone changes nothing.
- The current version remains identifiable in history.
- Record
- DNA-001
- Target version
- 6 · primer pair added
- Affected current state
- Version 7 annotation changes
- Cancel
- Restore version 6

**Authored actions:**

- `107:33437` Action / Close → `[{"type":"CLOSE"}]`
- `107:33459` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:33460` Action / Restore version 6 → `[{"type":"NODE","target":"107:34901","navigation":"SWAP"}]`

## ST-144 · State / AuditTrail

**Role:** Linked functional state. **Frame:** 107:33461 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-33461).

**Displayed content:**

- Record activity
- Close
- EXP-014 · changes and structured submissions are recorded separately.
- Time
- Event
- Actor
- 10:42
- Feature annotation saved
- You
- 10:36
- Notebook text saved
- 10:30
- Plate snapshot captured
- M. Chen
- Open notebook

**Authored actions:**

- `107:33465` Action / Close → `[{"type":"CLOSE"}]`
- `107:33487` Action / Open notebook → `[{"type":"NODE","target":"107:30702","navigation":"NAVIGATE"}]`
- `107:33488` Action / Close → `[{"type":"CLOSE"}]`

## ST-145 · State / InsertResults

**Role:** Linked functional state. **Frame:** 107:33489 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-33489).

**Displayed content:**

- Insert a result table
- Close
- Structured results become reusable records only after submission.
- Schema
- Expression assay results
- Related entity
- DNA-001
- Starting rows
- 2 rows
- Destination
- EXP-014 / Assay results
- Insert table
- Cancel

**Authored actions:**

- `107:33493` Action / Close → `[{"type":"CLOSE"}]`
- `107:33514` Action / Insert table → `[{"type":"NODE","target":"107:30702","navigation":"NAVIGATE"}]`
- `107:33515` Action / Cancel → `[{"type":"CLOSE"}]`

## ST-146 · State / SubmitRows

**Role:** Linked functional state. **Frame:** 107:33516 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-33516).

**Displayed content:**

- Review result submission
- Close
- EXP-014 · text is already saved; these rows are not yet submitted.
- Submission will create two result records linked to this entry.
- Source row
- Sample
- Signal (a.u.)
- 1
- S-001
- 1280
- 2
- S-002
- 1315
- Write destination
- Expression results
- Provenance
- EXP-014 / Assay results · DNA-001
- Validation
- 2 valid · 0 errors · 0 skipped
- Cancel
- Submit 2 results

**Authored actions:**

- `107:33520` Action / Close → `[{"type":"CLOSE"}]`
- `107:33552` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:33553` Action / Submit 2 results → `[{"type":"NODE","target":"107:34507","navigation":"SWAP"}]`

## ST-147 · State / ReviewBlocked

**Role:** Linked functional state. **Frame:** 107:33554 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-33554).

**Displayed content:**

- Entry is not ready for review
- Close
- EXP-014 · draft saved · formal review has not started.
- Submit the two pending result rows before starting entry review.
- Blocking prerequisite
- 2 structured result rows are unsubmitted
- What is already safe
- Notebook prose is saved
- Next action
- Review and submit the result rows
- Keep editing
- Review unsubmitted rows

**Authored actions:**

- `107:33558` Action / Close → `[{"type":"CLOSE"}]`
- `107:33577` Action / Keep editing → `[{"type":"NODE","target":"107:30702","navigation":"NAVIGATE"}]`
- `107:33578` Action / Review unsubmitted rows → `[{"type":"NODE","target":"107:33516","navigation":"SWAP"}]`

## ST-148 · State / Snapshot

**Role:** Linked functional state. **Frame:** 107:33579 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-33579).

**Displayed content:**

- Experiment plate snapshot
- Close
- P-014 captured for EXP-014 · 14 Nov at 16:20 · read-only
- This historical view does not change when live inventory is edited.
- Well
- Captured contents
- Volume
- A1
- DNA-001 / sample S-001
- 50 µL
- A2
- DNA-001 / sample S-002
- A3
- Control C-001
- Captured for
- EXP-014 · source plate P-014
- Mode
- Historical snapshot · read-only
- Open live plate
- Compare with live

**Authored actions:**

- `107:33583` Action / Close → `[{"type":"CLOSE"}]`
- `107:33616` Action / Open live plate → `[{"type":"NODE","target":"107:31089","navigation":"NAVIGATE"}]`
- `107:33617` Action / Compare with live → `[{"type":"NODE","target":"107:34584","navigation":"SWAP"}]`

## ST-149 · State / FindSamples

**Role:** Linked functional state. **Frame:** 107:33618 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-33618).

**Displayed content:**

- Physical samples for DNA-001
- Close
- An entity can be held in several physical containers.
- Container
- Location
- Quantity
- P-014 / A1
- Freezer A / Shelf 2
- 50 µL
- P-014 / A2
- Tube T-006
- Freezer B / Rack 1
- 80 µL
- Scientific entity
- DNA-001
- Scope
- Accessible physical containers
- Open selected container
- Open scientific identity

**Authored actions:**

- `107:33622` Action / Close → `[{"type":"CLOSE"}]`
- `107:33650` Action / Open selected container → `[{"type":"NODE","target":"107:31089","navigation":"NAVIGATE"}]`
- `107:33651` Action / Open scientific identity → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`

## ST-150 · State / ImportUpload

**Role:** Linked functional state. **Frame:** 107:33652 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-33652).

**Displayed content:**

- Import entities from spreadsheet
- Close
- Upload, map, validate and review before records are written.
- Destination
- Registry / DNA construct
- File
- constructs_batch.csv · 24 rows
- Duplicate policy
- Block duplicate identities; review conflicts
- Template
- DNA construct import template
- Map columns
- Cancel

**Authored actions:**

- `107:33656` Action / Close → `[{"type":"CLOSE"}]`
- `107:33677` Action / Map columns → `[{"type":"NODE","target":"107:34389","navigation":"SWAP"}]`
- `107:33678` Action / Cancel → `[{"type":"CLOSE"}]`

## ST-151 · State / EntityMap

**Role:** Linked functional state. **Frame:** 107:33679 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-33679).

**Displayed content:**

- Related scientific records
- Close
- DNA-001 · relationships are also available as an accessible list.
- Related record
- Relationship
- EXP-014
- Experiment using DNA-001
- P-014
- Contains physical samples
- PR-001
- Primer binds to DNA-001
- Preview selected record
- Open inventory

**Authored actions:**

- `107:33683` Action / Close → `[{"type":"CLOSE"}]`
- `107:33701` Action / Preview selected record → `[{"type":"NODE","target":"107:33242","navigation":"SWAP"}]`
- `107:33702` Action / Open inventory → `[{"type":"NODE","target":"107:33618","navigation":"SWAP"}]`

## ST-152 · State / TransferReview

**Role:** Linked functional state. **Frame:** 107:33703 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-33703).

**Displayed content:**

- Review sample transfer
- Close
- Live inventory · P-014 · A1–A4 selected
- 4 samples will move. Experimental snapshots will remain unchanged.
- Source
- Destination
- Volume
- P-014 / A1
- P-021 / B1
- 10 µL
- P-014 / A2
- P-021 / B2
- P-014 / A3–A4
- P-021 / B3–B4
- 10 µL each
- Validation
- Destination wells free · sufficient source volume
- Scope
- 4 selected wells; not all matching wells
- Cancel
- Confirm transfer

**Authored actions:**

- `107:33707` Action / Close → `[{"type":"CLOSE"}]`
- `107:33740` Action / Cancel → `[{"type":"NODE","target":"107:31089","navigation":"NAVIGATE"}]`
- `107:33741` Action / Confirm transfer → `[{"type":"NODE","target":"128:5896","navigation":"NAVIGATE"}]`

## ST-153 · State / BarcodeLookup

**Role:** Linked functional state. **Frame:** 107:33742 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-33742).

**Displayed content:**

- Find a physical container
- Close
- Scan or paste a barcode; verify the matched item before opening.
- Barcode
- PLT-P014
- Match
- Plate P-014 · Freezer A / Shelf 2
- Contents
- 4 selected samples · DNA-001
- Open container
- Cancel

**Authored actions:**

- `107:33746` Action / Close → `[{"type":"CLOSE"}]`
- `107:33763` Action / Open container → `[{"type":"NODE","target":"107:31089","navigation":"NAVIGATE"}]`
- `107:33764` Action / Cancel → `[{"type":"CLOSE"}]`

## ST-154 · State / MappingRepair

**Role:** Linked functional state. **Frame:** 107:33765 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-33765).

**Displayed content:**

- Repair the failed sample mapping
- Close
- WF-042 · sequence validation · task identity is preserved.
- The downstream task is blocked until this required link is valid.
- Affected field
- Sample · required entity link
- Map from
- Upstream task / registered construct
- Map to
- Sequence validation / Sample
- Selected record
- DNA-001 · pHelixBench-Demo
- Validate correction
- Cancel

**Authored actions:**

- `107:33769` Action / Close → `[{"type":"CLOSE"}]`
- `107:33795` Action / Validate correction → `[{"type":"NODE","target":"107:34612","navigation":"SWAP"}]`
- `107:33796` Action / Cancel → `[{"type":"CLOSE"}]`

## ST-155 · State / RequestSent

**Role:** Linked functional state. **Frame:** 107:33797 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-33797).

**Displayed content:**

- Request is sent
- Close
- REQ-014 · sent to Molecular Biology · progress is read-only.
- Changes after sending require a supported lifecycle action.
- Task
- Assignee
- State
- Sequence validation
- You
- Failed mapping
- Assay review
- M. Chen
- Pending
- Request
- REQ-014 · sent
- Editing
- Read-only after sending
- Open related task
- View activity

**Authored actions:**

- `107:33801` Action / Close → `[{"type":"CLOSE"}]`
- `107:33830` Action / Open related task → `[{"type":"NODE","target":"107:33832","navigation":"SWAP"}]`
- `107:33831` Action / View activity → `[{"type":"NODE","target":"107:33461","navigation":"SWAP"}]`

## ST-156 · State / ExecuteTask

**Role:** Linked functional state. **Frame:** 107:33832 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-33832).

**Displayed content:**

- Execute sequence validation
- Close
- WF-043 · ready · required inputs are available.
- Input record
- DNA-001 / v7
- Assignee
- You
- Output
- Sequence QC result
- Execution entry
- EXP-014
- Review and start
- Cancel

**Authored actions:**

- `107:33836` Action / Close → `[{"type":"CLOSE"}]`
- `107:33857` Action / Review and start → `[{"type":"NODE","target":"107:34635","navigation":"SWAP"}]`
- `107:33858` Action / Cancel → `[{"type":"CLOSE"}]`

## ST-157 · State / RequestDraft

**Role:** Linked functional state. **Frame:** 107:33859 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-33859).

**Displayed content:**

- Prepare a work request
- Close
- Drafts remain editable until you send them.
- Request type
- Sequence validation
- Name
- Expression construct QC
- Project
- Expression study
- Requested record
- DNA-001
- Instructions
- Validate sequence and attach the reviewed result.
- Send request
- Keep draft

**Authored actions:**

- `107:33863` Action / Close → `[{"type":"CLOSE"}]`
- `107:33888` Action / Send request → `[{"type":"NODE","target":"107:33797","navigation":"SWAP"}]`
- `107:33889` Action / Keep draft → `[{"type":"CLOSE"}]`

## ST-158 · State / AnalysisAccess

**Role:** Linked functional state. **Frame:** 107:33890 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-33890).

**Displayed content:**

- Review investigation access
- Close
- Open a separate investigation with this dashboard’s parameters.
- Dashboard access and investigation access must not be assumed identical.
- Source
- Expression QC dashboard
- Parameters
- Project = Expression study
- Included records
- Accessible result set at capture time
- Audience
- Only me · change access explicitly
- Create private investigation
- Cancel

**Authored actions:**

- `107:33894` Action / Close → `[{"type":"CLOSE"}]`
- `107:33920` Action / Create private investigation → `[{"type":"NODE","target":"107:34941","navigation":"SWAP"}]`
- `107:33921` Action / Cancel → `[{"type":"CLOSE"}]`

## ST-159 · State / ParameterEmpty

**Role:** Linked functional state. **Frame:** 107:33922 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-33922).

**Displayed content:**

- Choose a dashboard parameter
- Close
- No query is running. A required project filter is unset.
- A blank parameter differs from a completed query with no matching rows.
- Project
- Expression study
- Date range
- Last 30 days
- Data scope
- Records you can access
- Apply filters
- Cancel

**Authored actions:**

- `107:33926` Action / Close → `[{"type":"CLOSE"}]`
- `107:33948` Action / Apply filters → `[{"type":"NODE","target":"107:31682","navigation":"NAVIGATE"}]`
- `107:33949` Action / Cancel → `[{"type":"CLOSE"}]`

## ST-160 · State / AutomationTest

**Role:** Linked functional state. **Frame:** 107:33950 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-33950).

**Displayed content:**

- Test the draft pipeline
- Close
- TPL-014 draft · test output has not been written to Registry.
- Step
- Validation
- Output
- CSV input
- Passed
- 96 rows
- Column types
- Sample ID / signal
- Write preview
- Not committed
- 96 proposed results
- Inspect step
- Use published template

**Authored actions:**

- `107:33954` Action / Close → `[{"type":"CLOSE"}]`
- `107:33976` Action / Inspect step → `[{"type":"NODE","target":"107:34103","navigation":"SWAP"}]`
- `107:33977` Action / Review run inputs → `[{"type":"NODE","target":"107:33978","navigation":"SWAP"}]`

## ST-161 · State / RunReview

**Role:** Linked functional state. **Frame:** 107:33978 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-33978).

**Displayed content:**

- Review automation inputs
- Close
- This run will use a fixed template reference and explicit writeback target.
- Review inputs and destination before starting a write operation.
- Template reference
- TPL-012 · published configuration
- Input
- assay_results.csv · 96 source rows
- Destination
- Expression results / Expression study
- Writeback policy
- Create reviewed results; reject duplicate run IDs
- Start run
- Cancel

**Authored actions:**

- `107:33982` Action / Close → `[{"type":"CLOSE"}]`
- `107:34008` Action / Start run → `[{"type":"NODE","target":"128:6148","navigation":"NAVIGATE"}]`
- `107:34009` Action / Cancel → `[{"type":"CLOSE"}]`

## ST-162 · State / PublishReview

**Role:** Linked functional state. **Frame:** 107:34010 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-34010).

**Displayed content:**

- Publish this automation template
- Close
- Publication makes this configuration available for execution.
- Changing a published configuration requires a deliberate replacement draft.
- Template
- Expression QC · TPL-014
- Test status
- Passed · 96 valid source rows
- Publication state
- Draft → Published
- Existing runs
- Keep their original immutable template reference
- Publish template
- Keep draft

**Authored actions:**

- `107:34014` Action / Close → `[{"type":"CLOSE"}]`
- `107:34040` Action / Publish template → `[{"type":"NODE","target":"107:34734","navigation":"SWAP"}]`
- `107:34041` Action / Keep draft → `[{"type":"CLOSE"}]`

## ST-163 · State / RunFailed

**Role:** Linked functional state. **Frame:** 107:34042 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-34042).

**Displayed content:**

- Run R-018 needs attention
- Close
- Validation failed · no result records were written.
- Missing Sample ID in source row 18. The input file and diagnostics are retained.
- Source row
- Field
- Issue
- 18
- Sample ID
- Missing required value
- Failed stage
- Column validation
- Writeback
- No records written
- Recovery
- Use corrected input in a new reviewed run
- Inspect error
- Review corrected input

**Authored actions:**

- `107:34046` Action / Close → `[{"type":"CLOSE"}]`
- `107:34074` Action / Inspect error → `[{"type":"NODE","target":"107:34103","navigation":"SWAP"}]`
- `107:34075` Action / Review corrected input → `[{"type":"NODE","target":"128:6118","navigation":"NAVIGATE"}]`

## ST-164 · State / ConnectionAccess

**Role:** Linked functional state. **Frame:** 107:34076 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-34076).

**Displayed content:**

- Instrument connections
- Close
- Configured input sources are separate from installation and administration.
- Available source
- LC reader · configured connection
- Connection health
- Available · last file received 10:31
- Your capability
- Use configured files
- Restricted action
- Connection setup requires workspace administrator

**Authored actions:**

- `107:34080` Action / Close → `[{"type":"CLOSE"}]`
- `107:34101` Action / Use configured files → `[{"type":"NODE","target":"107:29155","navigation":"SWAP"}]`
- `107:34102` Action / Close → `[{"type":"CLOSE"}]`

## ST-165 · State / PipelineStep

**Role:** Linked functional state. **Frame:** 107:34103 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-34103).

**Displayed content:**

- Configure column validation
- Close
- Selected step 2 · input and output contracts remain visible.
- Input
- Parsed CSV dataset
- Required fields
- Sample ID · Signal
- Field types
- Text · Number (a.u.)
- Error policy
- Block writeback and retain row diagnostics
- Save and test
- View failed rows

**Authored actions:**

- `107:34107` Action / Close → `[{"type":"CLOSE"}]`
- `107:34128` Action / Save and test → `[{"type":"NODE","target":"107:33950","navigation":"SWAP"}]`
- `107:34129` Action / View failed rows → `[{"type":"NODE","target":"107:34423","navigation":"SWAP"}]`

## ST-166 · State / AlignmentResult

**Role:** Linked functional state. **Frame:** 107:34130 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-34130).

**Displayed content:**

- Pairwise alignment result
- Close
- DNA-001 / v7 versus DNA-002 / v3
- Input pair
- DNA-001 / v7 and DNA-002 / v3
- Identity
- 99.2% in aligned region
- Differences
- 3 substitutions · 1 gap
- Output
- New alignment record; source sequences unchanged
- Save alignment
- Review input pair

**Authored actions:**

- `107:34134` Action / Close → `[{"type":"CLOSE"}]`
- `107:34151` Action / Save alignment → `[{"type":"NODE","target":"107:34861","navigation":"SWAP"}]`
- `107:34152` Action / Review input pair → `[{"type":"NODE","target":"107:25425","navigation":"SWAP"}]`

## ST-167 · State / OligoEdit

**Role:** Linked functional state. **Frame:** 107:34153 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-34153).

**Displayed content:**

- Edit nucleotide modifications
- Close
- OL-014 · RNA oligo · position 3 selected
- Notation
- HELM
- Base
- G · natural analogue G
- Sugar
- 2′-O-methyl ribose
- 3′ phosphate
- Phosphorothioate
- Scope
- Selected nucleotide only
- Review modification
- Cancel

**Authored actions:**

- `107:34157` Action / Close → `[{"type":"CLOSE"}]`
- `107:34182` Action / Review modification → `[{"type":"NODE","target":"107:34184","navigation":"SWAP"}]`
- `107:34183` Action / Cancel → `[{"type":"CLOSE"}]`

## ST-168 · State / PatternPreview

**Role:** Linked functional state. **Frame:** 107:34184 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-34184).

**Displayed content:**

- Preview the oligo pattern
- Close
- Eligible RNA oligo OL-014 · original values remain available.
- This bulk operation does not apply to general RNA sequence records.
- Position
- Original
- Proposed
- 3 · sugar
- Ribose
- 2′-O-methyl
- 3 · phosphate
- Phosphate
- Phosphorothioate
- 6 · sugar
- Eligible selection
- 1 oligo · OL-014
- Overwrite policy
- Preserve modifications outside reviewed positions
- Cancel
- Apply to 1 oligo

**Authored actions:**

- `107:34188` Action / Close → `[{"type":"CLOSE"}]`
- `107:34221` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:34222` Action / Apply to 1 oligo → `[{"type":"NODE","target":"107:34961","navigation":"SWAP"}]`

## ST-169 · State / MonomerPicker

**Role:** Linked functional state. **Frame:** 107:34223 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-34223).

**Displayed content:**

- Choose a monomer
- Close
- Use allowed monomers; library administration is a separate capability.
- Polymer
- RNA
- Search
- 2′-O-methyl
- Selected symbol
- mG · 2′-O-methyl guanosine
- Access
- Read and use · library editing requires administrator
- Use monomer
- Cancel

**Authored actions:**

- `107:34227` Action / Close → `[{"type":"CLOSE"}]`
- `107:34248` Action / Use monomer → `[{"type":"NODE","target":"107:34153","navigation":"SWAP"}]`
- `107:34249` Action / Cancel → `[{"type":"CLOSE"}]`

## ST-170 · State / MeasureAnimal

**Role:** Linked functional state. **Frame:** 107:34250 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-34250).

**Displayed content:**

- Record body weight
- Close
- Study S-014 · scheduled task · cohort C-01
- Animal / ear tag
- A-003 · identity confirmed
- Measurement
- 23.6
- Unit
- g
- Previous measurement
- 23.4 g · 14 Nov
- After save
- Confirm result, then move to next animal
- Review measurement
- Cancel

**Authored actions:**

- `107:34254` Action / Close → `[{"type":"CLOSE"}]`
- `107:34279` Action / Review measurement → `[{"type":"NODE","target":"107:34807","navigation":"SWAP"}]`
- `107:34280` Action / Cancel → `[{"type":"CLOSE"}]`

## ST-171 · State / StudySchedule

**Role:** Linked functional state. **Frame:** 107:34281 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-34281).

**Displayed content:**

- Today’s study tasks
- Close
- Study S-014 · scheduled work and capture share the same context.
- Animal
- Task
- State
- A-003
- Body weight
- Due today
- A-004
- A-005
- Completed
- Record selected task
- Open study

**Authored actions:**

- `107:34285` Action / Close → `[{"type":"CLOSE"}]`
- `107:34307` Action / Record selected task → `[{"type":"NODE","target":"107:34250","navigation":"SWAP"}]`
- `107:34308` Action / Open study → `[{"type":"NODE","target":"107:32841","navigation":"NAVIGATE"}]`

## ST-172 · State / MeasurementSaved

**Role:** Linked functional state. **Frame:** 107:34309 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-34309).

**Displayed content:**

- Measurement saved
- Close
- A-003 · body weight 23.6 g · linked to the scheduled task.
- Saved value
- 23.6 g
- Animal
- A-003 · identity confirmed
- Task link
- Study S-014 / today’s body weight
- Next animal
- A-004 · identity must be confirmed separately
- Next animal A-004
- Return to study

**Authored actions:**

- `107:34313` Action / Close → `[{"type":"CLOSE"}]`
- `107:34330` Action / Next animal A-004 → `[{"type":"NODE","target":"107:34834","navigation":"SWAP"}]`
- `107:34331` Action / Return to study → `[{"type":"NODE","target":"107:32841","navigation":"NAVIGATE"}]`

## ST-173 · State / RecipeDraft

**Role:** Linked functional state. **Frame:** 107:34332 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-34332).

**Displayed content:**

- Create a recipe draft
- Close
- RCP-014 / revision 2 is effective and remains unchanged.
- New draft
- RCP-014-D3
- Based on
- Effective revision 2
- Reason
- Evaluate a new culture pH condition
- Existing runs
- Remain linked to effective revision 2
- Create editable draft
- Cancel

**Authored actions:**

- `107:34336` Action / Close → `[{"type":"CLOSE"}]`
- `107:34357` Action / Create editable draft → `[{"type":"NODE","target":"107:34981","navigation":"SWAP"}]`
- `107:34358` Action / Cancel → `[{"type":"CLOSE"}]`

## ST-174 · State / RunCompare

**Role:** Linked functional state. **Frame:** 107:34359 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-34359).

**Displayed content:**

- Compare bioprocess runs
- Close
- Named conditions and units are retained across runs.
- Run
- Condition
- Peak signal
- BP-014-A1
- pH 7.0
- 82 a.u.
- BP-014-B1
- pH 7.2
- 96 a.u.
- Recipe reference
- RCP-014 / revision 2
- Comparison scope
- Same elapsed time and measurement unit
- Open source run
- Return to recipe

**Authored actions:**

- `107:34363` Action / Close → `[{"type":"CLOSE"}]`
- `107:34387` Action / Open source run → `[{"type":"NODE","target":"107:33242","navigation":"SWAP"}]`
- `107:34388` Action / Return to recipe → `[{"type":"NODE","target":"107:33033","navigation":"NAVIGATE"}]`

## ST-175 · State / ImportMapping

**Role:** Linked functional state. **Frame:** 107:34389 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-34389).

**Displayed content:**

- Map spreadsheet columns
- Close
- constructs_batch.csv · 24 rows · DNA construct schema
- Source column
- Target field
- Example
- construct_name
- Name
- pHelixBench-Demo
- sequence
- DNA sequence
- ACAGTG…
- parent_id
- Parent entity
- DNA-000
- Saved mapping
- DNA construct CSV · reusable after review
- Next step
- Validate types, identities and linked records
- Validate rows
- Back

**Authored actions:**

- `107:34393` Action / Close → `[{"type":"CLOSE"}]`
- `107:34421` Action / Validate rows → `[{"type":"NODE","target":"107:34423","navigation":"SWAP"}]`
- `107:34422` Action / Back → `[{"type":"NODE","target":"107:33652","navigation":"SWAP"}]`

## ST-176 · State / ImportErrors

**Role:** Linked functional state. **Frame:** 107:34423 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-34423).

**Displayed content:**

- Resolve source-row errors
- Close
- constructs_batch.csv · 22 valid · 2 invalid · nothing imported
- Row 7 has a duplicate identity; row 18 has an unresolved parent link.
- Row
- Field
- Error
- 7
- Name
- Duplicate identity
- 18
- Parent entity
- Unresolved record link
- Default behavior
- Block commit until errors are resolved
- Partial import
- Requires explicit skipped-row review
- Retained data
- File, mappings and valid rows are preserved
- Review corrected rows
- Back to mapping

**Authored actions:**

- `107:34427` Action / Close → `[{"type":"CLOSE"}]`
- `107:34459` Action / Review corrected rows → `[{"type":"NODE","target":"107:34461","navigation":"SWAP"}]`
- `107:34460` Action / Back to mapping → `[{"type":"NODE","target":"107:34389","navigation":"SWAP"}]`

## ST-177 · State / ImportReview

**Role:** Linked functional state. **Frame:** 107:34461 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-34461).

**Displayed content:**

- Review corrected import
- Close
- 24 valid rows · 0 unresolved errors · 0 skipped
- Destination
- Registry / DNA construct
- New entities
- 24
- Invalid rows
- 0
- Skipped rows
- Mapping policy
- Linked identities resolved before commit
- Import 24 entities
- Back to errors

**Authored actions:**

- `107:34465` Action / Close → `[{"type":"CLOSE"}]`
- `107:34485` Action / Import 24 entities → `[{"type":"NODE","target":"107:34487","navigation":"SWAP"}]`
- `107:34486` Action / Back to errors → `[{"type":"NODE","target":"107:34423","navigation":"SWAP"}]`

## ST-178 · State / ImportDone

**Role:** Linked functional state. **Frame:** 107:34487 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-34487).

**Displayed content:**

- Import completed
- Close
- 24 entities created · 0 skipped · source file retained
- Created IDs
- DNA-021 through DNA-044
- Source
- constructs_batch.csv
- Write summary
- 24 created · 0 updated · 0 skipped
- Open Registry

**Authored actions:**

- `107:34491` Action / Close → `[{"type":"CLOSE"}]`
- `107:34505` Action / Open Registry → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`
- `107:34506` Action / Close → `[{"type":"CLOSE"}]`

## ST-179 · State / RowsSubmitted

**Role:** Linked functional state. **Frame:** 107:34507 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-34507).

**Displayed content:**

- Results submitted
- Close
- Two reusable result records were created from EXP-014.
- Result ID
- Source row
- State
- RES-101
- EXP-014 / row 1
- Submitted
- RES-102
- EXP-014 / row 2
- Notebook prose
- Saved separately
- Review
- Ready to start; not yet sent
- Start entry review
- Open notebook

**Authored actions:**

- `107:34511` Action / Close → `[{"type":"CLOSE"}]`
- `107:34535` Action / Start entry review → `[{"type":"NODE","target":"107:34537","navigation":"SWAP"}]`
- `107:34536` Action / Open notebook → `[{"type":"NODE","target":"107:35097","navigation":"SWAP"}]`

## ST-180 · State / ReviewReady

**Role:** Linked functional state. **Frame:** 107:34537 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-34537).

**Displayed content:**

- Send entry for review
- Close
- EXP-014 · required structured results have been submitted.
- Review process
- Expression QC review
- Reviewer
- M. Chen
- Entry version
- EXP-014 / draft snapshot
- Submission note
- Construct and assay results ready for review.
- Send for review
- Cancel

**Authored actions:**

- `107:34541` Action / Close → `[{"type":"CLOSE"}]`
- `107:34562` Action / Send for review → `[{"type":"NODE","target":"107:34564","navigation":"SWAP"}]`
- `107:34563` Action / Cancel → `[{"type":"CLOSE"}]`

## ST-181 · State / ReviewSent

**Role:** Linked functional state. **Frame:** 107:34564 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-34564).

**Displayed content:**

- Entry sent for review
- Close
- EXP-014 · awaiting M. Chen · submission snapshot retained
- Status
- Saved and linked to the originating task
- Provenance
- Input record, parameters and action retained
- Next step
- Open the related record or return to your workspace
- Open submitted entry

**Authored actions:**

- `107:34568` Action / Close → `[{"type":"CLOSE"}]`
- `107:34582` Action / Open submitted entry → `[{"type":"NODE","target":"107:35097","navigation":"SWAP"}]`
- `107:34583` Action / Close → `[{"type":"CLOSE"}]`

## ST-182 · State / SnapshotCompare

**Role:** Linked functional state. **Frame:** 107:34584 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-34584).

**Displayed content:**

- Snapshot versus live plate
- Close
- P-014 · captured experimental context is preserved.
- Well
- Snapshot 14 Nov
- Live inventory
- A1
- 50 µL
- 40 µL
- A2
- A3
- Control C-001
- Keep historical snapshot
- Open live inventory

**Authored actions:**

- `107:34588` Action / Close → `[{"type":"CLOSE"}]`
- `107:34610` Action / Keep historical snapshot → `[{"type":"NODE","target":"107:33579","navigation":"SWAP"}]`
- `107:34611` Action / Open live inventory → `[{"type":"NODE","target":"107:31089","navigation":"NAVIGATE"}]`

## ST-183 · State / MappingValidated

**Role:** Linked functional state. **Frame:** 107:34612 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-34612).

**Displayed content:**

- Mapping correction is valid
- Close
- WF-042 · required sample link now resolves to DNA-001.
- Corrected link
- Upstream record → validation Sample
- Affected task
- WF-042
- Validated record
- DNA-001
- Next state
- Ready to resume supported downstream execution
- Resume downstream task
- Return to workflow

**Authored actions:**

- `107:34616` Action / Close → `[{"type":"CLOSE"}]`
- `107:34633` Action / Resume downstream task → `[{"type":"NODE","target":"107:34635","navigation":"SWAP"}]`
- `107:34634` Action / Return to workflow → `[{"type":"NODE","target":"107:31495","navigation":"NAVIGATE"}]`

## ST-184 · State / TaskRunning

**Role:** Linked functional state. **Frame:** 107:34635 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-34635).

**Displayed content:**

- Task execution in progress
- Close
- WF-043 · sequence validation · inputs locked for this execution
- Current stage
- Validation and processing
- Input state
- Fixed for this execution
- Writeback
- Pending output review
- Status
- In progress · return without losing the run
- View completed result
- Return to task list

**Authored actions:**

- `107:34639` Action / Close → `[{"type":"CLOSE"}]`
- `107:34656` Action / View completed result → `[{"type":"NODE","target":"107:34658","navigation":"SWAP"}]`
- `107:34657` Action / Return to task list → `[{"type":"NODE","target":"107:31495","navigation":"NAVIGATE"}]`

## ST-185 · State / TaskDone

**Role:** Linked functional state. **Frame:** 107:34658 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-34658).

**Displayed content:**

- Task completed
- Close
- WF-043 · sequence QC result linked to EXP-014.
- Status
- Saved and linked to the originating task
- Provenance
- Input record, parameters and action retained
- Next step
- Open the related record or return to your workspace
- Open result
- Return to workflow

**Authored actions:**

- `107:34662` Action / Close → `[{"type":"CLOSE"}]`
- `107:34676` Action / Open result → `[{"type":"NODE","target":"107:34861","navigation":"SWAP"}]`
- `107:34677` Action / Return to workflow → `[{"type":"NODE","target":"107:31495","navigation":"NAVIGATE"}]`

## ST-186 · State / RunRunning

**Role:** Reference or alternate fixture. **Frame:** 107:34678 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-34678).

**Displayed content:**

- Automation run in progress
- Close
- Run R-019 · fixed input and template references
- Current stage
- Validation and processing
- Input state
- Fixed for this execution
- Writeback
- Pending output review
- Status
- In progress · return without losing the run
- Inspect completed output
- Return to Automation

**Authored actions:**

- `107:34682` Action / Close → `[{"type":"CLOSE"}]`
- `107:34699` Action / Inspect completed output → `[{"type":"NODE","target":"107:34701","navigation":"SWAP"}]`
- `107:34700` Action / Return to Automation → `[{"type":"NODE","target":"107:31876","navigation":"NAVIGATE"}]`

## ST-187 · State / RunDone

**Role:** Reference or alternate fixture. **Frame:** 107:34701 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-34701).

**Displayed content:**

- Automation run completed
- Close
- R-019 · 96 result records created · no skipped source rows
- Output
- Count
- Destination
- Result records
- 96
- Expression results
- Skipped source rows
- 0
- —
- Run reference
- R-019
- Input
- assay_results.csv
- Template
- TPL-012 · published configuration
- Open written results
- View activity

**Authored actions:**

- `107:34705` Action / Close → `[{"type":"CLOSE"}]`
- `107:34732` Action / Open written results → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`
- `107:34733` Action / View activity → `[{"type":"NODE","target":"107:33461","navigation":"SWAP"}]`

## ST-188 · State / Published

**Role:** Linked functional state. **Frame:** 107:34734 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-34734).

**Displayed content:**

- Template published
- Close
- TPL-014 · published configuration is now immutable.
- Template ID
- TPL-014
- State
- Published · not directly editable
- Future changes
- Duplicate to a replacement draft
- Existing links
- Retained until deliberately updated
- Create replacement draft
- Open Automation

**Authored actions:**

- `107:34738` Action / Close → `[{"type":"CLOSE"}]`
- `107:34755` Action / Create replacement draft → `[{"type":"NODE","target":"107:34757","navigation":"SWAP"}]`
- `107:34756` Action / Open Automation → `[{"type":"NODE","target":"107:31876","navigation":"NAVIGATE"}]`

## ST-189 · State / ReplacementDraft

**Role:** Linked functional state. **Frame:** 107:34757 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-34757).

**Displayed content:**

- Prepare a replacement draft
- Close
- The published template and linked runs remain unchanged.
- Source template
- TPL-014 · published
- New draft name
- Expression QC replacement
- Existing configurations
- No links redirected automatically
- Create draft
- Cancel

**Authored actions:**

- `107:34761` Action / Close → `[{"type":"CLOSE"}]`
- `107:34778` Action / Create draft → `[{"type":"NODE","target":"107:31876","navigation":"NAVIGATE"}]`
- `107:34779` Action / Cancel → `[{"type":"CLOSE"}]`

## ST-190 · State / RetryReview

**Role:** Reference or alternate fixture. **Frame:** 107:34780 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-34780).

**Displayed content:**

- Review the corrected input
- Close
- Retry is a new run; the failed run remains inspectable.
- Failed run
- R-018 · no records written
- Corrected input
- assay_results_corrected.csv
- Destination
- Expression results
- Duplicate handling
- Validate run IDs before writeback
- Validate corrected input
- Cancel

**Authored actions:**

- `107:34784` Action / Close → `[{"type":"CLOSE"}]`
- `107:34805` Action / Validate corrected input → `[{"type":"NODE","target":"107:33950","navigation":"SWAP"}]`
- `107:34806` Action / Cancel → `[{"type":"CLOSE"}]`

## ST-191 · State / MeasurementReview

**Role:** Linked functional state. **Frame:** 107:34807 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-34807).

**Displayed content:**

- Confirm animal and value
- Close
- Study S-014 · body weight task
- Animal / ear tag
- A-003
- Value and unit
- 23.6 g
- Change from previous
- +0.2 g
- Attribution
- You · scheduled task
- Save measurement
- Edit value

**Authored actions:**

- `107:34811` Action / Close → `[{"type":"CLOSE"}]`
- `107:34832` Action / Save measurement → `[{"type":"NODE","target":"107:34309","navigation":"SWAP"}]`
- `107:34833` Action / Edit value → `[{"type":"NODE","target":"107:34250","navigation":"SWAP"}]`

## ST-192 · State / MeasureNext

**Role:** Linked functional state. **Frame:** 107:34834 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-34834).

**Displayed content:**

- Record next animal
- Close
- Study S-014 · same scheduled body-weight task
- Animal / ear tag
- A-004 · confirm before saving
- Measurement
- Enter value
- Unit
- g
- Previous measurement
- 24.1 g
- Return to schedule
- Cancel

**Authored actions:**

- `107:34838` Action / Close → `[{"type":"CLOSE"}]`
- `107:34859` Action / Return to schedule → `[{"type":"NODE","target":"107:34281","navigation":"SWAP"}]`
- `107:34860` Action / Cancel → `[{"type":"CLOSE"}]`

## ST-193 · State / ProductSaved

**Role:** Linked functional state. **Frame:** 107:34861 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-34861).

**Displayed content:**

- Scientific result saved
- Close
- The result retains its input identity and parameters.
- Status
- Saved and linked to the originating task
- Provenance
- Input record, parameters and action retained
- Next step
- Open the related record or return to your workspace
- Open source record

**Authored actions:**

- `107:34865` Action / Close → `[{"type":"CLOSE"}]`
- `107:34879` Action / Open source record → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`
- `107:34880` Action / Close → `[{"type":"CLOSE"}]`

## ST-194 · State / FeatureSaved

**Role:** Linked functional state. **Frame:** 107:34881 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-34881).

**Displayed content:**

- Annotations updated
- Close
- DNA-001 · a new saved version retains the annotation change.
- Status
- Saved and linked to the originating task
- Provenance
- Input record, parameters and action retained
- Next step
- Open the related record or return to your workspace
- Open features
- View history

**Authored actions:**

- `107:34885` Action / Close → `[{"type":"CLOSE"}]`
- `107:34899` Action / Open features → `[{"type":"NODE","target":"107:29886","navigation":"NAVIGATE"}]`
- `107:34900` Action / View history → `[{"type":"NODE","target":"107:30500","navigation":"NAVIGATE"}]`

## ST-195 · State / VersionRestored

**Role:** Linked functional state. **Frame:** 107:34901 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-34901).

**Displayed content:**

- Version restored
- Close
- DNA-001 · restoration recorded as an identifiable history event.
- Status
- Saved and linked to the originating task
- Provenance
- Input record, parameters and action retained
- Next step
- Open the related record or return to your workspace
- Open current sequence
- View history

**Authored actions:**

- `107:34905` Action / Close → `[{"type":"CLOSE"}]`
- `107:34919` Action / Open current sequence → `[{"type":"NODE","target":"107:29677","navigation":"NAVIGATE"}]`
- `107:34920` Action / View history → `[{"type":"NODE","target":"107:30500","navigation":"NAVIGATE"}]`

## ST-196 · State / TransferSaved

**Role:** Reference or alternate fixture. **Frame:** 107:34921 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-34921).

**Displayed content:**

- Transfer completed
- Close
- 4 physical samples transferred; experimental snapshots unchanged.
- Status
- Saved and linked to the originating task
- Provenance
- Input record, parameters and action retained
- Next step
- Open the related record or return to your workspace
- Open live Inventory
- View audit trail

**Authored actions:**

- `107:34925` Action / Close → `[{"type":"CLOSE"}]`
- `107:34939` Action / Open live Inventory → `[{"type":"NODE","target":"107:31089","navigation":"NAVIGATE"}]`
- `107:34940` Action / View audit trail → `[{"type":"NODE","target":"107:33461","navigation":"SWAP"}]`

## ST-197 · State / InvestigationCreated

**Role:** Linked functional state. **Frame:** 107:34941 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-34941).

**Displayed content:**

- Private investigation created
- Close
- Expression QC · filter context and source references retained.
- Status
- Saved and linked to the originating task
- Provenance
- Input record, parameters and action retained
- Next step
- Open the related record or return to your workspace
- Open source dashboard
- Review access

**Authored actions:**

- `107:34945` Action / Close → `[{"type":"CLOSE"}]`
- `107:34959` Action / Open source dashboard → `[{"type":"NODE","target":"107:31682","navigation":"NAVIGATE"}]`
- `107:34960` Action / Review access → `[{"type":"NODE","target":"107:33890","navigation":"SWAP"}]`

## ST-198 · State / PatternSaved

**Role:** Linked functional state. **Frame:** 107:34961 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-34961).

**Displayed content:**

- Modification pattern applied
- Close
- OL-014 updated after preview; original record state remains identifiable.
- Status
- Saved and linked to the originating task
- Provenance
- Input record, parameters and action retained
- Next step
- Open the related record or return to your workspace
- Open oligo
- View activity

**Authored actions:**

- `107:34965` Action / Close → `[{"type":"CLOSE"}]`
- `107:34979` Action / Open oligo → `[{"type":"NODE","target":"107:32649","navigation":"NAVIGATE"}]`
- `107:34980` Action / View activity → `[{"type":"NODE","target":"107:33461","navigation":"SWAP"}]`

## ST-199 · State / RecipeDraftCreated

**Role:** Linked functional state. **Frame:** 107:34981 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-34981).

**Displayed content:**

- Recipe draft created
- Close
- RCP-014-D3 · effective revision 2 and active runs are unchanged.
- Status
- Saved and linked to the originating task
- Provenance
- Input record, parameters and action retained
- Next step
- Open the related record or return to your workspace
- Open recipe workspace
- Plan a run

**Authored actions:**

- `107:34985` Action / Close → `[{"type":"CLOSE"}]`
- `107:34999` Action / Open recipe workspace → `[{"type":"NODE","target":"107:33033","navigation":"NAVIGATE"}]`
- `107:35000` Action / Plan a run → `[{"type":"NODE","target":"107:26776","navigation":"SWAP"}]`

## ST-200 · State / RecordSaved

**Role:** Reference/superseded example. **Frame:** 107:35001 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-35001).

**Displayed content:**

- Record saved
- Close
- The edit was committed and linked to the active record.
- Status
- Saved and linked to the originating task
- Provenance
- Input record, parameters and action retained
- Next step
- Open the related record or return to your workspace
- Open Registry

**Authored actions:**

- `107:35005` Action / Close → `[{"type":"CLOSE"}]`
- `107:35019` Action / Open Registry → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`
- `107:35020` Action / Close → `[{"type":"CLOSE"}]`

## ST-201 · State / ProteinResult

**Role:** Linked functional state. **Frame:** 107:35021 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-35021).

**Displayed content:**

- Protein analysis result
- Close
- Input HC-014 · 450 amino acids · parameters retained.
- Length
- 450 aa
- Molecular mass
- 49.5 kDa · sequence-derived estimate
- Input
- HC-014 · selected chain
- Analysis metadata
- Method and input reference retained
- Save analysis
- Review settings

**Authored actions:**

- `107:35025` Action / Close → `[{"type":"CLOSE"}]`
- `107:35042` Action / Save analysis → `[{"type":"NODE","target":"107:34861","navigation":"SWAP"}]`
- `107:35043` Action / Review settings → `[{"type":"NODE","target":"107:26168","navigation":"SWAP"}]`

## ST-202 · State / BlastResult

**Role:** Linked functional state. **Frame:** 107:35044 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-35044).

**Displayed content:**

- BLAST results
- Close
- Query DNA-001 / v7 · selected range 97–116
- Match
- Identity
- Coverage
- Reference construct A
- 100%
- 20 / 20 nt
- Reference construct B
- 95%
- Query
- 97–116 · DNA-001 / v7
- Database
- Selected reference collection
- Save result
- Change query

**Authored actions:**

- `107:35048` Action / Close → `[{"type":"CLOSE"}]`
- `107:35072` Action / Save result → `[{"type":"NODE","target":"107:34861","navigation":"SWAP"}]`
- `107:35073` Action / Change query → `[{"type":"NODE","target":"107:25595","navigation":"SWAP"}]`

## ST-203 · State / CodonResult

**Role:** Linked functional state. **Frame:** 107:35074 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-35074).

**Displayed content:**

- Codon optimization preview
- Close
- DNA-001 / v7 · review the proposed sequence before saving.
- Target system
- E. coli
- Input reference
- DNA-001 / v7 · selected CDS
- Proposed sequence
- Separate candidate; source remains unchanged
- Review required
- Sequence differences and constraints
- Save as new sequence
- Review parameters

**Authored actions:**

- `107:35078` Action / Close → `[{"type":"CLOSE"}]`
- `107:35095` Action / Save as new sequence → `[{"type":"NODE","target":"107:34861","navigation":"SWAP"}]`
- `107:35096` Action / Review parameters → `[{"type":"NODE","target":"107:25557","navigation":"SWAP"}]`

## ST-204 · State / NotebookSubmitted

**Role:** Linked functional state. **Frame:** 107:35097 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-35097).

**Displayed content:**

- Submitted notebook results
- Close
- EXP-014 · two structured result records are linked to this entry.
- Sample
- Result
- State
- S-001
- RES-101 · 1280 a.u.
- Submitted
- S-002
- RES-102 · 1315 a.u.
- Entry
- EXP-014 · prose saved
- Review state
- See the review submission history
- Open entry workspace
- Review status

**Authored actions:**

- `107:35101` Action / Close → `[{"type":"CLOSE"}]`
- `107:35125` Action / Open entry workspace → `[{"type":"NODE","target":"107:30702","navigation":"NAVIGATE"}]`
- `107:35126` Action / Review status → `[{"type":"NODE","target":"107:34564","navigation":"SWAP"}]`

## ST-205 · State / modules

**Role:** Reference or alternate fixture. **Frame:** 107:35127 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-35127).

**Displayed content:**

- All modules
- Close
- Open a workspace or a view of the active sequence.
- Home
- Map
- Sequence
- Features
- Primers
- Enzymes
- History
- Notebook
- Registry
- Inventory
- Workflows
- Insights
- Automation
- Split
- Biologics
- Chemistry
- RNA / Oligos
- In Vivo
- Bioprocess

**Authored actions:**

- `107:35131` Action / Close → `[{"type":"CLOSE"}]`
- `107:35135` Action / Home → `[{"type":"NODE","target":"107:29237","navigation":"NAVIGATE"}]`
- `107:35136` Action / Map → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`
- `107:35137` Action / Sequence → `[{"type":"NODE","target":"107:29677","navigation":"NAVIGATE"}]`
- `107:35139` Action / Features → `[{"type":"NODE","target":"107:29886","navigation":"NAVIGATE"}]`
- `107:35140` Action / Primers → `[{"type":"NODE","target":"107:30093","navigation":"NAVIGATE"}]`
- `107:35141` Action / Enzymes → `[{"type":"NODE","target":"107:30295","navigation":"NAVIGATE"}]`
- `107:35143` Action / History → `[{"type":"NODE","target":"107:30500","navigation":"NAVIGATE"}]`
- `107:35144` Action / Notebook → `[{"type":"NODE","target":"107:30702","navigation":"NAVIGATE"}]`
- `107:35145` Action / Registry → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`
- `107:35147` Action / Inventory → `[{"type":"NODE","target":"107:31089","navigation":"NAVIGATE"}]`
- `107:35148` Action / Workflows → `[{"type":"NODE","target":"107:31495","navigation":"NAVIGATE"}]`
- `107:35149` Action / Insights → `[{"type":"NODE","target":"107:31682","navigation":"NAVIGATE"}]`
- `107:35151` Action / Automation → `[{"type":"NODE","target":"107:31876","navigation":"NAVIGATE"}]`
- `107:35152` Action / Split → `[{"type":"NODE","target":"107:32075","navigation":"NAVIGATE"}]`
- `107:35153` Action / Biologics → `[{"type":"NODE","target":"107:32270","navigation":"NAVIGATE"}]`
- `107:35155` Action / Chemistry → `[{"type":"NODE","target":"107:32457","navigation":"NAVIGATE"}]`
- `107:35156` Action / RNA / Oligos → `[{"type":"NODE","target":"107:32649","navigation":"NAVIGATE"}]`
- `107:35157` Action / In Vivo → `[{"type":"NODE","target":"107:32841","navigation":"NAVIGATE"}]`
- `107:35159` Action / Bioprocess → `[{"type":"NODE","target":"107:33033","navigation":"NAVIGATE"}]`
- `107:35162` Action / Close → `[{"type":"CLOSE"}]`

## ST-206 · Context overlay / Notebook

**Role:** Reference or alternate fixture. **Frame:** 107:35163 · 320 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-35163).

**Displayed content:**

- Details & tools
- All notebook tools
- Current selection
- Expression validation
- Entry
- EXP-014 · draft
- Save state
- Text saved · 2 unsubmitted rows
- Review
- Not submitted for review
- Actions
- Insert result table
- Submit 2 rows
- Send for review
- Related records
- DNA-001 · Entity
- P-014 · Plate snapshot

**Authored actions:**

- `107:35177` Action / All notebook tools → `[{"type":"NODE","target":"107:28334","navigation":"OVERLAY"}]`
- `107:35191` Action / Insert result table → `[{"type":"NODE","target":"107:33489","navigation":"OVERLAY"}]`
- `107:35192` Action / Submit 2 rows → `[{"type":"NODE","target":"107:33516","navigation":"OVERLAY"}]`
- `107:35193` Action / Send for review → `[{"type":"NODE","target":"107:33554","navigation":"OVERLAY"}]`
- `107:35196` Related / DNA-001 · Entity → `[{"type":"NODE","target":"107:33242","navigation":"OVERLAY"}]`
- `107:35201` Related / P-014 · Plate snapshot → `[{"type":"NODE","target":"107:33579","navigation":"OVERLAY"}]`

## ST-207 · Responsive / Notebook / 1280

**Role:** Responsive/context variant. **Frame:** 107:35206 · 1280 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-35206).

**Displayed content:**

- Notebook
- Capture structured data and submit reusable results.
- Search
- New
- Tools
- All tools
- Insert result table
- Submit 2 rows
- Send for review
- Expression cassette validation
- Draft
- EXP-014 · Expression study · text saved at 10:36
- Notes
- Results
- Review
- Activity
- Purpose
- Verify the expression construct and record assay results.
- DNA-001 · construct
- P-014 · snapshot
- Assay results
- Sample
- Signal (a.u.)
- Result state
- S-001
- 1280
- Unsubmitted
- S-002
- 1315
- Submitting this table creates reusable results linked to this entry.
- Text saved. Two result rows are still unsubmitted; entry review has not started.

**Authored actions:**

- `107:35313` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:35314` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `107:35315` Action / Tools → `[{"type":"NODE","target":"107:35163","navigation":"OVERLAY"}]`
- `107:35316` Action / All tools → `[{"type":"NODE","target":"107:28334","navigation":"OVERLAY"}]`
- `107:35319` Action / Insert result table → `[{"type":"NODE","target":"107:33489","navigation":"OVERLAY"}]`
- `107:35320` Action / Submit 2 rows → `[{"type":"NODE","target":"107:33516","navigation":"OVERLAY"}]`
- `107:35321` Action / Send for review → `[{"type":"NODE","target":"107:33554","navigation":"OVERLAY"}]`
- `107:35331` Action / Results → `[{"type":"NODE","target":"107:33516","navigation":"OVERLAY"}]`
- `107:35332` Action / Review → `[{"type":"NODE","target":"107:33554","navigation":"OVERLAY"}]`
- `107:35333` Action / Activity → `[{"type":"NODE","target":"107:33461","navigation":"OVERLAY"}]`
- `107:35338` Action / DNA-001 · construct → `[{"type":"NODE","target":"107:33242","navigation":"OVERLAY"}]`
- `107:35339` Action / P-014 · snapshot → `[{"type":"NODE","target":"107:33579","navigation":"OVERLAY"}]`
- `107:35346` Record / S-001 → `[{"type":"NODE","target":"107:33516","navigation":"OVERLAY"}]`
- `107:35350` Record / S-002 → `[{"type":"NODE","target":"107:33516","navigation":"OVERLAY"}]`

## ST-208 · Responsive / Notebook / 1440

**Role:** Responsive/context variant. **Frame:** 107:35360 · 1440 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-35360).

**Displayed content:**

- Notebook
- Capture structured data and submit reusable results.
- Search
- New
- Tools
- All tools
- Insert result table
- Submit 2 rows
- Send for review
- Expression cassette validation
- Draft
- EXP-014 · Expression study · text saved at 10:36
- Notes
- Results
- Review
- Activity
- Purpose
- Verify the expression construct and record assay results.
- DNA-001 · construct
- P-014 · snapshot
- Assay results
- Sample
- Signal (a.u.)
- Result state
- S-001
- 1280
- Unsubmitted
- S-002
- 1315
- Submitting this table creates reusable results linked to this entry.
- Text saved. Two result rows are still unsubmitted; entry review has not started.

**Authored actions:**

- `107:35467` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:35468` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `107:35469` Action / Tools → `[{"type":"NODE","target":"107:35163","navigation":"OVERLAY"}]`
- `107:35470` Action / All tools → `[{"type":"NODE","target":"107:28334","navigation":"OVERLAY"}]`
- `107:35473` Action / Insert result table → `[{"type":"NODE","target":"107:33489","navigation":"OVERLAY"}]`
- `107:35474` Action / Submit 2 rows → `[{"type":"NODE","target":"107:33516","navigation":"OVERLAY"}]`
- `107:35475` Action / Send for review → `[{"type":"NODE","target":"107:33554","navigation":"OVERLAY"}]`
- `107:35485` Action / Results → `[{"type":"NODE","target":"107:33516","navigation":"OVERLAY"}]`
- `107:35486` Action / Review → `[{"type":"NODE","target":"107:33554","navigation":"OVERLAY"}]`
- `107:35487` Action / Activity → `[{"type":"NODE","target":"107:33461","navigation":"OVERLAY"}]`
- `107:35492` Action / DNA-001 · construct → `[{"type":"NODE","target":"107:33242","navigation":"OVERLAY"}]`
- `107:35493` Action / P-014 · snapshot → `[{"type":"NODE","target":"107:33579","navigation":"OVERLAY"}]`
- `107:35500` Record / S-001 → `[{"type":"NODE","target":"107:33516","navigation":"OVERLAY"}]`
- `107:35504` Record / S-002 → `[{"type":"NODE","target":"107:33516","navigation":"OVERLAY"}]`

## ST-209 · Context overlay / Map

**Role:** Reference or alternate fixture. **Frame:** 107:35514 · 320 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-35514).

**Displayed content:**

- Details & tools
- All map tools
- Current selection
- T7 promoter · 97–116
- Record
- DNA-001 · version 7
- Selected range
- 97–116 · forward strand
- Type
- Promoter · 20 bp
- Actions
- Edit feature
- Add primer from range
- Design primer pair
- Related records
- DNA-001 · Registry
- Plate P-014 · Inventory

**Authored actions:**

- `107:35528` Action / All map tools → `[{"type":"NODE","target":"110:22016","navigation":"NAVIGATE"}]`
- `107:35542` Action / Edit feature → `[{"type":"NODE","target":"107:25134","navigation":"OVERLAY"}]`
- `107:35543` Action / Add primer from range → `[{"type":"NODE","target":"107:25182","navigation":"OVERLAY"}]`
- `107:35544` Action / Design primer pair → `[{"type":"NODE","target":"107:25303","navigation":"OVERLAY"}]`
- `107:35547` Related / DNA-001 · Registry → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`
- `107:35552` Related / Plate P-014 · Inventory → `[{"type":"NODE","target":"107:31089","navigation":"NAVIGATE"}]`

## ST-210 · Responsive / Map / 1280

**Role:** Responsive/context variant. **Frame:** 107:35557 · 1280 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-35557).

**Displayed content:**

- Map
- Inspect a construct and act on the selected region.
- Search
- New
- Tools
- All tools
- pHelixBench-Demo
- v7 · saved 10:42
- Undo
- Save version
- Export
- Sequence
- Features
- Primers
- Enzymes
- History
- Add feature
- Analyze
- Display layers
- Circular map
- Selection 97–116 · forward
- 3021 bp · circular
- GC 54.5%
- drag to select
- T7 promoter
- lac operator
- RBS
- demo_cds
- Demo-Rev
- rrnB T1 terminator
- M13 fwd site
- pUC19 MCS
- M13 rev site
- ori (demo)
- demo_marker
- loxP
- FRT
- Kozak
- EcoRI
- BamHI
- ScaI
- AhdI
- SexAI
- PshAI
- DraI
- NcoI
- T7 promoter selected. Create and analysis tools retain this document and range.

**Authored actions:**

- `107:35664` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:35665` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `107:35666` Action / Tools → `[{"type":"NODE","target":"107:35514","navigation":"OVERLAY"}]`
- `107:35667` Action / All tools → `[{"type":"NODE","target":"110:22016","navigation":"NAVIGATE"}]`
- `107:35672` Action / Undo → `[{"type":"NODE","target":"107:29125","navigation":"OVERLAY"}]`
- `107:35673` Action / Save version → `[{"type":"NODE","target":"107:29211","navigation":"OVERLAY"}]`
- `107:35674` Action / Export → `[{"type":"NODE","target":"107:27142","navigation":"OVERLAY"}]`
- `107:35677` Action / Sequence → `[{"type":"NODE","target":"107:29677","navigation":"NAVIGATE"}]`
- `107:35678` Action / Features → `[{"type":"NODE","target":"107:29886","navigation":"NAVIGATE"}]`
- `107:35679` Action / Primers → `[{"type":"NODE","target":"107:30093","navigation":"NAVIGATE"}]`
- `107:35680` Action / Enzymes → `[{"type":"NODE","target":"107:30295","navigation":"NAVIGATE"}]`
- `107:35681` Action / History → `[{"type":"NODE","target":"107:30500","navigation":"NAVIGATE"}]`
- `107:35684` Action / Add feature → `[{"type":"NODE","target":"107:25134","navigation":"OVERLAY"}]`
- `107:35685` Action / Analyze → `[{"type":"NODE","target":"107:28930","navigation":"OVERLAY"}]`
- `107:35686` Action / Display layers → `[{"type":"NODE","target":"107:25217","navigation":"OVERLAY"}]`

## ST-211 · Responsive / Map / 1440

**Role:** Responsive/context variant. **Frame:** 107:35764 · 1440 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-35764).

**Displayed content:**

- Map
- Inspect a construct and act on the selected region.
- Search
- New
- Tools
- All tools
- pHelixBench-Demo
- v7 · saved 10:42
- Undo
- Save version
- Export
- Sequence
- Features
- Primers
- Enzymes
- History
- Add feature
- Analyze
- Display layers
- Circular map
- Selection 97–116 · forward
- 3021 bp · circular
- GC 54.5%
- drag to select
- T7 promoter
- lac operator
- RBS
- demo_cds
- Demo-Rev
- rrnB T1 terminator
- M13 fwd site
- pUC19 MCS
- M13 rev site
- ori (demo)
- demo_marker
- loxP
- FRT
- Kozak
- EcoRI
- BamHI
- ScaI
- AhdI
- SexAI
- PshAI
- DraI
- NcoI
- T7 promoter selected. Create and analysis tools retain this document and range.

**Authored actions:**

- `107:35871` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:35872` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `107:35873` Action / Tools → `[{"type":"NODE","target":"107:35514","navigation":"OVERLAY"}]`
- `107:35874` Action / All tools → `[{"type":"NODE","target":"110:22016","navigation":"NAVIGATE"}]`
- `107:35879` Action / Undo → `[{"type":"NODE","target":"107:29125","navigation":"OVERLAY"}]`
- `107:35880` Action / Save version → `[{"type":"NODE","target":"107:29211","navigation":"OVERLAY"}]`
- `107:35881` Action / Export → `[{"type":"NODE","target":"107:27142","navigation":"OVERLAY"}]`
- `107:35884` Action / Sequence → `[{"type":"NODE","target":"107:29677","navigation":"NAVIGATE"}]`
- `107:35885` Action / Features → `[{"type":"NODE","target":"107:29886","navigation":"NAVIGATE"}]`
- `107:35886` Action / Primers → `[{"type":"NODE","target":"107:30093","navigation":"NAVIGATE"}]`
- `107:35887` Action / Enzymes → `[{"type":"NODE","target":"107:30295","navigation":"NAVIGATE"}]`
- `107:35888` Action / History → `[{"type":"NODE","target":"107:30500","navigation":"NAVIGATE"}]`
- `107:35891` Action / Add feature → `[{"type":"NODE","target":"107:25134","navigation":"OVERLAY"}]`
- `107:35892` Action / Analyze → `[{"type":"NODE","target":"107:28930","navigation":"OVERLAY"}]`
- `107:35893` Action / Display layers → `[{"type":"NODE","target":"107:25217","navigation":"OVERLAY"}]`

## ST-212 · Context overlay / Inventory

**Role:** Reference or alternate fixture. **Frame:** 107:35971 · 320 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-35971).

**Displayed content:**

- Details & tools
- All inventory tools
- Current selection
- P-014 · 4 wells selected
- Record type
- Live plate · 96 wells
- Location
- Freezer A / Shelf 2
- Selection
- A1–A4 · 4 wells
- Actions
- Assign contents
- Transfer selected
- Open experiment snapshot
- Related records
- DNA-001 · Entity
- EXP-014 · Notebook

**Authored actions:**

- `107:35985` Action / All inventory tools → `[{"type":"NODE","target":"107:28459","navigation":"OVERLAY"}]`
- `107:35999` Action / Assign contents → `[{"type":"NODE","target":"107:26858","navigation":"OVERLAY"}]`
- `107:36000` Action / Transfer selected → `[{"type":"NODE","target":"107:33703","navigation":"OVERLAY"}]`
- `107:36001` Action / Open experiment snapshot → `[{"type":"NODE","target":"107:33579","navigation":"OVERLAY"}]`
- `107:36004` Related / DNA-001 · Entity → `[{"type":"NODE","target":"107:33242","navigation":"OVERLAY"}]`
- `107:36009` Related / EXP-014 · Notebook → `[{"type":"NODE","target":"107:30702","navigation":"NAVIGATE"}]`

## ST-213 · Responsive / Inventory / 1280

**Role:** Responsive/context variant. **Frame:** 107:36014 · 1280 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-36014).

**Displayed content:**

- Inventory
- Locate physical samples and review changes before transfer.
- Search
- New
- Tools
- All tools
- Add container
- Transfer selected
- Scan / paste barcode
- Storage
- Freezer A
-   Shelf 2
-     Rack R-01
- P-014 · live plate
- 96 wells · 4 occupied
- Find entity samples
- Plate P-014
- Live inventory
- A1–A4 selected · 4 wells · Freezer A / Shelf 2
- 
- 1
- 2
- 3
- 4
- 5
- 6
- 7
- 8
- 9
- 10
- 11
- 12
- A
- S1
- S2
- S3
- S4
- —
- B
- C
- D
- E
- F
- G
- H
- Assign contents
- Set role / concentration
- Live changes do not modify the plate snapshot already attached to EXP-014.

**Authored actions:**

- `107:36121` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:36122` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `107:36123` Action / Tools → `[{"type":"NODE","target":"107:35971","navigation":"OVERLAY"}]`
- `107:36124` Action / All tools → `[{"type":"NODE","target":"107:28459","navigation":"OVERLAY"}]`
- `107:36127` Action / Add container → `[{"type":"NODE","target":"107:27008","navigation":"OVERLAY"}]`
- `107:36128` Action / Transfer selected → `[{"type":"NODE","target":"107:33703","navigation":"OVERLAY"}]`
- `107:36129` Action / Scan / paste barcode → `[{"type":"NODE","target":"107:33742","navigation":"OVERLAY"}]`
- `107:36140` Action / Find entity samples → `[{"type":"NODE","target":"107:33618","navigation":"OVERLAY"}]`
- `107:36165` Well / A1 → `[{"type":"NODE","target":"107:26858","navigation":"OVERLAY"}]`
- `107:36369` Well / H12 → `[{"type":"NODE","target":"107:26858","navigation":"OVERLAY"}]`
- `107:36372` Action / Assign contents → `[{"type":"NODE","target":"107:26858","navigation":"OVERLAY"}]`
- `107:36373` Action / Set role / concentration → `[{"type":"NODE","target":"107:25650","navigation":"OVERLAY"}]`

Well controls: 96. A1/H12 are representative authored action examples; implementation must generate identity-bound controls for every actual well.

## ST-214 · Responsive / Inventory / 1440

**Role:** Responsive/context variant. **Frame:** 107:36379 · 1440 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-36379).

**Displayed content:**

- Inventory
- Locate physical samples and review changes before transfer.
- Search
- New
- Tools
- All tools
- Add container
- Transfer selected
- Scan / paste barcode
- Storage
- Freezer A
-   Shelf 2
-     Rack R-01
- P-014 · live plate
- 96 wells · 4 occupied
- Find entity samples
- Plate P-014
- Live inventory
- A1–A4 selected · 4 wells · Freezer A / Shelf 2
- 
- 1
- 2
- 3
- 4
- 5
- 6
- 7
- 8
- 9
- 10
- 11
- 12
- A
- S1
- S2
- S3
- S4
- —
- B
- C
- D
- E
- F
- G
- H
- Assign contents
- Set role / concentration
- Live changes do not modify the plate snapshot already attached to EXP-014.

**Authored actions:**

- `107:36486` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:36487` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `107:36488` Action / Tools → `[{"type":"NODE","target":"107:35971","navigation":"OVERLAY"}]`
- `107:36489` Action / All tools → `[{"type":"NODE","target":"107:28459","navigation":"OVERLAY"}]`
- `107:36492` Action / Add container → `[{"type":"NODE","target":"107:27008","navigation":"OVERLAY"}]`
- `107:36493` Action / Transfer selected → `[{"type":"NODE","target":"107:33703","navigation":"OVERLAY"}]`
- `107:36494` Action / Scan / paste barcode → `[{"type":"NODE","target":"107:33742","navigation":"OVERLAY"}]`
- `107:36505` Action / Find entity samples → `[{"type":"NODE","target":"107:33618","navigation":"OVERLAY"}]`
- `107:36530` Well / A1 → `[{"type":"NODE","target":"107:26858","navigation":"OVERLAY"}]`
- `107:36734` Well / H12 → `[{"type":"NODE","target":"107:26858","navigation":"OVERLAY"}]`
- `107:36737` Action / Assign contents → `[{"type":"NODE","target":"107:26858","navigation":"OVERLAY"}]`
- `107:36738` Action / Set role / concentration → `[{"type":"NODE","target":"107:25650","navigation":"OVERLAY"}]`

Well controls: 96. A1/H12 are representative authored action examples; implementation must generate identity-bound controls for every actual well.

## ST-215 · Reference / Previous v3 review index

**Role:** Reference/superseded example. **Frame:** 107:36744 · 1600 × 560. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-36744).

**Displayed content:**

- HelixBench · Soft Gallery UX v3
- A connected scientific workspace with contextual tools and explicit task outcomes.
- Start at Home
- Notebook submission
- Inventory & snapshots
- Automation lifecycle
- 19 main screens · 79 task states · feature forms and catalogs · 1280 / 1440 / 1600 examples
- Prototype data and scientific results are illustrative. Controls demonstrate navigation and reviewed states; they do not execute calculations or write application records.
- Research decisions: draft versus submission · scientific identity versus custody · immutable experiment snapshots · mapping repair · explicit publication · eligible oligo modification
- Evidence: 178 supplied assets and the 14 November 2026 research document. Benchling-specific lifecycle details informed the proposal; HelixBench capabilities require implementation validation.
- No matching results
- Query failure
- Restricted record
- Run planning
- Empty states
- Feedback states

**Authored actions:**

- `107:36748` Action / Start at Home → `[{"type":"NODE","target":"107:29237","navigation":"NAVIGATE"}]`
- `107:36749` Action / Notebook submission → `[{"type":"NODE","target":"107:30702","navigation":"NAVIGATE"}]`
- `107:36750` Action / Inventory & snapshots → `[{"type":"NODE","target":"107:31089","navigation":"NAVIGATE"}]`
- `107:36751` Action / Automation lifecycle → `[{"type":"NODE","target":"107:31876","navigation":"NAVIGATE"}]`
- `107:36757` Action / No matching results → `[{"type":"NODE","target":"107:36814","navigation":"OVERLAY"}]`
- `107:36758` Action / Query failure → `[{"type":"NODE","target":"107:36841","navigation":"OVERLAY"}]`
- `107:36759` Action / Restricted record → `[{"type":"NODE","target":"107:36868","navigation":"OVERLAY"}]`
- `107:36760` Action / Run planning → `[{"type":"NODE","target":"107:36763","navigation":"OVERLAY"}]`
- `107:36761` Action / Empty states → `[{"type":"NODE","target":"107:39945","navigation":"NAVIGATE"}]`
- `107:36762` Action / Feedback states → `[{"type":"NODE","target":"107:41512","navigation":"NAVIGATE"}]`

## ST-216 · State / PlanRunReview

**Role:** Linked functional state. **Frame:** 107:36763 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-36763).

**Displayed content:**

- Review bioprocess run plan
- Close
- Plan from a named effective recipe; no runs created yet.
- Recipe reference
- RCP-014 / revision 2 · effective
- Study
- BP-014
- Conditions
- A: pH 7.0 · B: pH 7.2
- Replicates
- 2 per condition · 4 planned runs
- Output destination
- BP-014 / planned runs
- Create 4 planned runs
- Cancel

**Authored actions:**

- `107:36767` Action / Close → `[{"type":"CLOSE"}]`
- `107:36792` Action / Create 4 planned runs → `[{"type":"NODE","target":"107:36794","navigation":"SWAP"}]`
- `107:36793` Action / Cancel → `[{"type":"CLOSE"}]`

## ST-217 · State / RunsPlanned

**Role:** Linked functional state. **Frame:** 107:36794 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-36794).

**Displayed content:**

- Bioprocess runs planned
- Close
- 4 planned runs retain the effective recipe reference.
- Recipe reference
- RCP-014 / revision 2
- Planned runs
- BP-014-A1, A2, B1, B2
- Execution state
- Planned; no measurements captured yet
- Open Bioprocess

**Authored actions:**

- `107:36798` Action / Close → `[{"type":"CLOSE"}]`
- `107:36812` Action / Open Bioprocess → `[{"type":"NODE","target":"107:33033","navigation":"NAVIGATE"}]`
- `107:36813` Action / Close → `[{"type":"CLOSE"}]`

## ST-218 · State / NoMatchingResults

**Role:** Reference or alternate fixture. **Frame:** 107:36814 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-36814).

**Displayed content:**

- No results match these filters
- Close
- The query completed successfully; no matching accessible rows were returned.
- Project filter
- Expression study
- Date range
- Last 24 hours
- Data scope
- Your accessible records
- Last query
- Completed · today at 10:38
- Change filters
- Return to dashboard

**Authored actions:**

- `107:36818` Action / Close → `[{"type":"CLOSE"}]`
- `107:36839` Action / Change filters → `[{"type":"NODE","target":"107:33922","navigation":"SWAP"}]`
- `107:36840` Action / Return to dashboard → `[{"type":"NODE","target":"107:31682","navigation":"NAVIGATE"}]`

## ST-219 · State / QueryFailed

**Role:** Reference or alternate fixture. **Frame:** 107:36841 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-36841).

**Displayed content:**

- The dashboard query failed
- Close
- A failed query does not mean there are no experimental results.
- Source block
- Expression QC / mean signal
- Issue
- Signal field type does not match the saved query
- Retained state
- Filters and previous successful result remain available
- Recovery
- Review query fields before running again
- Edit query
- Return to dashboard

**Authored actions:**

- `107:36845` Action / Close → `[{"type":"CLOSE"}]`
- `107:36866` Action / Edit query → `[{"type":"NODE","target":"107:26918","navigation":"SWAP"}]`
- `107:36867` Action / Return to dashboard → `[{"type":"NODE","target":"107:31682","navigation":"NAVIGATE"}]`

## ST-220 · State / RestrictedRecord

**Role:** Reference or alternate fixture. **Frame:** 107:36868 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-36868).

**Displayed content:**

- This record is unavailable
- Close
- The linked record cannot be opened with your current access.
- Record reference
- Linked entity · details withheld
- Known state
- You do not have permission to view this record
- Current workspace
- Your accessible records remain available
- Return to Registry

**Authored actions:**

- `107:36872` Action / Close → `[{"type":"CLOSE"}]`
- `107:36889` Action / Return to Registry → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`
- `107:36890` Action / Close → `[{"type":"CLOSE"}]`

## ST-221 · State / PrimerCandidates

**Role:** Linked functional state. **Frame:** 107:36891 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-36891).

**Displayed content:**

- Review primer candidates
- Close
- Input DNA-001 / v7 · target 97–351 · no primers saved yet.
- Candidate pair
- T7-F / Demo-R
- Target product
- 255 bp
- Review
- Binding positions and pair linkage
- Output
- Create a linked primer pair
- Add selected pair
- Back to design

**Authored actions:**

- `107:36895` Action / Close → `[{"type":"CLOSE"}]`
- `107:36916` Action / Add selected pair → `[{"type":"NODE","target":"128:8054","navigation":"NAVIGATE"}]`
- `107:36917` Action / Back to design → `[{"type":"NODE","target":"107:25303","navigation":"SWAP"}]`

## ST-222 · State / AssemblyPreview

**Role:** Linked functional state. **Frame:** 107:36918 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-36918).

**Displayed content:**

- Review the proposed construct
- Close
- Gibson assembly · source vector and insert remain unchanged.
- Vector
- DNA-001 / v7
- Insert
- Expression insert
- Proposed construct
- Assembled construct · new record
- Review
- Junctions, orientation and feature propagation
- Create construct
- Back to setup

**Authored actions:**

- `107:36922` Action / Close → `[{"type":"CLOSE"}]`
- `107:36943` Action / Create construct → `[{"type":"NODE","target":"107:34861","navigation":"SWAP"}]`
- `107:36944` Action / Back to setup → `[{"type":"NODE","target":"107:25463","navigation":"SWAP"}]`

## ST-223 · State / GuideCandidates

**Role:** Linked functional state. **Frame:** 107:36945 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-36945).

**Displayed content:**

- Review CRISPR candidates
- Close
- DNA-001 · selected target region · candidates are not saved.
- Candidate
- Guide G-01 · forward strand
- PAM
- NGG
- Review scope
- Target position, score method and off-target limits
- Output
- Create an individually identified guide record
- Save selected guide
- Back to setup

**Authored actions:**

- `107:36949` Action / Close → `[{"type":"CLOSE"}]`
- `107:36970` Action / Save selected guide → `[{"type":"NODE","target":"107:34861","navigation":"SWAP"}]`
- `107:36971` Action / Back to setup → `[{"type":"NODE","target":"107:25505","navigation":"SWAP"}]`

## ST-224 · State / NotebookAddSection

**Role:** Reference or alternate fixture. **Frame:** 107:36972 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-36972).

**Displayed content:**

- Add a notebook section
- Close
- EXP-014 · prose editing remains separate from result submission.
- Section title
- Assay observations
- Section date
- 15 November 2026
- Placement
- After Assay results
- Add section
- Cancel

**Authored actions:**

- `107:36976` Action / Close → `[{"type":"CLOSE"}]`
- `107:36993` Action / Add section → `[{"type":"NODE","target":"107:30702","navigation":"NAVIGATE"}]`
- `107:36994` Action / Cancel → `[{"type":"CLOSE"}]`

## ST-225 · Cave art / Empty-state illustration library

**Role:** Reference/superseded example. **Frame:** 107:36995 · 1600 × 850. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-36995).

**Displayed content:**

- Empty states · cave painting illustrations
- Charcoal silhouettes, ochre accents and quiet limestone shapes. Artwork is decorative; the adjacent text carries the meaning.
- First mark
- Gather & organize
- Follow the tracks
- Connect the work
- Observe nature
- Find a way forward

**Authored actions:**

- No clickable transition captured in this frame.

## ST-226 · Empty / Home

**Role:** Linked functional state. **Frame:** 107:37024 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-37024).

**Displayed content:**

- Home
- Search
- Start with your sequence
- Open your file, understand the input, and run your first analysis.
- Import a sequence
- Open GenBank, FASTA or a HelixBench document. Review the recognized sequence before opening it.
- Import sequence
- Try a sample
- No sequence ready? A sample is labeled and kept separate from your own files.
- A short path to your first result
- Review input → inspect sequence → run analysis → save and find it again.
- Create a blank sequence
- New notebook entry
- You can explore the product without creating inventory records or setting up a notebook.
- Your first task
- Import your own sequence, or
explore a labeled sample. You can
skip guidance and return to it
later.
- Task help

**Authored actions:**

- `107:37125` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `110:19622` Action / Import sequence → `[{"type":"NODE","target":"110:19645","navigation":"NAVIGATE"}]`
- `110:19624` Action / Try a sample → `[{"type":"NODE","target":"110:19881","navigation":"NAVIGATE"}]`
- `110:19631` Action / Create a blank sequence → `[{"type":"NODE","target":"110:20202","navigation":"NAVIGATE"}]`
- `110:19633` Action / New notebook entry → `[{"type":"NODE","target":"110:20232","navigation":"NAVIGATE"}]`
- `110:19639` Action / Import sequence → `[{"type":"NODE","target":"110:19645","navigation":"NAVIGATE"}]`
- `110:19641` Action / Try a sample → `[{"type":"NODE","target":"110:19881","navigation":"NAVIGATE"}]`
- `110:19643` Action / Task help → `[{"type":"NODE","target":"110:21252","navigation":"NAVIGATE"}]`

## ST-227 · Empty / Map

**Role:** Reference or alternate fixture. **Frame:** 107:37144 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-37144).

**Displayed content:**

- Map
- Search
- No sequence open
- Create a sequence or open a saved record to inspect its map, annotations and
cut sites.
- Create a sequence
- Open a sequence
- Analysis tools become available after you open a sequence.
- Details & tools
- All map tools
- No sequence selected
- Open or create a sequence to inspect
its details.
- Start here
- Tools that need a record require you to
create, open or select one first.

**Authored actions:**

- `107:37245` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:37251` Action / Create a sequence → `[{"type":"NODE","target":"110:20202","navigation":"NAVIGATE"}]`
- `107:37252` Action / Open a sequence → `[{"type":"NODE","target":"110:19645","navigation":"NAVIGATE"}]`
- `107:37257` Action / All map tools → `[{"type":"NODE","target":"110:22016","navigation":"NAVIGATE"}]`
- `107:37261` Action / Create a sequence → `[{"type":"NODE","target":"110:20202","navigation":"NAVIGATE"}]`
- `107:37262` Action / Open a sequence → `[{"type":"NODE","target":"110:19645","navigation":"NAVIGATE"}]`

## ST-228 · Empty / Sequence

**Role:** Reference or alternate fixture. **Frame:** 107:37264 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-37264).

**Displayed content:**

- Sequence
- Search
- Add a sequence to inspect its bases
- Paste a sequence, import a supported file, or open a saved record to begin.
- Create a sequence
- Open a sequence
- Your sequence and selected range will stay linked across views.
- Details & tools
- All sequence tools
- No sequence selected
- Sequence tools need an open document.
- Start here
- Tools that need a record require you to
create, open or select one first.

**Authored actions:**

- `107:37365` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:37371` Action / Create a sequence → `[{"type":"NODE","target":"110:20202","navigation":"NAVIGATE"}]`
- `107:37372` Action / Open a sequence → `[{"type":"NODE","target":"110:19645","navigation":"NAVIGATE"}]`
- `107:37377` Action / All sequence tools → `[{"type":"NODE","target":"110:22016","navigation":"NAVIGATE"}]`
- `107:37381` Action / Create a sequence → `[{"type":"NODE","target":"110:20202","navigation":"NAVIGATE"}]`
- `107:37382` Action / Open a sequence → `[{"type":"NODE","target":"110:19645","navigation":"NAVIGATE"}]`

## ST-229 · Empty / Features

**Role:** Reference or alternate fixture. **Frame:** 107:37384 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-37384).

**Displayed content:**

- Features
- pHelixBench-Demo · 3021 bp
- Search
- No annotations in this sequence
- Mark a feature yourself or review common annotations before adding them to
the sequence.
- Add a feature
- Detect common features
- No changes are applied until you review and confirm them.
- Details & tools
- All features tools
- Nothing selected
- Select an annotation to inspect its
coordinates and strand.
- Start here
- Tools that need a record require you to
create, open or select one first.

**Authored actions:**

- `107:37490` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:37496` Action / Add a feature → `[{"type":"NODE","target":"107:25134","navigation":"OVERLAY"}]`
- `107:37497` Action / Detect common features → `[{"type":"NODE","target":"107:26060","navigation":"OVERLAY"}]`
- `107:37502` Action / All features tools → `[{"type":"NODE","target":"107:28097","navigation":"OVERLAY"}]`
- `107:37506` Action / Add a feature → `[{"type":"NODE","target":"107:25134","navigation":"OVERLAY"}]`
- `107:37507` Action / Detect common features → `[{"type":"NODE","target":"107:26060","navigation":"OVERLAY"}]`

## ST-230 · Empty / Primers

**Role:** Reference or alternate fixture. **Frame:** 107:37509 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-37509).

**Displayed content:**

- Primers
- pHelixBench-Demo · 3021 bp
- Search
- No primers in this sequence
- Add a primer or design a linked pair using a selected region of the
template.
- Add a primer
- Design a primer pair
- A linked forward and reverse pair is needed for PCR simulation.
- Details & tools
- All primers tools
- No primer selected
- Primer details and pair-specific
actions will appear here.
- Start here
- Tools that need a record require you to
create, open or select one first.

**Authored actions:**

- `107:37615` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:37621` Action / Add a primer → `[{"type":"NODE","target":"107:25182","navigation":"OVERLAY"}]`
- `107:37622` Action / Design a primer pair → `[{"type":"NODE","target":"107:25303","navigation":"OVERLAY"}]`
- `107:37627` Action / All primers tools → `[{"type":"NODE","target":"107:28166","navigation":"OVERLAY"}]`
- `107:37631` Action / Add a primer → `[{"type":"NODE","target":"107:25182","navigation":"OVERLAY"}]`
- `107:37632` Action / Design a primer pair → `[{"type":"NODE","target":"107:25303","navigation":"OVERLAY"}]`

## ST-231 · Empty / Enzymes

**Role:** Reference or alternate fixture. **Frame:** 107:37634 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-37634).

**Displayed content:**

- Enzymes
- pHelixBench-Demo · 3021 bp
- Search
- No cut sites match this enzyme set
- Try another enzyme set or relax the active filters to find relevant cutting
sites.
- Change enzyme set
- Reset filters
- Try a wider enzyme set or review the current filters.
- Details & tools
- All enzymes tools
- No matching cut site
- Change the enzyme set or filters to
inspect other sites.
- Start here
- Tools that need a record require you to
create, open or select one first.

**Authored actions:**

- `107:37740` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:37746` Action / Change enzyme set → `[{"type":"NODE","target":"107:25217","navigation":"OVERLAY"}]`
- `107:37747` Action / Reset filters → `[{"type":"NODE","target":"107:30295","navigation":"NAVIGATE"}]`
- `107:37752` Action / All enzymes tools → `[{"type":"NODE","target":"107:28231","navigation":"OVERLAY"}]`
- `107:37756` Action / Change enzyme set → `[{"type":"NODE","target":"107:25217","navigation":"OVERLAY"}]`
- `107:37757` Action / Reset filters → `[{"type":"NODE","target":"107:30295","navigation":"NAVIGATE"}]`

## ST-232 · Empty / History

**Role:** Reference or alternate fixture. **Frame:** 107:37759 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-37759).

**Displayed content:**

- History
- pHelixBench-Demo · 3021 bp
- Search
- No saved versions yet
- Save a version to create a named point you can compare or restore later.
- Save a version
- Return to sequence
- Saving a version and undoing a recent edit are different actions.
- Details & tools
- All history tools
- No version selected
- Select a saved version to compare
changes before restoring.
- Start here
- Tools that need a record require you to
create, open or select one first.

**Authored actions:**

- `107:37865` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:37871` Action / Save a version → `[{"type":"NODE","target":"107:29211","navigation":"OVERLAY"}]`
- `107:37872` Action / Return to sequence → `[{"type":"NODE","target":"107:29677","navigation":"NAVIGATE"}]`
- `107:37877` Action / All history tools → `[{"type":"NODE","target":"107:28296","navigation":"OVERLAY"}]`
- `107:37881` Action / Save a version → `[{"type":"NODE","target":"107:29211","navigation":"OVERLAY"}]`
- `107:37882` Action / Return to sequence → `[{"type":"NODE","target":"107:29677","navigation":"NAVIGATE"}]`

## ST-233 · Empty / Notebook

**Role:** Reference or alternate fixture. **Frame:** 107:37884 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-37884).

**Displayed content:**

- Notebook
- Search
- No notebook entries yet
- Start an entry for your protocol, observations and structured results.
- Create an entry
- Use a template
- Saving text and submitting structured results remain separate.
- Details & tools
- All notebook tools
- No entry selected
- Create or open an entry to see its
details and review state.
- Start here
- Tools that need a record require you to
create, open or select one first.

**Authored actions:**

- `107:37985` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:37991` Action / Create an entry → `[{"type":"NODE","target":"110:20232","navigation":"NAVIGATE"}]`
- `107:37992` Action / Use a template → `[{"type":"NODE","target":"107:26892","navigation":"OVERLAY"}]`
- `107:37997` Action / All notebook tools → `[{"type":"NODE","target":"107:28334","navigation":"OVERLAY"}]`
- `107:38001` Action / Create an entry → `[{"type":"NODE","target":"110:20232","navigation":"NAVIGATE"}]`
- `107:38002` Action / Use a template → `[{"type":"NODE","target":"107:26892","navigation":"OVERLAY"}]`

## ST-234 · Empty / Registry

**Role:** Reference or alternate fixture. **Frame:** 107:38004 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-38004).

**Displayed content:**

- Registry
- Search
- No entities registered yet
- Give your sequences, proteins and other scientific entities a stable
identity.
- Register an entity
- Import a spreadsheet
- Registry identifies the entity. Inventory tracks its physical samples.
- Details & tools
- All registry tools
- No entity selected
- Entity properties and related samples
will appear here.
- Start here
- Tools that need a record require you to
create, open or select one first.

**Authored actions:**

- `107:38105` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:38111` Action / Register an entity → `[{"type":"NODE","target":"107:25875","navigation":"OVERLAY"}]`
- `107:38112` Action / Import a spreadsheet → `[{"type":"NODE","target":"107:33652","navigation":"OVERLAY"}]`
- `107:38117` Action / All registry tools → `[{"type":"NODE","target":"107:28399","navigation":"OVERLAY"}]`
- `107:38121` Action / Register an entity → `[{"type":"NODE","target":"107:25875","navigation":"OVERLAY"}]`
- `107:38122` Action / Import a spreadsheet → `[{"type":"NODE","target":"107:33652","navigation":"OVERLAY"}]`

## ST-235 · Empty / Inventory

**Role:** Reference or alternate fixture. **Frame:** 107:38124 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-38124).

**Displayed content:**

- Inventory
- Search
- No containers yet
- Add a container to begin mapping physical samples, quantities and storage
locations.
- Add a container
- You can link physical samples to their scientific entities.
- Details & tools
- All inventory tools
- No container selected
- Choose a container or well to inspect
contents and location.
- Start here
- Tools that need a record require you to
create, open or select one first.

**Authored actions:**

- `107:38225` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:38231` Action / Add a container → `[{"type":"NODE","target":"107:27008","navigation":"OVERLAY"}]`
- `107:38236` Action / All inventory tools → `[{"type":"NODE","target":"107:28459","navigation":"OVERLAY"}]`
- `107:38240` Action / Add a container → `[{"type":"NODE","target":"107:27008","navigation":"OVERLAY"}]`

## ST-236 · Empty / Workflows

**Role:** Reference or alternate fixture. **Frame:** 107:38242 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-38242).

**Displayed content:**

- Workflows
- Search
- No tasks yet
- Create a task or prepare a request to coordinate the next step of your work.
- Create a task
- Prepare a request
- Assigned work, blockers and progress will appear in this list.
- Details & tools
- All workflows tools
- No task selected
- Task inputs, outputs and the next
allowed action appear here.
- Start here
- Tools that need a record require you to
create, open or select one first.

**Authored actions:**

- `107:38343` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:38349` Action / Create a task → `[{"type":"NODE","target":"107:27034","navigation":"OVERLAY"}]`
- `107:38350` Action / Prepare a request → `[{"type":"NODE","target":"107:33859","navigation":"OVERLAY"}]`
- `107:38355` Action / All workflows tools → `[{"type":"NODE","target":"107:28515","navigation":"OVERLAY"}]`
- `107:38359` Action / Create a task → `[{"type":"NODE","target":"107:27034","navigation":"OVERLAY"}]`
- `107:38360` Action / Prepare a request → `[{"type":"NODE","target":"107:33859","navigation":"OVERLAY"}]`

## ST-237 · Empty / Insights

**Role:** Reference or alternate fixture. **Frame:** 107:38362 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-38362).

**Displayed content:**

- Insights
- Search
- No dashboards yet
- Choose a data source and configure a view to explore results you can access.
- Create an insight
- An empty dashboard differs from a query with no matching rows.
- Details & tools
- All insights tools
- No insight selected
- Select a dashboard or block to inspect
its source and filters.
- Start here
- Tools that need a record require you to
create, open or select one first.

**Authored actions:**

- `107:38463` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:38469` Action / Create an insight → `[{"type":"NODE","target":"107:26918","navigation":"OVERLAY"}]`
- `107:38474` Action / All insights tools → `[{"type":"NODE","target":"107:28571","navigation":"OVERLAY"}]`
- `107:38478` Action / Create an insight → `[{"type":"NODE","target":"107:26918","navigation":"OVERLAY"}]`

## ST-238 · Empty / Automation

**Role:** Reference or alternate fixture. **Frame:** 107:38480 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-38480).

**Displayed content:**

- Automation
- Search
- No pipelines yet
- Create a pipeline to validate inputs, transform data and review the
resulting output.
- Create a pipeline
- Test a draft before publication or writing results.
- Details & tools
- All automation tools
- No pipeline selected
- A selected step will show its input,
output and configuration.
- Start here
- Tools that need a record require you to
create, open or select one first.

**Authored actions:**

- `107:38581` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:38587` Action / Create a pipeline → `[{"type":"NODE","target":"107:26716","navigation":"OVERLAY"}]`
- `107:38592` Action / All automation tools → `[{"type":"NODE","target":"107:28604","navigation":"OVERLAY"}]`
- `107:38596` Action / Create a pipeline → `[{"type":"NODE","target":"107:26716","navigation":"OVERLAY"}]`

## ST-239 · Empty / Split

**Role:** Reference or alternate fixture. **Frame:** 107:38598 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-38598).

**Displayed content:**

- Split
- Search
- Choose two sequences to compare
- Open the first record, then choose a second sequence for a side-by-side
comparison.
- Choose a sequence
- Run an alignment after choosing both input sequences.
- Details & tools
- All split tools
- No comparison selected
- Choose a pair of sequences before
reviewing alignment settings.
- Start here
- Tools that need a record require you to
create, open or select one first.

**Authored actions:**

- `107:38699` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:38705` Action / Choose a sequence → `[{"type":"NODE","target":"110:19645","navigation":"NAVIGATE"}]`
- `107:38710` Action / All split tools → `[{"type":"NODE","target":"107:28655","navigation":"OVERLAY"}]`
- `107:38714` Action / Choose a sequence → `[{"type":"NODE","target":"110:19645","navigation":"NAVIGATE"}]`

## ST-240 · Empty / Biologics

**Role:** Reference or alternate fixture. **Frame:** 107:38716 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-38716).

**Displayed content:**

- Biologics
- Search
- No biologics registered yet
- Start with a protein format and connect its chains and component identities.
- Register a biologic
- You can review chain identity before creating a record.
- Details & tools
- All biologics tools
- No biologic selected
- Format, chains and related records
will appear here.
- Start here
- Tools that need a record require you to
create, open or select one first.

**Authored actions:**

- `107:38818` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:38824` Action / Register a biologic → `[{"type":"NODE","target":"107:26448","navigation":"OVERLAY"}]`
- `107:38829` Action / All biologics tools → `[{"type":"NODE","target":"107:28688","navigation":"OVERLAY"}]`
- `107:38833` Action / Register a biologic → `[{"type":"NODE","target":"107:26448","navigation":"OVERLAY"}]`

## ST-241 · Empty / Chemistry

**Role:** Reference or alternate fixture. **Frame:** 107:38835 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-38835).

**Displayed content:**

- Chemistry
- Search
- No compounds yet
- Register a chemical identity, then connect its forms and physical lots.
- Add a compound
- A compound and a physical lot are separate linked records.
- Details & tools
- All chemistry tools
- No compound selected
- Select a compound, form or lot to
inspect its relationships.
- Start here
- Tools that need a record require you to
create, open or select one first.

**Authored actions:**

- `107:38937` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:38943` Action / Add a compound → `[{"type":"NODE","target":"107:26478","navigation":"OVERLAY"}]`
- `107:38948` Action / All chemistry tools → `[{"type":"NODE","target":"107:28726","navigation":"OVERLAY"}]`
- `107:38952` Action / Add a compound → `[{"type":"NODE","target":"107:26478","navigation":"OVERLAY"}]`

## ST-242 · Empty / RNA / Oligos

**Role:** Reference or alternate fixture. **Frame:** 107:38954 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-38954).

**Displayed content:**

- RNA / Oligos
- Search
- No RNA or oligo records yet
- Add a record, choose its notation and review any nucleotide modifications.
- Add an oligo
- Bulk patterns apply only to eligible oligos.
- Details & tools
- All rna / oligos tools
- No oligo selected
- Select a record to see notation and
modification controls.
- Start here
- Tools that need a record require you to
create, open or select one first.

**Authored actions:**

- `107:39056` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:39062` Action / Add an oligo → `[{"type":"NODE","target":"107:26560","navigation":"OVERLAY"}]`
- `107:39067` Action / All rna / oligos tools → `[{"type":"NODE","target":"107:28777","navigation":"OVERLAY"}]`
- `107:39071` Action / Add an oligo → `[{"type":"NODE","target":"107:26560","navigation":"OVERLAY"}]`

## ST-243 · Empty / In Vivo

**Role:** Reference or alternate fixture. **Frame:** 107:39073 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-39073).

**Displayed content:**

- In Vivo
- Search
- No studies yet
- Create a study to organize animals, scheduled tasks and measurements.
- Create a study
- Animal identity and measurement units stay visible during capture.
- Details & tools
- All in vivo tools
- No study selected
- Study, cohort and scheduled-task
details will appear here.
- Start here
- Tools that need a record require you to
create, open or select one first.

**Authored actions:**

- `107:39175` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:39181` Action / Create a study → `[{"type":"NODE","target":"107:26590","navigation":"OVERLAY"}]`
- `107:39186` Action / All in vivo tools → `[{"type":"NODE","target":"107:28819","navigation":"OVERLAY"}]`
- `107:39190` Action / Create a study → `[{"type":"NODE","target":"107:26590","navigation":"OVERLAY"}]`

## ST-244 · Empty / Bioprocess

**Role:** Reference or alternate fixture. **Frame:** 107:39192 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-39192).

**Displayed content:**

- Bioprocess
- Search
- No recipes yet
- Create a recipe before planning conditions, replicates and execution runs.
- Create a recipe
- Planned runs retain their chosen recipe reference.
- Details & tools
- All bioprocess tools
- No recipe selected
- Choose a recipe or run to inspect its
parameters and status.
- Start here
- Tools that need a record require you to
create, open or select one first.

**Authored actions:**

- `107:39294` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:39300` Action / Create a recipe → `[{"type":"NODE","target":"107:26664","navigation":"OVERLAY"}]`
- `107:39305` Action / All bioprocess tools → `[{"type":"NODE","target":"107:28870","navigation":"OVERLAY"}]`
- `107:39309` Action / Create a recipe → `[{"type":"NODE","target":"107:26664","navigation":"OVERLAY"}]`

## ST-245 · Empty / Search · no matches

**Role:** Reference or alternate fixture. **Frame:** 107:39311 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-39311).

**Displayed content:**

- Registry
- Search
- No matches for “expression qc”
- Try a record name, stable ID, or a shorter search term.
- Clear search
- Change search
- The search completed. It found no matching records you can access.
- Details & tools
- All registry tools
- No search match
- A completed search with zero matches
is not a loading failure.
- Start here
- Change the query or filters to show accessible matches.

**Authored actions:**

- `107:39412` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:39418` Action / Clear search → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`
- `107:39419` Action / Change search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:39424` Action / All registry tools → `[{"type":"NODE","target":"107:28399","navigation":"OVERLAY"}]`
- `107:39428` Action / Clear search → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`
- `107:39429` Action / Change search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`

## ST-246 · Empty / Filters · no rows

**Role:** Reference or alternate fixture. **Frame:** 107:39431 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-39431).

**Displayed content:**

- Registry
- Search
- No records match these filters
- Try removing a filter or widening the selected date range.
- Clear filters
- Edit filters
- Active filters: Expression study · Archived · Last 24 hours.
- Details & tools
- All registry tools
- No filtered result
- The records may exist outside the
current filter scope.
- Start here
- Change the query or filters to show accessible matches.

**Authored actions:**

- `107:39532` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:39538` Action / Clear filters → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`
- `107:39539` Action / Edit filters → `[{"type":"NODE","target":"107:26978","navigation":"OVERLAY"}]`
- `107:39544` Action / All registry tools → `[{"type":"NODE","target":"107:28399","navigation":"OVERLAY"}]`
- `107:39548` Action / Clear filters → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`
- `107:39549` Action / Edit filters → `[{"type":"NODE","target":"107:26978","navigation":"OVERLAY"}]`

## ST-247 · Empty / Load failure

**Role:** Reference or alternate fixture. **Frame:** 107:39551 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-39551).

**Displayed content:**

- Registry
- Search
- We couldn’t load these records
- Try again to load this view, or return to another workspace while it is
unavailable.
- Try again
- Back to Home
- Saved records will appear when this view loads successfully.
- Details & tools
- All registry tools
- Records unavailable
- Details can be shown after the records
load successfully.
- Start here
- Retry loading before inspecting record details.

**Authored actions:**

- `107:39652` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:39658` Action / Try again → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`
- `107:39659` Action / Back to Home → `[{"type":"NODE","target":"107:29237","navigation":"NAVIGATE"}]`
- `107:39664` Action / All registry tools → `[{"type":"NODE","target":"107:28399","navigation":"OVERLAY"}]`
- `107:39668` Action / Try again → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`
- `107:39669` Action / Back to Home → `[{"type":"NODE","target":"107:29237","navigation":"NAVIGATE"}]`

## ST-248 · Empty / Restricted access

**Role:** Reference or alternate fixture. **Frame:** 107:39671 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-39671).

**Displayed content:**

- Registry
- Search
- This record isn’t available to you
- Your current access does not allow this record to be opened. You can return
to records you can access.
- Back to Registry
- Restricted record details are not shown.
- Details & tools
- All registry tools
- Access is restricted
- No hidden record details or counts are
exposed.
- Start here
- Only records you can access are shown.

**Authored actions:**

- `107:39772` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:39778` Action / Back to Registry → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`
- `107:39783` Action / All registry tools → `[{"type":"NODE","target":"107:28399","navigation":"OVERLAY"}]`
- `107:39787` Action / Back to Registry → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`

## ST-249 · Empty / No matching data

**Role:** Reference or alternate fixture. **Frame:** 107:39789 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-39789).

**Displayed content:**

- Insights
- Search
- No data matches this view
- The query completed, but the current parameters returned no accessible rows.
- Change parameters
- Return to dashboard
- Review the project, date range and other parameters before trying again.
- Details & tools
- All insights tools
- No matching data
- An unset parameter and a completed
zero-row query are distinct states.
- Start here
- Change the query or filters to show accessible matches.

**Authored actions:**

- `107:39890` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:39896` Action / Change parameters → `[{"type":"NODE","target":"107:33922","navigation":"OVERLAY"}]`
- `107:39897` Action / Return to dashboard → `[{"type":"NODE","target":"107:31682","navigation":"NAVIGATE"}]`
- `107:39902` Action / All insights tools → `[{"type":"NODE","target":"107:28571","navigation":"OVERLAY"}]`
- `107:39906` Action / Change parameters → `[{"type":"NODE","target":"107:33922","navigation":"OVERLAY"}]`
- `107:39907` Action / Return to dashboard → `[{"type":"NODE","target":"107:31682","navigation":"NAVIGATE"}]`

## ST-250 · Empty / All modules chooser

**Role:** Reference or alternate fixture. **Frame:** 107:39909 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-39909).

**Displayed content:**

- All modules
- Close
- Open a workspace or a view of the active sequence.
- Home
- Map
- Sequence
- Features
- Primers
- Enzymes
- History
- Notebook
- Registry
- Inventory
- Workflows
- Insights
- Automation
- Split
- Biologics
- Chemistry
- RNA / Oligos
- In Vivo
- Bioprocess

**Authored actions:**

- `107:39913` Action / Close → `[{"type":"CLOSE"}]`
- `107:39917` Action / Home → `[{"type":"NODE","target":"107:37024","navigation":"NAVIGATE"}]`
- `107:39918` Action / Map → `[{"type":"NODE","target":"107:37144","navigation":"NAVIGATE"}]`
- `107:39919` Action / Sequence → `[{"type":"NODE","target":"107:37264","navigation":"NAVIGATE"}]`
- `107:39921` Action / Features → `[{"type":"NODE","target":"107:37384","navigation":"NAVIGATE"}]`
- `107:39922` Action / Primers → `[{"type":"NODE","target":"107:37509","navigation":"NAVIGATE"}]`
- `107:39923` Action / Enzymes → `[{"type":"NODE","target":"107:37634","navigation":"NAVIGATE"}]`
- `107:39925` Action / History → `[{"type":"NODE","target":"107:37759","navigation":"NAVIGATE"}]`
- `107:39926` Action / Notebook → `[{"type":"NODE","target":"107:37884","navigation":"NAVIGATE"}]`
- `107:39927` Action / Registry → `[{"type":"NODE","target":"107:38004","navigation":"NAVIGATE"}]`
- `107:39929` Action / Inventory → `[{"type":"NODE","target":"107:38124","navigation":"NAVIGATE"}]`
- `107:39930` Action / Workflows → `[{"type":"NODE","target":"107:38242","navigation":"NAVIGATE"}]`
- `107:39931` Action / Insights → `[{"type":"NODE","target":"107:38362","navigation":"NAVIGATE"}]`
- `107:39933` Action / Automation → `[{"type":"NODE","target":"107:38480","navigation":"NAVIGATE"}]`
- `107:39934` Action / Split → `[{"type":"NODE","target":"107:38598","navigation":"NAVIGATE"}]`
- `107:39935` Action / Biologics → `[{"type":"NODE","target":"107:38716","navigation":"NAVIGATE"}]`
- `107:39937` Action / Chemistry → `[{"type":"NODE","target":"107:38835","navigation":"NAVIGATE"}]`
- `107:39938` Action / RNA / Oligos → `[{"type":"NODE","target":"107:38954","navigation":"NAVIGATE"}]`
- `107:39939` Action / In Vivo → `[{"type":"NODE","target":"107:39073","navigation":"NAVIGATE"}]`
- `107:39941` Action / Bioprocess → `[{"type":"NODE","target":"107:39192","navigation":"NAVIGATE"}]`
- `107:39944` Action / Close → `[{"type":"CLOSE"}]`

## ST-251 · Empty states / Start here

**Role:** Reference or alternate fixture. **Frame:** 107:39945 · 1600 × 850. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-39945).

**Displayed content:**

- HelixBench · Empty states
- Cave-painting artwork paired with specific explanations and next actions. The 19 module screens below are illustrative empty contexts, not one shared data fixture.
- Module empty states
- Home
- Map
- Sequence
- Features
- Primers
- Enzymes
- History
- Notebook
- Registry
- Inventory
- Workflows
- Insights
- Automation
- Split
- Biologics
- Chemistry
- RNA / Oligos
- In Vivo
- Bioprocess
- Shared no-content and recovery states
- Search · no matches
- Filters · no rows
- Load failure
- Restricted access
- No matching data
- State rules
- No records → create or import. No matches → change the query or clear filters. Loading failed → retry. Restricted access → return to accessible records. No selection → explain what can be selected.
- Artwork is decorative. Status and recovery are always communicated in text. Existing populated v3 screens remain intact.

**Authored actions:**

- `107:39950` Action / Home → `[{"type":"NODE","target":"107:37024","navigation":"NAVIGATE"}]`
- `107:39951` Action / Map → `[{"type":"NODE","target":"107:37144","navigation":"NAVIGATE"}]`
- `107:39952` Action / Sequence → `[{"type":"NODE","target":"107:37264","navigation":"NAVIGATE"}]`
- `107:39953` Action / Features → `[{"type":"NODE","target":"107:37384","navigation":"NAVIGATE"}]`
- `107:39955` Action / Primers → `[{"type":"NODE","target":"107:37509","navigation":"NAVIGATE"}]`
- `107:39956` Action / Enzymes → `[{"type":"NODE","target":"107:37634","navigation":"NAVIGATE"}]`
- `107:39957` Action / History → `[{"type":"NODE","target":"107:37759","navigation":"NAVIGATE"}]`
- `107:39958` Action / Notebook → `[{"type":"NODE","target":"107:37884","navigation":"NAVIGATE"}]`
- `107:39960` Action / Registry → `[{"type":"NODE","target":"107:38004","navigation":"NAVIGATE"}]`
- `107:39961` Action / Inventory → `[{"type":"NODE","target":"107:38124","navigation":"NAVIGATE"}]`
- `107:39962` Action / Workflows → `[{"type":"NODE","target":"107:38242","navigation":"NAVIGATE"}]`
- `107:39963` Action / Insights → `[{"type":"NODE","target":"107:38362","navigation":"NAVIGATE"}]`
- `107:39965` Action / Automation → `[{"type":"NODE","target":"107:38480","navigation":"NAVIGATE"}]`
- `107:39966` Action / Split → `[{"type":"NODE","target":"107:38598","navigation":"NAVIGATE"}]`
- `107:39967` Action / Biologics → `[{"type":"NODE","target":"107:38716","navigation":"NAVIGATE"}]`
- `107:39968` Action / Chemistry → `[{"type":"NODE","target":"107:38835","navigation":"NAVIGATE"}]`
- `107:39970` Action / RNA / Oligos → `[{"type":"NODE","target":"107:38954","navigation":"NAVIGATE"}]`
- `107:39971` Action / In Vivo → `[{"type":"NODE","target":"107:39073","navigation":"NAVIGATE"}]`
- `107:39972` Action / Bioprocess → `[{"type":"NODE","target":"107:39192","navigation":"NAVIGATE"}]`
- `107:39975` Action / Search · no matches → `[{"type":"NODE","target":"107:39311","navigation":"NAVIGATE"}]`
- `107:39976` Action / Filters · no rows → `[{"type":"NODE","target":"107:39431","navigation":"NAVIGATE"}]`
- `107:39977` Action / Load failure → `[{"type":"NODE","target":"107:39551","navigation":"NAVIGATE"}]`
- `107:39978` Action / Restricted access → `[{"type":"NODE","target":"107:39671","navigation":"NAVIGATE"}]`
- `107:39980` Action / No matching data → `[{"type":"NODE","target":"107:39789","navigation":"NAVIGATE"}]`

## ST-252 · Feedback / Saved

**Role:** Reference or alternate fixture. **Frame:** 107:39984 · 720 × 560. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-39984).

**Displayed content:**

- Success
- Close
- Record saved
- DNA-001 · changes saved as version 8.
- Record
- pHelixBench-Demo · DNA-001
- Saved state
- Version 8 · input history retained
- View record
- Done

**Authored actions:**

- `107:39990` Action / Close → `[{"type":"CLOSE"}]`
- `107:40004` Action / View record → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`
- `107:40005` Action / Done → `[{"type":"CLOSE"}]`

## ST-253 · Feedback / Submitted

**Role:** Reference or alternate fixture. **Frame:** 107:40006 · 720 × 620. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-40006).

**Displayed content:**

- Success
- Close
- Results submitted
- Two result records were created from EXP-014.
- Created results
- RES-101 and RES-102
- Source
- EXP-014 / Assay results
- Review state
- Ready to send for entry review
- View submitted entry
- Start review

**Authored actions:**

- `107:40012` Action / Close → `[{"type":"CLOSE"}]`
- `107:40029` Action / View submitted entry → `[{"type":"NODE","target":"107:35097","navigation":"SWAP"}]`
- `107:40030` Action / Start review → `[{"type":"NODE","target":"107:34537","navigation":"SWAP"}]`

## ST-254 · Feedback / Imported

**Role:** Reference or alternate fixture. **Frame:** 107:40031 · 720 × 620. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-40031).

**Displayed content:**

- Success
- Close
- Import completed with skipped rows
- 22 entities created. Two source rows were skipped by your choice.
- Source file
- constructs_batch.csv · 24 rows
- Created / skipped
- 22 created · 2 skipped · 0 updated
- Next action
- Review the skipped-row report before retrying those rows
- Open Registry
- Review skipped rows

**Authored actions:**

- `107:40037` Action / Close → `[{"type":"CLOSE"}]`
- `107:40054` Action / Open Registry → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`
- `107:40055` Action / Review skipped rows → `[{"type":"NODE","target":"107:34423","navigation":"SWAP"}]`

## ST-255 · Feedback / RunComplete

**Role:** Reference or alternate fixture. **Frame:** 107:40056 · 720 × 620. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-40056).

**Displayed content:**

- Success
- Close
- Automation run completed
- R-019 · 96 result records created.
- Input
- assay_results.csv
- Template reference
- TPL-012 · published configuration
- Output
- 96 results · Expression study
- View results
- Open run activity

**Authored actions:**

- `107:40062` Action / Close → `[{"type":"CLOSE"}]`
- `107:40079` Action / View results → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`
- `107:40080` Action / Open run activity → `[{"type":"NODE","target":"107:33461","navigation":"SWAP"}]`

## ST-256 · Feedback / Unsaved

**Role:** Reference or alternate fixture. **Frame:** 107:40081 · 720 × 560. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-40081).

**Displayed content:**

- Warning
- Close
- Save your changes before leaving?
- DNA-001 has edits that have not been saved.
- Affected record
- pHelixBench-Demo · DNA-001
- If you leave without saving
- The unsaved edits will be discarded
- Keep editing
- Save changes
- Discard edits

**Authored actions:**

- `107:40088` Action / Close → `[{"type":"CLOSE"}]`
- `107:40102` Action / Keep editing → `[{"type":"CLOSE"}]`
- `107:40103` Action / Save changes → `[{"type":"NODE","target":"107:39984","navigation":"SWAP"}]`
- `107:40104` Action / Discard edits → `[{"type":"NODE","target":"107:29237","navigation":"NAVIGATE"}]`

## ST-257 · Feedback / PartialImport

**Role:** Reference or alternate fixture. **Frame:** 107:40105 · 720 × 620. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-40105).

**Displayed content:**

- Warning
- Close
- Two rows need your attention
- 22 rows are valid. Two rows have unresolved identity or link errors.
- Default action
- Fix errors before importing
- Optional partial import
- Create 22 valid entities and skip 2 rows
- Retained information
- Source file, mapping and skipped-row details
- Fix errors
- Import 22 valid rows

**Authored actions:**

- `107:40112` Action / Close → `[{"type":"CLOSE"}]`
- `107:40129` Action / Fix errors → `[{"type":"NODE","target":"107:34423","navigation":"SWAP"}]`
- `107:40130` Action / Import 22 valid rows → `[{"type":"NODE","target":"107:40031","navigation":"SWAP"}]`

## ST-258 · Feedback / LowVolume

**Role:** Reference or alternate fixture. **Frame:** 107:40131 · 720 × 620. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-40131).

**Displayed content:**

- Warning
- Close
- Not enough volume for this transfer
- P-014 / A1 has 8 µL available; the requested transfer is 10 µL.
- Source
- Plate P-014 / well A1
- Requested / available
- 10 µL requested · 8 µL available
- Current state
- Transfer has not started
- Adjust transfer
- Cancel

**Authored actions:**

- `107:40138` Action / Close → `[{"type":"CLOSE"}]`
- `107:40155` Action / Adjust transfer → `[{"type":"NODE","target":"107:26858","navigation":"SWAP"}]`
- `107:40156` Action / Cancel → `[{"type":"CLOSE"}]`

## ST-259 · Feedback / Overwrite

**Role:** Reference or alternate fixture. **Frame:** 107:40157 · 720 × 620. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-40157).

**Displayed content:**

- Warning
- Close
- Review existing modifications
- OL-014 already has modifications at positions included in this pattern.
- Affected oligo
- OL-014 · eligible RNA oligo
- Conflict scope
- Positions 3 and 6
- Required decision
- Review the proposed changes before applying the pattern
- Review pattern
- Keep current pattern

**Authored actions:**

- `107:40164` Action / Close → `[{"type":"CLOSE"}]`
- `107:40181` Action / Review pattern → `[{"type":"NODE","target":"107:34184","navigation":"SWAP"}]`
- `107:40182` Action / Keep current pattern → `[{"type":"CLOSE"}]`

## ST-260 · Feedback / Validation

**Role:** Reference or alternate fixture. **Frame:** 107:40183 · 720 × 660. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-40183).

**Displayed content:**

- Alert
- Close
- Fix the signal value
- Row 2 contains “abc”. Signal requires a number in a.u.
- Field
- Assay results / row 2 / Signal
- Entered value
- abc
- Example of valid input
- 1315 · unit a.u.
- Submission state
- No result records submitted
- Back to table
- Keep editing

**Authored actions:**

- `107:40190` Action / Close → `[{"type":"CLOSE"}]`
- `107:40210` Action / Back to table → `[{"type":"NODE","target":"107:30702","navigation":"NAVIGATE"}]`
- `107:40211` Action / Keep editing → `[{"type":"CLOSE"}]`

## ST-261 · Feedback / RunFailed

**Role:** Reference or alternate fixture. **Frame:** 107:40212 · 720 × 620. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-40212).

**Displayed content:**

- Alert
- Close
- Run R-018 failed validation
- Sample ID is missing in source row 18. No result records were written.
- Failed step
- Validate columns
- Input
- assay_results.csv
- Recovery
- Review the error and use a corrected input in a new run
- Review run error
- Choose corrected file

**Authored actions:**

- `107:40219` Action / Close → `[{"type":"CLOSE"}]`
- `107:40236` Action / Review run error → `[{"type":"NODE","target":"107:34042","navigation":"SWAP"}]`
- `107:40237` Action / Choose corrected file → `[{"type":"NODE","target":"107:29155","navigation":"SWAP"}]`

## ST-262 · Feedback / Access

**Role:** Reference or alternate fixture. **Frame:** 107:40238 · 720 × 620. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-40238).

**Displayed content:**

- Alert
- Close
- You can’t delete this record
- Your current role allows viewing this record, but does not allow deletion.
- Record
- DNA-001 · details already accessible to you
- Operation
- Delete · unavailable for this role
- Next step
- Return to records you can access
- Back to Registry

**Authored actions:**

- `107:40245` Action / Close → `[{"type":"CLOSE"}]`
- `107:40262` Action / Back to Registry → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`
- `107:40263` Action / Close → `[{"type":"CLOSE"}]`

## ST-263 · Feedback / Conflict

**Role:** Reference or alternate fixture. **Frame:** 107:40264 · 720 × 620. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-40264).

**Displayed content:**

- Alert
- Close
- A newer version is available
- Compare the saved version with your draft before deciding what to keep.
- Your draft
- Based on version 7 · unsaved edits retained
- Latest saved version
- Version 8 · updated by M. Chen
- Write state
- Your draft has not replaced the newer version
- Compare versions
- Keep my draft

**Authored actions:**

- `107:40271` Action / Close → `[{"type":"CLOSE"}]`
- `107:40288` Action / Compare versions → `[{"type":"NODE","target":"107:33405","navigation":"SWAP"}]`
- `107:40289` Action / Keep my draft → `[{"type":"CLOSE"}]`

## ST-264 · Feedback / Connection

**Role:** Reference or alternate fixture. **Frame:** 107:40290 · 720 × 620. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-40290).

**Displayed content:**

- Alert
- Close
- Instrument connection unavailable
- You can choose an existing input file while this connection is unavailable.
- Source
- LC reader · configured connection
- Affected action
- Load the latest instrument file
- Alternative
- Select an existing file or upload a local file
- Choose a file
- View connection

**Authored actions:**

- `107:40297` Action / Close → `[{"type":"CLOSE"}]`
- `107:40314` Action / Choose a file → `[{"type":"NODE","target":"107:29155","navigation":"SWAP"}]`
- `107:40315` Action / View connection → `[{"type":"NODE","target":"107:34076","navigation":"SWAP"}]`

## ST-265 · Feedback / ArchiveConfirm

**Role:** Reference or alternate fixture. **Frame:** 107:40316 · 720 × 620. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-40316).

**Displayed content:**

- Delete & archive
- Close
- Archive DNA-001?
- The entity will be hidden from active views. Existing links and history are retained.
- Entity
- DNA-001 · pHelixBench-Demo
- Recovery
- Restore it from archived records
- Physical samples
- Containers are not removed by archiving the entity
- Cancel
- Archive entity

**Authored actions:**

- `107:40322` Action / Close → `[{"type":"CLOSE"}]`
- `107:40339` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:40340` Action / Archive entity → `[{"type":"NODE","target":"107:40341","navigation":"SWAP"}]`

## ST-266 · Feedback / Archived

**Role:** Reference or alternate fixture. **Frame:** 107:40341 · 720 × 560. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-40341).

**Displayed content:**

- Success
- Close
- DNA-001 archived
- The entity is hidden from active views. Existing links remain available.
- Archived entity
- DNA-001 · pHelixBench-Demo
- Recovery
- Restore the entity from archived records
- Undo archive
- Done

**Authored actions:**

- `107:40347` Action / Close → `[{"type":"CLOSE"}]`
- `107:40361` Action / Undo archive → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`
- `107:40362` Action / Done → `[{"type":"CLOSE"}]`

## ST-267 · Feedback / DeleteLocked

**Role:** Reference or alternate fixture. **Frame:** 107:40363 · 720 × 540. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-40363).

**Displayed content:**

- Delete & archive
- Close
- Permanently delete this draft?
- TMP-014 will be removed permanently. This action cannot be undone.
- Draft
- TMP-014 · Scratch construct
- Dependency check
- No linked records
- Type the draft ID to confirm
- Enter TMP-014
- Required value
- TMP-014
- Cancel
- Delete draft permanently

**Authored actions:**

- `107:40369` Action / Close → `[{"type":"CLOSE"}]`
- `107:40388` Action / Cancel → `[{"type":"CLOSE"}]`

## ST-268 · Feedback / DeleteReady

**Role:** Reference or alternate fixture. **Frame:** 107:40390 · 720 × 480. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-40390).

**Displayed content:**

- Delete & archive
- Close
- Permanently delete this draft?
- TMP-014 will be removed permanently. This action cannot be undone.
- Draft
- TMP-014 · Scratch construct
- Dependency check
- No linked records
- Confirm draft ID
- TMP-014
- Cancel
- Delete draft permanently

**Authored actions:**

- `107:40396` Action / Close → `[{"type":"CLOSE"}]`
- `107:40412` Action / Cancel → `[{"type":"CLOSE"}]`
- `107:40413` Action / Delete draft permanently → `[{"type":"NODE","target":"107:40463","navigation":"SWAP"}]`

## ST-269 · Feedback / DeleteBlocked

**Role:** Reference or alternate fixture. **Frame:** 107:40414 · 720 × 480. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-40414).

**Displayed content:**

- Delete & archive
- Close
- This entity is used by other records
- Resolve its dependencies before permanent deletion, or archive it instead.
- Entity
- DNA-001 · pHelixBench-Demo
- Linked dependencies
- 1 active run · 2 notebook entries
- Current state
- Permanent deletion is blocked
- View dependencies
- Archive instead

**Authored actions:**

- `107:40420` Action / Close → `[{"type":"CLOSE"}]`
- `107:40435` Action / View dependencies → `[{"type":"NODE","target":"107:33679","navigation":"SWAP"}]`
- `107:40436` Action / Archive instead → `[{"type":"NODE","target":"107:40316","navigation":"SWAP"}]`

## ST-270 · Feedback / DeleteFailed

**Role:** Reference or alternate fixture. **Frame:** 107:40437 · 720 × 620. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-40437).

**Displayed content:**

- Alert
- Close
- The draft could not be deleted
- TMP-014 is still present. Review the confirmation before trying again.
- Draft
- TMP-014 · Scratch construct
- Operation state
- Delete failed · draft retained
- Recovery
- Return to the confirmation or keep the draft
- Review and retry
- Keep draft

**Authored actions:**

- `107:40444` Action / Close → `[{"type":"CLOSE"}]`
- `107:40461` Action / Review and retry → `[{"type":"NODE","target":"107:40390","navigation":"SWAP"}]`
- `107:40462` Action / Keep draft → `[{"type":"CLOSE"}]`

## ST-271 · Feedback / Deleted

**Role:** Reference or alternate fixture. **Frame:** 107:40463 · 720 × 480. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-40463).

**Displayed content:**

- Success
- Close
- Draft permanently deleted
- TMP-014 was removed. This operation has no undo action.
- Deleted draft
- TMP-014 · Scratch construct
- Operation
- Permanent deletion completed
- Back to Registry
- Done

**Authored actions:**

- `107:40469` Action / Close → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`
- `107:40481` Action / Back to Registry → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`
- `107:40482` Action / Done → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`

## ST-272 · Feedback context / Notebook

**Role:** Responsive/context variant. **Frame:** 107:40483 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-40483).

**Displayed content:**

- Notebook
- Capture structured data and submit reusable results.
- Search
- New
- Results submitted
- 2 results created from EXP-014. Your entry is ready for review.
- View results
- Insert result table
- Send for review
- Expression cassette validation
- Draft
- EXP-014 · Expression study · text saved at 10:36
- Notes
- Results
- Review
- Activity
- Purpose
- Verify the expression construct and record assay results.
- DNA-001 · construct
- P-014 · snapshot
- Assay results
- Sample
- Signal (a.u.)
- Result state
- S-001
- 1280
- Submitted
- S-002
- 1315
- Submitting this table creates reusable results linked to this entry.
- Details & tools
- All notebook tools
- Current selection
- Expression validation
- Entry
- EXP-014 · draft
- Save state
- Text saved · 2 results submitted
- Ready to send for review
- Actions
- Related records
- DNA-001 · Entity
- P-014 · Plate snapshot

**Authored actions:**

- `107:40590` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:40591` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `107:40599` Action / View results → `[{"type":"NODE","target":"107:35097","navigation":"OVERLAY"}]`
- `107:40601` Action / Insert result table → `[{"type":"NODE","target":"107:33489","navigation":"OVERLAY"}]`
- `107:40602` Action / View results → `[{"type":"NODE","target":"107:35097","navigation":"OVERLAY"}]`
- `107:40603` Action / Send for review → `[{"type":"NODE","target":"107:34537","navigation":"OVERLAY"}]`
- `107:40613` Action / Results → `[{"type":"NODE","target":"107:33516","navigation":"OVERLAY"}]`
- `107:40614` Action / Review → `[{"type":"NODE","target":"107:33554","navigation":"OVERLAY"}]`
- `107:40615` Action / Activity → `[{"type":"NODE","target":"107:33461","navigation":"OVERLAY"}]`
- `107:40620` Action / DNA-001 · construct → `[{"type":"NODE","target":"107:33242","navigation":"OVERLAY"}]`
- `107:40621` Action / P-014 · snapshot → `[{"type":"NODE","target":"107:33579","navigation":"OVERLAY"}]`
- `107:40628` Record / S-001 → `[{"type":"NODE","target":"107:33516","navigation":"OVERLAY"}]`
- `107:40632` Record / S-002 → `[{"type":"NODE","target":"107:33516","navigation":"OVERLAY"}]`
- `107:40651` Action / All notebook tools → `[{"type":"NODE","target":"107:28334","navigation":"OVERLAY"}]`
- `107:40665` Action / Insert result table → `[{"type":"NODE","target":"107:33489","navigation":"OVERLAY"}]`
- `107:40666` Action / View results → `[{"type":"NODE","target":"107:35097","navigation":"OVERLAY"}]`
- `107:40667` Action / Send for review → `[{"type":"NODE","target":"107:34537","navigation":"OVERLAY"}]`
- `107:40670` Related / DNA-001 · Entity → `[{"type":"NODE","target":"107:33242","navigation":"OVERLAY"}]`
- `107:40675` Related / P-014 · Plate snapshot → `[{"type":"NODE","target":"107:33579","navigation":"OVERLAY"}]`

## ST-273 · Feedback context / Inventory

**Role:** Responsive/context variant. **Frame:** 107:40680 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-40680).

**Displayed content:**

- Inventory
- Locate physical samples and review changes before transfer.
- Search
- New
- Add container
- Transfer selected
- Scan / paste barcode
- Transfer needs attention
- Well A1 has 8 µL available; 10 µL was requested. Transfer has not started.
- Review transfer
- Storage
- Freezer A
-   Shelf 2
-     Rack R-01
- P-014 · live plate
- 96 wells · 4 occupied
- Find entity samples
- Plate P-014
- Live inventory
- A1–A4 selected · 4 wells · Freezer A / Shelf 2
- 
- 1
- 2
- 3
- 4
- 5
- 6
- 7
- 8
- 9
- 10
- 11
- 12
- A
- S1
- S2
- S3
- S4
- —
- B
- C
- D
- E
- F
- G
- H
- Assign contents
- Set role / concentration
- Details & tools
- All inventory tools
- Current selection
- P-014 · 4 wells selected
- Record type
- Live plate · 96 wells
- Location
- Freezer A / Shelf 2
- Selection
- A1–A4 · 4 wells
- Actions
- Open experiment snapshot
- Related records
- DNA-001 · Entity
- EXP-014 · Notebook

**Authored actions:**

- `107:40787` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:40788` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `107:40791` Action / Add container → `[{"type":"NODE","target":"107:27008","navigation":"OVERLAY"}]`
- `107:40792` Action / Transfer selected → `[{"type":"NODE","target":"107:40131","navigation":"OVERLAY"}]`
- `107:40793` Action / Scan / paste barcode → `[{"type":"NODE","target":"107:33742","navigation":"OVERLAY"}]`
- `107:40801` Action / Review transfer → `[{"type":"NODE","target":"107:40131","navigation":"OVERLAY"}]`
- `107:40812` Action / Find entity samples → `[{"type":"NODE","target":"107:33618","navigation":"OVERLAY"}]`
- `107:40837` Well / A1 → `[{"type":"NODE","target":"107:26858","navigation":"OVERLAY"}]`
- `107:41041` Well / H12 → `[{"type":"NODE","target":"107:26858","navigation":"OVERLAY"}]`
- `107:41044` Action / Assign contents → `[{"type":"NODE","target":"107:26858","navigation":"OVERLAY"}]`
- `107:41045` Action / Set role / concentration → `[{"type":"NODE","target":"107:25650","navigation":"OVERLAY"}]`
- `107:41060` Action / All inventory tools → `[{"type":"NODE","target":"107:28459","navigation":"OVERLAY"}]`
- `107:41074` Action / Assign contents → `[{"type":"NODE","target":"107:26858","navigation":"OVERLAY"}]`
- `107:41075` Action / Transfer selected → `[{"type":"NODE","target":"107:40131","navigation":"OVERLAY"}]`
- `107:41076` Action / Open experiment snapshot → `[{"type":"NODE","target":"107:33579","navigation":"OVERLAY"}]`
- `107:41079` Related / DNA-001 · Entity → `[{"type":"NODE","target":"107:33242","navigation":"OVERLAY"}]`
- `107:41084` Related / EXP-014 · Notebook → `[{"type":"NODE","target":"107:30702","navigation":"NAVIGATE"}]`

Well controls: 96. A1/H12 are representative authored action examples; implementation must generate identity-bound controls for every actual well.

## ST-274 · Feedback context / Automation

**Role:** Responsive/context variant. **Frame:** 107:41089 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-41089).

**Displayed content:**

- Automation
- Validate inputs and inspect outputs before writeback.
- Search
- New
- Load input
- Test pipeline
- Publish
- Design
- Run history
- Connections
- Run R-018 failed validation
- Source row 18 has no Sample ID. No result records were written.
- Review error
- Expression QC
- Input file → validated dataset → reviewed output
- CSV input
- assay_results.csv
- Inspect step
- Validate columns
- Sample ID · signal
- Configure selected
- Create results
- Expression results
- Recent run
- State
- Template reference
- Output
- R-018
- Validation failed
- TPL-014 / draft
- No records written
- R-017
- Completed
- TPL-012 / published
- 96 results
- Details & tools
- All automation tools
- Current selection
- Expression QC · draft
- Input
- assay_results.csv · 96 rows
- Destination
- Template state
- Draft · not published
- Actions
- Run published template
- Publish template
- Related records
- Run history R-018
- EXP-014 · Notebook

**Authored actions:**

- `107:41196` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:41197` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `107:41200` Action / Load input → `[{"type":"NODE","target":"107:29155","navigation":"OVERLAY"}]`
- `107:41201` Action / Test pipeline → `[{"type":"NODE","target":"107:33950","navigation":"OVERLAY"}]`
- `107:41202` Action / Publish → `[{"type":"NODE","target":"107:34010","navigation":"OVERLAY"}]`
- `107:41205` Action / Run history → `[{"type":"NODE","target":"107:34042","navigation":"OVERLAY"}]`
- `107:41206` Action / Connections → `[{"type":"NODE","target":"107:34076","navigation":"OVERLAY"}]`
- `107:41214` Action / Review error → `[{"type":"NODE","target":"107:40212","navigation":"OVERLAY"}]`
- `107:41223` Action / Inspect step → `[{"type":"NODE","target":"107:33950","navigation":"OVERLAY"}]`
- `107:41227` Action / Configure selected → `[{"type":"NODE","target":"107:34103","navigation":"OVERLAY"}]`
- `107:41231` Action / Inspect step → `[{"type":"NODE","target":"107:33950","navigation":"OVERLAY"}]`
- `107:41238` Record / R-018 → `[{"type":"NODE","target":"107:34042","navigation":"OVERLAY"}]`
- `107:41243` Record / R-017 → `[{"type":"NODE","target":"107:34042","navigation":"OVERLAY"}]`
- `107:41262` Action / All automation tools → `[{"type":"NODE","target":"107:28604","navigation":"OVERLAY"}]`
- `107:41276` Action / Test pipeline → `[{"type":"NODE","target":"107:33950","navigation":"OVERLAY"}]`
- `107:41277` Action / Run input review → `[{"type":"NODE","target":"107:33978","navigation":"OVERLAY"}]`
- `107:41278` Action / Publish template → `[{"type":"NODE","target":"107:34010","navigation":"OVERLAY"}]`
- `107:41281` Related / Run history R-018 → `[{"type":"NODE","target":"107:34042","navigation":"OVERLAY"}]`
- `107:41286` Related / EXP-014 · Notebook → `[{"type":"NODE","target":"107:30702","navigation":"NAVIGATE"}]`

## ST-275 · Feedback context / Registry

**Role:** Responsive/context variant. **Frame:** 107:41291 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-41291).

**Displayed content:**

- Registry
- Identify entities and follow their connected records.
- Search
- New
- Register entity
- Import spreadsheet
- Find samples
- Entities
- Saved views
- Relationships
- Name / ID
- Schema
- Samples
- Updated
- Scratch construct · TMP-014
- DNA construct
- No containers
- Today
- T7-F · PR-001
- Primer
- 1 container
- AB-014
- Protein
- 2 containers
- Yesterday
- OL-014
- RNA oligo
- Registry describes scientific identity. Quantities and storage positions belong to Inventory.
- Details & tools
- All registry tools
- Current selection
- TMP-014 · Scratch construct
- Draft state
- Not registered
- Physical samples
- Actions
- Edit entity
- Import entities
- Delete & archive
- Close
- Permanently delete this draft?
- TMP-014 will be removed permanently. This action cannot be undone.
- Draft
- Dependency check
- No linked records
- Confirm draft ID
- TMP-014
- Cancel
- Delete draft permanently

**Authored actions:**

- `107:41398` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `107:41399` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `107:41402` Action / Register entity → `[{"type":"NODE","target":"107:25875","navigation":"OVERLAY"}]`
- `107:41403` Action / Import spreadsheet → `[{"type":"NODE","target":"107:33652","navigation":"OVERLAY"}]`
- `107:41404` Action / Find samples → `[{"type":"NODE","target":"107:33618","navigation":"OVERLAY"}]`
- `107:41407` Action / Saved views → `[{"type":"NODE","target":"107:26948","navigation":"OVERLAY"}]`
- `107:41408` Action / Relationships → `[{"type":"NODE","target":"107:33679","navigation":"OVERLAY"}]`
- `107:41415` Record / pHelixBench-Demo · DNA-001 → `[{"type":"NODE","target":"107:33242","navigation":"OVERLAY"}]`
- `107:41420` Record / T7-F · PR-001 → `[{"type":"NODE","target":"107:33242","navigation":"OVERLAY"}]`
- `107:41425` Record / AB-014 → `[{"type":"NODE","target":"107:33242","navigation":"OVERLAY"}]`
- `107:41430` Record / OL-014 → `[{"type":"NODE","target":"107:33242","navigation":"OVERLAY"}]`
- `107:41454` Action / All registry tools → `[{"type":"NODE","target":"107:28399","navigation":"OVERLAY"}]`
- `107:41468` Action / Edit entity → `[{"type":"NODE","target":"107:25875","navigation":"OVERLAY"}]`
- `107:41469` Action / Find samples → `[{"type":"NODE","target":"107:33618","navigation":"OVERLAY"}]`
- `107:41470` Action / Import entities → `[{"type":"NODE","target":"107:33652","navigation":"OVERLAY"}]`
- `107:41479` Action / Close → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`
- `107:41495` Action / Cancel → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`
- `107:41496` Action / Delete draft permanently → `[{"type":"NODE","target":"107:40463","navigation":"OVERLAY"}]`

## ST-276 · Feedback / Compact success toast

**Role:** Reference or alternate fixture. **Frame:** 107:41497 · 600 × 104. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-41497).

**Displayed content:**

- Record saved
- DNA-001 · version 8
- View record

**Authored actions:**

- `107:41503` Action / View record → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`

## ST-277 · Feedback / Pattern guidance

**Role:** Reference or alternate fixture. **Frame:** 107:41504 · 1600 × 410. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-41504).

**Displayed content:**

- Use feedback that matches the consequence
- Success: confirm the completed action without interrupting work. Keep links to its result.
- Warning: describe the risk before proceeding and offer a safe way back.
- Alert: identify the affected record or field, preserve recoverable input, and give a repair action.
- Delete: identify the object, show dependency impact, and distinguish archiving from permanent deletion.
- Implementation handoff: announce success without stealing focus; keep important errors visible. Dialogs need keyboard focus management, a named heading, a safe initial focus and return focus. Never rely on color
or artwork alone.
- Examples use illustrative records. Archive/restore and permanent-delete behavior are proposed contracts and require capability checks in the implementation.

**Authored actions:**

- No clickable transition captured in this frame.

## ST-278 · Feedback states / Start here

**Role:** Reference or alternate fixture. **Frame:** 107:41512 · 1600 × 1100. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=107-41512).

**Displayed content:**

- HelixBench · Success, warnings, alerts & deletion
- Review the message, affected record and next action. These native Figma examples extend UX v3 and reuse the cave-art illustration family.
- Success
- Record saved
- Results submitted
- Import · completed with skipped rows
- Automation run completed
- Archive · completed with undo
- Delete · completed
- Warning
- Save your changes before leaving?
- Two rows need your attention
- Not enough volume for this transfer
- Review existing modifications
- Alert
- Fix the signal value
- Run R-018 failed validation
- You can’t delete this record
- A newer version is available
- Instrument connection unavailable
- Delete · failed, draft retained
- Archive and permanent-delete decisions
- Archive · confirm
- Delete · ID confirmation required
- Delete · ID confirmed
- Delete · blocked by linked records
- In-context examples
- Notebook · results submitted
- Inventory · insufficient volume
- Automation · run failed
- Registry · delete confirmation
- Compact feedback
- Preview success toast
- Archive can be restored. Permanent deletion has no undo. Confirmation-required and ID-confirmed frames are separate design states; implementation must validate the typed ID and current dependencies before enabling
deletion.
- All records and outcomes shown here are illustrative. This design work does not delete application data.

**Authored actions:**

- `107:41517` Action / Record saved → `[{"type":"NODE","target":"107:39984","navigation":"OVERLAY"}]`
- `107:41518` Action / Results submitted → `[{"type":"NODE","target":"107:40006","navigation":"OVERLAY"}]`
- `107:41519` Action / Import · completed with skipped rows → `[{"type":"NODE","target":"107:40031","navigation":"OVERLAY"}]`
- `107:41520` Action / Automation run completed → `[{"type":"NODE","target":"107:40056","navigation":"OVERLAY"}]`
- `107:41522` Action / Archive · completed with undo → `[{"type":"NODE","target":"107:40341","navigation":"OVERLAY"}]`
- `107:41523` Action / Delete · completed → `[{"type":"NODE","target":"107:40463","navigation":"OVERLAY"}]`
- `107:41526` Action / Save your changes before leaving? → `[{"type":"NODE","target":"107:40081","navigation":"OVERLAY"}]`
- `107:41527` Action / Two rows need your attention → `[{"type":"NODE","target":"107:40105","navigation":"OVERLAY"}]`
- `107:41528` Action / Not enough volume for this transfer → `[{"type":"NODE","target":"107:40131","navigation":"OVERLAY"}]`
- `107:41529` Action / Review existing modifications → `[{"type":"NODE","target":"107:40157","navigation":"OVERLAY"}]`
- `107:41532` Action / Fix the signal value → `[{"type":"NODE","target":"107:40183","navigation":"OVERLAY"}]`
- `107:41533` Action / Run R-018 failed validation → `[{"type":"NODE","target":"107:40212","navigation":"OVERLAY"}]`
- `107:41534` Action / You can’t delete this record → `[{"type":"NODE","target":"107:40238","navigation":"OVERLAY"}]`
- `107:41535` Action / A newer version is available → `[{"type":"NODE","target":"107:40264","navigation":"OVERLAY"}]`
- `107:41537` Action / Instrument connection unavailable → `[{"type":"NODE","target":"107:40290","navigation":"OVERLAY"}]`
- `107:41538` Action / Delete · failed, draft retained → `[{"type":"NODE","target":"107:40437","navigation":"OVERLAY"}]`
- `107:41541` Action / Archive · confirm → `[{"type":"NODE","target":"107:40316","navigation":"OVERLAY"}]`
- `107:41542` Action / Delete · ID confirmation required → `[{"type":"NODE","target":"107:40363","navigation":"OVERLAY"}]`
- `107:41543` Action / Delete · ID confirmed → `[{"type":"NODE","target":"107:40390","navigation":"OVERLAY"}]`
- `107:41544` Action / Delete · blocked by linked records → `[{"type":"NODE","target":"107:40414","navigation":"OVERLAY"}]`
- `107:41547` Action / Notebook · results submitted → `[{"type":"NODE","target":"107:40483","navigation":"NAVIGATE"}]`
- `107:41548` Action / Inventory · insufficient volume → `[{"type":"NODE","target":"107:40680","navigation":"NAVIGATE"}]`
- `107:41549` Action / Automation · run failed → `[{"type":"NODE","target":"107:41089","navigation":"NAVIGATE"}]`
- `107:41550` Action / Registry · delete confirmation → `[{"type":"NODE","target":"107:41291","navigation":"NAVIGATE"}]`
- `107:41552` Action / Preview success toast → `[{"type":"NODE","target":"107:41497","navigation":"OVERLAY"}]`

## ST-279 · UX v4 flow / ImportSource

**Role:** Linked functional state. **Frame:** 110:19645 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=110-19645).

**Displayed content:**

- Open a sequence
- Close
- Choose a local file or paste a sequence. Review the recognized content before opening.
- Choose an import source
- Select the file format or paste input to review.
- HelixBench document
- GenBank file
- FASTA file
- Multiple records
- Paste sequence
- Cancel

**Authored actions:**

- `110:19649` Action / Close → `[{"type":"BACK"}]`
- `126:5380` Action / HelixBench document → `[{"type":"NODE","target":"110:19671","navigation":"NAVIGATE"}]`
- `126:5382` Action / GenBank file → `[{"type":"NODE","target":"126:5308","navigation":"NAVIGATE"}]`
- `126:5385` Action / FASTA file → `[{"type":"NODE","target":"126:5342","navigation":"NAVIGATE"}]`
- `126:5387` Action / Multiple records → `[{"type":"NODE","target":"126:5392","navigation":"NAVIGATE"}]`
- `126:5390` Action / Paste sequence → `[{"type":"NODE","target":"110:20202","navigation":"NAVIGATE"}]`
- `110:19667` Action / Cancel → `[{"type":"NODE","target":"107:29237","navigation":"NAVIGATE"}]`

## ST-280 · ImportReview

**Role:** Linked functional state. **Frame:** 110:19671 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=110-19671).

**Displayed content:**

- Review sequence import
- Confirm what was recognized before opening your document.
- Search
- New
- pHelixBench-Demo.hbx
- HelixBench document · 1 sequence · includes saved history.
- Recognized property
- Value
- Action
- Name / stable ID
- pHelixBench-Demo · DNA-001
- Retained from document
- Type / length
- DNA · 3021 bp
- Confirm recognized type
- Topology
- Circular
- Annotations
- Features and primer links
- Preserved
- Document version
- v7 · saved checkpoint
- History retained
- Sequence changes
- None
- Original bases retained
- Ready to open. No sequence symbols will be removed or changed.
- Back to source
- Open this sequence
- What happens next?
- Inspect the sequence, then run a
restriction analysis. Registration
and inventory setup are optional.
- Import help

**Authored actions:**

- `110:19778` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `110:19779` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `110:19867` Action / Back to source → `[{"type":"NODE","target":"110:19645","navigation":"NAVIGATE"}]`
- `110:19869` Action / Open this sequence → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`
- `110:19879` Action / Import help → `[{"type":"NODE","target":"110:21252","navigation":"NAVIGATE"}]`

## ST-281 · UX v4 flow / SampleReview

**Role:** Linked functional state. **Frame:** 110:19881 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=110-19881).

**Displayed content:**

- Try a sample sequence
- Close
- Sample data is for learning. It is not your imported file.
- Sample
- pHelixBench-Demo · SAMPLE-DNA-001 · v7
- Input
- DNA · 3021 bp · circular
- Try this task
- Choose Restriction analysis and inspect predicted fragments
- Personal work
- Keep sample records separate from personal records
- Cancel
- Open labeled sample

**Authored actions:**

- `110:19885` Action / Close → `[{"type":"BACK"}]`
- `110:19907` Action / Cancel → `[{"type":"NODE","target":"107:29237","navigation":"NAVIGATE"}]`
- `110:19909` Action / Open labeled sample → `[{"type":"NODE","target":"110:19911","navigation":"NAVIGATE"}]`

## ST-282 · SampleMap

**Role:** Linked functional state. **Frame:** 110:19911 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=110-19911).

**Displayed content:**

- Map
- Inspect a construct and act on the selected region.
- Exit sample
- Sample workspace · changes here are practice data. Import your own file from Home.
- pHelixBench-Demo
- v7 · saved 10:42
- Undo
- Save version
- Export
- Map · guided sample
- Analyze
- Circular map
- Selection 97–116 · forward
- 3021 bp · circular
- GC 54.5%
- drag to select
- T7 promoter
- lac operator
- RBS
- demo_cds
- Demo-Rev
- rrnB T1 terminator
- M13 fwd site
- pUC19 MCS
- M13 rev site
- ori (demo)
- demo_marker
- loxP
- FRT
- Kozak
- EcoRI
- BamHI
- ScaI
- AhdI
- SexAI
- PshAI
- DraI
- NcoI
- T7 promoter selected. Create and analysis tools retain this document and range.
- Sample workspace
- SAMPLE-DNA-001 · practice data.
Save a personal copy before using
it for your own work.
- Restriction analysis
- Save a personal copy
- Sample data · not a personal record

**Authored actions:**

- `110:20018` Action / Search → `[{"type":"NODE","target":"126:6528","navigation":"NAVIGATE"}]`
- `110:20025` Action / Save version → `[{"type":"NODE","target":"107:29211","navigation":"OVERLAY"}]`
- `110:20026` Action / Export → `[{"type":"NODE","target":"107:27142","navigation":"OVERLAY"}]`
- `110:20037` Action / Analyze → `[{"type":"NODE","target":"126:6169","navigation":"NAVIGATE"}]`
- `126:6164` Action / Restriction analysis → `[{"type":"NODE","target":"126:6169","navigation":"NAVIGATE"}]`
- `126:6167` Action / Save a personal copy → `[{"type":"NODE","target":"126:6556","navigation":"NAVIGATE"}]`

## ST-283 · UX v4 flow / ImportIssues

**Role:** Linked functional state. **Frame:** 110:20168 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=110-20168).

**Displayed content:**

- Review sequence issues
- Close
- Original input is retained. Resolve ambiguous symbols before opening.
- Sequence not opened. No bases have been silently removed.
- File
- example_sequence.fasta · record 1
- Issue
- Position 142: unsupported symbol “?”
- Proposed change
- No automatic change applied
- Recovery
- Edit the source or paste corrected sequence; review again
- Back to source
- Review corrected example

**Authored actions:**

- `110:20172` Action / Close → `[{"type":"BACK"}]`
- `110:20198` Action / Back to source → `[{"type":"NODE","target":"110:19645","navigation":"NAVIGATE"}]`
- `110:20200` Action / Review corrected example → `[{"type":"NODE","target":"110:19671","navigation":"NAVIGATE"}]`

## ST-284 · UX v4 flow / BlankSequence

**Role:** Linked functional state. **Frame:** 110:20202 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=110-20202).

**Displayed content:**

- Create a sequence
- Close
- Choose a type and topology explicitly. Review pasted bases before creation.
- Name
- Untitled sequence
- Sequence type
- DNA · select RNA or protein if needed
- Topology
- Linear · change if circular
- Sequence
- Paste your sequence here
- Cancel
- Review input

**Authored actions:**

- `110:20206` Action / Close → `[{"type":"BACK"}]`
- `110:20228` Action / Cancel → `[{"type":"NODE","target":"107:29237","navigation":"NAVIGATE"}]`
- `110:20230` Action / Review input → `[{"type":"NODE","target":"125:6417","navigation":"NAVIGATE"}]`

## ST-285 · UX v4 flow / BlankEntry

**Role:** Linked functional state. **Frame:** 110:20232 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=110-20232).

**Displayed content:**

- New notebook entry
- Close
- Start blank, or deliberately choose a template.
- Title
- Untitled experiment
- Project
- Choose a project · optional for local draft
- Linked input
- Choose an existing sequence or record
- Notes
- Blank · your content starts here
- Cancel
- Choose a template
- Create draft

**Authored actions:**

- `110:20236` Action / Close → `[{"type":"BACK"}]`
- `110:20258` Action / Cancel → `[{"type":"NODE","target":"107:30702","navigation":"NAVIGATE"}]`
- `110:20260` Action / Choose a template → `[{"type":"NODE","target":"125:6940","navigation":"NAVIGATE"}]`
- `110:20262` Action / Create draft → `[{"type":"NODE","target":"110:20264","navigation":"NAVIGATE"}]`

## ST-286 · UX v4 flow / DraftCreated

**Role:** Linked functional state. **Frame:** 110:20264 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=110-20264).

**Displayed content:**

- Notebook draft created
- Close
- Your draft has no submitted result rows.
- Entry
- Untitled experiment · local draft
- Next step
- Open the editor, link an input and write your experiment
- Save versus submit
- Saving text does not submit a result table
- Open notebook

**Authored actions:**

- `110:20268` Action / Close → `[{"type":"BACK"}]`
- `110:20286` Action / Open notebook → `[{"type":"NODE","target":"125:6756","navigation":"NAVIGATE"}]`

## ST-287 · AnalysisSetup

**Role:** Linked functional state. **Frame:** 110:20288 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=110-20288).

**Displayed content:**

- Restriction analysis
- Confirm your input and review recommended settings.
- Search
- New
- Input: DNA-001 · pHelixBench-Demo
- Version 7 · 3021 bp · circular · full sequence, not only the selected promoter.
- Recommended setup
- Enzyme selection
- EcoRI + BamHI
- Analysis scope
- Whole circular sequence · 3021 bp
- Both enzymes have one predicted cut site in this example input. Settings are suggestions; review assumptions before
interpretation.
- Advanced settings
- Why these settings?
- The result will retain input version, enzymes, scope and parameters.
- Back to sequence
- Run restriction analysis
- Input & prerequisites
- Restriction analysis needs a valid
DNA sequence and selected enzymes.
Your entire input is in scope; no
selected region is required.
- Review source sequence
- Choose another enzyme set

**Authored actions:**

- `110:20395` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `110:20396` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `110:20463` Action / Advanced settings → `[{"type":"NODE","target":"110:20486","navigation":"NAVIGATE"}]`
- `110:20465` Action / Why these settings? → `[{"type":"NODE","target":"110:20514","navigation":"NAVIGATE"}]`
- `110:20472` Action / Back to sequence → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`
- `110:20474` Action / Run restriction analysis → `[{"type":"NODE","target":"110:20538","navigation":"NAVIGATE"}]`
- `110:20482` Action / Review source sequence → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`
- `110:20484` Action / Choose another enzyme set → `[{"type":"NODE","target":"107:26978","navigation":"NAVIGATE"}]`

## ST-288 · UX v4 flow / AnalysisAdvanced

**Role:** Linked functional state. **Frame:** 110:20486 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=110-20486).

**Displayed content:**

- Advanced restriction settings
- Close
- Review scientific assumptions. These are illustrative prototype parameters.
- Input
- DNA-001 · v7 · whole circular sequence
- Enzymes
- EcoRI and BamHI
- Methylation / ambiguity
- Review enzyme sensitivity and ambiguous-base handling
- Output
- Predicted fragments and gel preview
- Back to setup

**Authored actions:**

- `110:20490` Action / Close → `[{"type":"BACK"}]`
- `110:20512` Action / Back to setup → `[{"type":"NODE","target":"110:20288","navigation":"NAVIGATE"}]`

## ST-289 · UX v4 flow / AnalysisHelp

**Role:** Linked functional state. **Frame:** 110:20514 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=110-20514).

**Displayed content:**

- Understanding restriction analysis
- Close
- A prediction describes expected cut positions and fragment lengths for the specified input and
enzyme set.
- Scope
- Whole sequence includes every base, regardless of current map selection
- Interpretation
- Check fragment lengths sum to input length for this circular example
- Limitations
- An in-silico result does not guarantee an experimental outcome
- Back to setup

**Authored actions:**

- `110:20518` Action / Close → `[{"type":"BACK"}]`
- `110:20536` Action / Back to setup → `[{"type":"NODE","target":"110:20288","navigation":"NAVIGATE"}]`

## ST-290 · UX v4 flow / AnalysisRunning

**Role:** Linked functional state. **Frame:** 110:20538 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=110-20538).

**Displayed content:**

- Restriction analysis is running
- Close
- Keep the input and operation identity visible while work is in progress.
- Operation
- OP-001 · DNA-001 v7
- Stage
- Calculating fragments · progress indeterminate
- Input
- EcoRI + BamHI · whole sequence
- Cancellation
- Cancel stops this operation before result creation
- Cancel analysis
- Preview completed state
- Preview failed state

**Authored actions:**

- `110:20542` Action / Close → `[{"type":"BACK"}]`
- `110:20564` Action / Cancel analysis → `[{"type":"NODE","target":"110:20288","navigation":"NAVIGATE"}]`
- `110:20566` Action / Preview completed state → `[{"type":"NODE","target":"110:20596","navigation":"NAVIGATE"}]`
- `110:20568` Action / Preview failed state → `[{"type":"NODE","target":"110:20570","navigation":"NAVIGATE"}]`

## ST-291 · UX v4 flow / AnalysisFailed

**Role:** Linked functional state. **Frame:** 110:20570 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=110-20570).

**Displayed content:**

- Analysis could not finish
- Close
- No result was created. Your input and settings are retained.
- Operation
- OP-001 · DNA-001 v7
- Problem
- The analysis service did not respond
- Recovery
- Check connection, then retry with the same input and settings
- Return to setup
- Retry analysis

**Authored actions:**

- `110:20574` Action / Close → `[{"type":"BACK"}]`
- `110:20592` Action / Return to setup → `[{"type":"NODE","target":"110:20288","navigation":"NAVIGATE"}]`
- `110:20594` Action / Retry analysis → `[{"type":"NODE","target":"110:20538","navigation":"NAVIGATE"}]`

## ST-292 · AnalysisResult

**Role:** Linked functional state. **Frame:** 110:20596 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=110-20596).

**Displayed content:**

- Restriction analysis result
- RES-001 · pHelixBench-Demo · input DNA-001 v7
- Search
- New
- Two predicted fragments
- EcoRI at 216 · BamHI at 234 · circular input length 3021 bp.
- Fragment
- Length
- Interpretation
- Fragment 1
- 18 bp
- Between the two selected cut positions
- Fragment 2
- 3003 bp
- Remaining circular backbone
- Total
- 3021 bp
- Matches input length
- Illustrative result fixture. Verify predictions and experimental assumptions before laboratory use.
- Input and settings stay attached
- DNA-001 · v7 · whole sequence · EcoRI + BamHI · RES-001
- Edit settings
- View source
- Save result
- Read the result
- Fragment lengths describe this
input and enzyme set. The small
fragment may need different
experimental handling.
- Parameter summary
- How to interpret this

**Authored actions:**

- `110:20703` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `110:20704` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `110:20783` Action / Edit settings → `[{"type":"NODE","target":"110:20288","navigation":"NAVIGATE"}]`
- `110:20785` Action / View source → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`
- `110:20787` Action / Save result → `[{"type":"NODE","target":"110:20799","navigation":"NAVIGATE"}]`
- `110:20795` Action / Parameter summary → `[{"type":"NODE","target":"110:20486","navigation":"NAVIGATE"}]`
- `110:20797` Action / How to interpret this → `[{"type":"NODE","target":"110:20514","navigation":"NAVIGATE"}]`

## ST-293 · UX v4 flow / SaveResult

**Role:** Linked functional state. **Frame:** 110:20799 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=110-20799).

**Displayed content:**

- Save this analysis result
- Close
- Keep a named output linked to its source.
- Result name
- Restriction analysis · EcoRI + BamHI
- Result ID
- RES-001
- Source
- DNA-001 · v7 · whole sequence
- Location
- Local workspace / Analyses
- Cancel
- Save named result

**Authored actions:**

- `110:20803` Action / Close → `[{"type":"BACK"}]`
- `110:20825` Action / Cancel → `[{"type":"NODE","target":"110:20596","navigation":"NAVIGATE"}]`
- `110:20827` Action / Save named result → `[{"type":"NODE","target":"110:20829","navigation":"NAVIGATE"}]`

## ST-294 · SavedResult

**Role:** Linked functional state. **Frame:** 110:20829 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=110-20829).

**Displayed content:**

- Result saved
- RES-001 · Local workspace / Analyses
- Search
- New
- Restriction analysis · EcoRI + BamHI was saved with input DNA-001 v7.
- Find it again
- Use Home recent work, workspace search, or the source sequence’s related results.
- View result
- Go to Home
- Find by ID
- What was saved?
- Result identity, source version,
scope and settings are retained
together.
- Open source
- Open parameter summary

**Authored actions:**

- `110:20936` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `110:20937` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `110:20996` Action / View result → `[{"type":"NODE","target":"110:20596","navigation":"NAVIGATE"}]`
- `110:20998` Action / Go to Home → `[{"type":"NODE","target":"107:29237","navigation":"NAVIGATE"}]`
- `110:21000` Action / Find by ID → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `110:21008` Action / Open source → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`
- `110:21010` Action / Open parameter summary → `[{"type":"NODE","target":"110:20486","navigation":"NAVIGATE"}]`

## ST-295 · WorkspaceSearch

**Role:** Linked functional state. **Frame:** 110:21012 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=110-21012).

**Displayed content:**

- Search your workspace
- Search by name or stable ID. Results retain their record type and context.
- Search
- New
- Query
- DNA-001
- All records
- Sequences
- Results
- Record
- Type / scope
- Open
- pHelixBench-Demo · DNA-001
- Sequence · v7
- Sequence workspace
- Restriction analysis · RES-001
- Analysis · input DNA-001 v7
- Saved result
- Back to Home
- Clear search
- Recognize the right record
- Check type, ID and source before
opening a result. Your query is
retained when you return.
- Search help

**Authored actions:**

- `110:21120` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `110:21178` Action / Sequences → `[{"type":"NODE","target":"125:5843","navigation":"NAVIGATE"}]`
- `110:21180` Action / Results → `[{"type":"NODE","target":"125:6032","navigation":"NAVIGATE"}]`
- `110:21187` Record / pHelixBench-Demo · DNA-001 → `[{"type":"NODE","target":"130:4410","navigation":"NAVIGATE"}]`
- `110:21191` Record / Restriction analysis · RES-001 → `[{"type":"NODE","target":"130:4669","navigation":"NAVIGATE"}]`
- `110:21200` Action / Back to Home → `[{"type":"NODE","target":"107:29237","navigation":"NAVIGATE"}]`
- `125:6415` Action / Clear search → `[{"type":"NODE","target":"125:6221","navigation":"NAVIGATE"}]`
- `110:21208` Action / Search help → `[{"type":"NODE","target":"110:21252","navigation":"NAVIGATE"}]`

## ST-296 · UX v4 flow / NoMatches

**Role:** Reference or alternate fixture. **Frame:** 110:21210 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=110-21210).

**Displayed content:**

- No matching records
- Close
- Search completed. No accessible records match this query.
- Query
- DNA-999
- Try next
- Check the ID or broaden the search scope
- Edit search

**Authored actions:**

- `110:21214` Action / Close → `[{"type":"BACK"}]`
- `110:21228` Action / Edit search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`

## ST-297 · UX v4 flow / SearchFailed

**Role:** Reference or alternate fixture. **Frame:** 110:21230 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=110-21230).

**Displayed content:**

- Search could not load
- Close
- This is a connection problem, not a no-results response.
- Query
- DNA-001 · retained
- Recovery
- Retry, or return to your current work
- Return to search
- Retry search

**Authored actions:**

- `110:21234` Action / Close → `[{"type":"BACK"}]`
- `110:21248` Action / Return to search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `110:21250` Action / Retry search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`

## ST-298 · TaskHelp

**Role:** Linked functional state. **Frame:** 110:21252 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=110-21252).

**Displayed content:**

- Help for your current task
- Short explanations, examples and deeper guidance without losing your work.
- Search
- New
- From a sequence to a result
- Import → review recognized content → inspect → analyze → save → reopen.
- Task
- What you need
- Open
- Open a file
- Supported sequence or HelixBench document
- Import help
- Run restriction analysis
- DNA input and enzyme set
- Analysis setup
- Find saved work
- Name or stable record ID
- Workspace search
- Understand product areas
- Sequence, Registry, Inventory and Notebook
- Workspace guide
- Return to Home
- Return to sequence
- Get help where you work
- Import and analysis setup have
their own help. Closing help
returns to the task; it never
reports a save.
- Analysis explanation

**Authored actions:**

- `110:21359` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `110:21360` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `110:21419` Record / Open a file → `[{"type":"NODE","target":"110:19645","navigation":"NAVIGATE"}]`
- `110:21423` Record / Run restriction analysis → `[{"type":"NODE","target":"110:20288","navigation":"NAVIGATE"}]`
- `110:21427` Record / Find saved work → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `110:21431` Record / Understand product areas → `[{"type":"NODE","target":"110:21450","navigation":"NAVIGATE"}]`
- `110:21436` Action / Return to Home → `[{"type":"NODE","target":"107:29237","navigation":"NAVIGATE"}]`
- `110:21438` Action / Return to sequence → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`
- `110:21446` Action / Import help → `[{"type":"NODE","target":"110:19645","navigation":"NAVIGATE"}]`
- `110:21448` Action / Analysis explanation → `[{"type":"NODE","target":"110:20514","navigation":"NAVIGATE"}]`

## ST-299 · UX v4 flow / WorkspaceGuide

**Role:** Linked functional state. **Frame:** 110:21450 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=110-21450).

**Displayed content:**

- How your work fits together
- Close
- Use only the areas needed for your current task.
- Sequence workspace
- Inspect and analyze sequence documents
- Registry
- Scientific identity and relationships between records
- Inventory
- Physical samples, quantities and storage locations
- Notebook
- Experiment notes and submitted result rows
- Back to help
- Open sequence

**Authored actions:**

- `110:21454` Action / Close → `[{"type":"BACK"}]`
- `110:21476` Action / Back to help → `[{"type":"NODE","target":"110:21252","navigation":"NAVIGATE"}]`
- `110:21478` Action / Open sequence → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`

## ST-300 · UX v4 flow / UndoResult

**Role:** Linked functional state. **Frame:** 110:21480 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=110-21480).

**Displayed content:**

- Edit undone
- Close
- The last unsaved sequence edit was reversed.
- Input
- DNA-001 · v7 checkpoint
- Restored context
- T7 promoter · 97–116 · forward
- Saved versions
- Undo does not replace saved version history
- Return to sequence

**Authored actions:**

- `110:21484` Action / Close → `[{"type":"BACK"}]`
- `110:21502` Action / Return to sequence → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`

## ST-301 · UX v4 flow / Entity-DNA-001

**Role:** Linked functional state. **Frame:** 110:21504 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=110-21504).

**Displayed content:**

- pHelixBench-Demo · DNA-001
- Close
- The selected scientific record retains its own identity.
- Stable ID
- DNA-001
- Record type
- DNA construct
- Scientific identity
- 3021 bp · circular
- Physical material
- Open Inventory for quantities and storage
- Back to Registry
- Open workspace
- Archive record

**Authored actions:**

- `110:21508` Action / Close → `[{"type":"BACK"}]`
- `110:21530` Action / Back to Registry → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`
- `110:21532` Action / Open workspace → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`
- `110:21534` Action / Archive record → `[{"type":"NODE","target":"125:5170","navigation":"NAVIGATE"}]`

## ST-302 · UX v4 flow / Entity-PR-001

**Role:** Linked functional state. **Frame:** 110:21536 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=110-21536).

**Displayed content:**

- T7-F · PR-001
- Close
- The selected scientific record retains its own identity.
- Stable ID
- PR-001
- Record type
- Primer
- Scientific identity
- 20 nt · forward
- Physical material
- Open Inventory for quantities and storage
- Back to Registry
- Open workspace
- Archive record

**Authored actions:**

- `110:21540` Action / Close → `[{"type":"BACK"}]`
- `110:21562` Action / Back to Registry → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`
- `110:21564` Action / Open workspace → `[{"type":"NODE","target":"107:30093","navigation":"NAVIGATE"}]`
- `110:21566` Action / Archive record → `[{"type":"NODE","target":"125:5240","navigation":"NAVIGATE"}]`

## ST-303 · UX v4 flow / Entity-AB-014

**Role:** Linked functional state. **Frame:** 110:21568 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=110-21568).

**Displayed content:**

- AB-014 · AB-014
- Close
- The selected scientific record retains its own identity.
- Stable ID
- AB-014
- Record type
- Biologic
- Scientific identity
- IgG · HC-014 and LC-014
- Physical material
- Open Inventory for quantities and storage
- Back to Registry
- Open workspace
- Archive record

**Authored actions:**

- `110:21572` Action / Close → `[{"type":"BACK"}]`
- `110:21594` Action / Back to Registry → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`
- `110:21596` Action / Open workspace → `[{"type":"NODE","target":"107:32270","navigation":"NAVIGATE"}]`
- `110:21598` Action / Archive record → `[{"type":"NODE","target":"125:5310","navigation":"NAVIGATE"}]`

## ST-304 · UX v4 flow / Entity-OL-014

**Role:** Linked functional state. **Frame:** 110:21600 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=110-21600).

**Displayed content:**

- OL-014 · OL-014
- Close
- The selected scientific record retains its own identity.
- Stable ID
- OL-014
- Record type
- RNA oligo
- Scientific identity
- RNA · modification pattern
- Physical material
- Open Inventory for quantities and storage
- Back to Registry
- Open workspace
- Archive record

**Authored actions:**

- `110:21604` Action / Close → `[{"type":"BACK"}]`
- `110:21626` Action / Back to Registry → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`
- `110:21628` Action / Open workspace → `[{"type":"NODE","target":"107:32649","navigation":"NAVIGATE"}]`
- `110:21630` Action / Archive record → `[{"type":"NODE","target":"125:5380","navigation":"NAVIGATE"}]`

## ST-305 · UX v4 flow / ArchiveReview

**Role:** Linked functional state. **Frame:** 110:21637 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=110-21637).

**Displayed content:**

- Archive DNA-001?
- Close
- The record leaves active views. Review linked material before proceeding.
- Record
- DNA-001 · pHelixBench-Demo
- Impact
- Linked notebook references remain identifiable
- Recovery
- Restore from archived records when supported
- Cancel
- Archive record

**Authored actions:**

- `110:21641` Action / Close → `[{"type":"BACK"}]`
- `110:21659` Action / Cancel → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`
- `110:21661` Action / Archive record → `[{"type":"NODE","target":"110:21663","navigation":"NAVIGATE"}]`

## ST-306 · UX v4 flow / ArchivedResult

**Role:** Linked functional state. **Frame:** 110:21663 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=110-21663).

**Displayed content:**

- DNA-001 archived
- Close
- The record is retained and excluded from active views.
- Record
- DNA-001 · pHelixBench-Demo
- Next step
- Return to Registry or review the record
- Return to Registry
- View archived record

**Authored actions:**

- `110:21667` Action / Close → `[{"type":"BACK"}]`
- `110:21681` Action / Return to Registry → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`
- `110:21683` Action / View archived record → `[{"type":"NODE","target":"110:21504","navigation":"NAVIGATE"}]`

## ST-307 · UX v4 flow / DeleteReview

**Role:** Linked functional state. **Frame:** 110:21685 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=110-21685).

**Displayed content:**

- Review permanent deletion
- Close
- This action requires dependency review and explicit confirmation.
- Prototype preview: the next screen demonstrates valid confirmation; it does not
simulate typing.
- Record
- DNA-001 · pHelixBench-Demo
- Affected context
- Review linked results, notebook references and samples
- Confirmation
- Type DNA-001 to enable permanent deletion
- Current state
- Confirmation is empty · deletion unavailable
- Cancel
- Preview confirmed state

**Authored actions:**

- `110:21689` Action / Close → `[{"type":"BACK"}]`
- `110:21715` Action / Cancel → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`
- `110:21717` Action / Preview confirmed state → `[{"type":"NODE","target":"110:21719","navigation":"NAVIGATE"}]`

## ST-308 · UX v4 flow / DeleteConfirmed

**Role:** Linked functional state. **Frame:** 110:21719 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=110-21719).

**Displayed content:**

- Delete DNA-001 permanently?
- Close
- Confirmation matches DNA-001. This action cannot be undone.
- Confirmation
- DNA-001 · matches selected ID
- Before deletion
- Resolve blocking dependencies; preserve required records
- Scope
- This record only · no silent cascade deletion
- Keep record
- Preview blocked outcome
- Confirm deletion

**Authored actions:**

- `110:21723` Action / Close → `[{"type":"BACK"}]`
- `110:21741` Action / Keep record → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`
- `110:21743` Action / Preview blocked outcome → `[{"type":"NODE","target":"110:21747","navigation":"NAVIGATE"}]`
- `110:21745` Action / Confirm deletion → `[{"type":"NODE","target":"110:21775","navigation":"NAVIGATE"}]`

## ST-309 · UX v4 flow / DeleteBlocked

**Role:** Linked functional state. **Frame:** 110:21747 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=110-21747).

**Displayed content:**

- Deletion blocked
- Close
- A linked result prevents permanent deletion. The record is unchanged.
- Record
- DNA-001
- Dependency
- RES-001 · Restriction analysis
- Recovery
- Review linked result, or archive the record instead
- Keep record
- Review linked result
- Archive instead

**Authored actions:**

- `110:21751` Action / Close → `[{"type":"BACK"}]`
- `110:21769` Action / Keep record → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`
- `110:21771` Action / Review linked result → `[{"type":"NODE","target":"110:20596","navigation":"NAVIGATE"}]`
- `110:21773` Action / Archive instead → `[{"type":"NODE","target":"110:21637","navigation":"NAVIGATE"}]`

## ST-310 · UX v4 flow / DeletedResult

**Role:** Linked functional state. **Frame:** 110:21775 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=110-21775).

**Displayed content:**

- DNA-001 deleted
- Close
- Deletion completed for the reviewed record.
- Deleted ID
- DNA-001
- Linked material
- No additional records were silently deleted
- Return to Registry

**Authored actions:**

- `110:21779` Action / Close → `[{"type":"BACK"}]`
- `110:21793` Action / Return to Registry → `[{"type":"NODE","target":"110:21795","navigation":"NAVIGATE"}]`

## ST-311 · RegistryAfterDelete

**Role:** Linked functional state. **Frame:** 110:21795 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=110-21795).

**Displayed content:**

- Registry
- DNA-001 deleted · other records remain
- Search
- New
- Record
- Type
- Open
- T7-F · PR-001
- Primer
- Primer workspace
- AB-014
- Biologic
- Biologics workspace
- OL-014
- RNA oligo
- RNA / Oligos workspace
- No record selected
- DNA-001 was deleted. Select another
record to see its details and
available actions.

**Authored actions:**

- `110:21902` Action / Search → `[{"type":"NODE","target":"125:5668","navigation":"NAVIGATE"}]`
- `110:21903` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `110:22004` Record / T7-F · PR-001 → `[{"type":"NODE","target":"132:5711","navigation":"NAVIGATE"}]`
- `110:22008` Record / AB-014 → `[{"type":"NODE","target":"110:21568","navigation":"NAVIGATE"}]`
- `110:22012` Record / OL-014 → `[{"type":"NODE","target":"110:21600","navigation":"NAVIGATE"}]`

## ST-312 · UX v4 flow / GroupedTools

**Role:** Linked functional state. **Frame:** 110:22016 · 1000 × 1200. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=110-22016).

**Displayed content:**

- Sequence tools
- DNA-001 · v7 · T7 promoter 97–116 · forward strand
- Inspect & select
- Find bases or a motif
- Readable feature list
- Map display options
- Edit selected region
- Edit sequence · 97–116
- Add an annotation
- Add primer from range
- Analyze
- Restriction analysis · whole sequence
- Design primer pair · requires target region
- Simulate PCR · requires a linked pair
- Align sequences · requires two inputs
- Advanced & reference
- Assembly wizard
- CRISPR guide design
- BLAST search
- Protein analysis
- Codon optimization
- Detect common features
- Feature library
- Combinatorial assembly
- Zoom / fit / center
- Rotate origin / topology
- Color selection
- Genetic code
- Nucleotide letter help
- Back to sequence
- Help for this task

**Authored actions:**

- `110:22021` Action / Find bases or a motif → `[{"type":"NODE","target":"107:33290","navigation":"NAVIGATE"}]`
- `110:22023` Action / Readable feature list → `[{"type":"NODE","target":"107:29886","navigation":"NAVIGATE"}]`
- `110:22026` Action / Map display options → `[{"type":"NODE","target":"107:25217","navigation":"NAVIGATE"}]`
- `110:22030` Action / Edit sequence · 97–116 → `[{"type":"NODE","target":"107:29125","navigation":"NAVIGATE"}]`
- `110:22032` Action / Add an annotation → `[{"type":"NODE","target":"107:25134","navigation":"NAVIGATE"}]`
- `110:22035` Action / Add primer from range → `[{"type":"NODE","target":"107:25182","navigation":"NAVIGATE"}]`
- `110:22039` Action / Restriction analysis · whole sequence → `[{"type":"NODE","target":"110:20288","navigation":"NAVIGATE"}]`
- `110:22041` Action / Design primer pair · requires target region → `[{"type":"NODE","target":"107:25303","navigation":"NAVIGATE"}]`
- `110:22044` Action / Simulate PCR · requires a linked pair → `[{"type":"NODE","target":"107:25266","navigation":"NAVIGATE"}]`
- `110:22046` Action / Align sequences · requires two inputs → `[{"type":"NODE","target":"107:25425","navigation":"NAVIGATE"}]`
- `110:22050` Action / Assembly wizard → `[{"type":"NODE","target":"107:25463","navigation":"NAVIGATE"}]`
- `110:22052` Action / CRISPR guide design → `[{"type":"NODE","target":"107:25505","navigation":"NAVIGATE"}]`
- `110:22055` Action / BLAST search → `[{"type":"NODE","target":"107:25595","navigation":"NAVIGATE"}]`
- `118:3392` Action / Protein analysis → `[{"type":"NODE","target":"126:7729","navigation":"NAVIGATE"}]`
- `118:3395` Action / Codon optimization → `[{"type":"NODE","target":"107:25557","navigation":"NAVIGATE"}]`
- `118:3399` Action / Detect common features → `[{"type":"NODE","target":"107:26060","navigation":"NAVIGATE"}]`
- `118:3402` Action / Feature library → `[{"type":"NODE","target":"107:26194","navigation":"NAVIGATE"}]`
- `118:3406` Action / Combinatorial assembly → `[{"type":"NODE","target":"107:26112","navigation":"NAVIGATE"}]`
- `118:3409` Action / Zoom / fit / center → `[{"type":"NODE","target":"107:25217","navigation":"NAVIGATE"}]`
- `118:3413` Action / Rotate origin / topology → `[{"type":"NODE","target":"107:26038","navigation":"NAVIGATE"}]`
- `118:3416` Action / Color selection → `[{"type":"NODE","target":"107:26086","navigation":"NAVIGATE"}]`
- `118:3420` Action / Genetic code → `[{"type":"NODE","target":"107:25938","navigation":"NAVIGATE"}]`
- `118:3423` Action / Nucleotide letter help → `[{"type":"NODE","target":"107:25960","navigation":"NAVIGATE"}]`
- `110:22060` Action / Back to sequence → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`
- `110:22062` Action / Help for this task → `[{"type":"NODE","target":"110:21252","navigation":"NAVIGATE"}]`

## ST-313 · Start here / UX v4

**Role:** Linked functional state. **Frame:** 110:22064 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=110-22064).

**Displayed content:**

- HelixBench · Soft Gallery UX v4
- A complete first-task path, clearer tool discovery, and consistent record context.
- Search
- New
- Start with the first-use journey
- Use the example import to review recognized content and complete a restriction analysis.
- Open first-use Home
- Open populated Home
- Review import
- Review the key improvements
- Every screen is editable. Linked states represent a fixed prototype fixture, not a functioning scientific application.
- Grouped tools
- Search & reopen
- Analysis result
- Registry identities
- Archive / delete
- Task help
- Implementation and research checks
- Keyboard behavior, file parsing, dynamic validation and scientific computations require implementation and participant testing.
- Feature entry target: within two clicks from its owning page. Data entry, file selection and safe confirmations are
counted separately.
- Review completed branches
- Review scope
- 19 module screens retained and
improved. New first-use, analysis,
search, help and recovery states
connect the audit recommendations.
- Begin first-use journey
- Open saved-result journey

**Authored actions:**

- `110:22171` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `110:22172` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `110:22227` Action / Open first-use Home → `[{"type":"NODE","target":"107:37024","navigation":"NAVIGATE"}]`
- `110:22229` Action / Open populated Home → `[{"type":"NODE","target":"107:29237","navigation":"NAVIGATE"}]`
- `110:22231` Action / Review import → `[{"type":"NODE","target":"110:19671","navigation":"NAVIGATE"}]`
- `110:22237` Action / Grouped tools → `[{"type":"NODE","target":"110:22016","navigation":"NAVIGATE"}]`
- `110:22239` Action / Search & reopen → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `110:22241` Action / Analysis result → `[{"type":"NODE","target":"110:20596","navigation":"NAVIGATE"}]`
- `110:22244` Action / Registry identities → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`
- `110:22246` Action / Archive / delete → `[{"type":"NODE","target":"110:21685","navigation":"NAVIGATE"}]`
- `110:22248` Action / Task help → `[{"type":"NODE","target":"110:21252","navigation":"NAVIGATE"}]`
- `128:8487` Action / Review completed branches → `[{"type":"NODE","target":"128:8269","navigation":"NAVIGATE"}]`
- `110:22260` Action / Begin first-use journey → `[{"type":"NODE","target":"107:37024","navigation":"NAVIGATE"}]`
- `110:22262` Action / Open saved-result journey → `[{"type":"NODE","target":"110:20829","navigation":"NAVIGATE"}]`

## ST-314 · UX v4 / Map · unsaved edit

**Role:** Linked functional state. **Frame:** 115:3326 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=115-3326).

**Displayed content:**

- Map
- Inspect a construct and act on the selected region.
- Search
- New
- pHelixBench-Demo
- Unsaved draft · based on v7
- Undo
- Save version
- Export
- Sequence
- Features
- Primers
- Enzymes
- History
- Add feature
- Analyze
- Display layers
- Circular map
- Selection 97–116 · forward
- 3021 bp · circular
- GC 54.5%
- drag to select
- T7 promoter
- lac operator
- RBS
- demo_cds
- Demo-Rev
- rrnB T1 terminator
- M13 fwd site
- pUC19 MCS
- M13 rev site
- ori (demo)
- demo_marker
- loxP
- FRT
- Kozak
- EcoRI
- BamHI
- ScaI
- AhdI
- SexAI
- PshAI
- DraI
- NcoI
- Unsaved sequence edit applied. Undo restores the saved v7 context and selected range.
- Readable annotations
- Restriction analysis
- Task help
- Close document
- Details & tools
- All map tools
- Current selection
- T7 promoter · 97–116
- How to use Map
- Select a feature on the map or use
Features for a readable list. Tools
use the range shown at right.
- Record
- DNA-001 · version 7
- Selected range
- 97–116 · forward strand
- Type
- Promoter · 20 bp
- Actions
- Edit feature
- Add primer from range
- Design primer pair
- Related records
- DNA-001 · Registry
- Plate P-014 · Inventory

**Authored actions:**

- `115:3433` Action / Search → `[{"type":"NODE","target":"125:9306","navigation":"NAVIGATE"}]`
- `115:3434` Action / New → `[{"type":"NODE","target":"125:9358","navigation":"NAVIGATE"}]`
- `115:3439` Action / Undo → `[{"type":"NODE","target":"110:21480","navigation":"NAVIGATE"}]`
- `115:3440` Action / Save version → `[{"type":"NODE","target":"125:8336","navigation":"NAVIGATE"}]`
- `115:3441` Action / Export → `[{"type":"NODE","target":"125:9410","navigation":"NAVIGATE"}]`
- `115:3444` Action / Sequence → `[{"type":"NODE","target":"125:7296","navigation":"NAVIGATE"}]`
- `115:3445` Action / Features → `[{"type":"NODE","target":"125:7508","navigation":"NAVIGATE"}]`
- `115:3446` Action / Primers → `[{"type":"NODE","target":"125:7718","navigation":"NAVIGATE"}]`
- `115:3447` Action / Enzymes → `[{"type":"NODE","target":"125:7923","navigation":"NAVIGATE"}]`
- `115:3448` Action / History → `[{"type":"NODE","target":"125:8131","navigation":"NAVIGATE"}]`
- `115:3451` Action / Add feature → `[{"type":"NODE","target":"125:9462","navigation":"NAVIGATE"}]`
- `115:3452` Action / Analyze → `[{"type":"NODE","target":"125:9514","navigation":"NAVIGATE"}]`
- `115:3453` Action / Display layers → `[{"type":"NODE","target":"125:9566","navigation":"NAVIGATE"}]`
- `115:3532` Action / Readable annotations → `[{"type":"NODE","target":"125:9618","navigation":"NAVIGATE"}]`
- `115:3533` Action / Restriction analysis → `[{"type":"NODE","target":"125:9514","navigation":"NAVIGATE"}]`
- `115:3534` Action / Task help → `[{"type":"NODE","target":"125:9254","navigation":"NAVIGATE"}]`
- `125:10967` Action / Close document → `[{"type":"NODE","target":"125:10969","navigation":"NAVIGATE"}]`
- `115:3549` Action / All map tools → `[{"type":"NODE","target":"125:9670","navigation":"NAVIGATE"}]`
- `115:3566` Action / Edit feature → `[{"type":"NODE","target":"125:9462","navigation":"NAVIGATE"}]`
- `115:3567` Action / Add primer from range → `[{"type":"NODE","target":"125:9722","navigation":"NAVIGATE"}]`
- `115:3568` Action / Design primer pair → `[{"type":"NODE","target":"125:9774","navigation":"NAVIGATE"}]`
- `115:3571` Related / DNA-001 · Registry → `[{"type":"NODE","target":"125:8890","navigation":"NAVIGATE"}]`
- `115:3576` Related / Plate P-014 · Inventory → `[{"type":"NODE","target":"125:8942","navigation":"NAVIGATE"}]`

## ST-315 · Responsive v4 / UX v4 / Home / 1280

**Role:** Responsive/context variant. **Frame:** 115:3588 · 1280 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=115-3588).

**Displayed content:**

- Home
- Continue your work and resolve what needs attention.
- Search
- New
- Details & tools
- Start with your sequence
- Open your file, understand the input, and run your first analysis.
- Import a sequence
- Open GenBank, FASTA or a HelixBench document. Review the recognized sequence before opening it.
- Import sequence
- Try a sample
- No sequence ready? A sample is labeled and kept separate from your own files.
- A short path to your first result
- Review input → inspect sequence → run analysis → save and find it again.
- Create a blank sequence
- New notebook entry
- Continue working
- Open the exact item with its context preserved.
- Work item
- Type
- Next action
- pHelixBench-Demo · DNA-001
- Sequence · v7
- Open sequence
- Restriction analysis · RES-001
- Saved result · DNA-001 v7
- View result
- Expression validation · EXP-014
- Notebook draft
- Continue entry

**Authored actions:**

- `115:3695` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `115:3696` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `115:3756` Action / Details & tools → `[{"type":"NODE","target":"115:3749","navigation":"OVERLAY"}]`
- `115:3705` Action / Import sequence → `[{"type":"NODE","target":"110:19645","navigation":"NAVIGATE"}]`
- `115:3706` Action / Try a sample → `[{"type":"NODE","target":"110:19881","navigation":"NAVIGATE"}]`
- `115:3712` Action / Create a blank sequence → `[{"type":"NODE","target":"110:20202","navigation":"NAVIGATE"}]`
- `115:3713` Action / New notebook entry → `[{"type":"NODE","target":"110:20232","navigation":"NAVIGATE"}]`
- `115:3722` Record / pHelixBench-Demo · DNA-001 → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`
- `115:3726` Record / Restriction analysis · RES-001 → `[{"type":"NODE","target":"110:20596","navigation":"NAVIGATE"}]`
- `115:3730` Record / Expression validation · EXP-014 → `[{"type":"NODE","target":"107:30702","navigation":"NAVIGATE"}]`

## ST-316 · UX v4 context / UX v4 / Home

**Role:** Responsive/context variant. **Frame:** 115:3749 · 320 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=115-3749).

**Displayed content:**

- Your first task
- Import your own sequence, or
explore a labeled sample. You can
skip guidance and return to it
later.
- Import sequence
- Try a sample
- Task help
- Close panel

**Authored actions:**

- `115:3753` Action / Import sequence → `[{"type":"NODE","target":"110:19645","navigation":"NAVIGATE"}]`
- `115:3754` Action / Try a sample → `[{"type":"NODE","target":"110:19881","navigation":"NAVIGATE"}]`
- `115:3755` Action / Task help → `[{"type":"NODE","target":"110:21252","navigation":"NAVIGATE"}]`
- `128:8084` Action / Close panel → `[{"type":"CLOSE"}]`

## ST-317 · Responsive v4 / ImportReview / 1280

**Role:** Responsive/context variant. **Frame:** 115:3759 · 1280 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=115-3759).

**Displayed content:**

- Review sequence import
- Confirm what was recognized before opening your document.
- Search
- New
- Details & tools
- pHelixBench-Demo.hbx
- HelixBench document · 1 sequence · includes saved history.
- Recognized property
- Value
- Action
- Name / stable ID
- pHelixBench-Demo · DNA-001
- Retained from document
- Type / length
- DNA · 3021 bp
- Confirm recognized type
- Topology
- Circular
- Annotations
- Features and primer links
- Preserved
- Document version
- v7 · saved checkpoint
- History retained
- Sequence changes
- None
- Original bases retained
- Ready to open. No sequence symbols will be removed or changed.
- Back to source
- Open this sequence
- Review an invalid file

**Authored actions:**

- `115:3866` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `115:3867` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `115:3926` Action / Details & tools → `[{"type":"NODE","target":"115:3921","navigation":"OVERLAY"}]`
- `115:3906` Action / Back to source → `[{"type":"NODE","target":"110:19645","navigation":"NAVIGATE"}]`
- `115:3907` Action / Open this sequence → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`
- `115:3908` Action / Review an invalid file → `[{"type":"NODE","target":"110:20168","navigation":"NAVIGATE"}]`

## ST-318 · UX v4 context / ImportReview

**Role:** Responsive/context variant. **Frame:** 115:3921 · 320 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=115-3921).

**Displayed content:**

- What happens next?
- Inspect the sequence, then run a
restriction analysis. Registration
and inventory setup are optional.
- Import help
- Close panel

**Authored actions:**

- `115:3925` Action / Import help → `[{"type":"NODE","target":"110:21252","navigation":"NAVIGATE"}]`
- `128:8086` Action / Close panel → `[{"type":"CLOSE"}]`

## ST-319 · Responsive v4 / AnalysisSetup / 1280

**Role:** Responsive/context variant. **Frame:** 115:3929 · 1280 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=115-3929).

**Displayed content:**

- Restriction analysis
- Confirm your input and review recommended settings.
- Search
- New
- Details & tools
- Input: DNA-001 · pHelixBench-Demo
- Version 7 · 3021 bp · circular · full sequence, not only the selected promoter.
- Recommended setup
- Enzyme selection
- EcoRI + BamHI
- Analysis scope
- Whole circular sequence · 3021 bp
- Both enzymes have one predicted cut site in this example input. Settings are suggestions; review assumptions before
interpretation.
- Advanced settings
- Why these settings?
- The result will retain input version, enzymes, scope and parameters.
- Back to sequence
- Run restriction analysis

**Authored actions:**

- `115:4036` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `115:4037` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `115:4084` Action / Details & tools → `[{"type":"NODE","target":"115:4078","navigation":"OVERLAY"}]`
- `115:4055` Action / Advanced settings → `[{"type":"NODE","target":"110:20486","navigation":"NAVIGATE"}]`
- `115:4056` Action / Why these settings? → `[{"type":"NODE","target":"110:20514","navigation":"NAVIGATE"}]`
- `115:4062` Action / Back to sequence → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`
- `115:4063` Action / Run restriction analysis → `[{"type":"NODE","target":"110:20538","navigation":"NAVIGATE"}]`

## ST-320 · UX v4 context / AnalysisSetup

**Role:** Responsive/context variant. **Frame:** 115:4078 · 320 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=115-4078).

**Displayed content:**

- Input & prerequisites
- Restriction analysis needs a valid
DNA sequence and selected enzymes.
Your entire input is in scope; no
selected region is required.
- Review source sequence
- Choose another enzyme set
- Close panel

**Authored actions:**

- `115:4082` Action / Review source sequence → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`
- `115:4083` Action / Choose another enzyme set → `[{"type":"NODE","target":"107:26978","navigation":"NAVIGATE"}]`
- `128:8088` Action / Close panel → `[{"type":"CLOSE"}]`

## ST-321 · Responsive v4 / AnalysisResult / 1280

**Role:** Responsive/context variant. **Frame:** 115:4087 · 1280 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=115-4087).

**Displayed content:**

- Restriction analysis result
- RES-001 · pHelixBench-Demo · input DNA-001 v7
- Search
- New
- Details & tools
- Two predicted fragments
- EcoRI at 216 · BamHI at 234 · circular input length 3021 bp.
- Fragment
- Length
- Interpretation
- Fragment 1
- 18 bp
- Between the two selected cut positions
- Fragment 2
- 3003 bp
- Remaining circular backbone
- Total
- 3021 bp
- Matches input length
- Illustrative result fixture. Verify predictions and experimental assumptions before laboratory use.
- Input and settings stay attached
- DNA-001 · v7 · whole sequence · EcoRI + BamHI · RES-001
- Edit settings
- View source
- Save result

**Authored actions:**

- `115:4194` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `115:4195` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `115:4247` Action / Details & tools → `[{"type":"NODE","target":"115:4241","navigation":"OVERLAY"}]`
- `115:4225` Action / Edit settings → `[{"type":"NODE","target":"110:20288","navigation":"NAVIGATE"}]`
- `115:4226` Action / View source → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`
- `115:4227` Action / Save result → `[{"type":"NODE","target":"110:20799","navigation":"NAVIGATE"}]`

## ST-322 · UX v4 context / AnalysisResult

**Role:** Responsive/context variant. **Frame:** 115:4241 · 320 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=115-4241).

**Displayed content:**

- Read the result
- Fragment lengths describe this
input and enzyme set. The small
fragment may need different
experimental handling.
- Parameter summary
- How to interpret this
- Close panel

**Authored actions:**

- `115:4245` Action / Parameter summary → `[{"type":"NODE","target":"110:20486","navigation":"NAVIGATE"}]`
- `115:4246` Action / How to interpret this → `[{"type":"NODE","target":"110:20514","navigation":"NAVIGATE"}]`
- `128:8090` Action / Close panel → `[{"type":"CLOSE"}]`

## ST-323 · Flow completion / Archive DNA-001

**Role:** Linked functional state. **Frame:** 125:5170 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-5170).

**Displayed content:**

- Archive DNA-001?
- Close
- pHelixBench-Demo · DNA construct
- Selected record
- DNA-001 · pHelixBench-Demo
- Effect
- Remove from active lists; retain identity and linked references
- Recovery
- Restore from the archived-records view
- Keep active
- Archive DNA-001

**Authored actions:**

- `125:5174` Action / Close → `[{"type":"BACK"}]`
- `125:5192` Action / Keep active → `[{"type":"NODE","target":"110:21504","navigation":"NAVIGATE"}]`
- `125:5194` Action / Archive DNA-001 → `[{"type":"NODE","target":"125:5196","navigation":"NAVIGATE"}]`

## ST-324 · Flow completion / Archived DNA-001

**Role:** Linked functional state. **Frame:** 125:5196 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-5196).

**Displayed content:**

- DNA-001 archived
- Close
- The selected record is retained and excluded from active results.
- Record
- DNA-001 · pHelixBench-Demo
- Linked references
- Remain identifiable; archive does not silently delete data
- View archived records
- Restore DNA-001

**Authored actions:**

- `125:5200` Action / Close → `[{"type":"BACK"}]`
- `125:5214` Action / View archived records → `[{"type":"NODE","target":"128:4143","navigation":"NAVIGATE"}]`
- `125:5216` Action / Restore DNA-001 → `[{"type":"NODE","target":"125:5218","navigation":"NAVIGATE"}]`

## ST-325 · Flow completion / Restored DNA-001

**Role:** Linked functional state. **Frame:** 125:5218 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-5218).

**Displayed content:**

- DNA-001 restored
- Close
- The same record is active again.
- Record
- DNA-001 · pHelixBench-Demo
- Identity
- Original ID retained
- Open record
- Return to Registry

**Authored actions:**

- `125:5222` Action / Close → `[{"type":"BACK"}]`
- `125:5236` Action / Open record → `[{"type":"NODE","target":"110:21504","navigation":"NAVIGATE"}]`
- `125:5238` Action / Return to Registry → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`

## ST-326 · Flow completion / Archive PR-001

**Role:** Linked functional state. **Frame:** 125:5240 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-5240).

**Displayed content:**

- Archive PR-001?
- Close
- T7-F · Primer
- Selected record
- PR-001 · T7-F
- Effect
- Remove from active lists; retain identity and linked references
- Recovery
- Restore from the archived-records view
- Keep active
- Archive PR-001

**Authored actions:**

- `125:5244` Action / Close → `[{"type":"BACK"}]`
- `125:5262` Action / Keep active → `[{"type":"NODE","target":"110:21536","navigation":"NAVIGATE"}]`
- `125:5264` Action / Archive PR-001 → `[{"type":"NODE","target":"125:5266","navigation":"NAVIGATE"}]`

## ST-327 · Flow completion / Archived PR-001

**Role:** Linked functional state. **Frame:** 125:5266 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-5266).

**Displayed content:**

- PR-001 archived
- Close
- The selected record is retained and excluded from active results.
- Record
- PR-001 · T7-F
- Linked references
- Remain identifiable; archive does not silently delete data
- View archived records
- Restore PR-001

**Authored actions:**

- `125:5270` Action / Close → `[{"type":"BACK"}]`
- `125:5284` Action / View archived records → `[{"type":"NODE","target":"128:4499","navigation":"NAVIGATE"}]`
- `125:5286` Action / Restore PR-001 → `[{"type":"NODE","target":"125:5288","navigation":"NAVIGATE"}]`

## ST-328 · Flow completion / Restored PR-001

**Role:** Linked functional state. **Frame:** 125:5288 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-5288).

**Displayed content:**

- PR-001 restored
- Close
- The same record is active again.
- Record
- PR-001 · T7-F
- Identity
- Original ID retained
- Open record
- Return to Registry

**Authored actions:**

- `125:5292` Action / Close → `[{"type":"BACK"}]`
- `125:5306` Action / Open record → `[{"type":"NODE","target":"110:21536","navigation":"NAVIGATE"}]`
- `125:5308` Action / Return to Registry → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`

## ST-329 · Flow completion / Archive AB-014

**Role:** Linked functional state. **Frame:** 125:5310 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-5310).

**Displayed content:**

- Archive AB-014?
- Close
- AB-014 · Biologic
- Selected record
- AB-014 · AB-014
- Effect
- Remove from active lists; retain identity and linked references
- Recovery
- Restore from the archived-records view
- Keep active
- Archive AB-014

**Authored actions:**

- `125:5314` Action / Close → `[{"type":"BACK"}]`
- `125:5332` Action / Keep active → `[{"type":"NODE","target":"110:21568","navigation":"NAVIGATE"}]`
- `125:5334` Action / Archive AB-014 → `[{"type":"NODE","target":"125:5336","navigation":"NAVIGATE"}]`

## ST-330 · Flow completion / Archived AB-014

**Role:** Linked functional state. **Frame:** 125:5336 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-5336).

**Displayed content:**

- AB-014 archived
- Close
- The selected record is retained and excluded from active results.
- Record
- AB-014 · AB-014
- Linked references
- Remain identifiable; archive does not silently delete data
- View archived records
- Restore AB-014

**Authored actions:**

- `125:5340` Action / Close → `[{"type":"BACK"}]`
- `125:5354` Action / View archived records → `[{"type":"NODE","target":"128:4855","navigation":"NAVIGATE"}]`
- `125:5356` Action / Restore AB-014 → `[{"type":"NODE","target":"125:5358","navigation":"NAVIGATE"}]`

## ST-331 · Flow completion / Restored AB-014

**Role:** Linked functional state. **Frame:** 125:5358 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-5358).

**Displayed content:**

- AB-014 restored
- Close
- The same record is active again.
- Record
- AB-014 · AB-014
- Identity
- Original ID retained
- Open record
- Return to Registry

**Authored actions:**

- `125:5362` Action / Close → `[{"type":"BACK"}]`
- `125:5376` Action / Open record → `[{"type":"NODE","target":"110:21568","navigation":"NAVIGATE"}]`
- `125:5378` Action / Return to Registry → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`

## ST-332 · Flow completion / Archive OL-014

**Role:** Linked functional state. **Frame:** 125:5380 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-5380).

**Displayed content:**

- Archive OL-014?
- Close
- OL-014 · RNA oligo
- Selected record
- OL-014 · OL-014
- Effect
- Remove from active lists; retain identity and linked references
- Recovery
- Restore from the archived-records view
- Keep active
- Archive OL-014

**Authored actions:**

- `125:5384` Action / Close → `[{"type":"BACK"}]`
- `125:5402` Action / Keep active → `[{"type":"NODE","target":"110:21600","navigation":"NAVIGATE"}]`
- `125:5404` Action / Archive OL-014 → `[{"type":"NODE","target":"125:5406","navigation":"NAVIGATE"}]`

## ST-333 · Flow completion / Archived OL-014

**Role:** Linked functional state. **Frame:** 125:5406 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-5406).

**Displayed content:**

- OL-014 archived
- Close
- The selected record is retained and excluded from active results.
- Record
- OL-014 · OL-014
- Linked references
- Remain identifiable; archive does not silently delete data
- View archived records
- Restore OL-014

**Authored actions:**

- `125:5410` Action / Close → `[{"type":"BACK"}]`
- `125:5424` Action / View archived records → `[{"type":"NODE","target":"128:5211","navigation":"NAVIGATE"}]`
- `125:5426` Action / Restore OL-014 → `[{"type":"NODE","target":"125:5428","navigation":"NAVIGATE"}]`

## ST-334 · Flow completion / Restored OL-014

**Role:** Linked functional state. **Frame:** 125:5428 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-5428).

**Displayed content:**

- OL-014 restored
- Close
- The same record is active again.
- Record
- OL-014 · OL-014
- Identity
- Original ID retained
- Open record
- Return to Registry

**Authored actions:**

- `125:5432` Action / Close → `[{"type":"BACK"}]`
- `125:5446` Action / Open record → `[{"type":"NODE","target":"110:21600","navigation":"NAVIGATE"}]`
- `125:5448` Action / Return to Registry → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`

## ST-335 · Flow completion / ArchiveBrowser

**Role:** Linked functional state. **Frame:** 125:5450 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-5450).

**Displayed content:**

- Archived records
- Archived records retain stable identities and can be restored.
- Search
- New
- No archived records
- Archived records will appear here when you deliberately archive them.
- Show active records
- Context & next steps

**Authored actions:**

- `125:5557` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `125:5558` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `128:5571` Action / Show active records → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`

## ST-336 · Flow completion / DeletedSource

**Role:** Linked functional state. **Frame:** 125:5642 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-5642).

**Displayed content:**

- Source DNA-001 was deleted
- Close
- The saved result keeps its original provenance.
- Source
- DNA-001 · version 7 · unavailable
- Retained output
- RES-001 · input version and parameters retained
- Available actions
- Inspect retained result or select an accessible source; no automatic rerun
- View retained result
- Back to Registry

**Authored actions:**

- `125:5646` Action / Close → `[{"type":"BACK"}]`
- `125:5664` Action / View retained result → `[{"type":"NODE","target":"132:5347","navigation":"NAVIGATE"}]`
- `125:5666` Action / Back to Registry → `[{"type":"NODE","target":"110:21795","navigation":"NAVIGATE"}]`

## ST-337 · Flow completion / SearchAfterDelete

**Role:** Linked functional state. **Frame:** 125:5668 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-5668).

**Displayed content:**

- Search your workspace
- DNA-001 was deleted. Surviving results keep an unavailable-source reference.
- Search
- New
- Record
- Type
- Source
- Restriction analysis · RES-001
- Saved analysis
- DNA-001 v7 · deleted
- Return to Registry
- Context & next steps

**Authored actions:**

- `125:5775` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `125:5776` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `125:5836` Record / Restriction analysis · RES-001 → `[{"type":"NODE","target":"125:5642","navigation":"NAVIGATE"}]`
- `125:5841` Action / Return to Registry → `[{"type":"NODE","target":"110:21795","navigation":"NAVIGATE"}]`

## ST-338 · Flow completion / SearchSequences

**Role:** Linked functional state. **Frame:** 125:5843 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-5843).

**Displayed content:**

- Sequence search
- Query: DNA-001 · scope: Sequences
- Search
- New
- Search query
- DNA-001
- All records
- Sequences
- Results
- Clear search
- Record
- Type / context
- Open
- pHelixBench-Demo · DNA-001
- Sequence · v7
- Open source
- Back to Home
- Context & next steps
- Search scope and query remain
visible. Return to the same result
list after inspecting an item.

**Authored actions:**

- `125:5950` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `125:5951` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `125:6011` Action / All records → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `125:6015` Action / Results → `[{"type":"NODE","target":"125:6032","navigation":"NAVIGATE"}]`
- `125:6017` Action / Clear search → `[{"type":"NODE","target":"125:6221","navigation":"NAVIGATE"}]`
- `125:6024` Record / pHelixBench-Demo · DNA-001 → `[{"type":"NODE","target":"130:4820","navigation":"NAVIGATE"}]`
- `125:6029` Action / Back to Home → `[{"type":"NODE","target":"107:29237","navigation":"NAVIGATE"}]`

## ST-339 · Flow completion / SearchResults

**Role:** Linked functional state. **Frame:** 125:6032 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-6032).

**Displayed content:**

- Analysis search
- Query: DNA-001 · scope: Results
- Search
- New
- Search query
- DNA-001
- All records
- Sequences
- Results
- Clear search
- Record
- Type / context
- Open
- Restriction analysis · RES-001
- Analysis · DNA-001 v7
- Open result
- Back to Home
- Context & next steps
- Search scope and query remain
visible. Return to the same result
list after inspecting an item.

**Authored actions:**

- `125:6139` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `125:6140` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `125:6200` Action / All records → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `125:6202` Action / Sequences → `[{"type":"NODE","target":"125:5843","navigation":"NAVIGATE"}]`
- `125:6206` Action / Clear search → `[{"type":"NODE","target":"125:6221","navigation":"NAVIGATE"}]`
- `125:6213` Record / Restriction analysis · RES-001 → `[{"type":"NODE","target":"130:5079","navigation":"NAVIGATE"}]`
- `125:6218` Action / Back to Home → `[{"type":"NODE","target":"107:29237","navigation":"NAVIGATE"}]`

## ST-340 · Flow completion / SearchRecent

**Role:** Linked functional state. **Frame:** 125:6221 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-6221).

**Displayed content:**

- Search your workspace
- Query cleared · recent work
- Search
- New
- Search query
- Search by name or stable ID
- All records
- Sequences
- Results
- Clear search
- Record
- Type / context
- Open
- pHelixBench-Demo · DNA-001
- Recent sequence
- Open source
- Restriction analysis · RES-001
- Recent result
- Open result
- Back to Home
- Context & next steps
- Search scope and query remain
visible. Return to the same result
list after inspecting an item.

**Authored actions:**

- `125:6328` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `125:6329` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `125:6389` Action / All records → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `125:6391` Action / Sequences → `[{"type":"NODE","target":"125:5843","navigation":"NAVIGATE"}]`
- `125:6393` Action / Results → `[{"type":"NODE","target":"125:6032","navigation":"NAVIGATE"}]`
- `125:6402` Record / pHelixBench-Demo · DNA-001 → `[{"type":"NODE","target":"130:5230","navigation":"NAVIGATE"}]`
- `125:6406` Record / Restriction analysis · RES-001 → `[{"type":"NODE","target":"130:5489","navigation":"NAVIGATE"}]`
- `125:6411` Action / Back to Home → `[{"type":"NODE","target":"107:29237","navigation":"NAVIGATE"}]`

## ST-341 · Flow completion / BlankReview

**Role:** Linked functional state. **Frame:** 125:6417 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-6417).

**Displayed content:**

- Review your new sequence
- Close
- Check the input before creation.
- Name / ID
- Untitled sequence · DNA-NEW-001
- Type / topology
- DNA · linear
- Length
- 12 nt
- Original bases
- ATGCGTACGTAA
- Changes
- None · no unsupported symbols
- Back to input
- Create sequence
- Review an input issue

**Authored actions:**

- `125:6421` Action / Close → `[{"type":"BACK"}]`
- `125:6447` Action / Back to input → `[{"type":"NODE","target":"110:20202","navigation":"NAVIGATE"}]`
- `125:6449` Action / Create sequence → `[{"type":"NODE","target":"125:6453","navigation":"NAVIGATE"}]`
- `125:6451` Action / Review an input issue → `[{"type":"NODE","target":"110:20168","navigation":"NAVIGATE"}]`

## ST-342 · Flow completion / BlankEditor

**Role:** Linked functional state. **Frame:** 125:6453 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-6453).

**Displayed content:**

- Untitled sequence
- DNA-NEW-001 · version 1 · 12 nt · linear · not registered
- Search
- New
- Your sequence
- Coordinates are 1-based and inclusive. No features or primers have been added.
- 1                           12
- ATGCGTACGTAA
- Edit sequence
- Save document
- Restriction analysis
- Context & next steps
- This is your newly created
document. Registry and physical
inventory are optional.

**Authored actions:**

- `125:6560` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `125:6561` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `125:6623` Action / Edit sequence → `[{"type":"NODE","target":"125:6630","navigation":"NAVIGATE"}]`
- `125:6625` Action / Save document → `[{"type":"NODE","target":"125:6656","navigation":"NAVIGATE"}]`
- `125:6627` Action / Restriction analysis → `[{"type":"NODE","target":"125:6704","navigation":"NAVIGATE"}]`

## ST-343 · Flow completion / BlankEdit

**Role:** Linked functional state. **Frame:** 125:6630 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-6630).

**Displayed content:**

- Edit DNA-NEW-001
- Close
- Keep the original until you confirm an edit.
- Range
- 1–12 · forward
- Sequence
- ATGCGTACGTAA
- Validation
- 12 DNA bases · no changes
- Cancel
- Apply unchanged sequence

**Authored actions:**

- `125:6634` Action / Close → `[{"type":"BACK"}]`
- `125:6652` Action / Cancel → `[{"type":"NODE","target":"125:6453","navigation":"NAVIGATE"}]`
- `125:6654` Action / Apply unchanged sequence → `[{"type":"NODE","target":"125:6453","navigation":"NAVIGATE"}]`

## ST-344 · Flow completion / BlankSave

**Role:** Linked functional state. **Frame:** 125:6656 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-6656).

**Displayed content:**

- Save DNA-NEW-001
- Close
- Choose a destination and retain the document identity.
- Name
- Untitled sequence
- Location
- Local workspace / Sequences
- Version
- 1
- Cancel
- Save document

**Authored actions:**

- `125:6660` Action / Close → `[{"type":"BACK"}]`
- `125:6678` Action / Cancel → `[{"type":"NODE","target":"125:6453","navigation":"NAVIGATE"}]`
- `125:6680` Action / Save document → `[{"type":"NODE","target":"125:6682","navigation":"NAVIGATE"}]`

## ST-345 · Flow completion / BlankSaved

**Role:** Linked functional state. **Frame:** 125:6682 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-6682).

**Displayed content:**

- Untitled sequence saved
- Close
- DNA-NEW-001 · Local workspace / Sequences
- Saved version
- 1
- Length / topology
- 12 nt · linear
- Open sequence
- Home

**Authored actions:**

- `125:6686` Action / Close → `[{"type":"BACK"}]`
- `125:6700` Action / Open sequence → `[{"type":"NODE","target":"125:6453","navigation":"NAVIGATE"}]`
- `125:6702` Action / Home → `[{"type":"NODE","target":"126:7787","navigation":"NAVIGATE"}]`

## ST-346 · Flow completion / BlankAnalysis

**Role:** Linked functional state. **Frame:** 125:6704 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-6704).

**Displayed content:**

- Restriction analysis · DNA-NEW-001
- Close
- Use the complete 12 nt linear input.
- Input
- DNA-NEW-001 · v1 · ATGCGTACGTAA
- Enzymes
- EcoRI and BamHI
- Scope
- Whole linear sequence
- Cancel
- Run analysis

**Authored actions:**

- `125:6708` Action / Close → `[{"type":"BACK"}]`
- `125:6726` Action / Cancel → `[{"type":"NODE","target":"125:6453","navigation":"NAVIGATE"}]`
- `125:6728` Action / Run analysis → `[{"type":"NODE","target":"125:6730","navigation":"NAVIGATE"}]`

## ST-347 · Flow completion / BlankNoSites

**Role:** Linked functional state. **Frame:** 125:6730 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-6730).

**Displayed content:**

- No cut sites found
- Close
- The analysis completed; this is not an error.
- Input
- DNA-NEW-001 · v1
- Enzymes
- EcoRI and BamHI
- Predicted output
- One uncut linear fragment · 12 nt
- Change enzyme selection
- Return to input

**Authored actions:**

- `125:6734` Action / Close → `[{"type":"BACK"}]`
- `125:6752` Action / Change enzyme selection → `[{"type":"NODE","target":"125:6704","navigation":"NAVIGATE"}]`
- `125:6754` Action / Return to input → `[{"type":"NODE","target":"125:6453","navigation":"NAVIGATE"}]`

## ST-348 · Flow completion / BlankNotebook

**Role:** Linked functional state. **Frame:** 125:6756 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-6756).

**Displayed content:**

- Untitled experiment
- EXP-NEW-001 · local draft · no submitted rows
- Search
- New
- Title
- Notes
- Write the purpose of your experiment…
- No source linked
- Link a sequence
- Add result table
- Choose a template
- Draft saved locally. Saving text does not submit results.
- Context & next steps
- Start with your own notes or
deliberately apply a template. This
draft remains EXP-NEW-001.

**Authored actions:**

- `125:6863` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `125:6864` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `125:6929` Action / Link a sequence → `[{"type":"NODE","target":"125:6966","navigation":"NAVIGATE"}]`
- `125:6931` Action / Add result table → `[{"type":"NODE","target":"125:7160","navigation":"NAVIGATE"}]`
- `125:6933` Action / Choose a template → `[{"type":"NODE","target":"125:6940","navigation":"NAVIGATE"}]`

## ST-349 · Flow completion / NotebookTemplate

**Role:** Linked functional state. **Frame:** 125:6940 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-6940).

**Displayed content:**

- Choose a notebook template
- Close
- Applying a template adds structure to your own draft.
- Entry
- EXP-NEW-001 · Untitled experiment
- Template
- Sequence validation · headings only
- What is added
- Purpose, input, method and results sections; no demo measurements
- Keep blank
- Apply headings

**Authored actions:**

- `125:6944` Action / Close → `[{"type":"BACK"}]`
- `125:6962` Action / Keep blank → `[{"type":"NODE","target":"125:6756","navigation":"NAVIGATE"}]`
- `125:6964` Action / Apply headings → `[{"type":"NODE","target":"132:5034","navigation":"NAVIGATE"}]`

## ST-350 · Flow completion / NotebookLink

**Role:** Linked functional state. **Frame:** 125:6966 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-6966).

**Displayed content:**

- Link an experimental input
- Close
- Choose the exact source record and version.
- Draft
- EXP-NEW-001
- Selected input
- DNA-001 · pHelixBench-Demo · v7
- Cancel
- Link selected input

**Authored actions:**

- `125:6970` Action / Close → `[{"type":"BACK"}]`
- `125:6984` Action / Cancel → `[{"type":"NODE","target":"125:6756","navigation":"NAVIGATE"}]`
- `125:6986` Action / Link selected input → `[{"type":"NODE","target":"125:6988","navigation":"NAVIGATE"}]`

## ST-351 · Flow completion / NotebookLinked

**Role:** Linked functional state. **Frame:** 125:6988 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-6988).

**Displayed content:**

- Untitled experiment
- EXP-NEW-001 · draft · input DNA-001 v7
- Search
- New
- Purpose
- Describe what you want to learn.
- Linked input: pHelixBench-Demo · DNA-001 · version 7
- Add result table
- Return to blank draft
- Context & next steps

**Authored actions:**

- `125:7095` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `125:7096` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `125:7156` Action / Add result table → `[{"type":"NODE","target":"125:7160","navigation":"NAVIGATE"}]`
- `125:7158` Action / Return to blank draft → `[{"type":"NODE","target":"125:6756","navigation":"NAVIGATE"}]`

## ST-352 · Flow completion / NotebookTable

**Role:** Linked functional state. **Frame:** 125:7160 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-7160).

**Displayed content:**

- Add structured results
- Close
- EXP-NEW-001 · input DNA-001 v7
- Table schema
- Sequence check
- Example row entered
- DNA-001 · sequence reviewed · Pass
- State
- One unsubmitted row · editable
- Keep editing
- Review submission

**Authored actions:**

- `125:7164` Action / Close → `[{"type":"BACK"}]`
- `125:7182` Action / Keep editing → `[{"type":"NODE","target":"125:6988","navigation":"NAVIGATE"}]`
- `125:7184` Action / Review submission → `[{"type":"NODE","target":"125:7186","navigation":"NAVIGATE"}]`

## ST-353 · Flow completion / NotebookSubmit

**Role:** Linked functional state. **Frame:** 125:7186 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-7186).

**Displayed content:**

- Submit one result row?
- Close
- Text remains saved separately from the submitted result.
- Entry
- EXP-NEW-001
- Source
- DNA-001 · version 7
- Row
- Sequence reviewed · Pass
- Validation
- Required source and result fields present
- Keep as draft
- Submit row

**Authored actions:**

- `125:7190` Action / Close → `[{"type":"BACK"}]`
- `125:7212` Action / Keep as draft → `[{"type":"NODE","target":"125:7160","navigation":"NAVIGATE"}]`
- `125:7214` Action / Submit row → `[{"type":"NODE","target":"125:7216","navigation":"NAVIGATE"}]`

## ST-354 · Flow completion / NotebookSubmitted

**Role:** Linked functional state. **Frame:** 125:7216 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-7216).

**Displayed content:**

- One result row submitted
- Close
- The reusable result is linked to your own entry.
- Entry
- EXP-NEW-001
- Result
- NRES-001 · DNA-001 v7
- Review state
- Not sent for review
- Open result
- Send for review
- Return to entry

**Authored actions:**

- `125:7220` Action / Close → `[{"type":"BACK"}]`
- `125:7238` Action / Open result → `[{"type":"NODE","target":"125:7244","navigation":"NAVIGATE"}]`
- `125:7240` Action / Send for review → `[{"type":"NODE","target":"125:7268","navigation":"NAVIGATE"}]`
- `125:7242` Action / Return to entry → `[{"type":"NODE","target":"125:6988","navigation":"NAVIGATE"}]`

## ST-355 · Flow completion / NotebookResult

**Role:** Linked functional state. **Frame:** 125:7244 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-7244).

**Displayed content:**

- Sequence check result
- Close
- NRES-001 · submitted from EXP-NEW-001
- Source
- DNA-001 · v7
- Result
- Sequence reviewed · Pass
- Provenance
- Entry and source version retained
- Return to entry

**Authored actions:**

- `125:7248` Action / Close → `[{"type":"BACK"}]`
- `125:7266` Action / Return to entry → `[{"type":"NODE","target":"125:6988","navigation":"NAVIGATE"}]`

## ST-356 · Flow completion / NotebookReview

**Role:** Linked functional state. **Frame:** 125:7268 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-7268).

**Displayed content:**

- Review request
- Close
- Choose the audience before sending.
- Entry
- EXP-NEW-001
- Submitted rows
- 1 · NRES-001
- Reviewer
- Choose an authorized collaborator
- Current state
- Reviewer required before sending
- Choose reviewer
- Return to entry

**Authored actions:**

- `125:7272` Action / Close → `[{"type":"BACK"}]`
- `132:5345` Action / Choose reviewer → `[{"type":"NODE","target":"132:5188","navigation":"NAVIGATE"}]`
- `125:7294` Action / Return to entry → `[{"type":"NODE","target":"125:6988","navigation":"NAVIGATE"}]`

## ST-357 · Flow completion / Draft Sequence

**Role:** Linked functional state. **Frame:** 125:7296 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-7296).

**Displayed content:**

- Sequence
- Inspect bases, annotations and the active selection.
- Search
- New
- pHelixBench-Demo
- Unsaved draft · based on v7
- Undo
- Save version
- Export
- Map
- Features
- Primers
- Enzymes
- History
- Find in sequence
- Edit selection
- Analyze
- DNA sequence · forward strand
- 1                                                 60
- ACAGTGGTTCACAGACAGATACAGACTTTCGGTTCAAGACTCTGCTGACTGC
- 61                                                 120
- GCTTATAGTTCACCTACGATCGGTGACATCGGCTAGCTAGCTAGGCTAACGT
- 121                                                 180
- TACGGTACCGGAATTCGGATCCATGTCTAAAGGTGAAGAATTATTCACTGGT
- 181                                                 240
- GTTGTCCCAATTCTTGTTGAATTAGATGGTGATGTTAATGGTCACAAATTT
- Selected 97–116 · 20 nt. Changes are validated before the saved sequence is updated.
- Annotation
- Coordinates
- Strand
- T7 promoter
- 97–116
- Forward
- lac operator
- 119–143
- demo_cds
- 180–897
- Details & tools
- All sequence tools
- Current selection
- Bases 97–116
- How to use Sequence
- Coordinates are 1-based and inclusive.
Switch views without losing your
selected region.
- Document
- DNA-001 · 3021 bp
- Selection
- 20 nt · forward strand
- Translation
- Frame +1 · standard code
- Actions
- Add feature
- BLAST selection
- Related records
- Primer pair P-01

**Authored actions:**

- `125:7403` Action / Search → `[{"type":"NODE","target":"125:9306","navigation":"NAVIGATE"}]`
- `125:7404` Action / New → `[{"type":"NODE","target":"125:9358","navigation":"NAVIGATE"}]`
- `125:7410` Action / Save version → `[{"type":"NODE","target":"125:8336","navigation":"NAVIGATE"}]`
- `125:7411` Action / Export → `[{"type":"NODE","target":"125:9410","navigation":"NAVIGATE"}]`
- `125:7413` Action / Map → `[{"type":"NODE","target":"115:3326","navigation":"NAVIGATE"}]`
- `125:7415` Action / Features → `[{"type":"NODE","target":"125:7508","navigation":"NAVIGATE"}]`
- `125:7416` Action / Primers → `[{"type":"NODE","target":"125:7718","navigation":"NAVIGATE"}]`
- `125:7417` Action / Enzymes → `[{"type":"NODE","target":"125:7923","navigation":"NAVIGATE"}]`
- `125:7418` Action / History → `[{"type":"NODE","target":"125:8131","navigation":"NAVIGATE"}]`
- `125:7421` Action / Find in sequence → `[{"type":"NODE","target":"125:9898","navigation":"NAVIGATE"}]`
- `125:7422` Action / Edit selection → `[{"type":"NODE","target":"125:9950","navigation":"NAVIGATE"}]`
- `125:7423` Action / Analyze → `[{"type":"NODE","target":"125:9514","navigation":"NAVIGATE"}]`
- `125:7476` Action / All sequence tools → `[{"type":"NODE","target":"125:9670","navigation":"NAVIGATE"}]`
- `125:7493` Action / Edit selection → `[{"type":"NODE","target":"125:9950","navigation":"NAVIGATE"}]`
- `125:7494` Action / Add feature → `[{"type":"NODE","target":"125:9462","navigation":"NAVIGATE"}]`
- `125:7495` Action / BLAST selection → `[{"type":"NODE","target":"125:10002","navigation":"NAVIGATE"}]`
- `125:7498` Related / T7 promoter → `[{"type":"NODE","target":"125:9618","navigation":"NAVIGATE"}]`
- `125:7503` Related / Primer pair P-01 → `[{"type":"NODE","target":"125:10054","navigation":"NAVIGATE"}]`

## ST-358 · Flow completion / Draft Features

**Role:** Linked functional state. **Frame:** 125:7508 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-7508).

**Displayed content:**

- Features
- Review annotations before applying changes.
- Search
- New
- pHelixBench-Demo
- Unsaved draft · based on v7
- Undo
- Save version
- Export
- Map
- Sequence
- Primers
- Enzymes
- History
- Add feature
- Detect common
- Review auto-annotation
- Name
- Type
- Range
- Strand
- T7 promoter
- Promoter
- 97–116
- Forward
- lac operator
- Operator
- 119–143
- demo_cds
- CDS
- 180–897
- rrnB T1 terminator
- Terminator
- 940–1025
- AmpR
- 1700–2558
- Reverse
- Review proposed additions and conflicts before changing the annotation set.
- Details & tools
- All features tools
- Current selection
- How to use Features
- Features describe annotated regions.
Add your own annotation or review
suggested annotations before applying.
- 97–116 · forward
- Source
- Manual annotation
- Actions
- Edit selected
- Auto-annotation preview
- Related records
- Expression validation

**Authored actions:**

- `125:7615` Action / Search → `[{"type":"NODE","target":"125:9306","navigation":"NAVIGATE"}]`
- `125:7616` Action / New → `[{"type":"NODE","target":"125:9358","navigation":"NAVIGATE"}]`
- `125:7622` Action / Save version → `[{"type":"NODE","target":"125:8336","navigation":"NAVIGATE"}]`
- `125:7623` Action / Export → `[{"type":"NODE","target":"125:9410","navigation":"NAVIGATE"}]`
- `125:7625` Action / Map → `[{"type":"NODE","target":"115:3326","navigation":"NAVIGATE"}]`
- `125:7626` Action / Sequence → `[{"type":"NODE","target":"125:7296","navigation":"NAVIGATE"}]`
- `125:7628` Action / Primers → `[{"type":"NODE","target":"125:7718","navigation":"NAVIGATE"}]`
- `125:7629` Action / Enzymes → `[{"type":"NODE","target":"125:7923","navigation":"NAVIGATE"}]`
- `125:7630` Action / History → `[{"type":"NODE","target":"125:8131","navigation":"NAVIGATE"}]`
- `125:7633` Action / Add feature → `[{"type":"NODE","target":"125:9462","navigation":"NAVIGATE"}]`
- `125:7634` Action / Detect common → `[{"type":"NODE","target":"125:10126","navigation":"NAVIGATE"}]`
- `125:7635` Action / Review auto-annotation → `[{"type":"NODE","target":"125:10178","navigation":"NAVIGATE"}]`
- `125:7642` Record / T7 promoter → `[{"type":"NODE","target":"125:9462","navigation":"NAVIGATE"}]`
- `125:7647` Record / lac operator → `[{"type":"NODE","target":"125:9462","navigation":"NAVIGATE"}]`
- `125:7652` Record / demo_cds → `[{"type":"NODE","target":"125:9462","navigation":"NAVIGATE"}]`
- `125:7657` Record / rrnB T1 terminator → `[{"type":"NODE","target":"125:9462","navigation":"NAVIGATE"}]`
- `125:7662` Record / AmpR → `[{"type":"NODE","target":"125:9462","navigation":"NAVIGATE"}]`
- `125:7686` Action / All features tools → `[{"type":"NODE","target":"125:10230","navigation":"NAVIGATE"}]`
- `125:7703` Action / Edit selected → `[{"type":"NODE","target":"125:9462","navigation":"NAVIGATE"}]`
- `125:7704` Action / Detect common → `[{"type":"NODE","target":"125:10126","navigation":"NAVIGATE"}]`
- `125:7705` Action / Auto-annotation preview → `[{"type":"NODE","target":"125:10178","navigation":"NAVIGATE"}]`
- `125:7708` Related / pHelixBench-Demo → `[{"type":"NODE","target":"125:9846","navigation":"NAVIGATE"}]`
- `125:7713` Related / Expression validation → `[{"type":"NODE","target":"125:8838","navigation":"NAVIGATE"}]`

## ST-359 · Flow completion / Draft Primers

**Role:** Linked functional state. **Frame:** 125:7718 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-7718).

**Displayed content:**

- Primers
- Review candidate pairs and verify the predicted product.
- Search
- New
- pHelixBench-Demo
- Unsaved draft · based on v7
- Undo
- Save version
- Export
- Map
- Sequence
- Features
- Enzymes
- History
- Design pair
- Add primer
- Simulate PCR
- Primer / pair
- Position
- Tm
- State
- P-01 · T7-F / Demo-R
- 97–351
- 61.2 / 62.0 °C
- Linked pair
- T7-F
- 97–116
- 61.2 °C
- Forward
- Demo-R
- 330–351
- 62.0 °C
- Reverse
- P-02 · screening
- 180–598
- 60.8 / 61.4 °C
- Review needed
- P-01 is linked to DNA-001. Predicted product: 255 bp; inspect simulation before saving.
- Details & tools
- All primers tools
- Current selection
- Pair P-01
- How to use Primers
- A primer is linked to an input
sequence. Verify range, strand and
pair before simulating PCR.
- T7-F · 20 nt
- Demo-R · 22 nt
- Predicted product
- 255 bp · ready to simulate
- Actions
- Edit primer
- Related records
- DNA-001 template
- PCR product history

**Authored actions:**

- `125:7825` Action / Search → `[{"type":"NODE","target":"125:9306","navigation":"NAVIGATE"}]`
- `125:7826` Action / New → `[{"type":"NODE","target":"125:9358","navigation":"NAVIGATE"}]`
- `125:7832` Action / Save version → `[{"type":"NODE","target":"125:8336","navigation":"NAVIGATE"}]`
- `125:7833` Action / Export → `[{"type":"NODE","target":"125:9410","navigation":"NAVIGATE"}]`
- `125:7835` Action / Map → `[{"type":"NODE","target":"115:3326","navigation":"NAVIGATE"}]`
- `125:7836` Action / Sequence → `[{"type":"NODE","target":"125:7296","navigation":"NAVIGATE"}]`
- `125:7837` Action / Features → `[{"type":"NODE","target":"125:7508","navigation":"NAVIGATE"}]`
- `125:7839` Action / Enzymes → `[{"type":"NODE","target":"125:7923","navigation":"NAVIGATE"}]`
- `125:7840` Action / History → `[{"type":"NODE","target":"125:8131","navigation":"NAVIGATE"}]`
- `125:7843` Action / Design pair → `[{"type":"NODE","target":"125:9774","navigation":"NAVIGATE"}]`
- `125:7844` Action / Add primer → `[{"type":"NODE","target":"125:9722","navigation":"NAVIGATE"}]`
- `125:7845` Action / Simulate PCR → `[{"type":"NODE","target":"125:10302","navigation":"NAVIGATE"}]`
- `125:7852` Record / P-01 · T7-F / Demo-R → `[{"type":"NODE","target":"125:9722","navigation":"NAVIGATE"}]`
- `125:7857` Record / T7-F → `[{"type":"NODE","target":"125:9722","navigation":"NAVIGATE"}]`
- `125:7862` Record / Demo-R → `[{"type":"NODE","target":"125:9722","navigation":"NAVIGATE"}]`
- `125:7867` Record / P-02 · screening → `[{"type":"NODE","target":"125:9722","navigation":"NAVIGATE"}]`
- `125:7891` Action / All primers tools → `[{"type":"NODE","target":"125:10354","navigation":"NAVIGATE"}]`
- `125:7908` Action / Design pair → `[{"type":"NODE","target":"125:9774","navigation":"NAVIGATE"}]`
- `125:7909` Action / Edit primer → `[{"type":"NODE","target":"125:9722","navigation":"NAVIGATE"}]`
- `125:7910` Action / Simulate PCR → `[{"type":"NODE","target":"125:10302","navigation":"NAVIGATE"}]`
- `125:7913` Related / DNA-001 template → `[{"type":"NODE","target":"125:9846","navigation":"NAVIGATE"}]`
- `125:7918` Related / PCR product history → `[{"type":"NODE","target":"125:10406","navigation":"NAVIGATE"}]`

## ST-360 · Flow completion / Draft Enzymes

**Role:** Linked functional state. **Frame:** 125:7923 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-7923).

**Displayed content:**

- Enzymes
- Choose cutters and inspect predicted fragments.
- Search
- New
- pHelixBench-Demo
- Unsaved draft · based on v7
- Undo
- Save version
- Export
- Map
- Sequence
- Features
- Primers
- History
- Enzyme set
- Filter cutters
- Digest & gel
- Enzyme
- Sites
- Cut position
- Selection
- EcoRI
- 1
- 216
- Selected
- BamHI
- 234
- HindIII
- 260
- Available
- NotI
- 0
- No sites
- Filtered out
- Digest setup
- Two selected enzymes · circular DNA · input version 7
- Expected fragments and gel preview are generated after parameter validation.
- Details & tools
- All enzymes tools
- Current selection
- EcoRI + BamHI
- How to use Enzymes
- Restriction analysis predicts cut
sites and fragments. Confirm your
input and enzyme set before running.
- Active set
- Common cloning enzymes
- Cut sites
- 2 selected · 4 total
- Document
- DNA-001 · circular
- Actions
- Choose enzyme set
- Related records
- Input DNA-001
- Digest result

**Authored actions:**

- `125:8030` Action / Search → `[{"type":"NODE","target":"125:9306","navigation":"NAVIGATE"}]`
- `125:8031` Action / New → `[{"type":"NODE","target":"125:9358","navigation":"NAVIGATE"}]`
- `125:8037` Action / Save version → `[{"type":"NODE","target":"125:8336","navigation":"NAVIGATE"}]`
- `125:8038` Action / Export → `[{"type":"NODE","target":"125:9410","navigation":"NAVIGATE"}]`
- `125:8040` Action / Map → `[{"type":"NODE","target":"115:3326","navigation":"NAVIGATE"}]`
- `125:8041` Action / Sequence → `[{"type":"NODE","target":"125:7296","navigation":"NAVIGATE"}]`
- `125:8042` Action / Features → `[{"type":"NODE","target":"125:7508","navigation":"NAVIGATE"}]`
- `125:8043` Action / Primers → `[{"type":"NODE","target":"125:7718","navigation":"NAVIGATE"}]`
- `125:8045` Action / History → `[{"type":"NODE","target":"125:8131","navigation":"NAVIGATE"}]`
- `125:8048` Action / Enzyme set → `[{"type":"NODE","target":"125:9566","navigation":"NAVIGATE"}]`
- `125:8049` Action / Filter cutters → `[{"type":"NODE","target":"125:10478","navigation":"NAVIGATE"}]`
- `125:8050` Action / Digest & gel → `[{"type":"NODE","target":"125:9514","navigation":"NAVIGATE"}]`
- `125:8057` Record / EcoRI → `[{"type":"NODE","target":"125:9514","navigation":"NAVIGATE"}]`
- `125:8062` Record / BamHI → `[{"type":"NODE","target":"125:9514","navigation":"NAVIGATE"}]`
- `125:8067` Record / HindIII → `[{"type":"NODE","target":"125:9514","navigation":"NAVIGATE"}]`
- `125:8072` Record / NotI → `[{"type":"NODE","target":"125:9514","navigation":"NAVIGATE"}]`
- `125:8099` Action / All enzymes tools → `[{"type":"NODE","target":"125:10530","navigation":"NAVIGATE"}]`
- `125:8116` Action / Choose enzyme set → `[{"type":"NODE","target":"125:9566","navigation":"NAVIGATE"}]`
- `125:8117` Action / Digest & gel → `[{"type":"NODE","target":"125:9514","navigation":"NAVIGATE"}]`
- `125:8118` Action / Filter cutters → `[{"type":"NODE","target":"125:10478","navigation":"NAVIGATE"}]`
- `125:8121` Related / Input DNA-001 → `[{"type":"NODE","target":"125:9846","navigation":"NAVIGATE"}]`
- `125:8126` Related / Digest result → `[{"type":"NODE","target":"125:10582","navigation":"NAVIGATE"}]`

## ST-361 · Flow completion / Draft History

**Role:** Linked functional state. **Frame:** 125:8131 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-8131).

**Displayed content:**

- History
- Compare changes before restoring a saved version.
- Search
- New
- pHelixBench-Demo
- Unsaved draft · based on v7
- Undo
- Save version
- Export
- Map
- Sequence
- Features
- Primers
- Enzymes
- Compare versions
- Duplicate version
- Restore preview
- Version
- Changed by
- Summary
- When
- v7 · current
- You
- T7 promoter annotation
- Today 10:42
- v6
- Reverse primer added
- Today 10:12
- v5
- M. Chen
- CDS coordinates updated
- Yesterday
- v4
- Imported GenBank file
- 12 Nov
- Previewing a version does not replace the current record. Restore includes an impact review.
- Details & tools
- All history tools
- Current selection
- Version 7 · current
- How to use History
- Versions are saved checkpoints. Undo
reverses an unsaved edit; restoring a
version requires an impact review.
- Saved by
- You · today at 10:42
- Changes
- Feature annotation updated
- Record
- DNA-001
- Actions
- Related records
- Current sequence
- Record activity

**Authored actions:**

- `125:8238` Action / Search → `[{"type":"NODE","target":"125:9306","navigation":"NAVIGATE"}]`
- `125:8239` Action / New → `[{"type":"NODE","target":"125:9358","navigation":"NAVIGATE"}]`
- `125:8245` Action / Save version → `[{"type":"NODE","target":"125:8336","navigation":"NAVIGATE"}]`
- `125:8246` Action / Export → `[{"type":"NODE","target":"125:9410","navigation":"NAVIGATE"}]`
- `125:8248` Action / Map → `[{"type":"NODE","target":"115:3326","navigation":"NAVIGATE"}]`
- `125:8249` Action / Sequence → `[{"type":"NODE","target":"125:7296","navigation":"NAVIGATE"}]`
- `125:8250` Action / Features → `[{"type":"NODE","target":"125:7508","navigation":"NAVIGATE"}]`
- `125:8251` Action / Primers → `[{"type":"NODE","target":"125:7718","navigation":"NAVIGATE"}]`
- `125:8252` Action / Enzymes → `[{"type":"NODE","target":"125:7923","navigation":"NAVIGATE"}]`
- `125:8256` Action / Compare versions → `[{"type":"NODE","target":"125:10654","navigation":"NAVIGATE"}]`
- `125:8257` Action / Duplicate version → `[{"type":"NODE","target":"125:10706","navigation":"NAVIGATE"}]`
- `125:8258` Action / Restore preview → `[{"type":"NODE","target":"125:10758","navigation":"NAVIGATE"}]`
- `125:8265` Record / v7 · current → `[{"type":"NODE","target":"125:10654","navigation":"NAVIGATE"}]`
- `125:8270` Record / v6 → `[{"type":"NODE","target":"125:10654","navigation":"NAVIGATE"}]`
- `125:8275` Record / v5 → `[{"type":"NODE","target":"125:10654","navigation":"NAVIGATE"}]`
- `125:8280` Record / v4 → `[{"type":"NODE","target":"125:10654","navigation":"NAVIGATE"}]`
- `125:8304` Action / All history tools → `[{"type":"NODE","target":"125:10810","navigation":"NAVIGATE"}]`
- `125:8321` Action / Compare versions → `[{"type":"NODE","target":"125:10654","navigation":"NAVIGATE"}]`
- `125:8322` Action / Duplicate version → `[{"type":"NODE","target":"125:10706","navigation":"NAVIGATE"}]`
- `125:8323` Action / Restore preview → `[{"type":"NODE","target":"125:10758","navigation":"NAVIGATE"}]`
- `125:8326` Related / Current sequence → `[{"type":"NODE","target":"125:10862","navigation":"NAVIGATE"}]`
- `125:8331` Related / Record activity → `[{"type":"NODE","target":"125:10914","navigation":"NAVIGATE"}]`

## ST-362 · Flow completion / DraftSaved

**Role:** Linked functional state. **Frame:** 125:8336 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-8336).

**Displayed content:**

- Sequence version saved
- DNA-001 · version 8 · previous version 7 retained
- Search
- New
- The sequence draft was saved as version 8. Existing results still identify their original input version.
- Open saved sequence
- Inspect older analysis
- Context & next steps

**Authored actions:**

- `125:8443` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `125:8444` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `125:8504` Action / Open saved sequence → `[{"type":"NODE","target":"125:8508","navigation":"NAVIGATE"}]`
- `125:8506` Action / Inspect older analysis → `[{"type":"NODE","target":"125:10993","navigation":"NAVIGATE"}]`

## ST-363 · Flow completion / SavedVersion8

**Role:** Linked functional state. **Frame:** 125:8508 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-8508).

**Displayed content:**

- Map
- Inspect a construct and act on the selected region.
- Search
- New
- pHelixBench-Demo
- v8 · saved just now
- Undo
- Save version
- Export
- Sequence
- Features
- Primers
- Enzymes
- History
- Add feature
- Analyze
- Display layers
- Circular map
- Selection 97–116 · forward
- 3021 bp · circular
- GC 54.5%
- drag to select
- T7 promoter
- lac operator
- RBS
- demo_cds
- Demo-Rev
- rrnB T1 terminator
- M13 fwd site
- pUC19 MCS
- M13 rev site
- ori (demo)
- demo_marker
- loxP
- FRT
- Kozak
- EcoRI
- BamHI
- ScaI
- AhdI
- SexAI
- PshAI
- DraI
- NcoI
- T7 promoter selected. Create and analysis tools retain this document and range.
- Readable annotations
- Restriction analysis
- Task help
- Details & tools
- All map tools
- Current selection
- T7 promoter · 97–116
- How to use Map
- Select a feature on the map or use
Features for a readable list. Tools
use the range shown at right.
- Record
- DNA-001 · version 8
- Selected range
- 97–116 · forward strand
- Type
- Promoter · 20 bp
- Actions
- Edit feature
- Add primer from range
- Design primer pair
- Related records
- DNA-001 · Registry
- Plate P-014 · Inventory

**Authored actions:**

- `125:8615` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `125:8616` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `125:8622` Action / Save version → `[{"type":"NODE","target":"107:29211","navigation":"OVERLAY"}]`
- `125:8623` Action / Export → `[{"type":"NODE","target":"107:27142","navigation":"OVERLAY"}]`
- `125:8626` Action / Sequence → `[{"type":"NODE","target":"130:5640","navigation":"NAVIGATE"}]`
- `125:8627` Action / Features → `[{"type":"NODE","target":"130:5852","navigation":"NAVIGATE"}]`
- `125:8628` Action / Primers → `[{"type":"NODE","target":"130:6062","navigation":"NAVIGATE"}]`
- `125:8629` Action / Enzymes → `[{"type":"NODE","target":"130:6267","navigation":"NAVIGATE"}]`
- `125:8630` Action / History → `[{"type":"NODE","target":"130:6475","navigation":"NAVIGATE"}]`
- `125:8633` Action / Add feature → `[{"type":"NODE","target":"107:25134","navigation":"OVERLAY"}]`
- `125:8634` Action / Analyze → `[{"type":"NODE","target":"126:7098","navigation":"NAVIGATE"}]`
- `125:8635` Action / Display layers → `[{"type":"NODE","target":"107:25217","navigation":"OVERLAY"}]`
- `125:8714` Action / Readable annotations → `[{"type":"NODE","target":"107:29886","navigation":"NAVIGATE"}]`
- `125:8715` Action / Restriction analysis → `[{"type":"NODE","target":"126:7098","navigation":"NAVIGATE"}]`
- `125:8716` Action / Task help → `[{"type":"NODE","target":"110:21252","navigation":"NAVIGATE"}]`
- `125:8731` Action / All map tools → `[{"type":"NODE","target":"110:22016","navigation":"NAVIGATE"}]`
- `125:8748` Action / Edit feature → `[{"type":"NODE","target":"107:25134","navigation":"OVERLAY"}]`
- `125:8749` Action / Add primer from range → `[{"type":"NODE","target":"107:25182","navigation":"OVERLAY"}]`
- `125:8750` Action / Design primer pair → `[{"type":"NODE","target":"107:25303","navigation":"OVERLAY"}]`
- `125:8753` Related / DNA-001 · Registry → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`
- `125:8758` Related / Plate P-014 · Inventory → `[{"type":"NODE","target":"107:31089","navigation":"NAVIGATE"}]`

## ST-364 · Flow completion / Leave draft 0

**Role:** Reference or alternate fixture. **Frame:** 125:8786 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-8786).

**Displayed content:**

- Save your sequence changes?
- Close
- DNA-001 has an unsaved draft based on version 7.
- Destination
- UX v4 / Home
- Save
- Create version 8 before continuing
- Discard
- Leave the saved version 7 unchanged
- Stay
- Return to the unsaved draft
- Stay on draft
- Discard and continue
- Save and continue

**Authored actions:**

- `125:8790` Action / Close → `[{"type":"BACK"}]`
- `125:8812` Action / Stay on draft → `[{"type":"BACK"}]`
- `125:8814` Action / Discard and continue → `[{"type":"NODE","target":"107:29237","navigation":"NAVIGATE"}]`
- `125:8816` Action / Save and continue → `[{"type":"NODE","target":"125:8818","navigation":"NAVIGATE"}]`

## ST-365 · Flow completion / Saved then 1

**Role:** Reference or alternate fixture. **Frame:** 125:8818 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-8818).

**Displayed content:**

- Version 8 saved
- Close
- Your sequence was saved before leaving.
- Document
- DNA-001 · version 8
- Next destination
- UX v4 / Home
- Continue

**Authored actions:**

- `125:8822` Action / Close → `[{"type":"BACK"}]`
- `125:8836` Action / Continue → `[{"type":"NODE","target":"132:5892","navigation":"NAVIGATE"}]`

## ST-366 · Flow completion / Leave draft 1

**Role:** Linked functional state. **Frame:** 125:8838 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-8838).

**Displayed content:**

- Save your sequence changes?
- Close
- DNA-001 has an unsaved draft based on version 7.
- Destination
- UX v4 / Notebook
- Save
- Create version 8 before continuing
- Discard
- Leave the saved version 7 unchanged
- Stay
- Return to the unsaved draft
- Stay on draft
- Discard and continue
- Save and continue

**Authored actions:**

- `125:8842` Action / Close → `[{"type":"BACK"}]`
- `125:8864` Action / Stay on draft → `[{"type":"BACK"}]`
- `125:8866` Action / Discard and continue → `[{"type":"NODE","target":"107:30702","navigation":"NAVIGATE"}]`
- `125:8868` Action / Save and continue → `[{"type":"NODE","target":"125:8870","navigation":"NAVIGATE"}]`

## ST-367 · Flow completion / Saved then 2

**Role:** Linked functional state. **Frame:** 125:8870 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-8870).

**Displayed content:**

- Version 8 saved
- Close
- Your sequence was saved before leaving.
- Document
- DNA-001 · version 8
- Next destination
- UX v4 / Notebook
- Continue

**Authored actions:**

- `125:8874` Action / Close → `[{"type":"BACK"}]`
- `125:8888` Action / Continue → `[{"type":"NODE","target":"107:30702","navigation":"NAVIGATE"}]`

## ST-368 · Flow completion / Leave draft 2

**Role:** Linked functional state. **Frame:** 125:8890 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-8890).

**Displayed content:**

- Save your sequence changes?
- Close
- DNA-001 has an unsaved draft based on version 7.
- Destination
- UX v4 / Registry
- Save
- Create version 8 before continuing
- Discard
- Leave the saved version 7 unchanged
- Stay
- Return to the unsaved draft
- Stay on draft
- Discard and continue
- Save and continue

**Authored actions:**

- `125:8894` Action / Close → `[{"type":"BACK"}]`
- `125:8916` Action / Stay on draft → `[{"type":"BACK"}]`
- `125:8918` Action / Discard and continue → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`
- `125:8920` Action / Save and continue → `[{"type":"NODE","target":"125:8922","navigation":"NAVIGATE"}]`

## ST-369 · Flow completion / Saved then 3

**Role:** Linked functional state. **Frame:** 125:8922 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-8922).

**Displayed content:**

- Version 8 saved
- Close
- Your sequence was saved before leaving.
- Document
- DNA-001 · version 8
- Next destination
- UX v4 / Registry
- Continue

**Authored actions:**

- `125:8926` Action / Close → `[{"type":"BACK"}]`
- `125:8940` Action / Continue → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`

## ST-370 · Flow completion / Leave draft 3

**Role:** Linked functional state. **Frame:** 125:8942 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-8942).

**Displayed content:**

- Save your sequence changes?
- Close
- DNA-001 has an unsaved draft based on version 7.
- Destination
- UX v4 / Inventory
- Save
- Create version 8 before continuing
- Discard
- Leave the saved version 7 unchanged
- Stay
- Return to the unsaved draft
- Stay on draft
- Discard and continue
- Save and continue

**Authored actions:**

- `125:8946` Action / Close → `[{"type":"BACK"}]`
- `125:8968` Action / Stay on draft → `[{"type":"BACK"}]`
- `125:8970` Action / Discard and continue → `[{"type":"NODE","target":"107:31089","navigation":"NAVIGATE"}]`
- `125:8972` Action / Save and continue → `[{"type":"NODE","target":"125:8974","navigation":"NAVIGATE"}]`

## ST-371 · Flow completion / Saved then 4

**Role:** Linked functional state. **Frame:** 125:8974 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-8974).

**Displayed content:**

- Version 8 saved
- Close
- Your sequence was saved before leaving.
- Document
- DNA-001 · version 8
- Next destination
- UX v4 / Inventory
- Continue

**Authored actions:**

- `125:8978` Action / Close → `[{"type":"BACK"}]`
- `125:8992` Action / Continue → `[{"type":"NODE","target":"107:31089","navigation":"NAVIGATE"}]`

## ST-372 · Flow completion / Leave draft 4

**Role:** Reference or alternate fixture. **Frame:** 125:8994 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-8994).

**Displayed content:**

- Save your sequence changes?
- Close
- DNA-001 has an unsaved draft based on version 7.
- Destination
- UX v4 / Workflows
- Save
- Create version 8 before continuing
- Discard
- Leave the saved version 7 unchanged
- Stay
- Return to the unsaved draft
- Stay on draft
- Discard and continue
- Save and continue

**Authored actions:**

- `125:8998` Action / Close → `[{"type":"BACK"}]`
- `125:9020` Action / Stay on draft → `[{"type":"BACK"}]`
- `125:9022` Action / Discard and continue → `[{"type":"NODE","target":"107:31495","navigation":"NAVIGATE"}]`
- `125:9024` Action / Save and continue → `[{"type":"NODE","target":"125:9026","navigation":"NAVIGATE"}]`

## ST-373 · Flow completion / Saved then 5

**Role:** Reference or alternate fixture. **Frame:** 125:9026 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-9026).

**Displayed content:**

- Version 8 saved
- Close
- Your sequence was saved before leaving.
- Document
- DNA-001 · version 8
- Next destination
- UX v4 / Workflows
- Continue

**Authored actions:**

- `125:9030` Action / Close → `[{"type":"BACK"}]`
- `125:9044` Action / Continue → `[{"type":"NODE","target":"107:31495","navigation":"NAVIGATE"}]`

## ST-374 · Flow completion / Leave draft 5

**Role:** Reference or alternate fixture. **Frame:** 125:9046 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-9046).

**Displayed content:**

- Save your sequence changes?
- Close
- DNA-001 has an unsaved draft based on version 7.
- Destination
- UX v4 / Insights
- Save
- Create version 8 before continuing
- Discard
- Leave the saved version 7 unchanged
- Stay
- Return to the unsaved draft
- Stay on draft
- Discard and continue
- Save and continue

**Authored actions:**

- `125:9050` Action / Close → `[{"type":"BACK"}]`
- `125:9072` Action / Stay on draft → `[{"type":"BACK"}]`
- `125:9074` Action / Discard and continue → `[{"type":"NODE","target":"107:31682","navigation":"NAVIGATE"}]`
- `125:9076` Action / Save and continue → `[{"type":"NODE","target":"125:9078","navigation":"NAVIGATE"}]`

## ST-375 · Flow completion / Saved then 6

**Role:** Reference or alternate fixture. **Frame:** 125:9078 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-9078).

**Displayed content:**

- Version 8 saved
- Close
- Your sequence was saved before leaving.
- Document
- DNA-001 · version 8
- Next destination
- UX v4 / Insights
- Continue

**Authored actions:**

- `125:9082` Action / Close → `[{"type":"BACK"}]`
- `125:9096` Action / Continue → `[{"type":"NODE","target":"107:31682","navigation":"NAVIGATE"}]`

## ST-376 · Flow completion / Leave draft 6

**Role:** Reference or alternate fixture. **Frame:** 125:9098 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-9098).

**Displayed content:**

- Save your sequence changes?
- Close
- DNA-001 has an unsaved draft based on version 7.
- Destination
- UX v4 / Automation
- Save
- Create version 8 before continuing
- Discard
- Leave the saved version 7 unchanged
- Stay
- Return to the unsaved draft
- Stay on draft
- Discard and continue
- Save and continue

**Authored actions:**

- `125:9102` Action / Close → `[{"type":"BACK"}]`
- `125:9124` Action / Stay on draft → `[{"type":"BACK"}]`
- `125:9126` Action / Discard and continue → `[{"type":"NODE","target":"107:31876","navigation":"NAVIGATE"}]`
- `125:9128` Action / Save and continue → `[{"type":"NODE","target":"125:9130","navigation":"NAVIGATE"}]`

## ST-377 · Flow completion / Saved then 7

**Role:** Reference or alternate fixture. **Frame:** 125:9130 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-9130).

**Displayed content:**

- Version 8 saved
- Close
- Your sequence was saved before leaving.
- Document
- DNA-001 · version 8
- Next destination
- UX v4 / Automation
- Continue

**Authored actions:**

- `125:9134` Action / Close → `[{"type":"BACK"}]`
- `125:9148` Action / Continue → `[{"type":"NODE","target":"107:31876","navigation":"NAVIGATE"}]`

## ST-378 · Flow completion / Leave draft 7

**Role:** Reference or alternate fixture. **Frame:** 125:9150 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-9150).

**Displayed content:**

- Save your sequence changes?
- Close
- DNA-001 has an unsaved draft based on version 7.
- Destination
- UX v4 / Split
- Save
- Create version 8 before continuing
- Discard
- Leave the saved version 7 unchanged
- Stay
- Return to the unsaved draft
- Stay on draft
- Discard and continue
- Save and continue

**Authored actions:**

- `125:9154` Action / Close → `[{"type":"BACK"}]`
- `125:9176` Action / Stay on draft → `[{"type":"BACK"}]`
- `125:9178` Action / Discard and continue → `[{"type":"NODE","target":"107:32075","navigation":"NAVIGATE"}]`
- `125:9180` Action / Save and continue → `[{"type":"NODE","target":"125:9182","navigation":"NAVIGATE"}]`

## ST-379 · Flow completion / Saved then 8

**Role:** Reference or alternate fixture. **Frame:** 125:9182 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-9182).

**Displayed content:**

- Version 8 saved
- Close
- Your sequence was saved before leaving.
- Document
- DNA-001 · version 8
- Next destination
- UX v4 / Split
- Continue

**Authored actions:**

- `125:9186` Action / Close → `[{"type":"BACK"}]`
- `125:9200` Action / Continue → `[{"type":"NODE","target":"107:32075","navigation":"NAVIGATE"}]`

## ST-380 · Flow completion / Leave draft 8

**Role:** Reference or alternate fixture. **Frame:** 125:9202 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-9202).

**Displayed content:**

- Save your sequence changes?
- Close
- DNA-001 has an unsaved draft based on version 7.
- Destination
- State / modules
- Save
- Create version 8 before continuing
- Discard
- Leave the saved version 7 unchanged
- Stay
- Return to the unsaved draft
- Stay on draft
- Discard and continue
- Save and continue

**Authored actions:**

- `125:9206` Action / Close → `[{"type":"BACK"}]`
- `125:9228` Action / Stay on draft → `[{"type":"BACK"}]`
- `125:9230` Action / Discard and continue → `[{"type":"NODE","target":"107:35127","navigation":"NAVIGATE"}]`
- `125:9232` Action / Save and continue → `[{"type":"NODE","target":"125:9234","navigation":"NAVIGATE"}]`

## ST-381 · Flow completion / Saved then 9

**Role:** Reference or alternate fixture. **Frame:** 125:9234 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-9234).

**Displayed content:**

- Version 8 saved
- Close
- Your sequence was saved before leaving.
- Document
- DNA-001 · version 8
- Next destination
- State / modules
- Continue

**Authored actions:**

- `125:9238` Action / Close → `[{"type":"BACK"}]`
- `125:9252` Action / Continue → `[{"type":"NODE","target":"107:35127","navigation":"NAVIGATE"}]`

## ST-382 · Flow completion / Leave draft 9

**Role:** Linked functional state. **Frame:** 125:9254 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-9254).

**Displayed content:**

- Save your sequence changes?
- Close
- DNA-001 has an unsaved draft based on version 7.
- Destination
- TaskHelp
- Save
- Create version 8 before continuing
- Discard
- Leave the saved version 7 unchanged
- Stay
- Return to the unsaved draft
- Stay on draft
- Discard and continue
- Save and continue

**Authored actions:**

- `125:9258` Action / Close → `[{"type":"BACK"}]`
- `125:9280` Action / Stay on draft → `[{"type":"BACK"}]`
- `125:9282` Action / Discard and continue → `[{"type":"NODE","target":"110:21252","navigation":"NAVIGATE"}]`
- `125:9284` Action / Save and continue → `[{"type":"NODE","target":"125:9286","navigation":"NAVIGATE"}]`

## ST-383 · Flow completion / Saved then 10

**Role:** Linked functional state. **Frame:** 125:9286 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-9286).

**Displayed content:**

- Version 8 saved
- Close
- Your sequence was saved before leaving.
- Document
- DNA-001 · version 8
- Next destination
- TaskHelp
- Continue

**Authored actions:**

- `125:9290` Action / Close → `[{"type":"BACK"}]`
- `125:9304` Action / Continue → `[{"type":"NODE","target":"110:21252","navigation":"NAVIGATE"}]`

## ST-384 · Flow completion / Leave draft 10

**Role:** Linked functional state. **Frame:** 125:9306 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-9306).

**Displayed content:**

- Save your sequence changes?
- Close
- DNA-001 has an unsaved draft based on version 7.
- Destination
- WorkspaceSearch
- Save
- Create version 8 before continuing
- Discard
- Leave the saved version 7 unchanged
- Stay
- Return to the unsaved draft
- Stay on draft
- Discard and continue
- Save and continue

**Authored actions:**

- `125:9310` Action / Close → `[{"type":"BACK"}]`
- `125:9332` Action / Stay on draft → `[{"type":"BACK"}]`
- `125:9334` Action / Discard and continue → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `125:9336` Action / Save and continue → `[{"type":"NODE","target":"125:9338","navigation":"NAVIGATE"}]`

## ST-385 · Flow completion / Saved then 11

**Role:** Linked functional state. **Frame:** 125:9338 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-9338).

**Displayed content:**

- Version 8 saved
- Close
- Your sequence was saved before leaving.
- Document
- DNA-001 · version 8
- Next destination
- WorkspaceSearch
- Continue

**Authored actions:**

- `125:9342` Action / Close → `[{"type":"BACK"}]`
- `125:9356` Action / Continue → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`

## ST-386 · Flow completion / Leave draft 11

**Role:** Linked functional state. **Frame:** 125:9358 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-9358).

**Displayed content:**

- Save your sequence changes?
- Close
- DNA-001 has an unsaved draft based on version 7.
- Destination
- Action panel / New
- Save
- Create version 8 before continuing
- Discard
- Leave the saved version 7 unchanged
- Stay
- Return to the unsaved draft
- Stay on draft
- Discard and continue
- Save and continue

**Authored actions:**

- `125:9362` Action / Close → `[{"type":"BACK"}]`
- `125:9384` Action / Stay on draft → `[{"type":"BACK"}]`
- `125:9386` Action / Discard and continue → `[{"type":"NODE","target":"107:28979","navigation":"NAVIGATE"}]`
- `125:9388` Action / Save and continue → `[{"type":"NODE","target":"125:9390","navigation":"NAVIGATE"}]`

## ST-387 · Flow completion / Saved then 12

**Role:** Linked functional state. **Frame:** 125:9390 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-9390).

**Displayed content:**

- Version 8 saved
- Close
- Your sequence was saved before leaving.
- Document
- DNA-001 · version 8
- Next destination
- Action panel / New
- Continue

**Authored actions:**

- `125:9394` Action / Close → `[{"type":"BACK"}]`
- `125:9408` Action / Continue → `[{"type":"NODE","target":"107:28979","navigation":"NAVIGATE"}]`

## ST-388 · Flow completion / Leave draft 12

**Role:** Linked functional state. **Frame:** 125:9410 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-9410).

**Displayed content:**

- Save your sequence changes?
- Close
- DNA-001 has an unsaved draft based on version 7.
- Destination
- Form / Export document
- Save
- Create version 8 before continuing
- Discard
- Leave the saved version 7 unchanged
- Stay
- Return to the unsaved draft
- Stay on draft
- Discard and continue
- Save and continue

**Authored actions:**

- `125:9414` Action / Close → `[{"type":"BACK"}]`
- `125:9436` Action / Stay on draft → `[{"type":"BACK"}]`
- `125:9438` Action / Discard and continue → `[{"type":"NODE","target":"107:27142","navigation":"NAVIGATE"}]`
- `125:9440` Action / Save and continue → `[{"type":"NODE","target":"125:9442","navigation":"NAVIGATE"}]`

## ST-389 · Flow completion / Saved then 13

**Role:** Linked functional state. **Frame:** 125:9442 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-9442).

**Displayed content:**

- Version 8 saved
- Close
- Your sequence was saved before leaving.
- Document
- DNA-001 · version 8
- Next destination
- Form / Export document
- Continue

**Authored actions:**

- `125:9446` Action / Close → `[{"type":"BACK"}]`
- `125:9460` Action / Continue → `[{"type":"NODE","target":"107:27142","navigation":"NAVIGATE"}]`

## ST-390 · Flow completion / Leave draft 13

**Role:** Linked functional state. **Frame:** 125:9462 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-9462).

**Displayed content:**

- Save your sequence changes?
- Close
- DNA-001 has an unsaved draft based on version 7.
- Destination
- Form / Add Feature
- Save
- Create version 8 before continuing
- Discard
- Leave the saved version 7 unchanged
- Stay
- Return to the unsaved draft
- Stay on draft
- Discard and continue
- Save and continue

**Authored actions:**

- `125:9466` Action / Close → `[{"type":"BACK"}]`
- `125:9488` Action / Stay on draft → `[{"type":"BACK"}]`
- `125:9490` Action / Discard and continue → `[{"type":"NODE","target":"107:25134","navigation":"NAVIGATE"}]`
- `125:9492` Action / Save and continue → `[{"type":"NODE","target":"125:9494","navigation":"NAVIGATE"}]`

## ST-391 · Flow completion / Saved then 14

**Role:** Linked functional state. **Frame:** 125:9494 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-9494).

**Displayed content:**

- Version 8 saved
- Close
- Your sequence was saved before leaving.
- Document
- DNA-001 · version 8
- Next destination
- Form / Add Feature
- Continue

**Authored actions:**

- `125:9498` Action / Close → `[{"type":"BACK"}]`
- `125:9512` Action / Continue → `[{"type":"NODE","target":"107:25134","navigation":"NAVIGATE"}]`

## ST-392 · Flow completion / Leave draft 14

**Role:** Linked functional state. **Frame:** 125:9514 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-9514).

**Displayed content:**

- Save your sequence changes?
- Close
- DNA-001 has an unsaved draft based on version 7.
- Destination
- AnalysisSetup
- Save
- Create version 8 before continuing
- Discard
- Leave the saved version 7 unchanged
- Stay
- Return to the unsaved draft
- Stay on draft
- Discard and continue
- Save and continue

**Authored actions:**

- `125:9518` Action / Close → `[{"type":"BACK"}]`
- `125:9540` Action / Stay on draft → `[{"type":"BACK"}]`
- `125:9542` Action / Discard and continue → `[{"type":"NODE","target":"110:20288","navigation":"NAVIGATE"}]`
- `125:9544` Action / Save and continue → `[{"type":"NODE","target":"125:9546","navigation":"NAVIGATE"}]`

## ST-393 · Flow completion / Saved then 15

**Role:** Linked functional state. **Frame:** 125:9546 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-9546).

**Displayed content:**

- Version 8 saved
- Close
- Your sequence was saved before leaving.
- Document
- DNA-001 · version 8
- Next destination
- AnalysisSetup
- Continue

**Authored actions:**

- `125:9550` Action / Close → `[{"type":"BACK"}]`
- `125:9564` Action / Continue → `[{"type":"NODE","target":"110:20288","navigation":"NAVIGATE"}]`

## ST-394 · Flow completion / Leave draft 15

**Role:** Linked functional state. **Frame:** 125:9566 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-9566).

**Displayed content:**

- Save your sequence changes?
- Close
- DNA-001 has an unsaved draft based on version 7.
- Destination
- Form / Map & Sequence Options
- Save
- Create version 8 before continuing
- Discard
- Leave the saved version 7 unchanged
- Stay
- Return to the unsaved draft
- Stay on draft
- Discard and continue
- Save and continue

**Authored actions:**

- `125:9570` Action / Close → `[{"type":"BACK"}]`
- `125:9592` Action / Stay on draft → `[{"type":"BACK"}]`
- `125:9594` Action / Discard and continue → `[{"type":"NODE","target":"107:25217","navigation":"NAVIGATE"}]`
- `125:9596` Action / Save and continue → `[{"type":"NODE","target":"125:9598","navigation":"NAVIGATE"}]`

## ST-395 · Flow completion / Saved then 16

**Role:** Linked functional state. **Frame:** 125:9598 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-9598).

**Displayed content:**

- Version 8 saved
- Close
- Your sequence was saved before leaving.
- Document
- DNA-001 · version 8
- Next destination
- Form / Map & Sequence Options
- Continue

**Authored actions:**

- `125:9602` Action / Close → `[{"type":"BACK"}]`
- `125:9616` Action / Continue → `[{"type":"NODE","target":"107:25217","navigation":"NAVIGATE"}]`

## ST-396 · Flow completion / Leave draft 16

**Role:** Linked functional state. **Frame:** 125:9618 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-9618).

**Displayed content:**

- Save your sequence changes?
- Close
- DNA-001 has an unsaved draft based on version 7.
- Destination
- UX v4 / Features
- Save
- Create version 8 before continuing
- Discard
- Leave the saved version 7 unchanged
- Stay
- Return to the unsaved draft
- Stay on draft
- Discard and continue
- Save and continue

**Authored actions:**

- `125:9622` Action / Close → `[{"type":"BACK"}]`
- `125:9644` Action / Stay on draft → `[{"type":"BACK"}]`
- `125:9646` Action / Discard and continue → `[{"type":"NODE","target":"107:29886","navigation":"NAVIGATE"}]`
- `125:9648` Action / Save and continue → `[{"type":"NODE","target":"125:9650","navigation":"NAVIGATE"}]`

## ST-397 · Flow completion / Saved then 17

**Role:** Linked functional state. **Frame:** 125:9650 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-9650).

**Displayed content:**

- Version 8 saved
- Close
- Your sequence was saved before leaving.
- Document
- DNA-001 · version 8
- Next destination
- UX v4 / Features
- Continue

**Authored actions:**

- `125:9654` Action / Close → `[{"type":"BACK"}]`
- `125:9668` Action / Continue → `[{"type":"NODE","target":"130:5852","navigation":"NAVIGATE"}]`

## ST-398 · Flow completion / Leave draft 17

**Role:** Linked functional state. **Frame:** 125:9670 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-9670).

**Displayed content:**

- Save your sequence changes?
- Close
- DNA-001 has an unsaved draft based on version 7.
- Destination
- UX v4 flow / GroupedTools
- Save
- Create version 8 before continuing
- Discard
- Leave the saved version 7 unchanged
- Stay
- Return to the unsaved draft
- Stay on draft
- Discard and continue
- Save and continue

**Authored actions:**

- `125:9674` Action / Close → `[{"type":"BACK"}]`
- `125:9696` Action / Stay on draft → `[{"type":"BACK"}]`
- `125:9698` Action / Discard and continue → `[{"type":"NODE","target":"110:22016","navigation":"NAVIGATE"}]`
- `125:9700` Action / Save and continue → `[{"type":"NODE","target":"125:9702","navigation":"NAVIGATE"}]`

## ST-399 · Flow completion / Saved then 18

**Role:** Linked functional state. **Frame:** 125:9702 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-9702).

**Displayed content:**

- Version 8 saved
- Close
- Your sequence was saved before leaving.
- Document
- DNA-001 · version 8
- Next destination
- UX v4 flow / GroupedTools
- Continue

**Authored actions:**

- `125:9706` Action / Close → `[{"type":"BACK"}]`
- `125:9720` Action / Continue → `[{"type":"NODE","target":"110:22016","navigation":"NAVIGATE"}]`

## ST-400 · Flow completion / Leave draft 18

**Role:** Linked functional state. **Frame:** 125:9722 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-9722).

**Displayed content:**

- Save your sequence changes?
- Close
- DNA-001 has an unsaved draft based on version 7.
- Destination
- Form / Add Primer
- Save
- Create version 8 before continuing
- Discard
- Leave the saved version 7 unchanged
- Stay
- Return to the unsaved draft
- Stay on draft
- Discard and continue
- Save and continue

**Authored actions:**

- `125:9726` Action / Close → `[{"type":"BACK"}]`
- `125:9748` Action / Stay on draft → `[{"type":"BACK"}]`
- `125:9750` Action / Discard and continue → `[{"type":"NODE","target":"107:25182","navigation":"NAVIGATE"}]`
- `125:9752` Action / Save and continue → `[{"type":"NODE","target":"125:9754","navigation":"NAVIGATE"}]`

## ST-401 · Flow completion / Saved then 19

**Role:** Linked functional state. **Frame:** 125:9754 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-9754).

**Displayed content:**

- Version 8 saved
- Close
- Your sequence was saved before leaving.
- Document
- DNA-001 · version 8
- Next destination
- Form / Add Primer
- Continue

**Authored actions:**

- `125:9758` Action / Close → `[{"type":"BACK"}]`
- `125:9772` Action / Continue → `[{"type":"NODE","target":"107:25182","navigation":"NAVIGATE"}]`

## ST-402 · Flow completion / Leave draft 19

**Role:** Linked functional state. **Frame:** 125:9774 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-9774).

**Displayed content:**

- Save your sequence changes?
- Close
- DNA-001 has an unsaved draft based on version 7.
- Destination
- Form / Primer Design
- Save
- Create version 8 before continuing
- Discard
- Leave the saved version 7 unchanged
- Stay
- Return to the unsaved draft
- Stay on draft
- Discard and continue
- Save and continue

**Authored actions:**

- `125:9778` Action / Close → `[{"type":"BACK"}]`
- `125:9800` Action / Stay on draft → `[{"type":"BACK"}]`
- `125:9802` Action / Discard and continue → `[{"type":"NODE","target":"107:25303","navigation":"NAVIGATE"}]`
- `125:9804` Action / Save and continue → `[{"type":"NODE","target":"125:9806","navigation":"NAVIGATE"}]`

## ST-403 · Flow completion / Saved then 20

**Role:** Linked functional state. **Frame:** 125:9806 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-9806).

**Displayed content:**

- Version 8 saved
- Close
- Your sequence was saved before leaving.
- Document
- DNA-001 · version 8
- Next destination
- Form / Primer Design
- Continue

**Authored actions:**

- `125:9810` Action / Close → `[{"type":"BACK"}]`
- `125:9824` Action / Continue → `[{"type":"NODE","target":"107:25303","navigation":"NAVIGATE"}]`

## ST-404 · Flow completion / Leave draft 20

**Role:** Linked functional state. **Frame:** 125:9846 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-9846).

**Displayed content:**

- Save your sequence changes?
- Close
- DNA-001 has an unsaved draft based on version 7.
- Destination
- UX v4 / Map
- Save
- Create version 8 before continuing
- Discard
- Leave the saved version 7 unchanged
- Stay
- Return to the unsaved draft
- Stay on draft
- Discard and continue
- Save and continue

**Authored actions:**

- `125:9850` Action / Close → `[{"type":"BACK"}]`
- `125:9872` Action / Stay on draft → `[{"type":"BACK"}]`
- `125:9874` Action / Discard and continue → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`
- `125:9876` Action / Save and continue → `[{"type":"NODE","target":"125:9878","navigation":"NAVIGATE"}]`

## ST-405 · Flow completion / Saved then 21

**Role:** Linked functional state. **Frame:** 125:9878 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-9878).

**Displayed content:**

- Version 8 saved
- Close
- Your sequence was saved before leaving.
- Document
- DNA-001 · version 8
- Next destination
- UX v4 / Map
- Continue

**Authored actions:**

- `125:9882` Action / Close → `[{"type":"BACK"}]`
- `125:9896` Action / Continue → `[{"type":"NODE","target":"125:8508","navigation":"NAVIGATE"}]`

## ST-406 · Flow completion / Leave draft 21

**Role:** Linked functional state. **Frame:** 125:9898 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-9898).

**Displayed content:**

- Save your sequence changes?
- Close
- DNA-001 has an unsaved draft based on version 7.
- Destination
- State / SequenceFind
- Save
- Create version 8 before continuing
- Discard
- Leave the saved version 7 unchanged
- Stay
- Return to the unsaved draft
- Stay on draft
- Discard and continue
- Save and continue

**Authored actions:**

- `125:9902` Action / Close → `[{"type":"BACK"}]`
- `125:9924` Action / Stay on draft → `[{"type":"BACK"}]`
- `125:9926` Action / Discard and continue → `[{"type":"NODE","target":"107:33290","navigation":"NAVIGATE"}]`
- `125:9928` Action / Save and continue → `[{"type":"NODE","target":"125:9930","navigation":"NAVIGATE"}]`

## ST-407 · Flow completion / Saved then 22

**Role:** Linked functional state. **Frame:** 125:9930 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-9930).

**Displayed content:**

- Version 8 saved
- Close
- Your sequence was saved before leaving.
- Document
- DNA-001 · version 8
- Next destination
- State / SequenceFind
- Continue

**Authored actions:**

- `125:9934` Action / Close → `[{"type":"BACK"}]`
- `125:9948` Action / Continue → `[{"type":"NODE","target":"107:33290","navigation":"NAVIGATE"}]`

## ST-408 · Flow completion / Leave draft 22

**Role:** Linked functional state. **Frame:** 125:9950 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-9950).

**Displayed content:**

- Save your sequence changes?
- Close
- DNA-001 has an unsaved draft based on version 7.
- Destination
- Form / Edit sequence
- Save
- Create version 8 before continuing
- Discard
- Leave the saved version 7 unchanged
- Stay
- Return to the unsaved draft
- Stay on draft
- Discard and continue
- Save and continue

**Authored actions:**

- `125:9954` Action / Close → `[{"type":"BACK"}]`
- `125:9976` Action / Stay on draft → `[{"type":"BACK"}]`
- `125:9978` Action / Discard and continue → `[{"type":"NODE","target":"107:29125","navigation":"NAVIGATE"}]`
- `125:9980` Action / Save and continue → `[{"type":"NODE","target":"125:9982","navigation":"NAVIGATE"}]`

## ST-409 · Flow completion / Saved then 23

**Role:** Linked functional state. **Frame:** 125:9982 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-9982).

**Displayed content:**

- Version 8 saved
- Close
- Your sequence was saved before leaving.
- Document
- DNA-001 · version 8
- Next destination
- Form / Edit sequence
- Continue

**Authored actions:**

- `125:9986` Action / Close → `[{"type":"BACK"}]`
- `125:10000` Action / Continue → `[{"type":"NODE","target":"107:29125","navigation":"NAVIGATE"}]`

## ST-410 · Flow completion / Leave draft 23

**Role:** Linked functional state. **Frame:** 125:10002 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-10002).

**Displayed content:**

- Save your sequence changes?
- Close
- DNA-001 has an unsaved draft based on version 7.
- Destination
- Form / Local BLAST
- Save
- Create version 8 before continuing
- Discard
- Leave the saved version 7 unchanged
- Stay
- Return to the unsaved draft
- Stay on draft
- Discard and continue
- Save and continue

**Authored actions:**

- `125:10006` Action / Close → `[{"type":"BACK"}]`
- `125:10028` Action / Stay on draft → `[{"type":"BACK"}]`
- `125:10030` Action / Discard and continue → `[{"type":"NODE","target":"107:25595","navigation":"NAVIGATE"}]`
- `125:10032` Action / Save and continue → `[{"type":"NODE","target":"125:10034","navigation":"NAVIGATE"}]`

## ST-411 · Flow completion / Saved then 24

**Role:** Linked functional state. **Frame:** 125:10034 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-10034).

**Displayed content:**

- Version 8 saved
- Close
- Your sequence was saved before leaving.
- Document
- DNA-001 · version 8
- Next destination
- Form / Local BLAST
- Continue

**Authored actions:**

- `125:10038` Action / Close → `[{"type":"BACK"}]`
- `125:10052` Action / Continue → `[{"type":"NODE","target":"107:25595","navigation":"NAVIGATE"}]`

## ST-412 · Flow completion / Leave draft 24

**Role:** Linked functional state. **Frame:** 125:10054 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-10054).

**Displayed content:**

- Save your sequence changes?
- Close
- DNA-001 has an unsaved draft based on version 7.
- Destination
- UX v4 / Primers
- Save
- Create version 8 before continuing
- Discard
- Leave the saved version 7 unchanged
- Stay
- Return to the unsaved draft
- Stay on draft
- Discard and continue
- Save and continue

**Authored actions:**

- `125:10058` Action / Close → `[{"type":"BACK"}]`
- `125:10080` Action / Stay on draft → `[{"type":"BACK"}]`
- `125:10082` Action / Discard and continue → `[{"type":"NODE","target":"107:30093","navigation":"NAVIGATE"}]`
- `125:10084` Action / Save and continue → `[{"type":"NODE","target":"125:10086","navigation":"NAVIGATE"}]`

## ST-413 · Flow completion / Saved then 25

**Role:** Linked functional state. **Frame:** 125:10086 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-10086).

**Displayed content:**

- Version 8 saved
- Close
- Your sequence was saved before leaving.
- Document
- DNA-001 · version 8
- Next destination
- UX v4 / Primers
- Continue

**Authored actions:**

- `125:10090` Action / Close → `[{"type":"BACK"}]`
- `125:10104` Action / Continue → `[{"type":"NODE","target":"130:6062","navigation":"NAVIGATE"}]`

## ST-414 · Flow completion / Leave draft 25

**Role:** Linked functional state. **Frame:** 125:10126 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-10126).

**Displayed content:**

- Save your sequence changes?
- Close
- DNA-001 has an unsaved draft based on version 7.
- Destination
- Form / Detect common features
- Save
- Create version 8 before continuing
- Discard
- Leave the saved version 7 unchanged
- Stay
- Return to the unsaved draft
- Stay on draft
- Discard and continue
- Save and continue

**Authored actions:**

- `125:10130` Action / Close → `[{"type":"BACK"}]`
- `125:10152` Action / Stay on draft → `[{"type":"BACK"}]`
- `125:10154` Action / Discard and continue → `[{"type":"NODE","target":"107:26060","navigation":"NAVIGATE"}]`
- `125:10156` Action / Save and continue → `[{"type":"NODE","target":"125:10158","navigation":"NAVIGATE"}]`

## ST-415 · Flow completion / Saved then 26

**Role:** Linked functional state. **Frame:** 125:10158 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-10158).

**Displayed content:**

- Version 8 saved
- Close
- Your sequence was saved before leaving.
- Document
- DNA-001 · version 8
- Next destination
- Form / Detect common features
- Continue

**Authored actions:**

- `125:10162` Action / Close → `[{"type":"BACK"}]`
- `125:10176` Action / Continue → `[{"type":"NODE","target":"107:26060","navigation":"NAVIGATE"}]`

## ST-416 · Flow completion / Leave draft 26

**Role:** Linked functional state. **Frame:** 125:10178 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-10178).

**Displayed content:**

- Save your sequence changes?
- Close
- DNA-001 has an unsaved draft based on version 7.
- Destination
- State / AnnotationPreview
- Save
- Create version 8 before continuing
- Discard
- Leave the saved version 7 unchanged
- Stay
- Return to the unsaved draft
- Stay on draft
- Discard and continue
- Save and continue

**Authored actions:**

- `125:10182` Action / Close → `[{"type":"BACK"}]`
- `125:10204` Action / Stay on draft → `[{"type":"BACK"}]`
- `125:10206` Action / Discard and continue → `[{"type":"NODE","target":"107:33313","navigation":"NAVIGATE"}]`
- `125:10208` Action / Save and continue → `[{"type":"NODE","target":"125:10210","navigation":"NAVIGATE"}]`

## ST-417 · Flow completion / Saved then 27

**Role:** Linked functional state. **Frame:** 125:10210 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-10210).

**Displayed content:**

- Version 8 saved
- Close
- Your sequence was saved before leaving.
- Document
- DNA-001 · version 8
- Next destination
- State / AnnotationPreview
- Continue

**Authored actions:**

- `125:10214` Action / Close → `[{"type":"BACK"}]`
- `125:10228` Action / Continue → `[{"type":"NODE","target":"107:33313","navigation":"NAVIGATE"}]`

## ST-418 · Flow completion / Leave draft 27

**Role:** Linked functional state. **Frame:** 125:10230 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-10230).

**Displayed content:**

- Save your sequence changes?
- Close
- DNA-001 has an unsaved draft based on version 7.
- Destination
- Page tools / Features
- Save
- Create version 8 before continuing
- Discard
- Leave the saved version 7 unchanged
- Stay
- Return to the unsaved draft
- Stay on draft
- Discard and continue
- Save and continue

**Authored actions:**

- `125:10234` Action / Close → `[{"type":"BACK"}]`
- `125:10256` Action / Stay on draft → `[{"type":"BACK"}]`
- `125:10258` Action / Discard and continue → `[{"type":"NODE","target":"107:28097","navigation":"NAVIGATE"}]`
- `125:10260` Action / Save and continue → `[{"type":"NODE","target":"125:10262","navigation":"NAVIGATE"}]`

## ST-419 · Flow completion / Saved then 28

**Role:** Linked functional state. **Frame:** 125:10262 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-10262).

**Displayed content:**

- Version 8 saved
- Close
- Your sequence was saved before leaving.
- Document
- DNA-001 · version 8
- Next destination
- Page tools / Features
- Continue

**Authored actions:**

- `125:10266` Action / Close → `[{"type":"BACK"}]`
- `125:10280` Action / Continue → `[{"type":"NODE","target":"107:28097","navigation":"NAVIGATE"}]`

## ST-420 · Flow completion / Leave draft 28

**Role:** Linked functional state. **Frame:** 125:10302 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-10302).

**Displayed content:**

- Save your sequence changes?
- Close
- DNA-001 has an unsaved draft based on version 7.
- Destination
- Form / PCR Simulation
- Save
- Create version 8 before continuing
- Discard
- Leave the saved version 7 unchanged
- Stay
- Return to the unsaved draft
- Stay on draft
- Discard and continue
- Save and continue

**Authored actions:**

- `125:10306` Action / Close → `[{"type":"BACK"}]`
- `125:10328` Action / Stay on draft → `[{"type":"BACK"}]`
- `125:10330` Action / Discard and continue → `[{"type":"NODE","target":"107:25266","navigation":"NAVIGATE"}]`
- `125:10332` Action / Save and continue → `[{"type":"NODE","target":"125:10334","navigation":"NAVIGATE"}]`

## ST-421 · Flow completion / Saved then 29

**Role:** Linked functional state. **Frame:** 125:10334 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-10334).

**Displayed content:**

- Version 8 saved
- Close
- Your sequence was saved before leaving.
- Document
- DNA-001 · version 8
- Next destination
- Form / PCR Simulation
- Continue

**Authored actions:**

- `125:10338` Action / Close → `[{"type":"BACK"}]`
- `125:10352` Action / Continue → `[{"type":"NODE","target":"107:25266","navigation":"NAVIGATE"}]`

## ST-422 · Flow completion / Leave draft 29

**Role:** Linked functional state. **Frame:** 125:10354 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-10354).

**Displayed content:**

- Save your sequence changes?
- Close
- DNA-001 has an unsaved draft based on version 7.
- Destination
- Page tools / Primers
- Save
- Create version 8 before continuing
- Discard
- Leave the saved version 7 unchanged
- Stay
- Return to the unsaved draft
- Stay on draft
- Discard and continue
- Save and continue

**Authored actions:**

- `125:10358` Action / Close → `[{"type":"BACK"}]`
- `125:10380` Action / Stay on draft → `[{"type":"BACK"}]`
- `125:10382` Action / Discard and continue → `[{"type":"NODE","target":"107:28166","navigation":"NAVIGATE"}]`
- `125:10384` Action / Save and continue → `[{"type":"NODE","target":"125:10386","navigation":"NAVIGATE"}]`

## ST-423 · Flow completion / Saved then 30

**Role:** Linked functional state. **Frame:** 125:10386 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-10386).

**Displayed content:**

- Version 8 saved
- Close
- Your sequence was saved before leaving.
- Document
- DNA-001 · version 8
- Next destination
- Page tools / Primers
- Continue

**Authored actions:**

- `125:10390` Action / Close → `[{"type":"BACK"}]`
- `125:10404` Action / Continue → `[{"type":"NODE","target":"107:28166","navigation":"NAVIGATE"}]`

## ST-424 · Flow completion / Leave draft 30

**Role:** Linked functional state. **Frame:** 125:10406 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-10406).

**Displayed content:**

- Save your sequence changes?
- Close
- DNA-001 has an unsaved draft based on version 7.
- Destination
- State / PcrResult
- Save
- Create version 8 before continuing
- Discard
- Leave the saved version 7 unchanged
- Stay
- Return to the unsaved draft
- Stay on draft
- Discard and continue
- Save and continue

**Authored actions:**

- `125:10410` Action / Close → `[{"type":"BACK"}]`
- `125:10432` Action / Stay on draft → `[{"type":"BACK"}]`
- `125:10434` Action / Discard and continue → `[{"type":"NODE","target":"107:33352","navigation":"NAVIGATE"}]`
- `125:10436` Action / Save and continue → `[{"type":"NODE","target":"125:10438","navigation":"NAVIGATE"}]`

## ST-425 · Flow completion / Saved then 31

**Role:** Linked functional state. **Frame:** 125:10438 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-10438).

**Displayed content:**

- Version 8 saved
- Close
- Your sequence was saved before leaving.
- Document
- DNA-001 · version 8
- Next destination
- State / PcrResult
- Continue

**Authored actions:**

- `125:10442` Action / Close → `[{"type":"BACK"}]`
- `125:10456` Action / Continue → `[{"type":"NODE","target":"107:33352","navigation":"NAVIGATE"}]`

## ST-426 · Flow completion / Leave draft 31

**Role:** Linked functional state. **Frame:** 125:10478 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-10478).

**Displayed content:**

- Save your sequence changes?
- Close
- DNA-001 has an unsaved draft based on version 7.
- Destination
- Form / Enzyme filters
- Save
- Create version 8 before continuing
- Discard
- Leave the saved version 7 unchanged
- Stay
- Return to the unsaved draft
- Stay on draft
- Discard and continue
- Save and continue

**Authored actions:**

- `125:10482` Action / Close → `[{"type":"BACK"}]`
- `125:10504` Action / Stay on draft → `[{"type":"BACK"}]`
- `125:10506` Action / Discard and continue → `[{"type":"NODE","target":"107:26978","navigation":"NAVIGATE"}]`
- `125:10508` Action / Save and continue → `[{"type":"NODE","target":"125:10510","navigation":"NAVIGATE"}]`

## ST-427 · Flow completion / Saved then 32

**Role:** Linked functional state. **Frame:** 125:10510 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-10510).

**Displayed content:**

- Version 8 saved
- Close
- Your sequence was saved before leaving.
- Document
- DNA-001 · version 8
- Next destination
- Form / Enzyme filters
- Continue

**Authored actions:**

- `125:10514` Action / Close → `[{"type":"BACK"}]`
- `125:10528` Action / Continue → `[{"type":"NODE","target":"107:26978","navigation":"NAVIGATE"}]`

## ST-428 · Flow completion / Leave draft 32

**Role:** Linked functional state. **Frame:** 125:10530 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-10530).

**Displayed content:**

- Save your sequence changes?
- Close
- DNA-001 has an unsaved draft based on version 7.
- Destination
- Page tools / Enzymes
- Save
- Create version 8 before continuing
- Discard
- Leave the saved version 7 unchanged
- Stay
- Return to the unsaved draft
- Stay on draft
- Discard and continue
- Save and continue

**Authored actions:**

- `125:10534` Action / Close → `[{"type":"BACK"}]`
- `125:10556` Action / Stay on draft → `[{"type":"BACK"}]`
- `125:10558` Action / Discard and continue → `[{"type":"NODE","target":"107:28231","navigation":"NAVIGATE"}]`
- `125:10560` Action / Save and continue → `[{"type":"NODE","target":"125:10562","navigation":"NAVIGATE"}]`

## ST-429 · Flow completion / Saved then 33

**Role:** Linked functional state. **Frame:** 125:10562 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-10562).

**Displayed content:**

- Version 8 saved
- Close
- Your sequence was saved before leaving.
- Document
- DNA-001 · version 8
- Next destination
- Page tools / Enzymes
- Continue

**Authored actions:**

- `125:10566` Action / Close → `[{"type":"BACK"}]`
- `125:10580` Action / Continue → `[{"type":"NODE","target":"107:28231","navigation":"NAVIGATE"}]`

## ST-430 · Flow completion / Leave draft 33

**Role:** Linked functional state. **Frame:** 125:10582 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-10582).

**Displayed content:**

- Save your sequence changes?
- Close
- DNA-001 has an unsaved draft based on version 7.
- Destination
- State / DigestResult
- Save
- Create version 8 before continuing
- Discard
- Leave the saved version 7 unchanged
- Stay
- Return to the unsaved draft
- Stay on draft
- Discard and continue
- Save and continue

**Authored actions:**

- `125:10586` Action / Close → `[{"type":"BACK"}]`
- `125:10608` Action / Stay on draft → `[{"type":"BACK"}]`
- `125:10610` Action / Discard and continue → `[{"type":"NODE","target":"107:33375","navigation":"NAVIGATE"}]`
- `125:10612` Action / Save and continue → `[{"type":"NODE","target":"125:10614","navigation":"NAVIGATE"}]`

## ST-431 · Flow completion / Saved then 34

**Role:** Linked functional state. **Frame:** 125:10614 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-10614).

**Displayed content:**

- Version 8 saved
- Close
- Your sequence was saved before leaving.
- Document
- DNA-001 · version 8
- Next destination
- State / DigestResult
- Continue

**Authored actions:**

- `125:10618` Action / Close → `[{"type":"BACK"}]`
- `125:10632` Action / Continue → `[{"type":"NODE","target":"107:33375","navigation":"NAVIGATE"}]`

## ST-432 · Flow completion / Leave draft 34

**Role:** Linked functional state. **Frame:** 125:10654 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-10654).

**Displayed content:**

- Save your sequence changes?
- Close
- DNA-001 has an unsaved draft based on version 7.
- Destination
- State / VersionCompare
- Save
- Create version 8 before continuing
- Discard
- Leave the saved version 7 unchanged
- Stay
- Return to the unsaved draft
- Stay on draft
- Discard and continue
- Save and continue

**Authored actions:**

- `125:10658` Action / Close → `[{"type":"BACK"}]`
- `125:10680` Action / Stay on draft → `[{"type":"BACK"}]`
- `125:10682` Action / Discard and continue → `[{"type":"NODE","target":"107:33405","navigation":"NAVIGATE"}]`
- `125:10684` Action / Save and continue → `[{"type":"NODE","target":"125:10686","navigation":"NAVIGATE"}]`

## ST-433 · Flow completion / Saved then 35

**Role:** Linked functional state. **Frame:** 125:10686 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-10686).

**Displayed content:**

- Version 8 saved
- Close
- Your sequence was saved before leaving.
- Document
- DNA-001 · version 8
- Next destination
- State / VersionCompare
- Continue

**Authored actions:**

- `125:10690` Action / Close → `[{"type":"BACK"}]`
- `125:10704` Action / Continue → `[{"type":"NODE","target":"107:33405","navigation":"NAVIGATE"}]`

## ST-434 · Flow completion / Leave draft 35

**Role:** Linked functional state. **Frame:** 125:10706 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-10706).

**Displayed content:**

- Save your sequence changes?
- Close
- DNA-001 has an unsaved draft based on version 7.
- Destination
- Form / Save document version
- Save
- Create version 8 before continuing
- Discard
- Leave the saved version 7 unchanged
- Stay
- Return to the unsaved draft
- Stay on draft
- Discard and continue
- Save and continue

**Authored actions:**

- `125:10710` Action / Close → `[{"type":"BACK"}]`
- `125:10732` Action / Stay on draft → `[{"type":"BACK"}]`
- `125:10734` Action / Discard and continue → `[{"type":"NODE","target":"107:29025","navigation":"NAVIGATE"}]`
- `125:10736` Action / Save and continue → `[{"type":"NODE","target":"125:10738","navigation":"NAVIGATE"}]`

## ST-435 · Flow completion / Saved then 36

**Role:** Linked functional state. **Frame:** 125:10738 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-10738).

**Displayed content:**

- Version 8 saved
- Close
- Your sequence was saved before leaving.
- Document
- DNA-001 · version 8
- Next destination
- Form / Save document version
- Continue

**Authored actions:**

- `125:10742` Action / Close → `[{"type":"BACK"}]`
- `125:10756` Action / Continue → `[{"type":"NODE","target":"107:29025","navigation":"NAVIGATE"}]`

## ST-436 · Flow completion / Leave draft 36

**Role:** Linked functional state. **Frame:** 125:10758 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-10758).

**Displayed content:**

- Save your sequence changes?
- Close
- DNA-001 has an unsaved draft based on version 7.
- Destination
- State / RestorePreview
- Save
- Create version 8 before continuing
- Discard
- Leave the saved version 7 unchanged
- Stay
- Return to the unsaved draft
- Stay on draft
- Discard and continue
- Save and continue

**Authored actions:**

- `125:10762` Action / Close → `[{"type":"BACK"}]`
- `125:10784` Action / Stay on draft → `[{"type":"BACK"}]`
- `125:10786` Action / Discard and continue → `[{"type":"NODE","target":"107:33433","navigation":"NAVIGATE"}]`
- `125:10788` Action / Save and continue → `[{"type":"NODE","target":"125:10790","navigation":"NAVIGATE"}]`

## ST-437 · Flow completion / Saved then 37

**Role:** Linked functional state. **Frame:** 125:10790 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-10790).

**Displayed content:**

- Version 8 saved
- Close
- Your sequence was saved before leaving.
- Document
- DNA-001 · version 8
- Next destination
- State / RestorePreview
- Continue

**Authored actions:**

- `125:10794` Action / Close → `[{"type":"BACK"}]`
- `125:10808` Action / Continue → `[{"type":"NODE","target":"107:33433","navigation":"NAVIGATE"}]`

## ST-438 · Flow completion / Leave draft 37

**Role:** Linked functional state. **Frame:** 125:10810 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-10810).

**Displayed content:**

- Save your sequence changes?
- Close
- DNA-001 has an unsaved draft based on version 7.
- Destination
- Page tools / History
- Save
- Create version 8 before continuing
- Discard
- Leave the saved version 7 unchanged
- Stay
- Return to the unsaved draft
- Stay on draft
- Discard and continue
- Save and continue

**Authored actions:**

- `125:10814` Action / Close → `[{"type":"BACK"}]`
- `125:10836` Action / Stay on draft → `[{"type":"BACK"}]`
- `125:10838` Action / Discard and continue → `[{"type":"NODE","target":"107:28296","navigation":"NAVIGATE"}]`
- `125:10840` Action / Save and continue → `[{"type":"NODE","target":"125:10842","navigation":"NAVIGATE"}]`

## ST-439 · Flow completion / Saved then 38

**Role:** Linked functional state. **Frame:** 125:10842 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-10842).

**Displayed content:**

- Version 8 saved
- Close
- Your sequence was saved before leaving.
- Document
- DNA-001 · version 8
- Next destination
- Page tools / History
- Continue

**Authored actions:**

- `125:10846` Action / Close → `[{"type":"BACK"}]`
- `125:10860` Action / Continue → `[{"type":"NODE","target":"107:28296","navigation":"NAVIGATE"}]`

## ST-440 · Flow completion / Leave draft 38

**Role:** Linked functional state. **Frame:** 125:10862 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-10862).

**Displayed content:**

- Save your sequence changes?
- Close
- DNA-001 has an unsaved draft based on version 7.
- Destination
- UX v4 / Sequence
- Save
- Create version 8 before continuing
- Discard
- Leave the saved version 7 unchanged
- Stay
- Return to the unsaved draft
- Stay on draft
- Discard and continue
- Save and continue

**Authored actions:**

- `125:10866` Action / Close → `[{"type":"BACK"}]`
- `125:10888` Action / Stay on draft → `[{"type":"BACK"}]`
- `125:10890` Action / Discard and continue → `[{"type":"NODE","target":"107:29677","navigation":"NAVIGATE"}]`
- `125:10892` Action / Save and continue → `[{"type":"NODE","target":"125:10894","navigation":"NAVIGATE"}]`

## ST-441 · Flow completion / Saved then 39

**Role:** Linked functional state. **Frame:** 125:10894 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-10894).

**Displayed content:**

- Version 8 saved
- Close
- Your sequence was saved before leaving.
- Document
- DNA-001 · version 8
- Next destination
- UX v4 / Sequence
- Continue

**Authored actions:**

- `125:10898` Action / Close → `[{"type":"BACK"}]`
- `125:10912` Action / Continue → `[{"type":"NODE","target":"130:5640","navigation":"NAVIGATE"}]`

## ST-442 · Flow completion / Leave draft 39

**Role:** Linked functional state. **Frame:** 125:10914 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-10914).

**Displayed content:**

- Save your sequence changes?
- Close
- DNA-001 has an unsaved draft based on version 7.
- Destination
- State / AuditTrail
- Save
- Create version 8 before continuing
- Discard
- Leave the saved version 7 unchanged
- Stay
- Return to the unsaved draft
- Stay on draft
- Discard and continue
- Save and continue

**Authored actions:**

- `125:10918` Action / Close → `[{"type":"BACK"}]`
- `125:10940` Action / Stay on draft → `[{"type":"BACK"}]`
- `125:10942` Action / Discard and continue → `[{"type":"NODE","target":"107:33461","navigation":"NAVIGATE"}]`
- `125:10944` Action / Save and continue → `[{"type":"NODE","target":"125:10946","navigation":"NAVIGATE"}]`

## ST-443 · Flow completion / Saved then 40

**Role:** Linked functional state. **Frame:** 125:10946 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-10946).

**Displayed content:**

- Version 8 saved
- Close
- Your sequence was saved before leaving.
- Document
- DNA-001 · version 8
- Next destination
- State / AuditTrail
- Continue

**Authored actions:**

- `125:10950` Action / Close → `[{"type":"BACK"}]`
- `125:10964` Action / Continue → `[{"type":"NODE","target":"107:33461","navigation":"NAVIGATE"}]`

## ST-444 · Flow completion / CloseDraft

**Role:** Linked functional state. **Frame:** 125:10969 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-10969).

**Displayed content:**

- Close this unsaved document?
- Close
- Save, discard, or stay with DNA-001.
- Document
- DNA-001 · unsaved draft
- Saved checkpoint
- Version 7
- Stay
- Discard and close
- Save and close

**Authored actions:**

- `125:10973` Action / Close → `[{"type":"BACK"}]`
- `125:10987` Action / Stay → `[{"type":"NODE","target":"115:3326","navigation":"NAVIGATE"}]`
- `125:10989` Action / Discard and close → `[{"type":"NODE","target":"107:29237","navigation":"NAVIGATE"}]`
- `125:10991` Action / Save and close → `[{"type":"NODE","target":"128:5573","navigation":"NAVIGATE"}]`

## ST-445 · Flow completion / ResultOutdated

**Role:** Linked functional state. **Frame:** 125:10993 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=125-10993).

**Displayed content:**

- Result uses an older source version
- Close
- RES-001 was calculated from DNA-001 version 7. Version 8 is now available.
- Result input
- DNA-001 · version 7
- Current source
- DNA-001 · version 8
- Meaning
- The saved result is unchanged; it has not been recalculated
- Rerun with version 8
- Inspect saved result
- Review current source

**Authored actions:**

- `125:10997` Action / Close → `[{"type":"BACK"}]`
- `126:7478` Action / Rerun with version 8 → `[{"type":"NODE","target":"126:7098","navigation":"NAVIGATE"}]`
- `125:11015` Action / Inspect saved result → `[{"type":"NODE","target":"110:20596","navigation":"NAVIGATE"}]`
- `125:11017` Action / Review current source → `[{"type":"NODE","target":"125:8508","navigation":"NAVIGATE"}]`

## ST-446 · Flow completion / GenBankDocument

**Role:** Linked functional state. **Frame:** 126:3910 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-3910).

**Displayed content:**

- Expression fragment
- DNA-GB-001 · version 1 · DNA · 12 nt · linear
- Search
- New
- Sequence and provenance
- expression_fragment.gb
- 1                        12
- ATGCGTACGTAA
- Property
- Value
- Stable identity
- DNA-GB-001 · version 1
- Annotations
- CDS · 1–12 · forward
- Original source
- Restriction analysis
- Save document
- Context & next steps
- The displayed identity is retained
through setup, result and saved
document.

**Authored actions:**

- `126:4017` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `126:4018` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `126:4093` Action / Restriction analysis → `[{"type":"NODE","target":"126:4098","navigation":"NAVIGATE"}]`
- `126:4095` Action / Save document → `[{"type":"NODE","target":"126:4156","navigation":"NAVIGATE"}]`

## ST-447 · Flow completion / GenBankDocument Setup

**Role:** Linked functional state. **Frame:** 126:4098 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-4098).

**Displayed content:**

- Restriction analysis · DNA-GB-001
- Close
- Confirm the complete input before running.
- Input
- DNA-GB-001 · version 1
- Scope
- Whole linear sequence · 12 nt
- Enzymes
- EcoRI and BamHI
- Back to sequence
- Run analysis

**Authored actions:**

- `126:4102` Action / Close → `[{"type":"BACK"}]`
- `126:4120` Action / Back to sequence → `[{"type":"NODE","target":"126:3910","navigation":"NAVIGATE"}]`
- `126:4122` Action / Run analysis → `[{"type":"NODE","target":"126:4124","navigation":"NAVIGATE"}]`

## ST-448 · Flow completion / GenBankDocument Result

**Role:** Linked functional state. **Frame:** 126:4124 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-4124).

**Displayed content:**

- No cut sites found
- Close
- Analysis completed for the selected input.
- Input
- DNA-GB-001 · version 1
- Enzymes
- EcoRI and BamHI
- Predicted output
- One uncut linear fragment · 12 nt
- Result state
- Completed · not a connection or validation error
- Change settings
- View source
- Save document

**Authored actions:**

- `126:4128` Action / Close → `[{"type":"BACK"}]`
- `126:4150` Action / Change settings → `[{"type":"NODE","target":"126:4098","navigation":"NAVIGATE"}]`
- `126:4152` Action / View source → `[{"type":"NODE","target":"126:3910","navigation":"NAVIGATE"}]`
- `126:4154` Action / Save document → `[{"type":"NODE","target":"126:4156","navigation":"NAVIGATE"}]`

## ST-449 · Flow completion / GenBankDocument Save

**Role:** Linked functional state. **Frame:** 126:4156 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-4156).

**Displayed content:**

- Save Expression fragment
- Close
- Choose a location without changing the source identity.
- Document
- DNA-GB-001 · version 1
- Location
- Local workspace / Sequences
- Annotations
- CDS · 1–12 · forward
- Cancel
- Save document

**Authored actions:**

- `126:4160` Action / Close → `[{"type":"BACK"}]`
- `126:4178` Action / Cancel → `[{"type":"NODE","target":"126:3910","navigation":"NAVIGATE"}]`
- `126:4180` Action / Save document → `[{"type":"NODE","target":"126:4182","navigation":"NAVIGATE"}]`

## ST-450 · Flow completion / GenBankDocument Saved

**Role:** Linked functional state. **Frame:** 126:4182 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-4182).

**Displayed content:**

- Expression fragment saved
- Close
- DNA-GB-001 · Local workspace / Sequences
- Input version
- 1
- Retained source
- expression_fragment.gb
- Open sequence
- Find saved document

**Authored actions:**

- `126:4186` Action / Close → `[{"type":"BACK"}]`
- `126:4200` Action / Open sequence → `[{"type":"NODE","target":"126:3910","navigation":"NAVIGATE"}]`
- `126:4202` Action / Find saved document → `[{"type":"NODE","target":"126:4204","navigation":"NAVIGATE"}]`

## ST-451 · Flow completion / GenBankDocument Home

**Role:** Linked functional state. **Frame:** 126:4204 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-4204).

**Displayed content:**

- Continue your work
- Your newly saved document is available by name and ID.
- Search
- New
- Document
- Identity
- Location
- Expression fragment
- DNA-GB-001 · v1
- Local workspace / Sequences
- Context & next steps

**Authored actions:**

- `126:4311` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `126:4312` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `126:4372` Record / Expression fragment → `[{"type":"NODE","target":"126:3910","navigation":"NAVIGATE"}]`

## ST-452 · Flow completion / FastaDocument

**Role:** Linked functional state. **Frame:** 126:4376 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-4376).

**Displayed content:**

- Expression fragment FASTA
- DNA-FA-001 · version 1 · DNA · 12 nt · linear
- Search
- New
- Sequence and provenance
- expression_fragment.fasta
- 1                        12
- ATGCGTACGTAA
- Property
- Value
- Stable identity
- DNA-FA-001 · version 1
- Annotations
- No annotations in source FASTA
- Original source
- Restriction analysis
- Save document
- Context & next steps
- The displayed identity is retained
through setup, result and saved
document.

**Authored actions:**

- `126:4483` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `126:4484` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `126:4559` Action / Restriction analysis → `[{"type":"NODE","target":"126:4564","navigation":"NAVIGATE"}]`
- `126:4561` Action / Save document → `[{"type":"NODE","target":"126:4622","navigation":"NAVIGATE"}]`

## ST-453 · Flow completion / FastaDocument Setup

**Role:** Linked functional state. **Frame:** 126:4564 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-4564).

**Displayed content:**

- Restriction analysis · DNA-FA-001
- Close
- Confirm the complete input before running.
- Input
- DNA-FA-001 · version 1
- Scope
- Whole linear sequence · 12 nt
- Enzymes
- EcoRI and BamHI
- Back to sequence
- Run analysis

**Authored actions:**

- `126:4568` Action / Close → `[{"type":"BACK"}]`
- `126:4586` Action / Back to sequence → `[{"type":"NODE","target":"126:4376","navigation":"NAVIGATE"}]`
- `126:4588` Action / Run analysis → `[{"type":"NODE","target":"126:4590","navigation":"NAVIGATE"}]`

## ST-454 · Flow completion / FastaDocument Result

**Role:** Linked functional state. **Frame:** 126:4590 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-4590).

**Displayed content:**

- No cut sites found
- Close
- Analysis completed for the selected input.
- Input
- DNA-FA-001 · version 1
- Enzymes
- EcoRI and BamHI
- Predicted output
- One uncut linear fragment · 12 nt
- Result state
- Completed · not a connection or validation error
- Change settings
- View source
- Save document

**Authored actions:**

- `126:4594` Action / Close → `[{"type":"BACK"}]`
- `126:4616` Action / Change settings → `[{"type":"NODE","target":"126:4564","navigation":"NAVIGATE"}]`
- `126:4618` Action / View source → `[{"type":"NODE","target":"126:4376","navigation":"NAVIGATE"}]`
- `126:4620` Action / Save document → `[{"type":"NODE","target":"126:4622","navigation":"NAVIGATE"}]`

## ST-455 · Flow completion / FastaDocument Save

**Role:** Linked functional state. **Frame:** 126:4622 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-4622).

**Displayed content:**

- Save Expression fragment FASTA
- Close
- Choose a location without changing the source identity.
- Document
- DNA-FA-001 · version 1
- Location
- Local workspace / Sequences
- Annotations
- No annotations in source FASTA
- Cancel
- Save document

**Authored actions:**

- `126:4626` Action / Close → `[{"type":"BACK"}]`
- `126:4644` Action / Cancel → `[{"type":"NODE","target":"126:4376","navigation":"NAVIGATE"}]`
- `126:4646` Action / Save document → `[{"type":"NODE","target":"126:4648","navigation":"NAVIGATE"}]`

## ST-456 · Flow completion / FastaDocument Saved

**Role:** Linked functional state. **Frame:** 126:4648 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-4648).

**Displayed content:**

- Expression fragment FASTA saved
- Close
- DNA-FA-001 · Local workspace / Sequences
- Input version
- 1
- Retained source
- expression_fragment.fasta
- Open sequence
- Find saved document

**Authored actions:**

- `126:4652` Action / Close → `[{"type":"BACK"}]`
- `126:4666` Action / Open sequence → `[{"type":"NODE","target":"126:4376","navigation":"NAVIGATE"}]`
- `126:4668` Action / Find saved document → `[{"type":"NODE","target":"126:4670","navigation":"NAVIGATE"}]`

## ST-457 · Flow completion / FastaDocument Home

**Role:** Linked functional state. **Frame:** 126:4670 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-4670).

**Displayed content:**

- Continue your work
- Your newly saved document is available by name and ID.
- Search
- New
- Document
- Identity
- Location
- Expression fragment FASTA
- DNA-FA-001 · v1
- Local workspace / Sequences
- Context & next steps

**Authored actions:**

- `126:4777` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `126:4778` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `126:4838` Record / Expression fragment FASTA → `[{"type":"NODE","target":"126:4376","navigation":"NAVIGATE"}]`

## ST-458 · Flow completion / SecondFastaDocument

**Role:** Linked functional state. **Frame:** 126:4842 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-4842).

**Displayed content:**

- Control fragment
- DNA-FA-002 · version 1 · DNA · 12 nt · linear
- Search
- New
- Sequence and provenance
- control_fragment.fasta
- 1                        12
- ATGCGTACGTAA
- Property
- Value
- Stable identity
- DNA-FA-002 · version 1
- Annotations
- No annotations in source FASTA
- Original source
- Restriction analysis
- Save document
- Context & next steps
- The displayed identity is retained
through setup, result and saved
document.

**Authored actions:**

- `126:4949` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `126:4950` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `126:5025` Action / Restriction analysis → `[{"type":"NODE","target":"126:5030","navigation":"NAVIGATE"}]`
- `126:5027` Action / Save document → `[{"type":"NODE","target":"126:5088","navigation":"NAVIGATE"}]`

## ST-459 · Flow completion / SecondFastaDocument Setup

**Role:** Linked functional state. **Frame:** 126:5030 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-5030).

**Displayed content:**

- Restriction analysis · DNA-FA-002
- Close
- Confirm the complete input before running.
- Input
- DNA-FA-002 · version 1
- Scope
- Whole linear sequence · 12 nt
- Enzymes
- EcoRI and BamHI
- Back to sequence
- Run analysis

**Authored actions:**

- `126:5034` Action / Close → `[{"type":"BACK"}]`
- `126:5052` Action / Back to sequence → `[{"type":"NODE","target":"126:4842","navigation":"NAVIGATE"}]`
- `126:5054` Action / Run analysis → `[{"type":"NODE","target":"126:5056","navigation":"NAVIGATE"}]`

## ST-460 · Flow completion / SecondFastaDocument Result

**Role:** Linked functional state. **Frame:** 126:5056 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-5056).

**Displayed content:**

- No cut sites found
- Close
- Analysis completed for the selected input.
- Input
- DNA-FA-002 · version 1
- Enzymes
- EcoRI and BamHI
- Predicted output
- One uncut linear fragment · 12 nt
- Result state
- Completed · not a connection or validation error
- Change settings
- View source
- Save document

**Authored actions:**

- `126:5060` Action / Close → `[{"type":"BACK"}]`
- `126:5082` Action / Change settings → `[{"type":"NODE","target":"126:5030","navigation":"NAVIGATE"}]`
- `126:5084` Action / View source → `[{"type":"NODE","target":"126:4842","navigation":"NAVIGATE"}]`
- `126:5086` Action / Save document → `[{"type":"NODE","target":"126:5088","navigation":"NAVIGATE"}]`

## ST-461 · Flow completion / SecondFastaDocument Save

**Role:** Linked functional state. **Frame:** 126:5088 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-5088).

**Displayed content:**

- Save Control fragment
- Close
- Choose a location without changing the source identity.
- Document
- DNA-FA-002 · version 1
- Location
- Local workspace / Sequences
- Annotations
- No annotations in source FASTA
- Cancel
- Save document

**Authored actions:**

- `126:5092` Action / Close → `[{"type":"BACK"}]`
- `126:5110` Action / Cancel → `[{"type":"NODE","target":"126:4842","navigation":"NAVIGATE"}]`
- `126:5112` Action / Save document → `[{"type":"NODE","target":"126:5114","navigation":"NAVIGATE"}]`

## ST-462 · Flow completion / SecondFastaDocument Saved

**Role:** Linked functional state. **Frame:** 126:5114 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-5114).

**Displayed content:**

- Control fragment saved
- Close
- DNA-FA-002 · Local workspace / Sequences
- Input version
- 1
- Retained source
- control_fragment.fasta
- Open sequence
- Find saved document

**Authored actions:**

- `126:5118` Action / Close → `[{"type":"BACK"}]`
- `126:5132` Action / Open sequence → `[{"type":"NODE","target":"126:4842","navigation":"NAVIGATE"}]`
- `126:5134` Action / Find saved document → `[{"type":"NODE","target":"126:5136","navigation":"NAVIGATE"}]`

## ST-463 · Flow completion / SecondFastaDocument Home

**Role:** Linked functional state. **Frame:** 126:5136 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-5136).

**Displayed content:**

- Continue your work
- Your newly saved document is available by name and ID.
- Search
- New
- Document
- Identity
- Location
- Control fragment
- DNA-FA-002 · v1
- Local workspace / Sequences
- Context & next steps

**Authored actions:**

- `126:5243` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `126:5244` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `126:5304` Record / Control fragment → `[{"type":"NODE","target":"126:4842","navigation":"NAVIGATE"}]`

## ST-464 · Flow completion / GenBankReview

**Role:** Linked functional state. **Frame:** 126:5308 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-5308).

**Displayed content:**

- Review GenBank import
- Close
- A sequence and its annotation were recognized.
- File
- expression_fragment.gb
- New identity
- DNA-GB-001 · version 1
- Sequence
- DNA · 12 nt · linear
- Annotations
- CDS 1–12 forward · retained
- Changes
- None · original bases retained
- Back to source
- Open sequence

**Authored actions:**

- `126:5312` Action / Close → `[{"type":"BACK"}]`
- `126:5338` Action / Back to source → `[{"type":"NODE","target":"110:19645","navigation":"NAVIGATE"}]`
- `126:5340` Action / Open sequence → `[{"type":"NODE","target":"126:3910","navigation":"NAVIGATE"}]`

## ST-465 · Flow completion / FastaReview

**Role:** Linked functional state. **Frame:** 126:5342 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-5342).

**Displayed content:**

- Review FASTA import
- Close
- FASTA does not supply all document metadata. Confirm the interpretation.
- File
- expression_fragment.fasta
- Header
- Expression fragment FASTA
- New identity
- DNA-FA-001 · version 1
- Interpretation
- DNA · linear · 12 nt
- Annotation warning
- No annotations supplied; none will be invented
- Back to source
- Confirm DNA and open

**Authored actions:**

- `126:5346` Action / Close → `[{"type":"BACK"}]`
- `126:5372` Action / Back to source → `[{"type":"NODE","target":"110:19645","navigation":"NAVIGATE"}]`
- `126:5374` Action / Confirm DNA and open → `[{"type":"NODE","target":"126:4376","navigation":"NAVIGATE"}]`

## ST-466 · Flow completion / BulkReview

**Role:** Linked functional state. **Frame:** 126:5392 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-5392).

**Displayed content:**

- Review two FASTA records
- Close
- Choose which records to open.
- Record 1
- Expression fragment · DNA-FA-001 · valid · 12 nt
- Record 2
- Control fragment · DNA-FA-002 · valid · 12 nt
- Selection
- Both records selected
- Next step
- Each record receives its own document identity
- Import first only
- Open both documents
- Review partial import

**Authored actions:**

- `126:5396` Action / Close → `[{"type":"BACK"}]`
- `126:5418` Action / Import first only → `[{"type":"NODE","target":"126:4376","navigation":"NAVIGATE"}]`
- `126:5420` Action / Open both documents → `[{"type":"NODE","target":"126:5424","navigation":"NAVIGATE"}]`
- `126:5422` Action / Review partial import → `[{"type":"NODE","target":"126:5600","navigation":"NAVIGATE"}]`

## ST-467 · Flow completion / ImportedDocuments

**Role:** Linked functional state. **Frame:** 126:5424 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-5424).

**Displayed content:**

- Two documents imported
- Each imported sequence has its own stable ID.
- Search
- New
- Document
- Identity
- Input
- Expression fragment FASTA
- DNA-FA-001 · v1
- 12 nt · linear
- Control fragment
- DNA-FA-002 · v1
- Context & next steps

**Authored actions:**

- `126:5531` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `126:5532` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `126:5592` Record / Expression fragment FASTA → `[{"type":"NODE","target":"126:4376","navigation":"NAVIGATE"}]`
- `126:5596` Record / Control fragment → `[{"type":"NODE","target":"126:4842","navigation":"NAVIGATE"}]`

## ST-468 · Flow completion / PartialImport

**Role:** Linked functional state. **Frame:** 126:5600 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-5600).

**Displayed content:**

- One record needs correction
- Close
- Valid input and rejected input are kept separate.
- Skipping a record is an explicit decision.
- Record 1
- Expression fragment · valid · 12 nt
- Record 2
- Control fragment · unsupported “?” at position 4
- Preservation
- Original rejected record retained; no symbols silently removed
- Choice
- Correct the source or deliberately import only the valid record
- Correct rejected record
- Import valid record only

**Authored actions:**

- `126:5604` Action / Close → `[{"type":"BACK"}]`
- `126:5630` Action / Correct rejected record → `[{"type":"NODE","target":"126:5634","navigation":"NAVIGATE"}]`
- `126:5632` Action / Import valid record only → `[{"type":"NODE","target":"126:4376","navigation":"NAVIGATE"}]`

## ST-469 · Flow completion / ImportCorrection

**Role:** Linked functional state. **Frame:** 126:5634 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-5634).

**Displayed content:**

- Correct the rejected sequence
- Close
- The original value remains available for comparison.
- Original
- ATG?GTACGTAA
- Issue
- Position 4: unsupported “?”
- Corrected value
- ATGCGTACGTAA
- Review
- One explicit replacement · length remains 12 nt
- Keep original
- Review corrected records

**Authored actions:**

- `126:5638` Action / Close → `[{"type":"BACK"}]`
- `126:5660` Action / Keep original → `[{"type":"NODE","target":"126:5600","navigation":"NAVIGATE"}]`
- `126:5662` Action / Review corrected records → `[{"type":"NODE","target":"126:5392","navigation":"NAVIGATE"}]`

## ST-470 · Flow completion / DuplicateImport

**Role:** Linked functional state. **Frame:** 126:5664 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-5664).

**Displayed content:**

- A matching document already exists
- Close
- Choose the intended outcome before importing.
- Existing
- DNA-FA-001 · Expression fragment FASTA
- Incoming
- expression_fragment.fasta · same example input
- Safe choices
- Open existing document, skip, or create an independent copy
- Open existing
- Skip import
- Create a copy

**Authored actions:**

- `126:5668` Action / Close → `[{"type":"BACK"}]`
- `126:5686` Action / Open existing → `[{"type":"NODE","target":"126:4376","navigation":"NAVIGATE"}]`
- `126:5688` Action / Skip import → `[{"type":"NODE","target":"110:19645","navigation":"NAVIGATE"}]`
- `126:5690` Action / Create a copy → `[{"type":"NODE","target":"126:5692","navigation":"NAVIGATE"}]`

## ST-471 · Flow completion / DuplicateCopy

**Role:** Linked functional state. **Frame:** 126:5692 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-5692).

**Displayed content:**

- Copy of Expression fragment
- DNA-COPY-001 · version 1 · DNA · 12 nt · linear
- Search
- New
- Sequence and provenance
- Independent copy of expression_fragment.fasta
- 1                        12
- ATGCGTACGTAA
- Property
- Value
- Stable identity
- DNA-COPY-001 · version 1
- Annotations
- None
- Original source
- Restriction analysis
- Save document
- Context & next steps
- The displayed identity is retained
through setup, result and saved
document.

**Authored actions:**

- `126:5799` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `126:5800` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `126:5875` Action / Restriction analysis → `[{"type":"NODE","target":"126:5880","navigation":"NAVIGATE"}]`
- `126:5877` Action / Save document → `[{"type":"NODE","target":"126:5938","navigation":"NAVIGATE"}]`

## ST-472 · Flow completion / DuplicateCopy Setup

**Role:** Linked functional state. **Frame:** 126:5880 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-5880).

**Displayed content:**

- Restriction analysis · DNA-COPY-001
- Close
- Confirm the complete input before running.
- Input
- DNA-COPY-001 · version 1
- Scope
- Whole linear sequence · 12 nt
- Enzymes
- EcoRI and BamHI
- Back to sequence
- Run analysis

**Authored actions:**

- `126:5884` Action / Close → `[{"type":"BACK"}]`
- `126:5902` Action / Back to sequence → `[{"type":"NODE","target":"126:5692","navigation":"NAVIGATE"}]`
- `126:5904` Action / Run analysis → `[{"type":"NODE","target":"126:5906","navigation":"NAVIGATE"}]`

## ST-473 · Flow completion / DuplicateCopy Result

**Role:** Linked functional state. **Frame:** 126:5906 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-5906).

**Displayed content:**

- No cut sites found
- Close
- Analysis completed for the selected input.
- Input
- DNA-COPY-001 · version 1
- Enzymes
- EcoRI and BamHI
- Predicted output
- One uncut linear fragment · 12 nt
- Result state
- Completed · not a connection or validation error
- Change settings
- View source
- Save document

**Authored actions:**

- `126:5910` Action / Close → `[{"type":"BACK"}]`
- `126:5932` Action / Change settings → `[{"type":"NODE","target":"126:5880","navigation":"NAVIGATE"}]`
- `126:5934` Action / View source → `[{"type":"NODE","target":"126:5692","navigation":"NAVIGATE"}]`
- `126:5936` Action / Save document → `[{"type":"NODE","target":"126:5938","navigation":"NAVIGATE"}]`

## ST-474 · Flow completion / DuplicateCopy Save

**Role:** Linked functional state. **Frame:** 126:5938 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-5938).

**Displayed content:**

- Save Copy of Expression fragment
- Close
- Choose a location without changing the source identity.
- Document
- DNA-COPY-001 · version 1
- Location
- Local workspace / Sequences
- Annotations
- None
- Cancel
- Save document

**Authored actions:**

- `126:5942` Action / Close → `[{"type":"BACK"}]`
- `126:5960` Action / Cancel → `[{"type":"NODE","target":"126:5692","navigation":"NAVIGATE"}]`
- `126:5962` Action / Save document → `[{"type":"NODE","target":"126:5964","navigation":"NAVIGATE"}]`

## ST-475 · Flow completion / DuplicateCopy Saved

**Role:** Linked functional state. **Frame:** 126:5964 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-5964).

**Displayed content:**

- Copy of Expression fragment saved
- Close
- DNA-COPY-001 · Local workspace / Sequences
- Input version
- 1
- Retained source
- Independent copy of expression_fragment.fasta
- Open sequence
- Find saved document

**Authored actions:**

- `126:5968` Action / Close → `[{"type":"BACK"}]`
- `126:5982` Action / Open sequence → `[{"type":"NODE","target":"126:5692","navigation":"NAVIGATE"}]`
- `126:5984` Action / Find saved document → `[{"type":"NODE","target":"126:5986","navigation":"NAVIGATE"}]`

## ST-476 · Flow completion / DuplicateCopy Home

**Role:** Linked functional state. **Frame:** 126:5986 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-5986).

**Displayed content:**

- Continue your work
- Your newly saved document is available by name and ID.
- Search
- New
- Document
- Identity
- Location
- Copy of Expression fragment
- DNA-COPY-001 · v1
- Local workspace / Sequences
- Context & next steps

**Authored actions:**

- `126:6093` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `126:6094` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `126:6154` Record / Copy of Expression fragment → `[{"type":"NODE","target":"126:5692","navigation":"NAVIGATE"}]`

## ST-477 · Flow completion / SampleSetup

**Role:** Linked functional state. **Frame:** 126:6169 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-6169).

**Displayed content:**

- Sample restriction analysis
- Confirm your input and review recommended settings.
- Exit sample
- Input: SAMPLE-DNA-001 · pHelixBench-Demo
- Version 7 · 3021 bp · circular · full sequence, not only the selected promoter.
- Recommended setup
- Enzyme selection
- EcoRI + BamHI
- Analysis scope
- Whole circular sequence · 3021 bp
- Both enzymes have one predicted cut site in this example input. Settings are suggestions; review assumptions before
interpretation.
- Advanced settings
- Why these settings?
- The result will retain input version, enzymes, scope and parameters.
- Back to sequence
- Run restriction analysis
- Input & prerequisites
- Sample restriction analysis needs a valid
DNA sequence and selected enzymes.
Your entire input is in scope; no
selected region is required.
- Review source sequence
- Choose another enzyme set
- Sample data · not a personal record

**Authored actions:**

- `126:6276` Action / Search → `[{"type":"NODE","target":"126:6528","navigation":"NAVIGATE"}]`
- `126:6295` Action / Advanced settings → `[{"type":"NODE","target":"126:7959","navigation":"NAVIGATE"}]`
- `126:6296` Action / Why these settings? → `[{"type":"NODE","target":"126:7959","navigation":"NAVIGATE"}]`
- `126:6302` Action / Back to sequence → `[{"type":"NODE","target":"110:19911","navigation":"NAVIGATE"}]`
- `126:6303` Action / Run restriction analysis → `[{"type":"NODE","target":"126:6457","navigation":"NAVIGATE"}]`
- `126:6308` Action / Review source sequence → `[{"type":"NODE","target":"110:19911","navigation":"NAVIGATE"}]`
- `126:6309` Action / Choose another enzyme set → `[{"type":"NODE","target":"107:26978","navigation":"NAVIGATE"}]`

## ST-478 · Flow completion / SampleResult

**Role:** Linked functional state. **Frame:** 126:6310 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-6310).

**Displayed content:**

- Restriction analysis result
- SAMPLE-RES-001 · pHelixBench-Demo · input SAMPLE-DNA-001 v7
- Exit sample
- Two predicted fragments
- EcoRI at 216 · BamHI at 234 · circular input length 3021 bp.
- Fragment
- Length
- Interpretation
- Fragment 1
- 18 bp
- Between the two selected cut positions
- Fragment 2
- 3003 bp
- Remaining circular backbone
- Total
- 3021 bp
- Matches input length
- Illustrative result fixture. Verify predictions and experimental assumptions before laboratory use.
- Input and settings stay attached
- SAMPLE-DNA-001 · v7 · whole sequence · EcoRI + BamHI · SAMPLE-RES-001
- Edit settings
- View source
- Save result
- Read the result
- Fragment lengths describe this
input and enzyme set. The small
fragment may need different
experimental handling.
- Parameter summary
- How to interpret this
- Sample data · not a personal record

**Authored actions:**

- `126:6417` Action / Search → `[{"type":"NODE","target":"126:6528","navigation":"NAVIGATE"}]`
- `126:6448` Action / Edit settings → `[{"type":"NODE","target":"126:6169","navigation":"NAVIGATE"}]`
- `126:6449` Action / View source → `[{"type":"NODE","target":"110:19911","navigation":"NAVIGATE"}]`
- `126:6450` Action / Save result → `[{"type":"NODE","target":"126:6483","navigation":"NAVIGATE"}]`
- `126:6455` Action / Parameter summary → `[{"type":"NODE","target":"126:7959","navigation":"NAVIGATE"}]`
- `126:6456` Action / How to interpret this → `[{"type":"NODE","target":"126:7959","navigation":"NAVIGATE"}]`

## ST-479 · Flow completion / SampleRunning

**Role:** Linked functional state. **Frame:** 126:6457 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-6457).

**Displayed content:**

- Analyzing the sample
- Close
- Sample operation · practice data
- Input
- SAMPLE-DNA-001 · v7
- Scope
- Whole circular sequence · 3021 bp
- Enzymes
- EcoRI and BamHI
- Cancel
- View sample result

**Authored actions:**

- `126:6461` Action / Close → `[{"type":"BACK"}]`
- `126:6479` Action / Cancel → `[{"type":"NODE","target":"110:19911","navigation":"NAVIGATE"}]`
- `126:6481` Action / View sample result → `[{"type":"NODE","target":"126:6310","navigation":"NAVIGATE"}]`

## ST-480 · Flow completion / SampleSaved

**Role:** Linked functional state. **Frame:** 126:6483 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-6483).

**Displayed content:**

- Sample result saved
- Close
- This result remains part of the sample workspace.
- Result
- SAMPLE-RES-001
- Input
- SAMPLE-DNA-001 · version 7
- Personal work
- No personal record was created
- Return to sample
- Save a personal copy

**Authored actions:**

- `126:6487` Action / Close → `[{"type":"BACK"}]`
- `126:6505` Action / Return to sample → `[{"type":"NODE","target":"110:19911","navigation":"NAVIGATE"}]`
- `126:6507` Action / Save a personal copy → `[{"type":"NODE","target":"126:6556","navigation":"NAVIGATE"}]`

## ST-481 · Flow completion / ExitSample

**Role:** Linked functional state. **Frame:** 126:6528 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-6528).

**Displayed content:**

- Leave the sample workspace?
- Close
- Practice data stays separate from your personal records.
- Current input
- SAMPLE-DNA-001 · sample data
- Leave
- Return to your workspace without creating a personal record
- Keep a copy
- Create an independent document with a new ID
- Stay in sample
- Leave sample
- Save personal copy

**Authored actions:**

- `126:6532` Action / Close → `[{"type":"BACK"}]`
- `126:6550` Action / Stay in sample → `[{"type":"NODE","target":"110:19911","navigation":"NAVIGATE"}]`
- `126:6552` Action / Leave sample → `[{"type":"NODE","target":"107:29237","navigation":"NAVIGATE"}]`
- `126:6554` Action / Save personal copy → `[{"type":"NODE","target":"126:6556","navigation":"NAVIGATE"}]`

## ST-482 · Flow completion / SampleCopyReview

**Role:** Linked functional state. **Frame:** 126:6556 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-6556).

**Displayed content:**

- Save a personal copy
- Close
- A copy gets its own identity; sample records remain unchanged.
- Source
- SAMPLE-DNA-001 · version 7
- New name
- Copy of pHelixBench-Demo
- New identity
- DNA-PERSONAL-001 · version 1
- Results
- Sample results are not relabeled as personal analyses
- Cancel
- Create personal copy

**Authored actions:**

- `126:6560` Action / Close → `[{"type":"BACK"}]`
- `126:6582` Action / Cancel → `[{"type":"NODE","target":"110:19911","navigation":"NAVIGATE"}]`
- `126:6584` Action / Create personal copy → `[{"type":"NODE","target":"126:6586","navigation":"NAVIGATE"}]`

## ST-483 · Flow completion / PersonalSampleCopy

**Role:** Linked functional state. **Frame:** 126:6586 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-6586).

**Displayed content:**

- Copy of pHelixBench-Demo
- DNA-PERSONAL-001 · version 1 · 3021 bp · circular
- Search
- New
- Property
- Value
- Origin
- Independent copy of SAMPLE-DNA-001
- Scientific identity
- 3021 bp · circular · retained annotations
- Analysis results
- No personal analyses yet
- Run a fresh analysis
- Context & next steps
- Personal document · independent
identity. A new analysis will
retain this input ID.

**Authored actions:**

- `126:6693` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `126:6694` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `126:6763` Action / Run a fresh analysis → `[{"type":"NODE","target":"126:6766","navigation":"NAVIGATE"}]`

## ST-484 · Flow completion / PersonalCopySetup

**Role:** Linked functional state. **Frame:** 126:6766 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-6766).

**Displayed content:**

- Restriction analysis
- Confirm your input and review recommended settings.
- Search
- New
- Input: DNA-PERSONAL-001 · pHelixBench-Demo
- Version 1 · 3021 bp · circular · full sequence, not only the selected promoter.
- Recommended setup
- Enzyme selection
- EcoRI + BamHI
- Analysis scope
- Whole circular sequence · 3021 bp
- Both enzymes have one predicted cut site in this example input. Settings are suggestions; review assumptions before
interpretation.
- Advanced settings
- Why these settings?
- The result will retain input version, enzymes, scope and parameters.
- Back to sequence
- Run restriction analysis
- Input & prerequisites
- Restriction analysis needs a valid
DNA sequence and selected enzymes.
Your entire input is in scope; no
selected region is required.
- Review source sequence
- Choose another enzyme set

**Authored actions:**

- `126:6873` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `126:6874` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `126:6892` Action / Advanced settings → `[{"type":"NODE","target":"126:7989","navigation":"NAVIGATE"}]`
- `126:6893` Action / Why these settings? → `[{"type":"NODE","target":"126:7989","navigation":"NAVIGATE"}]`
- `126:6899` Action / Back to sequence → `[{"type":"NODE","target":"126:6586","navigation":"NAVIGATE"}]`
- `126:6900` Action / Run restriction analysis → `[{"type":"NODE","target":"126:6907","navigation":"NAVIGATE"}]`
- `126:6905` Action / Review source sequence → `[{"type":"NODE","target":"126:6586","navigation":"NAVIGATE"}]`
- `126:6906` Action / Choose another enzyme set → `[{"type":"NODE","target":"107:26978","navigation":"NAVIGATE"}]`

## ST-485 · Flow completion / PersonalCopyResult

**Role:** Linked functional state. **Frame:** 126:6907 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-6907).

**Displayed content:**

- Restriction analysis result
- RES-PERSONAL-001 · pHelixBench-Demo · input DNA-PERSONAL-001 v1
- Search
- New
- Two predicted fragments
- EcoRI at 216 · BamHI at 234 · circular input length 3021 bp.
- Fragment
- Length
- Interpretation
- Fragment 1
- 18 bp
- Between the two selected cut positions
- Fragment 2
- 3003 bp
- Remaining circular backbone
- Total
- 3021 bp
- Matches input length
- Illustrative result fixture. Verify predictions and experimental assumptions before laboratory use.
- Input and settings stay attached
- DNA-PERSONAL-001 · v1 · whole sequence · EcoRI + BamHI · RES-PERSONAL-001
- Edit settings
- View source
- Save result
- Read the result
- Fragment lengths describe this
input and enzyme set. The small
fragment may need different
experimental handling.
- Parameter summary
- How to interpret this

**Authored actions:**

- `126:7014` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `126:7015` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `126:7045` Action / Edit settings → `[{"type":"NODE","target":"126:6766","navigation":"NAVIGATE"}]`
- `126:7046` Action / View source → `[{"type":"NODE","target":"126:6586","navigation":"NAVIGATE"}]`
- `126:7047` Action / Save result → `[{"type":"NODE","target":"126:7073","navigation":"NAVIGATE"}]`
- `126:7052` Action / Parameter summary → `[{"type":"NODE","target":"126:7989","navigation":"NAVIGATE"}]`
- `126:7053` Action / How to interpret this → `[{"type":"NODE","target":"126:7989","navigation":"NAVIGATE"}]`

## ST-486 · Flow completion / PersonalCopySaved

**Role:** Linked functional state. **Frame:** 126:7073 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-7073).

**Displayed content:**

- Personal result saved
- Close
- Your sample copy and its new result retain independent IDs.
- Result
- RES-PERSONAL-001
- Input
- DNA-PERSONAL-001 · version 1
- Open result
- Open input

**Authored actions:**

- `126:7077` Action / Close → `[{"type":"BACK"}]`
- `126:7091` Action / Open result → `[{"type":"NODE","target":"126:6907","navigation":"NAVIGATE"}]`
- `126:7093` Action / Open input → `[{"type":"NODE","target":"126:6586","navigation":"NAVIGATE"}]`

## ST-487 · Flow completion / RerunVersion8

**Role:** Linked functional state. **Frame:** 126:7098 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-7098).

**Displayed content:**

- Restriction analysis
- Confirm your input and review recommended settings.
- Search
- New
- Input: DNA-001 · pHelixBench-Demo
- Version 8 · 3021 bp · circular · full sequence, not only the selected promoter.
- Recommended setup
- Enzyme selection
- EcoRI + BamHI
- Analysis scope
- Whole circular sequence · 3021 bp
- Both enzymes have one predicted cut site in this example input. Settings are suggestions; review assumptions before
interpretation.
- Advanced settings
- Why these settings?
- The result will retain input version, enzymes, scope and parameters.
- Back to sequence
- Run restriction analysis
- Input & prerequisites
- Restriction analysis needs a valid
DNA sequence and selected enzymes.
Your entire input is in scope; no
selected region is required.
- Review source sequence
- Choose another enzyme set

**Authored actions:**

- `126:7205` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `126:7206` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `126:7224` Action / Advanced settings → `[{"type":"NODE","target":"126:8019","navigation":"NAVIGATE"}]`
- `126:7225` Action / Why these settings? → `[{"type":"NODE","target":"126:8019","navigation":"NAVIGATE"}]`
- `126:7231` Action / Back to sequence → `[{"type":"NODE","target":"125:8508","navigation":"NAVIGATE"}]`
- `126:7232` Action / Run restriction analysis → `[{"type":"NODE","target":"126:7386","navigation":"NAVIGATE"}]`
- `126:7237` Action / Review source sequence → `[{"type":"NODE","target":"125:8508","navigation":"NAVIGATE"}]`
- `126:7238` Action / Choose another enzyme set → `[{"type":"NODE","target":"107:26978","navigation":"NAVIGATE"}]`

## ST-488 · Flow completion / ResultVersion8

**Role:** Linked functional state. **Frame:** 126:7239 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-7239).

**Displayed content:**

- Restriction analysis result
- RES-002 · pHelixBench-Demo · input DNA-001 v8
- Search
- New
- Two predicted fragments
- EcoRI at 216 · BamHI at 234 · circular input length 3021 bp.
- Fragment
- Length
- Interpretation
- Fragment 1
- 18 bp
- Between the two selected cut positions
- Fragment 2
- 3003 bp
- Remaining circular backbone
- Total
- 3021 bp
- Matches input length
- Illustrative result fixture. Verify predictions and experimental assumptions before laboratory use.
- Input and settings stay attached
- DNA-001 · v8 · whole sequence · EcoRI + BamHI · RES-002
- Edit settings
- View source
- Save result
- Read the result
- Fragment lengths describe this
input and enzyme set. The small
fragment may need different
experimental handling.
- Parameter summary
- How to interpret this

**Authored actions:**

- `126:7346` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `126:7347` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `126:7377` Action / Edit settings → `[{"type":"NODE","target":"126:7098","navigation":"NAVIGATE"}]`
- `126:7378` Action / View source → `[{"type":"NODE","target":"125:8508","navigation":"NAVIGATE"}]`
- `126:7379` Action / Save result → `[{"type":"NODE","target":"126:7427","navigation":"NAVIGATE"}]`
- `126:7384` Action / Parameter summary → `[{"type":"NODE","target":"126:8019","navigation":"NAVIGATE"}]`
- `126:7385` Action / How to interpret this → `[{"type":"NODE","target":"126:8019","navigation":"NAVIGATE"}]`

## ST-489 · Flow completion / RunVersion8

**Role:** Linked functional state. **Frame:** 126:7386 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-7386).

**Displayed content:**

- Analyzing version 8
- Close
- A new operation will create a new result, preserving RES-001.
- Input
- DNA-001 · version 8
- Output
- RES-002 · new result ID
- Cancel
- View completed result

**Authored actions:**

- `126:7390` Action / Close → `[{"type":"BACK"}]`
- `126:7404` Action / Cancel → `[{"type":"NODE","target":"126:7098","navigation":"NAVIGATE"}]`
- `126:7406` Action / View completed result → `[{"type":"NODE","target":"126:7239","navigation":"NAVIGATE"}]`

## ST-490 · Flow completion / SaveVersion8Result

**Role:** Linked functional state. **Frame:** 126:7427 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-7427).

**Displayed content:**

- Save result RES-002
- Close
- The earlier version-7 result is preserved.
- Result
- RES-002
- Source
- DNA-001 · version 8
- Location
- Local workspace / Analyses
- Cancel
- Save result

**Authored actions:**

- `126:7431` Action / Close → `[{"type":"BACK"}]`
- `126:7449` Action / Cancel → `[{"type":"NODE","target":"126:7239","navigation":"NAVIGATE"}]`
- `126:7451` Action / Save result → `[{"type":"NODE","target":"126:7453","navigation":"NAVIGATE"}]`

## ST-491 · Flow completion / SavedVersion8Result

**Role:** Linked functional state. **Frame:** 126:7453 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-7453).

**Displayed content:**

- RES-002 saved
- Close
- Input DNA-001 version 8 · settings retained
- Earlier result
- RES-001 remains linked to version 7
- Current result
- RES-002 is linked to version 8
- Open result
- Compare results
- Export result

**Authored actions:**

- `126:7457` Action / Close → `[{"type":"BACK"}]`
- `126:7471` Action / Open result → `[{"type":"NODE","target":"126:7239","navigation":"NAVIGATE"}]`
- `126:7473` Action / Compare results → `[{"type":"NODE","target":"126:7480","navigation":"NAVIGATE"}]`
- `126:7475` Action / Export result → `[{"type":"NODE","target":"126:7675","navigation":"NAVIGATE"}]`

## ST-492 · Flow completion / CompareResults

**Role:** Linked functional state. **Frame:** 126:7480 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-7480).

**Displayed content:**

- Compare saved analysis results
- Each result retains its own source version and parameters.
- Search
- New
- Property
- RES-001
- RES-002
- Input version
- DNA-001 · v7
- DNA-001 · v8
- Enzymes
- EcoRI + BamHI
- Scope
- Whole circular sequence
- Fragment lengths
- 18 bp + 3003 bp
- Interpretation
- Earlier input
- Newer input · same illustrated lengths
- Open older result
- Open newer result
- Export comparison
- Context & next steps

**Authored actions:**

- `126:7587` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `126:7588` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `126:7669` Action / Open older result → `[{"type":"NODE","target":"110:20596","navigation":"NAVIGATE"}]`
- `126:7671` Action / Open newer result → `[{"type":"NODE","target":"126:7239","navigation":"NAVIGATE"}]`
- `126:7673` Action / Export comparison → `[{"type":"NODE","target":"126:7675","navigation":"NAVIGATE"}]`

## ST-493 · Flow completion / ExportResult

**Role:** Linked functional state. **Frame:** 126:7675 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-7675).

**Displayed content:**

- Export analysis results
- Close
- Include provenance so the output can be interpreted later.
- Result
- RES-002 · DNA-001 v8
- Format
- CSV · fragment lengths and source metadata
- Included
- Input ID/version, enzymes, scope, fragment lengths and units
- Filename
- RES-002_restriction_analysis.csv
- Cancel
- Export CSV

**Authored actions:**

- `126:7679` Action / Close → `[{"type":"BACK"}]`
- `126:7701` Action / Cancel → `[{"type":"NODE","target":"126:7239","navigation":"NAVIGATE"}]`
- `126:7703` Action / Export CSV → `[{"type":"NODE","target":"126:7705","navigation":"NAVIGATE"}]`

## ST-494 · Flow completion / ExportComplete

**Role:** Linked functional state. **Frame:** 126:7705 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-7705).

**Displayed content:**

- Export prepared
- Close
- RES-002_restriction_analysis.csv
- Contents
- Fragment data and provenance
- Source
- DNA-001 · version 8
- Destination
- Your selected local folder
- Return to result

**Authored actions:**

- `126:7709` Action / Close → `[{"type":"BACK"}]`
- `126:7727` Action / Return to result → `[{"type":"NODE","target":"126:7239","navigation":"NAVIGATE"}]`

## ST-495 · Flow completion / ProteinInput

**Role:** Linked functional state. **Frame:** 126:7729 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-7729).

**Displayed content:**

- Choose a protein input
- Close
- The current DNA sequence cannot be analyzed as a protein without an explicit translation step.
- Current context
- DNA-001 · DNA · version 7
- Available protein
- HC-014 · heavy chain · 450 aa
- Alternative
- Translate a validated coding region, then review the translated product
- Keep DNA context
- Use HC-014 explicitly
- Review translation prerequisites

**Authored actions:**

- `126:7733` Action / Close → `[{"type":"BACK"}]`
- `126:7751` Action / Keep DNA context → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`
- `126:7753` Action / Use HC-014 explicitly → `[{"type":"NODE","target":"107:26168","navigation":"NAVIGATE"}]`
- `126:7755` Action / Review translation prerequisites → `[{"type":"NODE","target":"126:7757","navigation":"NAVIGATE"}]`

## ST-496 · Flow completion / TranslationPrerequisites

**Role:** Linked functional state. **Frame:** 126:7757 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-7757).

**Displayed content:**

- Choose a valid coding region
- Close
- Translation requires explicit range, strand and reading frame.
- DNA input
- DNA-001 · version 7
- Current selection
- 97–116 · 20 nt · promoter
- Prerequisite
- Select a coding region and confirm frame; partial codons require review
- Current state
- No protein product created
- Return to features
- Choose an existing protein

**Authored actions:**

- `126:7761` Action / Close → `[{"type":"BACK"}]`
- `126:7783` Action / Return to features → `[{"type":"NODE","target":"107:29886","navigation":"NAVIGATE"}]`
- `126:7785` Action / Choose an existing protein → `[{"type":"NODE","target":"126:7729","navigation":"NAVIGATE"}]`

## ST-497 · Flow completion / BlankDocumentHome

**Role:** Linked functional state. **Frame:** 126:7787 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-7787).

**Displayed content:**

- Continue your work
- Your newly saved sequence is available by its own identity.
- Search
- New
- Document
- Identity
- Location
- Untitled sequence
- DNA-NEW-001 · version 1
- Local workspace / Sequences
- Context & next steps

**Authored actions:**

- `126:7894` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `126:7895` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `126:7955` Record / Untitled sequence → `[{"type":"NODE","target":"125:6453","navigation":"NAVIGATE"}]`

## ST-498 · Flow completion / Sample Parameters

**Role:** Linked functional state. **Frame:** 126:7959 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-7959).

**Displayed content:**

- Analysis parameters
- Close
- SAMPLE-DNA-001 · version 7
- Source
- Scope
- Whole circular sequence · 3021 bp
- Enzymes
- EcoRI + BamHI
- Assumptions
- Review methylation sensitivity and ambiguous bases before experimental use
- Back to setup
- Return to result

**Authored actions:**

- `126:7963` Action / Close → `[{"type":"BACK"}]`
- `126:7985` Action / Back to setup → `[{"type":"NODE","target":"126:6169","navigation":"NAVIGATE"}]`
- `126:7987` Action / Return to result → `[{"type":"NODE","target":"126:6310","navigation":"NAVIGATE"}]`

## ST-499 · Flow completion / Personal Parameters

**Role:** Linked functional state. **Frame:** 126:7989 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-7989).

**Displayed content:**

- Analysis parameters
- Close
- DNA-PERSONAL-001 · version 1
- Source
- Scope
- Whole circular sequence · 3021 bp
- Enzymes
- EcoRI + BamHI
- Assumptions
- Review methylation sensitivity and ambiguous bases before experimental use
- Back to setup
- Return to result

**Authored actions:**

- `126:7993` Action / Close → `[{"type":"BACK"}]`
- `126:8015` Action / Back to setup → `[{"type":"NODE","target":"126:6766","navigation":"NAVIGATE"}]`
- `126:8017` Action / Return to result → `[{"type":"NODE","target":"126:6907","navigation":"NAVIGATE"}]`

## ST-500 · Flow completion / NewVersion Parameters

**Role:** Linked functional state. **Frame:** 126:8019 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=126-8019).

**Displayed content:**

- Analysis parameters
- Close
- DNA-001 · version 8
- Source
- Scope
- Whole circular sequence · 3021 bp
- Enzymes
- EcoRI + BamHI
- Assumptions
- Review methylation sensitivity and ambiguous bases before experimental use
- Back to setup
- Return to result

**Authored actions:**

- `126:8023` Action / Close → `[{"type":"BACK"}]`
- `126:8045` Action / Back to setup → `[{"type":"NODE","target":"126:7098","navigation":"NAVIGATE"}]`
- `126:8047` Action / Return to result → `[{"type":"NODE","target":"126:7239","navigation":"NAVIGATE"}]`

## ST-501 · Flow completion / Archived list DNA-001

**Role:** Linked functional state. **Frame:** 128:4143 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-4143).

**Displayed content:**

- Archived records
- DNA-001 was archived; its identity is retained.
- Search
- New
- Record
- Type
- Status
- pHelixBench-Demo · DNA-001
- DNA construct
- Archived
- Restore DNA-001
- Show active records
- Context & next steps

**Authored actions:**

- `128:4250` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `128:4251` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `128:4311` Record / pHelixBench-Demo · DNA-001 → `[{"type":"NODE","target":"125:5196","navigation":"NAVIGATE"}]`
- `128:4316` Action / Restore DNA-001 → `[{"type":"NODE","target":"125:5218","navigation":"NAVIGATE"}]`
- `128:4318` Action / Show active records → `[{"type":"NODE","target":"128:4320","navigation":"NAVIGATE"}]`

## ST-502 · Flow completion / Active after archive DNA-001

**Role:** Linked functional state. **Frame:** 128:4320 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-4320).

**Displayed content:**

- Registry
- Active records · DNA-001 excluded by archive filter
- Search
- New
- Name / ID
- State
- T7-F · PR-001
- Active
- AB-014 · AB-014
- OL-014 · OL-014
- Archived records
- Context & next steps

**Authored actions:**

- `128:4427` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `128:4428` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `128:4497` Action / Archived records → `[{"type":"NODE","target":"128:4143","navigation":"NAVIGATE"}]`

## ST-503 · Flow completion / Archived list PR-001

**Role:** Linked functional state. **Frame:** 128:4499 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-4499).

**Displayed content:**

- Archived records
- PR-001 was archived; its identity is retained.
- Search
- New
- Record
- Type
- Status
- T7-F · PR-001
- Primer
- Archived
- Restore PR-001
- Show active records
- Context & next steps

**Authored actions:**

- `128:4606` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `128:4607` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `128:4667` Record / T7-F · PR-001 → `[{"type":"NODE","target":"125:5266","navigation":"NAVIGATE"}]`
- `128:4672` Action / Restore PR-001 → `[{"type":"NODE","target":"125:5288","navigation":"NAVIGATE"}]`
- `128:4674` Action / Show active records → `[{"type":"NODE","target":"128:4676","navigation":"NAVIGATE"}]`

## ST-504 · Flow completion / Active after archive PR-001

**Role:** Linked functional state. **Frame:** 128:4676 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-4676).

**Displayed content:**

- Registry
- Active records · PR-001 excluded by archive filter
- Search
- New
- Name / ID
- State
- pHelixBench-Demo · DNA-001
- Active
- AB-014 · AB-014
- OL-014 · OL-014
- Archived records
- Context & next steps

**Authored actions:**

- `128:4783` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `128:4784` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `128:4853` Action / Archived records → `[{"type":"NODE","target":"128:4499","navigation":"NAVIGATE"}]`

## ST-505 · Flow completion / Archived list AB-014

**Role:** Linked functional state. **Frame:** 128:4855 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-4855).

**Displayed content:**

- Archived records
- AB-014 was archived; its identity is retained.
- Search
- New
- Record
- Type
- Status
- AB-014 · AB-014
- Biologic
- Archived
- Restore AB-014
- Show active records
- Context & next steps

**Authored actions:**

- `128:4962` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `128:4963` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `128:5023` Record / AB-014 · AB-014 → `[{"type":"NODE","target":"125:5336","navigation":"NAVIGATE"}]`
- `128:5028` Action / Restore AB-014 → `[{"type":"NODE","target":"125:5358","navigation":"NAVIGATE"}]`
- `128:5030` Action / Show active records → `[{"type":"NODE","target":"128:5032","navigation":"NAVIGATE"}]`

## ST-506 · Flow completion / Active after archive AB-014

**Role:** Linked functional state. **Frame:** 128:5032 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-5032).

**Displayed content:**

- Registry
- Active records · AB-014 excluded by archive filter
- Search
- New
- Name / ID
- State
- pHelixBench-Demo · DNA-001
- Active
- T7-F · PR-001
- OL-014 · OL-014
- Archived records
- Context & next steps

**Authored actions:**

- `128:5139` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `128:5140` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `128:5209` Action / Archived records → `[{"type":"NODE","target":"128:4855","navigation":"NAVIGATE"}]`

## ST-507 · Flow completion / Archived list OL-014

**Role:** Linked functional state. **Frame:** 128:5211 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-5211).

**Displayed content:**

- Archived records
- OL-014 was archived; its identity is retained.
- Search
- New
- Record
- Type
- Status
- OL-014 · OL-014
- RNA oligo
- Archived
- Restore OL-014
- Show active records
- Context & next steps

**Authored actions:**

- `128:5318` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `128:5319` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `128:5379` Record / OL-014 · OL-014 → `[{"type":"NODE","target":"125:5406","navigation":"NAVIGATE"}]`
- `128:5384` Action / Restore OL-014 → `[{"type":"NODE","target":"125:5428","navigation":"NAVIGATE"}]`
- `128:5386` Action / Show active records → `[{"type":"NODE","target":"128:5388","navigation":"NAVIGATE"}]`

## ST-508 · Flow completion / Active after archive OL-014

**Role:** Linked functional state. **Frame:** 128:5388 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-5388).

**Displayed content:**

- Registry
- Active records · OL-014 excluded by archive filter
- Search
- New
- Name / ID
- State
- pHelixBench-Demo · DNA-001
- Active
- T7-F · PR-001
- AB-014 · AB-014
- Archived records
- Context & next steps

**Authored actions:**

- `128:5495` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `128:5496` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `128:5565` Action / Archived records → `[{"type":"NODE","target":"128:5211","navigation":"NAVIGATE"}]`

## ST-509 · Flow completion / SaveCloseDone

**Role:** Linked functional state. **Frame:** 128:5573 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-5573).

**Displayed content:**

- Document saved and closed
- Close
- DNA-001 · version 8 · no unsaved changes remain.
- Location
- Local workspace / Sequences
- Continue to Home

**Authored actions:**

- `128:5577` Action / Close → `[{"type":"BACK"}]`
- `128:5587` Action / Continue to Home → `[{"type":"NODE","target":"132:5892","navigation":"NAVIGATE"}]`

## ST-510 · Flow completion / SelectedWell

**Role:** Linked functional state. **Frame:** 128:5594 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-5594).

**Displayed content:**

- Selected inventory well
- Close
- The selected coordinate is retained through review and completion.
- Well
- P-014 / A1
- Contents
- S-001 · DNA-001
- Quantity
- 50 µL · 100 ng/µL
- Occupancy
- Occupied
- Back to plate
- Assign contents

**Authored actions:**

- `128:5598` Action / Close → `[{"type":"BACK"}]`
- `128:5620` Action / Back to plate → `[{"type":"NODE","target":"107:31089","navigation":"NAVIGATE"}]`
- `128:5622` Action / Assign contents → `[{"type":"NODE","target":"128:5624","navigation":"NAVIGATE"}]`

## ST-511 · Flow completion / WellAssignment

**Role:** Linked functional state. **Frame:** 128:5624 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-5624).

**Displayed content:**

- Assign contents to selected well
- Close
- Review the existing contents before applying a change.
- Well
- P-014 / A1
- Existing contents
- S-001 · DNA-001
- New entity
- DNA-001 · pHelixBench-Demo
- Proposed quantity
- 50 µL · 100 ng/µL
- Role
- Sample
- Cancel
- Review assignment

**Authored actions:**

- `128:5628` Action / Close → `[{"type":"BACK"}]`
- `128:5654` Action / Cancel → `[{"type":"NODE","target":"128:5594","navigation":"NAVIGATE"}]`
- `128:5656` Action / Review assignment → `[{"type":"NODE","target":"128:5658","navigation":"NAVIGATE"}]`

## ST-512 · Flow completion / WellReview

**Role:** Linked functional state. **Frame:** 128:5658 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-5658).

**Displayed content:**

- Review selected-well assignment
- Close
- An occupied well requires explicit replacement confirmation.
- Review the before and after values. This action does not apply to the full plate.
- Well
- P-014 / A1
- Previous contents
- S-001 · DNA-001
- New contents
- DNA-001 · 50 µL · 100 ng/µL
- Effect
- This well only; linked experimental snapshots remain unchanged
- Keep previous contents
- Confirm this assignment

**Authored actions:**

- `128:5662` Action / Close → `[{"type":"BACK"}]`
- `128:5688` Action / Keep previous contents → `[{"type":"NODE","target":"128:5594","navigation":"NAVIGATE"}]`
- `128:5690` Action / Confirm this assignment → `[{"type":"SET_VARIABLE","variable":"VariableID:128:5591","value":"DNA-001 · assigned contents"},{"type":"SET_VARIABLE","variable":"VariableID:128:5592","value":"50 µL · 100 ng/µL"},{"type":"SET_VARIABLE","variable":"VariableID:128:5593","value":"Occupied"},{"type":"NODE","target":"128:5692","navigation":"NAVIGATE"}]`

## ST-513 · Flow completion / WellAssigned

**Role:** Linked functional state. **Frame:** 128:5692 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-5692).

**Displayed content:**

- Selected well updated
- Close
- The assignment retains the selected well coordinate.
- Well
- P-014 / A1
- Contents
- DNA-001 · pHelixBench-Demo
- Quantity
- 50 µL · 100 ng/µL
- Snapshot effect
- No change to earlier notebook snapshots
- Inspect updated well

**Authored actions:**

- `128:5696` Action / Close → `[{"type":"BACK"}]`
- `128:5718` Action / Inspect updated well → `[{"type":"NODE","target":"128:5594","navigation":"NAVIGATE"}]`

## ST-514 · Flow completion / BulkWellReview

**Role:** Linked functional state. **Frame:** 128:5816 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-5816).

**Displayed content:**

- Review four selected wells
- Close
- P-014 / A1–A4 · exactly four wells selected.
- Selected sample IDs
- S-001, S-002, S-003 and S-004
- Source entity
- DNA-001
- New quantity
- 50 µL · 100 ng/µL per selected well
- Impact
- Four existing assignments replaced; snapshots unchanged
- Cancel
- Apply to these four wells

**Authored actions:**

- `128:5820` Action / Close → `[{"type":"BACK"}]`
- `128:5842` Action / Cancel → `[{"type":"NODE","target":"107:31089","navigation":"NAVIGATE"}]`
- `128:5844` Action / Apply to these four wells → `[{"type":"NODE","target":"128:5846","navigation":"NAVIGATE"}]`

## ST-515 · Flow completion / BulkWellDone

**Role:** Linked functional state. **Frame:** 128:5846 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-5846).

**Displayed content:**

- Four selected wells updated
- Close
- P-014 / A1–A4 · no other wells changed.
- Entity
- DNA-001
- Quantity per well
- 50 µL · 100 ng/µL
- Return to plate

**Authored actions:**

- `128:5850` Action / Close → `[{"type":"BACK"}]`
- `128:5864` Action / Return to plate → `[{"type":"NODE","target":"107:31089","navigation":"NAVIGATE"}]`

## ST-516 · Flow completion / TransferInput

**Role:** Linked functional state. **Frame:** 128:5866 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-5866).

**Displayed content:**

- Plan a sample transfer
- Close
- The four selected source wells remain explicit.
- Source
- P-014 / A1–A4 · S-001–S-004
- Destination
- P-021 / B1–B4
- Volume
- 10 µL per source well
- Required review
- Source volume, destination occupancy and compatible units
- Cancel
- Review transfer

**Authored actions:**

- `128:5870` Action / Close → `[{"type":"BACK"}]`
- `128:5892` Action / Cancel → `[{"type":"NODE","target":"107:31089","navigation":"NAVIGATE"}]`
- `128:5894` Action / Review transfer → `[{"type":"NODE","target":"107:33703","navigation":"NAVIGATE"}]`

## ST-517 · Flow completion / TransferDone

**Role:** Linked functional state. **Frame:** 128:5896 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-5896).

**Displayed content:**

- Four samples transferred
- Close
- The source-to-destination mapping is retained.
- Source
- P-014 / A1–A4
- Destination
- P-021 / B1–B4
- Transferred quantity
- 10 µL each · 40 µL total
- Remaining source volume
- 40 µL each · from the reviewed 50 µL starting amount
- Snapshots
- Existing experiment snapshots remain unchanged
- Inspect transferred samples

**Authored actions:**

- `128:5900` Action / Close → `[{"type":"BACK"}]`
- `128:5926` Action / Inspect transferred samples → `[{"type":"NODE","target":"128:5928","navigation":"NAVIGATE"}]`

## ST-518 · Flow completion / TransferredSamples

**Role:** Linked functional state. **Frame:** 128:5928 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-5928).

**Displayed content:**

- Transferred samples
- Transfer record · four selected samples · live inventory
- Search
- New
- Sample
- Source
- Destination
- Transferred
- S-001
- P-014 / A1
- P-021 / B1
- 10 µL
- S-002
- P-014 / A2
- P-021 / B2
- S-003
- P-014 / A3
- P-021 / B3
- S-004
- P-014 / A4
- P-021 / B4
- Context & next steps
- The transfer is a live inventory
change. Notebook snapshots preserve
their earlier state.

**Authored actions:**

- `128:6035` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `128:6036` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`

## ST-519 · Flow completion / AutomationCorrection

**Role:** Linked functional state. **Frame:** 128:6118 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-6118).

**Displayed content:**

- Review corrected input
- Close
- R-018 failed validation; no records were written.
- Previous run
- R-018 · missing Sample ID in row 18
- Corrected source
- assay_results_corrected.csv · 96 rows
- Validation
- Required IDs present; mapping retained
- New run
- R-019 · TPL-012 · reject duplicate run IDs
- Keep original diagnostics
- Start reviewed run

**Authored actions:**

- `128:6122` Action / Close → `[{"type":"BACK"}]`
- `128:6144` Action / Keep original diagnostics → `[{"type":"NODE","target":"107:34042","navigation":"NAVIGATE"}]`
- `128:6146` Action / Start reviewed run → `[{"type":"NODE","target":"128:6148","navigation":"NAVIGATE"}]`

## ST-520 · Flow completion / AutomationRunning

**Role:** Linked functional state. **Frame:** 128:6148 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-6148).

**Displayed content:**

- Run R-019 is processing
- Close
- Input and destination are fixed for this run.
- Source
- assay_results_corrected.csv · 96 rows
- Template
- TPL-012
- Destination
- Expression results / Expression study
- Operation stage
- Validating before writeback
- Request cancellation
- View completed run

**Authored actions:**

- `128:6152` Action / Close → `[{"type":"BACK"}]`
- `128:6174` Action / Request cancellation → `[{"type":"NODE","target":"128:6178","navigation":"NAVIGATE"}]`
- `128:6176` Action / View completed run → `[{"type":"NODE","target":"128:6226","navigation":"NAVIGATE"}]`

## ST-521 · Flow completion / AutomationCancel

**Role:** Linked functional state. **Frame:** 128:6178 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-6178).

**Displayed content:**

- Cancel R-019?
- Close
- Cancellation depends on the current operation stage.
- Current stage
- Validation · writeback has not started
- Safe outcome
- Stop before creating result records
- Retained material
- Input and diagnostics remain available
- Keep running
- Cancel before writeback

**Authored actions:**

- `128:6182` Action / Close → `[{"type":"BACK"}]`
- `128:6200` Action / Keep running → `[{"type":"NODE","target":"128:6148","navigation":"NAVIGATE"}]`
- `128:6202` Action / Cancel before writeback → `[{"type":"NODE","target":"128:6204","navigation":"NAVIGATE"}]`

## ST-522 · Flow completion / AutomationCanceled

**Role:** Linked functional state. **Frame:** 128:6204 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-6204).

**Displayed content:**

- R-019 canceled
- Close
- No result records were written.
- Retained
- Source input and diagnostics
- Recovery
- Review input before starting a new run
- Review input
- Return to Automation

**Authored actions:**

- `128:6208` Action / Close → `[{"type":"BACK"}]`
- `128:6222` Action / Review input → `[{"type":"NODE","target":"128:6118","navigation":"NAVIGATE"}]`
- `128:6224` Action / Return to Automation → `[{"type":"NODE","target":"107:31876","navigation":"NAVIGATE"}]`

## ST-523 · Flow completion / AutomationComplete

**Role:** Linked functional state. **Frame:** 128:6226 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-6226).

**Displayed content:**

- Run R-019 completed
- Close
- 96 result rows written with retained source provenance.
- Run
- R-019 · TPL-012
- Input
- assay_results_corrected.csv · 96 rows
- Destination
- Expression results / Expression study
- Writeback
- 96 created · 0 rejected · duplicate run IDs protected
- View run output
- Return to Automation

**Authored actions:**

- `128:6230` Action / Close → `[{"type":"BACK"}]`
- `128:6252` Action / View run output → `[{"type":"NODE","target":"128:6256","navigation":"NAVIGATE"}]`
- `128:6254` Action / Return to Automation → `[{"type":"NODE","target":"107:31876","navigation":"NAVIGATE"}]`

## ST-524 · Flow completion / AutomationOutput

**Role:** Linked functional state. **Frame:** 128:6256 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-6256).

**Displayed content:**

- Run R-019 output
- 96 created result rows · source and template retained
- Search
- New
- Output group
- Count
- Source
- Created results
- 96
- R-019 / TPL-012
- Rejected rows
- 0
- Validation passed
- Input file
- 96 rows
- assay_results_corrected.csv
- Open linked notebook
- Inspect partial-write recovery
- Context & next steps

**Authored actions:**

- `128:6363` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `128:6364` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `128:6437` Action / Open linked notebook → `[{"type":"NODE","target":"132:6266","navigation":"NAVIGATE"}]`
- `128:6439` Action / Inspect partial-write recovery → `[{"type":"NODE","target":"128:6441","navigation":"NAVIGATE"}]`

## ST-525 · Flow completion / PartialWriteReview

**Role:** Linked functional state. **Frame:** 128:6441 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-6441).

**Displayed content:**

- Review a partially completed run
- Close
- R-020 stopped after 24 of 96 rows were written.
- Already written
- 24 result rows · retained output IDs
- Not written
- 72 source rows
- Retry policy
- Reconcile existing output IDs before any new writeback
- Safe choices
- Inspect the existing output or prepare a reviewed recovery run
- Inspect written rows
- Review remaining rows

**Authored actions:**

- `128:6445` Action / Close → `[{"type":"BACK"}]`
- `128:6467` Action / Inspect written rows → `[{"type":"NODE","target":"128:6471","navigation":"NAVIGATE"}]`
- `128:6469` Action / Review remaining rows → `[{"type":"NODE","target":"128:6650","navigation":"NAVIGATE"}]`

## ST-526 · Flow completion / PartialWriteOutput

**Role:** Linked functional state. **Frame:** 128:6471 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-6471).

**Displayed content:**

- Run R-020 retained output
- 24 of 96 rows were written before the interruption.
- Search
- New
- Outcome
- Count
- Next action
- Written
- 24
- Retain IDs and inspect output
- Pending
- 72
- Review recovery scope
- Review remaining rows
- Context & next steps

**Authored actions:**

- `128:6578` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `128:6579` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `128:6648` Action / Review remaining rows → `[{"type":"NODE","target":"128:6650","navigation":"NAVIGATE"}]`

## ST-527 · Flow completion / ResumeWriteReview

**Role:** Linked functional state. **Frame:** 128:6650 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-6650).

**Displayed content:**

- Review recovery run
- Close
- Do not repeat the 24 successful writes.
- Previous run
- R-020 · 24 outputs retained
- Reconciled scope
- 72 remaining source rows only
- Recovery run
- R-021 · same source/template references
- Required support
- Stable output IDs and duplicate detection must be enforced
- Cancel
- Create reviewed recovery run

**Authored actions:**

- `128:6654` Action / Close → `[{"type":"BACK"}]`
- `128:6676` Action / Cancel → `[{"type":"NODE","target":"128:6441","navigation":"NAVIGATE"}]`
- `128:6678` Action / Create reviewed recovery run → `[{"type":"NODE","target":"128:6680","navigation":"NAVIGATE"}]`

## ST-528 · Flow completion / ResumeWriteDone

**Role:** Linked functional state. **Frame:** 128:6680 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-6680).

**Displayed content:**

- Recovery run R-021 completed
- Close
- 72 remaining rows written; 24 earlier outputs retained.
- Combined output
- 96 unique result rows
- Provenance
- R-020 retained outputs + R-021 recovery outputs
- Duplicate check
- No earlier output written a second time
- Return to Automation

**Authored actions:**

- `128:6684` Action / Close → `[{"type":"BACK"}]`
- `128:6702` Action / Return to Automation → `[{"type":"NODE","target":"107:31876","navigation":"NAVIGATE"}]`

## ST-529 · Flow completion / Operation outcome 107:25182

**Role:** Linked functional state. **Frame:** 128:6704 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-6704).

**Displayed content:**

- Add Primer updated
- Close
- The completed action stays associated with its source task.
- Operation
- Add primer
- Source task
- Add Primer
- Reviewed context
- Name · M13-F (-20) · Sequence 5′→3′
- Next step
- Inspect the updated task or return to its owning workspace
- Review task
- Open owning workspace

**Authored actions:**

- `128:6708` Action / Close → `[{"type":"BACK"}]`
- `128:6730` Action / Review task → `[{"type":"NODE","target":"107:25182","navigation":"NAVIGATE"}]`
- `128:6732` Action / Open owning workspace → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`

## ST-530 · Flow completion / Operation outcome 107:25303

**Role:** Linked functional state. **Frame:** 128:6734 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-6734).

**Displayed content:**

- Primer Design updated
- Close
- The completed action stays associated with its source task.
- Operation
- Add selected pair
- Source task
- Primer Design
- Reviewed context
- DNA-001 v7 · selected anchor 97–116. Review the proposed target 97–351 b
- Next step
- Inspect the updated task or return to its owning workspace
- Review task
- Open owning workspace

**Authored actions:**

- `128:6738` Action / Close → `[{"type":"BACK"}]`
- `128:6760` Action / Review task → `[{"type":"NODE","target":"107:25303","navigation":"NAVIGATE"}]`
- `128:6762` Action / Open owning workspace → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`

## ST-531 · Flow completion / Operation outcome 107:25875

**Role:** Linked functional state. **Frame:** 128:6764 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-6764).

**Displayed content:**

- Registry Entry updated
- Close
- The completed action stays associated with its source task.
- Operation
- Save entry
- Source task
- Registry Entry
- Reviewed context
- Kind · Plasmid · Name
- Next step
- Inspect the updated task or return to its owning workspace
- Review task
- Open owning workspace

**Authored actions:**

- `128:6768` Action / Close → `[{"type":"BACK"}]`
- `128:6790` Action / Review task → `[{"type":"NODE","target":"107:25875","navigation":"NAVIGATE"}]`
- `128:6792` Action / Open owning workspace → `[{"type":"NODE","target":"107:30702","navigation":"NAVIGATE"}]`

## ST-532 · Flow completion / Operation outcome 107:25918

**Role:** Reference or alternate fixture. **Frame:** 128:6794 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-6794).

**Displayed content:**

- Unsaved Changes updated
- Close
- The completed action stays associated with its source task.
- Operation
- Save
- Source task
- Unsaved Changes
- Reviewed context
- Save changes to pHelixBench-Demo before closing? · Your edits will be lo
- Next step
- Inspect the updated task or return to its owning workspace
- Review task
- Open owning workspace

**Authored actions:**

- `128:6798` Action / Close → `[{"type":"BACK"}]`
- `128:6820` Action / Review task → `[{"type":"NODE","target":"107:25918","navigation":"NAVIGATE"}]`
- `128:6822` Action / Open owning workspace → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`

## ST-533 · Flow completion / Operation outcome 107:25938

**Role:** Linked functional state. **Frame:** 128:6824 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-6824).

**Displayed content:**

- Genetic code updated
- Close
- The completed action stays associated with its source task.
- Operation
- Use selected code
- Source task
- Genetic code
- Reviewed context
- Current code · Standard (1) · Use document default
- Next step
- Inspect the updated task or return to its owning workspace
- Review task
- Open owning workspace

**Authored actions:**

- `128:6828` Action / Close → `[{"type":"BACK"}]`
- `128:6850` Action / Review task → `[{"type":"NODE","target":"107:25938","navigation":"NAVIGATE"}]`
- `128:6852` Action / Open owning workspace → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`

## ST-534 · Flow completion / Operation outcome 107:25986

**Role:** Reference or alternate fixture. **Frame:** 128:6854 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-6854).

**Displayed content:**

- Find in sequence updated
- Close
- The completed action stays associated with its source task.
- Operation
- Find
- Source task
- Find in sequence
- Reviewed context
- Search mode · DNA · Protein · Annotation name · Query
- Next step
- Inspect the updated task or return to its owning workspace
- Review task
- Open owning workspace

**Authored actions:**

- `128:6858` Action / Close → `[{"type":"BACK"}]`
- `128:6880` Action / Review task → `[{"type":"NODE","target":"107:25986","navigation":"NAVIGATE"}]`
- `128:6882` Action / Open owning workspace → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`

## ST-535 · Flow completion / Operation outcome 107:26038

**Role:** Linked functional state. **Frame:** 128:6884 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-6884).

**Displayed content:**

- Rotate sequence origin updated
- Close
- The completed action stays associated with its source task.
- Operation
- Review rotation
- Source task
- Rotate sequence origin
- Reviewed context
- Document · pHelixBench-Demo · circular · New origin position
- Next step
- Inspect the updated task or return to its owning workspace
- Review task
- Open owning workspace

**Authored actions:**

- `128:6888` Action / Close → `[{"type":"BACK"}]`
- `128:6910` Action / Review task → `[{"type":"NODE","target":"107:26038","navigation":"NAVIGATE"}]`
- `128:6912` Action / Open owning workspace → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`

## ST-536 · Flow completion / Operation outcome 107:26086

**Role:** Linked functional state. **Frame:** 128:6914 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-6914).

**Displayed content:**

- Color sequence selection updated
- Close
- The completed action stays associated with its source task.
- Operation
- Apply color
- Source task
- Color sequence selection
- Reviewed context
- Selected range · 169..423 · Strands
- Next step
- Inspect the updated task or return to its owning workspace
- Review task
- Open owning workspace

**Authored actions:**

- `128:6918` Action / Close → `[{"type":"BACK"}]`
- `128:6940` Action / Review task → `[{"type":"NODE","target":"107:26086","navigation":"NAVIGATE"}]`
- `128:6942` Action / Open owning workspace → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`

## ST-537 · Flow completion / Operation outcome 107:26142

**Role:** Linked functional state. **Frame:** 128:6944 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-6944).

**Displayed content:**

- Registry lookup updated
- Close
- The completed action stays associated with its source task.
- Operation
- Open selected entity
- Source task
- Registry lookup
- Reviewed context
- Entity kind · Plasmids · Strains · Primers · Selected entity
- Next step
- Inspect the updated task or return to its owning workspace
- Review task
- Open owning workspace

**Authored actions:**

- `128:6948` Action / Close → `[{"type":"BACK"}]`
- `128:6970` Action / Review task → `[{"type":"NODE","target":"107:26142","navigation":"NAVIGATE"}]`
- `128:6972` Action / Open owning workspace → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`

## ST-538 · Flow completion / Operation outcome 107:26194

**Role:** Linked functional state. **Frame:** 128:6974 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-6974).

**Displayed content:**

- Feature library updated
- Close
- The completed action stays associated with its source task.
- Operation
- Review annotations
- Source task
- Feature library
- Reviewed context
- Motif name · T7 promoter · DNA motif
- Next step
- Inspect the updated task or return to its owning workspace
- Review task
- Open owning workspace

**Authored actions:**

- `128:6978` Action / Close → `[{"type":"BACK"}]`
- `128:7000` Action / Review task → `[{"type":"NODE","target":"107:26194","navigation":"NAVIGATE"}]`
- `128:7002` Action / Open owning workspace → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`

## ST-539 · Flow completion / Operation outcome 107:26224

**Role:** Reference or alternate fixture. **Frame:** 128:7004 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-7004).

**Displayed content:**

- Bulk document actions updated
- Close
- The completed action stays associated with its source task.
- Operation
- Apply selected action
- Source task
- Bulk document actions
- Reviewed context
- Selected documents · pHelixBench-Demo · PCR product · Tag
- Next step
- Inspect the updated task or return to its owning workspace
- Review task
- Open owning workspace

**Authored actions:**

- `128:7008` Action / Close → `[{"type":"BACK"}]`
- `128:7030` Action / Review task → `[{"type":"NODE","target":"107:26224","navigation":"NAVIGATE"}]`
- `128:7032` Action / Open owning workspace → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`

## ST-540 · Flow completion / Operation outcome 107:26250

**Role:** Linked functional state. **Frame:** 128:7034 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-7034).

**Displayed content:**

- Choose notebook entry updated
- Close
- The completed action stays associated with its source task.
- Operation
- Open entry
- Source task
- Choose notebook entry
- Reviewed context
- Entry status · Draft · Submitted · Approved · Entry
- Next step
- Inspect the updated task or return to its owning workspace
- Review task
- Open owning workspace

**Authored actions:**

- `128:7038` Action / Close → `[{"type":"BACK"}]`
- `128:7060` Action / Review task → `[{"type":"NODE","target":"107:26250","navigation":"NAVIGATE"}]`
- `128:7062` Action / Open owning workspace → `[{"type":"NODE","target":"107:30702","navigation":"NAVIGATE"}]`

## ST-541 · Flow completion / Operation outcome 107:26276

**Role:** Linked functional state. **Frame:** 128:7064 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-7064).

**Displayed content:**

- Projects & folders updated
- Close
- The completed action stays associated with its source task.
- Operation
- Create folder
- Source task
- Projects & folders
- Reviewed context
- Project or folder name · Expression construct · Parent folder
- Next step
- Inspect the updated task or return to its owning workspace
- Review task
- Open owning workspace

**Authored actions:**

- `128:7068` Action / Close → `[{"type":"BACK"}]`
- `128:7090` Action / Review task → `[{"type":"NODE","target":"107:26276","navigation":"NAVIGATE"}]`
- `128:7092` Action / Open owning workspace → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`

## ST-542 · Flow completion / Operation outcome 107:26302

**Role:** Linked functional state. **Frame:** 128:7094 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-7094).

**Displayed content:**

- Audit log updated
- Close
- The completed action stays associated with its source task.
- Operation
- Export CSV
- Source task
- Audit log
- Reviewed context
- Scope · Workspace events · Latest event
- Next step
- Inspect the updated task or return to its owning workspace
- Review task
- Open owning workspace

**Authored actions:**

- `128:7098` Action / Close → `[{"type":"BACK"}]`
- `128:7120` Action / Review task → `[{"type":"NODE","target":"107:26302","navigation":"NAVIGATE"}]`
- `128:7122` Action / Open owning workspace → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`

## ST-543 · Flow completion / Operation outcome 107:26328

**Role:** Reference or alternate fixture. **Frame:** 128:7124 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-7124).

**Displayed content:**

- Resume workspace item updated
- Close
- The completed action stays associated with its source task.
- Operation
- Resume selected item
- Source task
- Resume workspace item
- Reviewed context
- My work · pHelixBench-Demo · PCR product · Expression insert · Recent en
- Next step
- Inspect the updated task or return to its owning workspace
- Review task
- Open owning workspace

**Authored actions:**

- `128:7128` Action / Close → `[{"type":"BACK"}]`
- `128:7150` Action / Review task → `[{"type":"NODE","target":"107:26328","navigation":"NAVIGATE"}]`
- `128:7152` Action / Open owning workspace → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`

## ST-544 · Flow completion / Operation outcome 107:26354

**Role:** Linked functional state. **Frame:** 128:7154 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-7154).

**Displayed content:**

- Entity relationships updated
- Close
- The completed action stays associated with its source task.
- Operation
- Link entities
- Source task
- Entity relationships
- Reviewed context
- Source entity · pHelixBench-Demo · Related entity
- Next step
- Inspect the updated task or return to its owning workspace
- Review task
- Open owning workspace

**Authored actions:**

- `128:7158` Action / Close → `[{"type":"BACK"}]`
- `128:7180` Action / Review task → `[{"type":"NODE","target":"107:26354","navigation":"NAVIGATE"}]`
- `128:7182` Action / Open owning workspace → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`

## ST-545 · Flow completion / Operation outcome 107:26410

**Role:** Linked functional state. **Frame:** 128:7184 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-7184).

**Displayed content:**

- Task details & output updated
- Close
- The completed action stays associated with its source task.
- Operation
- Complete with output
- Source task
- Task details & output
- Reviewed context
- Details · Output · Requests
- Next step
- Inspect the updated task or return to its owning workspace
- Review task
- Open owning workspace

**Authored actions:**

- `128:7188` Action / Close → `[{"type":"BACK"}]`
- `128:7210` Action / Review task → `[{"type":"NODE","target":"107:26410","navigation":"NAVIGATE"}]`
- `128:7212` Action / Open owning workspace → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`

## ST-546 · Flow completion / Operation outcome 107:26448

**Role:** Linked functional state. **Frame:** 128:7214 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-7214).

**Displayed content:**

- Register biologic updated
- Close
- The completed action stays associated with its source task.
- Operation
- Register biologic
- Source task
- Reviewed context
- Demo IgG1 · Format · IgG · Bispecific · Nanobody · TCR · Fab
- Next step
- Inspect the updated task or return to its owning workspace
- Review task
- Open owning workspace

**Authored actions:**

- `128:7218` Action / Close → `[{"type":"BACK"}]`
- `128:7240` Action / Review task → `[{"type":"NODE","target":"107:26448","navigation":"NAVIGATE"}]`
- `128:7242` Action / Open owning workspace → `[{"type":"NODE","target":"107:32270","navigation":"NAVIGATE"}]`

## ST-547 · Flow completion / Operation outcome 107:26478

**Role:** Linked functional state. **Frame:** 128:7244 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-7244).

**Displayed content:**

- Add compound updated
- Close
- The completed action stays associated with its source task.
- Operation
- Add compound
- Source task
- Reviewed context
- Ethanol · SMILES · CCO
- Next step
- Inspect the updated task or return to its owning workspace
- Review task
- Open owning workspace

**Authored actions:**

- `128:7248` Action / Close → `[{"type":"BACK"}]`
- `128:7270` Action / Review task → `[{"type":"NODE","target":"107:26478","navigation":"NAVIGATE"}]`
- `128:7272` Action / Open owning workspace → `[{"type":"NODE","target":"107:32457","navigation":"NAVIGATE"}]`

## ST-548 · Flow completion / Operation outcome 107:26504

**Role:** Linked functional state. **Frame:** 128:7274 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-7274).

**Displayed content:**

- Add reaction updated
- Close
- The completed action stays associated with its source task.
- Operation
- Add reaction
- Source task
- Reviewed context
- Esterification · Reactant · Ethanol
- Next step
- Inspect the updated task or return to its owning workspace
- Review task
- Open owning workspace

**Authored actions:**

- `128:7278` Action / Close → `[{"type":"BACK"}]`
- `128:7300` Action / Review task → `[{"type":"NODE","target":"107:26504","navigation":"NAVIGATE"}]`
- `128:7302` Action / Open owning workspace → `[{"type":"NODE","target":"107:32457","navigation":"NAVIGATE"}]`

## ST-549 · Flow completion / Operation outcome 107:26534

**Role:** Linked functional state. **Frame:** 128:7304 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-7304).

**Displayed content:**

- Add material lot updated
- Close
- The completed action stays associated with its source task.
- Operation
- Add lot
- Source task
- Add material lot
- Reviewed context
- Compound · Ethanol · Quantity
- Next step
- Inspect the updated task or return to its owning workspace
- Review task
- Open owning workspace

**Authored actions:**

- `128:7308` Action / Close → `[{"type":"BACK"}]`
- `128:7330` Action / Review task → `[{"type":"NODE","target":"107:26534","navigation":"NAVIGATE"}]`
- `128:7332` Action / Open owning workspace → `[{"type":"NODE","target":"107:32457","navigation":"NAVIGATE"}]`

## ST-550 · Flow completion / Operation outcome 107:26560

**Role:** Linked functional state. **Frame:** 128:7334 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-7334).

**Displayed content:**

- Add RNA / oligo updated
- Close
- The completed action stays associated with its source task.
- Operation
- Add oligo
- Source task
- Add RNA / oligo
- Reviewed context
- Sequence · Modifications · HELM
- Next step
- Inspect the updated task or return to its owning workspace
- Review task
- Open owning workspace

**Authored actions:**

- `128:7338` Action / Close → `[{"type":"BACK"}]`
- `128:7360` Action / Review task → `[{"type":"NODE","target":"107:26560","navigation":"NAVIGATE"}]`
- `128:7362` Action / Open owning workspace → `[{"type":"NODE","target":"107:32649","navigation":"NAVIGATE"}]`

## ST-551 · Flow completion / Operation outcome 107:26590

**Role:** Linked functional state. **Frame:** 128:7364 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-7364).

**Displayed content:**

- New study updated
- Close
- The completed action stays associated with its source task.
- Operation
- Create study
- Source task
- New study
- Reviewed context
- Study name · Demo study 01 · Species
- Next step
- Inspect the updated task or return to its owning workspace
- Review task
- Open owning workspace

**Authored actions:**

- `128:7368` Action / Close → `[{"type":"BACK"}]`
- `128:7390` Action / Review task → `[{"type":"NODE","target":"107:26590","navigation":"NAVIGATE"}]`
- `128:7392` Action / Open owning workspace → `[{"type":"NODE","target":"107:32841","navigation":"NAVIGATE"}]`

## ST-552 · Flow completion / Operation outcome 107:26612

**Role:** Linked functional state. **Frame:** 128:7394 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-7394).

**Displayed content:**

- Add animal updated
- Close
- The completed action stays associated with its source task.
- Operation
- Add animal
- Source task
- Reviewed context
- Demo study 01 · Animal tag · M001
- Next step
- Inspect the updated task or return to its owning workspace
- Review task
- Open owning workspace

**Authored actions:**

- `128:7398` Action / Close → `[{"type":"BACK"}]`
- `128:7420` Action / Review task → `[{"type":"NODE","target":"107:26612","navigation":"NAVIGATE"}]`
- `128:7422` Action / Open owning workspace → `[{"type":"NODE","target":"107:32841","navigation":"NAVIGATE"}]`

## ST-553 · Flow completion / Operation outcome 107:26638

**Role:** Linked functional state. **Frame:** 128:7424 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-7424).

**Displayed content:**

- Record animal measurement updated
- Close
- The completed action stays associated with its source task.
- Operation
- Record measurement
- Source task
- Record animal measurement
- Reviewed context
- Study · Demo study 01 · Animal
- Next step
- Inspect the updated task or return to its owning workspace
- Review task
- Open owning workspace

**Authored actions:**

- `128:7428` Action / Close → `[{"type":"BACK"}]`
- `128:7450` Action / Review task → `[{"type":"NODE","target":"107:26638","navigation":"NAVIGATE"}]`
- `128:7452` Action / Open owning workspace → `[{"type":"NODE","target":"107:32841","navigation":"NAVIGATE"}]`

## ST-554 · Flow completion / Operation outcome 107:26664

**Role:** Linked functional state. **Frame:** 128:7454 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-7454).

**Displayed content:**

- New bioprocess recipe updated
- Close
- The completed action stays associated with its source task.
- Operation
- Create recipe
- Source task
- New bioprocess recipe
- Reviewed context
- Recipe name · Demo upstream → downstream · Product
- Next step
- Inspect the updated task or return to its owning workspace
- Review task
- Open owning workspace

**Authored actions:**

- `128:7458` Action / Close → `[{"type":"BACK"}]`
- `128:7480` Action / Review task → `[{"type":"NODE","target":"107:26664","navigation":"NAVIGATE"}]`
- `128:7482` Action / Open owning workspace → `[{"type":"NODE","target":"107:33033","navigation":"NAVIGATE"}]`

## ST-555 · Flow completion / Operation outcome 107:26686

**Role:** Linked functional state. **Frame:** 128:7484 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-7484).

**Displayed content:**

- Add replicate updated
- Close
- The completed action stays associated with its source task.
- Operation
- Add replicate
- Source task
- Reviewed context
- Run-01 · Replicate name · Rep-03
- Next step
- Inspect the updated task or return to its owning workspace
- Review task
- Open owning workspace

**Authored actions:**

- `128:7488` Action / Close → `[{"type":"BACK"}]`
- `128:7510` Action / Review task → `[{"type":"NODE","target":"107:26686","navigation":"NAVIGATE"}]`
- `128:7512` Action / Open owning workspace → `[{"type":"NODE","target":"107:33033","navigation":"NAVIGATE"}]`

## ST-556 · Flow completion / Operation outcome 107:26716

**Role:** Linked functional state. **Frame:** 128:7514 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-7514).

**Displayed content:**

- Configure pipeline updated
- Close
- The completed action stays associated with its source task.
- Operation
- Save pipeline
- Source task
- Configure pipeline
- Reviewed context
- Pipeline name · Assay QC import · Input CSV
- Next step
- Inspect the updated task or return to its owning workspace
- Review task
- Open owning workspace

**Authored actions:**

- `128:7518` Action / Close → `[{"type":"BACK"}]`
- `128:7540` Action / Review task → `[{"type":"NODE","target":"107:26716","navigation":"NAVIGATE"}]`
- `128:7542` Action / Open owning workspace → `[{"type":"NODE","target":"107:31876","navigation":"NAVIGATE"}]`

## ST-557 · Flow completion / Operation outcome 107:26802

**Role:** Linked functional state. **Frame:** 128:7544 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-7544).

**Displayed content:**

- Randomize cohorts updated
- Close
- The completed action stays associated with its source task.
- Operation
- Review assignment
- Source task
- Randomize cohorts
- Reviewed context
- Study · Demo study 01 · Number of groups
- Next step
- Inspect the updated task or return to its owning workspace
- Review task
- Open owning workspace

**Authored actions:**

- `128:7548` Action / Close → `[{"type":"BACK"}]`
- `128:7570` Action / Review task → `[{"type":"NODE","target":"107:26802","navigation":"NAVIGATE"}]`
- `128:7572` Action / Open owning workspace → `[{"type":"NODE","target":"107:32841","navigation":"NAVIGATE"}]`

## ST-558 · Flow completion / Operation outcome 107:26858

**Role:** Linked functional state. **Frame:** 128:7574 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-7574).

**Displayed content:**

- Assign plate wells updated
- Close
- The completed action stays associated with its source task.
- Operation
- Review well assignment
- Source task
- Assign plate wells
- Reviewed context
- Selected plate · P-014 · Well range
- Next step
- Inspect the updated task or return to its owning workspace
- Review task
- Open owning workspace

**Authored actions:**

- `128:7578` Action / Close → `[{"type":"BACK"}]`
- `128:7600` Action / Review task → `[{"type":"NODE","target":"107:26858","navigation":"NAVIGATE"}]`
- `128:7602` Action / Open owning workspace → `[{"type":"NODE","target":"107:31089","navigation":"NAVIGATE"}]`

## ST-559 · Flow completion / Operation outcome 107:26892

**Role:** Linked functional state. **Frame:** 128:7604 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-7604).

**Displayed content:**

- Choose entry template updated
- Close
- The completed action stays associated with its source task.
- Operation
- Create draft from template
- Source task
- Choose entry template
- Reviewed context
- Template · Experiment · PCR · Assembly review · Project
- Next step
- Inspect the updated task or return to its owning workspace
- Review task
- Open owning workspace

**Authored actions:**

- `128:7608` Action / Close → `[{"type":"BACK"}]`
- `128:7630` Action / Review task → `[{"type":"NODE","target":"107:26892","navigation":"NAVIGATE"}]`
- `128:7632` Action / Open owning workspace → `[{"type":"NODE","target":"107:30702","navigation":"NAVIGATE"}]`

## ST-560 · Flow completion / Operation outcome 107:26918

**Role:** Linked functional state. **Frame:** 128:7634 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-7634).

**Displayed content:**

- Configure insight updated
- Close
- The completed action stays associated with its source task.
- Operation
- Save insight
- Source task
- Configure insight
- Reviewed context
- Insight title · Registry by kind · Dataset
- Next step
- Inspect the updated task or return to its owning workspace
- Review task
- Open owning workspace

**Authored actions:**

- `128:7638` Action / Close → `[{"type":"BACK"}]`
- `128:7660` Action / Review task → `[{"type":"NODE","target":"107:26918","navigation":"NAVIGATE"}]`
- `128:7662` Action / Open owning workspace → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`

## ST-561 · Flow completion / Operation outcome 107:26948

**Role:** Linked functional state. **Frame:** 128:7664 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-7664).

**Displayed content:**

- Columns & saved views updated
- Close
- The completed action stays associated with its source task.
- Operation
- Apply view
- Source task
- Columns & saved views
- Reviewed context
- Visible columns · Name · Kind · Direction · Length · Location · Product 
- Next step
- Inspect the updated task or return to its owning workspace
- Review task
- Open owning workspace

**Authored actions:**

- `128:7668` Action / Close → `[{"type":"BACK"}]`
- `128:7690` Action / Review task → `[{"type":"NODE","target":"107:26948","navigation":"NAVIGATE"}]`
- `128:7692` Action / Open owning workspace → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`

## ST-562 · Flow completion / Operation outcome 107:26978

**Role:** Linked functional state. **Frame:** 128:7694 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-7694).

**Displayed content:**

- Enzyme filters updated
- Close
- The completed action stays associated with its source task.
- Operation
- Apply filters
- Source task
- Enzyme filters
- Reviewed context
- Name contains · Enter name contains… · Cut count
- Next step
- Inspect the updated task or return to its owning workspace
- Review task
- Open owning workspace

**Authored actions:**

- `128:7698` Action / Close → `[{"type":"BACK"}]`
- `128:7720` Action / Review task → `[{"type":"NODE","target":"107:26978","navigation":"NAVIGATE"}]`
- `128:7722` Action / Open owning workspace → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`

## ST-563 · Flow completion / Operation outcome 107:27008

**Role:** Linked functional state. **Frame:** 128:7724 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-7724).

**Displayed content:**

- Add storage container updated
- Close
- The completed action stays associated with its source task.
- Operation
- Add container
- Source task
- Add storage container
- Reviewed context
- Container kind · Location · Box · Plate · Tube · Name
- Next step
- Inspect the updated task or return to its owning workspace
- Review task
- Open owning workspace

**Authored actions:**

- `128:7728` Action / Close → `[{"type":"BACK"}]`
- `128:7750` Action / Review task → `[{"type":"NODE","target":"107:27008","navigation":"NAVIGATE"}]`
- `128:7752` Action / Open owning workspace → `[{"type":"NODE","target":"107:31089","navigation":"NAVIGATE"}]`

## ST-564 · Flow completion / Operation outcome 107:27034

**Role:** Linked functional state. **Frame:** 128:7754 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-7754).

**Displayed content:**

- New task updated
- Close
- The completed action stays associated with its source task.
- Operation
- Create task
- Source task
- New task
- Reviewed context
- Task title · Validate expression cassette · Input schema
- Next step
- Inspect the updated task or return to its owning workspace
- Review task
- Open owning workspace

**Authored actions:**

- `128:7758` Action / Close → `[{"type":"BACK"}]`
- `128:7780` Action / Review task → `[{"type":"NODE","target":"107:27034","navigation":"NAVIGATE"}]`
- `128:7782` Action / Open owning workspace → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`

## ST-565 · Flow completion / Operation outcome 107:27060

**Role:** Linked functional state. **Frame:** 128:7784 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-7784).

**Displayed content:**

- New request updated
- Close
- The completed action stays associated with its source task.
- Operation
- Save request draft
- Source task
- New request
- Reviewed context
- Request name · Sequence review request · Input schema
- Next step
- Inspect the updated task or return to its owning workspace
- Review task
- Open owning workspace

**Authored actions:**

- `128:7788` Action / Close → `[{"type":"BACK"}]`
- `128:7810` Action / Review task → `[{"type":"NODE","target":"107:27060","navigation":"NAVIGATE"}]`
- `128:7812` Action / Open owning workspace → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`

## ST-566 · Flow completion / Operation outcome 107:27086

**Role:** Linked functional state. **Frame:** 128:7814 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-7814).

**Displayed content:**

- Review record change updated
- Close
- The completed action stays associated with its source task.
- Operation
- Confirm change
- Source task
- Review record change
- Reviewed context
- Selected record · The record you selected · Action
- Next step
- Inspect the updated task or return to its owning workspace
- Review task
- Open owning workspace

**Authored actions:**

- `128:7818` Action / Close → `[{"type":"BACK"}]`
- `128:7840` Action / Review task → `[{"type":"NODE","target":"107:27086","navigation":"NAVIGATE"}]`
- `128:7842` Action / Open owning workspace → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`

## ST-567 · Flow completion / Operation outcome 107:27142

**Role:** Linked functional state. **Frame:** 128:7844 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-7844).

**Displayed content:**

- Export document updated
- Close
- The completed action stays associated with its source task.
- Operation
- Export
- Source task
- Export document
- Reviewed context
- Document · pHelixBench-Demo · Format
- Next step
- Inspect the updated task or return to its owning workspace
- Review task
- Open owning workspace

**Authored actions:**

- `128:7848` Action / Close → `[{"type":"BACK"}]`
- `128:7870` Action / Review task → `[{"type":"NODE","target":"107:27142","navigation":"NAVIGATE"}]`
- `128:7872` Action / Open owning workspace → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`

## ST-568 · Flow completion / Operation outcome 107:27168

**Role:** Reference or alternate fixture. **Frame:** 128:7874 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-7874).

**Displayed content:**

- Search workspace records updated
- Close
- The completed action stays associated with its source task.
- Operation
- Open selected record
- Source task
- Search workspace records
- Reviewed context
- Search records · Enter search records… · Record kind
- Next step
- Inspect the updated task or return to its owning workspace
- Review task
- Open owning workspace

**Authored actions:**

- `128:7878` Action / Close → `[{"type":"BACK"}]`
- `128:7900` Action / Review task → `[{"type":"NODE","target":"107:27168","navigation":"NAVIGATE"}]`
- `128:7902` Action / Open owning workspace → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`

## ST-569 · Flow completion / Operation outcome 107:28999

**Role:** Reference or alternate fixture. **Frame:** 128:7904 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-7904).

**Displayed content:**

- Open / import sequence updated
- Close
- The completed action stays associated with its source task.
- Operation
- Open file
- Source task
- Open / import sequence
- Reviewed context
- File · Choose a local sequence file · Supported formats
- Next step
- Inspect the updated task or return to its owning workspace
- Review task
- Open owning workspace

**Authored actions:**

- `128:7908` Action / Close → `[{"type":"BACK"}]`
- `128:7930` Action / Review task → `[{"type":"NODE","target":"107:28999","navigation":"NAVIGATE"}]`
- `128:7932` Action / Open owning workspace → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`

## ST-570 · Flow completion / Operation outcome 107:29025

**Role:** Linked functional state. **Frame:** 128:7934 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-7934).

**Displayed content:**

- Save document version updated
- Close
- The completed action stays associated with its source task.
- Operation
- Save version
- Source task
- Save document version
- Reviewed context
- Document · pHelixBench-Demo · Version name
- Next step
- Inspect the updated task or return to its owning workspace
- Review task
- Open owning workspace

**Authored actions:**

- `128:7938` Action / Close → `[{"type":"BACK"}]`
- `128:7960` Action / Review task → `[{"type":"NODE","target":"107:29025","navigation":"NAVIGATE"}]`
- `128:7962` Action / Open owning workspace → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`

## ST-571 · Flow completion / Operation outcome 107:29047

**Role:** Linked functional state. **Frame:** 128:7964 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-7964).

**Displayed content:**

- Export workspace data updated
- Close
- The completed action stays associated with its source task.
- Operation
- Export CSV
- Source task
- Export workspace data
- Reviewed context
- Dataset · Registry / Inventory / Tasks / Entries · Export format
- Next step
- Inspect the updated task or return to its owning workspace
- Review task
- Open owning workspace

**Authored actions:**

- `128:7968` Action / Close → `[{"type":"BACK"}]`
- `128:7990` Action / Review task → `[{"type":"NODE","target":"107:29047","navigation":"NAVIGATE"}]`
- `128:7992` Action / Open owning workspace → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`

## ST-572 · Flow completion / Operation outcome 107:29155

**Role:** Linked functional state. **Frame:** 128:7994 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-7994).

**Displayed content:**

- Load pipeline CSV updated
- Close
- The completed action stays associated with its source task.
- Operation
- Use CSV
- Source task
- Load pipeline CSV
- Reviewed context
- Selected pipeline · Assay QC import · CSV file
- Next step
- Inspect the updated task or return to its owning workspace
- Review task
- Open owning workspace

**Authored actions:**

- `128:7998` Action / Close → `[{"type":"BACK"}]`
- `128:8020` Action / Review task → `[{"type":"NODE","target":"107:29155","navigation":"NAVIGATE"}]`
- `128:8022` Action / Open owning workspace → `[{"type":"NODE","target":"107:31876","navigation":"NAVIGATE"}]`

## ST-573 · Flow completion / Operation outcome 107:29211

**Role:** Linked functional state. **Frame:** 128:8024 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-8024).

**Displayed content:**

- Save document updated
- Close
- The completed action stays associated with its source task.
- Operation
- Save document
- Source task
- Reviewed context
- pHelixBench-Demo · Filename · pHelixBench-Demo.hbx
- Next step
- Inspect the updated task or return to its owning workspace
- Review task
- Open owning workspace

**Authored actions:**

- `128:8028` Action / Close → `[{"type":"BACK"}]`
- `128:8050` Action / Review task → `[{"type":"NODE","target":"107:29211","navigation":"NAVIGATE"}]`
- `128:8052` Action / Open owning workspace → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`

## ST-574 · Flow completion / Operation outcome 107:36891

**Role:** Linked functional state. **Frame:** 128:8054 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-8054).

**Displayed content:**

- PrimerCandidates updated
- Close
- The completed action stays associated with its source task.
- Operation
- Add selected pair
- Source task
- PrimerCandidates
- Reviewed context
- Input DNA-001 / v7 · target 97–351 · no primers saved yet. · Candidate p
- Next step
- Inspect the updated task or return to its owning workspace
- Review task
- Open owning workspace

**Authored actions:**

- `128:8058` Action / Close → `[{"type":"BACK"}]`
- `128:8080` Action / Review task → `[{"type":"NODE","target":"107:36891","navigation":"NAVIGATE"}]`
- `128:8082` Action / Open owning workspace → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`

## ST-575 · Flow completion / Interaction contracts

**Role:** Linked functional state. **Frame:** 128:8092 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-8092).

**Displayed content:**

- Interaction and accessibility contracts
- Implementation handoff · required behavior beyond static frames
- Search
- New
- Selection and record context
- Keep document ID/version, selected range, strand and selected wells through every tool and output.
- Panels and dialogs
- Name the panel; provide close and Escape; define modal versus non-modal behavior.
- Modal: move focus inside, contain its tab order, keep the background inert, and return focus to the invoking control. Non-modal:
allow a deliberate return to the workspace.
- Validation, status and recovery
- Associate errors with fields, retain input, announce status changes, and preserve a safe cancellation path.
- Scientific and persistence checks
- Validate algorithms, units, parsing, provenance, file export, cancellation and duplicate detection in implementation.
- Focus and field validation
- Visible focus and adjacent correction messages are specified for implementation.
- Sequence name
- Expression fragment
- Keyboard focus remains visible until focus moves.
- Sequence input
- ATG?GTACGTAA
- Position 4: replace “?” with a supported base.
- Context & next steps
- These contracts require runtime and
accessibility verification. Native
prototype links alone do not
establish conformance.

**Authored actions:**

- `128:8199` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `128:8200` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`

## ST-576 · Flow completion / Closure review index

**Role:** Linked functional state. **Frame:** 128:8269 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=128-8269).

**Displayed content:**

- Soft Gallery · flow completion
- Review the remaining branch designs and their connected outcomes.
- Search
- New
- Record safety and saved work
- Identity-specific archiving, deletion context and unsaved-change decisions.
- Archive a primer
- After deletion
- Unsaved draft
- Older result
- Create and import
- Successful blank creation, format review and explicit correction choices.
- New sequence
- New notebook
- Import formats
- Partial import
- Duplicate handling
- Guided sample
- Search filters
- Scientific results and secondary tasks
- Explicit input selection, source-version changes and domain-specific recovery.
- Choose protein input
- Rerun version 8
- Compare / export
- Inventory selection
- Transfer review
- Automation recovery
- Partial write recovery
- Interaction contracts
- Open 1280px first-use flow
- Context & next steps
- Use this index to review alternate
branches. Product screens carry
only task-relevant controls.

**Authored actions:**

- `128:8376` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `128:8377` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `128:8436` Action / Archive a primer → `[{"type":"NODE","target":"110:21536","navigation":"NAVIGATE"}]`
- `128:8438` Action / After deletion → `[{"type":"NODE","target":"110:21795","navigation":"NAVIGATE"}]`
- `128:8440` Action / Unsaved draft → `[{"type":"NODE","target":"115:3326","navigation":"NAVIGATE"}]`
- `128:8442` Action / Older result → `[{"type":"NODE","target":"125:10993","navigation":"NAVIGATE"}]`
- `128:8448` Action / New sequence → `[{"type":"NODE","target":"110:20202","navigation":"NAVIGATE"}]`
- `128:8450` Action / New notebook → `[{"type":"NODE","target":"110:20232","navigation":"NAVIGATE"}]`
- `128:8452` Action / Import formats → `[{"type":"NODE","target":"110:19645","navigation":"NAVIGATE"}]`
- `128:8454` Action / Partial import → `[{"type":"NODE","target":"126:5600","navigation":"NAVIGATE"}]`
- `128:8457` Action / Duplicate handling → `[{"type":"NODE","target":"126:5664","navigation":"NAVIGATE"}]`
- `128:8459` Action / Guided sample → `[{"type":"NODE","target":"110:19881","navigation":"NAVIGATE"}]`
- `128:8461` Action / Search filters → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `128:8467` Action / Choose protein input → `[{"type":"NODE","target":"126:7729","navigation":"NAVIGATE"}]`
- `128:8469` Action / Rerun version 8 → `[{"type":"NODE","target":"126:7098","navigation":"NAVIGATE"}]`
- `128:8471` Action / Compare / export → `[{"type":"NODE","target":"126:7480","navigation":"NAVIGATE"}]`
- `128:8474` Action / Inventory selection → `[{"type":"NODE","target":"107:31089","navigation":"NAVIGATE"}]`
- `128:8476` Action / Transfer review → `[{"type":"NODE","target":"128:5866","navigation":"NAVIGATE"}]`
- `128:8478` Action / Automation recovery → `[{"type":"NODE","target":"128:6118","navigation":"NAVIGATE"}]`
- `128:8481` Action / Partial write recovery → `[{"type":"NODE","target":"128:6441","navigation":"NAVIGATE"}]`
- `128:8483` Action / Interaction contracts → `[{"type":"NODE","target":"128:8092","navigation":"NAVIGATE"}]`
- `130:13195` Action / Open 1280px first-use flow → `[{"type":"NODE","target":"130:6799","navigation":"NAVIGATE"}]`

## ST-577 · Flow completion / AllSearch Source

**Role:** Linked functional state. **Frame:** 130:4410 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-4410).

**Displayed content:**

- Map
- Inspect a construct and act on the selected region.
- Search
- New
- Back to search
- pHelixBench-Demo
- v7 · saved 10:42
- Undo
- Save version
- Export
- Sequence
- Features
- Primers
- Enzymes
- History
- Add feature
- Analyze
- Display layers
- Circular map
- Selection 97–116 · forward
- 3021 bp · circular
- GC 54.5%
- drag to select
- T7 promoter
- lac operator
- RBS
- demo_cds
- Demo-Rev
- rrnB T1 terminator
- M13 fwd site
- pUC19 MCS
- M13 rev site
- ori (demo)
- demo_marker
- loxP
- FRT
- Kozak
- EcoRI
- BamHI
- ScaI
- AhdI
- SexAI
- PshAI
- DraI
- NcoI
- T7 promoter selected. Create and analysis tools retain this document and range.
- Readable annotations
- Restriction analysis
- Task help
- Details & tools
- All map tools
- Current selection
- T7 promoter · 97–116
- How to use Map
- Select a feature on the map or use
Features for a readable list. Tools
use the range shown at right.
- Record
- DNA-001 · version 7
- Selected range
- 97–116 · forward strand
- Type
- Promoter · 20 bp
- Actions
- Edit feature
- Add primer from range
- Design primer pair
- Related records
- DNA-001 · Registry
- Plate P-014 · Inventory

**Authored actions:**

- `130:4517` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `130:4518` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `130:4667` Action / Back to search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `130:4524` Action / Save version → `[{"type":"NODE","target":"107:29211","navigation":"OVERLAY"}]`
- `130:4525` Action / Export → `[{"type":"NODE","target":"107:27142","navigation":"OVERLAY"}]`
- `130:4528` Action / Sequence → `[{"type":"NODE","target":"107:29677","navigation":"NAVIGATE"}]`
- `130:4529` Action / Features → `[{"type":"NODE","target":"107:29886","navigation":"NAVIGATE"}]`
- `130:4530` Action / Primers → `[{"type":"NODE","target":"107:30093","navigation":"NAVIGATE"}]`
- `130:4531` Action / Enzymes → `[{"type":"NODE","target":"107:30295","navigation":"NAVIGATE"}]`
- `130:4532` Action / History → `[{"type":"NODE","target":"107:30500","navigation":"NAVIGATE"}]`
- `130:4535` Action / Add feature → `[{"type":"NODE","target":"107:25134","navigation":"OVERLAY"}]`
- `130:4536` Action / Analyze → `[{"type":"NODE","target":"110:20288","navigation":"NAVIGATE"}]`
- `130:4537` Action / Display layers → `[{"type":"NODE","target":"107:25217","navigation":"OVERLAY"}]`
- `130:4616` Action / Readable annotations → `[{"type":"NODE","target":"107:29886","navigation":"NAVIGATE"}]`
- `130:4617` Action / Restriction analysis → `[{"type":"NODE","target":"110:20288","navigation":"NAVIGATE"}]`
- `130:4618` Action / Task help → `[{"type":"NODE","target":"110:21252","navigation":"NAVIGATE"}]`
- `130:4633` Action / All map tools → `[{"type":"NODE","target":"110:22016","navigation":"NAVIGATE"}]`
- `130:4650` Action / Edit feature → `[{"type":"NODE","target":"107:25134","navigation":"OVERLAY"}]`
- `130:4651` Action / Add primer from range → `[{"type":"NODE","target":"107:25182","navigation":"OVERLAY"}]`
- `130:4652` Action / Design primer pair → `[{"type":"NODE","target":"107:25303","navigation":"OVERLAY"}]`
- `130:4655` Related / DNA-001 · Registry → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`
- `130:4660` Related / Plate P-014 · Inventory → `[{"type":"NODE","target":"107:31089","navigation":"NAVIGATE"}]`

## ST-578 · Flow completion / AllSearch Result

**Role:** Linked functional state. **Frame:** 130:4669 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-4669).

**Displayed content:**

- Restriction analysis result
- RES-001 · pHelixBench-Demo · input DNA-001 v7
- Search
- New
- Back to search
- Two predicted fragments
- EcoRI at 216 · BamHI at 234 · circular input length 3021 bp.
- Fragment
- Length
- Interpretation
- Fragment 1
- 18 bp
- Between the two selected cut positions
- Fragment 2
- 3003 bp
- Remaining circular backbone
- Total
- 3021 bp
- Matches input length
- Illustrative result fixture. Verify predictions and experimental assumptions before laboratory use.
- Input and settings stay attached
- DNA-001 · v7 · whole sequence · EcoRI + BamHI · RES-001
- Edit settings
- View source
- Save result
- Read the result
- Fragment lengths describe this
input and enzyme set. The small
fragment may need different
experimental handling.
- Parameter summary
- How to interpret this

**Authored actions:**

- `130:4776` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `130:4777` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `130:4818` Action / Back to search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `130:4807` Action / Edit settings → `[{"type":"NODE","target":"110:20288","navigation":"NAVIGATE"}]`
- `130:4808` Action / View source → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`
- `130:4809` Action / Save result → `[{"type":"NODE","target":"110:20799","navigation":"NAVIGATE"}]`
- `130:4814` Action / Parameter summary → `[{"type":"NODE","target":"110:20486","navigation":"NAVIGATE"}]`
- `130:4815` Action / How to interpret this → `[{"type":"NODE","target":"110:20514","navigation":"NAVIGATE"}]`

## ST-579 · Flow completion / SequenceSearch Source

**Role:** Linked functional state. **Frame:** 130:4820 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-4820).

**Displayed content:**

- Map
- Inspect a construct and act on the selected region.
- Search
- New
- Back to search
- pHelixBench-Demo
- v7 · saved 10:42
- Undo
- Save version
- Export
- Sequence
- Features
- Primers
- Enzymes
- History
- Add feature
- Analyze
- Display layers
- Circular map
- Selection 97–116 · forward
- 3021 bp · circular
- GC 54.5%
- drag to select
- T7 promoter
- lac operator
- RBS
- demo_cds
- Demo-Rev
- rrnB T1 terminator
- M13 fwd site
- pUC19 MCS
- M13 rev site
- ori (demo)
- demo_marker
- loxP
- FRT
- Kozak
- EcoRI
- BamHI
- ScaI
- AhdI
- SexAI
- PshAI
- DraI
- NcoI
- T7 promoter selected. Create and analysis tools retain this document and range.
- Readable annotations
- Restriction analysis
- Task help
- Details & tools
- All map tools
- Current selection
- T7 promoter · 97–116
- How to use Map
- Select a feature on the map or use
Features for a readable list. Tools
use the range shown at right.
- Record
- DNA-001 · version 7
- Selected range
- 97–116 · forward strand
- Type
- Promoter · 20 bp
- Actions
- Edit feature
- Add primer from range
- Design primer pair
- Related records
- DNA-001 · Registry
- Plate P-014 · Inventory

**Authored actions:**

- `130:4927` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `130:4928` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `130:5077` Action / Back to search → `[{"type":"NODE","target":"125:5843","navigation":"NAVIGATE"}]`
- `130:4934` Action / Save version → `[{"type":"NODE","target":"107:29211","navigation":"OVERLAY"}]`
- `130:4935` Action / Export → `[{"type":"NODE","target":"107:27142","navigation":"OVERLAY"}]`
- `130:4938` Action / Sequence → `[{"type":"NODE","target":"107:29677","navigation":"NAVIGATE"}]`
- `130:4939` Action / Features → `[{"type":"NODE","target":"107:29886","navigation":"NAVIGATE"}]`
- `130:4940` Action / Primers → `[{"type":"NODE","target":"107:30093","navigation":"NAVIGATE"}]`
- `130:4941` Action / Enzymes → `[{"type":"NODE","target":"107:30295","navigation":"NAVIGATE"}]`
- `130:4942` Action / History → `[{"type":"NODE","target":"107:30500","navigation":"NAVIGATE"}]`
- `130:4945` Action / Add feature → `[{"type":"NODE","target":"107:25134","navigation":"OVERLAY"}]`
- `130:4946` Action / Analyze → `[{"type":"NODE","target":"110:20288","navigation":"NAVIGATE"}]`
- `130:4947` Action / Display layers → `[{"type":"NODE","target":"107:25217","navigation":"OVERLAY"}]`
- `130:5026` Action / Readable annotations → `[{"type":"NODE","target":"107:29886","navigation":"NAVIGATE"}]`
- `130:5027` Action / Restriction analysis → `[{"type":"NODE","target":"110:20288","navigation":"NAVIGATE"}]`
- `130:5028` Action / Task help → `[{"type":"NODE","target":"110:21252","navigation":"NAVIGATE"}]`
- `130:5043` Action / All map tools → `[{"type":"NODE","target":"110:22016","navigation":"NAVIGATE"}]`
- `130:5060` Action / Edit feature → `[{"type":"NODE","target":"107:25134","navigation":"OVERLAY"}]`
- `130:5061` Action / Add primer from range → `[{"type":"NODE","target":"107:25182","navigation":"OVERLAY"}]`
- `130:5062` Action / Design primer pair → `[{"type":"NODE","target":"107:25303","navigation":"OVERLAY"}]`
- `130:5065` Related / DNA-001 · Registry → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`
- `130:5070` Related / Plate P-014 · Inventory → `[{"type":"NODE","target":"107:31089","navigation":"NAVIGATE"}]`

## ST-580 · Flow completion / ResultSearch Result

**Role:** Linked functional state. **Frame:** 130:5079 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-5079).

**Displayed content:**

- Restriction analysis result
- RES-001 · pHelixBench-Demo · input DNA-001 v7
- Search
- New
- Back to search
- Two predicted fragments
- EcoRI at 216 · BamHI at 234 · circular input length 3021 bp.
- Fragment
- Length
- Interpretation
- Fragment 1
- 18 bp
- Between the two selected cut positions
- Fragment 2
- 3003 bp
- Remaining circular backbone
- Total
- 3021 bp
- Matches input length
- Illustrative result fixture. Verify predictions and experimental assumptions before laboratory use.
- Input and settings stay attached
- DNA-001 · v7 · whole sequence · EcoRI + BamHI · RES-001
- Edit settings
- View source
- Save result
- Read the result
- Fragment lengths describe this
input and enzyme set. The small
fragment may need different
experimental handling.
- Parameter summary
- How to interpret this

**Authored actions:**

- `130:5186` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `130:5187` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `130:5228` Action / Back to search → `[{"type":"NODE","target":"125:6032","navigation":"NAVIGATE"}]`
- `130:5217` Action / Edit settings → `[{"type":"NODE","target":"110:20288","navigation":"NAVIGATE"}]`
- `130:5218` Action / View source → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`
- `130:5219` Action / Save result → `[{"type":"NODE","target":"110:20799","navigation":"NAVIGATE"}]`
- `130:5224` Action / Parameter summary → `[{"type":"NODE","target":"110:20486","navigation":"NAVIGATE"}]`
- `130:5225` Action / How to interpret this → `[{"type":"NODE","target":"110:20514","navigation":"NAVIGATE"}]`

## ST-581 · Flow completion / RecentSearch Source

**Role:** Linked functional state. **Frame:** 130:5230 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-5230).

**Displayed content:**

- Map
- Inspect a construct and act on the selected region.
- Search
- New
- Back to search
- pHelixBench-Demo
- v7 · saved 10:42
- Undo
- Save version
- Export
- Sequence
- Features
- Primers
- Enzymes
- History
- Add feature
- Analyze
- Display layers
- Circular map
- Selection 97–116 · forward
- 3021 bp · circular
- GC 54.5%
- drag to select
- T7 promoter
- lac operator
- RBS
- demo_cds
- Demo-Rev
- rrnB T1 terminator
- M13 fwd site
- pUC19 MCS
- M13 rev site
- ori (demo)
- demo_marker
- loxP
- FRT
- Kozak
- EcoRI
- BamHI
- ScaI
- AhdI
- SexAI
- PshAI
- DraI
- NcoI
- T7 promoter selected. Create and analysis tools retain this document and range.
- Readable annotations
- Restriction analysis
- Task help
- Details & tools
- All map tools
- Current selection
- T7 promoter · 97–116
- How to use Map
- Select a feature on the map or use
Features for a readable list. Tools
use the range shown at right.
- Record
- DNA-001 · version 7
- Selected range
- 97–116 · forward strand
- Type
- Promoter · 20 bp
- Actions
- Edit feature
- Add primer from range
- Design primer pair
- Related records
- DNA-001 · Registry
- Plate P-014 · Inventory

**Authored actions:**

- `130:5337` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `130:5338` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `130:5487` Action / Back to search → `[{"type":"NODE","target":"125:6221","navigation":"NAVIGATE"}]`
- `130:5344` Action / Save version → `[{"type":"NODE","target":"107:29211","navigation":"OVERLAY"}]`
- `130:5345` Action / Export → `[{"type":"NODE","target":"107:27142","navigation":"OVERLAY"}]`
- `130:5348` Action / Sequence → `[{"type":"NODE","target":"107:29677","navigation":"NAVIGATE"}]`
- `130:5349` Action / Features → `[{"type":"NODE","target":"107:29886","navigation":"NAVIGATE"}]`
- `130:5350` Action / Primers → `[{"type":"NODE","target":"107:30093","navigation":"NAVIGATE"}]`
- `130:5351` Action / Enzymes → `[{"type":"NODE","target":"107:30295","navigation":"NAVIGATE"}]`
- `130:5352` Action / History → `[{"type":"NODE","target":"107:30500","navigation":"NAVIGATE"}]`
- `130:5355` Action / Add feature → `[{"type":"NODE","target":"107:25134","navigation":"OVERLAY"}]`
- `130:5356` Action / Analyze → `[{"type":"NODE","target":"110:20288","navigation":"NAVIGATE"}]`
- `130:5357` Action / Display layers → `[{"type":"NODE","target":"107:25217","navigation":"OVERLAY"}]`
- `130:5436` Action / Readable annotations → `[{"type":"NODE","target":"107:29886","navigation":"NAVIGATE"}]`
- `130:5437` Action / Restriction analysis → `[{"type":"NODE","target":"110:20288","navigation":"NAVIGATE"}]`
- `130:5438` Action / Task help → `[{"type":"NODE","target":"110:21252","navigation":"NAVIGATE"}]`
- `130:5453` Action / All map tools → `[{"type":"NODE","target":"110:22016","navigation":"NAVIGATE"}]`
- `130:5470` Action / Edit feature → `[{"type":"NODE","target":"107:25134","navigation":"OVERLAY"}]`
- `130:5471` Action / Add primer from range → `[{"type":"NODE","target":"107:25182","navigation":"OVERLAY"}]`
- `130:5472` Action / Design primer pair → `[{"type":"NODE","target":"107:25303","navigation":"OVERLAY"}]`
- `130:5475` Related / DNA-001 · Registry → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`
- `130:5480` Related / Plate P-014 · Inventory → `[{"type":"NODE","target":"107:31089","navigation":"NAVIGATE"}]`

## ST-582 · Flow completion / RecentSearch Result

**Role:** Linked functional state. **Frame:** 130:5489 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-5489).

**Displayed content:**

- Restriction analysis result
- RES-001 · pHelixBench-Demo · input DNA-001 v7
- Search
- New
- Back to search
- Two predicted fragments
- EcoRI at 216 · BamHI at 234 · circular input length 3021 bp.
- Fragment
- Length
- Interpretation
- Fragment 1
- 18 bp
- Between the two selected cut positions
- Fragment 2
- 3003 bp
- Remaining circular backbone
- Total
- 3021 bp
- Matches input length
- Illustrative result fixture. Verify predictions and experimental assumptions before laboratory use.
- Input and settings stay attached
- DNA-001 · v7 · whole sequence · EcoRI + BamHI · RES-001
- Edit settings
- View source
- Save result
- Read the result
- Fragment lengths describe this
input and enzyme set. The small
fragment may need different
experimental handling.
- Parameter summary
- How to interpret this

**Authored actions:**

- `130:5596` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `130:5597` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `130:5638` Action / Back to search → `[{"type":"NODE","target":"125:6221","navigation":"NAVIGATE"}]`
- `130:5627` Action / Edit settings → `[{"type":"NODE","target":"110:20288","navigation":"NAVIGATE"}]`
- `130:5628` Action / View source → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`
- `130:5629` Action / Save result → `[{"type":"NODE","target":"110:20799","navigation":"NAVIGATE"}]`
- `130:5634` Action / Parameter summary → `[{"type":"NODE","target":"110:20486","navigation":"NAVIGATE"}]`
- `130:5635` Action / How to interpret this → `[{"type":"NODE","target":"110:20514","navigation":"NAVIGATE"}]`

## ST-583 · Flow completion / Saved v8 Sequence

**Role:** Linked functional state. **Frame:** 130:5640 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-5640).

**Displayed content:**

- Sequence
- Inspect bases, annotations and the active selection.
- Search
- New
- pHelixBench-Demo
- v8 · saved just now
- Undo
- Save version
- Export
- Map
- Features
- Primers
- Enzymes
- History
- Find in sequence
- Edit selection
- Analyze
- DNA sequence · forward strand
- 1                                                 60
- ACAGTGGTTCACAGACAGATACAGACTTTCGGTTCAAGACTCTGCTGACTGC
- 61                                                 120
- GCTTATAGTTCACCTACGATCGGTGACATCGGCTAGCTAGCTAGGCTAACGT
- 121                                                 180
- TACGGTACCGGAATTCGGATCCATGTCTAAAGGTGAAGAATTATTCACTGGT
- 181                                                 240
- GTTGTCCCAATTCTTGTTGAATTAGATGGTGATGTTAATGGTCACAAATTT
- Selected 97–116 · 20 nt. Changes are validated before the saved sequence is updated.
- Annotation
- Coordinates
- Strand
- T7 promoter
- 97–116
- Forward
- lac operator
- 119–143
- demo_cds
- 180–897
- Details & tools
- All sequence tools
- Current selection
- Bases 97–116
- How to use Sequence
- Coordinates are 1-based and inclusive.
Switch views without losing your
selected region.
- Document
- DNA-001 · 3021 bp
- Selection
- 20 nt · forward strand
- Translation
- Frame +1 · standard code
- Actions
- Add feature
- BLAST selection
- Related records
- Primer pair P-01

**Authored actions:**

- `130:5747` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `130:5748` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `130:5754` Action / Save version → `[{"type":"NODE","target":"107:29211","navigation":"OVERLAY"}]`
- `130:5755` Action / Export → `[{"type":"NODE","target":"107:27142","navigation":"OVERLAY"}]`
- `130:5757` Action / Map → `[{"type":"NODE","target":"125:8508","navigation":"NAVIGATE"}]`
- `130:5759` Action / Features → `[{"type":"NODE","target":"130:5852","navigation":"NAVIGATE"}]`
- `130:5760` Action / Primers → `[{"type":"NODE","target":"130:6062","navigation":"NAVIGATE"}]`
- `130:5761` Action / Enzymes → `[{"type":"NODE","target":"130:6267","navigation":"NAVIGATE"}]`
- `130:5762` Action / History → `[{"type":"NODE","target":"130:6475","navigation":"NAVIGATE"}]`
- `130:5765` Action / Find in sequence → `[{"type":"NODE","target":"107:33290","navigation":"OVERLAY"}]`
- `130:5766` Action / Edit selection → `[{"type":"NODE","target":"107:29125","navigation":"OVERLAY"}]`
- `130:5767` Action / Analyze → `[{"type":"NODE","target":"126:7098","navigation":"NAVIGATE"}]`
- `130:5820` Action / All sequence tools → `[{"type":"NODE","target":"110:22016","navigation":"NAVIGATE"}]`
- `130:5837` Action / Edit selection → `[{"type":"NODE","target":"107:29125","navigation":"OVERLAY"}]`
- `130:5838` Action / Add feature → `[{"type":"NODE","target":"107:25134","navigation":"OVERLAY"}]`
- `130:5839` Action / BLAST selection → `[{"type":"NODE","target":"107:25595","navigation":"OVERLAY"}]`
- `130:5842` Related / T7 promoter → `[{"type":"NODE","target":"107:29886","navigation":"NAVIGATE"}]`
- `130:5847` Related / Primer pair P-01 → `[{"type":"NODE","target":"107:30093","navigation":"NAVIGATE"}]`

## ST-584 · Flow completion / Saved v8 Features

**Role:** Linked functional state. **Frame:** 130:5852 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-5852).

**Displayed content:**

- Features
- Review annotations before applying changes.
- Search
- New
- pHelixBench-Demo
- v8 · saved just now
- Undo
- Save version
- Export
- Map
- Sequence
- Primers
- Enzymes
- History
- Add feature
- Detect common
- Review auto-annotation
- Name
- Type
- Range
- Strand
- T7 promoter
- Promoter
- 97–116
- Forward
- lac operator
- Operator
- 119–143
- demo_cds
- CDS
- 180–897
- rrnB T1 terminator
- Terminator
- 940–1025
- AmpR
- 1700–2558
- Reverse
- Review proposed additions and conflicts before changing the annotation set.
- Details & tools
- All features tools
- Current selection
- How to use Features
- Features describe annotated regions.
Add your own annotation or review
suggested annotations before applying.
- 97–116 · forward
- Source
- Manual annotation
- Actions
- Edit selected
- Auto-annotation preview
- Related records
- Expression validation

**Authored actions:**

- `130:5959` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `130:5960` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `130:5966` Action / Save version → `[{"type":"NODE","target":"107:29211","navigation":"OVERLAY"}]`
- `130:5967` Action / Export → `[{"type":"NODE","target":"107:27142","navigation":"OVERLAY"}]`
- `130:5969` Action / Map → `[{"type":"NODE","target":"125:8508","navigation":"NAVIGATE"}]`
- `130:5970` Action / Sequence → `[{"type":"NODE","target":"130:5640","navigation":"NAVIGATE"}]`
- `130:5972` Action / Primers → `[{"type":"NODE","target":"130:6062","navigation":"NAVIGATE"}]`
- `130:5973` Action / Enzymes → `[{"type":"NODE","target":"130:6267","navigation":"NAVIGATE"}]`
- `130:5974` Action / History → `[{"type":"NODE","target":"130:6475","navigation":"NAVIGATE"}]`
- `130:5977` Action / Add feature → `[{"type":"NODE","target":"107:25134","navigation":"OVERLAY"}]`
- `130:5978` Action / Detect common → `[{"type":"NODE","target":"107:26060","navigation":"OVERLAY"}]`
- `130:5979` Action / Review auto-annotation → `[{"type":"NODE","target":"107:33313","navigation":"OVERLAY"}]`
- `130:5986` Record / T7 promoter → `[{"type":"NODE","target":"107:25134","navigation":"OVERLAY"}]`
- `130:5991` Record / lac operator → `[{"type":"NODE","target":"107:25134","navigation":"OVERLAY"}]`
- `130:5996` Record / demo_cds → `[{"type":"NODE","target":"107:25134","navigation":"OVERLAY"}]`
- `130:6001` Record / rrnB T1 terminator → `[{"type":"NODE","target":"107:25134","navigation":"OVERLAY"}]`
- `130:6006` Record / AmpR → `[{"type":"NODE","target":"107:25134","navigation":"OVERLAY"}]`
- `130:6030` Action / All features tools → `[{"type":"NODE","target":"107:28097","navigation":"OVERLAY"}]`
- `130:6047` Action / Edit selected → `[{"type":"NODE","target":"107:25134","navigation":"OVERLAY"}]`
- `130:6048` Action / Detect common → `[{"type":"NODE","target":"107:26060","navigation":"OVERLAY"}]`
- `130:6049` Action / Auto-annotation preview → `[{"type":"NODE","target":"107:33313","navigation":"OVERLAY"}]`
- `130:6052` Related / pHelixBench-Demo → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`
- `130:6057` Related / Expression validation → `[{"type":"NODE","target":"107:30702","navigation":"NAVIGATE"}]`

## ST-585 · Flow completion / Saved v8 Primers

**Role:** Linked functional state. **Frame:** 130:6062 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-6062).

**Displayed content:**

- Primers
- Review candidate pairs and verify the predicted product.
- Search
- New
- pHelixBench-Demo
- v8 · saved just now
- Undo
- Save version
- Export
- Map
- Sequence
- Features
- Enzymes
- History
- Design pair
- Add primer
- Simulate PCR
- Primer / pair
- Position
- Tm
- State
- P-01 · T7-F / Demo-R
- 97–351
- 61.2 / 62.0 °C
- Linked pair
- T7-F
- 97–116
- 61.2 °C
- Forward
- Demo-R
- 330–351
- 62.0 °C
- Reverse
- P-02 · screening
- 180–598
- 60.8 / 61.4 °C
- Review needed
- P-01 is linked to DNA-001. Predicted product: 255 bp; inspect simulation before saving.
- Details & tools
- All primers tools
- Current selection
- Pair P-01
- How to use Primers
- A primer is linked to an input
sequence. Verify range, strand and
pair before simulating PCR.
- T7-F · 20 nt
- Demo-R · 22 nt
- Predicted product
- 255 bp · ready to simulate
- Actions
- Edit primer
- Related records
- DNA-001 template
- PCR product history

**Authored actions:**

- `130:6169` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `130:6170` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `130:6176` Action / Save version → `[{"type":"NODE","target":"107:29211","navigation":"OVERLAY"}]`
- `130:6177` Action / Export → `[{"type":"NODE","target":"107:27142","navigation":"OVERLAY"}]`
- `130:6179` Action / Map → `[{"type":"NODE","target":"125:8508","navigation":"NAVIGATE"}]`
- `130:6180` Action / Sequence → `[{"type":"NODE","target":"130:5640","navigation":"NAVIGATE"}]`
- `130:6181` Action / Features → `[{"type":"NODE","target":"130:5852","navigation":"NAVIGATE"}]`
- `130:6183` Action / Enzymes → `[{"type":"NODE","target":"130:6267","navigation":"NAVIGATE"}]`
- `130:6184` Action / History → `[{"type":"NODE","target":"130:6475","navigation":"NAVIGATE"}]`
- `130:6187` Action / Design pair → `[{"type":"NODE","target":"107:25303","navigation":"OVERLAY"}]`
- `130:6188` Action / Add primer → `[{"type":"NODE","target":"107:25182","navigation":"OVERLAY"}]`
- `130:6189` Action / Simulate PCR → `[{"type":"NODE","target":"107:25266","navigation":"OVERLAY"}]`
- `130:6196` Record / P-01 · T7-F / Demo-R → `[{"type":"NODE","target":"107:25182","navigation":"OVERLAY"}]`
- `130:6201` Record / T7-F → `[{"type":"NODE","target":"107:25182","navigation":"OVERLAY"}]`
- `130:6206` Record / Demo-R → `[{"type":"NODE","target":"107:25182","navigation":"OVERLAY"}]`
- `130:6211` Record / P-02 · screening → `[{"type":"NODE","target":"107:25182","navigation":"OVERLAY"}]`
- `130:6235` Action / All primers tools → `[{"type":"NODE","target":"107:28166","navigation":"OVERLAY"}]`
- `130:6252` Action / Design pair → `[{"type":"NODE","target":"107:25303","navigation":"OVERLAY"}]`
- `130:6253` Action / Edit primer → `[{"type":"NODE","target":"107:25182","navigation":"OVERLAY"}]`
- `130:6254` Action / Simulate PCR → `[{"type":"NODE","target":"107:25266","navigation":"OVERLAY"}]`
- `130:6257` Related / DNA-001 template → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`
- `130:6262` Related / PCR product history → `[{"type":"NODE","target":"107:33352","navigation":"OVERLAY"}]`

## ST-586 · Flow completion / Saved v8 Enzymes

**Role:** Linked functional state. **Frame:** 130:6267 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-6267).

**Displayed content:**

- Enzymes
- Choose cutters and inspect predicted fragments.
- Search
- New
- pHelixBench-Demo
- v8 · saved just now
- Undo
- Save version
- Export
- Map
- Sequence
- Features
- Primers
- History
- Enzyme set
- Filter cutters
- Digest & gel
- Enzyme
- Sites
- Cut position
- Selection
- EcoRI
- 1
- 216
- Selected
- BamHI
- 234
- HindIII
- 260
- Available
- NotI
- 0
- No sites
- Filtered out
- Digest setup
- Two selected enzymes · circular DNA · input version 8
- Expected fragments and gel preview are generated after parameter validation.
- Details & tools
- All enzymes tools
- Current selection
- EcoRI + BamHI
- How to use Enzymes
- Restriction analysis predicts cut
sites and fragments. Confirm your
input and enzyme set before running.
- Active set
- Common cloning enzymes
- Cut sites
- 2 selected · 4 total
- Document
- DNA-001 · circular
- Actions
- Choose enzyme set
- Related records
- Input DNA-001
- Digest result

**Authored actions:**

- `130:6374` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `130:6375` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `130:6381` Action / Save version → `[{"type":"NODE","target":"107:29211","navigation":"OVERLAY"}]`
- `130:6382` Action / Export → `[{"type":"NODE","target":"107:27142","navigation":"OVERLAY"}]`
- `130:6384` Action / Map → `[{"type":"NODE","target":"125:8508","navigation":"NAVIGATE"}]`
- `130:6385` Action / Sequence → `[{"type":"NODE","target":"130:5640","navigation":"NAVIGATE"}]`
- `130:6386` Action / Features → `[{"type":"NODE","target":"130:5852","navigation":"NAVIGATE"}]`
- `130:6387` Action / Primers → `[{"type":"NODE","target":"130:6062","navigation":"NAVIGATE"}]`
- `130:6389` Action / History → `[{"type":"NODE","target":"130:6475","navigation":"NAVIGATE"}]`
- `130:6392` Action / Enzyme set → `[{"type":"NODE","target":"107:25217","navigation":"OVERLAY"}]`
- `130:6393` Action / Filter cutters → `[{"type":"NODE","target":"107:26978","navigation":"OVERLAY"}]`
- `130:6394` Action / Digest & gel → `[{"type":"NODE","target":"110:20288","navigation":"NAVIGATE"}]`
- `130:6401` Record / EcoRI → `[{"type":"NODE","target":"110:20288","navigation":"NAVIGATE"}]`
- `130:6406` Record / BamHI → `[{"type":"NODE","target":"110:20288","navigation":"NAVIGATE"}]`
- `130:6411` Record / HindIII → `[{"type":"NODE","target":"110:20288","navigation":"NAVIGATE"}]`
- `130:6416` Record / NotI → `[{"type":"NODE","target":"110:20288","navigation":"NAVIGATE"}]`
- `130:6443` Action / All enzymes tools → `[{"type":"NODE","target":"107:28231","navigation":"OVERLAY"}]`
- `130:6460` Action / Choose enzyme set → `[{"type":"NODE","target":"107:25217","navigation":"OVERLAY"}]`
- `130:6461` Action / Digest & gel → `[{"type":"NODE","target":"110:20288","navigation":"NAVIGATE"}]`
- `130:6462` Action / Filter cutters → `[{"type":"NODE","target":"107:26978","navigation":"OVERLAY"}]`
- `130:6465` Related / Input DNA-001 → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`
- `130:6470` Related / Digest result → `[{"type":"NODE","target":"107:33375","navigation":"OVERLAY"}]`

## ST-587 · Flow completion / Saved v8 History

**Role:** Linked functional state. **Frame:** 130:6475 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-6475).

**Displayed content:**

- History
- Compare changes before restoring a saved version.
- Search
- New
- pHelixBench-Demo
- v8 · saved just now
- Undo
- Save version
- Export
- Map
- Sequence
- Features
- Primers
- Enzymes
- Saved checkpoints
- Version 8 is current. Previewing an older version does not restore it.
- Version
- State
- Change
- v8
- Current
- Sequence draft saved
- v7
- Previous
- Prior saved checkpoint
- Current sequence
- Inspect version-7 result
- Details & tools
- All history tools
- Current selection
- Version 7 · current
- How to use History
- Versions are saved checkpoints. Undo
reverses an unsaved edit; restoring a
version requires an impact review.
- Saved by
- You · today at 10:42
- Changes
- Feature annotation updated
- Record
- DNA-001
- Actions
- Compare versions
- Duplicate version
- Restore preview
- Related records
- Record activity

**Authored actions:**

- `130:6582` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `130:6583` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `130:6589` Action / Save version → `[{"type":"NODE","target":"107:29211","navigation":"OVERLAY"}]`
- `130:6590` Action / Export → `[{"type":"NODE","target":"107:27142","navigation":"OVERLAY"}]`
- `130:6592` Action / Map → `[{"type":"NODE","target":"125:8508","navigation":"NAVIGATE"}]`
- `130:6593` Action / Sequence → `[{"type":"NODE","target":"130:5640","navigation":"NAVIGATE"}]`
- `130:6594` Action / Features → `[{"type":"NODE","target":"130:5852","navigation":"NAVIGATE"}]`
- `130:6595` Action / Primers → `[{"type":"NODE","target":"130:6062","navigation":"NAVIGATE"}]`
- `130:6596` Action / Enzymes → `[{"type":"NODE","target":"130:6267","navigation":"NAVIGATE"}]`
- `130:6710` Action / Current sequence → `[{"type":"NODE","target":"125:8508","navigation":"NAVIGATE"}]`
- `130:6712` Action / Inspect version-7 result → `[{"type":"NODE","target":"125:10993","navigation":"NAVIGATE"}]`
- `130:6648` Action / All history tools → `[{"type":"NODE","target":"107:28296","navigation":"OVERLAY"}]`
- `130:6665` Action / Compare versions → `[{"type":"NODE","target":"107:33405","navigation":"OVERLAY"}]`
- `130:6666` Action / Duplicate version → `[{"type":"NODE","target":"107:29025","navigation":"OVERLAY"}]`
- `130:6667` Action / Restore preview → `[{"type":"NODE","target":"107:33433","navigation":"OVERLAY"}]`
- `130:6670` Related / Current sequence → `[{"type":"NODE","target":"107:29677","navigation":"NAVIGATE"}]`
- `130:6675` Related / Record activity → `[{"type":"NODE","target":"107:33461","navigation":"OVERLAY"}]`

## ST-588 · Flow completion / 1280 / Empty / Home

**Role:** Responsive/context variant. **Frame:** 130:6799 · 1280 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-6799).

**Displayed content:**

- Home
- Search
- Details & tools
- Start with your sequence
- Open your file, understand the input, and run your first analysis.
- Import a sequence
- Open GenBank, FASTA or a HelixBench document. Review the recognized sequence before opening it.
- Import sequence
- Try a sample
- No sequence ready? A sample is labeled and kept separate from your own files.
- A short path to your first result
- Review input → inspect sequence → run analysis → save and find it again.
- Create a blank sequence
- New notebook entry
- You can explore the product without creating inventory records or setting up a notebook.

**Authored actions:**

- `130:6900` Action / Search → `[{"type":"NODE","target":"130:8241","navigation":"NAVIGATE"}]`
- `130:6943` Action / Details & tools → `[{"type":"NODE","target":"130:6936","navigation":"OVERLAY"}]`
- `130:6909` Action / Import sequence → `[{"type":"NODE","target":"130:7123","navigation":"NAVIGATE"}]`
- `130:6910` Action / Try a sample → `[{"type":"NODE","target":"110:19881","navigation":"NAVIGATE"}]`
- `130:6916` Action / Create a blank sequence → `[{"type":"NODE","target":"110:20202","navigation":"NAVIGATE"}]`
- `130:6917` Action / New notebook entry → `[{"type":"NODE","target":"110:20232","navigation":"NAVIGATE"}]`

## ST-589 · Flow completion / 1280 panel / Empty / Home

**Role:** Responsive/context variant. **Frame:** 130:6936 · 320 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-6936).

**Displayed content:**

- Your first task
- Import your own sequence, or
explore a labeled sample. You can
skip guidance and return to it
later.
- Import sequence
- Try a sample
- Task help
- Close panel

**Authored actions:**

- `130:6940` Action / Import sequence → `[{"type":"NODE","target":"130:7123","navigation":"NAVIGATE"}]`
- `130:6941` Action / Try a sample → `[{"type":"NODE","target":"110:19881","navigation":"NAVIGATE"}]`
- `130:6942` Action / Task help → `[{"type":"NODE","target":"130:8895","navigation":"NAVIGATE"}]`
- `130:6946` Action / Close panel → `[{"type":"CLOSE"}]`

## ST-590 · Flow completion / 1280 / UX v4 / Home

**Role:** Responsive/context variant. **Frame:** 130:6949 · 1280 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-6949).

**Displayed content:**

- Home
- Continue your work and resolve what needs attention.
- Search
- New
- Details & tools
- Start with your sequence
- Open your file, understand the input, and run your first analysis.
- Import a sequence
- Open GenBank, FASTA or a HelixBench document. Review the recognized sequence before opening it.
- Import sequence
- Try a sample
- No sequence ready? A sample is labeled and kept separate from your own files.
- A short path to your first result
- Review input → inspect sequence → run analysis → save and find it again.
- Create a blank sequence
- New notebook entry
- Continue working
- Open the exact item with its context preserved.
- Work item
- Type
- Next action
- pHelixBench-Demo · DNA-001
- Sequence · v7
- Open sequence
- Restriction analysis · RES-001
- Saved result · DNA-001 v7
- View result
- Expression validation · EXP-014
- Notebook draft
- Continue entry

**Authored actions:**

- `130:7056` Action / Search → `[{"type":"NODE","target":"130:8241","navigation":"NAVIGATE"}]`
- `130:7057` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `130:7117` Action / Details & tools → `[{"type":"NODE","target":"130:7110","navigation":"OVERLAY"}]`
- `130:7066` Action / Import sequence → `[{"type":"NODE","target":"130:7123","navigation":"NAVIGATE"}]`
- `130:7067` Action / Try a sample → `[{"type":"NODE","target":"110:19881","navigation":"NAVIGATE"}]`
- `130:7073` Action / Create a blank sequence → `[{"type":"NODE","target":"110:20202","navigation":"NAVIGATE"}]`
- `130:7074` Action / New notebook entry → `[{"type":"NODE","target":"110:20232","navigation":"NAVIGATE"}]`
- `130:7083` Record / pHelixBench-Demo · DNA-001 → `[{"type":"NODE","target":"130:7316","navigation":"NAVIGATE"}]`
- `130:7087` Record / Restriction analysis · RES-001 → `[{"type":"NODE","target":"130:7902","navigation":"NAVIGATE"}]`
- `130:7091` Record / Expression validation · EXP-014 → `[{"type":"NODE","target":"107:30702","navigation":"NAVIGATE"}]`

## ST-591 · Flow completion / 1280 panel / UX v4 / Home

**Role:** Responsive/context variant. **Frame:** 130:7110 · 320 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-7110).

**Displayed content:**

- Your first task
- Import your own sequence, or
explore a labeled sample. You can
skip guidance and return to it
later.
- Import sequence
- Try a sample
- Task help
- Close panel

**Authored actions:**

- `130:7114` Action / Import sequence → `[{"type":"NODE","target":"130:7123","navigation":"NAVIGATE"}]`
- `130:7115` Action / Try a sample → `[{"type":"NODE","target":"110:19881","navigation":"NAVIGATE"}]`
- `130:7116` Action / Task help → `[{"type":"NODE","target":"130:8895","navigation":"NAVIGATE"}]`
- `130:7120` Action / Close panel → `[{"type":"CLOSE"}]`

## ST-592 · Flow completion / 1280 / UX v4 flow / ImportSource

**Role:** Responsive/context variant. **Frame:** 130:7123 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-7123).

**Displayed content:**

- Open a sequence
- Close
- Choose a local file or paste a sequence. Review the recognized content before opening.
- Choose an import source
- Select the file format or paste input to review.
- HelixBench document
- GenBank file
- FASTA file
- Multiple records
- Paste sequence
- Cancel

**Authored actions:**

- `130:7127` Action / Close → `[{"type":"BACK"}]`
- `130:7134` Action / HelixBench document → `[{"type":"NODE","target":"130:7145","navigation":"NAVIGATE"}]`
- `130:7135` Action / GenBank file → `[{"type":"NODE","target":"130:9056","navigation":"NAVIGATE"}]`
- `130:7137` Action / FASTA file → `[{"type":"NODE","target":"130:9463","navigation":"NAVIGATE"}]`
- `130:7138` Action / Multiple records → `[{"type":"NODE","target":"130:9870","navigation":"NAVIGATE"}]`
- `130:7140` Action / Paste sequence → `[{"type":"NODE","target":"110:20202","navigation":"NAVIGATE"}]`
- `130:7144` Action / Cancel → `[{"type":"NODE","target":"130:6949","navigation":"NAVIGATE"}]`

## ST-593 · Flow completion / 1280 / ImportReview

**Role:** Responsive/context variant. **Frame:** 130:7145 · 1280 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-7145).

**Displayed content:**

- Review sequence import
- Confirm what was recognized before opening your document.
- Search
- New
- Details & tools
- pHelixBench-Demo.hbx
- HelixBench document · 1 sequence · includes saved history.
- Recognized property
- Value
- Action
- Name / stable ID
- pHelixBench-Demo · DNA-001
- Retained from document
- Type / length
- DNA · 3021 bp
- Confirm recognized type
- Topology
- Circular
- Annotations
- Features and primer links
- Preserved
- Document version
- v7 · saved checkpoint
- History retained
- Sequence changes
- None
- Original bases retained
- Ready to open. No sequence symbols will be removed or changed.
- Back to source
- Open this sequence

**Authored actions:**

- `130:7252` Action / Search → `[{"type":"NODE","target":"130:8241","navigation":"NAVIGATE"}]`
- `130:7253` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `130:7310` Action / Details & tools → `[{"type":"NODE","target":"130:7305","navigation":"OVERLAY"}]`
- `130:7292` Action / Back to source → `[{"type":"NODE","target":"130:7123","navigation":"NAVIGATE"}]`
- `130:7293` Action / Open this sequence → `[{"type":"NODE","target":"130:7316","navigation":"NAVIGATE"}]`

## ST-594 · Flow completion / 1280 panel / ImportReview

**Role:** Responsive/context variant. **Frame:** 130:7305 · 320 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-7305).

**Displayed content:**

- What happens next?
- Inspect the sequence, then run a
restriction analysis. Registration
and inventory setup are optional.
- Import help
- Close panel

**Authored actions:**

- `130:7309` Action / Import help → `[{"type":"NODE","target":"130:8895","navigation":"NAVIGATE"}]`
- `130:7313` Action / Close panel → `[{"type":"CLOSE"}]`

## ST-595 · Flow completion / 1280 / UX v4 / Map

**Role:** Responsive/context variant. **Frame:** 130:7316 · 1280 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-7316).

**Displayed content:**

- Map
- Inspect a construct and act on the selected region.
- Search
- New
- Details & tools
- pHelixBench-Demo
- v7 · saved 10:42
- Undo
- Save version
- Export
- Sequence
- Features
- Primers
- Enzymes
- History
- Add feature
- Analyze
- Display layers
- Circular map
- Selection 97–116 · forward
- 3021 bp · circular
- GC 54.5%
- drag to select
- T7 promoter
- lac operator
- RBS
- demo_cds
- Demo-Rev
- rrnB T1 terminator
- M13 fwd site
- pUC19 MCS
- M13 rev site
- ori (demo)
- demo_marker
- loxP
- FRT
- Kozak
- EcoRI
- BamHI
- ScaI
- AhdI
- SexAI
- PshAI
- DraI
- NcoI
- T7 promoter selected. Create and analysis tools retain this document and range.
- Readable annotations
- Restriction analysis
- Task help

**Authored actions:**

- `130:7423` Action / Search → `[{"type":"NODE","target":"130:8241","navigation":"NAVIGATE"}]`
- `130:7424` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `130:7636` Action / Details & tools → `[{"type":"NODE","target":"130:7590","navigation":"OVERLAY"}]`
- `130:7430` Action / Save version → `[{"type":"NODE","target":"107:29211","navigation":"OVERLAY"}]`
- `130:7431` Action / Export → `[{"type":"NODE","target":"107:27142","navigation":"OVERLAY"}]`
- `130:7434` Action / Sequence → `[{"type":"NODE","target":"107:29677","navigation":"NAVIGATE"}]`
- `130:7435` Action / Features → `[{"type":"NODE","target":"107:29886","navigation":"NAVIGATE"}]`
- `130:7436` Action / Primers → `[{"type":"NODE","target":"107:30093","navigation":"NAVIGATE"}]`
- `130:7437` Action / Enzymes → `[{"type":"NODE","target":"107:30295","navigation":"NAVIGATE"}]`
- `130:7438` Action / History → `[{"type":"NODE","target":"107:30500","navigation":"NAVIGATE"}]`
- `130:7441` Action / Add feature → `[{"type":"NODE","target":"107:25134","navigation":"OVERLAY"}]`
- `130:7442` Action / Analyze → `[{"type":"NODE","target":"130:7642","navigation":"NAVIGATE"}]`
- `130:7443` Action / Display layers → `[{"type":"NODE","target":"107:25217","navigation":"OVERLAY"}]`
- `130:7522` Action / Readable annotations → `[{"type":"NODE","target":"107:29886","navigation":"NAVIGATE"}]`
- `130:7523` Action / Restriction analysis → `[{"type":"NODE","target":"130:7642","navigation":"NAVIGATE"}]`
- `130:7524` Action / Task help → `[{"type":"NODE","target":"130:8895","navigation":"NAVIGATE"}]`

## ST-596 · Flow completion / 1280 panel / UX v4 / Map

**Role:** Responsive/context variant. **Frame:** 130:7590 · 320 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-7590).

**Displayed content:**

- Details & tools
- All map tools
- Current selection
- T7 promoter · 97–116
- How to use Map
- Select a feature on the map or use
Features for a readable list. Tools
use the range shown at right.
- Record
- DNA-001 · version 7
- Selected range
- 97–116 · forward strand
- Type
- Promoter · 20 bp
- Actions
- Edit feature
- Add primer from range
- Design primer pair
- Related records
- DNA-001 · Registry
- Plate P-014 · Inventory
- Close panel

**Authored actions:**

- `130:7604` Action / All map tools → `[{"type":"NODE","target":"110:22016","navigation":"NAVIGATE"}]`
- `130:7621` Action / Edit feature → `[{"type":"NODE","target":"107:25134","navigation":"OVERLAY"}]`
- `130:7622` Action / Add primer from range → `[{"type":"NODE","target":"107:25182","navigation":"OVERLAY"}]`
- `130:7623` Action / Design primer pair → `[{"type":"NODE","target":"107:25303","navigation":"OVERLAY"}]`
- `130:7626` Related / DNA-001 · Registry → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`
- `130:7631` Related / Plate P-014 · Inventory → `[{"type":"NODE","target":"107:31089","navigation":"NAVIGATE"}]`
- `130:7639` Action / Close panel → `[{"type":"CLOSE"}]`

## ST-597 · Flow completion / 1280 / AnalysisSetup

**Role:** Responsive/context variant. **Frame:** 130:7642 · 1280 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-7642).

**Displayed content:**

- Restriction analysis
- Confirm your input and review recommended settings.
- Search
- New
- Details & tools
- Input: DNA-001 · pHelixBench-Demo
- Version 7 · 3021 bp · circular · full sequence, not only the selected promoter.
- Recommended setup
- Enzyme selection
- EcoRI + BamHI
- Analysis scope
- Whole circular sequence · 3021 bp
- Both enzymes have one predicted cut site in this example input. Settings are suggestions; review assumptions before
interpretation.
- Advanced settings
- Why these settings?
- The result will retain input version, enzymes, scope and parameters.
- Back to sequence
- Run restriction analysis

**Authored actions:**

- `130:7749` Action / Search → `[{"type":"NODE","target":"130:8241","navigation":"NAVIGATE"}]`
- `130:7750` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `130:7797` Action / Details & tools → `[{"type":"NODE","target":"130:7791","navigation":"OVERLAY"}]`
- `130:7768` Action / Advanced settings → `[{"type":"NODE","target":"130:7803","navigation":"NAVIGATE"}]`
- `130:7769` Action / Why these settings? → `[{"type":"NODE","target":"130:7829","navigation":"NAVIGATE"}]`
- `130:7775` Action / Back to sequence → `[{"type":"NODE","target":"130:7316","navigation":"NAVIGATE"}]`
- `130:7776` Action / Run restriction analysis → `[{"type":"NODE","target":"130:7851","navigation":"NAVIGATE"}]`

## ST-598 · Flow completion / 1280 panel / AnalysisSetup

**Role:** Responsive/context variant. **Frame:** 130:7791 · 320 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-7791).

**Displayed content:**

- Input & prerequisites
- Restriction analysis needs a valid
DNA sequence and selected enzymes.
Your entire input is in scope; no
selected region is required.
- Review source sequence
- Choose another enzyme set
- Close panel

**Authored actions:**

- `130:7795` Action / Review source sequence → `[{"type":"NODE","target":"130:7316","navigation":"NAVIGATE"}]`
- `130:7796` Action / Choose another enzyme set → `[{"type":"NODE","target":"107:26978","navigation":"NAVIGATE"}]`
- `130:7800` Action / Close panel → `[{"type":"CLOSE"}]`

## ST-599 · Flow completion / 1280 / UX v4 flow / AnalysisAdvanced

**Role:** Responsive/context variant. **Frame:** 130:7803 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-7803).

**Displayed content:**

- Advanced restriction settings
- Close
- Review scientific assumptions. These are illustrative prototype parameters.
- Input
- DNA-001 · v7 · whole circular sequence
- Enzymes
- EcoRI and BamHI
- Methylation / ambiguity
- Review enzyme sensitivity and ambiguous-base handling
- Output
- Predicted fragments and gel preview
- Back to setup

**Authored actions:**

- `130:7807` Action / Close → `[{"type":"BACK"}]`
- `130:7828` Action / Back to setup → `[{"type":"NODE","target":"130:7642","navigation":"NAVIGATE"}]`

## ST-600 · Flow completion / 1280 / UX v4 flow / AnalysisHelp

**Role:** Responsive/context variant. **Frame:** 130:7829 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-7829).

**Displayed content:**

- Understanding restriction analysis
- Close
- A prediction describes expected cut positions and fragment lengths for the specified input and
enzyme set.
- Scope
- Whole sequence includes every base, regardless of current map selection
- Interpretation
- Check fragment lengths sum to input length for this circular example
- Limitations
- An in-silico result does not guarantee an experimental outcome
- Back to setup

**Authored actions:**

- `130:7833` Action / Close → `[{"type":"BACK"}]`
- `130:7850` Action / Back to setup → `[{"type":"NODE","target":"130:7642","navigation":"NAVIGATE"}]`

## ST-601 · Flow completion / 1280 / UX v4 flow / AnalysisRunning

**Role:** Responsive/context variant. **Frame:** 130:7851 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-7851).

**Displayed content:**

- Restriction analysis is running
- Close
- Keep the input and operation identity visible while work is in progress.
- Operation
- OP-001 · DNA-001 v7
- Stage
- Calculating fragments · progress indeterminate
- Input
- EcoRI + BamHI · whole sequence
- Cancellation
- Cancel stops this operation before result creation
- Cancel analysis
- Preview completed state
- Preview failed state

**Authored actions:**

- `130:7855` Action / Close → `[{"type":"BACK"}]`
- `130:7876` Action / Cancel analysis → `[{"type":"NODE","target":"130:7642","navigation":"NAVIGATE"}]`
- `130:7877` Action / Preview completed state → `[{"type":"NODE","target":"130:7902","navigation":"NAVIGATE"}]`
- `130:7878` Action / Preview failed state → `[{"type":"NODE","target":"130:7879","navigation":"NAVIGATE"}]`

## ST-602 · Flow completion / 1280 / UX v4 flow / AnalysisFailed

**Role:** Responsive/context variant. **Frame:** 130:7879 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-7879).

**Displayed content:**

- Analysis could not finish
- Close
- No result was created. Your input and settings are retained.
- Operation
- OP-001 · DNA-001 v7
- Problem
- The analysis service did not respond
- Recovery
- Check connection, then retry with the same input and settings
- Return to setup
- Retry analysis

**Authored actions:**

- `130:7883` Action / Close → `[{"type":"BACK"}]`
- `130:7900` Action / Return to setup → `[{"type":"NODE","target":"130:7642","navigation":"NAVIGATE"}]`
- `130:7901` Action / Retry analysis → `[{"type":"NODE","target":"130:7851","navigation":"NAVIGATE"}]`

## ST-603 · Flow completion / 1280 / AnalysisResult

**Role:** Responsive/context variant. **Frame:** 130:7902 · 1280 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-7902).

**Displayed content:**

- Restriction analysis result
- RES-001 · pHelixBench-Demo · input DNA-001 v7
- Search
- New
- Details & tools
- Two predicted fragments
- EcoRI at 216 · BamHI at 234 · circular input length 3021 bp.
- Fragment
- Length
- Interpretation
- Fragment 1
- 18 bp
- Between the two selected cut positions
- Fragment 2
- 3003 bp
- Remaining circular backbone
- Total
- 3021 bp
- Matches input length
- Illustrative result fixture. Verify predictions and experimental assumptions before laboratory use.
- Input and settings stay attached
- DNA-001 · v7 · whole sequence · EcoRI + BamHI · RES-001
- Edit settings
- View source
- Save result

**Authored actions:**

- `130:8009` Action / Search → `[{"type":"NODE","target":"130:8241","navigation":"NAVIGATE"}]`
- `130:8010` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `130:8062` Action / Details & tools → `[{"type":"NODE","target":"130:8056","navigation":"OVERLAY"}]`
- `130:8040` Action / Edit settings → `[{"type":"NODE","target":"130:7642","navigation":"NAVIGATE"}]`
- `130:8041` Action / View source → `[{"type":"NODE","target":"130:7316","navigation":"NAVIGATE"}]`
- `130:8042` Action / Save result → `[{"type":"NODE","target":"130:8068","navigation":"NAVIGATE"}]`

## ST-604 · Flow completion / 1280 panel / AnalysisResult

**Role:** Responsive/context variant. **Frame:** 130:8056 · 320 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-8056).

**Displayed content:**

- Read the result
- Fragment lengths describe this
input and enzyme set. The small
fragment may need different
experimental handling.
- Parameter summary
- How to interpret this
- Close panel

**Authored actions:**

- `130:8060` Action / Parameter summary → `[{"type":"NODE","target":"130:7803","navigation":"NAVIGATE"}]`
- `130:8061` Action / How to interpret this → `[{"type":"NODE","target":"130:7829","navigation":"NAVIGATE"}]`
- `130:8065` Action / Close panel → `[{"type":"CLOSE"}]`

## ST-605 · Flow completion / 1280 / UX v4 flow / SaveResult

**Role:** Responsive/context variant. **Frame:** 130:8068 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-8068).

**Displayed content:**

- Save this analysis result
- Close
- Keep a named output linked to its source.
- Result name
- Restriction analysis · EcoRI + BamHI
- Result ID
- RES-001
- Source
- DNA-001 · v7 · whole sequence
- Location
- Local workspace / Analyses
- Cancel
- Save named result

**Authored actions:**

- `130:8072` Action / Close → `[{"type":"BACK"}]`
- `130:8093` Action / Cancel → `[{"type":"NODE","target":"130:7902","navigation":"NAVIGATE"}]`
- `130:8094` Action / Save named result → `[{"type":"NODE","target":"130:8095","navigation":"NAVIGATE"}]`

## ST-606 · Flow completion / 1280 / SavedResult

**Role:** Responsive/context variant. **Frame:** 130:8095 · 1280 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-8095).

**Displayed content:**

- Result saved
- RES-001 · Local workspace / Analyses
- Search
- New
- Details & tools
- Restriction analysis · EcoRI + BamHI was saved with input DNA-001 v7.
- Find it again
- Use Home recent work, workspace search, or the source sequence’s related results.
- View result
- Go to Home
- Find by ID

**Authored actions:**

- `130:8202` Action / Search → `[{"type":"NODE","target":"130:8241","navigation":"NAVIGATE"}]`
- `130:8203` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `130:8235` Action / Details & tools → `[{"type":"NODE","target":"130:8229","navigation":"OVERLAY"}]`
- `130:8213` Action / View result → `[{"type":"NODE","target":"130:7902","navigation":"NAVIGATE"}]`
- `130:8214` Action / Go to Home → `[{"type":"NODE","target":"130:6949","navigation":"NAVIGATE"}]`
- `130:8215` Action / Find by ID → `[{"type":"NODE","target":"130:8241","navigation":"NAVIGATE"}]`

## ST-607 · Flow completion / 1280 panel / SavedResult

**Role:** Responsive/context variant. **Frame:** 130:8229 · 320 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-8229).

**Displayed content:**

- What was saved?
- Result identity, source version,
scope and settings are retained
together.
- Open source
- Open parameter summary
- Close panel

**Authored actions:**

- `130:8233` Action / Open source → `[{"type":"NODE","target":"130:7316","navigation":"NAVIGATE"}]`
- `130:8234` Action / Open parameter summary → `[{"type":"NODE","target":"130:7803","navigation":"NAVIGATE"}]`
- `130:8238` Action / Close panel → `[{"type":"CLOSE"}]`

## ST-608 · Flow completion / 1280 / WorkspaceSearch

**Role:** Responsive/context variant. **Frame:** 130:8241 · 1280 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-8241).

**Displayed content:**

- Search your workspace
- Search by name or stable ID. Results retain their record type and context.
- Search
- New
- Details & tools
- Query
- DNA-001
- All records
- Sequences
- Results
- Record
- Type / scope
- Open
- pHelixBench-Demo · DNA-001
- Sequence · v7
- Sequence workspace
- Restriction analysis · RES-001
- Analysis · input DNA-001 v7
- Saved result
- Back to Home
- Clear search

**Authored actions:**

- `130:8349` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `130:8395` Action / Details & tools → `[{"type":"NODE","target":"130:8390","navigation":"OVERLAY"}]`
- `130:8357` Action / Sequences → `[{"type":"NODE","target":"130:8401","navigation":"NAVIGATE"}]`
- `130:8358` Action / Results → `[{"type":"NODE","target":"130:8552","navigation":"NAVIGATE"}]`
- `130:8364` Record / pHelixBench-Demo · DNA-001 → `[{"type":"NODE","target":"130:10861","navigation":"NAVIGATE"}]`
- `130:8368` Record / Restriction analysis · RES-001 → `[{"type":"NODE","target":"130:11189","navigation":"NAVIGATE"}]`
- `130:8373` Action / Back to Home → `[{"type":"NODE","target":"130:6949","navigation":"NAVIGATE"}]`
- `130:8375` Action / Clear search → `[{"type":"NODE","target":"130:8703","navigation":"NAVIGATE"}]`

## ST-609 · Flow completion / 1280 panel / WorkspaceSearch

**Role:** Responsive/context variant. **Frame:** 130:8390 · 320 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-8390).

**Displayed content:**

- Recognize the right record
- Check type, ID and source before
opening a result. Your query is
retained when you return.
- Search help
- Close panel

**Authored actions:**

- `130:8394` Action / Search help → `[{"type":"NODE","target":"130:8895","navigation":"NAVIGATE"}]`
- `130:8398` Action / Close panel → `[{"type":"CLOSE"}]`

## ST-610 · Flow completion / 1280 / Flow completion / SearchSequences

**Role:** Responsive/context variant. **Frame:** 130:8401 · 1280 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-8401).

**Displayed content:**

- Sequence search
- Query: DNA-001 · scope: Sequences
- Search
- New
- Details & tools
- Search query
- DNA-001
- All records
- Sequences
- Results
- Clear search
- Record
- Type / context
- Open
- pHelixBench-Demo · DNA-001
- Sequence · v7
- Open source
- Back to Home

**Authored actions:**

- `130:8508` Action / Search → `[{"type":"NODE","target":"130:8241","navigation":"NAVIGATE"}]`
- `130:8509` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `130:8546` Action / Details & tools → `[{"type":"NODE","target":"130:8543","navigation":"OVERLAY"}]`
- `130:8516` Action / All records → `[{"type":"NODE","target":"130:8241","navigation":"NAVIGATE"}]`
- `130:8518` Action / Results → `[{"type":"NODE","target":"130:8552","navigation":"NAVIGATE"}]`
- `130:8519` Action / Clear search → `[{"type":"NODE","target":"130:8703","navigation":"NAVIGATE"}]`
- `130:8525` Record / pHelixBench-Demo · DNA-001 → `[{"type":"NODE","target":"130:11357","navigation":"NAVIGATE"}]`
- `130:8530` Action / Back to Home → `[{"type":"NODE","target":"130:6949","navigation":"NAVIGATE"}]`

## ST-611 · Flow completion / 1280 panel / Flow completion / SearchSequences

**Role:** Responsive/context variant. **Frame:** 130:8543 · 320 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-8543).

**Displayed content:**

- Context & next steps
- Search scope and query remain
visible. Return to the same result
list after inspecting an item.
- Close panel

**Authored actions:**

- `130:8549` Action / Close panel → `[{"type":"CLOSE"}]`

## ST-612 · Flow completion / 1280 / Flow completion / SearchResults

**Role:** Responsive/context variant. **Frame:** 130:8552 · 1280 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-8552).

**Displayed content:**

- Analysis search
- Query: DNA-001 · scope: Results
- Search
- New
- Details & tools
- Search query
- DNA-001
- All records
- Sequences
- Results
- Clear search
- Record
- Type / context
- Open
- Restriction analysis · RES-001
- Analysis · DNA-001 v7
- Open result
- Back to Home

**Authored actions:**

- `130:8659` Action / Search → `[{"type":"NODE","target":"130:8241","navigation":"NAVIGATE"}]`
- `130:8660` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `130:8697` Action / Details & tools → `[{"type":"NODE","target":"130:8694","navigation":"OVERLAY"}]`
- `130:8667` Action / All records → `[{"type":"NODE","target":"130:8241","navigation":"NAVIGATE"}]`
- `130:8668` Action / Sequences → `[{"type":"NODE","target":"130:8401","navigation":"NAVIGATE"}]`
- `130:8670` Action / Clear search → `[{"type":"NODE","target":"130:8703","navigation":"NAVIGATE"}]`
- `130:8676` Record / Restriction analysis · RES-001 → `[{"type":"NODE","target":"130:11685","navigation":"NAVIGATE"}]`
- `130:8681` Action / Back to Home → `[{"type":"NODE","target":"130:6949","navigation":"NAVIGATE"}]`

## ST-613 · Flow completion / 1280 panel / Flow completion / SearchResults

**Role:** Responsive/context variant. **Frame:** 130:8694 · 320 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-8694).

**Displayed content:**

- Context & next steps
- Search scope and query remain
visible. Return to the same result
list after inspecting an item.
- Close panel

**Authored actions:**

- `130:8700` Action / Close panel → `[{"type":"CLOSE"}]`

## ST-614 · Flow completion / 1280 / Flow completion / SearchRecent

**Role:** Responsive/context variant. **Frame:** 130:8703 · 1280 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-8703).

**Displayed content:**

- Search your workspace
- Query cleared · recent work
- Search
- New
- Details & tools
- Search query
- Search by name or stable ID
- All records
- Sequences
- Results
- Clear search
- Record
- Type / context
- Open
- pHelixBench-Demo · DNA-001
- Recent sequence
- Open source
- Restriction analysis · RES-001
- Recent result
- Open result
- Back to Home

**Authored actions:**

- `130:8810` Action / Search → `[{"type":"NODE","target":"130:8241","navigation":"NAVIGATE"}]`
- `130:8811` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `130:8852` Action / Details & tools → `[{"type":"NODE","target":"130:8849","navigation":"OVERLAY"}]`
- `130:8818` Action / All records → `[{"type":"NODE","target":"130:8241","navigation":"NAVIGATE"}]`
- `130:8819` Action / Sequences → `[{"type":"NODE","target":"130:8401","navigation":"NAVIGATE"}]`
- `130:8820` Action / Results → `[{"type":"NODE","target":"130:8552","navigation":"NAVIGATE"}]`
- `130:8827` Record / pHelixBench-Demo · DNA-001 → `[{"type":"NODE","target":"130:11853","navigation":"NAVIGATE"}]`
- `130:8831` Record / Restriction analysis · RES-001 → `[{"type":"NODE","target":"130:12181","navigation":"NAVIGATE"}]`
- `130:8836` Action / Back to Home → `[{"type":"NODE","target":"130:6949","navigation":"NAVIGATE"}]`

## ST-615 · Flow completion / 1280 panel / Flow completion / SearchRecent

**Role:** Responsive/context variant. **Frame:** 130:8849 · 320 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-8849).

**Displayed content:**

- Context & next steps
- Search scope and query remain
visible. Return to the same result
list after inspecting an item.
- Close panel

**Authored actions:**

- `130:8855` Action / Close panel → `[{"type":"CLOSE"}]`

## ST-616 · Flow completion / 1280 / UX v4 flow / NoMatches

**Role:** Responsive/context variant. **Frame:** 130:8858 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-8858).

**Displayed content:**

- No matching records
- Close
- Search completed. No accessible records match this query.
- Query
- DNA-999
- Try next
- Check the ID or broaden the search scope
- Edit search

**Authored actions:**

- `130:8862` Action / Close → `[{"type":"BACK"}]`
- `130:8875` Action / Edit search → `[{"type":"NODE","target":"130:8241","navigation":"NAVIGATE"}]`

## ST-617 · Flow completion / 1280 / UX v4 flow / SearchFailed

**Role:** Responsive/context variant. **Frame:** 130:8876 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-8876).

**Displayed content:**

- Search could not load
- Close
- This is a connection problem, not a no-results response.
- Query
- DNA-001 · retained
- Recovery
- Retry, or return to your current work
- Return to search
- Retry search

**Authored actions:**

- `130:8880` Action / Close → `[{"type":"BACK"}]`
- `130:8893` Action / Return to search → `[{"type":"NODE","target":"130:8241","navigation":"NAVIGATE"}]`
- `130:8894` Action / Retry search → `[{"type":"NODE","target":"130:8241","navigation":"NAVIGATE"}]`

## ST-618 · Flow completion / 1280 / TaskHelp

**Role:** Responsive/context variant. **Frame:** 130:8895 · 1280 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-8895).

**Displayed content:**

- Help for your current task
- Short explanations, examples and deeper guidance without losing your work.
- Search
- New
- Details & tools
- From a sequence to a result
- Import → review recognized content → inspect → analyze → save → reopen.
- Task
- What you need
- Open
- Open a file
- Supported sequence or HelixBench document
- Import help
- Run restriction analysis
- DNA input and enzyme set
- Analysis setup
- Find saved work
- Name or stable record ID
- Workspace search
- Understand product areas
- Sequence, Registry, Inventory and Notebook
- Workspace guide
- Return to Home
- Return to sequence

**Authored actions:**

- `130:9002` Action / Search → `[{"type":"NODE","target":"130:8241","navigation":"NAVIGATE"}]`
- `130:9003` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `130:9050` Action / Details & tools → `[{"type":"NODE","target":"130:9044","navigation":"OVERLAY"}]`
- `130:9013` Record / Open a file → `[{"type":"NODE","target":"130:7123","navigation":"NAVIGATE"}]`
- `130:9017` Record / Run restriction analysis → `[{"type":"NODE","target":"130:7642","navigation":"NAVIGATE"}]`
- `130:9021` Record / Find saved work → `[{"type":"NODE","target":"130:8241","navigation":"NAVIGATE"}]`
- `130:9025` Record / Understand product areas → `[{"type":"NODE","target":"110:21450","navigation":"NAVIGATE"}]`
- `130:9030` Action / Return to Home → `[{"type":"NODE","target":"130:6949","navigation":"NAVIGATE"}]`
- `130:9031` Action / Return to sequence → `[{"type":"NODE","target":"130:7316","navigation":"NAVIGATE"}]`

## ST-619 · Flow completion / 1280 panel / TaskHelp

**Role:** Responsive/context variant. **Frame:** 130:9044 · 320 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-9044).

**Displayed content:**

- Get help where you work
- Import and analysis setup have
their own help. Closing help
returns to the task; it never
reports a save.
- Import help
- Analysis explanation
- Close panel

**Authored actions:**

- `130:9048` Action / Import help → `[{"type":"NODE","target":"130:7123","navigation":"NAVIGATE"}]`
- `130:9049` Action / Analysis explanation → `[{"type":"NODE","target":"130:7829","navigation":"NAVIGATE"}]`
- `130:9053` Action / Close panel → `[{"type":"CLOSE"}]`

## ST-620 · Flow completion / 1280 / Flow completion / GenBankReview

**Role:** Responsive/context variant. **Frame:** 130:9056 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-9056).

**Displayed content:**

- Review GenBank import
- Close
- A sequence and its annotation were recognized.
- File
- expression_fragment.gb
- New identity
- DNA-GB-001 · version 1
- Sequence
- DNA · 12 nt · linear
- Annotations
- CDS 1–12 forward · retained
- Changes
- None · original bases retained
- Back to source
- Open sequence

**Authored actions:**

- `130:9060` Action / Close → `[{"type":"BACK"}]`
- `130:9085` Action / Back to source → `[{"type":"NODE","target":"130:7123","navigation":"NAVIGATE"}]`
- `130:9086` Action / Open sequence → `[{"type":"NODE","target":"130:9087","navigation":"NAVIGATE"}]`

## ST-621 · Flow completion / 1280 / Flow completion / GenBankDocument

**Role:** Responsive/context variant. **Frame:** 130:9087 · 1280 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-9087).

**Displayed content:**

- Expression fragment
- DNA-GB-001 · version 1 · DNA · 12 nt · linear
- Search
- New
- Details & tools
- Sequence and provenance
- expression_fragment.gb
- 1                        12
- ATGCGTACGTAA
- Property
- Value
- Stable identity
- DNA-GB-001 · version 1
- Annotations
- CDS · 1–12 · forward
- Original source
- Restriction analysis
- Save document

**Authored actions:**

- `130:9194` Action / Search → `[{"type":"NODE","target":"130:8241","navigation":"NAVIGATE"}]`
- `130:9195` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `130:9231` Action / Details & tools → `[{"type":"NODE","target":"130:9228","navigation":"OVERLAY"}]`
- `130:9217` Action / Restriction analysis → `[{"type":"NODE","target":"130:9237","navigation":"NAVIGATE"}]`
- `130:9218` Action / Save document → `[{"type":"NODE","target":"130:9288","navigation":"NAVIGATE"}]`

## ST-622 · Flow completion / 1280 panel / Flow completion / GenBankDocument

**Role:** Responsive/context variant. **Frame:** 130:9228 · 320 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-9228).

**Displayed content:**

- Context & next steps
- The displayed identity is retained
through setup, result and saved
document.
- Close panel

**Authored actions:**

- `130:9234` Action / Close panel → `[{"type":"CLOSE"}]`

## ST-623 · Flow completion / 1280 / Flow completion / GenBankDocument Setup

**Role:** Responsive/context variant. **Frame:** 130:9237 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-9237).

**Displayed content:**

- Restriction analysis · DNA-GB-001
- Close
- Confirm the complete input before running.
- Input
- DNA-GB-001 · version 1
- Scope
- Whole linear sequence · 12 nt
- Enzymes
- EcoRI and BamHI
- Back to sequence
- Run analysis

**Authored actions:**

- `130:9241` Action / Close → `[{"type":"BACK"}]`
- `130:9258` Action / Back to sequence → `[{"type":"NODE","target":"130:9087","navigation":"NAVIGATE"}]`
- `130:9259` Action / Run analysis → `[{"type":"NODE","target":"130:9260","navigation":"NAVIGATE"}]`

## ST-624 · Flow completion / 1280 / Flow completion / GenBankDocument Result

**Role:** Responsive/context variant. **Frame:** 130:9260 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-9260).

**Displayed content:**

- No cut sites found
- Close
- Analysis completed for the selected input.
- Input
- DNA-GB-001 · version 1
- Enzymes
- EcoRI and BamHI
- Predicted output
- One uncut linear fragment · 12 nt
- Result state
- Completed · not a connection or validation error
- Change settings
- View source
- Save document

**Authored actions:**

- `130:9264` Action / Close → `[{"type":"BACK"}]`
- `130:9285` Action / Change settings → `[{"type":"NODE","target":"130:9237","navigation":"NAVIGATE"}]`
- `130:9286` Action / View source → `[{"type":"NODE","target":"130:9087","navigation":"NAVIGATE"}]`
- `130:9287` Action / Save document → `[{"type":"NODE","target":"130:9288","navigation":"NAVIGATE"}]`

## ST-625 · Flow completion / 1280 / Flow completion / GenBankDocument Save

**Role:** Responsive/context variant. **Frame:** 130:9288 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-9288).

**Displayed content:**

- Save Expression fragment
- Close
- Choose a location without changing the source identity.
- Document
- DNA-GB-001 · version 1
- Location
- Local workspace / Sequences
- Annotations
- CDS · 1–12 · forward
- Cancel
- Save document

**Authored actions:**

- `130:9292` Action / Close → `[{"type":"BACK"}]`
- `130:9309` Action / Cancel → `[{"type":"NODE","target":"130:9087","navigation":"NAVIGATE"}]`
- `130:9310` Action / Save document → `[{"type":"NODE","target":"130:9311","navigation":"NAVIGATE"}]`

## ST-626 · Flow completion / 1280 / Flow completion / GenBankDocument Saved

**Role:** Responsive/context variant. **Frame:** 130:9311 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-9311).

**Displayed content:**

- Expression fragment saved
- Close
- DNA-GB-001 · Local workspace / Sequences
- Input version
- 1
- Retained source
- expression_fragment.gb
- Open sequence
- Find saved document

**Authored actions:**

- `130:9315` Action / Close → `[{"type":"BACK"}]`
- `130:9328` Action / Open sequence → `[{"type":"NODE","target":"130:9087","navigation":"NAVIGATE"}]`
- `130:9329` Action / Find saved document → `[{"type":"NODE","target":"130:9330","navigation":"NAVIGATE"}]`

## ST-627 · Flow completion / 1280 / Flow completion / GenBankDocument Home

**Role:** Responsive/context variant. **Frame:** 130:9330 · 1280 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-9330).

**Displayed content:**

- Continue your work
- Your newly saved document is available by name and ID.
- Search
- New
- Details & tools
- Document
- Identity
- Location
- Expression fragment
- DNA-GB-001 · v1
- Local workspace / Sequences

**Authored actions:**

- `130:9437` Action / Search → `[{"type":"NODE","target":"130:8241","navigation":"NAVIGATE"}]`
- `130:9438` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `130:9457` Action / Details & tools → `[{"type":"NODE","target":"130:9455","navigation":"OVERLAY"}]`
- `130:9445` Record / Expression fragment → `[{"type":"NODE","target":"130:9087","navigation":"NAVIGATE"}]`

## ST-628 · Flow completion / 1280 panel / Flow completion / GenBankDocument Home

**Role:** Responsive/context variant. **Frame:** 130:9455 · 320 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-9455).

**Displayed content:**

- Context & next steps
- Close panel

**Authored actions:**

- `130:9460` Action / Close panel → `[{"type":"CLOSE"}]`

## ST-629 · Flow completion / 1280 / Flow completion / FastaReview

**Role:** Responsive/context variant. **Frame:** 130:9463 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-9463).

**Displayed content:**

- Review FASTA import
- Close
- FASTA does not supply all document metadata. Confirm the interpretation.
- File
- expression_fragment.fasta
- Header
- Expression fragment FASTA
- New identity
- DNA-FA-001 · version 1
- Interpretation
- DNA · linear · 12 nt
- Annotation warning
- No annotations supplied; none will be invented
- Back to source
- Confirm DNA and open

**Authored actions:**

- `130:9467` Action / Close → `[{"type":"BACK"}]`
- `130:9492` Action / Back to source → `[{"type":"NODE","target":"130:7123","navigation":"NAVIGATE"}]`
- `130:9493` Action / Confirm DNA and open → `[{"type":"NODE","target":"130:9494","navigation":"NAVIGATE"}]`

## ST-630 · Flow completion / 1280 / Flow completion / FastaDocument

**Role:** Responsive/context variant. **Frame:** 130:9494 · 1280 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-9494).

**Displayed content:**

- Expression fragment FASTA
- DNA-FA-001 · version 1 · DNA · 12 nt · linear
- Search
- New
- Details & tools
- Sequence and provenance
- expression_fragment.fasta
- 1                        12
- ATGCGTACGTAA
- Property
- Value
- Stable identity
- DNA-FA-001 · version 1
- Annotations
- No annotations in source FASTA
- Original source
- Restriction analysis
- Save document

**Authored actions:**

- `130:9601` Action / Search → `[{"type":"NODE","target":"130:8241","navigation":"NAVIGATE"}]`
- `130:9602` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `130:9638` Action / Details & tools → `[{"type":"NODE","target":"130:9635","navigation":"OVERLAY"}]`
- `130:9624` Action / Restriction analysis → `[{"type":"NODE","target":"130:9644","navigation":"NAVIGATE"}]`
- `130:9625` Action / Save document → `[{"type":"NODE","target":"130:9695","navigation":"NAVIGATE"}]`

## ST-631 · Flow completion / 1280 panel / Flow completion / FastaDocument

**Role:** Responsive/context variant. **Frame:** 130:9635 · 320 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-9635).

**Displayed content:**

- Context & next steps
- The displayed identity is retained
through setup, result and saved
document.
- Close panel

**Authored actions:**

- `130:9641` Action / Close panel → `[{"type":"CLOSE"}]`

## ST-632 · Flow completion / 1280 / Flow completion / FastaDocument Setup

**Role:** Responsive/context variant. **Frame:** 130:9644 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-9644).

**Displayed content:**

- Restriction analysis · DNA-FA-001
- Close
- Confirm the complete input before running.
- Input
- DNA-FA-001 · version 1
- Scope
- Whole linear sequence · 12 nt
- Enzymes
- EcoRI and BamHI
- Back to sequence
- Run analysis

**Authored actions:**

- `130:9648` Action / Close → `[{"type":"BACK"}]`
- `130:9665` Action / Back to sequence → `[{"type":"NODE","target":"130:9494","navigation":"NAVIGATE"}]`
- `130:9666` Action / Run analysis → `[{"type":"NODE","target":"130:9667","navigation":"NAVIGATE"}]`

## ST-633 · Flow completion / 1280 / Flow completion / FastaDocument Result

**Role:** Responsive/context variant. **Frame:** 130:9667 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-9667).

**Displayed content:**

- No cut sites found
- Close
- Analysis completed for the selected input.
- Input
- DNA-FA-001 · version 1
- Enzymes
- EcoRI and BamHI
- Predicted output
- One uncut linear fragment · 12 nt
- Result state
- Completed · not a connection or validation error
- Change settings
- View source
- Save document

**Authored actions:**

- `130:9671` Action / Close → `[{"type":"BACK"}]`
- `130:9692` Action / Change settings → `[{"type":"NODE","target":"130:9644","navigation":"NAVIGATE"}]`
- `130:9693` Action / View source → `[{"type":"NODE","target":"130:9494","navigation":"NAVIGATE"}]`
- `130:9694` Action / Save document → `[{"type":"NODE","target":"130:9695","navigation":"NAVIGATE"}]`

## ST-634 · Flow completion / 1280 / Flow completion / FastaDocument Save

**Role:** Responsive/context variant. **Frame:** 130:9695 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-9695).

**Displayed content:**

- Save Expression fragment FASTA
- Close
- Choose a location without changing the source identity.
- Document
- DNA-FA-001 · version 1
- Location
- Local workspace / Sequences
- Annotations
- No annotations in source FASTA
- Cancel
- Save document

**Authored actions:**

- `130:9699` Action / Close → `[{"type":"BACK"}]`
- `130:9716` Action / Cancel → `[{"type":"NODE","target":"130:9494","navigation":"NAVIGATE"}]`
- `130:9717` Action / Save document → `[{"type":"NODE","target":"130:9718","navigation":"NAVIGATE"}]`

## ST-635 · Flow completion / 1280 / Flow completion / FastaDocument Saved

**Role:** Responsive/context variant. **Frame:** 130:9718 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-9718).

**Displayed content:**

- Expression fragment FASTA saved
- Close
- DNA-FA-001 · Local workspace / Sequences
- Input version
- 1
- Retained source
- expression_fragment.fasta
- Open sequence
- Find saved document

**Authored actions:**

- `130:9722` Action / Close → `[{"type":"BACK"}]`
- `130:9735` Action / Open sequence → `[{"type":"NODE","target":"130:9494","navigation":"NAVIGATE"}]`
- `130:9736` Action / Find saved document → `[{"type":"NODE","target":"130:9737","navigation":"NAVIGATE"}]`

## ST-636 · Flow completion / 1280 / Flow completion / FastaDocument Home

**Role:** Responsive/context variant. **Frame:** 130:9737 · 1280 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-9737).

**Displayed content:**

- Continue your work
- Your newly saved document is available by name and ID.
- Search
- New
- Details & tools
- Document
- Identity
- Location
- Expression fragment FASTA
- DNA-FA-001 · v1
- Local workspace / Sequences

**Authored actions:**

- `130:9844` Action / Search → `[{"type":"NODE","target":"130:8241","navigation":"NAVIGATE"}]`
- `130:9845` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `130:9864` Action / Details & tools → `[{"type":"NODE","target":"130:9862","navigation":"OVERLAY"}]`
- `130:9852` Record / Expression fragment FASTA → `[{"type":"NODE","target":"130:9494","navigation":"NAVIGATE"}]`

## ST-637 · Flow completion / 1280 panel / Flow completion / FastaDocument Home

**Role:** Responsive/context variant. **Frame:** 130:9862 · 320 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-9862).

**Displayed content:**

- Context & next steps
- Close panel

**Authored actions:**

- `130:9867` Action / Close panel → `[{"type":"CLOSE"}]`

## ST-638 · Flow completion / 1280 / Flow completion / BulkReview

**Role:** Responsive/context variant. **Frame:** 130:9870 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-9870).

**Displayed content:**

- Review two FASTA records
- Close
- Choose which records to open.
- Record 1
- Expression fragment · DNA-FA-001 · valid · 12 nt
- Record 2
- Control fragment · DNA-FA-002 · valid · 12 nt
- Selection
- Both records selected
- Next step
- Each record receives its own document identity
- Import first only
- Open both documents
- Review partial import

**Authored actions:**

- `130:9874` Action / Close → `[{"type":"BACK"}]`
- `130:9895` Action / Import first only → `[{"type":"NODE","target":"130:9494","navigation":"NAVIGATE"}]`
- `130:9896` Action / Open both documents → `[{"type":"NODE","target":"130:9898","navigation":"NAVIGATE"}]`
- `130:9897` Action / Review partial import → `[{"type":"NODE","target":"130:10035","navigation":"NAVIGATE"}]`

## ST-639 · Flow completion / 1280 / Flow completion / ImportedDocuments

**Role:** Responsive/context variant. **Frame:** 130:9898 · 1280 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-9898).

**Displayed content:**

- Two documents imported
- Each imported sequence has its own stable ID.
- Search
- New
- Details & tools
- Document
- Identity
- Input
- Expression fragment FASTA
- DNA-FA-001 · v1
- 12 nt · linear
- Control fragment
- DNA-FA-002 · v1

**Authored actions:**

- `130:10005` Action / Search → `[{"type":"NODE","target":"130:8241","navigation":"NAVIGATE"}]`
- `130:10006` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `130:10029` Action / Details & tools → `[{"type":"NODE","target":"130:10027","navigation":"OVERLAY"}]`
- `130:10013` Record / Expression fragment FASTA → `[{"type":"NODE","target":"130:9494","navigation":"NAVIGATE"}]`
- `130:10017` Record / Control fragment → `[{"type":"NODE","target":"130:10093","navigation":"NAVIGATE"}]`

## ST-640 · Flow completion / 1280 panel / Flow completion / ImportedDocuments

**Role:** Responsive/context variant. **Frame:** 130:10027 · 320 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-10027).

**Displayed content:**

- Context & next steps
- Close panel

**Authored actions:**

- `130:10032` Action / Close panel → `[{"type":"CLOSE"}]`

## ST-641 · Flow completion / 1280 / Flow completion / PartialImport

**Role:** Responsive/context variant. **Frame:** 130:10035 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-10035).

**Displayed content:**

- One record needs correction
- Close
- Valid input and rejected input are kept separate.
- Skipping a record is an explicit decision.
- Record 1
- Expression fragment · valid · 12 nt
- Record 2
- Control fragment · unsupported “?” at position 4
- Preservation
- Original rejected record retained; no symbols silently removed
- Choice
- Correct the source or deliberately import only the valid record
- Correct rejected record
- Import valid record only

**Authored actions:**

- `130:10039` Action / Close → `[{"type":"BACK"}]`
- `130:10064` Action / Correct rejected record → `[{"type":"NODE","target":"130:10066","navigation":"NAVIGATE"}]`
- `130:10065` Action / Import valid record only → `[{"type":"NODE","target":"130:9494","navigation":"NAVIGATE"}]`

## ST-642 · Flow completion / 1280 / Flow completion / ImportCorrection

**Role:** Responsive/context variant. **Frame:** 130:10066 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-10066).

**Displayed content:**

- Correct the rejected sequence
- Close
- The original value remains available for comparison.
- Original
- ATG?GTACGTAA
- Issue
- Position 4: unsupported “?”
- Corrected value
- ATGCGTACGTAA
- Review
- One explicit replacement · length remains 12 nt
- Keep original
- Review corrected records

**Authored actions:**

- `130:10070` Action / Close → `[{"type":"BACK"}]`
- `130:10091` Action / Keep original → `[{"type":"NODE","target":"130:10035","navigation":"NAVIGATE"}]`
- `130:10092` Action / Review corrected records → `[{"type":"NODE","target":"130:9870","navigation":"NAVIGATE"}]`

## ST-643 · Flow completion / 1280 / Flow completion / SecondFastaDocument

**Role:** Responsive/context variant. **Frame:** 130:10093 · 1280 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-10093).

**Displayed content:**

- Control fragment
- DNA-FA-002 · version 1 · DNA · 12 nt · linear
- Search
- New
- Details & tools
- Sequence and provenance
- control_fragment.fasta
- 1                        12
- ATGCGTACGTAA
- Property
- Value
- Stable identity
- DNA-FA-002 · version 1
- Annotations
- No annotations in source FASTA
- Original source
- Restriction analysis
- Save document

**Authored actions:**

- `130:10200` Action / Search → `[{"type":"NODE","target":"130:8241","navigation":"NAVIGATE"}]`
- `130:10201` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `130:10237` Action / Details & tools → `[{"type":"NODE","target":"130:10234","navigation":"OVERLAY"}]`
- `130:10223` Action / Restriction analysis → `[{"type":"NODE","target":"130:10243","navigation":"NAVIGATE"}]`
- `130:10224` Action / Save document → `[{"type":"NODE","target":"130:10294","navigation":"NAVIGATE"}]`

## ST-644 · Flow completion / 1280 panel / Flow completion / SecondFastaDocument

**Role:** Responsive/context variant. **Frame:** 130:10234 · 320 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-10234).

**Displayed content:**

- Context & next steps
- The displayed identity is retained
through setup, result and saved
document.
- Close panel

**Authored actions:**

- `130:10240` Action / Close panel → `[{"type":"CLOSE"}]`

## ST-645 · Flow completion / 1280 / Flow completion / SecondFastaDocument Setup

**Role:** Responsive/context variant. **Frame:** 130:10243 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-10243).

**Displayed content:**

- Restriction analysis · DNA-FA-002
- Close
- Confirm the complete input before running.
- Input
- DNA-FA-002 · version 1
- Scope
- Whole linear sequence · 12 nt
- Enzymes
- EcoRI and BamHI
- Back to sequence
- Run analysis

**Authored actions:**

- `130:10247` Action / Close → `[{"type":"BACK"}]`
- `130:10264` Action / Back to sequence → `[{"type":"NODE","target":"130:10093","navigation":"NAVIGATE"}]`
- `130:10265` Action / Run analysis → `[{"type":"NODE","target":"130:10266","navigation":"NAVIGATE"}]`

## ST-646 · Flow completion / 1280 / Flow completion / SecondFastaDocument Result

**Role:** Responsive/context variant. **Frame:** 130:10266 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-10266).

**Displayed content:**

- No cut sites found
- Close
- Analysis completed for the selected input.
- Input
- DNA-FA-002 · version 1
- Enzymes
- EcoRI and BamHI
- Predicted output
- One uncut linear fragment · 12 nt
- Result state
- Completed · not a connection or validation error
- Change settings
- View source
- Save document

**Authored actions:**

- `130:10270` Action / Close → `[{"type":"BACK"}]`
- `130:10291` Action / Change settings → `[{"type":"NODE","target":"130:10243","navigation":"NAVIGATE"}]`
- `130:10292` Action / View source → `[{"type":"NODE","target":"130:10093","navigation":"NAVIGATE"}]`
- `130:10293` Action / Save document → `[{"type":"NODE","target":"130:10294","navigation":"NAVIGATE"}]`

## ST-647 · Flow completion / 1280 / Flow completion / SecondFastaDocument Save

**Role:** Responsive/context variant. **Frame:** 130:10294 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-10294).

**Displayed content:**

- Save Control fragment
- Close
- Choose a location without changing the source identity.
- Document
- DNA-FA-002 · version 1
- Location
- Local workspace / Sequences
- Annotations
- No annotations in source FASTA
- Cancel
- Save document

**Authored actions:**

- `130:10298` Action / Close → `[{"type":"BACK"}]`
- `130:10315` Action / Cancel → `[{"type":"NODE","target":"130:10093","navigation":"NAVIGATE"}]`
- `130:10316` Action / Save document → `[{"type":"NODE","target":"130:10317","navigation":"NAVIGATE"}]`

## ST-648 · Flow completion / 1280 / Flow completion / SecondFastaDocument Saved

**Role:** Responsive/context variant. **Frame:** 130:10317 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-10317).

**Displayed content:**

- Control fragment saved
- Close
- DNA-FA-002 · Local workspace / Sequences
- Input version
- 1
- Retained source
- control_fragment.fasta
- Open sequence
- Find saved document

**Authored actions:**

- `130:10321` Action / Close → `[{"type":"BACK"}]`
- `130:10334` Action / Open sequence → `[{"type":"NODE","target":"130:10093","navigation":"NAVIGATE"}]`
- `130:10335` Action / Find saved document → `[{"type":"NODE","target":"130:10336","navigation":"NAVIGATE"}]`

## ST-649 · Flow completion / 1280 / Flow completion / SecondFastaDocument Home

**Role:** Responsive/context variant. **Frame:** 130:10336 · 1280 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-10336).

**Displayed content:**

- Continue your work
- Your newly saved document is available by name and ID.
- Search
- New
- Details & tools
- Document
- Identity
- Location
- Control fragment
- DNA-FA-002 · v1
- Local workspace / Sequences

**Authored actions:**

- `130:10443` Action / Search → `[{"type":"NODE","target":"130:8241","navigation":"NAVIGATE"}]`
- `130:10444` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `130:10463` Action / Details & tools → `[{"type":"NODE","target":"130:10461","navigation":"OVERLAY"}]`
- `130:10451` Record / Control fragment → `[{"type":"NODE","target":"130:10093","navigation":"NAVIGATE"}]`

## ST-650 · Flow completion / 1280 panel / Flow completion / SecondFastaDocument Home

**Role:** Responsive/context variant. **Frame:** 130:10461 · 320 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-10461).

**Displayed content:**

- Context & next steps
- Close panel

**Authored actions:**

- `130:10466` Action / Close panel → `[{"type":"CLOSE"}]`

## ST-651 · Flow completion / 1280 / Flow completion / BlankReview

**Role:** Responsive/context variant. **Frame:** 130:10469 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-10469).

**Displayed content:**

- Review your new sequence
- Close
- Check the input before creation.
- Name / ID
- Untitled sequence · DNA-NEW-001
- Type / topology
- DNA · linear
- Length
- 12 nt
- Original bases
- ATGCGTACGTAA
- Changes
- None · no unsupported symbols
- Back to input
- Create sequence
- Review an input issue

**Authored actions:**

- `130:10473` Action / Close → `[{"type":"BACK"}]`
- `130:10498` Action / Back to input → `[{"type":"NODE","target":"110:20202","navigation":"NAVIGATE"}]`
- `130:10499` Action / Create sequence → `[{"type":"NODE","target":"130:10501","navigation":"NAVIGATE"}]`
- `130:10500` Action / Review an input issue → `[{"type":"NODE","target":"110:20168","navigation":"NAVIGATE"}]`

## ST-652 · Flow completion / 1280 / Flow completion / BlankEditor

**Role:** Responsive/context variant. **Frame:** 130:10501 · 1280 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-10501).

**Displayed content:**

- Untitled sequence
- DNA-NEW-001 · version 1 · 12 nt · linear · not registered
- Search
- New
- Details & tools
- Your sequence
- Coordinates are 1-based and inclusive. No features or primers have been added.
- 1                           12
- ATGCGTACGTAA
- Edit sequence
- Save document
- Restriction analysis

**Authored actions:**

- `130:10608` Action / Search → `[{"type":"NODE","target":"130:8241","navigation":"NAVIGATE"}]`
- `130:10609` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `130:10634` Action / Details & tools → `[{"type":"NODE","target":"130:10631","navigation":"OVERLAY"}]`
- `130:10618` Action / Edit sequence → `[{"type":"NODE","target":"125:6630","navigation":"NAVIGATE"}]`
- `130:10619` Action / Save document → `[{"type":"NODE","target":"130:10640","navigation":"NAVIGATE"}]`
- `130:10620` Action / Restriction analysis → `[{"type":"NODE","target":"130:10815","navigation":"NAVIGATE"}]`

## ST-653 · Flow completion / 1280 panel / Flow completion / BlankEditor

**Role:** Responsive/context variant. **Frame:** 130:10631 · 320 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-10631).

**Displayed content:**

- Context & next steps
- This is your newly created
document. Registry and physical
inventory are optional.
- Close panel

**Authored actions:**

- `130:10637` Action / Close panel → `[{"type":"CLOSE"}]`

## ST-654 · Flow completion / 1280 / Flow completion / BlankSave

**Role:** Responsive/context variant. **Frame:** 130:10640 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-10640).

**Displayed content:**

- Save DNA-NEW-001
- Close
- Choose a destination and retain the document identity.
- Name
- Untitled sequence
- Location
- Local workspace / Sequences
- Version
- 1
- Cancel
- Save document

**Authored actions:**

- `130:10644` Action / Close → `[{"type":"BACK"}]`
- `130:10661` Action / Cancel → `[{"type":"NODE","target":"130:10501","navigation":"NAVIGATE"}]`
- `130:10662` Action / Save document → `[{"type":"NODE","target":"130:10663","navigation":"NAVIGATE"}]`

## ST-655 · Flow completion / 1280 / Flow completion / BlankSaved

**Role:** Responsive/context variant. **Frame:** 130:10663 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-10663).

**Displayed content:**

- Untitled sequence saved
- Close
- DNA-NEW-001 · Local workspace / Sequences
- Saved version
- 1
- Length / topology
- 12 nt · linear
- Open sequence
- Home

**Authored actions:**

- `130:10667` Action / Close → `[{"type":"BACK"}]`
- `130:10680` Action / Open sequence → `[{"type":"NODE","target":"130:10501","navigation":"NAVIGATE"}]`
- `130:10681` Action / Home → `[{"type":"NODE","target":"130:10682","navigation":"NAVIGATE"}]`

## ST-656 · Flow completion / 1280 / Flow completion / BlankDocumentHome

**Role:** Responsive/context variant. **Frame:** 130:10682 · 1280 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-10682).

**Displayed content:**

- Continue your work
- Your newly saved sequence is available by its own identity.
- Search
- New
- Details & tools
- Document
- Identity
- Location
- Untitled sequence
- DNA-NEW-001 · version 1
- Local workspace / Sequences

**Authored actions:**

- `130:10789` Action / Search → `[{"type":"NODE","target":"130:8241","navigation":"NAVIGATE"}]`
- `130:10790` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `130:10809` Action / Details & tools → `[{"type":"NODE","target":"130:10807","navigation":"OVERLAY"}]`
- `130:10797` Record / Untitled sequence → `[{"type":"NODE","target":"130:10501","navigation":"NAVIGATE"}]`

## ST-657 · Flow completion / 1280 panel / Flow completion / BlankDocumentHome

**Role:** Responsive/context variant. **Frame:** 130:10807 · 320 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-10807).

**Displayed content:**

- Context & next steps
- Close panel

**Authored actions:**

- `130:10812` Action / Close panel → `[{"type":"CLOSE"}]`

## ST-658 · Flow completion / 1280 / Flow completion / BlankAnalysis

**Role:** Responsive/context variant. **Frame:** 130:10815 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-10815).

**Displayed content:**

- Restriction analysis · DNA-NEW-001
- Close
- Use the complete 12 nt linear input.
- Input
- DNA-NEW-001 · v1 · ATGCGTACGTAA
- Enzymes
- EcoRI and BamHI
- Scope
- Whole linear sequence
- Cancel
- Run analysis

**Authored actions:**

- `130:10819` Action / Close → `[{"type":"BACK"}]`
- `130:10836` Action / Cancel → `[{"type":"NODE","target":"130:10501","navigation":"NAVIGATE"}]`
- `130:10837` Action / Run analysis → `[{"type":"NODE","target":"130:10838","navigation":"NAVIGATE"}]`

## ST-659 · Flow completion / 1280 / Flow completion / BlankNoSites

**Role:** Responsive/context variant. **Frame:** 130:10838 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-10838).

**Displayed content:**

- No cut sites found
- Close
- The analysis completed; this is not an error.
- Input
- DNA-NEW-001 · v1
- Enzymes
- EcoRI and BamHI
- Predicted output
- One uncut linear fragment · 12 nt
- Change enzyme selection
- Return to input

**Authored actions:**

- `130:10842` Action / Close → `[{"type":"BACK"}]`
- `130:10859` Action / Change enzyme selection → `[{"type":"NODE","target":"130:10815","navigation":"NAVIGATE"}]`
- `130:10860` Action / Return to input → `[{"type":"NODE","target":"130:10501","navigation":"NAVIGATE"}]`

## ST-660 · Flow completion / 1280 / Flow completion / AllSearch Source

**Role:** Responsive/context variant. **Frame:** 130:10861 · 1280 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-10861).

**Displayed content:**

- Map
- Inspect a construct and act on the selected region.
- Search
- New
- Back to search
- Details & tools
- pHelixBench-Demo
- v7 · saved 10:42
- Undo
- Save version
- Export
- Sequence
- Features
- Primers
- Enzymes
- History
- Add feature
- Analyze
- Display layers
- Circular map
- Selection 97–116 · forward
- 3021 bp · circular
- GC 54.5%
- drag to select
- T7 promoter
- lac operator
- RBS
- demo_cds
- Demo-Rev
- rrnB T1 terminator
- M13 fwd site
- pUC19 MCS
- M13 rev site
- ori (demo)
- demo_marker
- loxP
- FRT
- Kozak
- EcoRI
- BamHI
- ScaI
- AhdI
- SexAI
- PshAI
- DraI
- NcoI
- T7 promoter selected. Create and analysis tools retain this document and range.
- Readable annotations
- Restriction analysis
- Task help

**Authored actions:**

- `130:10968` Action / Search → `[{"type":"NODE","target":"130:8241","navigation":"NAVIGATE"}]`
- `130:10969` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `130:10970` Action / Back to search → `[{"type":"NODE","target":"130:8241","navigation":"NAVIGATE"}]`
- `130:11183` Action / Details & tools → `[{"type":"NODE","target":"130:11137","navigation":"OVERLAY"}]`
- `130:10976` Action / Save version → `[{"type":"NODE","target":"107:29211","navigation":"OVERLAY"}]`
- `130:10977` Action / Export → `[{"type":"NODE","target":"107:27142","navigation":"OVERLAY"}]`
- `130:10980` Action / Sequence → `[{"type":"NODE","target":"107:29677","navigation":"NAVIGATE"}]`
- `130:10981` Action / Features → `[{"type":"NODE","target":"107:29886","navigation":"NAVIGATE"}]`
- `130:10982` Action / Primers → `[{"type":"NODE","target":"107:30093","navigation":"NAVIGATE"}]`
- `130:10983` Action / Enzymes → `[{"type":"NODE","target":"107:30295","navigation":"NAVIGATE"}]`
- `130:10984` Action / History → `[{"type":"NODE","target":"107:30500","navigation":"NAVIGATE"}]`
- `130:10987` Action / Add feature → `[{"type":"NODE","target":"107:25134","navigation":"OVERLAY"}]`
- `130:10988` Action / Analyze → `[{"type":"NODE","target":"130:7642","navigation":"NAVIGATE"}]`
- `130:10989` Action / Display layers → `[{"type":"NODE","target":"107:25217","navigation":"OVERLAY"}]`
- `130:11068` Action / Readable annotations → `[{"type":"NODE","target":"107:29886","navigation":"NAVIGATE"}]`
- `130:11069` Action / Restriction analysis → `[{"type":"NODE","target":"130:7642","navigation":"NAVIGATE"}]`
- `130:11070` Action / Task help → `[{"type":"NODE","target":"130:8895","navigation":"NAVIGATE"}]`

## ST-661 · Flow completion / 1280 panel / Flow completion / AllSearch Source

**Role:** Responsive/context variant. **Frame:** 130:11137 · 320 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-11137).

**Displayed content:**

- Details & tools
- All map tools
- Current selection
- T7 promoter · 97–116
- How to use Map
- Select a feature on the map or use
Features for a readable list. Tools
use the range shown at right.
- Record
- DNA-001 · version 7
- Selected range
- 97–116 · forward strand
- Type
- Promoter · 20 bp
- Actions
- Edit feature
- Add primer from range
- Design primer pair
- Related records
- DNA-001 · Registry
- Plate P-014 · Inventory
- Close panel

**Authored actions:**

- `130:11151` Action / All map tools → `[{"type":"NODE","target":"110:22016","navigation":"NAVIGATE"}]`
- `130:11168` Action / Edit feature → `[{"type":"NODE","target":"107:25134","navigation":"OVERLAY"}]`
- `130:11169` Action / Add primer from range → `[{"type":"NODE","target":"107:25182","navigation":"OVERLAY"}]`
- `130:11170` Action / Design primer pair → `[{"type":"NODE","target":"107:25303","navigation":"OVERLAY"}]`
- `130:11173` Related / DNA-001 · Registry → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`
- `130:11178` Related / Plate P-014 · Inventory → `[{"type":"NODE","target":"107:31089","navigation":"NAVIGATE"}]`
- `130:11186` Action / Close panel → `[{"type":"CLOSE"}]`

## ST-662 · Flow completion / 1280 / Flow completion / AllSearch Result

**Role:** Responsive/context variant. **Frame:** 130:11189 · 1280 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-11189).

**Displayed content:**

- Restriction analysis result
- RES-001 · pHelixBench-Demo · input DNA-001 v7
- Search
- New
- Back to search
- Details & tools
- Two predicted fragments
- EcoRI at 216 · BamHI at 234 · circular input length 3021 bp.
- Fragment
- Length
- Interpretation
- Fragment 1
- 18 bp
- Between the two selected cut positions
- Fragment 2
- 3003 bp
- Remaining circular backbone
- Total
- 3021 bp
- Matches input length
- Illustrative result fixture. Verify predictions and experimental assumptions before laboratory use.
- Input and settings stay attached
- DNA-001 · v7 · whole sequence · EcoRI + BamHI · RES-001
- Edit settings
- View source
- Save result

**Authored actions:**

- `130:11296` Action / Search → `[{"type":"NODE","target":"130:8241","navigation":"NAVIGATE"}]`
- `130:11297` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `130:11298` Action / Back to search → `[{"type":"NODE","target":"130:8241","navigation":"NAVIGATE"}]`
- `130:11351` Action / Details & tools → `[{"type":"NODE","target":"130:11345","navigation":"OVERLAY"}]`
- `130:11328` Action / Edit settings → `[{"type":"NODE","target":"130:7642","navigation":"NAVIGATE"}]`
- `130:11329` Action / View source → `[{"type":"NODE","target":"130:7316","navigation":"NAVIGATE"}]`
- `130:11330` Action / Save result → `[{"type":"NODE","target":"130:8068","navigation":"NAVIGATE"}]`

## ST-663 · Flow completion / 1280 panel / Flow completion / AllSearch Result

**Role:** Responsive/context variant. **Frame:** 130:11345 · 320 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-11345).

**Displayed content:**

- Read the result
- Fragment lengths describe this
input and enzyme set. The small
fragment may need different
experimental handling.
- Parameter summary
- How to interpret this
- Close panel

**Authored actions:**

- `130:11349` Action / Parameter summary → `[{"type":"NODE","target":"130:7803","navigation":"NAVIGATE"}]`
- `130:11350` Action / How to interpret this → `[{"type":"NODE","target":"130:7829","navigation":"NAVIGATE"}]`
- `130:11354` Action / Close panel → `[{"type":"CLOSE"}]`

## ST-664 · Flow completion / 1280 / Flow completion / SequenceSearch Source

**Role:** Responsive/context variant. **Frame:** 130:11357 · 1280 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-11357).

**Displayed content:**

- Map
- Inspect a construct and act on the selected region.
- Search
- New
- Back to search
- Details & tools
- pHelixBench-Demo
- v7 · saved 10:42
- Undo
- Save version
- Export
- Sequence
- Features
- Primers
- Enzymes
- History
- Add feature
- Analyze
- Display layers
- Circular map
- Selection 97–116 · forward
- 3021 bp · circular
- GC 54.5%
- drag to select
- T7 promoter
- lac operator
- RBS
- demo_cds
- Demo-Rev
- rrnB T1 terminator
- M13 fwd site
- pUC19 MCS
- M13 rev site
- ori (demo)
- demo_marker
- loxP
- FRT
- Kozak
- EcoRI
- BamHI
- ScaI
- AhdI
- SexAI
- PshAI
- DraI
- NcoI
- T7 promoter selected. Create and analysis tools retain this document and range.
- Readable annotations
- Restriction analysis
- Task help

**Authored actions:**

- `130:11464` Action / Search → `[{"type":"NODE","target":"130:8241","navigation":"NAVIGATE"}]`
- `130:11465` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `130:11466` Action / Back to search → `[{"type":"NODE","target":"130:8401","navigation":"NAVIGATE"}]`
- `130:11679` Action / Details & tools → `[{"type":"NODE","target":"130:11633","navigation":"OVERLAY"}]`
- `130:11472` Action / Save version → `[{"type":"NODE","target":"107:29211","navigation":"OVERLAY"}]`
- `130:11473` Action / Export → `[{"type":"NODE","target":"107:27142","navigation":"OVERLAY"}]`
- `130:11476` Action / Sequence → `[{"type":"NODE","target":"107:29677","navigation":"NAVIGATE"}]`
- `130:11477` Action / Features → `[{"type":"NODE","target":"107:29886","navigation":"NAVIGATE"}]`
- `130:11478` Action / Primers → `[{"type":"NODE","target":"107:30093","navigation":"NAVIGATE"}]`
- `130:11479` Action / Enzymes → `[{"type":"NODE","target":"107:30295","navigation":"NAVIGATE"}]`
- `130:11480` Action / History → `[{"type":"NODE","target":"107:30500","navigation":"NAVIGATE"}]`
- `130:11483` Action / Add feature → `[{"type":"NODE","target":"107:25134","navigation":"OVERLAY"}]`
- `130:11484` Action / Analyze → `[{"type":"NODE","target":"130:7642","navigation":"NAVIGATE"}]`
- `130:11485` Action / Display layers → `[{"type":"NODE","target":"107:25217","navigation":"OVERLAY"}]`
- `130:11564` Action / Readable annotations → `[{"type":"NODE","target":"107:29886","navigation":"NAVIGATE"}]`
- `130:11565` Action / Restriction analysis → `[{"type":"NODE","target":"130:7642","navigation":"NAVIGATE"}]`
- `130:11566` Action / Task help → `[{"type":"NODE","target":"130:8895","navigation":"NAVIGATE"}]`

## ST-665 · Flow completion / 1280 panel / Flow completion / SequenceSearch Source

**Role:** Responsive/context variant. **Frame:** 130:11633 · 320 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-11633).

**Displayed content:**

- Details & tools
- All map tools
- Current selection
- T7 promoter · 97–116
- How to use Map
- Select a feature on the map or use
Features for a readable list. Tools
use the range shown at right.
- Record
- DNA-001 · version 7
- Selected range
- 97–116 · forward strand
- Type
- Promoter · 20 bp
- Actions
- Edit feature
- Add primer from range
- Design primer pair
- Related records
- DNA-001 · Registry
- Plate P-014 · Inventory
- Close panel

**Authored actions:**

- `130:11647` Action / All map tools → `[{"type":"NODE","target":"110:22016","navigation":"NAVIGATE"}]`
- `130:11664` Action / Edit feature → `[{"type":"NODE","target":"107:25134","navigation":"OVERLAY"}]`
- `130:11665` Action / Add primer from range → `[{"type":"NODE","target":"107:25182","navigation":"OVERLAY"}]`
- `130:11666` Action / Design primer pair → `[{"type":"NODE","target":"107:25303","navigation":"OVERLAY"}]`
- `130:11669` Related / DNA-001 · Registry → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`
- `130:11674` Related / Plate P-014 · Inventory → `[{"type":"NODE","target":"107:31089","navigation":"NAVIGATE"}]`
- `130:11682` Action / Close panel → `[{"type":"CLOSE"}]`

## ST-666 · Flow completion / 1280 / Flow completion / ResultSearch Result

**Role:** Responsive/context variant. **Frame:** 130:11685 · 1280 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-11685).

**Displayed content:**

- Restriction analysis result
- RES-001 · pHelixBench-Demo · input DNA-001 v7
- Search
- New
- Back to search
- Details & tools
- Two predicted fragments
- EcoRI at 216 · BamHI at 234 · circular input length 3021 bp.
- Fragment
- Length
- Interpretation
- Fragment 1
- 18 bp
- Between the two selected cut positions
- Fragment 2
- 3003 bp
- Remaining circular backbone
- Total
- 3021 bp
- Matches input length
- Illustrative result fixture. Verify predictions and experimental assumptions before laboratory use.
- Input and settings stay attached
- DNA-001 · v7 · whole sequence · EcoRI + BamHI · RES-001
- Edit settings
- View source
- Save result

**Authored actions:**

- `130:11792` Action / Search → `[{"type":"NODE","target":"130:8241","navigation":"NAVIGATE"}]`
- `130:11793` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `130:11794` Action / Back to search → `[{"type":"NODE","target":"130:8552","navigation":"NAVIGATE"}]`
- `130:11847` Action / Details & tools → `[{"type":"NODE","target":"130:11841","navigation":"OVERLAY"}]`
- `130:11824` Action / Edit settings → `[{"type":"NODE","target":"130:7642","navigation":"NAVIGATE"}]`
- `130:11825` Action / View source → `[{"type":"NODE","target":"130:7316","navigation":"NAVIGATE"}]`
- `130:11826` Action / Save result → `[{"type":"NODE","target":"130:8068","navigation":"NAVIGATE"}]`

## ST-667 · Flow completion / 1280 panel / Flow completion / ResultSearch Result

**Role:** Responsive/context variant. **Frame:** 130:11841 · 320 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-11841).

**Displayed content:**

- Read the result
- Fragment lengths describe this
input and enzyme set. The small
fragment may need different
experimental handling.
- Parameter summary
- How to interpret this
- Close panel

**Authored actions:**

- `130:11845` Action / Parameter summary → `[{"type":"NODE","target":"130:7803","navigation":"NAVIGATE"}]`
- `130:11846` Action / How to interpret this → `[{"type":"NODE","target":"130:7829","navigation":"NAVIGATE"}]`
- `130:11850` Action / Close panel → `[{"type":"CLOSE"}]`

## ST-668 · Flow completion / 1280 / Flow completion / RecentSearch Source

**Role:** Responsive/context variant. **Frame:** 130:11853 · 1280 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-11853).

**Displayed content:**

- Map
- Inspect a construct and act on the selected region.
- Search
- New
- Back to search
- Details & tools
- pHelixBench-Demo
- v7 · saved 10:42
- Undo
- Save version
- Export
- Sequence
- Features
- Primers
- Enzymes
- History
- Add feature
- Analyze
- Display layers
- Circular map
- Selection 97–116 · forward
- 3021 bp · circular
- GC 54.5%
- drag to select
- T7 promoter
- lac operator
- RBS
- demo_cds
- Demo-Rev
- rrnB T1 terminator
- M13 fwd site
- pUC19 MCS
- M13 rev site
- ori (demo)
- demo_marker
- loxP
- FRT
- Kozak
- EcoRI
- BamHI
- ScaI
- AhdI
- SexAI
- PshAI
- DraI
- NcoI
- T7 promoter selected. Create and analysis tools retain this document and range.
- Readable annotations
- Restriction analysis
- Task help

**Authored actions:**

- `130:11960` Action / Search → `[{"type":"NODE","target":"130:8241","navigation":"NAVIGATE"}]`
- `130:11961` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `130:11962` Action / Back to search → `[{"type":"NODE","target":"130:8703","navigation":"NAVIGATE"}]`
- `130:12175` Action / Details & tools → `[{"type":"NODE","target":"130:12129","navigation":"OVERLAY"}]`
- `130:11968` Action / Save version → `[{"type":"NODE","target":"107:29211","navigation":"OVERLAY"}]`
- `130:11969` Action / Export → `[{"type":"NODE","target":"107:27142","navigation":"OVERLAY"}]`
- `130:11972` Action / Sequence → `[{"type":"NODE","target":"107:29677","navigation":"NAVIGATE"}]`
- `130:11973` Action / Features → `[{"type":"NODE","target":"107:29886","navigation":"NAVIGATE"}]`
- `130:11974` Action / Primers → `[{"type":"NODE","target":"107:30093","navigation":"NAVIGATE"}]`
- `130:11975` Action / Enzymes → `[{"type":"NODE","target":"107:30295","navigation":"NAVIGATE"}]`
- `130:11976` Action / History → `[{"type":"NODE","target":"107:30500","navigation":"NAVIGATE"}]`
- `130:11979` Action / Add feature → `[{"type":"NODE","target":"107:25134","navigation":"OVERLAY"}]`
- `130:11980` Action / Analyze → `[{"type":"NODE","target":"130:7642","navigation":"NAVIGATE"}]`
- `130:11981` Action / Display layers → `[{"type":"NODE","target":"107:25217","navigation":"OVERLAY"}]`
- `130:12060` Action / Readable annotations → `[{"type":"NODE","target":"107:29886","navigation":"NAVIGATE"}]`
- `130:12061` Action / Restriction analysis → `[{"type":"NODE","target":"130:7642","navigation":"NAVIGATE"}]`
- `130:12062` Action / Task help → `[{"type":"NODE","target":"130:8895","navigation":"NAVIGATE"}]`

## ST-669 · Flow completion / 1280 panel / Flow completion / RecentSearch Source

**Role:** Responsive/context variant. **Frame:** 130:12129 · 320 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-12129).

**Displayed content:**

- Details & tools
- All map tools
- Current selection
- T7 promoter · 97–116
- How to use Map
- Select a feature on the map or use
Features for a readable list. Tools
use the range shown at right.
- Record
- DNA-001 · version 7
- Selected range
- 97–116 · forward strand
- Type
- Promoter · 20 bp
- Actions
- Edit feature
- Add primer from range
- Design primer pair
- Related records
- DNA-001 · Registry
- Plate P-014 · Inventory
- Close panel

**Authored actions:**

- `130:12143` Action / All map tools → `[{"type":"NODE","target":"110:22016","navigation":"NAVIGATE"}]`
- `130:12160` Action / Edit feature → `[{"type":"NODE","target":"107:25134","navigation":"OVERLAY"}]`
- `130:12161` Action / Add primer from range → `[{"type":"NODE","target":"107:25182","navigation":"OVERLAY"}]`
- `130:12162` Action / Design primer pair → `[{"type":"NODE","target":"107:25303","navigation":"OVERLAY"}]`
- `130:12165` Related / DNA-001 · Registry → `[{"type":"NODE","target":"107:30897","navigation":"NAVIGATE"}]`
- `130:12170` Related / Plate P-014 · Inventory → `[{"type":"NODE","target":"107:31089","navigation":"NAVIGATE"}]`
- `130:12178` Action / Close panel → `[{"type":"CLOSE"}]`

## ST-670 · Flow completion / 1280 / Flow completion / RecentSearch Result

**Role:** Responsive/context variant. **Frame:** 130:12181 · 1280 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-12181).

**Displayed content:**

- Restriction analysis result
- RES-001 · pHelixBench-Demo · input DNA-001 v7
- Search
- New
- Back to search
- Details & tools
- Two predicted fragments
- EcoRI at 216 · BamHI at 234 · circular input length 3021 bp.
- Fragment
- Length
- Interpretation
- Fragment 1
- 18 bp
- Between the two selected cut positions
- Fragment 2
- 3003 bp
- Remaining circular backbone
- Total
- 3021 bp
- Matches input length
- Illustrative result fixture. Verify predictions and experimental assumptions before laboratory use.
- Input and settings stay attached
- DNA-001 · v7 · whole sequence · EcoRI + BamHI · RES-001
- Edit settings
- View source
- Save result

**Authored actions:**

- `130:12288` Action / Search → `[{"type":"NODE","target":"130:8241","navigation":"NAVIGATE"}]`
- `130:12289` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `130:12290` Action / Back to search → `[{"type":"NODE","target":"130:8703","navigation":"NAVIGATE"}]`
- `130:12343` Action / Details & tools → `[{"type":"NODE","target":"130:12337","navigation":"OVERLAY"}]`
- `130:12320` Action / Edit settings → `[{"type":"NODE","target":"130:7642","navigation":"NAVIGATE"}]`
- `130:12321` Action / View source → `[{"type":"NODE","target":"130:7316","navigation":"NAVIGATE"}]`
- `130:12322` Action / Save result → `[{"type":"NODE","target":"130:8068","navigation":"NAVIGATE"}]`

## ST-671 · Flow completion / 1280 panel / Flow completion / RecentSearch Result

**Role:** Responsive/context variant. **Frame:** 130:12337 · 320 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=130-12337).

**Displayed content:**

- Read the result
- Fragment lengths describe this
input and enzyme set. The small
fragment may need different
experimental handling.
- Parameter summary
- How to interpret this
- Close panel

**Authored actions:**

- `130:12341` Action / Parameter summary → `[{"type":"NODE","target":"130:7803","navigation":"NAVIGATE"}]`
- `130:12342` Action / How to interpret this → `[{"type":"NODE","target":"130:7829","navigation":"NAVIGATE"}]`
- `130:12346` Action / Close panel → `[{"type":"CLOSE"}]`

## ST-672 · Flow completion / TemplateApplied

**Role:** Linked functional state. **Frame:** 132:5034 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=132-5034).

**Displayed content:**

- Untitled experiment
- EXP-NEW-001 · local draft · no submitted rows
- Search
- New
- Sequence validation
- EXP-NEW-001 · template headings applied · no source linked
- Purpose
- Add your own purpose…
- Method
- Add your own method…
- Results
- Add your own results…
- Link input
- Add result table
- Context & next steps
- Start with your own notes or
deliberately apply a template. This
draft remains EXP-NEW-001.

**Authored actions:**

- `132:5141` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `132:5142` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `132:5184` Action / Link input → `[{"type":"NODE","target":"125:6966","navigation":"NAVIGATE"}]`
- `132:5186` Action / Add result table → `[{"type":"NODE","target":"125:7160","navigation":"NAVIGATE"}]`

## ST-673 · Flow completion / ReviewerPicker

**Role:** Linked functional state. **Frame:** 132:5188 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=132-5188).

**Displayed content:**

- Choose an entry reviewer
- Close
- Only authorized collaborators can be selected.
- Entry
- EXP-NEW-001
- Available reviewer
- Reviewer A · authorized project reviewer
- Access
- Can read this entry and its submitted result
- Back to entry
- Select Reviewer A

**Authored actions:**

- `132:5192` Action / Close → `[{"type":"BACK"}]`
- `132:5210` Action / Back to entry → `[{"type":"NODE","target":"125:6988","navigation":"NAVIGATE"}]`
- `132:5212` Action / Select Reviewer A → `[{"type":"NODE","target":"132:5214","navigation":"NAVIGATE"}]`

## ST-674 · Flow completion / ReviewSummary

**Role:** Linked functional state. **Frame:** 132:5214 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=132-5214).

**Displayed content:**

- Send EXP-NEW-001 for review?
- Close
- Confirm the entry, submitted result and audience.
- Entry
- EXP-NEW-001
- Submitted result
- NRES-001 · source DNA-001 v7
- Recipient
- Reviewer A · authorized reviewer
- Access effect
- Review access to this entry; no public sharing
- Change reviewer
- Send review request

**Authored actions:**

- `132:5218` Action / Close → `[{"type":"BACK"}]`
- `132:5240` Action / Change reviewer → `[{"type":"NODE","target":"132:5188","navigation":"NAVIGATE"}]`
- `132:5242` Action / Send review request → `[{"type":"NODE","target":"132:5244","navigation":"NAVIGATE"}]`

## ST-675 · Flow completion / ReviewSent

**Role:** Linked functional state. **Frame:** 132:5244 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=132-5244).

**Displayed content:**

- Review request sent
- Close
- EXP-NEW-001 · awaiting Reviewer A
- Request
- REVIEW-NEW-001
- Submitted result
- NRES-001
- Next step
- Continue working or respond to reviewer feedback
- Return to entry
- Inspect requested changes

**Authored actions:**

- `132:5248` Action / Close → `[{"type":"BACK"}]`
- `132:5266` Action / Return to entry → `[{"type":"NODE","target":"135:5285","navigation":"NAVIGATE"}]`
- `132:5268` Action / Inspect requested changes → `[{"type":"NODE","target":"132:5270","navigation":"NAVIGATE"}]`

## ST-676 · Flow completion / ReviewChanges

**Role:** Linked functional state. **Frame:** 132:5270 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=132-5270).

**Displayed content:**

- Changes requested
- Close
- The reviewer requests a method summary before approval.
- Entry
- EXP-NEW-001
- Request
- REVIEW-NEW-001
- Feedback
- Add a method summary; retain the linked source and result
- Edit entry

**Authored actions:**

- `132:5274` Action / Close → `[{"type":"BACK"}]`
- `132:5292` Action / Edit entry → `[{"type":"NODE","target":"132:5294","navigation":"NAVIGATE"}]`

## ST-677 · Flow completion / ReviewAmendment

**Role:** Linked functional state. **Frame:** 132:5294 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=132-5294).

**Displayed content:**

- Update the method summary
- Close
- Your entry and source identity are retained.
- Entry
- EXP-NEW-001 · revision 2
- Method summary
- Sequence inspected; source version and validation result recorded
- Linked result
- NRES-001 · unchanged
- Keep editing
- Resubmit for review

**Authored actions:**

- `132:5298` Action / Close → `[{"type":"BACK"}]`
- `132:5316` Action / Keep editing → `[{"type":"NODE","target":"125:6988","navigation":"NAVIGATE"}]`
- `132:5318` Action / Resubmit for review → `[{"type":"NODE","target":"132:5320","navigation":"NAVIGATE"}]`

## ST-678 · Flow completion / ReviewResubmitted

**Role:** Linked functional state. **Frame:** 132:5320 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=132-5320).

**Displayed content:**

- Entry resubmitted
- Close
- EXP-NEW-001 · revision 2 · awaiting review
- Review request
- REVIEW-NEW-001
- Recipient
- Reviewer A
- Preserved context
- DNA-001 v7 · NRES-001
- Return to entry

**Authored actions:**

- `132:5324` Action / Close → `[{"type":"BACK"}]`
- `132:5342` Action / Return to entry → `[{"type":"NODE","target":"135:5475","navigation":"NAVIGATE"}]`

## ST-679 · Flow completion / RetainedResultAfterDelete

**Role:** Linked functional state. **Frame:** 132:5347 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=132-5347).

**Displayed content:**

- Retained analysis result
- RES-001 · pHelixBench-Demo · input DNA-001 v7
- Search
- New
- Two predicted fragments
- EcoRI at 216 · BamHI at 234 · circular input length 3021 bp.
- Fragment
- Length
- Interpretation
- Fragment 1
- 18 bp
- Between the two selected cut positions
- Fragment 2
- 3003 bp
- Remaining circular backbone
- Total
- 3021 bp
- Matches input length
- Illustrative result fixture. Verify predictions and experimental assumptions before laboratory use.
- Input and settings stay attached
- DNA-001 · v7 · whole sequence · EcoRI + BamHI · RES-001
- Edit settings
- View source
- Save result
- Source DNA-001 was deleted. Provenance is retained; a rerun requires an accessible input.
- Read the result
- Fragment lengths describe this
input and enzyme set. The small
fragment may need different
experimental handling.
- Parameter summary
- How to interpret this

**Authored actions:**

- `132:5454` Action / Search → `[{"type":"NODE","target":"125:5668","navigation":"NAVIGATE"}]`
- `132:5455` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `132:5485` Action / Edit settings → `[{"type":"NODE","target":"125:5642","navigation":"NAVIGATE"}]`
- `132:5486` Action / View source → `[{"type":"NODE","target":"125:5642","navigation":"NAVIGATE"}]`
- `132:5487` Action / Save result → `[{"type":"NODE","target":"132:5507","navigation":"NAVIGATE"}]`
- `132:5492` Action / Parameter summary → `[{"type":"NODE","target":"110:20486","navigation":"NAVIGATE"}]`
- `132:5493` Action / How to interpret this → `[{"type":"NODE","target":"110:20514","navigation":"NAVIGATE"}]`

## ST-680 · Flow completion / ExportRetained

**Role:** Linked functional state. **Frame:** 132:5507 · 720 × 900. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=132-5507).

**Displayed content:**

- Export retained result RES-001
- Close
- The source is unavailable, but its provenance remains attached.
- Result
- RES-001 · 18 bp + 3003 bp
- Original input
- DNA-001 · version 7 · deleted
- Filename
- RES-001_retained_result.csv
- Included metadata
- Original source version, enzymes, scope and fragment lengths
- Return to result

**Authored actions:**

- `132:5511` Action / Close → `[{"type":"BACK"}]`
- `132:5533` Action / Return to result → `[{"type":"NODE","target":"132:5347","navigation":"NAVIGATE"}]`

## ST-681 · Flow completion / DeletedContextHome

**Role:** Reference or alternate fixture. **Frame:** 132:5535 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=132-5535).

**Displayed content:**

- Continue your work
- DNA-001 is deleted; its retained output remains identifiable.
- Search
- New
- Record
- State
- Context
- Restriction analysis · RES-001
- Retained result
- Source DNA-001 v7 · unavailable
- T7-F · PR-001
- Primer
- Template DNA-001 unavailable
- Context & next steps

**Authored actions:**

- `132:5642` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `132:5643` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `132:5703` Record / Restriction analysis · RES-001 → `[{"type":"NODE","target":"132:5347","navigation":"NAVIGATE"}]`
- `132:5707` Record / T7-F · PR-001 → `[{"type":"NODE","target":"132:5711","navigation":"NAVIGATE"}]`

## ST-682 · Flow completion / PrimerAfterDelete

**Role:** Linked functional state. **Frame:** 132:5711 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=132-5711).

**Displayed content:**

- T7-F · PR-001
- Primer retained · linked template DNA-001 is unavailable
- Search
- New
- Property
- Value
- Primer identity
- PR-001 · T7-F · 20 nt · forward
- Template
- DNA-001 · deleted
- PCR prerequisite
- Select an accessible, compatible template before simulation
- Import another template
- Return to Registry
- Context & next steps

**Authored actions:**

- `132:5818` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `132:5819` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `132:5888` Action / Import another template → `[{"type":"NODE","target":"110:19645","navigation":"NAVIGATE"}]`
- `132:5890` Action / Return to Registry → `[{"type":"NODE","target":"110:21795","navigation":"NAVIGATE"}]`

## ST-683 · Flow completion / HomeAfterVersion8

**Role:** Linked functional state. **Frame:** 132:5892 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=132-5892).

**Displayed content:**

- Continue your work
- DNA-001 version 8 is saved; earlier analyses retain version 7.
- Search
- New
- Record
- Version / state
- Next action
- pHelixBench-Demo · DNA-001
- v8 · saved
- Open current sequence
- Restriction analysis · RES-001
- Input v7 · older source
- Inspect provenance
- Context & next steps

**Authored actions:**

- `132:5999` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `132:6000` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `132:6060` Record / pHelixBench-Demo · DNA-001 → `[{"type":"NODE","target":"125:8508","navigation":"NAVIGATE"}]`
- `132:6064` Record / Restriction analysis · RES-001 → `[{"type":"NODE","target":"125:10993","navigation":"NAVIGATE"}]`

## ST-684 · Flow completion / NotebookRunOutput

**Role:** Linked functional state. **Frame:** 132:6266 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=132-6266).

**Displayed content:**

- Expression validation
- EXP-014 · automation results and manual draft rows remain distinct
- Search
- New
- Automation output
- R-019 · 96 submitted result rows · source and template retained
- Data group
- Rows
- State
- R-019 automation output
- 96
- Submitted
- Manual assay table
- 2
- Unsubmitted draft rows
- Open R-019 results
- Review manual draft rows
- Context & next steps

**Authored actions:**

- `132:6373` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `132:6374` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `132:6446` Action / Open R-019 results → `[{"type":"NODE","target":"128:6256","navigation":"NAVIGATE"}]`
- `132:6448` Action / Review manual draft rows → `[{"type":"NODE","target":"107:33516","navigation":"NAVIGATE"}]`

## ST-685 · Flow completion / AwaitingReview1

**Role:** Linked functional state. **Frame:** 135:5285 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=135-5285).

**Displayed content:**

- Untitled experiment
- EXP-NEW-001 · submitted revision 1 · awaiting Reviewer A
- Search
- New
- Review snapshot
- The submitted revision is read-only while review is pending.
- Item
- State
- Source
- DNA-001 · v7
- Result
- NRES-001 · submitted
- Method
- Not provided · reviewer may request changes
- Review
- REVIEW-NEW-001 · Reviewer A
- View review request
- View source
- Respond to requested changes
- Context & next steps

**Authored actions:**

- `135:5392` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `135:5393` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `135:5468` Action / View review request → `[{"type":"NODE","target":"132:5244","navigation":"NAVIGATE"}]`
- `135:5470` Action / View source → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`
- `135:5473` Action / Respond to requested changes → `[{"type":"NODE","target":"132:5270","navigation":"NAVIGATE"}]`

## ST-686 · Flow completion / AwaitingReview2

**Role:** Linked functional state. **Frame:** 135:5475 · 1600 × 1080. [Open in Figma](https://www.figma.com/design/U1kWw1K5NZBgEYkuxMz3Qb/helixbench?node-id=135-5475).

**Displayed content:**

- Untitled experiment
- EXP-NEW-001 · submitted revision 2 · awaiting Reviewer A
- Search
- New
- Review snapshot
- The submitted revision is read-only while review is pending.
- Item
- State
- Source
- DNA-001 · v7
- Result
- NRES-001 · submitted
- Method
- Sequence inspected; source version and result recorded
- Review
- REVIEW-NEW-001 · Reviewer A
- View review request
- View source
- Context & next steps

**Authored actions:**

- `135:5582` Action / Search → `[{"type":"NODE","target":"110:21012","navigation":"NAVIGATE"}]`
- `135:5583` Action / New → `[{"type":"NODE","target":"107:28979","navigation":"OVERLAY"}]`
- `135:5658` Action / View review request → `[{"type":"NODE","target":"132:5320","navigation":"NAVIGATE"}]`
- `135:5660` Action / View source → `[{"type":"NODE","target":"107:29429","navigation":"NAVIGATE"}]`

