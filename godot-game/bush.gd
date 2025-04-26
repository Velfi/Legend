extends Node3D

var _is_cut = false
@export var drop_table: JSON = preload ("res://drop_table_bush.json")
@export var health = 100

func _ready():
	$Hurtbox.receive_damage.connect(_on_receive_damage)

func _on_receive_damage(damage: float):
	if _is_cut:
		return

	health -= damage
	if health <= 0:
		_cut()

func _cut():
	_is_cut = true
	%BushTop.visible = false
	$StaticBody3D.queue_free()
	$Hurtbox.queue_free()
