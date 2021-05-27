//! Sigma byte stream writer
use super::constant_store::ConstantStore;
use super::val_def_type_store::ValDefTypeStore;
use sigma_ser::{peekable_reader::Peekable, vlq_encode::ReadSigmaVlqExt};
use std::io::Read;

/// Implementation of SigmaByteRead
pub struct SigmaByteReader<R> {
    inner: R,
    constant_store: ConstantStore,
    substitute_placeholders: bool,
    val_def_type_store: ValDefTypeStore,
}

impl<R: Peekable> SigmaByteReader<R> {
    /// Create new reader from PeekableReader
    pub fn new(pr: R, constant_store: ConstantStore) -> SigmaByteReader<R> {
        SigmaByteReader {
            inner: pr,
            constant_store,
            substitute_placeholders: false,
            val_def_type_store: ValDefTypeStore::new(),
        }
    }

    /// Make a new reader with underlying PeekableReader and constant_store to resolve constant
    /// placeholders
    pub fn new_with_substitute_placeholders(
        pr: R,
        constant_store: ConstantStore,
    ) -> SigmaByteReader<R> {
        SigmaByteReader {
            inner: pr,
            constant_store,
            substitute_placeholders: true,
            val_def_type_store: ValDefTypeStore::new(),
        }
    }
}

/// Sigma byte reader trait with a constant store to resolve segregated constants
pub trait SigmaByteRead: ReadSigmaVlqExt + Peekable {
    /// Constant store with constants to resolve constant placeholder types
    fn constant_store(&mut self) -> &mut ConstantStore;

    /// Option to substitute ConstantPlaceholder with Constant from the store
    fn substitute_placeholders(&self) -> bool;

    /// Set new constant store
    fn set_constant_store(&mut self, constant_store: ConstantStore);

    /// ValDef types store (resolves tpe on ValUse parsing)
    fn val_def_type_store(&mut self) -> &mut ValDefTypeStore;
}

impl<R: Read> Read for SigmaByteReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.inner.read(buf)
    }
}

impl<R: Peekable> Peekable for SigmaByteReader<R> {
    fn peek_u8(&mut self) -> Result<u8, &std::io::Error> {
        self.inner.peek_u8()
    }
}

impl<R: ReadSigmaVlqExt + Peekable> SigmaByteRead for SigmaByteReader<R> {
    fn constant_store(&mut self) -> &mut ConstantStore {
        &mut self.constant_store
    }

    fn substitute_placeholders(&self) -> bool {
        self.substitute_placeholders
    }

    fn set_constant_store(&mut self, constant_store: ConstantStore) {
        self.constant_store = constant_store;
    }

    fn val_def_type_store(&mut self) -> &mut ValDefTypeStore {
        &mut self.val_def_type_store
    }
}
