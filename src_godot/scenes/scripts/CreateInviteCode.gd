extends Node

onready var global = get_node("/root/global")

func _on_CreateInviteCode_pressed():
	if global.session_id == "" or global.owner_id == "":
		print("No session started yet")
		return
	
	var request = {
		"uuid_game": global.session_id,
		"uuid_owner": global.owner_id,
	}
	
	if global.game_token != "":
		request["old_token"] = global.game_token
	
	var err = $CreateInviteCode/CreateInviteRequest.request(
	 global.server_url + "games/lobby/create_invite",
	 global.user_agent + global.content_type,
	 global.use_ssl,
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
	
	global.game_token = response["invite_code"]
	$CreateInviteCode/Code.text = global.game_token
