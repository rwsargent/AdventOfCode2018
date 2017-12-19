data = list(range(0,256))
pos = 0
skip = 0
lengths = [94,84,0,79,2,27,81,1,123,93,218,23,103,255,254,243]

def knot(pos, skip, data):
    for i in lengths:
        total_len = i + skip
        if total_len > data[pos:].length
            tmp = wrap(data, pos, i)
        else:
            ops

def wrap(seq, pos, length):
    tmp = []
    if length > seq.length:
        tmp.append(seq[pos:length+1])
        tmp.append(seq[length % seq.length:pos])
    else:
        tmp.append(seq)

    return tmp
    
    


    
