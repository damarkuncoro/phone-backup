use anyhow::Result;
use audio_engine::AudioPipelineBuilder;
use clap::{Args, Subcommand};
use std::fs;

#[derive(Args, Debug)]
pub struct AudioArgs {
    #[command(subcommand)]
    pub command: AudioCommands,
}

#[derive(Subcommand, Debug)]
pub enum AudioCommands {
    /// Inspect audio file metadata, tags, and category
    Inspect {
        /// Audio file path
        path: String,
    },
    /// Generate 100-point normalized waveform peak ASCII visualizer
    Waveform {
        /// Audio file path
        path: String,
    },
}

pub fn handle_audio(args: AudioArgs) -> Result<()> {
    match args.command {
        AudioCommands::Inspect { path } => {
            let bytes = fs::read(&path)?;
            let pipeline = AudioPipelineBuilder::new();
            let processed = pipeline.process(&path, &bytes)?;

            println!("Audio File Analysis: {}", path);
            println!("----------------------------------");
            println!("Format:   {:?}", processed.format);
            println!("Category: {}", processed.category.label());
            if let Some(t) = processed.metadata.title {
                println!("Title:    {}", t);
            }
            if let Some(a) = processed.metadata.artist {
                println!("Artist:   {}", a);
            }
            if let Some(call) = processed.call_info {
                println!("Call Info: Number={:?}, Direction={:?}", call.phone_number, call.direction);
            }
        }
        AudioCommands::Waveform { path } => {
            let bytes = fs::read(&path)?;
            let pipeline = AudioPipelineBuilder::new().with_waveform_points(60);
            let processed = pipeline.process(&path, &bytes)?;

            println!("Waveform Peak Envelope (60 points):");
            for peak in &processed.waveform.points {
                let bar_len = (peak / 5) as usize;
                let bar: String = "█".repeat(bar_len);
                println!("{:3}% | {}", peak, bar);
            }
        }
    }
    Ok(())
}
