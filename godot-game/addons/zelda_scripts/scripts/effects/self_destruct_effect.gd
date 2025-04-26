extends Node

signal self_destruct

func handle_interact():
	self_destruct.emit()
