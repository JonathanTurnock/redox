test.test("Should pass", function()
    test.assert_eq(1, 1)
end)

test.test("Should fail", function()
    test.assert_eq(1, 2)
end)

test.test("Should pass", function()
    test.assert_eq("one", "one")
end)

test.test("Should fail", function()
    test.assert_eq("one", "two")
end)