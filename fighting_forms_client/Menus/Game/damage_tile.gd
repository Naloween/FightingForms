extends Node2D

@export var damage = 0

func set_damage(dmg: int):
	damage = dmg
	$Label.text = str(damage)
