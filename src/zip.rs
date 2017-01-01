// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

use errors::Result;

use std::io::Read;
use std::path::PathBuf;
use std::path::Path;

/// An abstraction over possible Zip implementations.
///
/// The actual implementations are `ZipCommand` (uses the command zip) or
/// `ZipLibrary` (uses the `zip` library).
pub trait Zip {
    /// Write the source content to a file in the archive
    fn write_file<P: AsRef<Path>, R: Read>(&mut self, file: P, content: R) -> Result<()>;

    /// Generate the EPUB file
    fn generate(&mut self) -> Result<Vec<u8>>;
}


