import { BufReader } from "https://deno.land/std@0.163.0/io/mod.ts";

if (import.meta.main) {
  const listener = Deno.listen({ port: 8080 });
  if (!("hostname" in listener.addr)) {
    throw new Error("hostname is not available");
  }
  console.log(
    `Listening on port ${listener.addr.hostname}:${listener.addr.port}`,
  );
  for await (const conn of listener) {
    const bufReader = new BufReader(conn);
    const lineRes = await bufReader.readLine();
    if (lineRes === null) {
      break;
    }
    const { line } = lineRes;
    const [method, path, scheme] = new TextDecoder().decode(line).split(" ");
    if (scheme === undefined) {
      console.log("invalid request");
      continue;
    }

    if (method === "GET") {
      await Promise.all([
        conn.write(new TextEncoder().encode("HTTP/1.1 200 OK\r\n")),
        conn.write(new TextEncoder().encode("Content-Type: text/html\r\n")),
        conn.write(new TextEncoder().encode("\r\n")),
        conn.write(new TextEncoder().encode(`<p>anda mengakses ${path}</p>`)),
      ]);
    } else {
      await Promise.all([
        conn.write(
          new TextEncoder().encode("HTTP/1.1 405 Method Not Allowed\r\n"),
        ),
        conn.write(new TextEncoder().encode("\r\n")),
      ]);
    }
    conn.close();
  }
}
