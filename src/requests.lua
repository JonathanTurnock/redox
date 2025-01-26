---Global table for HTTP requests.
requests = {}

---Perform a GET request to a URL.
---@param url string The URL to request.
---@return string The response from the server.
function requests.get(url) end

---Perform a POST request to a URL.
---@param url string The URL to request.
---@param body string The body of the request.
---@return string The response from the server.
function requests.post(url, body) end

---Perform a PUT request to a URL.
---@param url string The URL to request.
---@param body string The body of the request.
---@return string The response from the server.
function requests.put(url, body) end

---Perform a DELETE request to a URL.
---@param url string The URL to request.
---@return string The response from the server.
function requests.delete(url) end

---Perform a PATCH request to a URL.
---@param url string The URL to request.
---@param body string The body of the request.
---@return string The response from the server.
function requests.patch(url, body) end

---Perform a HEAD request to a URL.
---@param url string The URL to request.
---@return string The response from the server.
function requests.head(url) end
