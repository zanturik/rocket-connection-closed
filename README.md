Sometimes POST /unauthorized works as expected (forwarding and cached by 401), but sometimes connection just closed. 
The bigger file, more often connection closed. (Use some 2-6Mb size)

PS: See screenshot
PPS: curl:

```
curl --request POST \
  --url http://127.0.0.1:8000/unauthorized \
  --header 'Content-Type: multipart/form-data' \
  --header 'User-Agent: insomnium/0.2.3-a' \
  --form field=somefield \
  --form c=@/home/user/file.jpg
```