pub use builtin::*;
pub use fankor::*;
use std::borrow::Cow;
use std::ops::{Deref, DerefMut};

mod builtin;
mod fankor;

pub struct TsTypesCache(pub Vec<(Cow<'static, str>, Cow<'static, str>)>);

impl TsTypesCache {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn new() -> TsTypesCache {
        TsTypesCache(Vec::new())
    }

    // METHODS ----------------------------------------------------------------

    pub fn contains_key(&self, key: &str) -> bool {
        self.0.iter().any(|(k, _)| k == key)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut Cow<'static, str>> {
        self.0.iter_mut().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    /// # Safety
    /// It does not assert duplicated keys.
    pub fn insert(&mut self, key: Cow<'static, str>, value: Cow<'static, str>) {
        self.0.push((key, value));
    }
}

impl AsRef<Vec<(Cow<'static, str>, Cow<'static, str>)>> for TsTypesCache {
    fn as_ref(&self) -> &Vec<(Cow<'static, str>, Cow<'static, str>)> {
        &self.0
    }
}

impl Deref for TsTypesCache {
    type Target = Vec<(Cow<'static, str>, Cow<'static, str>)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TsTypesCache {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for TsTypesCache {
    fn default() -> Self {
        Self::new()
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

pub trait TsTypeGen {
    // METHODS ----------------------------------------------------------------

    /// Gets the value of the type.
    fn value(&self) -> Cow<'static, str>;

    // STATIC METHODS ---------------------------------------------------------

    /// Gets the type of the value.
    fn value_type() -> Cow<'static, str>;

    /// Gets the schema name.
    fn schema_name() -> Cow<'static, str>;

    /// Generates the equivalent TypeScript type definition and returns the
    /// generated type name.
    #[allow(unused_variables)]
    fn generate_type(registered_types: &mut TsTypesCache) -> Cow<'static, str> {
        Self::value_type()
    }

    /// Generates the TypeScript schema of the type and returns the expression
    /// to access the schema.
    #[allow(unused_variables)]
    fn generate_schema(registered_schemas: &mut TsTypesCache) -> Cow<'static, str> {
        Self::schema_name()
    }
}

impl<T: TsTypeGen> TsTypeGen for Box<T> {
    fn value(&self) -> Cow<'static, str> {
        T::value(self)
    }

    fn value_type() -> Cow<'static, str> {
        T::value_type()
    }

    fn schema_name() -> Cow<'static, str> {
        T::schema_name()
    }

    fn generate_type(registered_types: &mut TsTypesCache) -> Cow<'static, str> {
        T::generate_type(registered_types)
    }

    fn generate_schema(registered_schemas: &mut TsTypesCache) -> Cow<'static, str> {
        T::generate_schema(registered_schemas)
    }
}
