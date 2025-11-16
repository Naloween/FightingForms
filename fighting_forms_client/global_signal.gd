extends Node

# Keeps track of what signal emitters have been registered.
var _emitters: Dictionary[String, Signal] = {}
# Keeps track of what listeners have been registered.
var _listeners: Dictionary[String, Array] = {}

# Register a signal with GlobalSignal, making it accessible to global listeners.
func add_emitter(emitter: Signal) -> void:
	var signal_name = emitter.get_name()
	_emitters[signal_name] = emitter
	if _listeners.has(signal_name):
		_connect_emitter_to_listeners(emitter)

# Adds a new global listener.
func add_listener(signal_name: String, listener: Callable) -> void:
	if not _listeners.has(signal_name):
		_listeners[signal_name] = []
	if not _listeners[signal_name].has(listener):
		_listeners[signal_name].push_back(listener)
		
	if _emitters.has(signal_name):
		_connect_listener_to_emitter(signal_name, listener)

# Connect an emitter to existing listeners of its signal.
func _connect_emitter_to_listeners(emitter: Signal) -> void:
	var signal_name = emitter.get_name()
	var listeners = _listeners[signal_name]
	for listener in listeners:
		emitter.connect(listener)

# Connect a listener to emitters who emit the signal it's listening for.
func _connect_listener_to_emitter(signal_name: String, listener: Callable) -> void:
	var emitter = _emitters[signal_name]
	emitter.connect(listener)

# Remove registered emitter and disconnect any listeners connected to it.
func remove_emitter(emitter: Signal) -> void:
	var signal_name = emitter.get_name()
	_emitters.erase(signal_name)

	if _listeners.has(signal_name):
		for listener: Callable in _listeners[signal_name]:
			if emitter.is_connected(listener):
				emitter.disconnect(listener)

# Remove registered listener and disconnect it from any emitters it was listening to.
func remove_listener(signal_name: String, listener: Callable) -> void:
	if not _listeners.has(signal_name): return
	if not _listeners[signal_name].has(listener): return  
	
	_listeners[signal_name].erase(listener)

	if _emitters.has(signal_name):
		var emitter = _emitters[signal_name]
		if emitter.is_connected(listener):
			emitter.disconnect(listener)
