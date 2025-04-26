extends Node

var _inventory = {}

func _ready():
	GlobalSignals.player_pickup.connect(pickup_item)
	GlobalSignals.player_drop.connect(drop_item)

func save():
	return _inventory

func is_seen_before(item: JSON) -> bool:
	# If it's in the inventory, even at 0 quantity, we've seen it before
	return _inventory.get(item.data.itemName) != null

func has_room_for(_item: JSON, _quantity: int):
	# TODO actually check
	return true

func pickup_item(item: JSON, quantity: int):
	var current_quantity = _inventory.get(item.data.itemName)
	if current_quantity:
		_inventory[item.data.itemName] = current_quantity + quantity
	else:
		_inventory[item.data.itemName] = quantity

func drop_item(item: JSON, quantity: int):
	var itemName = item.data.itemName
	var current_quantity = _inventory.get(itemName) | 0
	if current_quantity >= quantity:
		_inventory[itemName] = current_quantity - quantity
		# TODO actually respawn the item in the world
	else:
		printerr("can't drop more (%s) than we have in inventory (%s)", quantity, current_quantity)
