extends Control
class_name Game

signal win
signal loose
signal new_step(step: int)

@export var game_id: int = 0

var PLAYER_ICON_SCENE = preload("res://Menus/player_icon.tscn")
var CHARACTER_SCENE = preload("res://Characters/Character.tscn")
var CHOOSE_ACTION_BUTTON_SCENE = preload("res://Menus/Game/Action/ChooseActionButton.tscn")

var characters = Dictionary() # Store each character node for each character id
var is_ready = false
var step = -1

func _enter_tree():
	SpacetimeDB.FightingForms.db.game.on_update(_on_game_update)
	SpacetimeDB.FightingForms.db.game.on_delete(_on_game_delete)
	SpacetimeDB.FightingForms.db.character.on_update(_on_character_update)
	SpacetimeDB.FightingForms.db.player.on_update(_on_player_update)
	
	GlobalSignal.add_emitter(new_step)
	
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
		var character_node: Character
		var character = SpacetimeDB.FightingForms.db.character.id.find(player.character_id.unwrap())
		var character_class = SpacetimeDB.FightingForms.db.character_class.id.find(character.character_class_id)
		
		character_node = CHARACTER_SCENE.instantiate()
		character_node.init(character.id)
		
		$HBoxContainer/Board/Board/MarginContainer/Characters.add_child(character_node)
		character_node.set_node_position(character.current_state.position.x, character.current_state.position.y)
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

func update_actions(character: FightingFormsCharacter):
	var k = 0
	for child in $HBoxContainer/SideMenu/SideMenuContainer/ChooseActions.get_children():
		child.action = character.choosen_actions[k]
		child.update()
		k += 1

func update_step(_step: int):
	if _step == 4:
		step = -1
		$HBoxContainer/SideMenu/SideMenuContainer/Step.text = "View step: Current"
	else:
		step = _step
		$HBoxContainer/SideMenu/SideMenuContainer/Step.text = "View step: "+str(step)
	new_step.emit(step)
	
	
# Events

func _on_game_update(prev_game: FightingFormsGame, new_game: FightingFormsGame):
	# Set round label
	$HBoxContainer/SideMenu/SideMenuContainer/Round.text = "Round: " + str(new_game.round)
	
	update_step(0)
	
#	Show effects
	for applied_effect in new_game.round_effects:
		if !applied_effect.applied:
			continue
		if applied_effect.step-1 != step:
			update_step(applied_effect.step-1)
			
		if applied_effect.effect.value == FightingFormsEffect.Cost:
			var cost_config = applied_effect.effect.get_cost()
			var node = CostEffect.create_cost_effect(cost_config.amount, cost_config.jauge_type)
			var character_node = characters.get(cost_config.character_id)
			node.position.y = -character_node.tile_size/2
			character_node.add_child(node)
			await node.finished
		elif applied_effect.effect.value == FightingFormsEffect.Move:
			var move_config = applied_effect.effect.get_move()
			var character_node = characters.get(move_config.character_id)
			var node = MoveEffect.create_move_effect(character_node, move_config.direction, move_config.distance)
			character_node.add_child(node)
			await node.finished
		elif applied_effect.effect.value == FightingFormsEffect.DamageTile:
			var damage_tile_config = applied_effect.effect.get_damage_tile()
			var node = DamageTileEffect.create_damage_tile_effect($HBoxContainer/Board/Board, Vector2i(damage_tile_config.position.x, damage_tile_config.position.y),
				damage_tile_config.amount)
			add_child(node)
			await node.finished
		elif applied_effect.effect.value == FightingFormsEffect.StatusTile:
			var status_tile_config = applied_effect.effect.get_status_tile()
			var node = EffectTileEffect.create_effect_tile_effect($HBoxContainer/Board/Board, Vector2i(status_tile_config.position.x, status_tile_config.position.y))
			add_child(node)
			await node.finished
	
	update_step(4)
	
			
func _on_game_delete(game: FightingFormsGame):
	if SpacetimeDB.FightingForms.db.player.id.find(SpacetimeDB.FightingForms.get_local_identity()).eliminated:
		loose.emit()
	else:
		win.emit()

func _on_character_update(prev_character: FightingFormsCharacter, new_character: FightingFormsCharacter):
	if new_character.player_id == SpacetimeDB.FightingForms.get_local_identity():
		update_actions(new_character)

func _on_player_update(prev_player: FightingFormsPlayer, new_player: FightingFormsPlayer):
	if new_player.id == SpacetimeDB.FightingForms.get_local_identity():
		is_ready = new_player.ready
		if is_ready:
			$HBoxContainer/SideMenu/SideMenuContainer/ReadyButton.text = "Not Ready"
		else:
			$HBoxContainer/SideMenu/SideMenuContainer/ReadyButton.text = "Ready"


func _on_h_scroll_bar_scrolling() -> void:
	var _step = $HBoxContainer/SideMenu/SideMenuContainer/HScrollBar.value
	update_step(_step)

func _on_quit_button_pressed() -> void:
	SpacetimeDB.FightingForms.reducers.quit_game()

func _on_ready_button_pressed() -> void:
	SpacetimeDB.FightingForms.reducers.ready(!is_ready)

func _exit_tree() -> void:
	SpacetimeDB.FightingForms.db.game.remove_on_update(_on_game_update)
	SpacetimeDB.FightingForms.db.game.remove_on_delete(_on_game_delete)
	SpacetimeDB.FightingForms.db.character.remove_on_update(_on_character_update)
	SpacetimeDB.FightingForms.db.player.remove_on_update(_on_player_update)
	
	GlobalSignal.remove_emitter(new_step)
	
#	Remove player nodes
	for child in $HBoxContainer/SideMenu/SideMenuContainer/players.get_children():
		child.queue_free()
		
#	Remove characters
	for character_node in characters.values():
		character_node.queue_free()
	characters.clear()
		
#		Remove Action
	for child in $HBoxContainer/SideMenu/SideMenuContainer/ChooseActions.get_children():
		child.queue_free()
