extends Control

signal win
signal loose

@export var game_id: int = 0

var PLAYER_ICON_SCENE = preload("res://Menus/player_icon.tscn")
var CHARACTER_SCENE = preload("res://Characters/Character.tscn")
var CHOOSE_ACTION_BUTTON_SCENE = preload("res://Menus/Game/Action/ChooseActionButton.tscn")

var characters = Dictionary() # Store each character node for each character id
var is_ready = false

func init():
	SpacetimeDB.FightingForms.db.game.on_update(_on_game_update)
	SpacetimeDB.FightingForms.db.game.on_delete(_on_game_delete)
	SpacetimeDB.FightingForms.db.character.on_update(_on_character_update)
	SpacetimeDB.FightingForms.db.player.on_update(_on_player_update)
	
	var game = SpacetimeDB.FightingForms.db.game.id.find(game_id)
	
	var current_player_id = SpacetimeDB.FightingForms.get_local_identity()
	
	$HBoxContainer/SideMenu/SideMenuContainer/HP_Bar.init(current_player_id)
	$HBoxContainer/SideMenu/SideMenuContainer/Mana_Bar.init(current_player_id)
	$HBoxContainer/SideMenu/SideMenuContainer/Stamina_Bar.init(current_player_id)
	
#	Add connected players
	for player_id in game.players:
		var player = SpacetimeDB.FightingForms.db.player.id.find(player_id)
		
		# Add player icons
		var player_icon = PLAYER_ICON_SCENE.instantiate()
		player_icon.init(player)
		$HBoxContainer/SideMenu/SideMenuContainer/players.add_child(player_icon)
		
		# Add characters
		var character_node: Node2D
		var character = SpacetimeDB.FightingForms.db.character.id.find(player.character_id.unwrap())
		var character_class = SpacetimeDB.FightingForms.db.character_class.id.find(character.character_class_id)
		
		character_node = CHARACTER_SCENE.instantiate()
		character_node.init(character.id)
		
		$HBoxContainer/Board/Board/MarginContainer.add_child(character_node)
		$HBoxContainer/Board/Board.set_character_position(character_node, character.position)
		characters.set(character.id, character_node)
	
	$HBoxContainer/Board/Board.characters_node = characters
	
#	Add choosable actions
	var player = SpacetimeDB.FightingForms.db.player.id.find(SpacetimeDB.FightingForms.get_local_identity())
	var character_class = SpacetimeDB.FightingForms.db.character_class.id.find(player.character_class_id.unwrap())
	var character = SpacetimeDB.FightingForms.db.character.id.find(player.character_id.unwrap())
	
	for step in range(4):
		var action_button = CHOOSE_ACTION_BUTTON_SCENE.instantiate()
		action_button.step = step
		action_button.action = character.choosen_actions[step]
		action_button.actions_class = character_class.actions_class
		action_button.choose_direction_node = $ChooseDirection
		action_button.choose_cardinal_direction_node = $ChooseCardinalDirection
		action_button.board = $HBoxContainer/Board/Board
		action_button.init()
		$HBoxContainer/SideMenu/SideMenuContainer/ChooseActions.add_child(action_button)
	
func _on_game_update(prev_game: FightingFormsGame, new_game: FightingFormsGame):
	# Set round label
	$HBoxContainer/SideMenu/SideMenuContainer/Round.text = "Round: " + str(new_game.round)

func _on_game_delete(game: FightingFormsGame):
	exit()
	if SpacetimeDB.FightingForms.db.player.id.find(SpacetimeDB.FightingForms.get_local_identity()).eliminated:
		loose.emit()
	else:
		win.emit()


func _on_character_update(prev_character: FightingFormsCharacter, new_character: FightingFormsCharacter):
		var character_node: Node2D = characters.get(new_character.id)
		if character_node != null:
			$HBoxContainer/Board/Board.set_character_position(character_node, new_character.position)
			
			if new_character.player_id == SpacetimeDB.FightingForms.get_local_identity():
				update_actions(new_character)

func _on_player_update(prev_player: FightingFormsPlayer, new_player: FightingFormsPlayer):
	if new_player.id == SpacetimeDB.FightingForms.get_local_identity():
		is_ready = new_player.ready
		if is_ready:
			$HBoxContainer/SideMenu/SideMenuContainer/ReadyButton.text = "Not Ready"
		else:
			$HBoxContainer/SideMenu/SideMenuContainer/ReadyButton.text = "Ready"

func _on_button_pressed() -> void:
	SpacetimeDB.FightingForms.reducers.ready(!is_ready)

func update_actions(character: FightingFormsCharacter):
	var k = 0
	for child in $HBoxContainer/SideMenu/SideMenuContainer/ChooseActions.get_children():
		child.action = character.choosen_actions[k]
		child.update()
		k += 1

func exit():
	SpacetimeDB.FightingForms.db.game.remove_on_update(_on_game_update)
	SpacetimeDB.FightingForms.db.game.remove_on_delete(_on_game_delete)
	SpacetimeDB.FightingForms.db.character.remove_on_update(_on_character_update)
	SpacetimeDB.FightingForms.db.player.remove_on_update(_on_player_update)
