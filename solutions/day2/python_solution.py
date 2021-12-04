from collections import namedtuple
def move(current_position,instruction):
	direction,magnitude = instruction.split()
	magnitude = int(magnitude)
	if direction == "up":
		current_position[1] -= magnitude
	elif direction == "down":
		current_position[1] += magnitude
	elif direction == "forward":
		current_position[0] += magnitude
	else:
		raise NotImplemented
	return current_position

def move_logic2(current_position,instruction):
	direction,magnitude = instruction.split()
	magnitude = int(magnitude)
	if direction == "up":
		current_position = current_position._replace(aim = current_position.aim - magnitude)
	elif direction == "down":
		current_position = current_position._replace(aim = current_position.aim + magnitude)
	elif direction == "forward":
		current_position = current_position._replace(x = current_position.x + magnitude)
		current_position = current_position._replace(y = current_position.y + (current_position.aim * magnitude ))
	else:
		raise NotImplemented
	return current_position

Position2 = namedtuple("Position2",["x","y","aim"])

def main():
	fp = open("input.txt")
	lines = fp.readlines()
	fp.close()
	point = [0,0]
	for line in lines:
		point = move(point,line)
	
	print("Final Position :: ",point)
	print("Answer :: ",point[0]*point[1])
	
	point2 = Position2(0,0,0)
	for line in lines:
		point2 = move_logic2(point2,line)
	print("Final Position :: ",point2)
	print("Answer :: ",point2[0]*point2[1])
	

if __name__ == '__main__':
	main() 