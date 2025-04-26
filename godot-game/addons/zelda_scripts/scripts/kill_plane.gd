extends Area3D

func kill_on_collision(node):
	if node.has_method("die"):
		node.die()
	else:
		print("killplane tried to kill node %s but it wouldn't die" % node)
