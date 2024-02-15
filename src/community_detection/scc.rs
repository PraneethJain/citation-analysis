use crate::common::*;
use std::collections::{BTreeMap, BTreeSet};

pub fn tarjan_scc(adj_list: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    struct TarjanState {
        index: i32,
        stack: Vec<usize>,
        on_stack: Vec<bool>,
        index_of: Vec<i32>,
        lowlink_of: Vec<i32>,
        components: Vec<Vec<usize>>,
    }

    let mut state = TarjanState {
        index: 0,
        stack: Vec::new(),
        on_stack: vec![false; adj_list.len()],
        index_of: vec![-1; adj_list.len()],
        lowlink_of: vec![-1; adj_list.len()],
        components: Vec::new(),
    };

    fn strong_connect(v: usize, adj_list: &Vec<Vec<usize>>, state: &mut TarjanState) {
        state.index_of[v] = state.index;
        state.lowlink_of[v] = state.index;
        state.index += 1;
        state.stack.push(v);
        state.on_stack[v] = true;

        for &w in &adj_list[v] {
            if state.index_of[w] == -1 {
                strong_connect(w, adj_list, state);
                state.lowlink_of[v] = state.lowlink_of[v].min(state.lowlink_of[w]);
            } else if state.on_stack[w] {
                state.lowlink_of[v] = state.lowlink_of[v].min(state.index_of[w]);
            }
        }

        if state.lowlink_of[v] == state.index_of[v] {
            let mut component: Vec<usize> = Vec::new();
            while let Some(w) = state.stack.pop() {
                state.on_stack[w] = false;
                component.push(w);
                if w == v {
                    break;
                }
            }
            state.components.push(component);
        }
    }

    for v in 0..adj_list.len() {
        if state.index_of[v] == -1 {
            strong_connect(v, adj_list, &mut state);
        }
    }

    state.components
}

pub fn largest_scc_graph(full_graph: &Graph) -> Graph {
    let largest_scc = super::tarjan_scc(&full_graph.adj_list)
        .into_iter()
        .max_by_key(|x| x.len())
        .unwrap();
    let largest_scc = BTreeSet::from_iter(largest_scc.iter().cloned());

    let mut adj_list = vec![vec![]; MAX_NODES];
    let mut index_to_id: BTreeMap<usize, usize> = BTreeMap::new();
    let mut id_to_index: BTreeMap<usize, usize> = BTreeMap::new();
    let mut index = 0;

    for (from, tos) in full_graph.adj_list.iter().enumerate() {
        if !largest_scc.contains(&from) {
            continue;
        }

        if !id_to_index.contains_key(&from) {
            id_to_index.insert(from, index);
            index_to_id.insert(index, from);
            index += 1;
        }

        for to in tos {
            if !largest_scc.contains(to) {
                continue;
            }

            if !id_to_index.contains_key(&to) {
                id_to_index.insert(*to, index);
                index_to_id.insert(index, *to);
                index += 1;
            }

            adj_list[id_to_index[&from]].push(id_to_index[&to]);
        }
    }
    adj_list.resize(index, vec![]);
    Graph {
        adj_list,
        index_to_id,
        id_to_index,
    }
}
