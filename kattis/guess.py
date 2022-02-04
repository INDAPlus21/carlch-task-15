while True:
    try:        n = int(input())
    except:     break
    if not n:   break

    stack       = list()
    queue       = list()
    prioq       = list()
    li          = list()

    dstruct = [True, True, True]

    for _ in range(n):
        line    = list(map(int, input().split(" ")))
        if line[0] == 1:
            stack.append(line[1])
            queue.append(line[1])
            prioq.append(line[1])
            li.append(line[1])
        elif line[0] == 2:
            prioq.sort()
            if len(li) == 0:
                dstruct = [False, False, False]
                break
            li.pop()

            if dstruct[0] == True:
                if line[1] != stack[-1]:
                    dstruct[0] = False
                stack.pop()

            if dstruct[1] == True:
                if line[1] != queue[0]:
                    dstruct[1] = False
                queue.pop(0)

            if dstruct[2] == True:
                if line[1] != prioq[-1]:
                    dstruct[2] = False
                prioq.pop()

    if      dstruct == [False, False, False]:
        print("impossible")
    elif    dstruct == [True, False, False]:
        print("stack")
    elif    dstruct == [False, True, False]:
        print("queue")
    elif    dstruct == [False, False, True]:
        print("priority queue")
    else:
        print("not sure")

