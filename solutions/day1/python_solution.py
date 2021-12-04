
if __name__=="__main__":
	fp = open("input.txt")
	lines = fp.readlines()
	count = 0
	for i in range(1,len(lines)):
		first_measure = int(lines[i-1])
		second_measure = int(lines[i])
		if second_measure > first_measure:
			count += 1
	print("Solution 1 is :: ",count)

	counter2 = 0
	linesint = [int(x) for x in lines]
	for i in range(3,len(linesint)):
		first_window = linesint[i-3:i]
		assert len(first_window) == 3
		first_window_sum = sum(first_window)
		second_window = linesint[i-2:i+1]
		assert len(second_window) == 3
		second_window_sum = sum(second_window)
		if second_window_sum > first_window_sum:
			counter2 += 1
	print("Solution 2 is :: ",counter2)
	fp.close()
