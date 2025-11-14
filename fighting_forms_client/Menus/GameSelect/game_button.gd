extends Button
class_name GameButton

var game_id: int

signal enter_game(int)

static func create_game_button(game_id: int) -> GameButton:
	var node = GameButton.new()
	node.game_id = game_id
	
	return node

func _ready() -> void:
	var game = SpacetimeDB.FightingForms.db.game.id.find(game_id)
	var creator = SpacetimeDB.FightingForms.db.player.id.find(game.creator_id)
	
	text = creator.name+"'s Game\n 1/4"

func _on_pressed() -> void:
	enter_game.emit(game_id)
