class_name CostEffect extends Node2D

static var scene = preload("res://Effects/Cost/cost.tscn")

var type: FightingFormsModuleClient.Types.JaugeType
var amount: int
var speed = 10
var lifetime = 1
var age = 0
var color: Color

signal finished

static func create_cost_effect(amount: int, type: FightingFormsModuleClient.Types.JaugeType) -> CostEffect:
	var node: CostEffect = scene.instantiate()
	node.type = type
	node.amount = amount
	return node

func _ready() -> void:
	$Label.text = "- "+str(amount)
	if type == FightingFormsModuleClient.Types.JaugeType.Hp:
		color = Color(0.677, 0.161, 0.174, 1.0)
		$Hp.visible = true
	elif type == FightingFormsModuleClient.Types.JaugeType.Mana:
		color = Color(0.221, 0.377, 0.636, 1.0)
		$Mana.visible = true
	elif type == FightingFormsModuleClient.Types.JaugeType.Stamina:
		color = Color(0.581, 0.474, 0.099, 1.0)
		$Stamina.visible = true
	$Label.add_theme_color_override("font_color", color)

func _process(delta: float) -> void:
	position.y -= delta*speed
	age += delta
	color.a = 1-age/lifetime
	$Label.add_theme_color_override("font_color", color)
	
	if age > lifetime:
		finished.emit()
		queue_free()
