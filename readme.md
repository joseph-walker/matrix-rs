# Matrix RS

An exploration of linear algebra in Rust.

Contains:
- A library of useful structs and matrix operations.
- A REPL for playing with matrices and row reduction.

## REPL Example

```
> matrix 3 4 1 3 -2 5 3 5 6 7 2 4 3 8
Ok
> print
1       3       -2      5
3       5       6       7
2       4       3       8
Ok
> add 1 2 -3
Ok
> add 1 3 -2
Ok
> print
1       3       -2      5
0       -4      12      -8
0       -2      7       -2
Ok
> scale 2 -0.25
Ok
> print
1       3       -2      5
-0      1       -3      2
0       -2      7       -2
Ok
> add 2 3 2
Ok
> print
1       3       -2      5
-0      1       -3      2
0       0       1       2
Ok
> add 3 2 3
Ok
> add 3 1 2
Ok
> print
1       3       0       9
0       1       0       8
0       0       1       2
Ok
> add 2 1 -3
Ok
> print
1       0       0       -15
0       1       0       8
0       0       1       2
Ok
> quit
```
