from collections import Counter,defaultdict

def prob1():
	fp = open("input.txt")
	numlen = None
	for linenum,line in enumerate( fp.readlines() ):
		binstr = line.strip()
		if linenum==0:
			numlen = len(binstr)
			location_counts = defaultdict(Counter)
		else:
			assert len(binstr)==numlen
		for i in range(numlen):
			location_counts[i].update([binstr[i]])
	more_common = ""
	less_common = ""
	for loc in location_counts:
		loc_counts = location_counts[loc]
		zeros_count = loc_counts["0"]
		ones_count = loc_counts["1"]
		if zeros_count>=ones_count:
			more_common+="0"
			less_common+="1"
		else:
			less_common+="0"
			more_common+="1"
	gamma = int(more_common,2)
	epsilon = int(less_common,2)
	print(gamma,epsilon,gamma*epsilon)

def prob2():
	pass

if __name__=="__main__":
	prob1()
