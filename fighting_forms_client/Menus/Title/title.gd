extends Control

signal play_signal

func _on_play() -> void:
	SpacetimeDB.FightingForms.reducers.set_name($TextEdit.text);
	play_signal.emit()
