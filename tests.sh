curl -v -d '{"login":"usr1","auth_type":"plain","roles":["admin","seller"]}' --header "Content-Type: application/json" -X PUT "http://localhost:8088/internal/auth"

curl -v -d '{"login":"usr1","auth_type":"plain"}' --header "Content-Type: application/json" -X GET "http://localhost:8088/internal/auth"
