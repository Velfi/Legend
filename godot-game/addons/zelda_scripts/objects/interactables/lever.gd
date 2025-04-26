extends Area3D

signal on_interact(bool)

@export
var single_use = true

@export
var toggle = false

@export
var tooltip_text: String = "Pull lever"

@onready
var animation_player = $AnimationPlayer

func provide_tooltip() -> String:
	# Don't return a tooltip if
	#     the animation is playing
	#         OR
	#     this lever is single use and was already toggled
	if animation_player.is_playing() or toggle and single_use:
		return ""
	else:
		return tooltip_text

func handle_interact():
	# Don't respond to an interact whenever
	#     the animation is playing
	#         OR
	#     this lever is single use and was already toggled
	if animation_player.is_playing() or toggle and single_use:
		return
	else:
		if toggle:
			animation_player.play("b")
		else:
			animation_player.play("a")

		await get_tree().create_timer(1.2).timeout

		toggle = !toggle
		on_interact.emit(toggle)
