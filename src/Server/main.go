package main

// trigger workflow ||I|||

import (
	"crypto/rand"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"sync"
	"time"

	"github.com/gorilla/websocket"
)

const SERVER_PORT = ":8080"

var upgrader = websocket.Upgrader{
	ReadBufferSize:  2048 * 2048,
	WriteBufferSize: 2048 * 2048,
	CheckOrigin:     func(r *http.Request) bool { return true },
}

type Client struct {
	Conn     *websocket.Conn
	Username string
	Send     chan []byte
	IsHost   bool
}

type Room struct {
	mutex  sync.Mutex
	guests map[*Client]bool
	host   *Client
}

var rooms = make(map[string]*Room)
var roomsMutex sync.Mutex

func generateRoomID() string {
	bytes := make([]byte, 3)
	if _, err := rand.Read(bytes); err != nil {
		return "stream"
	}
	return hex.EncodeToString(bytes)
}

func main() {
	http.HandleFunc("/new", handleCreateRoom)
	http.HandleFunc("/host", handleHost)
	http.HandleFunc("/join", handleJoin)

	log.Printf("Signaling cluster processing live traffic on %s\n", SERVER_PORT)
	if err := http.ListenAndServe(SERVER_PORT, nil); err != nil {
		log.Fatal(err)
	}
}

func writePump(client *Client) {
	defer client.Conn.Close()

	for message := range client.Send {
		client.Conn.SetWriteDeadline(time.Now().Add(2 * time.Second))

		msgType := websocket.TextMessage
		if len(message) > 0 && message[0] != '{' && message[0] != '[' && message[0] != '"' {
			msgType = websocket.BinaryMessage
		}

		if err := client.Conn.WriteMessage(msgType, message); err != nil {
			return
		}
	}

	client.Conn.WriteMessage(websocket.CloseMessage, []byte{})
}

func handleCreateRoom(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Access-Control-Allow-Origin", "*")
	roomsMutex.Lock()
	roomID := generateRoomID()
	rooms[roomID] = &Room{
		guests: make(map[*Client]bool),
	}
	roomsMutex.Unlock()
	fmt.Fprint(w, roomID)
}

func broadcastUserList(room *Room) {
	room.mutex.Lock()
	var userList []string
	for client := range room.guests {
		userList = append(userList, client.Username)
	}
	host := room.host
	room.mutex.Unlock()

	packet := map[string]interface{}{
		"type":    "UserList",
		"payload": userList,
	}

	bytes, err := json.Marshal(packet)
	if err != nil {
		return
	}

	if host != nil {
		select {
		case host.Send <- bytes:
		default:
		}
	}
}

func handleHost(w http.ResponseWriter, r *http.Request) {
	roomID := r.URL.Query().Get("room")

	conn, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		return
	}

	client := &Client{Conn: conn, Send: make(chan []byte, 256), IsHost: true}
	go writePump(client)

	roomsMutex.Lock()
	room, exists := rooms[roomID]
	if !exists {
		room = &Room{guests: make(map[*Client]bool)}
		rooms[roomID] = room
	}
	roomsMutex.Unlock()

	room.mutex.Lock()
	room.host = client
	room.mutex.Unlock()

	for {
		msgType, payload, err := conn.ReadMessage()
		if err != nil {
			break
		}

		if msgType == websocket.TextMessage {
			var msg struct {
				Type    string          `json:"type"`
				Payload json.RawMessage `json:"payload"`
			}

			if err := json.Unmarshal(payload, &msg); err == nil && msg.Type == "KickUser" {
				var targetUser string
				if json.Unmarshal(msg.Payload, &targetUser) == nil {
					room.mutex.Lock()
					for guest := range room.guests {
						if guest.Username == targetUser {
							guest.Send <- []byte("KICKED")
							close(guest.Send)
							delete(room.guests, guest)
						}
					}
					room.mutex.Unlock()
					broadcastUserList(room)
				}
			}
			continue
		}

		room.mutex.Lock()
		for guest := range room.guests {
			select {
			case guest.Send <- payload:
			default:
				// full then maintain
			}
		}
		room.mutex.Unlock()
	}

	roomsMutex.Lock()
	delete(rooms, roomID)
	roomsMutex.Unlock()
	close(client.Send)
}

func handleJoin(w http.ResponseWriter, r *http.Request) {
	roomID := r.URL.Query().Get("room")
	username := r.URL.Query().Get("user")

	if username == "" {
		username = "Anonymous Guest"
	}

	conn, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		return
	}

	roomsMutex.Lock()
	room, exists := rooms[roomID]
	roomsMutex.Unlock()

	if !exists {
		conn.WriteMessage(websocket.TextMessage, []byte("Room not found"))
		conn.Close()
		return
	}

	// buffer of 60
	client := &Client{Conn: conn, Username: username, Send: make(chan []byte, 60)}

	room.mutex.Lock()
	room.guests[client] = true
	room.mutex.Unlock()

	go writePump(client)
	broadcastUserList(room)

	// alive loop
	for {
		if _, _, err := conn.ReadMessage(); err != nil {
			break
		}
	}

	room.mutex.Lock()
	if _, exists := room.guests[client]; exists {
		delete(room.guests, client)
		close(client.Send)
	}
	room.mutex.Unlock()

	broadcastUserList(room)
}
