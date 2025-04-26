extends Node3D
class_name ToggleGuard

signal self_destruct

var current_index = 0

func handle_interact():
	var children = get_children().filter(func (it): it.has_method("handle_interact"))
	var child = children[current_index]
	if child:
		current_index += 1
		if current_index > children.length:
			current_index = 0
		child.handle_interact()

func provide_tooltip() -> String:
	var children = get_children().filter(func (it): it.has_method("provide_tooltip"))
	var child = children[current_index]
	if child:
		current_index += 1
		if current_index > children.length:
			current_index = 0
		return child.provide_tooltip()
	else:
		return ""

func hookup_self_destruct():
	for child in self.get_children():
		if child.has_signal("self_destruct"):
			child.connect("self_destruct", func (): self_destruct.emit())
