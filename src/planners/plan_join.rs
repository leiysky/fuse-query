// Copyright 2020 The FuseQuery Authors.
//
// Code is licensed under AGPL License, Version 3.0.

use crate::{
    datavalues::DataSchemaRef,
    planners::{ExpressionPlan, PlanNode},
};
use std::sync::Arc;

// TODO: Support USING clause
#[derive(Clone)]
pub enum JoinType {
    Inner,
    LeftOuter,
    RightOuter,
    FullOuter,
    CrossJoin,
}

#[derive(Clone)]
pub struct JoinPlan {
    pub lhs: Arc<PlanNode>,
    pub rhs: Arc<PlanNode>,
    pub join_type: JoinType,
    // Natural Join if None
    pub condition: Option<ExpressionPlan>,
}

impl JoinPlan {
    pub fn schema(&self) -> DataSchemaRef {
        unimplemented!()
    }
}
