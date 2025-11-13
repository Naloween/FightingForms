extends Control

@export var game_id: int

signal enter_game(int)

func _on_pressed() -> void:
	enter_game.emit(game_id)
