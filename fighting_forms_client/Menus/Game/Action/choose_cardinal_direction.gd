extends Panel

signal choosen_cardinal_direction(direction: FightingFormsModuleClient.Types.CardinalDirection)

func _on_east_pressed() -> void:
	choosen_cardinal_direction.emit(FightingFormsModuleClient.Types.CardinalDirection.East)

func _on_west_pressed() -> void:
	choosen_cardinal_direction.emit(FightingFormsModuleClient.Types.CardinalDirection.West)

func _on_south_pressed() -> void:
	choosen_cardinal_direction.emit(FightingFormsModuleClient.Types.CardinalDirection.South)
	
func _on_north_pressed() -> void:
	choosen_cardinal_direction.emit(FightingFormsModuleClient.Types.CardinalDirection.North)
