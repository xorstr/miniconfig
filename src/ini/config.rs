use crate::*;

/// A trait which represents the config being filled by the [`.ini parser`](struct.IniParser.html)
/// during the call to [`parse`](struct.IniParser.html#method.parse).
/// Handles the events generated by the [`.ini parser`](struct.IniParser.html).
pub trait IniConfig<'s> {
    /// Returns `Ok(_)` if the current section already contains the `key`.
    /// The returned result value is `true` if the value is a section, `false` otherwise.
    /// Else returns an `Err(())` if the current section does not contain the `key`.
    ///
    /// NOTE - this is necessary because the [`.ini parser`](struct.IniParser.html) does not keep track internally of all previously parsed keys.
    fn contains_key(&self, key: NonEmptyIniStr<'s, '_>) -> Result<bool, ()>;

    /// Adds the `key` / `value` pair to the current section.
    ///
    /// If `overwrite` is `true`, the key is duplicate (i.e. [`contains_key`](#method.contains_key)
    /// previously returned `Ok(_)` for this `key`)
    /// and the parser is [`configured`](enum.IniDuplicateKeys.html)
    /// to [`overwrite`](enum.IniDuplicateKeys.html#variant.Last) the `key`.
    /// If `overwrite` is `false`, the `key` / `value` pair is added for the first time.
    fn add_value(&mut self, key: NonEmptyIniStr<'s, '_>, value: IniValue<'s, '_>, overwrite: bool);

    /// Adds the `section` to the current section and makes it the current section for the following calls to
    /// [`contains_key`](#method.contains_key), [`add_value`](#method.add_value), [`start_array`](#method.start_array),
    /// [`end_section`](#method.end_section).
    /// Pushes it onto the LIFO stack of sections.
    ///
    /// If `overwrite` is `true`, the section is duplicate (i.e. [`contains_key`](#method.contains_key)
    /// previously returned `Ok(_)` for this `section`)
    /// and the parser is [`configured`](enum.IniDuplicateSections.html)
    /// to [`overwrite`](enum.IniDuplicateSections.html#variant.Last) the `section`.
    /// If `overwrite` is `false`, the `section` is added for the first time,
    /// or it is the parent section of a nested section (if the parser is [`configured`](struct.IniParser.html#method.nested_section_depth) to support nested sections).
    ///
    /// Will be eventually followed by a call to [`end_section`](#method.end_section) with the same `section` name.
    fn start_section(&mut self, section: NonEmptyIniStr<'s, '_>, overwrite: bool);

    /// Finishes the current `section`, started by the preceding call to [`start_section`](#method.start_section) with the same `section` name,
    /// popping it off the LIFO stack of sections,
    /// making the previous section (if any, or the root section) the new current section for the following calls to
    /// [`contains_key`](#method.contains_key), [`add_value`](#method.add_value), [`start_section`](#method.start_section), [`start_array`](#method.start_array).
    fn end_section(&mut self, section: NonEmptyIniStr<'s, '_>);

    /// Adds an empty `array` to the current section, making it the current array for the following calls to
    /// [`add_array_value`](#method.add_array_value), [`end_array`](#method.end_array).
    ///
    /// If `overwrite` is `true`, the `array` key is duplicate (i.e. [`contains_key`](#method.contains_key)
    /// previously returned `Ok(_)` for this `array`) and the parser is [`configured`](enum.IniDuplicateKeys.html)
    /// to [`overwrite`](enum.IniDuplicateKeys.html#variant.Last) the `path`.
    /// If `overwrite` is `false`, the array is added for the first time.
    ///
    /// Will be eventually followed by a call to [`end_array`](#method.end_array) with the same `array` name.
    fn start_array(&mut self, array: NonEmptyIniStr<'s, '_>, overwrite: bool);

    /// Adds a new `value` to the current array.
    /// `value` is guaranteed to be of valid type (i.e. not mixed) for the array.
    fn add_array_value(&mut self, value: IniValue<'s, '_>);

    /// Finishes the current `array`, started by the preceding call to [`start_array`](#method.start_array) with the same `array` name.
    fn end_array(&mut self, array: NonEmptyIniStr<'s, '_>);
}
