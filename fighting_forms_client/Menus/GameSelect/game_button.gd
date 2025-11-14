extends Button
class_name GameButton

var game_id: int
var creator_name: String

signal enter_game(int)

static func create_game_button(game_id: int) -> GameButton:
	var node = GameButton.new()
	node.game_id = game_id
	
	return node

func _ready() -> void:
	SpacetimeDB.FightingForms.db.game.on_update(_on_game_update)
	
	var game = SpacetimeDB.FightingForms.db.game.id.find(game_id)
	var creator = SpacetimeDB.FightingForms.db.player.id.find(game.creator_id)
	creator_name = creator.name
	
	text = creator_name+"'s Game\n " + str(len(game.players)) + "/" + str(game.max_nb_players)

func _on_pressed() -> void:
	enter_game.emit(game_id)

func _on_game_update(old_game: FightingFormsGame, new_game: FightingFormsGame):
	text = creator_name+"'s Game\n " + str(len(new_game.players)) + "/" + str(new_game.max_nb_players)

func _exit_tree() -> void:
	SpacetimeDB.FightingForms.db.game.remove_on_update(_on_game_update)
