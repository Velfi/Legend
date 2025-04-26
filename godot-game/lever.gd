extends Node3D

signal on_activate
signal on_deactivate

var _is_activated = false

func _ready():
	on_activate.connect(_on_activate)
	on_deactivate.connect(_on_deactivate)


func _on_activate():
	print("Lever %s activated" % self.name)
	%AnimationPlayer.play("ACTIVATE")
	_is_activated = true


func _on_deactivate():
	print("Lever %s deactivated" % self.name)
	%AnimationPlayer.play("DEACTIVATE")
	_is_activated = false


func _on_interact() -> void:
	if %AnimationPlayer.is_playing():
		return
	else:
		if _is_activated == true:
			on_deactivate.emit()
		else:
			on_activate.emit()
