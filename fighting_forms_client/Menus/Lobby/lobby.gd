extends Control
class_name Lobby

var PLAYER_ICON_SCENE = preload("res://Menus/player_icon.tscn")

signal game_started(int)

@export var game_id: int = 0
var is_ready = false

var PLAYER_SELECT_SCENE = preload("res://Menus/Lobby/PlayerSelection.tscn")

func _ready():
	SpacetimeDB.FightingForms.db.game.on_update(_on_game_update)
	SpacetimeDB.FightingForms.db.player.on_update(_on_player_update)
	
	var game = SpacetimeDB.FightingForms.db.game.id.find(game_id)
	
#	Add players
	add_players(game)
	
	for character_class in SpacetimeDB.FightingForms.db.character_class.iter():
		if character_class.name == "Zytex":
			$CharactersClass/Zytex.character_class_id = character_class.id
		if character_class.name == "Stunlor":
			$CharactersClass/Stunlor.character_class_id = character_class.id
		if character_class.name == "Bardass":
			$CharactersClass/Bardass.character_class_id = character_class.id

func remove_players():
	for node in $Players.get_children():
		node.queue_free()

func add_players(game: FightingFormsGame):
	for player_id in game.players:
		var player = SpacetimeDB.FightingForms.db.player.id.find(player_id)
		
		var player_selection = PLAYER_SELECT_SCENE.instantiate()
		player_selection.init(player)
		
		$Players.add_child(player_selection)

func _on_ready_button_pressed() -> void:
	SpacetimeDB.FightingForms.reducers.ready(!is_ready)
	
	if is_ready:
		$ReadyButton.text = "Not Ready"
	else:
		$ReadyButton.text = "Ready"

func _on_game_update(prev_game: FightingFormsGame, new_game: FightingFormsGame):
	if new_game.id == game_id:
#		Start Game
		if new_game.started:
			game_started.emit(game_id)
#		Change participants
		remove_players()
		add_players(new_game)

func _on_player_update(prev_player: FightingFormsPlayer, new_player: FightingFormsPlayer):
	if new_player.id == SpacetimeDB.FightingForms.get_local_identity():
		is_ready = new_player.ready
	
#	If the character class changed
	if prev_player.character_class_id.is_none() and new_player.character_class_id.is_some() or prev_player.character_class_id.is_some() and prev_player.character_class_id.unwrap() != new_player.character_class_id.unwrap():
		for player_selection in $Players.get_children():
			if player_selection.get_node("./PlayerIcon").player_id == new_player.id:
				var character_class = SpacetimeDB.FightingForms.db.character_class.id.find(new_player.character_class_id.unwrap())
				player_selection.select_character_class(character_class.name)
#	Set connected of players
	for player_selection in $Players.get_children():
		if player_selection.get_node("./PlayerIcon").player_id == new_player.id:
			player_selection.set_connected(new_player.connected)

func _exit_tree() -> void:
	SpacetimeDB.FightingForms.db.game.remove_on_update(_on_game_update)
	SpacetimeDB.FightingForms.db.player.remove_on_update(_on_player_update)
