// Copyright 2020 The FuseQuery Authors.
//
// Code is licensed under AGPL License, Version 3.0.

use std::fmt;

use crate::datablocks::DataBlock;
use crate::datavalues::{DataColumnarValue, DataSchema, DataType, DataValue};
use crate::error::FuseQueryResult;
use crate::functions::Function;

#[derive(Clone, Debug)]
pub struct ConstantFunction {
    value: DataValue,
}

impl ConstantFunction {
    pub fn try_create(value: DataValue) -> FuseQueryResult<Function> {
        Ok(Function::Constant(ConstantFunction { value }))
    }

    pub fn return_type(&self, _input_schema: &DataSchema) -> FuseQueryResult<DataType> {
        Ok(self.value.data_type())
    }

    pub fn nullable(&self, _input_schema: &DataSchema) -> FuseQueryResult<bool> {
        Ok(self.value.is_null())
    }

    pub fn eval(&self, _block: &DataBlock) -> FuseQueryResult<DataColumnarValue> {
        Ok(DataColumnarValue::Scalar(self.value.clone()))
    }

    pub fn set_depth(&mut self, _depth: usize) {}

    pub fn accumulate(&mut self, _block: &DataBlock) -> FuseQueryResult<()> {
        Ok(())
    }

    pub fn accumulate_result(&self) -> FuseQueryResult<Vec<DataValue>> {
        Ok(vec![self.value.clone()])
    }

    pub fn merge(&mut self, _states: &[DataValue]) -> FuseQueryResult<()> {
        Ok(())
    }

    pub fn merge_result(&self) -> FuseQueryResult<DataValue> {
        Ok(self.value.clone())
    }
}

impl fmt::Display for ConstantFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.value)
    }
}
