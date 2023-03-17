# Fractal drum

## Grid constant

l = 0 => grid const = L

l = 1 => grid const = L/4
l = n => grid const = L/(4^n)

## Checking if a square is inside or outside

Note: the walls are already marked as walls
Note2: When talking about n: n is a number proportional to 1/lattice constant.

Method 1 (bfs/dfs):
    Begin at a point that we know is inside the fractal: the middle. Do a bfs/dfs where we do not cross the walls, and mark all found grid points as inside. All other points are outside. This assumes of course that the middle point is inside the fractal.

Method 2 (lt_slow - line trick slow):
    From a given point. Traverse a single direction until the end of the grid. Let the number of intersections with the fractal be N. If N is odd we are inside the drum, if N is even or zero we are outside. Note: needs double resolution (half grid constant) in order to add space between all walls in order to count them properly. This is O(n^3), but is O(n) if only looking at a single point (lazy).

Method 3 (lt - line trick):
    Iterate over entire grid. Count number of walls crossed for any given line. When number of walls crossed is odd we are inside, else we are outside. Only O(n^2). Still requires double resolution, is not lazy. I think it requires less memory than dfs or bfs since we dont need queue.


Timing:
```
Level 3
bfs single resolution | 1000000 loops: 39.8294176 µs
bfs double resolution | 1000 loops: 275.8226000000001 µs
dfs single resolution | 1000000 loops: 48.8377178 µs
dfs double resolution | 100 loops: 335.32000000000005 µs
lt double resolution | 1000000 loops: 121.31520490000001 µs
lt_slow double resolution | 100 loops: 4.201659 ms

Level 4
bfs single resolution | 1000 loops: 666.2706000000001 µs
bfs double resolution | 100 loops: 2.8250930000000003 ms
dfs single resolution | 100 loops: 832.682 µs
dfs double resolution | 100 loops: 3.46452 ms
lt double resolution | 100 loops: 1.524936 ms
lt_slow double resolution | 1 loops: 190.1811 ms

Level 5
bfs single resolution | 10 loops: 12.33178 ms
bfs double resolution | 10 loops: 49.52395 ms
dfs single resolution | 10 loops: 14.40914 ms
dfs double resolution | 10 loops: 53.323010000000004 ms
lt double resolution | 10 loops: 24.074039999999997 ms
lt_slow double resolution | 1 loops: 12.3228916 s

Level 6
bfs single resolution | 1 loops: 210.1188 ms
bfs double resolution | 1 loops: 909.714 ms
dfs single resolution | 1 loops: 210.53109999999998 ms
dfs double resolution | 1 loops: 855.6668000000001 ms
lt double resolution | 1 loops: 407.8406999999999 ms
lt_slow double resolution | dnf, too slow
```
