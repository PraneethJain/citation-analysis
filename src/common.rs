use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub date: u8,
}

impl Date {
    pub fn from(date_str: &str) -> Self {
        let date = date_str[8..].parse().unwrap();
        let month = date_str[5..7].parse().unwrap();
        let year = date_str[..4].parse().unwrap();

        return Date { date, month, year };
    }

    pub fn old() -> Self {
        return Date {
            date: 0,
            month: 0,
            year: 0,
        };
    }
}

pub struct Graph {
    pub adj_list: Vec<Vec<usize>>,
    pub index_to_id: BTreeMap<usize, usize>,
    pub id_to_index: BTreeMap<usize, usize>,
}

pub struct Graphs {
    full_graph: Graph,
    dates: Vec<Date>,
}

impl Graphs {
    pub fn new(full_graph: Graph, dates: Vec<Date>) -> Self {
        Graphs { full_graph, dates }
    }

    pub fn till(self: &Self, date: &Date) -> Graph {
        let mut adj_list = vec![vec![]; MAX_NODES];
        let mut index_to_id: BTreeMap<usize, usize> = BTreeMap::new();
        let mut id_to_index: BTreeMap<usize, usize> = BTreeMap::new();
        let mut dates: Vec<Date> = vec![Date::old(); MAX_NODES];
        let mut index = 0;

        for (from, tos) in self.full_graph.adj_list.iter().enumerate() {
            if self.dates[from] > *date {
                continue;
            }

            if !id_to_index.contains_key(&from) {
                id_to_index.insert(from, index);
                index_to_id.insert(index, from);
                index += 1;
            }

            for to in tos {
                if self.dates[*to] > *date {
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
        index_to_id = index_to_id
            .into_iter()
            .map(|(idx, id)| (idx, self.full_graph.index_to_id[&id]))
            .collect();
        adj_list.resize(index, vec![]);
        dates.resize(index, Date::old());
        Graph {
            adj_list,
            index_to_id,
            id_to_index,
        }
    }
}

pub const MAX_NODES: usize = 34546;
