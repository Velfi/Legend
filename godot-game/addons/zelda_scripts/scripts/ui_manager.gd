extends CanvasLayer
class_name UiManager

enum Overlay {
	Alert,
	ItemPickup,
}

enum Menu {
	Shop,
	Temple,
	Conversation,
	Inventory,
}

@onready var first_person_ui = %FirstPersonUi

@onready var alert_ui = %AlertUi
@onready var item_pickup_ui = %ItemPickupUi

@onready var shop_ui = %ShopUi
@onready var temple_ui = %TempleUi
@onready var conversation_ui = %ConversationUi
@onready var inventory_ui = %InventoryUi

func _ready():
	# Overlays
	GlobalSignals.open_item_pickup_ui.connect(func(item, quantity): open_overlay(Overlay.ItemPickup, {item: item, quantity: quantity}))
	GlobalSignals.close_item_pickup_ui.connect(func(): close_overlay(Overlay.ItemPickup))

	GlobalSignals.open_alert_ui.connect(func(data): open_overlay(Overlay.Alert, data))
	GlobalSignals.close_alert_ui.connect(func(): close_overlay(Overlay.Alert))

	# Menus
	GlobalSignals.open_shop_ui.connect(func(data): open_menu(Menu.Shop, data))
	GlobalSignals.close_shop_ui.connect(func(): close_menu(Menu.Shop))

	GlobalSignals.open_temple_ui.connect(func(data): open_menu(Menu.Temple, data))
	GlobalSignals.close_temple_ui.connect(func(): close_menu(Menu.Temple))

	GlobalSignals.open_conversation_ui.connect(func(data): open_menu(Menu.Conversation, data))
	GlobalSignals.close_conversation_ui.connect(func(): close_menu(Menu.Conversation))

	GlobalSignals.open_inventory_ui.connect(func(): open_menu(Menu.Inventory, null))
	GlobalSignals.close_inventory_ui.connect(func(): close_menu(Menu.Inventory))

	print(
		first_person_ui,
		alert_ui,
		item_pickup_ui,
		shop_ui,
		temple_ui,
		conversation_ui,
		inventory_ui,
	)

func _input(event):
	if first_person_ui.is_visible_in_tree() and not is_overlay_open():
		first_person_ui_handle_event(event)
	elif inventory_ui.is_visible_in_tree() and not is_overlay_open():
		inventory_ui_handle_event(event)
	elif item_pickup_ui.is_visible_in_tree():
		item_pickup_ui_handle_event(event)
	elif alert_ui.is_visible_in_tree():
		alert_ui_handle_event(event)

func capture_mouse():
	if Input.mouse_mode == Input.MOUSE_MODE_VISIBLE:
		Input.mouse_mode = Input.MOUSE_MODE_CAPTURED

func release_mouse():
	if Input.mouse_mode == Input.MOUSE_MODE_CAPTURED:
		Input.mouse_mode = Input.MOUSE_MODE_VISIBLE

func is_overlay_open():
	return alert_ui.is_visible_in_tree() \
		or item_pickup_ui.is_visible_in_tree()

func is_menu_open():
	return shop_ui.is_visible_in_tree() \
		or temple_ui.is_visible_in_tree() \
		or conversation_ui.is_visible_in_tree() \
		or inventory_ui.is_visible_in_tree()

func hide_all_uis():
	first_person_ui.hide()

	alert_ui.hide()
	item_pickup_ui.hide()

	shop_ui.hide()
	temple_ui.hide()
	conversation_ui.hide()
	inventory_ui.hide()

func open_overlay(overlay: Overlay, data):
	release_mouse()
	match overlay:
		Overlay.Alert:
			alert_ui.open(data)
		Overlay.ItemPickup:
			var item = data.get("item")
			var quantity = data.get("quantity")
			item_pickup_ui.open(item, quantity)

func close_overlay(overlay: Overlay):
	capture_mouse()
	match overlay:
		Overlay.Alert:
			alert_ui.close()
		Overlay.ItemPickup:
			item_pickup_ui.close()

func open_menu(menu: Menu, data):
	release_mouse()
	get_tree().paused = true
	first_person_ui.hide()

	match menu:
		Menu.Shop:
			shop_ui.open(data)
		Menu.Temple:
			temple_ui.open(data)
		Menu.Conversation:
			conversation_ui.open(data)
		Menu.Inventory:
			inventory_ui.open()

func close_menu(menu: Menu):
	capture_mouse()
	get_tree().paused = false
	first_person_ui.show()

	match menu:
		Menu.Shop:
			shop_ui.close()
		Menu.Temple:
			temple_ui.close()
		Menu.Conversation:
			conversation_ui.close()
		Menu.Inventory:
			inventory_ui.close()

# Remember to emit a signal instead of calling open/close directly.
# That way, other stuff can be notified too.

func first_person_ui_handle_event(event):
	if event.is_action_pressed("open_inventory"):
		GlobalSignals.open_inventory_ui.emit()

func inventory_ui_handle_event(event):
	if event.is_action_pressed("open_inventory"):
		GlobalSignals.close_inventory_ui.emit()

func item_pickup_ui_handle_event(event):
	if event.is_action_pressed("open_inventory"):
		GlobalSignals.close_item_pickup_ui.emit()
		GlobalSignals.open_inventory_ui.emit()

	if event.is_action_pressed("ui_accept"):
		GlobalSignals.close_item_pickup_ui.emit()

func alert_ui_handle_event(event):
	if event.is_action_pressed("ui_accept") \
		or event.is_action_pressed("ui_cancel"):
			GlobalSignals.close_alert_ui.emit()
