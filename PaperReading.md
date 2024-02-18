# node2vec
> https://arxiv.org/abs/1607.00653

## Summary
- node2vec is an algorithm that aims to find node embeddings based on both
  homophily and structural similarity. Previous methods either use spectral methods which are infeasible for large networks, or are very rigid with high training time complexity.
- Firstly, we define our objective function to optimize

INSERT LATEX HERE

- We use the softmax function to get a probability distribution

INSERT LATEX HERE

- We perform a 2nd order random walk with unnormalized transition probabilities

INSERT LATEX HERE

## Strengths

## Weaknesses

## Improvements
