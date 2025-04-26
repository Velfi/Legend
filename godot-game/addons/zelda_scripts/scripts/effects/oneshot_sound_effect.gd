# extends Node3D b/c it emits positional sound
extends Node3D

@export
var audio_stream: AudioStream

func _ready():
	if audio_stream:
		%AudioStreamPlayer3D.stream = audio_stream
	else:
		printerr("an audio_stream is required but none was provided")

func handle_interact():
	if %AudioStreamPlayer3D.get("stream"):
		%AudioStreamPlayer3D.play()
	else:
		printerr("an audio_stream is required but none was provided")
