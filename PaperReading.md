# node2vec
> https://arxiv.org/abs/1607.00653

## Summary
- node2vec is an algorithm that aims to find node embeddings based on both
  homophily and structural similarity. Previous methods either use spectral methods which are infeasible for large networks, or are very rigid with high training time complexity.
- Let $G = (V, E)$ be the graph.
- Firstly, we define the objective function to optimize

$$ \max_{f} \sum_{u \in V} log Pr(N_S(u)|f(u)) $$

where $N_S(u) \subset V$ is the network neighbourhood of $u$ generated through sampling strategy $S$, and $f$ is the node embedding function we want to find.

- We use the softmax function to get a probability distribution. The objective function simplifies to

$$ \max_{f} \sum_{u \in V} \[-log Z_u + \sum_{n_i \in N_S(u)} f(n_i) \cdot f(u) \] $$

where $Z_u = \sum_{u \in V} exp(f(u) \cdot f(v))$ is the per-node partition function. This is expensive to calculate, and is thus approximated using negative sampling.
This objective function is optimized using stochastic gradient ascent, as the maxixum is required.

- We perform the following second order random walk:
  - Assume the previous traversal was from t to v
  - The walk will go back to t with probability proportial to 1/p. The lower the p, the higher this probability, encouraging homophily and BFS like behaviour.
  - The walk will go to nodes away from t (not directly connected to t) with probability proportial to 1/q. The lower the q, the higher the probability, encouraging structural similarity and DFS like behaviour.
  - The walk will go to every other node with constant probability
 ![image](https://github.com/PraneethJain/citation-analysis/assets/49565677/a2126b0e-9a3b-4bdf-8453-525d01ba7cf6)

## Strengths

## Weaknesses

## Improvements
