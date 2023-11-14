pub use semantic::Semantic;

use crate::UniffiCustomTypeConverter;

pub(crate) mod semantic;

#[derive(Debug, Clone)]
pub struct Embedding(pub Vec<f32>);

impl Embedding {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, f32> {
        self.0.iter()
    }
}

impl UniffiCustomTypeConverter for Embedding {
    type Builtin = Vec<f32>;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok(Embedding(val))
    }

    // Convert our custom type to Builtin
    fn from_custom(obj: Self) -> Self::Builtin {
        obj.0
    }
}