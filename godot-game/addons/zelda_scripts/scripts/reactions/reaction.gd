extends Area3D
class_name Reaction

func _ready():
	self.area_entered.connect(func (_area): handle_interact())
	hookup_self_destruct()

# This section is shared with prop.gd and interactable.gd,
# All three of these should be kept in sync

signal on_interact

func hookup_self_destruct():
	for child in self.get_children():
			if child.is_in_group("Effects"):
				if child.has_signal("self_destruct"):
					child.connect("self_destruct", self_destruct)

func handle_interact():
	on_interact.emit()
	for child in get_children():
		if child.has_method("handle_interact"):
			child.handle_interact()

func provide_tooltip() -> String:
	for child in get_children():
		if child.has_method("provide_tooltip"):
			var tt = child.provide_tooltip()
			if tt and not tt.is_empty():
				return tt

	return ""

func self_destruct():
	self.queue_free()
