extends Node3D

signal on_activate
signal on_deactivate

var _is_activated = false;

func _process(_delta: float) -> void:
	if %Top.position.y <= 0.13 and not _is_activated:
		_is_activated = true
		on_activate.emit()
	elif _is_activated:
		_is_activated = false
		on_deactivate.emit()
