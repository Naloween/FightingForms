extends Node2D
class_name Character

var character_id: int
var step = -1
var offset = 50
var tile_size = 100

func _ready() -> void:
	GlobalSignal.add_listener("new_step", _on_new_step)
	SpacetimeDB.FightingForms.db.character.on_update(_on_character_update)

func init(new_character_id: int):
	character_id = new_character_id
	var character = SpacetimeDB.FightingForms.db.character.id.find(character_id)
	var character_class = SpacetimeDB.FightingForms.db.character_class.id.find(character.character_class_id)
	var player = SpacetimeDB.FightingForms.db.player.id.find(character.player_id)
	
	$HPBar.init(player.id)
	$HPBar.init(player.id)
	$ManaBar.init(player.id)
	$StaminaBar.init(player.id)
	
	$Pseudo.text = player.name
	
	if character_class.name == "Zytex":
		$Zytex.visible = true
	elif character_class.name == "Bardass":
		$Bardass.visible = true
	elif character_class.name == "Stunlor":
		$Stunlor.visible = true

func set_tile_size(size: int):
	scale = Vector2(size/100.0, size/100.0)
	tile_size = size

func set_offset(new_offset: int):
	offset = new_offset

func get_character_position() -> FightingFormsPosition:
	var character = SpacetimeDB.FightingForms.db.character.id.find(character_id)
	var character_state: FightingFormsCharacterState
	if step == -1:
		character_state = character.current_state
	else:
		character_state = character.states[step]
	return character_state.position

func set_node_position(new_position_x: float, new_position_y: float):
	position.x = offset + tile_size * new_position_x
	position.y = offset + tile_size * new_position_y

func show_bars():
	$HPBar.visible = true
	$ManaBar.visible = true
	$StaminaBar.visible = true

func hide_bars():
	$HPBar.visible = false
	$ManaBar.visible = false
	$StaminaBar.visible = false

func _on_new_step(new_step: int):
	step = new_step
	var pos = get_character_position()
	set_node_position(pos.x, pos.y)

func _on_character_update(old_character: FightingFormsCharacter, new_character: FightingFormsCharacter):
	var pos = get_character_position()
	set_node_position(pos.x, pos.y)

func _on_bardass_mouse_entered() -> void:
	show_bars()

func _on_bardass_mouse_exited() -> void:
	hide_bars()


func _on_zytex_mouse_entered() -> void:
	show_bars()


func _on_zytex_mouse_exited() -> void:
	hide_bars()


func _on_stunlor_mouse_entered() -> void:
	show_bars()


func _on_stunlor_mouse_exited() -> void:
	hide_bars()

func _exit_tree() -> void:
	GlobalSignal.remove_listener("new_step", _on_new_step)
