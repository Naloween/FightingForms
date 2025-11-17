class_name PlayerSelect extends VBoxContainer

static var scene = preload("res://Menus/Lobby/PlayerSelection.tscn")

var prev_visible_node;
var player_id: PackedByteArray

static func create_player_select(player_id: PackedByteArray) -> PlayerSelect:
	var player = SpacetimeDB.FightingForms.db.player.id.find(player_id)

	var node: PlayerSelect = scene.instantiate()
	node.player_id = player_id
	node.init(player)
	return node

func init(player: FightingFormsPlayer):
	prev_visible_node = $Empty
	$PlayerIcon.init(player)
	if player.character_class_id.is_some():
		var character_class = SpacetimeDB.FightingForms.db.character_class.id.find(player.character_class_id.unwrap())
		select_character_class(character_class.name)

func set_connected(connected: bool):
	$PlayerIcon.set_connected(connected)

func select_character_class(name: String):
	if name == "Zytex":
		prev_visible_node.visible = false
		$Zytex.visible = true
		prev_visible_node = $Zytex
	if name == "Bardass":
		prev_visible_node.visible = false
		prev_visible_node = $Bardass
		$Bardass.visible = true
	if name == "Stunlor":
		prev_visible_node.visible = false
		prev_visible_node = $Stunlor
		$Stunlor.visible = true
		
