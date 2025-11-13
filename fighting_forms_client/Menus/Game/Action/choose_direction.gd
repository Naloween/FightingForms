extends Panel

signal choosen_direction(direction: FightingFormsModuleClient.Types.Direction)

func _on_east_pressed() -> void:
	choosen_direction.emit(FightingFormsModuleClient.Types.Direction.East)

func _on_west_pressed() -> void:
	choosen_direction.emit(FightingFormsModuleClient.Types.Direction.West)

func _on_south_pressed() -> void:
	choosen_direction.emit(FightingFormsModuleClient.Types.Direction.South)
	
func _on_north_pressed() -> void:
	choosen_direction.emit(FightingFormsModuleClient.Types.Direction.North)

func _on_south_east_pressed() -> void:
	choosen_direction.emit(FightingFormsModuleClient.Types.Direction.SouthEast)

func _on_north_west_pressed() -> void:
	choosen_direction.emit(FightingFormsModuleClient.Types.Direction.NorthWest)

func _on_south_west_pressed() -> void:
	choosen_direction.emit(FightingFormsModuleClient.Types.Direction.SouthWest)

func _on_north_east_pressed() -> void:
	choosen_direction.emit(FightingFormsModuleClient.Types.Direction.NorthEast)
