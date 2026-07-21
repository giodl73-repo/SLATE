use clap::{Parser, Subcommand};
use slate_score::DimensionScorer;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "slate")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Corpus {
        path: PathBuf,
    },
    Score {
        path: PathBuf,
    },
    #[command(name = "tier-sla")]
    TierSla {
        path: PathBuf,
    },
    Gap {
        #[arg(long)]
        scale: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Corpus { path } => {
            let text = std::fs::read_to_string(&path)?;
            let entry = slate_corpus::CorpusEntry::from_markdown(&text)?;
            println!("id: {}", entry.id);
            println!("validate: {:?}", entry.validate());
        }
        Commands::Score { path } => {
            let text = std::fs::read_to_string(&path)?;
            let entry = slate_corpus::CorpusEntry::from_markdown(&text)?;
            let scorer = slate_score::ProvisionalScorer::default();
            for dim in slate_score::Dimension::all() {
                let score = scorer.score(&entry, dim);
                println!("{}: {}", dim.code(), score.value());
            }
        }
        Commands::TierSla { path } => {
            let text = std::fs::read_to_string(&path)?;
            let entry = slate_corpus::CorpusEntry::from_markdown(&text)?;
            println!("tier: {:?}", slate_tier::classify(&entry));
            println!(
                "tier_sla_gap: {}",
                slate_tier::tier_sla_gap(&entry).is_some()
            );
        }
        Commands::Gap { scale } => {
            let scale_value = slate_corpus::Scale::parse(&scale)
                .ok_or_else(|| format!("invalid scale: {}", scale))?;
            let rubric = slate_score::Rubric::v0();
            let result = slate_gap::find_gaps(&[], &rubric, scale_value, &[], false);
            println!("null_result: {}", result.null_result);
            println!("regions: {}", result.regions.len());
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        Cli::command().debug_assert();
    }

    #[test]
    fn parses_gap() {
        assert!(Cli::try_parse_from(["slate", "gap", "--scale", "national"]).is_ok());
    }

    #[test]
    fn parses_corpus() {
        assert!(Cli::try_parse_from(["slate", "corpus", "some.md"]).is_ok());
    }
}
