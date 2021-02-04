// Copyright 2020 The FuseQuery Authors.
//
// Code is licensed under AGPL License, Version 3.0.

use crate::datavalues::DataSchemaRef;
use crate::error::{FuseQueryError, FuseQueryResult};
use crate::planners::{
    AggregatePlan, EmptyPlan, ExplainPlan, FilterPlan, JoinPlan, LimitPlan, PlanBuilder,
    ProjectionPlan, ReadDataSourcePlan, ScanPlan, SelectPlan, SettingPlan,
};

#[derive(Clone)]
pub enum PlanNode {
    Empty(EmptyPlan),
    Projection(ProjectionPlan),
    Aggregate(AggregatePlan),
    Filter(FilterPlan),
    Limit(LimitPlan),
    Scan(ScanPlan),
    Join(JoinPlan),
    ReadSource(ReadDataSourcePlan),
    Explain(ExplainPlan),
    Select(SelectPlan),
    SetVariable(SettingPlan),
}

impl PlanNode {
    /// Get a reference to the logical plan's schema
    pub fn schema(&self) -> DataSchemaRef {
        match self {
            PlanNode::Empty(v) => v.schema(),
            PlanNode::Scan(v) => v.schema(),
            PlanNode::Join(v) => v.schema(),
            PlanNode::Projection(v) => v.schema(),
            PlanNode::Aggregate(v) => v.schema(),
            PlanNode::Filter(v) => v.schema(),
            PlanNode::Limit(v) => v.schema(),
            PlanNode::ReadSource(v) => v.schema(),
            PlanNode::Select(v) => v.plan.schema(),
            PlanNode::Explain(_) => unimplemented!(),
            PlanNode::SetVariable(_) => unimplemented!(),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            PlanNode::Empty(_) => "EmptyPlan",
            PlanNode::Scan(_) => "ScanPlan",
            PlanNode::Join(_) => "JoinPlan",
            PlanNode::Projection(_) => "ProjectionPlan",
            PlanNode::Aggregate(_) => "AggregatePlan",
            PlanNode::Filter(_) => "FilterPlan",
            PlanNode::Limit(_) => "LimitPlan",
            PlanNode::ReadSource(_) => "ReadSourcePlan",
            PlanNode::Select(_) => "SelectPlan",
            PlanNode::Explain(_) => "ExplainPlan",
            PlanNode::SetVariable(_) => "SetVariablePlan",
        }
    }

    /// build plan node to list
    /// with_parent only affected select/explain
    fn build_plan_list(&self, with_parent: bool) -> FuseQueryResult<Vec<PlanNode>> {
        let max_depth = 128;
        let mut depth = 0;
        let mut list = vec![];
        let mut plan = self.clone();

        loop {
            if depth > max_depth {
                return Err(FuseQueryError::Plan(format!(
                    "PlanNode depth more than {}",
                    max_depth
                )));
            }

            match plan {
                PlanNode::Aggregate(v) => {
                    list.push(PlanNode::Aggregate(v.clone()));
                    plan = v.input.as_ref().clone();
                    depth += 1;
                }
                PlanNode::Projection(v) => {
                    list.push(PlanNode::Projection(v.clone()));
                    plan = v.input.as_ref().clone();
                    depth += 1;
                }
                PlanNode::Filter(v) => {
                    list.push(PlanNode::Filter(v.clone()));
                    plan = v.input.as_ref().clone();
                    depth += 1;
                }
                PlanNode::Limit(v) => {
                    list.push(PlanNode::Limit(v.clone()));
                    plan = v.input.as_ref().clone();
                    depth += 1;
                }
                PlanNode::Select(v) => {
                    if with_parent {
                        list.push(PlanNode::Select(v.clone()));
                    }
                    plan = v.plan.as_ref().clone();
                    depth += 1;
                }
                PlanNode::Join(v) => {
                    if with_parent {
                        list.push(PlanNode::Join(v.clone()));
                    }
                    plan = v.lhs.as_ref().clone();
                    depth += 1;
                }
                PlanNode::Explain(v) => {
                    if with_parent {
                        list.push(PlanNode::Explain(v.clone()));
                    }
                    plan = v.plan.as_ref().clone();
                    depth += 1;
                }

                // Return.
                PlanNode::SetVariable(_) => {
                    break;
                }
                PlanNode::Empty(_) => {
                    break;
                }
                PlanNode::Scan(v) => {
                    list.push(PlanNode::Scan(v));
                    break;
                }
                PlanNode::ReadSource(v) => {
                    list.push(PlanNode::ReadSource(v));
                    break;
                }
            }
        }
        list.reverse();
        Ok(list)
    }

    pub fn subplan_to_list(&self) -> FuseQueryResult<Vec<PlanNode>> {
        self.build_plan_list(false)
    }

    pub fn plan_to_list(&self) -> FuseQueryResult<Vec<PlanNode>> {
        self.build_plan_list(true)
    }

    pub fn plan_list_to_node(list: &[PlanNode]) -> FuseQueryResult<PlanNode> {
        let mut builder = PlanBuilder::empty();
        for plan in list {
            match plan {
                PlanNode::Projection(v) => {
                    builder = builder.project(v.expr.clone())?;
                }
                PlanNode::Aggregate(v) => {
                    builder = builder.aggregate(v.group_expr.clone(), v.aggr_expr.clone())?;
                }
                PlanNode::Filter(v) => {
                    builder = builder.filter(v.predicate.clone())?;
                }
                PlanNode::Limit(v) => {
                    builder = builder.limit(v.n)?;
                }
                PlanNode::ReadSource(v) => {
                    builder = PlanBuilder::from(&PlanNode::ReadSource(v.clone()))
                }
                PlanNode::Explain(_v) => {
                    builder = builder.explain()?;
                }
                PlanNode::Select(_v) => {
                    builder = builder.select()?;
                }
                PlanNode::Join(v) => {
                    builder = PlanBuilder::from(&PlanNode::Join(v.clone()))
                }
                PlanNode::Empty(_) => {}
                PlanNode::Scan(_) => {}
                PlanNode::SetVariable(_) => {}
            }
        }
        builder.build()
    }
}
