extends Node2D

onready var global = get_node("/root/global")
onready var GameData = get_node("/root/GameData")

var http: HTTPRequest

var question: String
var question_id: String
var answer: String

func nextQuestion():
	if global.owner_id != global.user_id:
		push_error("User isn't owner, no right to use this function!")
		return

	http = HTTPRequest.new()
	add_child(http)
	http.connect("request_completed", self, "_on_request_completed")

	var request = {
		"uuid_game": global.session_id,
		"user_id": global.user_id
	}

	# Request Player Data
	var err = http.request(
		global.server_url + "games/svar/next_question",
		global.user_agent + global.content_type, 
		global.use_ssl, 
		HTTPClient.METHOD_POST,
		JSON.print(request)
	)
	if err != OK:
		push_error(err)

func getQuestion():
	http = HTTPRequest.new()
	add_child(http)
	http.connect("request_completed", self, "_on_request_completed")

	var request = {
		"uuid_game": global.session_id,
	}

	# Request Player Data
	var err = http.request(
		global.server_url + "games/svar/get_question",
		global.user_agent + global.content_type, 
		global.use_ssl, 
		HTTPClient.METHOD_POST,
		JSON.print(request)
	)
	if err != OK:
		push_error(err)

func _on_request_completed(result, response_code, headers, body):
	if response_code != 200:
		push_error("Received response code: " + str(response_code))
		return

	var response = JSON.parse(body.get_string_from_utf8()).result
	
	if response["worked"] != true:
		push_error("The server has issues with the request: " + response["error"])
	
	question = response["question"]
	question_id = response["question_id"]
	
	http.queue_free() # Once it has nothing to do, it will be deleted

func submit():
	http = HTTPRequest.new()
	add_child(http)
	http.connect("request_completed", self, "_answer_on_request_completed")

	var request = {
		"uuid_game": global.session_id,
		"answer": answer,
		"question_id": question_id,
		"user_id": global.user_id,
		"username": global.username
	}

	# Request Player Data
	var err = http.request(
		global.server_url + "games/svar/answer",
		global.user_agent + global.content_type, 
		global.use_ssl, 
		HTTPClient.METHOD_POST,
		JSON.print(request)
	)
	if err != OK:
		push_error(err)

func _answer_on_request_completed(result, response_code, headers, body):
	if response_code != 200:
		push_error("Received response code: " + str(response_code))
		return

	var response = JSON.parse(body.get_string_from_utf8()).result
	
	if response["worked"] != true:
		push_error("The server has issues with the request: " + response["error"])
	
	http.queue_free() # Once it has nothing to do, it will be deleted

func _on_Submit_pressed():
	submit()

func _on_AnswerTextbox_text_changed():
	answer = $AnswerTextbox.text

func _ready():
	if global.owner_id != "":
		nextQuestion()
	else:
		getQuestion()
	
	$QuestionLabel.text = question
	
	$Timer.start
	
	while $Timer.time_left > 0:
		$Timer/Label.text = "Remaining Time: \n" + str($Timer.time_left) + " Seconds"
		yield(get_tree().create_timer(1), "timeout") # Sleep for another second
	
	# @TODO: Implement next scene and switch to it

func _on_UpdatePlayerData_timeout():
	$Lobby._ready()
