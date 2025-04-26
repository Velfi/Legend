extends Area3D
class_name Hurtbox

signal receive_damage(damage: float)
signal receive_knockback(knockback: Vector3)

func _ready():
	self.area_entered.connect(_area_entered)

func _area_entered(area: Area3D):
	# Only receive hits from hitboxes
	if area is Hitbox:
		# that aren't in the same group
		for other_group in area.get_groups():
			if self.is_in_group(other_group):
				return

		if area.has_method("deal_damage"):
			var damage = area.deal_damage()
			receive_damage.emit(damage)

		if area.has_method("knock_back"):
			var knockback = area.knock_back(global_transform.origin)
			receive_knockback.emit(knockback)
