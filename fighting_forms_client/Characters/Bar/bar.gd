extends Control
class_name Bar

var step = -1

@export var type: FightingFormsModuleClient.Types.JaugeType
var player_id: PackedByteArray

func _enter_tree() -> void:
	SpacetimeDB.FightingForms.db.character.on_update(_on_character_update)
	GlobalSignal.add_listener("new_step", _on_new_step)

func init(_player_id: PackedByteArray):
	player_id = _player_id
	var player = SpacetimeDB.FightingForms.db.player.id	.find(player_id)
	var character = SpacetimeDB.FightingForms.db.character.id.find(player.character_id.unwrap())
	
	update(character)
	

func _on_character_update(prev_character: FightingFormsCharacter, new_character: FightingFormsCharacter):
	if new_character.player_id == SpacetimeDB.FightingForms.get_local_identity():
		update(new_character)

func update(character: FightingFormsCharacter):

#	Remove any bar elts
	for child in $HBoxContainer.get_children():
		$HBoxContainer.remove_child(child)
		
#	Add full elts
	var node: Node
	var max_elts: int
	var current: int
	var character_state: FightingFormsCharacterState
	if step == -1:
		character_state = character.current_state
	else:
		character_state = character.states[step]
	if type == FightingFormsModuleClient.Types.JaugeType.Hp:
		node = $HP
		max_elts = character_state.max_hp
		current = character_state.hp
	elif type == FightingFormsModuleClient.Types.JaugeType.Mana:
		node = $Mana
		max_elts = character_state.max_mana
		current = character_state.mana
	else:
		node = $Stamina
		max_elts = character_state.max_stamina
		current = character_state.stamina
	for k in range(current):
		var new_node = node.duplicate()
		new_node.visible = true
		$HBoxContainer.add_child(new_node)
		
#	Add empty elements
	if type == FightingFormsModuleClient.Types.JaugeType.Hp:
		node = $HP_empty
	elif type == FightingFormsModuleClient.Types.JaugeType.Mana:
		node = $Mana_empty
	else:
		node = $Stamina_empty
	for k in range(max_elts-current):
		var new_node = node.duplicate()
		new_node.visible = true
		$HBoxContainer.add_child(new_node)

func _on_new_step(new_step: int):
	step = new_step
	var player = SpacetimeDB.FightingForms.db.player.id	.find(player_id)
	var character = SpacetimeDB.FightingForms.db.character.id.find(player.character_id.unwrap())
	
	update(character)

func _exit_tree() -> void:
	SpacetimeDB.FightingForms.db.character.remove_on_update(_on_character_update)
	GlobalSignal.remove_listener("new_step", _on_new_step)
