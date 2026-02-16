//! Build orchestration tool for stm32f4xx-hal examples.
//!
//! Provides a CLI for building and running board-specific examples with
//! the correct feature flags.
//!
//! # Usage
//!
//! ```bash
//! # Build an example for a specific board
//! cargo xtask run-example --board f469disco --example f469disco-lcd-test
//!
//! # Check that all board/example combinations compile
//! cargo xtask check-all
//! ```

use clap::{Parser, Subcommand};
use std::process::{Command, ExitCode};

/// Board configuration: name, MCU feature, and additional feature flags.
struct BoardConfig {
    name: &'static str,
    mcu: &'static str,
    features: &'static [&'static str],
    chip: &'static str,
    /// Legacy examples in the top-level examples/ directory
    examples: &'static [&'static str],
    /// Binary targets in the boards/<name>/ crate
    board_bins: &'static [&'static str],
}

const BOARDS: &[BoardConfig] = &[
    BoardConfig {
        name: "f469disco",
        mcu: "stm32f469",
        features: &["stm32-fmc", "defmt", "framebuffer"],
        chip: "STM32F469NIHx",
        examples: &["f469disco-lcd-test", "stm32f469i_disco_screen", "fmc-sdram"],
        board_bins: &["lcd-framebuffer"],
    },
    BoardConfig {
        name: "f413disco",
        mcu: "stm32f413",
        features: &["fsmc_lcd", "defmt"],
        chip: "STM32F413ZHTx",
        examples: &["f413disco-lcd-ferris"],
        board_bins: &["st7789-fsmc"],
    },
    BoardConfig {
        name: "f429disco",
        mcu: "stm32f429",
        features: &["stm32-fmc"],
        chip: "STM32F429ZITx",
        examples: &["ltdc-screen"],
        board_bins: &["ltdc-framebuffer"],
    },
];

#[derive(Parser)]
#[command(name = "xtask", about = "Build orchestration for stm32f4xx-hal")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build and optionally run an example for a specific board.
    RunExample {
        /// Target board (e.g. f469disco, f413disco, f429disco)
        #[arg(long)]
        board: String,

        /// Example name (e.g. f469disco-lcd-test)
        #[arg(long)]
        example: String,

        /// Run the example on hardware instead of just building
        #[arg(long, default_value_t = false)]
        run: bool,
    },

    /// Check that all known board/example combinations compile.
    CheckAll,

    /// List all known boards and their examples.
    ListBoards,
}

fn find_board(name: &str) -> Option<&'static BoardConfig> {
    BOARDS.iter().find(|b| b.name == name)
}

fn features_string(board: &BoardConfig) -> String {
    let mut feats = vec![board.mcu.to_string()];
    feats.extend(board.features.iter().map(|f| f.to_string()));
    feats.join(",")
}

fn project_root() -> std::path::PathBuf {
    let dir = std::env::var("CARGO_MANIFEST_DIR")
        .unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_string());
    std::path::Path::new(&dir)
        .parent()
        .expect("xtask should be in a subdirectory")
        .to_path_buf()
}

fn run_example(board_name: &str, example: &str, run: bool) -> bool {
    let board = match find_board(board_name) {
        Some(b) => b,
        None => {
            eprintln!(
                "error: unknown board '{}'. Known boards: {}",
                board_name,
                BOARDS.iter().map(|b| b.name).collect::<Vec<_>>().join(", ")
            );
            return false;
        }
    };

    let features = features_string(board);
    let verb = if run { "run" } else { "build" };

    println!(
        "▶ cargo {} --example {} --features={} --release",
        verb, example, features
    );

    let mut cmd = Command::new("cargo");
    cmd.arg(verb)
        .arg("--example")
        .arg(example)
        .arg(format!("--features={}", features))
        .arg("--release")
        .current_dir(project_root());

    if run {
        cmd.env(
            "PROBE_RS_CHIP",
            board.chip,
        );
    }

    match cmd.status() {
        Ok(status) => status.success(),
        Err(e) => {
            eprintln!("error: failed to execute cargo: {}", e);
            false
        }
    }
}

fn check_all() -> bool {
    let mut all_ok = true;

    for board in BOARDS {
        let features = features_string(board);
        println!("── Checking board: {} (features: {}) ──", board.name, features);

        // Check legacy examples in top-level examples/ directory
        for example in board.examples {
            print!("  [example] {} ... ", example);

            let status = Command::new("cargo")
                .arg("check")
                .arg("--example")
                .arg(example)
                .arg(format!("--features={}", features))
                .current_dir(project_root())
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::piped())
                .status();

            match status {
                Ok(s) if s.success() => println!("ok"),
                Ok(_) => {
                    println!("FAILED");
                    all_ok = false;
                }
                Err(e) => {
                    println!("ERROR: {}", e);
                    all_ok = false;
                }
            }
        }

        // Check board-crate binaries in boards/<name>/
        let board_dir = project_root().join("boards").join(board.name);
        if board_dir.exists() {
            for bin in board.board_bins {
                print!("  [board]   {} ... ", bin);

                let status = Command::new("cargo")
                    .arg("check")
                    .arg("--bin")
                    .arg(bin)
                    .current_dir(&board_dir)
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::piped())
                    .status();

                match status {
                    Ok(s) if s.success() => println!("ok"),
                    Ok(_) => {
                        println!("FAILED");
                        all_ok = false;
                    }
                    Err(e) => {
                        println!("ERROR: {}", e);
                        all_ok = false;
                    }
                }
            }
        }
    }

    all_ok
}

fn list_boards() {
    for board in BOARDS {
        println!("{}:", board.name);
        println!("  MCU:      {}", board.mcu);
        println!("  Features: {}", board.features.join(", "));
        println!("  Chip:     {}", board.chip);
        println!("  Examples (legacy):");
        for ex in board.examples {
            println!("    - {}", ex);
        }
        if !board.board_bins.is_empty() {
            println!("  Board binaries:");
            for bin in board.board_bins {
                println!("    - {}", bin);
            }
        }
        println!();
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let success = match cli.command {
        Commands::RunExample {
            board,
            example,
            run,
        } => run_example(&board, &example, run),
        Commands::CheckAll => check_all(),
        Commands::ListBoards => {
            list_boards();
            true
        }
    };

    if success {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
