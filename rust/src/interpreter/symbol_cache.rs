//! What a written name dispatches to, remembered per name.
//!
//! Every Symbol the token walk meets was dispatched from its text: the name
//! folded to its canonical spelling (a scan of its bytes, and an allocation
//! when it held a lowercase letter), hashed, and probed against the Core
//! dictionary, whose key was then compared byte by byte — before a single
//! operand was looked at. A program writes a handful of names many times
//! over, so the answer is kept in a small table keyed by the name token's
//! own text (`types::Name`), where finding it again is a hash of one machine
//! word and a compare of a few.
//!
//! Only what depends on the name alone is kept. Core is sealed — it is
//! registered once from the generated registry and never changes after
//! (`builtins::register_builtins`; a session reset re-registers the same
//! Words and clears this) — so whether a name is a Core Word, and which, is a
//! function of its text. Bindings and User Words change as the program runs,
//! so a name that is not a Core Word is still looked up in them on every
//! dispatch, by its canonical name, exactly as before.
//!
//! The table is direct-mapped: a name that loses its slot to another is
//! simply resolved again.

use crate::kernel::generated::GeneratedWord;
use crate::types::Name;

use super::Interpreter;

/// What a name token dispatches to, as far as its text decides.
#[derive(Clone, Debug)]
pub(crate) enum Meaning {
    /// A Core Word whose definition is its registry entry (empty body), the
    /// case every Core Word is.
    Core(&'static GeneratedWord),
    /// Not a Core Word: a binding, a User Word, or nothing. `folded` is the
    /// canonical name when it differs from the token's own text.
    Named { folded: Option<Name> },
    /// A Core name whose definition is not a plain registry entry. Never
    /// built by the registry; dispatched by the general route, by its
    /// canonical name, in case a dictionary ever holds one.
    General { folded: Option<Name> },
}

const SLOTS: usize = 128;

/// A name token's text, and what it dispatches to.
type Slot = Option<(Name, Meaning)>;

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
    fn index(name: &Name) -> usize {
        (name.quick_hash() >> 57) as usize % SLOTS
    }
}

impl Interpreter {
    /// Dispatch the name token `name`: `dispatch_word` on its canonical
    /// name, with what its text decides already decided.
    ///
    /// The two common answers — a Core Word, and a name already in its
    /// canonical spelling — are taken straight from the slot; anything else,
    /// a name not yet in the table among them, goes the long way round.
    #[inline]
    pub(crate) fn dispatch_symbol(&mut self, name: &Name) -> crate::error::Result<()> {
        if let Some((held, meaning)) = &self.symbol_cache.slots[SymbolCache::index(name)] {
            if held == name {
                match meaning {
                    Meaning::Core(word) => {
                        let word = *word;
                        return self.dispatch_core_word(word);
                    }
                    Meaning::Named { folded: None } => return self.dispatch_word_beyond_core(name),
                    _ => {}
                }
            }
        }
        self.dispatch_symbol_by_meaning(name)
    }

    #[inline(never)]
    fn dispatch_symbol_by_meaning(&mut self, name: &Name) -> crate::error::Result<()> {
        match self.symbol_meaning(name) {
            Meaning::Core(word) => self.dispatch_core_word(word),
            Meaning::Named { folded } => {
                self.dispatch_word_beyond_core(folded.as_ref().unwrap_or(name))
            }
            Meaning::General { folded } => self.dispatch_word(folded.as_ref().unwrap_or(name)),
        }
    }

    /// What the name token `name` dispatches to, as far as its text decides,
    /// resolved and kept in its slot when the slot holds another name.
    fn symbol_meaning(&mut self, name: &Name) -> Meaning {
        let index = SymbolCache::index(name);
        if let Some((held, meaning)) = &self.symbol_cache.slots[index] {
            if held == name {
                return meaning.clone();
            }
        }
        let meaning = self.resolve_symbol_meaning(name);
        self.symbol_cache.slots[index] = Some((name.clone(), meaning.clone()));
        meaning
    }

    fn resolve_symbol_meaning(&self, name: &str) -> Meaning {
        let canonical = crate::word_name::canonical_word_name(name);
        let core = self.core_vocabulary.get(canonical.as_ref());
        let folded = match canonical {
            std::borrow::Cow::Owned(folded) => Some(Name::from(folded)),
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
}
