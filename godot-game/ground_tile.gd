@tool
extends MeshInstance3D
class_name GroundTile

var _north_wall = true
@export var north_wall: bool = true:
	get:
		return _north_wall
	set(value):
		set_wall("north", value)

var _south_wall = true
@export var south_wall: bool = true:
	get:
		return _south_wall
	set(value):
		set_wall("south", value)

var _east_wall = true
@export var east_wall: bool = true:
	get:
		return _east_wall
	set(value):
		set_wall("east", value)

var _west_wall = true
@export var west_wall: bool = true:
	get:
		return _west_wall
	set(value):
		set_wall("west", value)

var _cell_name = ""
@export var cell_name = "":
	get:
		return _cell_name
	set(value):
		_cell_name = value
		%CenterLabel.text = value

func set_wall(wall: String, value: bool):
	var new_process_mode
	if (value):
		new_process_mode = PROCESS_MODE_INHERIT
	else:
		new_process_mode = PROCESS_MODE_DISABLED
	match wall:
		"north":
			_north_wall = value
			%NorthWall.visible = value
			%NorthWall/StaticBody3D.process_mode = new_process_mode
		"south":
			_south_wall = value
			%SouthWall.visible = value
			%SouthWall/StaticBody3D.process_mode = new_process_mode
		"east":
			_east_wall = value
			%EastWall.visible = value
			%EastWall/StaticBody3D.process_mode = new_process_mode
		"west":
			_west_wall = value
			%WestWall.visible = value
			%WestWall/StaticBody3D.process_mode = new_process_mode
