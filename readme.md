#get all notes
curl -X GET http://localhost:3000/notes

#delete note
curl -X POST http://localhost:3000/deleteNote/550e8400-e29b-41d4-a716-446655440000

#Update Notes:
curl -X POST -H "Content-Type: application/json" -d '{ ─╯
"id": "550e8400-e29b-41d4-a716-446655440000",
"title": "Apps",
"body": "Make Apps",
"created_by": "tom"
}' http://localhost:3000/updateNote/550e8400-e29b-41d4-a716-446655440000
