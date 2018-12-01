f = open('data/day2.txt')

largest=0
smallest=0
checksum=[]

for line in f:
    data = line.split('\t')
    data[-1] = data[-1].strip()
    largest = int(data[0])
    smallest = int(data[0])
    
    for x in data:
        if largest < int(x):
            largest = int(x)
        if smallest > int(x):
            smallest = int(x)

    checksum.append(largest-smallest)


print sum(checksum)

    
    
