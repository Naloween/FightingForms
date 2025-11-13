extends Node

# Spacetime DB Setup ##############

func _ready():
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
		"http://88.170.179.219:50002", # Base HTTP URL
		"fighting-forms",             # Module Name
		options
	)

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
	# Safe to query the local DB for initially subscribed data
	var initial_players = SpacetimeDB.FightingForms.db.player.iter()
	print("Initial players found: %d" % initial_players.size())
	var identity = SpacetimeDB.FightingForms.get_local_identity()
	var current_player = SpacetimeDB.FightingForms.db.player.id.find(identity)
	# ... setup initial game state ...

func _on_spacetimedb_disconnected():
	print("Game: Disconnected.")

func _on_spacetimedb_connection_error(code, reason):
	printerr("Game: Connection Error (Code: %d): %s" % [code, reason])

func _on_transaction_update(update: TransactionUpdateMessage):
	# Handle results/errors from reducer calls
	if update.status.status_type == UpdateStatusData.StatusType.FAILED:
		printerr("Reducer call (ReqID: %d) failed: %s" % [update.reducer_call.request_id, update.status.failure_message])
	elif update.status.status_type == UpdateStatusData.StatusType.COMMITTED:
		print("Reducer call (ReqID: %d) committed." % update.reducer_call.request_id)
		# Optionally inspect update.status.committed_update for DB changes

# Menu Management #############

func _on_title_play_signal() -> void:
	$Title.visible = false
	$GameSelect.init()
	$GameSelect.visible = true

func _on_game_select_enter_lobby(game_id: int) -> void:
	$GameSelect.visible = false
	$Lobby.visible = true
	$Lobby.game_id = game_id
	$Lobby.init()


func _on_lobby_game_started(game_id: int) -> void:
	$Game.game_id = game_id
	$Game.init()
	$Lobby.visible = false
	$Game.visible = true

func _on_game_win() -> void:
	$Win.visible = true
	$Game.visible = false

func _on_game_loose() -> void:
	$Loose.visible = true
	$Game.visible = false

func _on_win_restart() -> void:
	$Win.visible = false
	$Title.visible = true

func _on_loose_restart() -> void:
	$Win.visible = false
	$Title.visible = true
