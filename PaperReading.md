# node2vec
> https://arxiv.org/abs/1607.00653

## Summary
node2vec is an algorithm that aims to find node embeddings based on both
  homophily and structural similarity. Previous methods either use spectral methods which are infeasible for large networks, or are very rigid with high training time complexity.

Let $G = (V, E)$ be the graph.
Firstly, we define the objective function to optimize

$$ \max_{f} \sum_{u \in V} log Pr(N_S(u)|f(u)) $$

where $N_S(u) \subset V$ is the network neighbourhood of $u$ generated through sampling strategy $S$, and $f$ is the node embedding function we want to find.

We use the softmax function to get a probability distribution. The objective function simplifies to

$$ \max_{f} \sum_{u \in V} \[-log Z_u + \sum_{n_i \in N_S(u)} f(n_i) \cdot f(u) \] $$

where $Z_u = \sum_{u \in V} exp(f(u) \cdot f(v))$ is the per-node partition function. This is expensive to calculate, and is thus approximated using negative sampling.
This objective function is optimized using stochastic gradient ascent, as the maxixum is required.

To compute the neigbourhood of a node, we perform the following second order random walk:
- Assume the previous traversal was from t to v
- The walk will go back to t with probability proportial to 1/p. The lower the p, the higher this probability, encouraging homophily and BFS like behaviour.
- The walk will go to nodes away from t (not directly connected to t) with probability proportial to 1/q. The lower the q, the higher the probability, encouraging structural similarity and DFS like behaviour.
- The walk will go to every other node with constant probability

<p align="middle">
  <img src="https://github.com/PraneethJain/citation-analysis/assets/49565677/a2126b0e-9a3b-4bdf-8453-525d01ba7cf6" align="middle"/>
</p>

In conclusion, the algorithm is as follows
- Compute the random walk probabilities
- Simulate the biased random walks from each node
- Use stochastic gradient ascent to optimize the objective function

## Strengths
- This algorithm is very flexible as compared to others. This can be attributed to the random walk parameters that allow for neighbourhoods based on a good interpolation of BFS-like and DFS-like traversals.
- It scales very well with large networks. As mentioned in the paper, the time taken to run node2vec scales linearly with the number of nodes in a network.
- All the 3 steps of the algorithm stated above are individually parallelizable. A simple shader can be written to execute this on the GPU for very large networks, or this can trivially be run over multiple threads concurrently.

## Weaknesses
- The algorithm only takes connectivity into account and discards other properties of the graph.
- The embeddings produced by node2vec are unstable. This means that a small change in the parameters might result in vastly different embeddings. [source](https://arxiv.org/pdf/2206.08252.pdf)
- The algorithm is very general and assumes no knowledge about the downstream prediction task. While in theory this may seem like an advantage, for practical purposes, we do have knowledge of the prediction task. Engineering our features for the specific prediction task would result in better accuracy. 

## Improvements
- One could consider weighted properties of nodes (say centrality measures) into account as well. For example, we could give a higher probability to nodes with higher centrality as they are a better representative of the entire network.
- To stabilize the embeddings, multiple runs of the algorithm can be performed on the same parameters, and the average embedding for each node can be considered as the final embedding.
- Better alternatives to softmax function can be considered. [source](https://arxiv.org/pdf/1711.03953.pdf)
