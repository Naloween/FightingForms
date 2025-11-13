extends VBoxContainer

var prev_visible_node;

func _ready() -> void:
	prev_visible_node = $Empty

func init(player: FightingFormsPlayer):
	$PlayerIcon.init(player)

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
		
