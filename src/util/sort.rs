use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

pub fn get_acyclic_dependency_graph<'a, T>(graph: HashMap<&'a T, Vec<&'a T>>) -> Result<Vec<&'a T>, Vec<&'a T>>
    where T: Clone + Eq + Hash + Debug {
    // ---
    let mut res: Vec<&T> = Vec::new();
    let mut graph = graph;

    while !graph.is_empty() {
        let leaf = graph.keys().find(|key| graph[**key].is_empty()).map(|i| *i);

        match leaf {
            Some(leaf) => {
                graph.iter_mut().for_each(|(_, deps)| {
                    while let Some(pos) = deps.iter().position(|x| *x == leaf) {
                        deps.remove(pos);
                    }
                });

                graph.remove(leaf);
                res.push(leaf);
            }
            None => {
                // Cycle detected
                let mut cycle: Vec<&T> = vec![];

                let mut current: &T = *graph.keys().next().unwrap();
                cycle.push(current);

                loop {
                    let next = graph[current].first().unwrap();
                    if cycle.contains(next) {
                        cycle.push(next);
                        break;
                    } else {
                        cycle.push(next);
                        current = next;
                    }
                }

                return Err(cycle);
            }
        }
    }

    Ok(res)
}

pub fn sort_dependencies(graph: HashMap<String, Vec<String>>) -> Result<Vec<String>, Vec<String>> {
    let mut res: Vec<String> = Vec::new();
    let mut graph = graph;

    while !graph.is_empty() {
        let leaf = graph.keys()
            .find(|key| graph[*key].is_empty())
            .map(|i| i.to_string());

        match leaf {
            Some(leaf) => {
                graph.iter_mut().for_each(|(_, deps)| {
                    while let Some(pos) = deps.iter().position(|x| x == &leaf) {
                        deps.remove(pos);
                    }
                });

                graph.remove(&leaf);
                res.push(leaf);
            }
            None => {
                // Cycle detected
                let mut cycle: Vec<String> = vec![];

                let mut current: String = graph.keys().next().unwrap().clone();
                cycle.push(current.clone());

                loop {
                    let next = graph[&current].first().unwrap().clone();
                    if cycle.contains(&next) {
                        cycle.push(next);
                        break;
                    } else {
                        cycle.push(next.clone());
                        current = next;
                    }
                }

                return Err(cycle);
            }
        }
    }

    Ok(res)
}