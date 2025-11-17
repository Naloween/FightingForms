extends AspectRatioContainer
class_name  Board

var board_size = 6
var margin = 10
var offset = 50 + margin
var tile_size = 100

var DAMAGE_TILE_SCENE = preload("res://Menus/Game/DamageTile.tscn")
var BLANK_TILE_SCENE = preload("res://Menus/Game/blank_tile.tscn")

var damage_tiles = []
var characters_node: Dictionary

func _enter_tree():
	var player: FightingFormsPlayer = SpacetimeDB.FightingForms.db.player.id.find(SpacetimeDB.FightingForms.get_local_identity())
	var game: FightingFormsGame = SpacetimeDB.FightingForms.db.game.id.find(player.game_id.unwrap())
	board_size = game.size
	
	$MarginContainer/GridContainer.columns = board_size
	
	for i in range(board_size):
		damage_tiles.append([])
		for j in range(board_size):
			var blank_tile: Control = BLANK_TILE_SCENE.instantiate()
			$MarginContainer/GridContainer.add_child(blank_tile)
			
			var tile: Node2D = DAMAGE_TILE_SCENE.instantiate()
			tile.position = Vector2(offset+tile_size*j, offset+tile_size*i)
			tile.visible = false
			$MarginContainer.add_child(tile)
			damage_tiles[-1].append(tile)
	
	SpacetimeDB.FightingForms.db.game.on_update(_on_game_update)
	
	_notification(NOTIFICATION_RESIZED)

func _notification(what):
	if damage_tiles.size() == 0:
		return
	if what == NOTIFICATION_RESIZED:
		tile_size = (min(size.x, size.y)-2*margin) / board_size
		offset = tile_size/2 + margin
		for i in range(board_size):
			for j in range(board_size):
				var tile: Node2D = damage_tiles[i][j]
				tile.position = Vector2(offset+tile_size*j, offset+tile_size*i)
				tile.scale = Vector2(tile_size/100, tile_size/100)
		
		for character_node in characters_node.values():
			var position = character_node.get_character_position()
			character_node.set_tile_size(tile_size)
			character_node.set_offset(offset)
			character_node.set_node_position(position.x, position.y)

func show_damage_tile(position: Vector2i, damage: int):
	if position[0] < board_size && position[1]< board_size && position[0] >= 0 && position[1]>= 0:
		var tile = damage_tiles[position[1]][position[0]]
		tile.set_damage(damage)
		tile.visible = true

func hide_damage_tile(position: Vector2i):
	if position[0] < board_size && position[1] < board_size && position[0] >= 0 && position[1]>= 0:
		var tile = damage_tiles[position[1]][position[0]]
		tile.visible = false

func hide_all_damage_tile():
	for line in damage_tiles:
		for tile in line:
			tile.visible = false

func _on_game_update(old_game: FightingFormsGame, new_game: FightingFormsGame):
	if new_game.round != old_game.round:
		hide_all_damage_tile()

func _exit_tree() -> void:
	SpacetimeDB.FightingForms.db.game.remove_on_update(_on_game_update)
	
	for i in range(len(damage_tiles)):
		for j in range(len(damage_tiles[0])):
			damage_tiles[i][j].queue_free()
	damage_tiles = []
	for child in $MarginContainer/GridContainer.get_children():
		child.queue_free()
