# Library WebApi


```bash
for i in `seq 1 10000`; do curl -X POST -H "content-type: application/json" \
-d '{
        "title": "Title",
        "subject": "Subject",
        "author": "Author,
        "published_data": "To Day",
        "editor": "Editor"
}' \
http://localhost:3333/api/v1/book; done
```