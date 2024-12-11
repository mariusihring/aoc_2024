from collections import Counter
from typing import List

def main():
    with open("input", "r") as file:
        data = file.read().strip().split()
    stones = Counter(data)

    for i in range(75):
        print(f"iteration: {i}")
        changes = Counter()
        for stone in list(stones.keys()):
            if stones[stone] > 0:
                count = stones[stone]
                stones[stone] = 0
                new_stones = transform_stone(stone)
                for new_stone in new_stones:
                    changes[new_stone] += count
        
        stones.update(changes)
    
    return sum(stones.values())

def transform_stone(stone: str) -> List[str]:
    if not stone or stone.isspace():
        return []
        
    if stone == "0":
        return ["1"]
    
    if len(stone) % 2 == 0:
        mid = len(stone) // 2
        left = stone[:mid].lstrip('0') or '0'
        right = stone[mid:].lstrip('0') or '0'
        return [left, right]
    
    return [str(int(stone) * 2024)]

if __name__ == "__main__":
    print(main())


