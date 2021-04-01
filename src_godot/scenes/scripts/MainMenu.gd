extends Node2D

var session_id
var owner_id
var game_token
var server_url = "http://localhost:8000/"
var header = ["user-agent: Godot Client"]
var use_ssl = false

func _on_CreateGame_pressed():
	var err = $CreateGame/CreateGameRequest.request("http://127.0.0.1:8000/games/replies/init", header, use_ssl, HTTPClient.METHOD_POST)
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
	pass

func _on_CreateInviteRequest_request_completed(result, response_code, headers, body):
	pass # Replace with function body.

func _on_JoinGame_pressed():
	pass # Replace with function body.

func _on_JoinGameRequest_request_completed(result, response_code, headers, body):
	pass # Replace with function body.
