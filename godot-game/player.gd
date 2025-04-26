extends CharacterBody3D
class_name Player

signal toggle_soul_form

var SPEED = 3
var AIR_CONTROL = 1.0
var FRICTION = 10.0
var ACCELERATION = 5.0
var TURN_SPEED = 10.0

var gravity = ProjectSettings.get_setting("physics/3d/default_gravity")

var _is_in_soul_form = false

func _ready():
	# Feels better because players forget they were moving before a dialog
	# GlobalSignals.open_dialog_ui.connect(_reset_velocity_if_on_floor)
	%Hurtbox.receive_damage.connect(_on_receive_damage)
	%Hurtbox.receive_knockback.connect(_on_receive_knockback)
	pass

func _reset_velocity_if_on_floor():
	if is_on_floor():
		velocity = Vector3.ZERO

func _input(event):
	if event.is_action_pressed("quit"):
		get_tree().quit()

	if event.is_action_pressed("toggle_soul_form") and not event.is_echo():
		toggle_soul_form.emit()
		_toggle_soul_form()

	if event.is_action_pressed("interact"):
		if %InteractionRaycast.is_colliding():
			var collider = %InteractionRaycast.get_collider()
			print("interacting with", collider)

			if collider.has_method("handle_interact"):
				collider.handle_interact()
			else:
				print("no handle_interact on ", collider, "?")

	# if %InteractionRaycast.is_colliding():
	# 	var collider = %InteractionRaycast.get_collider()
	# 	if collider and collider.has_method("provide_tooltip"):
	# 		var tt = collider.provide_tooltip()
	# 		if tt: set_interaction_tooltip(tt)
	# 	else:
	# 		clear_interaction_tooltip()
	# else:
	# 	clear_interaction_tooltip()

var knockback_force_that_still_needs_to_be_applied = null

func _physics_process(delta):
	if not is_on_floor():
		velocity.y -= gravity * delta
	else:
		velocity.y = 0

	var direction = Vector3(
		Input.get_axis("ui_left", "ui_right"),
		0.0,
		Input.get_axis("ui_up", "ui_down")
	).normalized()

	# Slide movement input along floor for better slope handling
	if is_on_floor() and direction != Vector3.ZERO:
		direction = direction.slide(get_floor_normal())

	# Rotate player to face movement direction
	if is_on_floor() and direction.length_squared() > 0.0:
		transform = transform.interpolate_with(
			transform.looking_at(global_position + direction), delta * TURN_SPEED
		)

	if is_on_floor():
		if direction.length_squared() > 0.0:
			velocity.x = lerp(velocity.x, direction.x * SPEED, ACCELERATION * delta)
			velocity.z = lerp(velocity.z, direction.z * SPEED, ACCELERATION * delta)
		else:
			velocity.x = lerp(velocity.x, 0.0, FRICTION * delta)
			velocity.z = lerp(velocity.z, 0.0, FRICTION * delta)
	else:
		velocity.x = lerp(velocity.x, direction.x * AIR_CONTROL, delta)
		velocity.z = lerp(velocity.z, direction.z * AIR_CONTROL, delta)

	%AnimationTree["parameters/blend_position"] = velocity.length() / SPEED

	if knockback_force_that_still_needs_to_be_applied:
		velocity += knockback_force_that_still_needs_to_be_applied
		knockback_force_that_still_needs_to_be_applied = null

	move_and_slide()

	if Input.is_action_just_pressed("swing_b"):
		if is_on_floor():
			velocity = Vector3.ZERO
		%SwingAnimationPlayer.play("SwingBat")
		%SwingAnimationPlayer.queue("RESET")


# func _set_interaction_tooltip(text: String):
# 	GlobalSignals.set_interaction_tooltip.emit(text)

# func _clear_interaction_tooltip():
# 	GlobalSignals.clear_interaction_tooltip.emit()

func _toggle_soul_form():
	_is_in_soul_form = !_is_in_soul_form

	if _is_in_soul_form:
		_set_soul_form()
	else:
		_set_body_form()

func _set_soul_form():
	%BodyModel.visible = false
	%SoulModel.visible = true
	# TODO replace body marker with a ragdoll of Lala
	%BodyMarker.visible = true
	%BodyMarker.global_position = self.global_position
	self.collision_layer = 0b0101
	self.collision_mask = 0b0101
	# for mesh in find_children("*", "MeshInstance3D"):
	# 	mesh.mesh.material = preload("res://player_soul.material")

func _set_body_form():
	global_position = %BodyMarker.global_position
	%BodyModel.visible = true
	%SoulModel.visible = false
	%BodyMarker.visible = false
	PhysicsServer3D.body_set_state(
		get_rid(),
		PhysicsServer3D.BODY_STATE_TRANSFORM,
		Transform3D.IDENTITY.translated(%BodyMarker.global_position)
	)
	self.collision_layer = 0b0011
	self.collision_mask = 0b0011
	# for mesh in find_children("*", "MeshInstance3D"):
	# 	mesh.mesh.material = preload("res://player_body.material")

func _on_receive_damage(damage: float):
	print("Player received damage:", damage)

func _on_receive_knockback(force: Vector3):
	print("Player received knockback:", force)
	knockback_force_that_still_needs_to_be_applied = force
