extends RigidBody3D
class_name Prop

# Props can be detected by zones, and the target name of the prop
# is used to determine if the zone is interested in the prop.
@export
var target_name = "unknown"

const PICKUP_SENSITIVITY = 0.01

var is_picked_up = false
var motion = null

func _ready():
	freeze_mode = RigidBody3D.FREEZE_MODE_KINEMATIC
	hookup_self_destruct()
	
	if self.collision_layer != 0b101000:
		printerr("Is %s in the right collision layers? It should be in 1 and 3", name)

func _input(event):
	if is_picked_up:
		if event is InputEventMouseMotion:
			motion = Vector3.ZERO
			motion.x = event.relative.x * PICKUP_SENSITIVITY
			motion.y = -event.relative.y * PICKUP_SENSITIVITY

func _physics_process(_delta):
	if motion:
		move_and_collide(motion)
		motion = null

func provide_target_name():
	return target_name

func pickup():
	Input.mouse_mode = Input.MOUSE_MODE_CAPTURED
	print("%s was picked up" % name)
	is_picked_up = true
	lock_rotation = true
	freeze = true

func put_down():
	Input.mouse_mode = Input.MOUSE_MODE_VISIBLE
	print("%s was put down" % name)
	is_picked_up = false
	lock_rotation = false
	freeze = false

# This section is shared with interactable.gd and reaction.gd,
# All three of these should be kept in sync

signal on_interact

# TODO this is broken when a guard is wrapping the SelfDestructEffect
func hookup_self_destruct():
	for child in self.get_children():
		if child.has_signal("self_destruct"):
			child.connect("self_destruct", self_destruct)

func handle_interact():
	on_interact.emit()
	for child in get_children():
		if child.has_method("handle_interact"):
			child.handle_interact()

func provide_tooltip() -> String:
	for child in get_children():
		if child.has_method("provide_tooltip"):
			var tt = child.provide_tooltip()
			if tt and not tt.is_empty():
				return tt

	return ""

func self_destruct():
	self.queue_free()
