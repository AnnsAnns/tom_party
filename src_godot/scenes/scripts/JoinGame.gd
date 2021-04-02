extends Node

onready var global = get_node("/root/global")

signal joined_game(worked)

func JoinGame():	
	if global.name == "":
		print("No name found")
		emit_signal("joined_game", false)
		return
			
	if global.game_token == "":
		print("No Token Found")
		emit_signal("joined_game", false)
		return
	
	var request = {
		"invite_code": global.game_token.json_escape(),
		"name": global.username.json_escape()
	}
	
	print(request)
	
	var err = $JoinGameRequest.request(
	 global.server_url + "games/lobby/join_game",
	 global.user_agent + global.content_type,
	 global.use_ssl,
	 HTTPClient.METHOD_POST,
	 JSON.print(request)
	)
	if err != OK:
		push_error("Couldn't connect")
		emit_signal("joined_game", false)
	

func _on_JoinGameRequest_request_completed(result, response_code, headers, body):
	var response = JSON.parse(body.get_string_from_utf8()).result
	
	print(response)

	if !response["worked"]:
		print("Server had issues processing the join request")
		
		emit_signal("joined_game", false)
		return
		
	global.session_id = response["session_id"]
	global.user_id = response["user_id"]
	global.username = response["username"]
	
	emit_signal("joined_game", true)
