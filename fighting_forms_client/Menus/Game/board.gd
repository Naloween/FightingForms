extends AspectRatioContainer
class_name  Board

var board_size = 6
var margin = 10
var offset = 50 + margin
var tile_size = 100

var TILE_SCENE = preload("res://Menus/Game/Tiles/tile.tscn")

var tiles = []
var characters_node: Dictionary

func _enter_tree():
	var player: FightingFormsPlayer = SpacetimeDB.FightingForms.db.player.id.find(SpacetimeDB.FightingForms.get_local_identity())
	var game: FightingFormsGame = SpacetimeDB.FightingForms.db.game.id.find(player.game_id.unwrap())
	board_size = game.size
	
	for i in range(board_size):
		tiles.append([])
		for j in range(board_size):
			var tile: Node2D = TILE_SCENE.instantiate()
			tile.position = Vector2(offset+tile_size*j, offset+tile_size*i)
			$MarginContainer/Tiles.add_child(tile)
			tiles[-1].append(tile)
	
	SpacetimeDB.FightingForms.db.game.on_update(_on_game_update)
	
	_notification(NOTIFICATION_RESIZED)

func _notification(what):
	if tiles.size() == 0:
		return
	if what == NOTIFICATION_RESIZED:
		tile_size = (min(size.x, size.y)-2*margin) / board_size
		offset = tile_size/2 + margin
		for i in range(board_size):
			for j in range(board_size):
				var tile: Node2D = tiles[i][j]
				tile.position = Vector2(offset+tile_size*j, offset+tile_size*i)
				tile.scale = Vector2(tile_size/100, tile_size/100)
		
		for character_node in characters_node.values():
			var position = character_node.get_character_position()
			character_node.set_tile_size(tile_size)
			character_node.set_offset(offset)
			character_node.set_node_position(position.x, position.y)

func show_damage_tile(position: Vector2i, damage: int):
	if position[0] < board_size && position[1]< board_size && position[0] >= 0 && position[1]>= 0:
		var tile = tiles[position[1]][position[0]]
		tile.set_damage(damage)
		tile.show_damage()

func hide_damage_tile(position: Vector2i):
	if position[0] < board_size && position[1] < board_size && position[0] >= 0 && position[1]>= 0:
		var tile = tiles[position[1]][position[0]]
		tile.hide_damage()

func show_effect_tile(position: Vector2i):
	if position[0] < board_size && position[1]< board_size && position[0] >= 0 && position[1]>= 0:
		var tile = tiles[position[1]][position[0]]
		tile.show_effect()

func hide_effect_tile(position: Vector2i):
	if position[0] < board_size && position[1] < board_size && position[0] >= 0 && position[1]>= 0:
		var tile = tiles[position[1]][position[0]]
		tile.hide_effect()

func hide_all_damage_tile():
	for line in tiles:
		for tile in line:
			tile.hide_damage()

func _on_game_update(old_game: FightingFormsGame, new_game: FightingFormsGame):
	if new_game.round != old_game.round:
		hide_all_damage_tile()

func _exit_tree() -> void:
	SpacetimeDB.FightingForms.db.game.remove_on_update(_on_game_update)
	
	for i in range(len(tiles)):
		for j in range(len(tiles[0])):
			tiles[i][j].queue_free()
	tiles = []
