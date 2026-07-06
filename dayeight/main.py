import math

class UnionFind:
    def __init__(self, size):
        self.parent = list(range(size))
        self.size = [1] * size

    def find(self, i):
        if self.parent[i] == i:
            return i
        self.parent[i] = self.find(self.parent[i])
        return self.parent[i]

    def union(self, i, j):
        root_i = self.find(i)
        root_j = self.find(j)
        
        if root_i != root_j:
            if self.size[root_i] < self.size[root_j]:
                root_i, root_j = root_j, root_i
            self.parent[root_j] = root_i
            self.size[root_i] += self.size[root_j]

def part_one(input_text, connections_to_make=1000):
    boxes = []
    for line in input_text.strip().split('\n'):
        if line.strip():
            x, y, z = map(int, line.strip().split(','))
            boxes.append((x, y, z))
            
    n = len(boxes)
    
    edges = []
    for i in range(n):
        for j in range(i + 1, n):
            dx = boxes[i][0] - boxes[j][0]
            dy = boxes[i][1] - boxes[j][1]
            dz = boxes[i][2] - boxes[j][2]
            dist_sq = dx**2 + dy**2 + dz**2
            edges.append((dist_sq, i, j))
            
    edges.sort(key=lambda x: x[0])
    
    uf = UnionFind(n)
    
    limit = min(connections_to_make, len(edges))
    for k in range(limit):
        dist_sq, u, v = edges[k]
        uf.union(u, v)
        
    circuit_sizes = []
    roots_seen = set()
    
    for i in range(n):
        root = uf.find(i)
        if root not in roots_seen:
            roots_seen.add(root)
            circuit_sizes.append(uf.size[root])
            
    circuit_sizes.sort(reverse=True)
    
    if len(circuit_sizes) >= 3:
        result = circuit_sizes[0] * circuit_sizes[1] * circuit_sizes[2]
    else:
        result = math.prod(circuit_sizes)
        
    return result

with open('input-eight.txt', 'r') as f:
    puzzle_input = f.read()
    answer = part_one(puzzle_input, connections_to_make=1000)
    print(f"Part 1 Answer: {answer}")
   