# DB related

## Regenerate entities
```
DATABASE_URL=sqlite:data.db?mode=rwc cargo run -p migration -- fresh  
sea-orm-cli generate entity --database-url sqlite:data.db -o entity/src --lib
rm data.db
```