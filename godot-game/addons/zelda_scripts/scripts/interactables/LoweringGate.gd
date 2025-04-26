extends AnimatableBody3D

@export
var single_use = true

@export
var toggle = false

@onready var animation_player = $AnimationPlayer

func _ready():
		animation_player.stop()

func handle_interact():
	# Don't respond to an interact whenever
	#     the animation is playing
	#         OR
	#     this gate was already toggled
	if animation_player.is_playing() or toggle and single_use:
		return

	if toggle:
		animation_player.play("raise_gate")
	else:
		animation_player.play("lower_gate")
	
	toggle = !toggle
