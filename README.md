## tom_party [Placeholder Name]

This is an experimental project of mine. The idea is to create a party-game similar to Jackbox.

The server will be written in Rust, using Reddis as the in-memory database for it, while the game will be programmed using GDScript in the Godot engine.

### Requirements:

This needs:
 - Rust, including all dependencies in the cargo file
 - Redis
 - Godot

### Structure:

This server, written in Rust using Rocket, is located within the ```src``` folder. The folder and file structure relates to the API structure ([See: Backend API](https://github.com/tumGER/tom_party#backend-api)).

The Godot client, written in GDScript, is located within the ```src_godot``` folder. It follows the standard Godot structure of ```Scenes``` and ```Scripts``` with subfolders relating to their usage, such as ```Functions```, ```AutoLoad``` or ```UI```.

The redis DB follows the standard config and works out of the box.

## Frontend

![ye743GaJiH](https://user-images.githubusercontent.com/25822956/113868802-5c174800-97b0-11eb-9d9e-38207530e7d1.gif)

## Backend Api:
### Lobby:

All functions related to the lobby system

- ```/games/lobby/init```
    - Creates the lobby session
    - Returns:
        - uuid_game
        - uuid_owner
        - worked
- ```/games/lobby/create_invite```
    - Creates the invite code
    - Needs:
        - uuid_game
        - uuid_owner
    - Returns:
        - uuid_game
        - invite_code
        - worked
- ```/games/lobby/join_game```
    - Joins the game
    - Needs:
        - invite_code
        - name
    - Returns:
        - active_players
        - user_id
        - username
        - uuid_game
        - worked
- ```/games/lobby/request_player_data```
    - Returns all (public) player data
    - Needs:
        - uuid_game
    - Returns:
        - \* (For example: Connected, Score, etc.)
        - worked
- ```/games/lobby/heartbeat```
    - Makes sure the user is still connected and active, must be sent every ~3 minutes.
    - Needs:
        - uuid_game
        - user_id
        - username
    - Returns:
        - StatusCode

### Svar

All functions related to the game Svar, a game where you have to create the funniest reply to the question.

- ```/games/svar/get_question```
    - Gets the current question from the server
    - Needs:
        - uuid_game
    - Returns:
        - question
        - question_id
        - worked
- ```/games/svar/next_question```
    - Forces the server to change the current question to another one
    - Needs:
        - uuid_game
        - user_id
    - Returns:
        - question
        - question_id
        - worked
- ```/games/svar/answer```
    - Receives the answer to a question from the client
    - Needs:
        - uuid_game
        - user_id
        - question_id
        - answer
        - username
    - Returns:
        - worked

## License

This work is under the [European Union Public License v1.2](LICENSE) or – as soon they will be approved by the European Commission - subsequent versions of the EUPL (the "Licence");

You may get a copy of this license in your language from the European Commission [here](https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12).

Unless required by applicable law or agreed to in writing, software distributed under the Licence is distributed on an "AS IS" basis, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.

See the Licence for the specific language governing permissions and limitations under the Licence.
