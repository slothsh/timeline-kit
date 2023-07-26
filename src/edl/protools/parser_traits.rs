
// Copyright (C) Stefan Olivier
// <https://stefanolivier.com>

///////////////////////////////////////////////////////////////////////////
//
//  -- @SECTION `EDLParser` Traits --
//
///////////////////////////////////////////////////////////////////////////

pub trait ParseField<T> {
    fn parse_field(field_string: &str) -> Option<T>;
}

pub trait ParseTable<T, D> {
    const TABLE_TOTAL_COLUMNS: usize;
    fn parse_table(table_data: &[String], defaults: D) -> Option<Vec<T>>;
}
