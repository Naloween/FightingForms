extends Control

var period = 1
var remaining_time = 0
var k = 0

func _process(delta: float) -> void:
	remaining_time -= delta
	if remaining_time < 0:
		remaining_time = period
		k = (k +1) %4
		$Label.text = "Loading"
		for i in range(k):
			$Label.text += "."
