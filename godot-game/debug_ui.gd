extends Control

func _ready() -> void:
	update_soulform_border()

func _process(_delta):
	# TODO does calling this every frame have a significant cost?
	update_soulform_border()

func _input(event):
	if event.is_action_pressed("menu"):
		_on_open_start_screen_pressed()

func update_soulform_border() -> void:
	# Update the border color based on the player's soul form state
	if %Player._is_in_soul_form:
		$SoulformBorder.show()
	else:
		$SoulformBorder.hide()

func _on_open_start_screen_pressed() -> void:
	# Ignore menu buttion presses if the start screen is already open
	if has_node("StartScreen"):
		return
	print("Opening start screen")
	var start_screen_scene = load("res://UI/StartScreen.tscn")
	var start_screen = start_screen_scene.instantiate()
	self.add_child(start_screen)
	get_tree().paused = true
