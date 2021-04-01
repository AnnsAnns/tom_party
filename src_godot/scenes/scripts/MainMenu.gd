extends Node2D

var session_id: String
var owner_id: String
var user_id: String
var game_token: String
var username: String

var server_url = "http://127.0.0.1:8000/"
var user_agent = ["user-agent: Godot Client"]
var content_type = ["Content-Type: application/json"]
var use_ssl = false

func _on_CreateGame_pressed():
	var err = $CreateGame/CreateGameRequest.request(
		server_url + "games/replies/init",
		user_agent, 
		use_ssl, 
		HTTPClient.METHOD_POST
	)
	if err != OK:
		push_error("Couldn't connect")

func _on_CreateGameRequest_request_completed(result, response_code, headers, body):
	var response = JSON.parse(body.get_string_from_utf8()).result
	
	print(response)

	if !response["worked"]:
		push_error("Server had issues creating the game!")

	session_id = response["uuid_game"]
	owner_id = response["uuid_owner"]

func _on_CreateInviteCode_pressed():
	if session_id == "" or owner_id == "":
		print("No session started yet")
		return
	
	var request = {
		"uuid_game": session_id,
		"uuid_owner": owner_id,
	}
	
	if game_token != "":
		request["old_token"] = game_token
	
	var err = $CreateInviteCode/CreateInviteRequest.request(
	 server_url + "games/replies/create_invite",
	 user_agent + content_type,
	 use_ssl,
	 HTTPClient.METHOD_POST,
	 JSON.print(request)
	)
	if err != OK:
		push_error("Couldn't connect")
	

func _on_CreateInviteRequest_request_completed(result, response_code, headers, body):
	var response = JSON.parse(body.get_string_from_utf8()).result
	
	print(response)

	if !response["worked"]:
		push_error("Server had issues creating the invite code!")
	
	game_token = response["invite_code"]
	$CreateInviteCode/Code.text = game_token

func _on_JoinGame_pressed():
	var input_token = $JoinGame/Token
	
	if $JoinGame/Name.text == "":
		print("No name found")
		return
	else:
		username = $JoinGame/Name.text
			
	if game_token == "" and input_token == "":
		print("No Token Found")
		return
	elif game_token != "":
		input_token = game_token
	
	var request = {
		"invite_code": game_token,
		"name": username
	}
	
	var err = $JoinGame/JoinGameRequest.request(
	 server_url + "games/replies/join_game",
	 user_agent + content_type,
	 use_ssl,
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
		
	session_id = response["session_id"]
	user_id = response["user_id"]
	username = response["username"]
	
	$Pog.show()
