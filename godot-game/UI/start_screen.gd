extends Control
class_name StartScreen

@onready var quit_confirm_modal = %QuitConfirmModal

func _ready() -> void:
	if quit_confirm_modal.visible:
		quit_confirm_modal.hide()

func _on_pressed_resume() -> void:
	get_tree().paused = false
	queue_free()

func _on_pressed_cancel_quit() -> void:
	quit_confirm_modal.hide()

func _on_pressed_quit() -> void:
	quit_confirm_modal.show()

func _on_pressed_quit_confirm() -> void:
	get_tree().quit()