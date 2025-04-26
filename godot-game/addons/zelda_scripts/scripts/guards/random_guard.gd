extends Node3D

var rng = RandomNumberGenerator.new()

func _ready():
	rng.ready()

func handle_interact():
	var children = get_children().filter(func (child): child.has_method("handle_interact"))
	var random_child = children.pick_random()
	
	if random_child:
		random_child.handle_interact()

func provide_tooltip() -> String:
	var children = get_children().filter(func (child): child.has_method("provide_tooltip"))
	var random_child = children.pick_random()
	
	if random_child:
		return random_child.provide_tooltip()
	else:
		return ""
