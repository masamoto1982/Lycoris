//! What a written name dispatches to, remembered per name token.
//!
//! Every Symbol the token walk meets was dispatched from its text: the name
//! folded to its canonical spelling (a scan of its bytes, and an allocation
//! when it held a lowercase letter), hashed, and probed against the Core
//! dictionary, whose key was then compared byte by byte — before a single
//! operand was looked at. A program writes a handful of names many times
//! over, and the tokenizer hands every repeat of a lexeme the *same*
//! `Arc<str>` (`tokenizer::LexemeMemo`), so the answer can be kept beside
//! that allocation and found again by its address.
//!
//! Only what depends on the name alone is kept. Core is sealed — it is
//! registered once from the generated registry and never changes after
//! (`builtins::register_builtins`; a session reset re-registers the same
//! Words and clears this) — so whether a name is a Core Word, and which, is a
//! function of its text. Bindings and User Words change as the program runs,
//! so a name that is not a Core Word is still looked up in them on every
//! dispatch, by its canonical name, exactly as before.
//!
//! A slot holds the name's `Arc` itself, so the address it is found by
//! cannot be freed and handed to a different name while the slot remembers
//! it. The table is small and direct-mapped: a name that loses its slot is
//! simply resolved again.

use std::sync::Arc;

use crate::kernel::generated::GeneratedWord;

use super::Interpreter;

/// What a name token dispatches to, as far as its text decides.
#[derive(Clone, Debug)]
pub(crate) enum Meaning {
    /// A Core Word whose definition is its registry entry (empty body), the
    /// case every Core Word is.
    Core(&'static GeneratedWord),
    /// Not a Core Word: a binding, a User Word, or nothing. `folded` is the
    /// canonical name when it differs from the token's own text.
    Named { folded: Option<Arc<str>> },
    /// A Core name whose definition is not plain registry entry. Never built
    /// by the registry; dispatched by the general route, by its canonical
    /// name, in case a dictionary ever holds one.
    General { folded: Option<Arc<str>> },
}

const SLOTS: usize = 128;

/// A name token, by its own `Arc`, and what it dispatches to.
type Slot = Option<(Arc<str>, Meaning)>;

#[derive(Debug)]
pub(crate) struct SymbolCache {
    slots: Box<[Slot]>,
}

impl Default for SymbolCache {
    fn default() -> Self {
        Self {
            slots: vec![None; SLOTS].into_boxed_slice(),
        }
    }
}

impl SymbolCache {
    pub(crate) fn clear(&mut self) {
        self.slots.iter_mut().for_each(|slot| *slot = None);
    }

    #[inline]
    fn index(name: &Arc<str>) -> usize {
        // An `Arc` allocation is at least word-aligned, so the low bits carry
        // nothing; two shifts mix the bits above them into the index.
        let address = Arc::as_ptr(name) as *const u8 as usize;
        ((address >> 4) ^ (address >> 11)) % SLOTS
    }
}

impl Interpreter {
    /// What the name token `name` dispatches to, as far as its text decides.
    #[inline]
    pub(crate) fn symbol_meaning(&mut self, name: &Arc<str>) -> Meaning {
        let index = SymbolCache::index(name);
        if let Some((held, meaning)) = &self.symbol_cache.slots[index] {
            if Arc::ptr_eq(held, name) {
                return meaning.clone();
            }
        }
        let meaning = self.resolve_symbol_meaning(name);
        self.symbol_cache.slots[index] = Some((Arc::clone(name), meaning.clone()));
        meaning
    }

    fn resolve_symbol_meaning(&self, name: &str) -> Meaning {
        let canonical = crate::word_name::canonical_word_name(name);
        let core = self.core_vocabulary.get(canonical.as_ref());
        let folded = match canonical {
            std::borrow::Cow::Owned(folded) => Some(Arc::from(folded)),
            std::borrow::Cow::Borrowed(_) => None,
        };
        match core {
            None => Meaning::Named { folded },
            Some(def) => match def.generated {
                Some(word) if def.body.is_empty() => Meaning::Core(word),
                _ => Meaning::General { folded },
            },
        }
    }

    /// Dispatch the name token `name`: `dispatch_word` on its canonical
    /// name, with what its text decides already decided.
    #[inline]
    pub(crate) fn dispatch_symbol(&mut self, name: &Arc<str>) -> crate::error::Result<()> {
        match self.symbol_meaning(name) {
            Meaning::Core(word) => self.dispatch_core_word(word),
            Meaning::Named { folded } => {
                self.dispatch_word_beyond_core(folded.as_deref().unwrap_or(name))
            }
            Meaning::General { folded } => self.dispatch_word(folded.as_deref().unwrap_or(name)),
        }
    }
}
