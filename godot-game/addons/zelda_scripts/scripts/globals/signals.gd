extends Node

# Game

signal save_game(path: String)
signal load_game(path: String)
signal quit

# UI
# - managed by the UI manager

signal show_first_person_ui
signal hide_first_person_ui

signal open_shop_ui(data)
signal close_shop_ui

signal open_temple_ui(data)
signal close_temple_ui

signal open_conversation_ui(data)
signal close_conversation_ui

signal open_item_pickup_ui(data)
signal close_item_pickup_ui

signal open_alert_ui(data)
signal close_alert_ui

signal open_inventory_ui
signal close_inventory_ui

signal open_dialog_ui
signal close_dialog_ui

# HUD
# - managed by the player

signal set_interaction_tooltip(text: String)
signal clear_interaction_tooltip
signal set_temporary_tooltip_override(text: String)

# Player

signal player_pickup(item: JSON, quantity: int)
signal player_drop(item: JSON, quantity: int)
