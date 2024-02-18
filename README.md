# Analysis of High-Energy Physics citation network
> http://snap.stanford.edu/data/cit-HepPh.html

## [Paper Reading](PaperReading.md)

![graph-animation](./graphs/output.gif)

## Some properties

### Strongly Connected Components

<p align="middle">
  <img src="plots/scc_counts.png" align="middle" width="45%" />
  <img src="plots/largest_scc_sizes.png" align="middle" width="45%" />
</p>

- It is interesting to note that the graph has only one big SCC, while the rest
  of the nodes are in components of size strictly less than 12. 
- This indicates that this citation network has one mainstream sub-field
  (papers which are part of the large SCC), and the rest of the papers are
  sparse sub-fields with ongoing research disconnected from the mainstream
  branch.
- The number of strongly connected components peaks around 2001. This can be
  attributed to the number of sub-fields being saturated. After this point, the
  frequency of papers linking different sub-fields together increases, which
  leads to the merging of different SCCs together.

### Weakly Connected Components

<p align="middle">
  <img src="plots/wcc_counts.png" align="middle" width="45%" />
  <img src="plots/largest_wcc_sizes.png" align="middle" width="45%" />
</p>

- As expected, the saturation point in case of WCCs occurs much earlier (around
  1993) than that of SCCs.
- After this, any one way linkage between two sub-fields causes a merge in
  between the two WCCs, thus we observe a rapid decline in the number of WCCs
  after this.
- Surprisingly, the size of the largest WCC increases almost linearly along with time.
- On performing linear regression on this, we get an R2 score of 97%, verifying that the trend is indeed linear.

<p align="middle">
    <img src="plots/wcc_size_linreg.png" align="middle" />
</p>

- This can be used to predict the size of the largest WCC at arbitrary times.

### Degree Centrality

<p align="middle">
  <img src="plots/freeman_degree_centralization.png" align="middle" width="45%" />
  <img src="plots/max_degree_centralization.png" align="middle" width="45%" />
</p>

### Indegree Centrality

<p align="middle">
  <img src="plots/freeman_indegree_centralization.png" align="middle" width="45%" />
  <img src="plots/max_indegree_centralization.png" align="middle" width="45%" />
</p>

### Outdegree Centrality

<p align="middle">
  <img src="plots/freeman_outdegree_centralization.png" align="middle" width="45%" />
  <img src="plots/max_outdegree_centralization.png" align="middle" width="45%" />
</p>

### Betweenness Centrality

<p align="middle">
  <img src="plots/freeman_betweenness_centralization.png" align="middle" width="45%" />
  <img src="plots/max_betweenness_centralization.png" align="middle" width="45%" />
</p>

## Community Detection

Since the graph has one big SCC and a large number of scattered smaller
components, we will only be performing community detection on the large SCC and
ignore the scattered components.

### Girwan-Newman 

#### Directed Graph

- We will restrict our discussion to the years 1999 and later, due to the fact
  that [the graph is heavily disconnected before
  that](#strongly-connected-components).
- The case of the directed graph is particularly uninteresting, as there is
  only one large community at all times. Girwan-Newman removes the edge with
  the highest betweenness centrality, and doing so for the directed graph
  results in the same community, but with one less node.

#### Undirected Graph
- The undirected graph results in interesting communities, but the algorithm is far too slow to run completely.
- Parallelizing Girwan-Newman over multiple threads resulted in a 4.9x speedup, which made community detection uptil 1995 feasible.

<p align="middle">
  <img src="community_graphs/girwan_newman/g1992.png" align="middle" width="45%" />
  <img src="community_graphs/girwan_newman/g1993.png" align="middle" width="45%" />
</p>
<p align="middle">
  <img src="community_graphs/girwan_newman/g1994.png" align="middle" width="45%" />
  <img src="community_graphs/girwan_newman/g1995.png" align="middle" width="45%" />
</p>

> 1992 (top left), 1993 (top right), 1994 (bottom left), 1995 (bottom right)

### Louvain
![louvain](./community_graphs/louvain/output.gif)
> Communities over time

<p align="middle">
  <img src="plots/louvain_community_counts.png" align="middle" width="45%" />
</p>

- The trend of community count over time is decreasing
- The communities themselves are growing denser and larger, and as more papers are added, it leads to intermingling of communities.

## Tools Used
- [rust](https://www.rust-lang.org/): Programming language of choice
- [graphviz](https://graphviz.org/): Command line graph visualization tool
- [gnuplot](http://www.gnuplot.info/): Command line plotting tool
- [ffmpeg](https://ffmpeg.org/): For creating the graph animation
- [fast-louvain](https://github.com/Splines/fast-louvain): For louvain algorithm

## Algorithms Implemented
- Degree Centrality
- Betweenness Centrality (Brande's Algorithm)
- Triangle Count
- SCCs and WCCs (Tarjan's)
- Girwan Newman
- Linear Regression

## Instructions
- To run the code `cargo run --release`
- In the plots/graphs folders, run the corresponding `plot.sh` script to generate plots/graphs from the saved files.
