#![deny(missing_docs)]

//! Crate for generating type-safe bindings to Ethereum smart contracts. This
//! crate is intended to be used either indirectly with the `ethcontract`
//! crate's `contract` procedural macro or directly from a build script.

mod contract;

use anyhow::Result;
use proc_macro2::TokenStream;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Internal global arguments passed to the generators for each individual
/// component that control expansion.
pub(crate) struct Args {
    /// The path to the truffle artifact for the contract whose bindings are
    /// being generated.
    artifact_path: PathBuf,
    /// The runtime crate name to use.
    runtime_crate_name: String,
}

impl Args {
    /// Creates a new builder given the path to a contract's truffle artifact
    /// JSON file.
    pub fn new<P>(artifact_path: P) -> Args
    where
        P: AsRef<Path>,
    {
        Args {
            artifact_path: artifact_path.as_ref().to_owned(),
            runtime_crate_name: "ethcontract".to_owned(),
        }
    }
}

/// Builder for generating contract code. Note that no code is generated until
/// the builder is finalized with `generate` or `output`.
pub struct Builder {
    /// The contract binding generation args.
    args: Args,
}

impl Builder {
    /// Creates a new builder given the path to a contract's truffle artifact
    /// JSON file.
    pub fn new<P>(artifact_path: P) -> Builder
    where
        P: AsRef<Path>,
    {
        Builder {
            args: Args::new(artifact_path),
        }
    }

    /// Sets the crate name for the runtime crate. This setting is usually only
    /// needed if the crate was renamed in the Cargo manifest.
    pub fn with_runtime_crate_name<S>(mut self, name: S) -> Builder
    where
        S: AsRef<str>,
    {
        self.args.runtime_crate_name = name.as_ref().to_owned();
        self
    }

    /// Generates the contract bindings.
    pub fn generate(self) -> Result<ContractBindings> {
        let tokens = contract::expand_contract(&self.args)?;
        Ok(ContractBindings { tokens })
    }
}

/// Type-safe contract bindings generated by a `Builder`. This type can be
/// either written to file or into a token stream for use in a procedural macro.
pub struct ContractBindings {
    /// The TokenStream representing the contract bindings.
    tokens: TokenStream,
}

impl ContractBindings {
    /// Writes the bindings to a given `Write`.
    pub fn write<W>(&self, mut w: W) -> Result<()>
    where
        W: Write,
    {
        write!(w, "{}", self.tokens)?;
        Ok(())
    }

    /// Writes the bindings to the specified file.
    pub fn write_to_file<P>(&self, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let file = File::create(path)?;
        self.write(file)
    }

    /// Converts the bindings into its underlying token stream. This allows it
    /// to be used within a procedural macro.
    pub fn into_tokens(self) -> TokenStream {
        self.tokens
    }
}