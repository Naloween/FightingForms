class_name DamageTileEffect extends Node2D

static var scene = preload("res://Effects/DamageTile/DamageTileEffect.tscn")

var lifetime = 0.4
var age = 0

var tile_position: Vector2i
var board: Board
var damage: int

signal finished

static func create_damage_tile_effect(board: Board, position: Vector2i, damage: int) -> DamageTileEffect:
	var node: DamageTileEffect = scene.instantiate()
	node.tile_position = position
	node.damage = damage
	node.board = board
	return node

func _ready() -> void:
	board.show_damage_tile(tile_position, damage)

func _process(delta: float) -> void:
	age += delta
	if age > lifetime:
		board.hide_damage_tile(tile_position)
		finished.emit()
		queue_free()
