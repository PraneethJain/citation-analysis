# Analysis of High-energy physics citation network
> http://snap.stanford.edu/data/cit-HepPh.html

![graph-animation](./graphs/output.gif)

## Some properties plotted over time

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

TO DO: PUT THE CENTRALITY MEASURES HERE

# Girwan-Newman Community Detection

Since the graph has one big SCC and a large number of scattered smaller
components, we will only be performing community detection on the large scc and
ignore the scattered components.

## Undirected

