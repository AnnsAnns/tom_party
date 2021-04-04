extends Node2D

onready var global = get_node("/root/global")
onready var GameData = get_node("/root/GameData")

func updateUiInfo():
	$PlayerNamesLabel.text = ""
	$PlayerAmount.text = "Current Players: " + str(GameData.active_players)
	$InviteCode.text = "Invite Code: \n" + global.game_token
	
	for player in GameData.players.keys():
		$PlayerNamesLabel.text += player + "\n"

func _ready():
	$ChooseGameButton.add_item("Svar")
	
	$RequestPlayerData.RequestPlayerData()

func _on_RequestPlayerData_worked():
	updateUiInfo()


func _on_Button_pressed():
	get_tree().quit()


func _on_PlayButton_pressed():
	$ChooseGameButton.get_item_text($ChooseGameButton.selected)

	if "Svar":
		global.goto_scene("res://Scenes/Svar/Svar.tscn")
