# Fractal drum

## Grid constant

l = 0 => grid const = L

l = 1 => grid const = L/4
l = n => grid const = L/(4^n)

## Checking if a square is inside or outside

Note: the walls are already marked as walls
Note2: When talking about n: n is a number proportional to 1/lattice constant.

Method 1 (bfs/dfs - breadth-first search/depth-first search):
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

From timing i learned that my implementation of bfs is usually slifhtly faster than my implementation of dfs for some reason. I have no idea why. I also learned that for double resolution the line trick solution is always faster. I will be using this going forward since i think that it is the cleanest solution, and fits best with the problem description. The "line trick slow" solution is unsuprisingly much slower. This i believe to be for two reasons. Firstly it uses bounds checks due to the way the iteration is done. Secondly it has to iterate over many more grid points, being O(n^3). The pattern could be useful since it is possible to check if a single point is inside or outside the grid (would be O(n)), but is very slow for checking all points.

## Solving the wave equation

Found using level 4

| Eigenvalues | w/v   |
| ----------- | ----- |
| 88.86       | 9.42  |
| 200.00      | 14.14 |
| 207.76      | 14.41 |
| 209.93      | 14.49 |
| 226.87      | 15.06 |
| 310.89      | 17.63 | 
| 356.64      | 18.88 |
| 377.01      | 19.42 |
| 398.39      | 19.96 |
| 421.55      | 20.53 |
| 452.72      | 21.28 |
| 465.27      | 21.57 |
| 540.17      | 23.24 |


## Task 5

Solving at level 2 takes 0.05 s.
Solving at level 3 takes 0.42 s.
Solving at level 4 takes 120 s, and uses 143 MB memory.
I don't think solving at level 5 is practical for such an iterative process, but it takes XXX s and uses 1.7 GB memory which is about a tenth of my RAM
Solving at level 6 would not be practical. 

### What problems do we have with increasing level

In order to solve at a higher level we would need to reduce the ram usage and the cpu time. 

### How do we fix these problems

Since we can't be expected to improve upon the fortran code used in the solving algorithm, i suppose this must be done by shrinking the matrix. 

One way to shrink the matrix can be to remove all pairwise rows and cols that contain only zeros, which will be most of the rows and cols. I did implement this :^). For level 5 this reduces the shape of my matrix from about (3e6)^2 to about (1e6)^2. It now only uses 1 GB memory, which is suprisingly not that much less. It was also finally realistically possible to solve for this level, which took 21 minutes.

The solution speed of level 4 was reduced from 120 seconds to only 4. The shape of the matrix is here reduced from 182329^2 to 5734^2. Do note that both the original and the shrunk matrices are sparse, and are thus stored in some way where size does not nessecarily correlate with their shape. By checking the actual size of the matrix before and after shrinking, i confirmed that the size of the matrix' internal data (before being sent into the solver) was not changed. It is however very possible that this had an effect on some intermediary state of the data inside the eigenvalue algorithm.

This does however not make it "practical" to solve for levels 8 or 10, as it merely shifted the runtimes by one level. The time it used to take for level 4 it now takes for level 5.

Maybe there exists a way to solve this by only considering pairs of points on the boundary, or line segments between points on the boundary. If this is the case the amount of points in the matrix would be severely reduced.

