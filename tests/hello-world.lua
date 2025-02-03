function main()
    print("Running Hello, World...")

    local myt = {}
    local v4 = uuid.v4

    for i = 1, 1000000 do
        myt[i] = v4()
    end
end