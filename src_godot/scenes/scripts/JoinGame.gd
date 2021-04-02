extends Node

onready var global = get_node("/root/global")

func _on_JoinGame_pressed():
	var input_token = $JoinGame/Token
	
	if $JoinGame/Name.text == "":
		print("No name found")
		return
	else:
		global.username = $JoinGame/Name.text
			
	if global.game_token == "" and input_token == "":
		print("No Token Found")
		return
	elif global.game_token != "":
		input_token = global.game_token
	
	var request = {
		"invite_code": global.game_token,
		"name": global.username
	}
	
	var err = $JoinGame/JoinGameRequest.request(
	 global.server_url + "games/lobby/join_game",
	 global.user_agent + global.content_type,
	 global.use_ssl,
	 HTTPClient.METHOD_POST,
	 JSON.print(request)
	)
	if err != OK:
		push_error("Couldn't connect")
	

func _on_JoinGameRequest_request_completed(result, response_code, headers, body):
	var response = JSON.parse(body.get_string_from_utf8()).result
	
	print(response)

	if !response["worked"]:
		print("Server had issues processing the join request")
		
		if response.contains("error"):
			print(response["error"])
		return
		
	global.session_id = response["session_id"]
	global.user_id = response["user_id"]
	global.username = response["username"]
