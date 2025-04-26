extends Node

@export var timeline_name: String

func handle_interact():
	if timeline_name:
		Dialogic.start(timeline_name)
		get_viewport().set_input_as_handled()
	else:
		printerr("forgot to set a timeline_name", get_stack())
