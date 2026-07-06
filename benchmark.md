# Benchmark – Subordinates

## Objective

Given the structure of a company represented as a tree, compute for every employee the number of subordinates.

### Input format

The program reads:

1. An integer $n$, the number of employees.
2. For employees $2$ to $n$, the parent of that employee.

Example:

```text
5
1 1 2 3
```

This means employee 2 has parent 1, employee 3 has parent 1, employee 4 has parent 2, and employee 5 has parent 3.

Two tree traversal algorithms are compared:

1. Recursive Depth-First Search (DFS)
2. Iterative Depth-First Search (DFS)

---

## Algorithm 1 – Recursive DFS

The recursive algorithm starts from the CEO (employee 1) and recursively visits every child. Each recursive call returns the size of the employee's subtree. The number of subordinates is obtained by subtracting one from the subtree size.

### Complexity

| Operation | Complexity |
|-----------|------------|
| Time | O(n) |
| Extra Space | O(n) |

### Advantages

- Very concise implementation.
- Naturally follows the recursive structure of trees.
- Easy to understand and implement.

### Disadvantages

- Uses the program call stack.
- Deep trees may lead to stack overflow in some environments.

---

## Algorithm 2 – Iterative DFS

The iterative algorithm replaces recursion with an explicit stack. It first performs a depth-first traversal to determine the processing order and then computes subtree sizes in reverse order.

### Complexity

| Operation | Complexity |
|-----------|------------|
| Time | O(n) |
| Extra Space | O(n) |

### Advantages

- Avoids recursion.
- Gives explicit control over memory usage.
- Suitable for environments where recursion depth is limited.

### Disadvantages

- Longer implementation.
- Requires managing an additional stack and traversal order.

---

## Comparison

| Feature | Recursive DFS | Iterative DFS |
|---------|---------------|---------------|
| Time Complexity | O(n) | O(n) |
| Extra Space | O(n) | O(n) |
| Uses Call Stack | Yes | No |
| Uses Explicit Stack | No | Yes |
| Simplicity | High | Moderate |

---

## Conclusion

Both algorithms correctly compute the number of subordinates for every employee in linear time.

The recursive implementation is shorter and closely matches the recursive nature of trees, making it easier to understand. The iterative implementation avoids recursion by using an explicit stack, providing greater control over memory usage while achieving the same time complexity.
