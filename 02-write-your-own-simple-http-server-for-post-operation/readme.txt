In this second challenge, you are tasked to add more feature to the solution you created on from the first challenge.

Create a simple HTTP server which can process HTTP Post request and return response based on data from payload

Requirements:
Use of existing framework or high level library are not allowed. You must only use socket API for any languages of your choice to receive and send data from and to the network.
You should be able to handle POST request to / endpoint with some data as the the request body. The payload will be arbitrary string. After the request is processed, you should response with the base64 value from the request body.
Request
POST / http/1.1
...... //http headers

this week is crazy

Response
HTTP/1.1 200 OK
..... //http headers

dGhpcyB3ZWVrIGlzIGNyYXp5

If you have solution from first challenge, you may continue using that code. If not, you can create new code repository.
do not steal any code from internet

From the first challenge, I observed that some submitted solutions were able to create the server works  but without any reusable API for the server its self. That's totally fine. However, if you want, you can use this opportunity to take some time to think about what API that we can expose to users if we want someone else to use this? This is totally optional.

Hint: you might need some information from http headers to know how long the request body is.

When you are done, commit and push to github repository and share the repository here. Please don't add my name as part of your repository name. lol

If the solution is submitted before Tuesday, Nov 22th 2022, I will try to take a look and leave feedback if possible. #HappyCoding! 
