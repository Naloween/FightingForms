extends AspectRatioContainer

@export var board_size = 6
var margin = 10
var offset = 50 + margin
var tile_size = 100

var DAMAGE_TILE_SCENE = preload("res://Menus/Game/DamageTile.tscn")

var damage_tiles = []
var characters_node: Dictionary

func _ready():
	for i in range(board_size):
		damage_tiles.append([])
		for j in range(board_size):
			var tile: Node2D = DAMAGE_TILE_SCENE.instantiate()
			tile.position = Vector2(offset+tile_size*j, offset+tile_size*i)
			tile.visible = false
			$MarginContainer.add_child(tile)
			damage_tiles[-1].append(tile)

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
			set_character_position(character_node, position)
			character_node.set_size(tile_size)

func set_character_position(character_node: Node2D, new_position: FightingFormsPosition):
	character_node.position.x = offset + tile_size * new_position.x
	character_node.position.y = offset + tile_size * new_position.y

func show_damage_tile(position: Vector2i, damage: int):
	if position[0] < board_size && position[1]< board_size && position[0] >= 0 && position[1]>= 0:
		var tile = damage_tiles[position[1]][position[0]]
		tile.set_damage(damage)
		tile.visible = true

func hide_damage_tile(position: Vector2i):
	if position[0] < board_size && position[1] < board_size && position[0] >= 0 && position[1]>= 0:
		var tile = damage_tiles[position[1]][position[0]]
		tile.visible = false
