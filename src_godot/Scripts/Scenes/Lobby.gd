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
	$RequestPlayerData.RequestPlayerData()

func _on_RequestPlayerData_worked():
	updateUiInfo()
