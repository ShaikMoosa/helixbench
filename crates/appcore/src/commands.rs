//! One catalog supplies tool menus, owning-page entries and workspace search.
use crate::{
    actions,
    navigation::{self, NavigationIntent, Route},
    state::{AppState, Dialog, View},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandId {
    Navigate(View),
    Pcr,
    PrimerDesign,
    Digest,
    Alignment,
    Assembly,
    Crispr,
    Codon,
    Blast,
    Combinatorial,
    Registry,
    Protein,
    FeatureLibrary,
    BulkDocuments,
    GeneticCodes,
    LetterCodes,
    SearchTips,
    Rotate,
    NewEntry,
    RegisterEntity,
    ReverseComplement,
    MakeLinear,
    MakeCircular,
}
#[derive(Clone, Copy)]
pub struct CommandDescriptor {
    pub id: CommandId,
    pub label: &'static str,
    pub owner: View,
    pub keywords: &'static str,
}
#[derive(Debug, PartialEq, Eq)]
pub enum Availability {
    Available,
    Disabled(String),
}
impl Availability {
    pub fn enabled(&self) -> bool {
        matches!(self, Self::Available)
    }
    pub fn reason(&self) -> &str {
        match self {
            Self::Available => "",
            Self::Disabled(reason) => reason,
        }
    }
}
const TOOLS: &[CommandDescriptor] = &[
    CommandDescriptor {
        id: CommandId::ReverseComplement,
        label: "Reverse complement sequence",
        owner: View::Map,
        keywords: "reverse strand",
    },
    CommandDescriptor {
        id: CommandId::MakeLinear,
        label: "Set topology: linear",
        owner: View::Map,
        keywords: "dna topology",
    },
    CommandDescriptor {
        id: CommandId::MakeCircular,
        label: "Set topology: circular",
        owner: View::Map,
        keywords: "plasmid topology",
    },
    CommandDescriptor {
        id: CommandId::Pcr,
        label: "In-silico PCR",
        owner: View::Sequence,
        keywords: "amplification primers product",
    },
    CommandDescriptor {
        id: CommandId::PrimerDesign,
        label: "Primer design",
        owner: View::Primers,
        keywords: "oligonucleotide pcr design",
    },
    CommandDescriptor {
        id: CommandId::Digest,
        label: "Virtual digest & gel",
        owner: View::Enzymes,
        keywords: "restriction cut fragments",
    },
    CommandDescriptor {
        id: CommandId::Alignment,
        label: "Sequence alignment",
        owner: View::Sequence,
        keywords: "compare pairwise msa",
    },
    CommandDescriptor {
        id: CommandId::Assembly,
        label: "Assembly wizard",
        owner: View::Map,
        keywords: "gibson golden gate ligation",
    },
    CommandDescriptor {
        id: CommandId::Crispr,
        label: "CRISPR guides",
        owner: View::Sequence,
        keywords: "cas9 pam guide rna",
    },
    CommandDescriptor {
        id: CommandId::Codon,
        label: "Codon optimization",
        owner: View::Sequence,
        keywords: "expression translation",
    },
    CommandDescriptor {
        id: CommandId::Blast,
        label: "Local BLAST",
        owner: View::Sequence,
        keywords: "homology database",
    },
    CommandDescriptor {
        id: CommandId::Combinatorial,
        label: "Combinatorial assembly",
        owner: View::Map,
        keywords: "combinations library construct",
    },
    CommandDescriptor {
        id: CommandId::Registry,
        label: "Local registry",
        owner: View::Registry,
        keywords: "catalog plasmid strain primer",
    },
    CommandDescriptor {
        id: CommandId::Protein,
        label: "Protein analysis",
        owner: View::Biologics,
        keywords: "amino acid molecular weight",
    },
    CommandDescriptor {
        id: CommandId::FeatureLibrary,
        label: "Feature library",
        owner: View::Features,
        keywords: "annotation reusable motif",
    },
    CommandDescriptor {
        id: CommandId::BulkDocuments,
        label: "Workspace / bulk documents",
        owner: View::Home,
        keywords: "tag export close",
    },
    CommandDescriptor {
        id: CommandId::GeneticCodes,
        label: "Genetic codes",
        owner: View::Sequence,
        keywords: "translation codon table",
    },
    CommandDescriptor {
        id: CommandId::LetterCodes,
        label: "Letter codes",
        owner: View::Sequence,
        keywords: "iupac ambiguity nucleotide",
    },
    CommandDescriptor {
        id: CommandId::SearchTips,
        label: "Sequence search tips",
        owner: View::Sequence,
        keywords: "find help",
    },
    CommandDescriptor {
        id: CommandId::Rotate,
        label: "Rotate circular sequence",
        owner: View::Map,
        keywords: "origin position plasmid",
    },
    CommandDescriptor {
        id: CommandId::NewEntry,
        label: "New experiment entry",
        owner: View::Notebook,
        keywords: "experiment notes",
    },
    CommandDescriptor {
        id: CommandId::RegisterEntity,
        label: "Register entity",
        owner: View::Registry,
        keywords: "create plasmid strain primer",
    },
];
pub fn tools() -> &'static [CommandDescriptor] {
    TOOLS
}
pub fn owning_page(view: View) -> Vec<CommandDescriptor> {
    TOOLS.iter().filter(|d| d.owner == view).copied().collect()
}
pub fn catalog() -> Vec<CommandDescriptor> {
    let mut entries = TOOLS.to_vec();
    entries.extend(
        View::ALL
            .into_iter()
            .chain(View::LAB)
            .map(|view| CommandDescriptor {
                id: CommandId::Navigate(view),
                label: view.label(),
                owner: view,
                keywords: "open page navigate",
            }),
    );
    entries
}
pub fn availability(app: &AppState, id: CommandId) -> Availability {
    let reason = if app.navigation.pending.is_some() {
        Some("Answer the unsaved-work decision first")
    } else if app.dialog.is_some() {
        Some("Apply or cancel the open form first")
    } else {
        match id {
            CommandId::Assembly | CommandId::Combinatorial if app.library.is_empty() => {
                Some("Open at least one additional DNA document for an insert")
            }
            CommandId::Rotate if !app.doc.sequence.is_circular() => {
                Some("Open a circular DNA document to rotate its origin")
            }
            CommandId::Pcr
            | CommandId::PrimerDesign
            | CommandId::Digest
            | CommandId::Alignment
            | CommandId::Assembly
            | CommandId::Crispr
            | CommandId::Codon
            | CommandId::Blast
            | CommandId::Combinatorial
            | CommandId::Rotate
                if app.doc.sequence.is_empty() =>
            {
                Some("Import or create a DNA sequence first")
            }
            _ => None,
        }
    };
    match reason {
        Some(reason) => Availability::Disabled(reason.into()),
        None => Availability::Available,
    }
}
pub fn execute(app: &mut AppState, id: CommandId) -> Result<(), String> {
    if let Availability::Disabled(reason) = availability(app, id) {
        return Err(reason);
    }
    crate::workspace_search::depart(app);
    match id {
        CommandId::Navigate(View::Results) => crate::results::browse(app)?,
        CommandId::Navigate(view) => {
            navigation::request_navigation(
                app,
                NavigationIntent::Navigate(Route {
                    view,
                    document: app.doc.id,
                    record: None,
                }),
            )?;
        }
        CommandId::Pcr => actions::open_pcr(app),
        CommandId::PrimerDesign => actions::open_primer_design(app),
        CommandId::Digest => actions::open_digest(app),
        CommandId::Alignment => actions::open_alignment(app),
        CommandId::Assembly => actions::open_assembly(app),
        CommandId::Crispr => actions::open_crispr(app),
        CommandId::Codon => actions::open_codon(app),
        CommandId::Blast => actions::open_blast(app),
        CommandId::Combinatorial => actions::open_combinatorial(app),
        CommandId::Registry => actions::open_registry(app),
        CommandId::Protein => actions::open_protein(app),
        CommandId::FeatureLibrary => actions::open_feature_lib(app),
        CommandId::BulkDocuments => actions::open_workspace(app),
        CommandId::GeneticCodes => {
            app.dialog = Some(Dialog::GeneticCodes {
                selected: app.doc.default_genetic_code,
            })
        }
        CommandId::LetterCodes => app.dialog = Some(Dialog::LetterCodes),
        CommandId::SearchTips => app.dialog = Some(Dialog::SearchTips),
        CommandId::Rotate => {
            app.dialog = Some(Dialog::Rotate {
                position: "1".into(),
                error: None,
            })
        }
        CommandId::NewEntry => actions::open_entry_editor(app, None, 0),
        CommandId::RegisterEntity => actions::open_registry_editor(app, 0, None),
        CommandId::ReverseComplement => actions::reverse_complement_sequence(app),
        CommandId::MakeLinear => actions::set_topology(app, domain::Topology::Linear),
        CommandId::MakeCircular => actions::set_topology(app, domain::Topology::Circular),
    }
    Ok(())
}
