extends Node

@export
var json: JSON

func provide_tooltip() -> String:
	var tt = json.data.get("tooltip")
	if tt:
		return tt
	else:
		printerr("no key 'tooltip' in %s" % json)
		return ""
