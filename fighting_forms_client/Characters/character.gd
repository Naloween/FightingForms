extends Node2D

var character_id: int

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

func set_size(size: int):
	scale = Vector2(size/100.0, size/100.0)

func get_character_position() -> FightingFormsPosition:
	var character = SpacetimeDB.FightingForms.db.character.id.find(character_id)
	return character.position

func show_bars():
	$HPBar.visible = true
	$ManaBar.visible = true
	$StaminaBar.visible = true

func hide_bars():
	$HPBar.visible = false
	$ManaBar.visible = false
	$StaminaBar.visible = false

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
