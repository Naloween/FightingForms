extends Control

@export var player_id: PackedByteArray

func init(player: FightingFormsPlayer):
	player_id = player.id
	set_player_name(player.name)
	SpacetimeDB.FightingForms.db.player.on_update(_on_player_update)

func set_player_name(name: String):
	$Label.text = name

func set_ready(ready: bool):
	if ready:
		$Ready.text = "✔️"
		$Ready.add_theme_color_override("font_color", Color(0.216, 0.535, 0.207, 1.0))
	else:
		$Ready.text = "❌"
		$Ready.add_theme_color_override("font_color", Color(0.641, 0.232, 0.17, 1.0))

func set_connected(connected: bool):
	if connected:
		$Connected.add_theme_color_override("bg_color", Color("#62aa4e"))
	else:
		$Connected.add_theme_color_override("bg_color", Color("cf4438ff"))


func _on_player_update(prev_player: FightingFormsPlayer, new_player: FightingFormsPlayer):
	if new_player.id == player_id:
		set_ready(new_player.ready)
		set_connected(new_player.connected)
