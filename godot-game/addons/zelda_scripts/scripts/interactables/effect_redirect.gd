extends Node3D

@export
var source: Interactable

func _ready():
	if source:
		source.on_interact.connect(handle_interact)
	else:
		printerr("No source to spy on?!?", get_stack())

func handle_interact():
	for child in get_children():
		if child.has_method("handle_interact"):
			child.handle_interact()
