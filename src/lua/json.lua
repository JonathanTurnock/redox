---Global table for JSON operations.
json = {}

---Encode data into a JSON string.
---@generic T
---@param data T The data to encode.
---@return string The JSON string.
function json.encode(data) end

---Decode a JSON string into data.
---@generic T
---@param data string The JSON string to decode.
---@return T The decoded data.
function json.decode(data) end
