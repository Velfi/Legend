extends RigidBody3D
class_name Worm


@export
var speed = 20
@export
var accel = 10
@export
var destination_margin = 10


enum State {
	SLEEPING,
	WAITING,
	MOVE_TOWARDS,
	NAVIGATE_TO,
}


var _state = State.SLEEPING
var _destination


func _physics_process(delta: float) -> void:
	match _state:
		State.SLEEPING:
			sleep()
		State.WAITING:
			waiting()
		State.MOVE_TOWARDS:
			move_towards(delta)
		State.NAVIGATE_TO:
			navigate_to(delta)


func sleep():
	pass


func waiting():
	var direction = _destination - global_transform.origin
	# If we get too far away, start moving again
	if direction.length_squared() > destination_margin:
		_state = State.MOVE_TOWARDS
		print("move to")


func move_towards(delta: float):
	var direction: Vector3
	if _destination:
		direction = _destination - global_transform.origin
		# If we get to destination, quit walking
		if direction.length_squared() <= destination_margin:
			_state = State.WAITING
			print("wait")
			return
		direction = direction.normalized()
		
	if direction:
		#%AnimationTree["parameters/blend_position"] = linear_velocity.length() / 2.9
		var delta_v = linear_velocity.lerp(direction * speed, accel * delta)
		apply_central_force(delta_v)

		# If we're moving, rotate in that direction.
		turn_towards_movement_direction(delta_v)


func turn_towards_movement_direction(direction: Vector3):
	if not is_zero_approx(direction.length_squared()):
		look_at(global_transform.origin + direction.rotated(Vector3.UP, deg_to_rad(90)), Vector3.UP)


func navigate_to(_delta: float):
	pass
