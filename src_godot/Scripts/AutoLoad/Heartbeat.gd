extends Node

onready var global = get_node("/root/global")

var heartbeatTimer: Timer
var http: HTTPRequest

func _ready():
	http = HTTPRequest.new()
	add_child(http)
	http.connect("request_completed", self, "_on_request_completed")

	while true:
		yield(get_tree().create_timer(2.2*60), "timeout")

		if global.session_id == "": # No session = No heartbeat
			push_warning("Couldn't send heartbeat since no game is alive")
			continue
		
		var request = {
			"uuid_game": global.session_id,
			"user_id": global.user_id,
			"username": global.username
		}

		# Request Player Data
		var err = http.request(
			global.server_url + "games/lobby/heartbeat",
			global.user_agent + global.content_type, 
			global.use_ssl, 
			HTTPClient.METHOD_POST,
			JSON.print(request)
		)
		if err != OK:
			push_error(err)

func _on_request_completed(result, response_code, headers, body):
	if response_code != 200:
		push_error("Error sending the heartbeat! Server responded with: " + str(response_code))
