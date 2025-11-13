extends Control

@export var step = 0
@export var actions_class: Array[FightingFormsModuleClient.Types.ActionClass]
@export var choose_direction_node: Control
@export var choose_cardinal_direction_node: Control
@export var action: Option
@export var board: Control

var damage_tiles_to_show = []

func init():
	var k = 0
	for action_class in actions_class:
		var action_class_config = SpacetimeDB.FightingForms.db.action_class_config.action_class.find(action_class)
		$MenuButton.get_popup().add_item(action_class_config.name, k)
		k += 1
	$MenuButton.get_popup().add_item("No action", k)
	$MenuButton.get_popup().connect("id_pressed", _on_id_pressed)
	update()

func update():
	if action.is_some():
		var action: FightingFormsAction = action.unwrap()
		var action_class_config = SpacetimeDB.FightingForms.db.action_class_config.action_class.find(action.value)
		$MenuButton.text = "Step: "+str(step+1)+"\n"+action_class_config.name
	else:
		damage_tiles_to_show = []
		$MenuButton.text = "Step: "+str(step+1)+"\nNo action"

func _on_id_pressed(id: int):
	if id == len(actions_class):
		var action = Option.none()
		SpacetimeDB.FightingForms.reducers.choose_action(action, step)
	else:
		var action: FightingFormsAction
		var action_class = actions_class[id]
		var action_class_config = SpacetimeDB.FightingForms.db.action_class_config.action_class.find(action_class)
		
		var player = SpacetimeDB.FightingForms.db.player.id.find(SpacetimeDB.FightingForms.get_local_identity())
		var character = SpacetimeDB.FightingForms.db.character.id.find(player.character_id.unwrap())
		
		if action_class_config.name == "Move":
			var direction = await choose_direction()
			action = FightingFormsAction.create_move(FightingFormsMoveAction.create(direction))
		elif action_class_config.name == "Zytex1":
			damage_tiles_to_show = [
				[Vector2i(character.position.x-1, character.position.y), 1],
				[Vector2i(character.position.x, character.position.y-1), 1],
				[Vector2i(character.position.x+1, character.position.y), 1],
				[Vector2i(character.position.x, character.position.y+1), 1]]
			action = FightingFormsAction.create_zytex_1(FightingFormsZytex1Action.create())
		elif action_class_config.name == "Zytex2":
			var direction = await choose_direction()
			action = FightingFormsAction.create_zytex_2(FightingFormsZytex2Action.create(direction))
			damage_tiles_to_show=[]
			
			var delta = direction_to_delta(direction)
			var delta_x = delta.x
			var delta_y = delta.y

			for k in range(1,4):
				damage_tiles_to_show.append([
					Vector2i(character.position.x+delta_x*k, character.position.y+delta_y*k), 1
				])
			if delta_x ==0 or delta_y == 0:
				damage_tiles_to_show.append([
					Vector2i(character.position.x+delta_x*4, character.position.y+delta_y*4), 2
				])
		elif action_class_config.name == "Zytex3":
			var direction = await choose_direction()
			action = FightingFormsAction.create_zytex_3(FightingFormsZytex3Action.create(direction))
			var delta = direction_to_delta(direction)
			damage_tiles_to_show=[[
				Vector2i(character.position.x + delta.x, character.position.y + delta.y),1
			]]
			
		elif action_class_config.name == "Bardass1":
			action = FightingFormsAction.create_bardass_1(FightingFormsBardass1Action.create())
		elif action_class_config.name == "Bardass2":
			var direction = await choose_cardinal_direction()
			action = FightingFormsAction.create_bardass_2(FightingFormsBardass2Action.create(direction))
		elif action_class_config.name == "Bardass3":
			action = FightingFormsAction.create_bardass_3(FightingFormsBardass3Action.create())
		elif action_class_config.name == "Stunlor1":
			var direction = await choose_cardinal_direction()
			action = FightingFormsAction.create_stunlor_1(FightingFormsStunlor1Action.create(direction))
		elif action_class_config.name == "Stunlor2":
			var direction = await choose_cardinal_direction()
			action = FightingFormsAction.create_stunlor_2(FightingFormsStunlor2Action.create(direction))
		elif action_class_config.name == "Stunlor3":
			var direction = await choose_cardinal_direction()
			action = FightingFormsAction.create_stunlor_3(FightingFormsStunlor3Action.create(direction))
		
		SpacetimeDB.FightingForms.reducers.choose_action(Option.some(action), step)

func choose_direction() -> FightingFormsModuleClient.Types.Direction:
	choose_direction_node.visible = true
	var direction = await choose_direction_node.choosen_direction
	choose_direction_node.visible = false
	return direction

func choose_cardinal_direction() -> FightingFormsModuleClient.Types.CardinalDirection:
	choose_cardinal_direction_node.visible = true
	var direction = await choose_cardinal_direction_node.choosen_cardinal_direction
	choose_cardinal_direction_node.visible = false
	return direction


func _on_menu_button_mouse_entered() -> void:
	for damage_tile_to_show in damage_tiles_to_show:
		var position = damage_tile_to_show[0]
		var damage = damage_tile_to_show[1]
		board.show_damage_tile(position, damage)

func _on_menu_button_mouse_exited() -> void:
	for damage_tile_to_show in damage_tiles_to_show:
		var position = damage_tile_to_show[0]
		board.hide_damage_tile(position)

func direction_to_delta(direction) -> Vector2i:
	var delta_x = 0
	var delta_y = 0
	if direction == SpacetimeDB.FightingForms.Types.Direction.North:
		delta_y += -1
	if direction == SpacetimeDB.FightingForms.Types.Direction.NorthEast:
		delta_y += -1
		delta_x += 1
	if direction == SpacetimeDB.FightingForms.Types.Direction.East:
		delta_x += 1
	if direction == SpacetimeDB.FightingForms.Types.Direction.SouthEast:
		delta_x += 1
		delta_y += 1
	if direction == SpacetimeDB.FightingForms.Types.Direction.South:
		delta_y += 1
	if direction == SpacetimeDB.FightingForms.Types.Direction.SouthWest:
		delta_y += 1
		delta_x += -1
	if direction == SpacetimeDB.FightingForms.Types.Direction.West:
		delta_x += -1
	if direction == SpacetimeDB.FightingForms.Types.Direction.NorthWest:
		delta_y += -1
		delta_x += -1
	
	return Vector2i(delta_x, delta_y)
