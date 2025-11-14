extends Node

var title_node: Title = preload("res://Menus/Title/title.tscn").instantiate()
var game_select_node: GameSelect = preload("res://Menus/GameSelect/game_select.tscn").instantiate()
var game_node: Game = preload("res://Menus/Game/game.tscn").instantiate()
var lobby_node: Lobby = preload("res://Menus/Lobby/lobby.tscn").instantiate()
var win_node: End = preload("res://Menus/End/Win.tscn").instantiate()
var loose_node: End = preload("res://Menus/End/Loose.tscn").instantiate()

func _ready():
#	SpacetimeDB setup

	# Connect to signals BEFORE connecting to the DB
	SpacetimeDB.FightingForms.connected.connect(_on_spacetimedb_connected)
	SpacetimeDB.FightingForms.disconnected.connect(_on_spacetimedb_disconnected)
	SpacetimeDB.FightingForms.connection_error.connect(_on_spacetimedb_connection_error)
	SpacetimeDB.FightingForms.transaction_update_received.connect(_on_transaction_update) # For reducer results

	var options = SpacetimeDBConnectionOptions.new()

	options.compression = SpacetimeDBConnection.CompressionPreference.NONE # Default
	# OR
	# options.compression = SpacetimeDBConnection.CompressionPreference.GZIP

	options.one_time_token = true # <--- anonymous-like. set to false to persist
	options.debug_mode = false # Default, set to true for verbose logging
	# Increase buffer size. In general, you don't need this.
	# options.set_all_buffer_size(1024 * 1024 * 2) # Defaults to 2MB

	# Disable threading (e.g., for web builds)
	# options.threading = false
	SpacetimeDB.FightingForms.connect_db(
		#"http://88.170.179.219:50002", # Base HTTP URL
		"http://localhost:3000", # Base HTTP URL
		"fighting-forms",             # Module Name
		options
	)
	
	# Connect signals
	title_node.play_signal.connect(_on_title_play_signal)
	game_select_node.enter_lobby.connect(_on_game_select_enter_lobby)
	lobby_node.game_started.connect(_on_lobby_game_started)
	game_node.win.connect(_on_game_win)
	game_node.loose.connect(_on_game_loose)
	win_node.restart.connect(_on_win_restart)
	loose_node.restart.connect(_on_loose_restart)

func _on_spacetimedb_connected(identity: PackedByteArray, token: String):
	print("Game: Connected to SpacetimeDB!")
	# Good place to subscribe to initial data
	var queries = [
		"SELECT * FROM player",
		"SELECT * FROM game",
		"SELECT * FROM action_class_config",
		"SELECT * FROM character_class",
		"SELECT * FROM character"
		]
	var subscription = SpacetimeDB.FightingForms.subscribe(queries)
	if subscription.error:
		printerr("Subscription failed!")
		return

	subscription.applied.connect(_on_subscription_applied)

func _on_subscription_applied():
	print("Game: Initial subscription applied.")
	#	Add title screen
	add_child(title_node)

func _on_spacetimedb_disconnected():
	print("Game: Disconnected.")

func _on_spacetimedb_connection_error(code, reason):
	printerr("Game: Connection Error (Code: %d): %s" % [code, reason])

func _on_transaction_update(update: TransactionUpdateMessage):
	# Handle results/errors from reducer calls
	if update.status.status_type == UpdateStatusData.StatusType.FAILED:
		printerr("Reducer call (ReqID: %d) failed: %s" % [update.reducer_call.request_id, update.status.failure_message])

# Menu Management #############

func _on_title_play_signal() -> void:
	remove_child(title_node)
	add_child(game_select_node)

func _on_game_select_enter_lobby(game_id: int) -> void:
	remove_child(game_select_node)
	lobby_node.game_id = game_id
	add_child(lobby_node)


func _on_lobby_game_started(game_id: int) -> void:
	remove_child(lobby_node)
	game_node.game_id = game_id
	add_child(game_node)

func _on_game_win() -> void:
	remove_child(game_node)
	add_child(win_node)

func _on_game_loose() -> void:
	remove_child(game_node)
	add_child(loose_node)

func _on_win_restart() -> void:
	remove_child(win_node)
	add_child(title_node)

func _on_loose_restart() -> void:
	remove_child(loose_node)
	add_child(title_node)
