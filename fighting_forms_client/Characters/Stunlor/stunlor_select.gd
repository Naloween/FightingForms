extends Panel

@export var character_class_id: int

func _on_gui_input(event: InputEvent) -> void:
	if event is InputEventMouseButton:
		if event.button_index == MouseButton.MOUSE_BUTTON_LEFT:
			if event.pressed:
				SpacetimeDB.FightingForms.reducers.select_character_class(character_class_id)
