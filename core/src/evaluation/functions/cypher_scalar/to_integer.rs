// Copyright 2024 The Drasi Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::evaluation::variable_value::VariableValue;
use async_trait::async_trait;
use drasi_query_ast::ast;

use crate::evaluation::functions::ScalarFunction;
use crate::evaluation::{ExpressionEvaluationContext, FunctionError, FunctionEvaluationError};

#[derive(Debug)]
pub struct ToInteger {}

#[async_trait]
impl ScalarFunction for ToInteger {
    async fn call(
        &self,
        _context: &ExpressionEvaluationContext,
        expression: &ast::FunctionExpression,
        args: Vec<VariableValue>,
    ) -> Result<VariableValue, FunctionError> {
        if args.len() != 1 {
            return Err(FunctionError {
                function_name: expression.name.to_string(),
                error: FunctionEvaluationError::InvalidArgumentCount,
            });
        }
        match &args[0] {
            VariableValue::Null => Ok(VariableValue::Null),
            VariableValue::Integer(i) => Ok(VariableValue::Integer(i.clone())),
            VariableValue::Float(f) => Ok(VariableValue::Integer(
                (match f.as_f64() {
                    Some(f) => f.floor() as i64,
                    None => {
                        return Err(FunctionError {
                            function_name: expression.name.to_string(),
                            error: FunctionEvaluationError::OverflowError,
                        })
                    }
                })
                .into(),
            )),
            VariableValue::Bool(b) => {
                if *b {
                    Ok(VariableValue::Integer(1.into()))
                } else {
                    Ok(VariableValue::Integer(0.into()))
                }
            }
            VariableValue::String(s) => {
                if let Ok(i) = s.parse::<i64>() {
                    Ok(VariableValue::Integer(i.into()))
                } else if let Ok(f) = s.parse::<f64>() {
                    Ok(VariableValue::Integer((f.floor() as i64).into()))
                } else {
                    Ok(VariableValue::Null)
                }
            }
            _ => Err(FunctionError {
                function_name: expression.name.to_string(),
                error: FunctionEvaluationError::InvalidArgument(0),
            }),
        }
    }
}

#[derive(Debug)]
pub struct ToIntegerOrNull {}

#[async_trait]
impl ScalarFunction for ToIntegerOrNull {
    async fn call(
        &self,
        _context: &ExpressionEvaluationContext,
        expression: &ast::FunctionExpression,
        args: Vec<VariableValue>,
    ) -> Result<VariableValue, FunctionError> {
        if args.len() != 1 {
            return Err(FunctionError {
                function_name: expression.name.to_string(),
                error: FunctionEvaluationError::InvalidArgumentCount,
            });
        }
        match &args[0] {
            VariableValue::Null => Ok(VariableValue::Null),
            VariableValue::Integer(i) => Ok(VariableValue::Integer(i.clone())),
            VariableValue::Float(f) => Ok(VariableValue::Integer(
                (match f.as_f64() {
                    Some(f) => f.floor() as i64,
                    None => {
                        return Err(FunctionError {
                            function_name: expression.name.to_string(),
                            error: FunctionEvaluationError::OverflowError,
                        })
                    }
                })
                .into(),
            )),
            VariableValue::Bool(b) => {
                if *b {
                    Ok(VariableValue::Integer(1.into()))
                } else {
                    Ok(VariableValue::Integer(0.into()))
                }
            }
            VariableValue::String(s) => {
                if let Ok(i) = s.parse::<i64>() {
                    Ok(VariableValue::Integer(i.into()))
                } else if let Ok(f) = s.parse::<f64>() {
                    Ok(VariableValue::Integer((f.floor() as i64).into()))
                } else {
                    Ok(VariableValue::Null)
                }
            }
            _ => Ok(VariableValue::Null),
        }
    }
}
