package.path = package.path .. ";tests/?.lua"
local requests = require("requests")

logger.debug("package.path: " .. package.path)

function main()
    logger.info("Running tests...")
    for _, test in ipairs(requests.tests) do
        test()
    end
end