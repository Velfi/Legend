extends AnimatableBody3D

@export
var is_open = false

@onready
var animation_player = $AnimationPlayer
@onready
var right_side = $RightSide
@onready
var wrong_side = $WrongSide
@onready
var collision = $CollisionShape3D

func _ready():
	%RightSide.on_interact.connect(handle_interact)

func handle_interact():
	if animation_player.is_playing() or is_open:
		return
	else:
		is_open = true
		collision.queue_free()
		right_side.queue_free()
		wrong_side.queue_free()
		animation_player.play("open")
		await get_tree().create_timer(1.1).timeout
		self.queue_free()
