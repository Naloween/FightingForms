extends Control

signal enter_lobby(game_id: int)

var GAME_BUTTON_SCENE = preload("res://Menus/GameSelect/GameButton.tscn")

func init() -> void:
	for game in SpacetimeDB.FightingForms.db.game.iter():
		_on_insert_game(game)
		
	SpacetimeDB.FightingForms.db.game.on_insert(_on_insert_game)
	SpacetimeDB.FightingForms.db.game.on_update(_on_update_game)
	SpacetimeDB.FightingForms.db.game.on_delete(_on_delete_game)
	

func _on_create_game_button_pressed() -> void:
	SpacetimeDB.FightingForms.reducers.create_game()

func _on_enter_game(game_id: int):
	SpacetimeDB.FightingForms.reducers.select_game(game_id)

func _on_insert_game(insert_game: FightingFormsGame):
	if !insert_game.started:
		var node = GAME_BUTTON_SCENE.instantiate()
		node.game_id = insert_game.id
		node.connect("enter_game", _on_enter_game)
		$Games.add_child(node)
		if insert_game.players.has(SpacetimeDB.FightingForms.get_local_identity()):
			enter_lobby.emit(insert_game.id)
			SpacetimeDB.FightingForms.db.game.remove_on_insert(_on_insert_game)
			SpacetimeDB.FightingForms.db.game.remove_on_update(_on_update_game)
			SpacetimeDB.FightingForms.db.game.remove_on_delete(_on_delete_game)

func _on_update_game(prev_game: FightingFormsGame, new_game: FightingFormsGame):
	if new_game.started:
		_on_delete_game(new_game)
	if new_game.players.has(SpacetimeDB.FightingForms.get_local_identity()):
		enter_lobby.emit(new_game.id)
		SpacetimeDB.FightingForms.db.game.remove_on_insert(_on_insert_game)
		SpacetimeDB.FightingForms.db.game.remove_on_update(_on_update_game)
		SpacetimeDB.FightingForms.db.game.remove_on_delete(_on_delete_game)

func _on_delete_game(deleted_game: FightingFormsGame):
	for node in $Games.get_children():
		if node.game_id == deleted_game.id:
			$Games.remove_child(node)
	
