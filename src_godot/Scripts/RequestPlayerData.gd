extends Node

onready var global = get_node("/root/global")

var http: HTTPRequest
var worked: bool = true

func RequestPlayerData() -> bool:
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
		return false
	
	yield(http, "request_completed") # Wait for the request to complete
	return worked

func _on_request_completed(result, response_code, headers, body):
	var response = JSON.parse(body.get_string_from_utf8()).result
	
	print(response)
	
	http.queue_free() # Once it has nothing to do, it will be deleted
