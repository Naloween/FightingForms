class_name Tile extends Node2D

@export var damage = 0

func show_damage():
	$DamageTile.visible = true
	$EffectTile.visible = false

func show_effect():
	$DamageTile.visible = false
	$EffectTile.visible = true

func hide_damage():
	$DamageTile.visible = false

func hide_effect():
	$EffectTile.visible = false

func set_damage(dmg: int):
	damage = dmg
	$DamageTile/Label.text = str(damage)
