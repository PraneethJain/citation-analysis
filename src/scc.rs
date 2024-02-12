use crate::common::*;

pub fn tarjan_scc(graph: &Graph) -> Vec<Vec<usize>> {
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
        on_stack: vec![false; MAX_NODES],
        index_of: vec![-1; MAX_NODES],
        lowlink_of: vec![-1; MAX_NODES],
        components: Vec::new(),
    };

    fn strong_connect(v: usize, graph: &Graph, state: &mut TarjanState) {
        state.index_of[v] = state.index;
        state.lowlink_of[v] = state.index;
        state.index += 1;
        state.stack.push(v);
        state.on_stack[v] = true;

        for &w in &graph[v] {
            if state.index_of[w] == -1 {
                strong_connect(w, graph, state);
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

    for v in 0..MAX_NODES {
        if state.index_of[v] == -1 {
            strong_connect(v, graph, &mut state);
        }
    }

    state.components
}
