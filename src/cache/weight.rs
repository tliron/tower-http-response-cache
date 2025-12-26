use kutil::{http::*, std::foster::*};

//
// CacheWeight
//

/// Cache weight.
pub trait CacheWeight {
    /// Cache weight as a byte count.
    ///
    /// It is *not* the amount of memory used, but rather an indicator of *potential* storage
    /// requirements.
    ///
    /// Its intended use is for apples-to-apples comparisons, e.g. to find out which of two items
    /// of the same type weighs more. But even then it may be misleading in some cases, e.g. if
    /// storage involves compression then the "heavier" item might end up taking less storage then
    /// the "lighter" item.
    ///
    /// Note that *sums* of weights can be especially misleading in terms of memory use because
    /// there might be memory shared between items, e.g. via the use of
    /// [ImmutableBytes](kutil::std::immutable::ImmutableBytes).
    fn cache_weight(&self) -> usize;
}

impl CacheWeight for ETag {
    fn cache_weight(&self) -> usize {
        const SELF_SIZE: usize = size_of::<ETag>();
        SELF_SIZE + self.tag.len()
    }
}

impl CacheWeight for Language {
    fn cache_weight(&self) -> usize {
        const SELF_SIZE: usize = size_of::<Language>();
        let mut size = SELF_SIZE;
        for subtag in &self.0 {
            size += subtag.len();
        }
        size
    }
}

impl CacheWeight for MediaType {
    fn cache_weight(&self) -> usize {
        const SELF_SIZE: usize = size_of::<MediaType>();
        SELF_SIZE + self.main.cache_weight() + self.subtype.cache_weight()
    }
}

impl CacheWeight for MediaTypeSegment {
    fn cache_weight(&self) -> usize {
        const SELF_SIZE: usize = size_of::<MediaTypeSegment>();
        SELF_SIZE + self.0.len()
    }
}

impl CacheWeight for MediaTypeSelector {
    fn cache_weight(&self) -> usize {
        const SELF_SIZE: usize = size_of::<MediaTypeSelector>();
        SELF_SIZE + self.main.cache_weight() + self.subtype.cache_weight()
    }
}

impl<SelectionT> CacheWeight for Selector<SelectionT>
where
    SelectionT: CacheWeight,
{
    fn cache_weight(&self) -> usize {
        let mut size = size_of::<Self>();
        if let Self::Specific(selection) = self {
            size += selection.cache_weight();
        }
        size
    }
}
