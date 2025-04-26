extends CharacterBody3D

var speed
const WALK_SPEED = 5.0
const SPRINT_SPEED = 9.0
const JUMP_VELOCITY = 4.8
const SENSITIVITY = 0.003

#bob variables
const BOB_FREQ = 2.4
const BOB_AMP = 0.08
var t_bob = 0.0

#fov variables
const BASE_FOV = 70.0
const FOV_CHANGE = 1

#interaction rays
const PICKUP_RAY_LENGTH = 1000.0

var gravity = ProjectSettings.get_setting("physics/3d/default_gravity")

@onready var head = $Head
@onready var camera = $Head/Camera3D
@onready var interaction_raycast = $Head/Camera3D/RayCast3D

func _ready():
	# Feels better because players forget they were moving before a dialog
	GlobalSignals.open_dialog_ui.connect(_reset_velocity_if_on_floor)

func _reset_velocity_if_on_floor():
	if is_on_floor():
		camera.fov = lerp(camera.fov, BASE_FOV, 1.0)
		velocity = Vector3.ZERO

func _unhandled_input(_event):
	pass

func _input(event):
	if Input.mouse_mode == Input.MOUSE_MODE_CAPTURED and grabbed_object == null:
		if event is InputEventMouseMotion:
			head.rotate_y(-event.relative.x * SENSITIVITY)
			camera.rotate_x(-event.relative.y * SENSITIVITY)
			camera.rotation.x = clamp(camera.rotation.x, deg_to_rad(-75), deg_to_rad(60))

	if event.is_action_pressed("toggle_show_mouse"):
		if Input.mouse_mode == Input.MOUSE_MODE_VISIBLE:
			Input.mouse_mode = Input.MOUSE_MODE_CAPTURED
		else: if Input.mouse_mode == Input.MOUSE_MODE_CAPTURED:
			Input.mouse_mode = Input.MOUSE_MODE_VISIBLE

	if event.is_action_pressed("escape"):
		get_tree().quit()

	if event.is_action_pressed("interact"):
		if interaction_raycast.is_colliding():
			var collider = interaction_raycast.get_collider()
			print("interacting with", collider)

			if collider.has_method("handle_interact"):
				collider.handle_interact()
			else:
				print("no handle_interact on ", collider, "?")

var grabbed_object = null

func _physics_process(delta):
	if Input.is_action_just_pressed("click"):
			if grabbed_object == null:
				if Input.mouse_mode == Input.MOUSE_MODE_VISIBLE:
					var mouse_pos = get_viewport().get_mouse_position()
					var from = camera.project_ray_origin(mouse_pos)
					var to = from + camera.project_ray_normal(mouse_pos) * PICKUP_RAY_LENGTH
					var space_state = get_world_3d().direct_space_state
					var query = PhysicsRayQueryParameters3D.create(from, to)

					var result = space_state.intersect_ray(query)
					var collider = result.get("collider")
					if collider:
						print("pickup")
						if collider.has_method("pickup"):
							collider.pickup()
							grabbed_object = collider
			else:
				print("put_down")
				if grabbed_object.has_method("put_down"):
					grabbed_object.put_down()
					grabbed_object = null
				else:
					printerr("can't put down %s, no `put_down` method" % grabbed_object, get_stack())

	if interaction_raycast.is_colliding():
		var collider = interaction_raycast.get_collider()
		if collider and collider.has_method("provide_tooltip"):
			var tt = collider.provide_tooltip()
			if tt: set_interaction_tooltip(tt)
		else:
			clear_interaction_tooltip()
	else:
		clear_interaction_tooltip()
	
	# Add the gravity.
	if not is_on_floor():
		velocity.y -= gravity * delta

	# Handle Jump.
	if Input.is_action_just_pressed("jump") and is_on_floor():
		velocity.y = JUMP_VELOCITY
	
	# Handle Sprint.
	if Input.is_action_pressed("sprint"):
		speed = SPRINT_SPEED
	else:
		speed = WALK_SPEED

	# Get the input direction and handle the movement/deceleration.
	var input_dir = Input.get_vector("move_left", "move_right", "move_forward", "move_back")
	var direction = (head.transform.basis * Vector3(input_dir.x, 0, input_dir.y)).normalized()
	if is_on_floor():
		if direction:
			velocity.x = direction.x * speed
			velocity.z = direction.z * speed
		else:
			velocity.x = lerp(velocity.x, direction.x * speed, delta * 7.0)
			velocity.z = lerp(velocity.z, direction.z * speed, delta * 7.0)
	else:
		velocity.x = lerp(velocity.x, direction.x * speed, delta * 3.0)
		velocity.z = lerp(velocity.z, direction.z * speed, delta * 3.0)
	
	# Head bob
	t_bob += delta * velocity.length() * float(is_on_floor())
	camera.transform.origin = _headbob(t_bob)
	
	# FOV
	var velocity_clamped = clamp(velocity.length(), 0.5, SPRINT_SPEED * 2)
	var target_fov = BASE_FOV + FOV_CHANGE * velocity_clamped
	camera.fov = lerp(camera.fov, target_fov, delta * 8.0)
	
	move_and_slide()

func set_interaction_tooltip(text: String):
	GlobalSignals.set_interaction_tooltip.emit(text)

func clear_interaction_tooltip():
	GlobalSignals.clear_interaction_tooltip.emit()

func _headbob(time) -> Vector3:
	var pos = Vector3.ZERO
	pos.y = sin(time * BOB_FREQ) * BOB_AMP
	pos.x = cos(time * BOB_FREQ / 2) * BOB_AMP
	return pos

func provide_inventory():
	return {}
