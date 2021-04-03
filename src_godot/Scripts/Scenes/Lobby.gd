extends Node2D

onready var global = get_node("/root/global")

func processServerData():
	$PlayerAmountLabel.text = global.player_amount
