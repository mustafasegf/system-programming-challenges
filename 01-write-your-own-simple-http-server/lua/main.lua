local socket = require("socket")
local server = assert(socket.bind("*", 8080))

local ip, port = server:getsockname()
print("listening on " .. ip .. ":" .. port)

while true do
	local conn = server:accept()
	local peerHost, peerPort = conn:getpeername()
	print("connected to " .. peerHost .. ":" .. peerPort)
	conn:settimeout(10)

	local line, err = conn:receive()
	if err then
		goto continue
	end

	local t = {}
	local size = 0
	for token in string.gmatch(line, "[^%s]+") do
		table.insert(t, token)
		size = size + 1
	end

	if size < 3 then
		goto continue
	end

	print(line)
	local method, path = t[1], t[2]

	if method == "GET" then
		conn:send("HTTP/1.1 200 OK\r\n")
		conn:send("Content-Type: text/html\r\n")
		conn:send("\r\n")
		conn:send("<p>anda mengakses " .. path .. "</p>")
	else
		conn:send("HTTP/1.1 405 Method Not Allowed\r\n")
		conn:send("\r\n")
	end

	::continue::
	conn:close()
end
