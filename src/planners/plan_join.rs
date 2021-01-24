// Copyright 2020 The FuseQuery Authors.
//
// Code is licensed under AGPL License, Version 3.0.

use crate::datasources::{Partitions, Statistics};
use crate::datavalues::DataSchemaRef;

enum JoinType {
    Inner,
    LeftOuter,
    RightOuter,
    FullOuter,
    Cross,
}

#[derive(Clone)]
pub struct Join {
    pub db: String,
    pub table: String,
    pub schema: DataSchemaRef,
    pub partitions: Partitions,
    pub statistics: Statistics,
    pub description: String,
}
