extends Node3D

var _is_cut = false
@export var drop_table: JSON = preload ("res://drop_table_grass.json")
@export var health = 100

func _ready():
	$Hurtbox.receive_damage.connect(_on_receive_damage)

func _on_receive_damage(damage: float):
	health -= damage
	if health <= 0:
		_cut()

func _cut():
	_is_cut = true
	%GrassTop.visible = false
	%GrassBottom.visible = true
	$Hurtbox.queue_free()
