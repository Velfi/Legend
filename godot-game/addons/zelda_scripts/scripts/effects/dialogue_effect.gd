extends Node
class_name DialogueEffect

#@export var Balloon: PackedScene = preload("res://ui/dialogue_balloon.tscn")
#@export var dialogue_resource: DialogueResource
#@export var startAtTitle: String = "START"

func _get_configuration_warnings():
	var warnings = []
	#if dialogue_resource == null:
		#warnings.append("\"dialogue_resource\" property needs a to point to a DialogueResource.")
	return warnings

func _ready():
	pass
	#DialogueManager.dialogue_ended.connect(_on_dialog_end)

func handle_interact():
	pass
	#show_dialogue(startAtTitle)
#
#func show_dialogue(key: String) -> void:
	#get_tree().paused = true
	#if Input.mouse_mode == Input.MOUSE_MODE_CAPTURED:
		#Input.mouse_mode = Input.MOUSE_MODE_VISIBLE
	#
	#if dialogue_resource == null:
		#printerr("missing dialog resource", get_stack())
		#return
#
	#GlobalSignals.open_dialog_ui.emit()
	#DialogueManager.show_dialogue_balloon_scene(Balloon, dialogue_resource, key)
#
#func _on_dialog_end(_foo):
	#get_tree().paused = false
	#if Input.mouse_mode == Input.MOUSE_MODE_VISIBLE:
		#Input.mouse_mode = Input.MOUSE_MODE_CAPTURED
	#
	#GlobalSignals.close_dialog_ui.emit()
