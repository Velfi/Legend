extends AnimatableBody3D

@export
var is_open = false
@export
var tooltip = "A locked door"
@export
var no_key_tooltip = "You don't have the key"

@export
var keys: Array[String] = [
	"skeleton_key", # A skeleton key opens every locked door
	"unlocked_objects" # Players even need a key for random shit
]

@onready
var animation_player = $AnimationPlayer
@onready
var interactable = $Interactable
@onready
var tooltip_provider = $Interactable/TooltipProvider

func _ready():
	tooltip_provider.tooltip_text = tooltip
	animation_player.stop()

func handle_interact():
	var player_has_key = false
	for key in keys:
		player_has_key = PersistentData.persistent_data.keys.get(key)
		if player_has_key:
			break
			
	if !player_has_key:
		tooltip_provider.tooltip_text = no_key_tooltip
		await get_tree().create_timer(1).timeout
		tooltip_provider.tooltip_text = tooltip
		return

	if animation_player.is_playing() or is_open:
		return
	else:
		is_open = true
		interactable.queue_free()
		animation_player.play("open")
		await get_tree().create_timer(1.1).timeout
		self.queue_free()
