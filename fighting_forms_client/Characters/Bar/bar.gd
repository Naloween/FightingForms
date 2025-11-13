extends Control

enum bar_type{
	HP,
	Mana,
	Stamina
}

@export var type: bar_type
var player_id: PackedByteArray

func init(player_id: PackedByteArray):
	var player = SpacetimeDB.FightingForms.db.player.id	.find(player_id)
	var character = SpacetimeDB.FightingForms.db.character.id.find(player.character_id.unwrap())
	
	update(character)
	
	SpacetimeDB.FightingForms.db.character.on_update(_on_character_update)

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
	if type == bar_type.HP:
		node = $HP
		max_elts = character.max_hp
		current = character.hp
	elif type == bar_type.Mana:
		node = $Mana
		max_elts = character.max_mana
		current = character.mana
	else:
		node = $Stamina
		max_elts = character.max_stamina
		current = character.stamina
	for k in range(current):
		var new_node = node.duplicate()
		new_node.visible = true
		$HBoxContainer.add_child(new_node)
		
#	Add empty elements
	if type == bar_type.HP:
		node = $HP_empty
	elif type == bar_type.Mana:
		node = $Mana_empty
	else:
		node = $Stamina_empty
	for k in range(max_elts-current):
		var new_node = node.duplicate()
		new_node.visible = true
		$HBoxContainer.add_child(new_node)
