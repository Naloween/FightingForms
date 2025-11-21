class_name EffectTileEffect extends Node2D

static var scene = preload("res://Effects/EffectTile/EffectTileEffect.tscn")

var lifetime = 0.4
var age = 0

var tile_position: Vector2i
var board: Board

signal finished

static func create_effect_tile_effect(board: Board, position: Vector2i) -> EffectTileEffect:
	var node: EffectTileEffect = scene.instantiate()
	node.tile_position = position
	node.board = board
	return node

func _ready() -> void:
	board.show_effect_tile(tile_position)

func _process(delta: float) -> void:
	age += delta
	if age > lifetime:
		board.hide_effect_tile(tile_position)
		finished.emit()
		queue_free()
