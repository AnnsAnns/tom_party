extends Node

var session_id: String
var owner_id: String
var user_id: String
var game_token: String
var username: String

var server_url = "http://127.0.0.1:8000/"
var user_agent = ["user-agent: Godot Client"]
var content_type = ["Content-Type: application/json"]
var use_ssl = false
