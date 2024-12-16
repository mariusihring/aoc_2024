from collections import defaultdict, deque
from typing import Set, Dict, List, Tuple

def find_regions(grid: List[str]) -> Dict[str, List[Set[Tuple[int, int]]]]:
    rows, cols = len(grid), len(grid[0])
    visited = set()
    regions = defaultdict(list)
    
    def get_neighbors(r: int, c: int) -> List[Tuple[int, int]]:
        neighbors = []
        for dr, dc in [(0, 1), (1, 0), (0, -1), (-1, 0)]:
            nr, nc = r + dr, c + dc
            if 0 <= nr < rows and 0 <= nc < cols:
                neighbors.append((nr, nc))
        return neighbors
    
    def bfs(start_r: int, start_c: int, plant_type: str) -> Set[Tuple[int, int]]:
        region = set()
        queue = deque([(start_r, start_c)])
        
        while queue:
            r, c = queue.popleft()
            if (r, c) in visited:
                continue
                
            visited.add((r, c))
            region.add((r, c))
            
            for nr, nc in get_neighbors(r, c):
                if grid[nr][nc] == plant_type and (nr, nc) not in visited:
                    queue.append((nr, nc))
        
        return region

    for r in range(rows):
        for c in range(cols):
            if (r, c) not in visited:
                plant_type = grid[r][c]
                region = bfs(r, c, plant_type)
                regions[plant_type].append(region)
    
    return regions

def calculate_perimeter(region: Set[Tuple[int, int]], grid: List[str]) -> int:
    perimeter = 0
    rows, cols = len(grid), len(grid[0])
    
    for r, c in region:
        for dr, dc in [(0, 1), (1, 0), (0, -1), (-1, 0)]:
            nr, nc = r + dr, c + dc
            if (nr < 0 or nr >= rows or 
                nc < 0 or nc >= cols or 
                (nr, nc) not in region):
                perimeter += 1
    
    return perimeter

def p_1(input_data: str) -> int:
    grid = input_data.strip().splitlines()
    
    regions = find_regions(grid)
    
    total_price = 0
    for plant_type, plant_regions in regions.items():
        for region in plant_regions:
            area = len(region)
            perimeter = calculate_perimeter(region, grid)
            price = area * perimeter
            total_price += price
    
    return total_price

def main():
    with open("input", "r") as file:
        input_data = file.read()
    result = p_1(input_data)
    print(f"P1: {result}")

if __name__ == "__main__":
    main()
