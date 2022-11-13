package main

import (
	"bufio"
	"fmt"
	"net"
	"strings"
)

func main() {
	ln, err := net.Listen("tcp", ":8080")
	if err != nil {
		panic(err)
	}
	fmt.Println("listening on" + ln.Addr().String())

	defer ln.Close()

	for {
		conn, err := ln.Accept()
		if err != nil {
			fmt.Println(err)
			return
		}

		go func() {
			fmt.Println("connected to " + conn.RemoteAddr().String())

			defer conn.Close()

			sc := bufio.NewScanner(conn)
			for !sc.Scan() {
			}

			s := strings.Split(sc.Text(), " ")
			fmt.Printf("%#v", s)
			if len(s) < 3{
				fmt.Println("invalid request")
				return
			}

			method, path := s[0], s[1]

			if method == "GET" {
				fmt.Fprint(conn, "HTTP/1.1 200 OK\r\n")
				fmt.Fprint(conn, "Content-Type: text/html\r\n")
				fmt.Fprint(conn, "\r\n")
				fmt.Fprint(conn, "<p>anda mengakses ", path, "</p>")

			} else {
				fmt.Fprint(conn, "HTTP/1.1 405 Method Not Allowed\r\n")
				fmt.Fprint(conn, "\r\n")
			}
		}()
	}
}
