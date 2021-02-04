// Copyright 2020 The FuseQuery Authors.
//
// Code is licensed under AGPL License, Version 3.0.

use std::sync::Arc;

use crate::contexts::FuseQueryContextRef;
use crate::error::{FuseQueryError, FuseQueryResult};
use crate::planners::PlanNode;
use crate::processors::Pipeline;
use crate::transforms::{
    AggregateFinalTransform, AggregatePartialTransform, FilterTransform, LimitTransform,
    ProjectionTransform, SourceTransform,
};

pub struct PipelineBuilder {
    ctx: FuseQueryContextRef,
    plan: PlanNode,
}

impl PipelineBuilder {
    pub fn create(ctx: FuseQueryContextRef, plan: PlanNode) -> Self {
        PipelineBuilder { ctx, plan }
    }

    pub fn build(&self) -> FuseQueryResult<Pipeline> {
        let mut pipeline = Pipeline::create();
        let plans = self.plan.subplan_to_list()?;
        for plan in &plans {
            match plan {
                PlanNode::Limit(plan) => {
                    pipeline.add_simple_transform(|| {
                        Ok(Box::new(LimitTransform::try_create(plan.n)?))
                    })?;
                    if pipeline.pipe_num() > 1 {
                        pipeline.merge_processor()?;
                        pipeline.add_simple_transform(|| {
                            Ok(Box::new(LimitTransform::try_create(plan.n)?))
                        })?;
                    }
                }
                PlanNode::Projection(plan) => {
                    pipeline.add_simple_transform(|| {
                        Ok(Box::new(ProjectionTransform::try_create(
                            plan.schema.clone(),
                            plan.expr.clone(),
                        )?))
                    })?;
                }
                PlanNode::Aggregate(plan) => {
                    pipeline.add_simple_transform(|| {
                        Ok(Box::new(AggregatePartialTransform::try_create(
                            plan.schema.clone(),
                            plan.aggr_expr.clone(),
                        )?))
                    })?;

                    pipeline.merge_processor()?;
                    pipeline.add_simple_transform(|| {
                        Ok(Box::new(AggregateFinalTransform::try_create(
                            plan.schema.clone(),
                            plan.aggr_expr.clone(),
                        )?))
                    })?;
                }
                PlanNode::Join(plan) => {
                    pipeline.add_simple_transform(|| {});
                }
                PlanNode::Filter(plan) => {
                    pipeline.add_simple_transform(|| {
                        Ok(Box::new(FilterTransform::try_create(
                            plan.predicate.clone(),
                        )?))
                    })?;
                }
                PlanNode::ReadSource(plan) => {
                    let mut shuffle = vec![];
                    let workers = self.ctx.get_max_threads()? as usize;
                    let workers = if workers == 0 || workers >= plan.partitions.len() {
                        1
                    } else {
                        let parts = plan.partitions.len();
                        if parts % workers != 0 {
                            parts / workers + 1
                        } else {
                            parts / workers
                        }
                    };

                    for chunk in plan.partitions.chunks(workers) {
                        shuffle.push(chunk);
                    }

                    for partition in shuffle {
                        let source = SourceTransform::try_create(
                            self.ctx.clone(),
                            plan.db.as_str(),
                            plan.table.as_str(),
                            partition.to_vec(),
                        )?;
                        pipeline.add_source(Arc::new(source))?;
                    }
                }
                other => {
                    return Err(FuseQueryError::Internal(format!(
                        "Build pipeline from the plan node unsupported:{:?}",
                        other.name()
                    )))
                }
            }
        }
        pipeline.merge_processor()?;
        Ok(pipeline)
    }
}
