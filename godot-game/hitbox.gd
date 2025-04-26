extends Area3D
class_name Hitbox

@export var damage = 100

func deal_damage() -> float:
	return damage

@export var knockback_force = 10
@export var zero_y_axis_knockback = true

func knock_back(pos: Vector3):
	var direction = (pos - global_transform.origin).normalized()
	var knockback = direction * knockback_force

	if zero_y_axis_knockback:
		knockback.y = 0

	return knockback