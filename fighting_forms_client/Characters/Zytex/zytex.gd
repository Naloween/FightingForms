extends Panel


func _on_mouse_entered() -> void:
	$Description.visible = true

func _on_mouse_exited() -> void:
	$Description.visible = false
