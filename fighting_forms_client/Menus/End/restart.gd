extends Control
class_name End

signal restart

func _on_go_title_pressed() -> void:
	restart.emit()
