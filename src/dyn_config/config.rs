use std::fmt::{Display, Formatter, Write};

use crate::{
    DisplayLua, DynArray, DynArrayMut, DynArrayRef, DynTable, DynTableMut, DynTableRef, Value,
};

#[cfg(feature = "bin")]
use crate::{BinConfigWriter, BinConfigWriterError};

#[cfg(feature = "ini")]
use crate::{ini::dyn_config_from_ini, DisplayINI, IniError, IniOptions, ToINIStringError};

/// Represents a mutable config with a root hashmap [`table`].
///
/// [`table`]: struct.DynTable.html
pub struct DynConfig(DynTable);

impl DynConfig {
    /// Creates a new [`config`] with an empty root [`table`].
    ///
    /// [`config`]: struct.DynConfig.html
    /// [`table`]: struct.DynTable.html
    pub fn new() -> Self {
        Self(DynTable::new())
    }

    /// Returns the immutable reference to the root [`table`] of the [`config`].
    ///
    /// [`table`]: struct.DynTable.html
    /// [`config`]: struct.DynConfig.html
    pub fn root(&self) -> DynTableRef<'_> {
        DynTableRef::new(&self.0)
    }

    /// Returns the mutable reference to the root [`table`] of the [`config`].
    ///
    /// [`table`]: struct.DynTable.html
    /// [`config`]: struct.DynConfig.html
    pub fn root_mut(&mut self) -> DynTableMut<'_> {
        DynTableMut::new(&mut self.0)
    }

    /// Tries to serialize this [`config`] to a [`binary config`].
    ///
    /// [`config`]: struct.LuaConfig.html
    /// [`binary config`]: struct.BinConfig.html
    #[cfg(feature = "bin")]
    pub fn to_bin_config(&self) -> Result<Box<[u8]>, BinConfigWriterError> {
        use BinConfigWriterError::*;

        let root = self.root();

        // The root table is empty - nothing to do.
        if root.len() == 0 {
            return Err(EmptyRootTable);
        }

        let mut writer = BinConfigWriter::new(root.len())?;

        Self::table_to_bin_config(root, &mut writer)?;

        writer.finish()
    }

    /// Creates a new [`config`] from the INI `string` using default [`options`].
    ///
    /// [`config`]: struct.DynConfig.html
    /// [`options`]: struct.IniOptions.html
    #[cfg(feature = "ini")]
    pub fn from_ini(string: &str) -> Result<Self, IniError> {
        dyn_config_from_ini(string, IniOptions::default())
    }

    /// Creates a new [`config`] from the INI `string` using custom [`options`].
    ///
    /// [`config`]: struct.DynConfig.html
    /// [`options`]: struct.IniOptions.html
    #[cfg(feature = "ini")]
    pub fn from_ini_opts(string: &str, options: IniOptions) -> Result<Self, IniError> {
        dyn_config_from_ini(string, options)
    }

    /// Tries to serialize this [`config`] to a Lua script string.
    ///
    /// NOTE: you may also call `to_string` via the [`config`]'s `Display` implementation.
    ///
    /// [`config`]: struct.DynConfig.html
    pub fn to_lua_string(&self) -> Result<String, std::fmt::Error> {
        let mut result = String::new();

        write!(&mut result, "{}", self)?;

        Ok(result)
    }

    /// Tries to serialize this [`config`] to an INI string.
    ///
    /// [`config`]: struct.DynConfig.html
    #[cfg(feature = "ini")]
    pub fn to_ini_string(&self) -> Result<String, ToINIStringError> {
        let mut result = String::new();

        self.root().fmt_ini(&mut result, 0)?;

        Ok(result)
    }

    #[cfg(feature = "bin")]
    fn table_to_bin_config(
        table: DynTableRef<'_>,
        writer: &mut BinConfigWriter,
    ) -> Result<(), BinConfigWriterError> {
        // Gather the keys.
        let mut key_strins: Vec<_> = table.iter().map(|(key, _)| key).collect();

        // Sort the keys in alphabetical order.
        key_strins.sort_by(|l, r| l.cmp(r));

        // Iterate the table using the sorted keys.
        for key_string in key_strins.into_iter() {
            // Must succeed.
            let value = table.get(key_string).unwrap();

            Self::value_to_bin_config(Some(key_string), value, writer)?;
        }

        Ok(())
    }

    #[cfg(feature = "bin")]
    fn array_to_bin_config(
        array: DynArrayRef<'_>,
        writer: &mut BinConfigWriter,
    ) -> Result<(), BinConfigWriterError> {
        // Iterate the array in order.
        for value in array.iter() {
            Self::value_to_bin_config(None, value, writer)?;
        }

        Ok(())
    }

    #[cfg(feature = "bin")]
    fn value_to_bin_config(
        key: Option<&str>,
        value: Value<&'_ str, DynArrayRef<'_>, DynTableRef<'_>>,
        writer: &mut BinConfigWriter,
    ) -> Result<(), BinConfigWriterError> {
        use Value::*;

        match value {
            Bool(value) => {
                writer.bool(key, value)?;
            }
            I64(value) => {
                writer.i64(key, value)?;
            }
            F64(value) => {
                writer.f64(key, value)?;
            }
            String(value) => {
                writer.string(key, value.as_ref())?;
            }
            Array(value) => {
                writer.array(key, value.len())?;
                Self::array_to_bin_config(value, writer)?;
                writer.end()?;
            }
            Table(value) => {
                writer.table(key, value.len())?;
                Self::table_to_bin_config(value, writer)?;
                writer.end()?;
            }
        }

        Ok(())
    }
}

impl Display for DynConfig {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        self.root().fmt_lua(f, 0)
    }
}

impl<'a> Display for Value<&'a str, DynArray, DynTable> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        self.fmt_lua(f, 0)
    }
}

impl<'a> Display for Value<&'a str, DynArrayRef<'a>, DynTableRef<'a>> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        self.fmt_lua(f, 0)
    }
}

impl<'a> Display for Value<&'a str, DynArrayMut<'a>, DynTableMut<'a>> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        self.fmt_lua(f, 0)
    }
}
