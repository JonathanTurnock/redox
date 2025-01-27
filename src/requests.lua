---Global table for HTTP requests.
requests = {}

---Create a new requests client for POST, PUT, PATCH, DELETE operations.
---@return table The requests client.
function requests.create_client()
    return {
        post = function(url, body)
        end,
        put = function(url, body)
        end,
        patch = function(url, body)
        end,
        delete = function(url)
        end
    }
end

---Perform a GET request to a URL.
---@param url string The URL to request.
---@return table The response object.
function requests.get(url)
end

---Get the status of the response.
---@param response table The response object.
---@return number The HTTP status code.
function requests.get_status(response)
end

---Get the headers of the response.
---@param response table The response object.
---@return table A table containing headers.
function requests.get_headers(response)
end

---Get a specific header value from the response.
---@param response table The response object.
---@param key string The header key to retrieve.
---@return string|nil The value of the header, or nil if not found.
function requests.get_header_value(response, key)
end

---Get the response body as text.
---@param response table The response object.
---@return string The response body as a string.
function requests.get_text(response)
end

---Get the response body as JSON.
---@param response table The response object.
---@return table The response body parsed as JSON.
function requests.get_json(response)
end
