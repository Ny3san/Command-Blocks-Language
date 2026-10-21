# Network and APIs

The initial transport uses native Windows WinHTTP and is undergoing validation. It needs no browser or another installed language. Other platforms currently return an explicit incompatibility error.

```cb
permission net
variable response = net.get("https://example.com/")
print(response["status"])
print(response["body"])
```

Run with `cb run program.cb --allow net`. `rede` is the Portuguese module alias.

`net.get(url)` uses GET with a 10-second timeout. `net.request(method, url, body, headers, timeout_ms)` supports GET, POST, PUT, DELETE, PATCH and HEAD. The Portuguese alias is `rede.requisicao`. Both return `status` (integer) and `body` (UTF-8 text). HTTP 4xx/5xx responses are returned for the script to handle.

Headers accept text or `secret("ENVIRONMENT_VARIABLE")`. Secrets are sent only as the requested header value; never place credentials in source or URLs. Transport errors omit URL, body, headers and credentials.

Limits: 1 MB bodies, 64 headers, 16 KB header text, 10 requests/second per runtime (shared with the bridge), and a 100–60000 ms timeout per network operation. An elapsed-time cap is also checked between reads. HTTPS uses Windows certificate validation. Plain HTTP is allowed only for localhost and 127.0.0.1. IPv6, URL fragments and URL credentials are not yet supported.

Automatic redirects, cookies and Windows authentication are disabled. Redirects return 3xx responses; scripts must validate a destination before making another request. Binary downloads, streaming and response headers are not yet exposed. Network calls are external and cannot be converted into Minecraft.

Built against the [WinHTTP API](https://learn.microsoft.com/en-us/windows/win32/api/winhttp/nf-winhttp-winhttpopenrequest) and its [transport options](https://learn.microsoft.com/en-us/windows/win32/winhttp/option-flags).
