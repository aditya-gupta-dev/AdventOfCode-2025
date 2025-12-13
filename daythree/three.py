def maxJolt(batteries: str) -> int: 
    required, max_jolt, start = 12, 0, 0
    
    for i in range(0, required):
        remain = required - i 
        end = required - remain + 1 
        
        max_digit = int(batteries[start])
        pos = start         
         
    return max_jolt 

with open('input-three.txt', 'r') as f: 
    total = 0
    for l in f: 
        total += maxJolt(l.rstrip())
    print(total)
