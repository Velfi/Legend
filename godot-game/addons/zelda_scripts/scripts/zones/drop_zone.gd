extends Area3D
class_name DropZone

## If true, show debug labels for DropZone state and targets list
@export var debug_mode = false

enum State {
	Empty,
	Partial,
	Full,
}

signal state_change(State)

var targets = []
var current = []

func _ready():
	if debug_mode:
		%Debug.show()
	else:
		%Debug.queue_free()

func update_targets():
	targets.clear()
	for child in self.get_children():
		if child.has_method("provide_target_name"):
			var target_name = child.provide_target_name()
			if target_name:
				targets.push_back(target_name.to_lower())

	if debug_mode:
		var list = "Target: "
		for t in targets:
			list += ("%s,\n" % t)
		%Target.text = list

# If a new child is added that provides a target name, then update our list
func _on_child_entered_tree(node):
	if node.has_method("provide_target_name"):
		update_targets()

# If a child is removed that was providing a target name, then update our list
func _on_child_exiting_tree(node):
	if node.has_method("provide_target_name"):
		update_targets()

func _on_body_entered(body):
	if body.has_method("provide_target_name"):
		var target_name = body.provide_target_name()
		if target_name:
			current.push_back(target_name.to_lower())
			_on_state_change()

func _on_body_exited(body):
	if body.has_method("provide_target_name"):
		var target_name = body.provide_target_name()
		if target_name:
			current.erase(target_name.to_lower())
			_on_state_change()

func _on_state_change():
	var targets_dup = targets.duplicate()
	for tn in current:
		targets_dup.erase(tn)

	if debug_mode:
		%State.text = "State: "
		var full_size = targets.size()

		match targets_dup.size():
			0:
				state_change.emit(State.Full)
				%State.text += ("Full")
			full_size:
				state_change.emit(State.Empty)
				%State.text += ("Empty")
			_:
				state_change.emit(State.Partial) 
				%State.text += ("Partial")
