extends Node3D
class_name SingleUseGuard

signal self_destruct

var has_been_used = false

func handle_interact():
	if !has_been_used:
		for child in get_children():
			if child.has_method("handle_interact"):
				child.handle_interact()
		
	has_been_used = true

func provide_tooltip() -> String:
	for child in get_children():
		if child.has_method("provide_tooltip"):
			var tt = child.provide_tooltip()
			if tt and not tt.is_empty():
				return tt
	
	return ""

func hookup_self_destruct():
	for child in self.get_children():
		if child.has_signal("self_destruct"):
			child.connect("self_destruct", func (): self_destruct.emit())
