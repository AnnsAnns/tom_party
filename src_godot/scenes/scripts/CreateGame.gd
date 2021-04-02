extends Node

onready var global = get_node("/root/global")

func _on_CreateGame_pressed():
	var err = $CreateGame/CreateGameRequest.request(
		global.server_url + "games/lobby/init",
		global.user_agent, 
		global.use_ssl, 
		HTTPClient.METHOD_POST
	)
	if err != OK:
		push_error("Couldn't connect")

func _on_CreateGameRequest_request_completed(result, response_code, headers, body):
	var response = JSON.parse(body.get_string_from_utf8()).result
	
	print(response)

	if !response["worked"]:
		push_error("Server had issues creating the game!")

	global.session_id = response["uuid_game"]
	global.owner_id = response["uuid_owner"]
