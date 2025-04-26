extends Node
class_name CallInteractHandlerEffect

@export var target: Node

func handle_interact():
    if target and target.has_method("_on_interact"):
        target._on_interact()
    else:
        printerr("please set a target")