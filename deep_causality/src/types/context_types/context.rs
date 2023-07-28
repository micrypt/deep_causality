// Copyright (c) "2023" . Marvin Hansen <marvin.hansen@gmail.com> All rights reserved.

use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use petgraph::Directed;
use petgraph::matrix_graph::MatrixGraph;
use crate::prelude::{Contextoid, NodeIndex,Datable, SpaceTemporal, Spatial, Temporal};

#[derive(Clone)]
pub struct Context<'l, D, S, T, ST>
    where
        D: Datable,
        S: Spatial,
        T: Temporal,
        ST: SpaceTemporal
{
    id: u64,
    name: String,
    // Edge weights need to be numerical (u64) to make shortest path algo work.
    graph: MatrixGraph<&'l Contextoid<D, S, T, ST>, u64, Directed, Option<u64>, u32>,
    context_map: HashMap<NodeIndex, &'l Contextoid<D, S, T, ST>>,
}


impl<'l, D, S, T, ST> Context<'l, D, S, T, ST>
    where
        D: Datable,
        S: Spatial,
        T: Temporal,
        ST: SpaceTemporal
{
    pub fn new(
        id: u64,
        name: String,
    )
        -> Self
    {
        Self { id, name, graph: MatrixGraph::default(), context_map: HashMap::new() }
    }

    pub fn with_capacity(
        id: u64,
        name: String,
        capacity: usize,
    )
        -> Self
    {
        Self { id, name, graph: MatrixGraph::default(), context_map: HashMap::with_capacity(capacity) }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<'l, D, S, T, ST> Default for Context<'l, D, S, T, ST>
    where
        D: Datable,
        S: Spatial,
        T: Temporal,
        ST: SpaceTemporal
{
    fn default() -> Self {
        Self::new(0, "default".to_string())
    }
}

impl<'l, D, S, T, ST> Context<'l, D, S, T, ST>
    where
        D: Datable,
        S: Spatial,
        T: Temporal,
        ST: SpaceTemporal
{
    fn add_contextoid(
        &mut self,
        value: &'l Contextoid<D, S, T, ST>,
    )
        -> NodeIndex
    {
        let node_index = self.graph.add_node(value);
        self.context_map.insert(node_index, value);

        node_index
    }

    fn contains_contextoid(
        &self,
        index: NodeIndex,
    )
        -> bool
    {
        self.context_map.contains_key(&index)
    }

    fn get_contextoid(
        &self,
        index: NodeIndex,
    )
        -> Option<&&Contextoid<D, S, T, ST>>
    {
        self.context_map.get(&index)
    }

    fn remove_contextoid(
        &mut self,
        index: NodeIndex,
    )
    {
        self.graph.remove_node(index);
        self.context_map.remove(&index);
    }


    fn contains_edge(
        &self,
        a: NodeIndex,
        b: NodeIndex,
    )
        -> bool
    {
        self.graph.has_edge(a, b)
    }

    fn size(
        &self
    )
        -> usize
    {
        self.context_map.len()
    }

    fn is_empty(
        &self
    )
        -> bool
    {
        self.context_map.is_empty()
    }

    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    pub fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }
}

impl<'l, D, S, T, ST> Context<'l, D, S, T, ST>
    where
        D: Datable,
        S: Spatial,
        T: Temporal,
        ST: SpaceTemporal
{
    fn format(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "Context: id: {}, name: {}, node_count: {}, edge_count: {}",
               self.id,
               self.name,
               self.node_count(),
               self.edge_count(),
        )
    }
}

impl<'l, D, S, T, ST> Debug for Context<'l, D, S, T, ST>
    where
        D: Datable,
        S: Spatial,
        T: Temporal,
        ST: SpaceTemporal
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.format(f)
    }
}

impl<'l, D, S, T, ST> Display for Context<'l, D, S, T, ST>
    where
        D: Datable,
        S: Spatial,
        T: Temporal,
        ST: SpaceTemporal
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.format(f)
    }
}