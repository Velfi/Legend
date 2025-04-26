extends Node3D

# @export var drop_table: JSON = preload ("res://drop_table_bush.json")
@export var health = 200

func _ready():
	$Hurtbox.receive_damage.connect(_on_receive_damage)
	$Hurtbox.receive_knockback.connect(_on_receive_knockback)

func _on_receive_damage(damage: float):
	health -= damage
	if health <= 0:
		_die()

func _on_receive_knockback(_knockback: Vector3):
	pass

func _die():
	self.queue_free()
