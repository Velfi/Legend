extends Node3D

signal on_activate
signal on_deactivate

var _is_activated = false

func _ready():
	on_activate.connect(_on_activate)
	on_deactivate.connect(_on_deactivate)

func _on_activate():
	print("Gate %s activated" % self.name)
	%AudioStreamPlayer3D.stream = preload("res://Sounds/flutter-01.ogg")
	%AudioStreamPlayer3D.play()
	%Door.visible = false
	%Door.use_collision = false
	_is_activated = true

func _on_deactivate():
	print("Gate %s deactivated" % self.name)
	%AudioStreamPlayer3D.stream = preload("res://Sounds/bang-90.ogg")
	%AudioStreamPlayer3D.play()
	%Door.visible = true
	%Door.use_collision = true
	_is_activated = false

func _on_interact() -> void:
	if _is_activated == true:
		on_deactivate.emit()
	else:
		on_activate.emit()


# # Uncomment this for testing the activation/deactivation
# func _process(_delta):
# 	if Input.is_action_just_pressed("ui_accept"):
# 		if _is_activated:
# 			on_deactivate.emit()
# 		else:
# 			on_activate.emit()
