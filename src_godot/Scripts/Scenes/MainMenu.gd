extends Node2D

onready var global = get_node("/root/global")

func _on_JoinGameButton_pressed():
	$JoinGame.JoinGame()

func _on_JoinGame_joined_game(worked):
	if !worked:
		print("Couldn't join game!")
		return
		
	global.goto_scene("res://Scenes/UI/Lobby.tscn")

func _on_CreateGameButton_pressed():
	$CreateGame.CreateGame()

func _on_CreateGame_created_game(worked):
	if !worked:
		print("Error creating game")
		return
	
	$CreateInvite.CreateInvite()

func _on_CreateInvite_created_invite(worked):
	if !worked:
		print("Error creating invite")
		return
	
	$InviteCodeTbx.text = global.game_token
	print(global.game_token)
	
	$JoinGame.JoinGame()

func _on_QuitButton_pressed():
	get_tree().quit()

func _on_NameTbx_text_changed():
	global.username = $NameTbx.text
	print(global.username)

func _on_InviteCodeTbx_text_changed():
	global.game_token = $InviteCodeTbx.text
	print(global.game_token)

func _on_TestButton_pressed():
	$TestButton/RequestPlayerData.RequestPlayerData()
	
