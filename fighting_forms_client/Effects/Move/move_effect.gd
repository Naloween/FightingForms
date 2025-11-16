class_name MoveEffect extends Node2D

static var scene = preload("res://Effects/Move/MoveEffect.tscn")

var character_node: Character
var lifetime = 1
var age = 0
var start_position: FightingFormsPosition
var end_position: FightingFormsPosition

signal finished

static func create_move_effect(character: Character, direction: FightingFormsModuleClient.Types.Direction, distance: int) -> MoveEffect:
	var node: MoveEffect = scene.instantiate()
	node.character_node = character
	node.start_position = character.get_character_position()
	var delta = distance*direction_to_delta(direction)
	node.end_position = FightingFormsPosition.create(node.start_position.x + delta.x, node.start_position.y + delta.y)
	
	return node

func _ready() -> void:
	pass

func _process(delta: float) -> void:
	var r = age/lifetime
	var new_position_x = start_position.x + r * (end_position.x - start_position.x)
	var new_position_y = start_position.y + r * (end_position.y - start_position.y)
	
	character_node.set_node_position(new_position_x, new_position_y)
	
	age += delta
	if age > lifetime:
		finished.emit()
		queue_free()

static func direction_to_delta(direction: FightingFormsModuleClient.Types.Direction) -> Vector2i:
	var delta = Vector2i(0, 0)
	
	if direction == FightingFormsModuleClient.Types.Direction.North:
		delta.y -= 1
	elif direction == FightingFormsModuleClient.Types.Direction.NorthEast:
		delta.y -= 1
		delta.x += 1
	elif direction == FightingFormsModuleClient.Types.Direction.East:
		delta.x += 1
	elif direction == FightingFormsModuleClient.Types.Direction.SouthEast:
		delta.x += 1
		delta.y += 1
	elif direction == FightingFormsModuleClient.Types.Direction.South:
		delta.y += 1
	elif direction == FightingFormsModuleClient.Types.Direction.SouthWest:
		delta.y += 1
		delta.x -= 1
	elif direction == FightingFormsModuleClient.Types.Direction.West:
		delta.x -= 1
	elif direction == FightingFormsModuleClient.Types.Direction.NorthWest:
		delta.y -= 1
		delta.x -= 1
	
	return delta
