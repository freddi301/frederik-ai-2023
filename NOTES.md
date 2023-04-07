# Concepts

- associative memory (recalling a knowledge links to other)
- correlation (an event happens in concert with others)
- induction (find a pattern from facts)
- deduction (create hypothetical fact from pattern)
- forecast (given some facts, what happens next?)
- generation (generate information from patterns)
- by analogy (apply a pattern on similar information)
- causality (classify if a pattern is correlation or causality)
- scientific method (given some facts, find patterns, ask for information needed to perfect patterns)
- classification (find out similar sets of events has same consequence)

# Tasks

- translation (english language -> agnostic language -> italian language)
- complete text
- summarize text
- query (get knowledge -> select and rearrange it based on a query)
- forecast event (given some events -> what happens next?)

# Training Data

https://dmf.unicatt.it/~della/pythoncourse18/commedia.txt
https://download.feedbooks.net/book/7384.epub?t=1548245072&filename=il-piccolo-principe

# Other

lazily sinthetize all possible predictions with no duplication

make it a lazy graph width predictions as nodes and edges that indicates fluctuations in complexity, accuracy, occurrences

sinthetize the less complex predictions with highest accuracy and occurrence

given some facts predict what happens next, with backtracking

- given current facts, predict most plausible one, add it to headings
- rank every heading with average accuracy based on all apredictions and predicted facts
- extend heading with highest
- present the sequence wiht highest accuracy as result

```graphql
mutation Train {
  train(
    textInputFilePath: "il-piccolo-principe.txt"
    csvOutputFilePath: "il-piccolo-principe.csv"
    jsonOutputFilePath: "il-piccolo-principe.json"
    modelOutputFilePath: "il-piccolo-principe.ron"
  ) {
    pattern
    accuracy
    conditionCount
    consequenceCount
  }
}

query Model {
  model(modelInputPath: "il-piccolo-principe.ron") {
    pattern
    accuracy
    conditionCount
    consequenceCount
  }
}
```
