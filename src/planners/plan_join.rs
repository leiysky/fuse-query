// Copyright 2020 The FuseQuery Authors.
//
// Code is licensed under AGPL License, Version 3.0.

use sqlparser::ast::JoinOperator;

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

impl From<JoinOperator> for JoinType {
    fn from(op: JoinOperator) -> JoinType {
        match op {
            JoinOperator::Inner(_) => JoinType::Inner,
            JoinOperator::LeftOuter(_) => JoinType::LeftOuter,
            JoinOperator::RightOuter(_) => JoinType::RightOuter,
            JoinOperator::FullOuter(_) => JoinType::FullOuter,
            JoinOperator::CrossJoin => JoinType::CrossJoin,
            _ => unimplemented!()
        }
    }
}

#[derive(Clone)]
pub struct JoinPlan {
    pub lhs: Arc<PlanNode>,
    pub rhs: Arc<PlanNode>,
    pub join_type: JoinType,
    // Natural Join if None
    pub cnf_conditions: Vec<ExpressionPlan>,

    pub schema: DataSchemaRef,
}

impl JoinPlan {
    pub fn schema(&self) -> DataSchemaRef {
        unimplemented!()
    }
}
