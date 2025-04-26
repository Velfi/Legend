extends Node
class_name AdhocTargetNameProvider

@export var target_name: String

func provide_target_name():
	if target_name and not target_name.is_empty():
		return target_name

	return null
