extends Node

# Game Data
var session_id: String
var owner_id: String
var user_id: String
var game_token: String
var username: String = "Test"

# Server Data
var server_url = "http://127.0.0.1:8000/"
var user_agent = ["user-agent: Godot Client"]
var content_type = ["Content-Type: application/json"]
var use_ssl = false

# Scene Data
var current_scene = null

func _ready():
	var root = get_tree().get_root()
	current_scene = root.get_child(root.get_child_count() - 1)

func goto_scene(path):
	call_deferred("_deferred_goto_scene", path) # Make sure the scene isn't running anything

func _deferred_goto_scene(path):
	current_scene.free() # Free old scene
	
	var s = ResourceLoader.load(path) # Load scene into memory
	current_scene = s.instance() # Instance new scene
	
	get_tree().get_root().add_child(current_scene) # Add new scene
	get_tree().set_current_scene(current_scene) # Compatibility
