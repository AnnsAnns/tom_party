extends Node

signal worked

onready var global = get_node("/root/global")
onready var GameData = get_node("/root/GameData")

var http: HTTPRequest
var worked: bool = true

func RequestPlayerData():
	# Create HTTPRequest Node
	http = HTTPRequest.new()
	add_child(http)
	http.connect("request_completed", self, "_on_request_completed")
	
	var request = {
		"uuid_game": global.session_id,
	}
	
	# Request Player Data
	var err = http.request(
		global.server_url + "games/lobby/request_player_data",
		global.user_agent + global.content_type, 
		global.use_ssl, 
		HTTPClient.METHOD_POST,
		JSON.print(request)
	)
	if err != OK:
		push_error(err)

func _on_request_completed(result, response_code, headers, body):
	var response = JSON.parse(body.get_string_from_utf8()).result
	
	print(response.keys())
	
	for player in response.keys():
		GameData.players[player] = player
	
	emit_signal("worked")
	
	http.queue_free() # Once it has nothing to do, it will be deleted
