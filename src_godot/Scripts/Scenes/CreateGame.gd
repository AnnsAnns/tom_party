extends Node

signal created_game(worked)

onready var global = get_node("/root/global")

func CreateGame():
	var err = $CreateGameRequest.request(
		global.server_url + "games/lobby/init",
		global.user_agent, 
		global.use_ssl, 
		HTTPClient.METHOD_POST
	)
	if err != OK:
		push_error("Couldn't connect")
		emit_signal("created_game", false)

func _on_CreateGameRequest_request_completed(result, response_code, headers, body):
	var response = JSON.parse(body.get_string_from_utf8()).result
	
	print(response)

	global.session_id = response["uuid_game"]
	global.owner_id = response["uuid_owner"]
	
	emit_signal("created_game", true)
